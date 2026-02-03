//! OpenAI-Compatible Provider Base
//!
//! Base implementation for providers that use OpenAI-compatible API format.
//! This includes Kimi, DeepSeek, LMStudio, Ollama, Z.ai, and others.

use async_trait::async_trait;
use futures::stream::{BoxStream, StreamExt};
use serde_json::Value;

use crate::http_client::AiHttpClient;
use crate::provider::AiProvider;
use crate::types::*;

/// Base struct for OpenAI-compatible providers
#[derive(Clone)]
pub struct OpenAiCompatibleProvider {
    pub config: ProviderConfig,
    pub client: AiHttpClient,
    pub models: Vec<ModelInfo>,
    pub api_key_state: ApiKeyState,
    pub capabilities: ProviderCapabilities,
    pub metadata: ProviderMetadata,
}

impl OpenAiCompatibleProvider {
    /// Create a new OpenAI-compatible provider
    pub fn new(
        config: ProviderConfig,
        metadata: ProviderMetadata,
        capabilities: ProviderCapabilities,
        default_models: Vec<ModelInfo>,
    ) -> Self {
        let client = AiHttpClient::new(&config);
        let api_key_state = Self::init_api_key_state(&config);

        Self {
            config,
            client,
            models: default_models,
            api_key_state,
            capabilities,
            metadata,
        }
    }

    /// Initialize API key state from config
    fn init_api_key_state(config: &ProviderConfig) -> ApiKeyState {
        // Check environment variable first
        let env_var = format!("{}_API_KEY", config.provider_id.as_str().to_uppercase());
        if let Ok(key) = std::env::var(&env_var) {
            if !key.is_empty() {
                return ApiKeyState::FromEnv(key);
            }
        }

        // Then check saved config
        if let Some(key) = &config.api_key {
            if !key.is_empty() {
                return ApiKeyState::Configured(key.clone());
            }
        }

        ApiKeyState::NotConfigured
    }

