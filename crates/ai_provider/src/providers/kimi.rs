//! Kimi (Moonshot) Provider
//!
//! Moonshot AI API provider with OpenAI-compatible interface.
//! Docs: https://platform.moonshot.cn/docs

use crate::providers::openai_compatible::OpenAiCompatibleProvider;
use crate::types::*;

/// Kimi API base URL
pub const KIMI_API_URL: &str = "https://api.moonshot.cn";

/// Default models for Kimi
pub fn default_kimi_models() -> Vec<ModelInfo> {
    vec![
        ModelInfo {
            id: ModelId::new("kimi-k2.5"),
            name: "Kimi K2.5".to_string(),
            description: Some("Latest Kimi model with superior performance and reasoning capabilities".to_string()),
            provider: ProviderId::new("kimi"),
            max_tokens: 256_000,
            capabilities: ProviderCapabilities {
                supports_chat: true,
                supports_vision: true,
                supports_tools: true,
                supports_streaming: true,
                supports_reasoning: true,
                supports_embeddings: false,
                max_context_length: Some(256_000),
            },
            pricing: Some(ModelPricing {
                input_price_per_1k: 0.5,
                output_price_per_1k: 2.0,
                currency: "CNY".to_string(),
            }),
            metadata: Default::default(),
        },
        ModelInfo {
            id: ModelId::new("moonshot-v1-128k"),
            name: "Moonshot v1 128K".to_string(),
            description: Some("Long context model with 128K token limit".to_string()),
            provider: ProviderId::new("kimi"),
            max_tokens: 128_000,
            capabilities: ProviderCapabilities {
                supports_chat: true,
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                supports_reasoning: false,
                supports_embeddings: false,
                max_context_length: Some(128_000),
            },
            pricing: Some(ModelPricing {
                input_price_per_1k: 0.6,
                output_price_per_1k: 0.6,
                currency: "CNY".to_string(),
            }),
            metadata: Default::default(),
        },
        ModelInfo {
            id: ModelId::new("moonshot-v1-32k"),
            name: "Moonshot v1 32K".to_string(),
            description: Some("Standard model with 32K token limit".to_string()),
            provider: ProviderId::new("kimi"),
            max_tokens: 32_000,
            capabilities: ProviderCapabilities {
                supports_chat: true,
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                supports_reasoning: false,
                supports_embeddings: false,
                max_context_length: Some(32_000),
            },
            pricing: Some(ModelPricing {
                input_price_per_1k: 0.24,
                output_price_per_1k: 0.24,
                currency: "CNY".to_string(),
            }),
            metadata: Default::default(),
        },
        ModelInfo {
            id: ModelId::new("moonshot-v1-8k"),
            name: "Moonshot v1 8K".to_string(),
            description: Some("Fast and economical model with 8K token limit".to_string()),
            provider: ProviderId::new("kimi"),
            max_tokens: 8_000,
            capabilities: ProviderCapabilities {
                supports_chat: true,
                supports_vision: false,
                supports_tools: false,
                supports_streaming: true,
                supports_reasoning: false,
                supports_embeddings: false,
                max_context_length: Some(8_000),
            },
            pricing: Some(ModelPricing {
                input_price_per_1k: 0.06,
                output_price_per_1k: 0.06,
                currency: "CNY".to_string(),
            }),
            metadata: Default::default(),
        },
    ]
}

/// Kimi provider
pub struct KimiProvider {
    inner: OpenAiCompatibleProvider,
}

impl Default for KimiProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl KimiProvider {
    /// Create a new Kimi provider with default configuration
    pub fn new() -> Self {
        let config = ProviderConfig {
            provider_id: ProviderId::new("kimi"),
            name: "Kimi (Moonshot)".to_string(),
            endpoint: KIMI_API_URL.to_string(),
            api_key: None,
            organization_id: None,
            project_id: None,
            region: Some("china".to_string()),
            timeout_seconds: 60,
            max_retries: 3,
            custom_headers: Default::default(),
            enabled: false,
        };

        let metadata = ProviderMetadata {
            id: ProviderId::new("kimi"),
            name: "Kimi (Moonshot)".to_string(),
            description: "Moonshot AI with long context and strong reasoning capabilities".to_string(),
            icon: Some("bot".to_string()),
            is_local: false,
            requires_api_key: true,
            website_url: "https://www.moonshot.cn".to_string(),
            documentation_url: "https://platform.moonshot.cn/docs".to_string(),
        };

        let capabilities = ProviderCapabilities {
            supports_chat: true,
            supports_vision: true,
            supports_tools: true,
            supports_streaming: true,
            supports_reasoning: true,
            supports_embeddings: false,
            max_context_length: Some(256_000),
        };

        Self {
            inner: OpenAiCompatibleProvider::new(
                config,
                metadata,
                capabilities,
                default_kimi_models(),
            ),
        }
    }

    /// Create with custom configuration
    pub fn with_config(config: ProviderConfig) -> Self {
        let metadata = ProviderMetadata {
            id: ProviderId::new("kimi"),
            name: "Kimi (Moonshot)".to_string(),
            description: "Moonshot AI with long context and strong reasoning capabilities".to_string(),
            icon: Some("bot".to_string()),
            is_local: false,
            requires_api_key: true,
            website_url: "https://www.moonshot.cn".to_string(),
            documentation_url: "https://platform.moonshot.cn/docs".to_string(),
        };

        let capabilities = ProviderCapabilities {
            supports_chat: true,
            supports_vision: true,
            supports_tools: true,
            supports_streaming: true,
            supports_reasoning: true,
            supports_embeddings: false,
            max_context_length: Some(256_000),
        };

        Self {
            inner: OpenAiCompatibleProvider::new(
                config,
                metadata,
                capabilities,
                default_kimi_models(),
            ),
        }
    }
}

use async_trait::async_trait;
use crate::provider::AiProvider;

#[async_trait]
impl AiProvider for KimiProvider {
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
