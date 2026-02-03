//! Claude (Anthropic) Provider
//!
//! Anthropic Claude API provider with native Anthropic API format.
//! Docs: https://docs.anthropic.com/claude/reference

use async_trait::async_trait;
use futures::stream::{BoxStream, StreamExt};
use serde_json::Value;

use crate::http_client::AiHttpClient;
use crate::provider::AiProvider;
use crate::types::*;

/// Claude API base URL
pub const CLAUDE_API_URL: &str = "https://api.anthropic.com";

/// Default models for Claude
pub fn default_claude_models() -> Vec<ModelInfo> {
    vec![
        ModelInfo {
            id: ModelId::new("claude-3-5-sonnet-20241022"),
            name: "Claude 3.5 Sonnet".to_string(),
            description: Some("Most capable Claude model for complex tasks".to_string()),
            provider: ProviderId::new("claude"),
            max_tokens: 200_000,
            capabilities: ProviderCapabilities {
                supports_chat: true,
                supports_vision: true,
                supports_tools: true,
                supports_streaming: true,
                supports_reasoning: true,
                supports_embeddings: false,
                max_context_length: Some(200_000),
            },
            pricing: Some(ModelPricing {
                input_price_per_1k: 3.00,
                output_price_per_1k: 15.00,
                currency: "USD".to_string(),
            }),
            metadata: Default::default(),
        },
        ModelInfo {
            id: ModelId::new("claude-3-5-haiku-20241022"),
            name: "Claude 3.5 Haiku".to_string(),
            description: Some("Fast and cost-effective model".to_string()),
            provider: ProviderId::new("claude"),
            max_tokens: 200_000,
            capabilities: ProviderCapabilities {
                supports_chat: true,
                supports_vision: true,
                supports_tools: true,
                supports_streaming: true,
                supports_reasoning: false,
                supports_embeddings: false,
                max_context_length: Some(200_000),
            },
            pricing: Some(ModelPricing {
                input_price_per_1k: 0.80,
                output_price_per_1k: 4.00,
                currency: "USD".to_string(),
            }),
            metadata: Default::default(),
        },
        ModelInfo {
            id: ModelId::new("claude-3-opus-20240229"),
            name: "Claude 3 Opus".to_string(),
            description: Some("Most powerful Claude model for highly complex tasks".to_string()),
            provider: ProviderId::new("claude"),
            max_tokens: 200_000,
            capabilities: ProviderCapabilities {
                supports_chat: true,
                supports_vision: true,
                supports_tools: true,
                supports_streaming: true,
                supports_reasoning: true,
                supports_embeddings: false,
                max_context_length: Some(200_000),
            },
            pricing: Some(ModelPricing {
                input_price_per_1k: 15.00,
                output_price_per_1k: 75.00,
                currency: "USD".to_string(),
            }),
            metadata: Default::default(),
        },
    ]
}

/// Claude provider
pub struct ClaudeProvider {
    config: ProviderConfig,
    client: AiHttpClient,
    models: Vec<ModelInfo>,
    api_key_state: ApiKeyState,
    metadata: ProviderMetadata,
}

impl Default for ClaudeProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl ClaudeProvider {
    /// Create a new Claude provider with default configuration
    pub fn new() -> Self {
        let config = ProviderConfig {
            provider_id: ProviderId::new("claude"),
            name: "Claude".to_string(),
            endpoint: CLAUDE_API_URL.to_string(),
            api_key: None,
            organization_id: None,
            project_id: None,
            region: Some("international".to_string()),
            timeout_seconds: 60,
            max_retries: 3,
            custom_headers: Default::default(),
            enabled: false,
        };

        let client = AiHttpClient::new(&config);
        let api_key_state = Self::init_api_key_state(&config);

        let metadata = ProviderMetadata {
            id: ProviderId::new("claude"),
            name: "Claude".to_string(),
            description: "Anthropic Claude AI models".to_string(),
            icon: Some("message-square".to_string()),
            is_local: false,
            requires_api_key: true,
            website_url: "https://anthropic.com".to_string(),
            documentation_url: "https://docs.anthropic.com".to_string(),
        };

        Self {
            config,
            client,
            models: default_claude_models(),
            api_key_state,
            metadata,
        }
    }

    fn init_api_key_state(config: &ProviderConfig) -> ApiKeyState {
        if let Ok(key) = std::env::var("ANTHROPIC_API_KEY") {
            if !key.is_empty() {
                return ApiKeyState::FromEnv(key);
            }
        }

        if let Some(key) = &config.api_key {
            if !key.is_empty() {
                return ApiKeyState::Configured(key.clone());
            }
        }

        ApiKeyState::NotConfigured
    }

