//! DeepSeek Provider
//!
//! DeepSeek AI API provider with OpenAI-compatible interface.
//! Docs: https://platform.deepseek.com/docs

use crate::providers::openai_compatible::OpenAiCompatibleProvider;
use crate::types::*;

/// DeepSeek API base URL
pub const DEEPSEEK_API_URL: &str = "https://api.deepseek.com";

/// Default models for DeepSeek
pub fn default_deepseek_models() -> Vec<ModelInfo> {
    vec![
        ModelInfo {
            id: ModelId::new("deepseek-chat"),
            name: "DeepSeek Chat".to_string(),
            description: Some("General purpose chat model with strong performance".to_string()),
            provider: ProviderId::new("deepseek"),
            max_tokens: 64_000,
            capabilities: ProviderCapabilities {
                supports_chat: true,
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                supports_reasoning: false,
                supports_embeddings: false,
                max_context_length: Some(64_000),
            },
            pricing: Some(ModelPricing {
                input_price_per_1k: 0.07,
                output_price_per_1k: 0.27,
                currency: "USD".to_string(),
            }),
            metadata: Default::default(),
        },
        ModelInfo {
            id: ModelId::new("deepseek-reasoner"),
            name: "DeepSeek Reasoner".to_string(),
            description: Some("Reasoning model with step-by-step thinking capabilities".to_string()),
            provider: ProviderId::new("deepseek"),
            max_tokens: 64_000,
            capabilities: ProviderCapabilities {
                supports_chat: true,
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                supports_reasoning: true,
                supports_embeddings: false,
                max_context_length: Some(64_000),
            },
            pricing: Some(ModelPricing {
                input_price_per_1k: 0.14,
                output_price_per_1k: 0.55,
                currency: "USD".to_string(),
            }),
            metadata: Default::default(),
        },
    ]
}

/// DeepSeek provider
pub struct DeepSeekProvider {
    inner: OpenAiCompatibleProvider,
}

impl Default for DeepSeekProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl DeepSeekProvider {
    /// Create a new DeepSeek provider with default configuration
    pub fn new() -> Self {
        let config = ProviderConfig {
            provider_id: ProviderId::new("deepseek"),
            name: "DeepSeek".to_string(),
            endpoint: DEEPSEEK_API_URL.to_string(),
            api_key: None,
            organization_id: None,
            project_id: None,
            region: Some("international".to_string()),
            timeout_seconds: 60,
            max_retries: 3,
            custom_headers: Default::default(),
            enabled: false,
        };

        let metadata = ProviderMetadata {
            id: ProviderId::new("deepseek"),
            name: "DeepSeek".to_string(),
            description: "DeepSeek AI with reasoning and chat capabilities".to_string(),
            icon: Some("brain".to_string()),
            is_local: false,
            requires_api_key: true,
            website_url: "https://www.deepseek.com".to_string(),
            documentation_url: "https://platform.deepseek.com/docs".to_string(),
        };

        let capabilities = ProviderCapabilities {
            supports_chat: true,
            supports_vision: false,
            supports_tools: true,
            supports_streaming: true,
            supports_reasoning: true,
            supports_embeddings: false,
            max_context_length: Some(64_000),
        };

        Self {
            inner: OpenAiCompatibleProvider::new(
                config,
                metadata,
                capabilities,
                default_deepseek_models(),
            ),
        }
    }

    /// Create with custom configuration
    pub fn with_config(config: ProviderConfig) -> Self {
        let metadata = ProviderMetadata {
            id: ProviderId::new("deepseek"),
            name: "DeepSeek".to_string(),
            description: "DeepSeek AI with reasoning and chat capabilities".to_string(),
            icon: Some("brain".to_string()),
            is_local: false,
            requires_api_key: true,
            website_url: "https://www.deepseek.com".to_string(),
            documentation_url: "https://platform.deepseek.com/docs".to_string(),
        };

        let capabilities = ProviderCapabilities {
            supports_chat: true,
            supports_vision: false,
            supports_tools: true,
            supports_streaming: true,
            supports_reasoning: true,
            supports_embeddings: false,
            max_context_length: Some(64_000),
        };

        Self {
            inner: OpenAiCompatibleProvider::new(
                config,
                metadata,
                capabilities,
                default_deepseek_models(),
            ),
        }
    }
}

use async_trait::async_trait;
use crate::provider::AiProvider;

#[async_trait]
impl AiProvider for DeepSeekProvider {
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
