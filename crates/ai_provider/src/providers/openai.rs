//! OpenAI Provider
//!
//! OpenAI API provider.
//! Docs: https://platform.openai.com/docs

use crate::providers::openai_compatible::OpenAiCompatibleProvider;
use crate::types::*;

/// OpenAI API base URL
pub const OPENAI_API_URL: &str = "https://api.openai.com/v1";

/// Default models for OpenAI
pub fn default_openai_models() -> Vec<ModelInfo> {
    vec![
        ModelInfo {
            id: ModelId::new("gpt-4o"),
            name: "GPT-4o".to_string(),
            description: Some("Most capable multimodal model".to_string()),
            provider: ProviderId::new("openai"),
            max_tokens: 128_000,
            capabilities: ProviderCapabilities {
                supports_chat: true,
                supports_vision: true,
                supports_tools: true,
                supports_streaming: true,
                supports_reasoning: false,
                supports_embeddings: false,
                max_context_length: Some(128_000),
            },
            pricing: Some(ModelPricing {
                input_price_per_1k: 2.50,
                output_price_per_1k: 10.00,
                currency: "USD".to_string(),
            }),
            metadata: Default::default(),
        },
        ModelInfo {
            id: ModelId::new("gpt-4o-mini"),
            name: "GPT-4o Mini".to_string(),
            description: Some("Fast and affordable multimodal model".to_string()),
            provider: ProviderId::new("openai"),
            max_tokens: 128_000,
            capabilities: ProviderCapabilities {
                supports_chat: true,
                supports_vision: true,
                supports_tools: true,
                supports_streaming: true,
                supports_reasoning: false,
                supports_embeddings: false,
                max_context_length: Some(128_000),
            },
            pricing: Some(ModelPricing {
                input_price_per_1k: 0.15,
                output_price_per_1k: 0.60,
                currency: "USD".to_string(),
            }),
            metadata: Default::default(),
        },
        ModelInfo {
            id: ModelId::new("gpt-4-turbo"),
            name: "GPT-4 Turbo".to_string(),
            description: Some("Previous generation GPT-4 with vision".to_string()),
            provider: ProviderId::new("openai"),
            max_tokens: 128_000,
            capabilities: ProviderCapabilities {
                supports_chat: true,
                supports_vision: true,
                supports_tools: true,
                supports_streaming: true,
                supports_reasoning: false,
                supports_embeddings: false,
                max_context_length: Some(128_000),
            },
            pricing: Some(ModelPricing {
                input_price_per_1k: 10.00,
                output_price_per_1k: 30.00,
                currency: "USD".to_string(),
            }),
            metadata: Default::default(),
        },
        ModelInfo {
            id: ModelId::new("text-embedding-3-small"),
            name: "Text Embedding 3 Small".to_string(),
            description: Some("Efficient embedding model".to_string()),
            provider: ProviderId::new("openai"),
            max_tokens: 8192,
            capabilities: ProviderCapabilities {
                supports_chat: false,
                supports_vision: false,
                supports_tools: false,
                supports_streaming: false,
                supports_reasoning: false,
                supports_embeddings: true,
                max_context_length: Some(8192),
            },
            pricing: Some(ModelPricing {
                input_price_per_1k: 0.02,
                output_price_per_1k: 0.0,
                currency: "USD".to_string(),
            }),
            metadata: Default::default(),
        },
    ]
}

/// OpenAI provider
pub struct OpenAiProvider {
    inner: OpenAiCompatibleProvider,
}

impl Default for OpenAiProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl OpenAiProvider {
    /// Create a new OpenAI provider with default configuration
    pub fn new() -> Self {
        let config = ProviderConfig {
            provider_id: ProviderId::new("openai"),
            name: "OpenAI".to_string(),
            endpoint: OPENAI_API_URL.to_string(),
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
            id: ProviderId::new("openai"),
            name: "OpenAI".to_string(),
            description: "OpenAI GPT models including GPT-4o".to_string(),
            icon: Some("sparkles".to_string()),
            is_local: false,
            requires_api_key: true,
            website_url: "https://openai.com".to_string(),
            documentation_url: "https://platform.openai.com/docs".to_string(),
        };

        let capabilities = ProviderCapabilities {
            supports_chat: true,
            supports_vision: true,
            supports_tools: true,
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
                default_openai_models(),
            ),
        }
    }

    /// Create with custom configuration
    pub fn with_config(config: ProviderConfig) -> Self {
        let metadata = ProviderMetadata {
            id: ProviderId::new("openai"),
            name: "OpenAI".to_string(),
            description: "OpenAI GPT models including GPT-4o".to_string(),
            icon: Some("sparkles".to_string()),
            is_local: false,
            requires_api_key: true,
            website_url: "https://openai.com".to_string(),
            documentation_url: "https://platform.openai.com/docs".to_string(),
        };

        let capabilities = ProviderCapabilities {
            supports_chat: true,
            supports_vision: true,
            supports_tools: true,
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
                default_openai_models(),
            ),
        }
    }
}

use async_trait::async_trait;
use crate::provider::AiProvider;

#[async_trait]
impl AiProvider for OpenAiProvider {
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
