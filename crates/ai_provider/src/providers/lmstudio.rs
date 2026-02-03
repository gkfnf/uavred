//! LMStudio Provider
//!
//! LMStudio local AI model provider with OpenAI-compatible API.
//! Docs: https://lmstudio.ai/docs

use crate::providers::openai_compatible::OpenAiCompatibleProvider;
use crate::types::*;

/// LMStudio API base URL (local)
pub const LMSTUDIO_API_URL: &str = "http://localhost:1234";

/// Default models for LMStudio (loaded dynamically)
pub fn default_lmstudio_models() -> Vec<ModelInfo> {
    vec![
        ModelInfo {
            id: ModelId::new("local-model"),
            name: "Local Model".to_string(),
            description: Some("Model loaded in LMStudio".to_string()),
            provider: ProviderId::new("lmstudio"),
            max_tokens: 4096,
            capabilities: ProviderCapabilities {
                supports_chat: true,
                supports_vision: false,
                supports_tools: false,
                supports_streaming: true,
                supports_reasoning: false,
                supports_embeddings: false,
                max_context_length: Some(4096),
            },
            pricing: None,
            metadata: Default::default(),
        },
    ]
}

/// LMStudio provider
pub struct LMStudioProvider {
    inner: OpenAiCompatibleProvider,
}

impl Default for LMStudioProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl LMStudioProvider {
    /// Create a new LMStudio provider with default configuration
    pub fn new() -> Self {
        let config = ProviderConfig {
            provider_id: ProviderId::new("lmstudio"),
            name: "LMStudio".to_string(),
            endpoint: LMSTUDIO_API_URL.to_string(),
            api_key: None, // Local, no API key needed
            organization_id: None,
            project_id: None,
            region: None,
            timeout_seconds: 120,
            max_retries: 1,
            custom_headers: Default::default(),
            enabled: false,
        };

        let metadata = ProviderMetadata {
            id: ProviderId::new("lmstudio"),
            name: "LMStudio".to_string(),
            description: "Run AI models locally with LMStudio GUI".to_string(),
            icon: Some("monitor".to_string()),
            is_local: true,
            requires_api_key: false,
            website_url: "https://lmstudio.ai".to_string(),
            documentation_url: "https://lmstudio.ai/docs".to_string(),
        };

        let capabilities = ProviderCapabilities {
            supports_chat: true,
            supports_vision: false,
            supports_tools: false,
            supports_streaming: true,
            supports_reasoning: false,
            supports_embeddings: true,
            max_context_length: Some(128_000),
        };

        Self {
            inner: OpenAiCompatibleProvider::new(
                config,
                metadata,
                capabilities,
                default_lmstudio_models(),
            ),
        }
    }

    /// Create with custom configuration
    pub fn with_config(config: ProviderConfig) -> Self {
        let metadata = ProviderMetadata {
            id: ProviderId::new("lmstudio"),
            name: "LMStudio".to_string(),
            description: "Run AI models locally with LMStudio GUI".to_string(),
            icon: Some("monitor".to_string()),
            is_local: true,
            requires_api_key: false,
            website_url: "https://lmstudio.ai".to_string(),
            documentation_url: "https://lmstudio.ai/docs".to_string(),
        };

        let capabilities = ProviderCapabilities {
            supports_chat: true,
            supports_vision: false,
            supports_tools: false,
            supports_streaming: true,
            supports_reasoning: false,
            supports_embeddings: true,
            max_context_length: Some(128_000),
        };

        Self {
            inner: OpenAiCompatibleProvider::new(
                config,
                metadata,
                capabilities,
                default_lmstudio_models(),
            ),
        }
    }

    /// Check if LMStudio is running locally
    pub async fn is_running(&self) -> bool {
        match self.inner.get_latency().await {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}

use async_trait::async_trait;
use crate::provider::AiProvider;

#[async_trait]
impl AiProvider for LMStudioProvider {
    fn provider_id(&self) -> ProviderId {
        self.inner.provider_id()
    }

    fn name(&self) -> &str {
        self.inner.name()
    }

    fn description(&self) -> &str {
        self.inner.description()
    }

    fn metadata(&self) -> ProviderMetadata {
        self.inner.metadata()
    }

    fn is_authenticated(&self) -> bool {
        self.inner.is_authenticated()
    }

    fn api_key_state(&self) -> ApiKeyState {
        self.inner.api_key_state()
    }

    fn set_api_key(&mut self, key: Option<String>) {
        self.inner.set_api_key(key);
    }

    fn capabilities(&self) -> ProviderCapabilities {
        self.inner.capabilities()
    }

    fn available_models(&self) -> Vec<ModelInfo> {
        self.inner.available_models()
    }

    async fn fetch_models(&self) -> Result<Vec<ModelInfo>, ApiError> {
        self.inner.fetch_models().await
    }

    async fn test_connection(&self) -> Result<ConnectionTestResult, ApiError> {
        self.inner.test_connection().await
    }

    async fn get_latency(&self) -> Result<u64, ApiError> {
        self.inner.get_latency().await
    }

    async fn chat_completion(
        &self,
        request: ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse, ApiError> {
        self.inner.chat_completion(request).await
    }

    async fn chat_completion_stream(
        &self,
        request: ChatCompletionRequest,
    ) -> Result<futures::stream::BoxStream<'static, Result<ChatCompletionChunk, ApiError>>, ApiError> {
        self.inner.chat_completion_stream(request).await
    }

    async fn create_embeddings(
        &self,
        request: EmbeddingRequest,
    ) -> Result<EmbeddingResponse, ApiError> {
        self.inner.create_embeddings(request).await
    }

    fn config(&self) -> &ProviderConfig {
        self.inner.config()
    }

    fn update_config(&mut self, config: ProviderConfig) {
        self.inner.update_config(config);
    }

    fn clone_box(&self) -> Box<dyn AiProvider> {
        Box::new(Self {
            inner: self.inner.clone(),
        })
    }
}