    /// Get API path for chat completions
    fn chat_completions_path(&self) -> &'static str {
        "/v1/chat/completions"
    }

    /// Get API path for models
    fn models_path(&self) -> &'static str {
        "/v1/models"
    }

    /// Get API path for embeddings
    fn embeddings_path(&self) -> &'static str {
        "/v1/embeddings"
    }

    /// Parse OpenAI-compatible model response
    fn parse_models_response(&self, response: Value) -> Vec<ModelInfo> {
        response
            .get("data")
            .and_then(|d| d.as_array())
            .map(|models| {
                models
                    .iter()
                    .filter_map(|m| {
                        let id = m.get("id")?.as_str()?;
                        Some(ModelInfo {
                            id: ModelId::new(id),
                            name: m.get("name")
                                .and_then(|n| n.as_str())
                                .unwrap_or(id)
                                .to_string(),
                            description: m.get("description")
                                .and_then(|d| d.as_str())
                                .map(String::from),
                            provider: self.config.provider_id.clone(),
                            max_tokens: m.get("context_length")
                                .or_else(|| m.get("max_tokens"))
                                .and_then(|v| v.as_u64())
                                .map(|v| v as u32)
                                .unwrap_or(4096),
                            capabilities: self.capabilities.clone(),
                            pricing: None,
                            metadata: Default::default(),
                        })
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Parse chat completion response
    fn parse_chat_response(&self, response: Value) -> Result<ChatCompletionResponse, ApiError> {
        let id = response
            .get("id")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();

        let model = response
            .get("model")
            .and_then(|v| v.as_str())
            .map(ModelId::new)
            .unwrap_or_else(|| ModelId::new("unknown"));

        let choices = response
            .get("choices")
            .and_then(|v| v.as_array())
            .ok_or_else(|| ApiError::InvalidRequest("No choices in response".to_string()))?;

        let choice = choices
            .first()
            .ok_or_else(|| ApiError::InvalidRequest("Empty choices array".to_string()))?;

        let content = choice
            .get("message")
            .and_then(|m| m.get("content"))
            .and_then(|c| c.as_str())
            .unwrap_or("")
            .to_string();

        let finish_reason = choice
            .get("finish_reason")
            .and_then(|v| v.as_str())
            .map(|s| match s {
                "stop" => FinishReason::Stop,
                "length" => FinishReason::Length,
                "tool_calls" => FinishReason::ToolCalls,
                "content_filter" => FinishReason::ContentFilter,
                _ => FinishReason::Other,
            })
            .unwrap_or(FinishReason::Other);

        let usage = response
            .get("usage")
            .map(|u| TokenUsage {
                prompt_tokens: u.get("prompt_tokens").and_then(|v| v.as_u64()).unwrap_or(0) as u32,
                completion_tokens: u.get("completion_tokens").and_then(|v| v.as_u64()).unwrap_or(0) as u32,
                total_tokens: u.get("total_tokens").and_then(|v| v.as_u64()).unwrap_or(0) as u32,
            })
            .unwrap_or_default();

        let tool_calls = choice
            .get("message")
            .and_then(|m| m.get("tool_calls"))
            .and_then(|t| t.as_array())
            .map(|calls| {
                calls
                    .iter()
                    .filter_map(|call| {
                        Some(ToolCall {
                            id: call.get("id")?.as_str()?.to_string(),
                            function: FunctionCall {
                                name: call.get("function")?.get("name")?.as_str()?.to_string(),
                                arguments: call
                                    .get("function")?
                                    .get("arguments")?
                                    .as_str()?
                                    .to_string(),
                            },
                        })
                    })
                    .collect()
            })
            .unwrap_or_default();

        Ok(ChatCompletionResponse {
            id,
            model,
            content,
            usage,
            finish_reason,
            tool_calls,
        })
    }

    /// Parse streaming chunk
    fn parse_stream_chunk(&self, line: &str) -> Result<Option<ChatCompletionChunk>, ApiError> {
        if !line.starts_with("data: ") {
            return Ok(None);
        }

        let data = &line[6..];

        if data == "[DONE]" {
            return Ok(None);
        }

        let json: Value = serde_json::from_str(data)
            .map_err(|e| ApiError::InvalidRequest(format!("Invalid JSON: {}", e)))?;

        let id = json
            .get("id")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();

        let model = json
            .get("model")
            .and_then(|v| v.as_str())
            .map(ModelId::new)
            .unwrap_or_else(|| ModelId::new("unknown"));

        let delta = json
            .get("choices")
            .and_then(|v| v.as_array())
            .and_then(|a| a.first())
            .and_then(|c| c.get("delta"))
            .and_then(|d| d.get("content"))
            .and_then(|c| c.as_str())
            .unwrap_or("")
            .to_string();

        let finish_reason = json
            .get("choices")
            .and_then(|v| v.as_array())
            .and_then(|a| a.first())
            .and_then(|c| c.get("finish_reason"))
            .and_then(|f| f.as_str())
            .map(|s| match s {
                "stop" => FinishReason::Stop,
                "length" => FinishReason::Length,
                "tool_calls" => FinishReason::ToolCalls,
                "content_filter" => FinishReason::ContentFilter,
                _ => FinishReason::Other,
            });

        Ok(Some(ChatCompletionChunk {
            id,
            model,
            content_delta: delta,
            finish_reason,
        }))
    }
}

#[async_trait]
impl AiProvider for OpenAiCompatibleProvider {
    fn provider_id(&self) -> ProviderId {
        self.config.provider_id.clone()
    }

    fn name(&self) -> &str {
        &self.metadata.name
    }

    fn description(&self) -> &str {
        &self.metadata.description
    }

    fn metadata(&self) -> ProviderMetadata {
        self.metadata.clone()
    }

    fn is_authenticated(&self) -> bool {
        self.api_key_state.is_configured()
    }

    fn api_key_state(&self) -> ApiKeyState {
        self.api_key_state.clone()
    }

    fn set_api_key(&mut self, key: Option<String>) {
        match key {
            Some(k) => {
                self.api_key_state = ApiKeyState::Configured(k.clone());
                self.config.api_key = Some(k);
            }
            None => {
                self.api_key_state = ApiKeyState::NotConfigured;
                self.config.api_key = None;
            }
        }
    }

    fn capabilities(&self) -> ProviderCapabilities {
        self.capabilities.clone()
    }

    fn available_models(&self) -> Vec<ModelInfo> {
        self.models.clone()
    }

    async fn fetch_models(&self) -> Result<Vec<ModelInfo>, ApiError> {
        let response = self.client.get(self.models_path()).await?;
        let json: Value = response
            .json()
            .await
            .map_err(|e| ApiError::InvalidRequest(format!("Failed to parse response: {}", e)))?;

        Ok(self.parse_models_response(json))
    }

    async fn test_connection(&self) -> Result<ConnectionTestResult, ApiError> {
        let start = std::time::Instant::now();

        match self.fetch_models().await {
            Ok(models) => Ok(ConnectionTestResult {
                success: true,
                latency_ms: start.elapsed().as_millis() as u64,
                message: format!("Successfully connected. Found {} models.", models.len()),
                models_available: Some(models.len()),
                error: None,
            }),
            Err(e) => Ok(ConnectionTestResult {
                success: false,
                latency_ms: start.elapsed().as_millis() as u64,
                message: format!("Connection failed: {}", e),
                models_available: None,
                error: Some(e.to_string()),
            }),
        }
    }

    async fn get_latency(&self) -> Result<u64, ApiError> {
        self.client.ping().await
    }

    async fn chat_completion(
        &self,
        request: ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse, ApiError> {
        let body = serde_json::json!({
            "model": request.model.as_str(),
            "messages": request.messages,
            "stream": false,
            "temperature": request.temperature.unwrap_or(0.7),
            "max_tokens": request.max_tokens,
        });

        let response = self
            .client
            .post(self.chat_completions_path(), body)
            .await?;

        let json: Value = response
            .json()
            .await
            .map_err(|e| ApiError::InvalidRequest(format!("Failed to parse response: {}", e)))?;

        self.parse_chat_response(json)
    }

    async fn chat_completion_stream(
        &self,
        _request: ChatCompletionRequest,
    ) -> Result<BoxStream<'static, Result<ChatCompletionChunk, ApiError>>, ApiError> {
        // Streaming requires reqwest's stream feature which may not be enabled
        // For now, return a single chunk with a note
        let chunk = ChatCompletionChunk {
            id: "stream-not-implemented".to_string(),
            model: ModelId::new("unknown"),
            content_delta: "Streaming not yet implemented. Use non-streaming chat_completion instead.".to_string(),
            finish_reason: Some(FinishReason::Stop),
        };

        Ok(futures::stream::once(async move { Ok(chunk) }).boxed())
    }

    async fn create_embeddings(
        &self,
        request: EmbeddingRequest,
    ) -> Result<EmbeddingResponse, ApiError> {
        let body = serde_json::json!({
            "model": request.model.as_str(),
            "input": request.input,
        });

        let response = self.client.post(self.embeddings_path(), body).await?;

        let json: Value = response
            .json()
            .await
            .map_err(|e| ApiError::InvalidRequest(format!("Failed to parse response: {}", e)))?;

        let embeddings = json
            .get("data")
            .and_then(|d| d.as_array())
            .map(|data| {
                data.iter()
                    .filter_map(|item| {
                        item.get("embedding")
                            .and_then(|e| e.as_array())
                            .map(|vec| vec.iter().filter_map(|v| v.as_f64().map(|f| f as f32)).collect())
                    })
                    .collect()
            })
            .unwrap_or_default();

        let usage = json
            .get("usage")
            .map(|u| TokenUsage {
                prompt_tokens: u.get("prompt_tokens").and_then(|v| v.as_u64()).unwrap_or(0) as u32,
                completion_tokens: 0,
                total_tokens: u.get("total_tokens").and_then(|v| v.as_u64()).unwrap_or(0) as u32,
            })
            .unwrap_or_default();

        Ok(EmbeddingResponse {
            model: request.model,
            embeddings,
            usage,
        })
    }

    fn config(&self) -> &ProviderConfig {
        &self.config
    }

    fn update_config(&mut self, config: ProviderConfig) {
        self.config = config.clone();
        self.client = AiHttpClient::new(&config);
        self.api_key_state = Self::init_api_key_state(&config);
    }

    fn clone_box(&self) -> Box<dyn AiProvider> {
        Box::new(Self {
            config: self.config.clone(),
            client: AiHttpClient::new(&self.config),
            models: self.models.clone(),
            api_key_state: self.api_key_state.clone(),
            capabilities: self.capabilities.clone(),
            metadata: self.metadata.clone(),
        })
    }
}
