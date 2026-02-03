//! Gemini (Google) Provider
//!
//! Google Gemini API provider.
//! Docs: https://ai.google.dev/docs

use async_trait::async_trait;
use futures::stream::{BoxStream, StreamExt};
use serde_json::Value;

use crate::http_client::AiHttpClient;
use crate::provider::AiProvider;
use crate::types::*;

/// Gemini API base URL
pub const GEMINI_API_URL: &str = "https://generativelanguage.googleapis.com";

/// Default models for Gemini
pub fn default_gemini_models() -> Vec<ModelInfo> {
    vec![
        ModelInfo {
            id: ModelId::new("gemini-1.5-pro"),
            name: "Gemini 1.5 Pro".to_string(),
            description: Some("Most capable Gemini model with 2M context".to_string()),
            provider: ProviderId::new("gemini"),
            max_tokens: 2_097_152,
            capabilities: ProviderCapabilities {
                supports_chat: true,
                supports_vision: true,
                supports_tools: true,
                supports_streaming: true,
                supports_reasoning: true,
                supports_embeddings: false,
                max_context_length: Some(2_097_152),
            },
            pricing: Some(ModelPricing {
                input_price_per_1k: 1.25,
                output_price_per_1k: 5.00,
                currency: "USD".to_string(),
            }),
            metadata: Default::default(),
        },
        ModelInfo {
            id: ModelId::new("gemini-1.5-flash"),
            name: "Gemini 1.5 Flash".to_string(),
            description: Some("Fast and efficient model".to_string()),
            provider: ProviderId::new("gemini"),
            max_tokens: 1_048_576,
            capabilities: ProviderCapabilities {
                supports_chat: true,
                supports_vision: true,
                supports_tools: true,
                supports_streaming: true,
                supports_reasoning: false,
                supports_embeddings: false,
                max_context_length: Some(1_048_576),
            },
            pricing: Some(ModelPricing {
                input_price_per_1k: 0.075,
                output_price_per_1k: 0.30,
                currency: "USD".to_string(),
            }),
            metadata: Default::default(),
        },
    ]
}

/// Gemini provider
pub struct GeminiProvider {
    config: ProviderConfig,
    client: AiHttpClient,
    models: Vec<ModelInfo>,
    api_key_state: ApiKeyState,
    metadata: ProviderMetadata,
}

impl Default for GeminiProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl GeminiProvider {
    /// Create a new Gemini provider with default configuration
    pub fn new() -> Self {
        let config = ProviderConfig {
            provider_id: ProviderId::new("gemini"),
            name: "Gemini".to_string(),
            endpoint: GEMINI_API_URL.to_string(),
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
            id: ProviderId::new("gemini"),
            name: "Gemini".to_string(),
            description: "Google Gemini AI models".to_string(),
            icon: Some("sparkle".to_string()),
            is_local: false,
            requires_api_key: true,
            website_url: "https://ai.google.dev".to_string(),
            documentation_url: "https://ai.google.dev/docs".to_string(),
        };

        Self {
            config,
            client,
            models: default_gemini_models(),
            api_key_state,
            metadata,
        }
    }

    fn init_api_key_state(config: &ProviderConfig) -> ApiKeyState {
        if let Ok(key) = std::env::var("GOOGLE_API_KEY") {
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
            max_context_length: Some(2_097_152),
        }
    }

    /// Convert messages to Gemini format
    fn convert_messages(&self, messages: &[ChatMessage]) -> Vec<Value> {
        messages
            .iter()
            .map(|msg| {
                let role = match msg.role {
                    MessageRole::User => "user",
                    MessageRole::Model | MessageRole::Assistant => "model",
                    _ => "user",
                };

                serde_json::json!({
                    "role": role,
                    "parts": [{"text": msg.content}]
                })
            })
            .collect()
    }

    /// Build API URL with key
    fn build_url(&self, path: &str) -> String {
        let key = match &self.api_key_state {
            ApiKeyState::Configured(k) | ApiKeyState::FromEnv(k) => k,
            _ => "",
        };
        format!("{}{}?key={}", self.config.endpoint, path, key)
    }

    /// Parse Gemini response
    fn parse_response(&self, json: Value) -> Result<ChatCompletionResponse, ApiError> {
        let candidates = json
            .get("candidates")
            .and_then(|v| v.as_array())
            .ok_or_else(|| ApiError::InvalidRequest("No candidates in response".to_string()))?;

        let candidate = candidates
            .first()
            .ok_or_else(|| ApiError::InvalidRequest("Empty candidates array".to_string()))?;

        let content = candidate
            .get("content")
            .and_then(|c| c.get("parts"))
            .and_then(|p| p.as_array())
            .and_then(|arr| arr.first())
            .and_then(|part| part.get("text"))
            .and_then(|t| t.as_str())
            .unwrap_or("")
            .to_string();

        let usage = json
            .get("usageMetadata")
            .map(|u| TokenUsage {
                prompt_tokens: u.get("promptTokenCount").and_then(|v| v.as_u64()).unwrap_or(0) as u32,
                completion_tokens: u.get("candidatesTokenCount").and_then(|v| v.as_u64()).unwrap_or(0) as u32,
                total_tokens: u.get("totalTokenCount").and_then(|v| v.as_u64()).unwrap_or(0) as u32,
            })
            .unwrap_or_default();

        let finish_reason = candidate
            .get("finishReason")
            .and_then(|v| v.as_str())
            .map(|s| match s {
                "STOP" => FinishReason::Stop,
                "MAX_TOKENS" => FinishReason::Length,
                "SAFETY" => FinishReason::ContentFilter,
                _ => FinishReason::Other,
            })
            .unwrap_or(FinishReason::Other);

        Ok(ChatCompletionResponse {
            id: "gemini-".to_string() + &uuid::Uuid::new_v4().to_string(),
            model: ModelId::new("gemini"),
            content,
            usage,
            finish_reason,
            tool_calls: vec![],
        })
    }
}

#[async_trait]
impl AiProvider for GeminiProvider {
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
        // Gemini API doesn't have a models endpoint in the same way
        Ok(self.models.clone())
    }

    async fn test_connection(&self) -> Result<ConnectionTestResult, ApiError> {
        let start = std::time::Instant::now();

        if !self.is_authenticated() {
            return Ok(ConnectionTestResult {
                success: false,
                latency_ms: 0,
                message: "API key not configured".to_string(),
                models_available: None,
                error: Some("Missing API key".to_string()),
            });
        }

        // Use a simple request to test connection
        let body = serde_json::json!({
            "contents": [{"role": "user", "parts": [{"text": "Hi"}]}]
        });

        let url = self.build_url("/v1beta/models/gemini-1.5-flash:generateContent");

        match self.client.post(&url, body).await {
            Ok(_) => Ok(ConnectionTestResult {
                success: true,
                latency_ms: start.elapsed().as_millis() as u64,
                message: "Successfully connected to Gemini API".to_string(),
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
        let contents = self.convert_messages(&request.messages);

        let mut body = serde_json::json!({
            "contents": contents,
        });

        if let Some(tokens) = request.max_tokens {
            body["generationConfig"] = serde_json::json!({
                "maxOutputTokens": tokens
            });
        }

        if let Some(temp) = request.temperature {
            body["generationConfig"]["temperature"] = serde_json::json!(temp);
        }

        let url = self.build_url(&format!("/v1beta/models/{}:generateContent", request.model.as_str()));

        let response = self.client.post(&url, body).await?;

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
        Err(ApiError::Other("Embeddings not yet implemented for Gemini".to_string()))
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