    fn capabilities(&self) -> ProviderCapabilities {
        ProviderCapabilities {
            supports_chat: true,
            supports_vision: true,
            supports_tools: true,
            supports_streaming: true,
            supports_reasoning: true,
            supports_embeddings: false,
            max_context_length: Some(200_000),
        }
    }

    /// Convert messages to Claude format
    fn convert_messages(&self, messages: &[ChatMessage]) -> (Option<String>, Vec<Value>) {
        let mut system = None;
        let mut claude_messages = Vec::new();

        for msg in messages {
            match msg.role {
                MessageRole::System => {
                    system = Some(msg.content.clone());
                }
                MessageRole::User => {
                    claude_messages.push(serde_json::json!({
                        "role": "user",
                        "content": msg.content
                    }));
                }
                MessageRole::Assistant => {
                    claude_messages.push(serde_json::json!({
                        "role": "assistant",
                        "content": msg.content
                    }));
                }
                _ => {}
            }
        }

        (system, claude_messages)
    }

    /// Parse Claude response
    fn parse_response(&self, json: Value) -> Result<ChatCompletionResponse, ApiError> {
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

        let content = json
            .get("content")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .and_then(|item| item.get("text"))
            .and_then(|t| t.as_str())
            .unwrap_or("")
            .to_string();

        let usage = json
            .get("usage")
            .map(|u| TokenUsage {
                prompt_tokens: u.get("input_tokens").and_then(|v| v.as_u64()).unwrap_or(0) as u32,
                completion_tokens: u.get("output_tokens").and_then(|v| v.as_u64()).unwrap_or(0) as u32,
                total_tokens: 0,
            })
            .unwrap_or_default();

        let finish_reason = json
            .get("stop_reason")
            .and_then(|v| v.as_str())
            .map(|s| match s {
                "end_turn" => FinishReason::Stop,
                "max_tokens" => FinishReason::Length,
                "stop_sequence" => FinishReason::Stop,
                _ => FinishReason::Other,
            })
            .unwrap_or(FinishReason::Other);

        Ok(ChatCompletionResponse {
            id,
            model,
            content,
            usage,
            finish_reason,
            tool_calls: vec![],
        })
    }
}

#[async_trait]
impl AiProvider for ClaudeProvider {
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
        self.capabilities()
    }

    fn available_models(&self) -> Vec<ModelInfo> {
        self.models.clone()
    }

    async fn fetch_models(&self) -> Result<Vec<ModelInfo>, ApiError> {
        // Claude doesn't have a models endpoint, return default models
        Ok(self.models.clone())
    }

    async fn test_connection(&self) -> Result<ConnectionTestResult, ApiError> {
        let start = std::time::Instant::now();

        // Use a simple request to test connection
        let body = serde_json::json!({
            "model": "claude-3-5-haiku-20241022",
            "max_tokens": 1,
            "messages": [{"role": "user", "content": "Hi"}]
        });

        match self.client.post("/v1/messages", body).await {
            Ok(_) => Ok(ConnectionTestResult {
                success: true,
                latency_ms: start.elapsed().as_millis() as u64,
                message: "Successfully connected to Claude API".to_string(),
                models_available: Some(self.models.len()),
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
        let (system, messages) = self.convert_messages(&request.messages);

        let mut body = serde_json::json!({
            "model": request.model.as_str(),
            "messages": messages,
            "max_tokens": request.max_tokens.unwrap_or(4096),
        });

        if let Some(system) = system {
            body["system"] = serde_json::json!(system);
        }

        if let Some(temp) = request.temperature {
            body["temperature"] = serde_json::json!(temp);
        }

        let response = self.client.post("/v1/messages", body).await?;

        let json: Value = response
            .json()
            .await
            .map_err(|e| ApiError::InvalidRequest(format!("Failed to parse response: {}", e)))?;

        self.parse_response(json)
    }

    async fn chat_completion_stream(
        &self,
        _request: ChatCompletionRequest,
    ) -> Result<BoxStream<'static, Result<ChatCompletionChunk, ApiError>>, ApiError> {
        // Streaming requires reqwest's stream feature which may not be enabled
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
        _request: EmbeddingRequest,
    ) -> Result<EmbeddingResponse, ApiError> {
        Err(ApiError::Other("Embeddings not supported by Claude".to_string()))
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
            metadata: self.metadata.clone(),
        })
    }
}
