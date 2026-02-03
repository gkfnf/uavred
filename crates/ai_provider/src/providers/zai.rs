//! Z.ai Provider
//!
//! Z.ai (ChatGLM / Zhipu AI) API provider with OpenAI-compatible interface.
//! Docs: https://open.bigmodel.cn/dev/howuse/introduction

use crate::providers::openai_compatible::OpenAiCompatibleProvider;
use crate::types::*;

/// Z.ai (ChatGLM) API base URL
pub const ZAI_API_URL: &str = "https://open.bigmodel.cn/api/paas";

/// Default models for Z.ai
pub fn default_zai_models() -> Vec<ModelInfo> {
    vec![
        ModelInfo {
            id: ModelId::new("glm-4"),
            name: "GLM-4".to_string(),
            description: Some("Zhipu AI's GLM-4 general purpose model".to_string()),
            provider: ProviderId::new("zai"),
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
                input_price_per_1k: 0.10,
                output_price_per_1k: 0.10,
                currency: "CNY".to_string(),
            }),
            metadata: Default::default(),
        },
        ModelInfo {
            id: ModelId::new("glm-4-plus"),
            name: "GLM-4 Plus".to_string(),
            description: Some("Enhanced GLM-4 with improved capabilities".to_string()),
            provider: ProviderId::new("zai"),
            max_tokens: 128_000,
            capabilities: ProviderCapabilities {
                supports_chat: true,
                supports_vision: true,
                supports_tools: true,
                supports_streaming: true,
                supports_reasoning: true,
                supports_embeddings: false,
                max_context_length: Some(128_000),
            },
            pricing: Some(ModelPricing {
                input_price_per_1k: 0.50,
                output_price_per_1k: 0.50,
                currency: "CNY".to_string(),
            }),
            metadata: Default::default(),
        },
        ModelInfo {
            id: ModelId::new("glm-4-flash"),
            name: "GLM-4 Flash".to_string(),
            description: Some("Fast and cost-effective model".to_string()),
            provider: ProviderId::new("zai"),
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
                input_price_per_1k: 0.01,
                output_price_per_1k: 0.01,
                currency: "CNY".to_string(),
            }),
            metadata: Default::default(),
        },
        ModelInfo {
            id: ModelId::new("glm-4v"),
            name: "GLM-4V".to_string(),
            description: Some("Vision-enabled model for image understanding".to_string()),
            provider: ProviderId::new("zai"),
            max_tokens: 8_000,
            capabilities: ProviderCapabilities {
                supports_chat: true,
                supports_vision: true,
                supports_tools: false,
                supports_streaming: true,
                supports_reasoning: false,
                supports_embeddings: false,
                max_context_length: Some(8_000),
            },
            pricing: Some(ModelPricing {
                input_price_per_1k: 0.05,
                output_price_per_1k: 0.05,
                currency: "CNY".to_string(),
            }),
            metadata: Default::default(),
        },
        ModelInfo {
            id: ModelId::new("chatglm3-6b"),
            name: "ChatGLM3-6B".to_string(),
            description: Some("Open source 6B parameter model".to_string()),
            provider: ProviderId::new("zai"),
            max_tokens: 32_000,
            capabilities: ProviderCapabilities {
                supports_chat: true,
                supports_vision: false,
                supports_tools: false,
                supports_streaming: true,
                supports_reasoning: false,
                supports_embeddings: false,
                max_context_length: Some(32_000),
            },
            pricing: Some(ModelPricing {
                input_price_per_1k: 0.005,
                output_price_per_1k: 0.005,
                currency: "CNY".to_string(),
            }),
            metadata: Default::default(),
        },
    ]
}

/// Z.ai provider
pub struct ZaiProvider {
    inner: OpenAiCompatibleProvider,
}

impl Default for ZaiProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl ZaiProvider {
    /// Create a new Z.ai provider with default configuration
    pub fn new() -> Self {
        let config = ProviderConfig {
            provider_id: ProviderId::new("zai"),
            name: "Z.ai".to_string(),
            endpoint: ZAI_API_URL.to_string(),
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
            id: ProviderId::new("zai"),
            name: "Z.ai".to_string(),
            description: "Zhipu AI (ChatGLM) models - Made in China".to_string(),
            icon: Some("z".to_string()),
            is_local: false,
            requires_api_key: true,
            website_url: "https://z.ai".to_string(),
            documentation_url: "https://open.bigmodel.cn".to_string(),
        };

        let capabilities = ProviderCapabilities {
            supports_chat: true,
            supports_vision: true,
            supports_tools: true,
            supports_streaming: true,
            supports_reasoning: false,
            supports_embeddings: false,
            max_context_length: Some(128_000),
        };

        Self {
            inner: OpenAiCompatibleProvider::new(
                config,
                metadata,
                capabilities,
                default_zai_models(),
            ),
        }
    }

    /// Create with custom configuration
    pub fn with_config(config: ProviderConfig) -> Self {
        let metadata = ProviderMetadata {
            id: ProviderId::new("zai"),
            name: "Z.ai".to_string(),
            description: "Zhipu AI (ChatGLM) models - Made in China".to_string(),
            icon: Some("z".to_string()),
            is_local: false,
            requires_api_key: true,
            website_url: "https://z.ai".to_string(),
            documentation_url: "https://open.bigmodel.cn".to_string(),
        };

        let capabilities = ProviderCapabilities {
            supports_chat: true,
            supports_vision: true,
            supports_tools: true,
            supports_streaming: true,
            supports_reasoning: false,
            supports_embeddings: false,
            max_context_length: Some(128_000),
        };

        Self {
            inner: OpenAiCompatibleProvider::new(
                config,
                metadata,
                capabilities,
                default_zai_models(),
            ),
        }
    }

    /// Get the recommended model for a task
    pub fn recommended_model_for_task(&self, task: &str, requires_vision: bool) -> ModelId {
        let task_lower = task.to_lowercase();

        if requires_vision {
            return ModelId::new("glm-4v");
        }

        if task_lower.contains("reasoning") || task_lower.contains("complex") {
            ModelId::new("glm-4-plus")
        } else if task_lower.contains("fast") || task_lower.contains("quick") {
            ModelId::new("glm-4-flash")
        } else {
            ModelId::new("glm-4")
        }
    }
}

use async_trait::async_trait;
use crate::provider::AiProvider;

#[async_trait]
impl AiProvider for ZaiProvider {
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
