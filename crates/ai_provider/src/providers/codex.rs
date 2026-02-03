//! OpenAI Codex Provider
//!
//! OpenAI Codex - AI model for coding tasks.
//! Uses OpenAI's API with Codex-specific models.
//! Docs: https://platform.openai.com/docs/guides/codex

use crate::providers::openai_compatible::OpenAiCompatibleProvider;
use crate::types::*;

/// OpenAI API base URL
pub const OPENAI_API_URL: &str = "https://api.openai.com/v1";

/// Default models for Codex
pub fn default_codex_models() -> Vec<ModelInfo> {
    vec![
        ModelInfo {
            id: ModelId::new("codex-latest"),
            name: "Codex Latest".to_string(),
            description: Some("OpenAI Codex - Latest coding model".to_string()),
            provider: ProviderId::new("codex"),
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
                input_price_per_1k: 0.15,
                output_price_per_1k: 0.60,
                currency: "USD".to_string(),
            }),
            metadata: Default::default(),
        },
        ModelInfo {
            id: ModelId::new("o3-mini"),
            name: "o3 Mini".to_string(),
            description: Some("Fast reasoning model for coding".to_string()),
            provider: ProviderId::new("codex"),
            max_tokens: 200_000,
            capabilities: ProviderCapabilities {
                supports_chat: true,
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                supports_reasoning: true,
                supports_embeddings: false,
                max_context_length: Some(200_000),
            },
            pricing: Some(ModelPricing {
                input_price_per_1k: 1.10,
                output_price_per_1k: 4.40,
                currency: "USD".to_string(),
            }),
            metadata: Default::default(),
        },
        ModelInfo {
            id: ModelId::new("o1"),
            name: "o1".to_string(),
            description: Some("Advanced reasoning model".to_string()),
            provider: ProviderId::new("codex"),
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
                output_price_per_1k: 60.00,
                currency: "USD".to_string(),
            }),
            metadata: Default::default(),
        },
    ]
}

/// Codex provider
pub struct CodexProvider {
    inner: OpenAiCompatibleProvider,
}

impl Default for CodexProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl CodexProvider {
    /// Create a new Codex provider with default configuration
    pub fn new() -> Self {
        let config = ProviderConfig {
            provider_id: ProviderId::new("codex"),
            name: "Codex".to_string(),
            endpoint: OPENAI_API_URL.to_string(),
            api_key: None,
            organization_id: None,
            project_id: None,
            region: Some("international".to_string()),
            timeout_seconds: 120,
            max_retries: 3,
            custom_headers: Default::default(),
            enabled: false,
        };

        let metadata = ProviderMetadata {
            id: ProviderId::new("codex"),
            name: "OpenAI Codex".to_string(),
            description: "AI models specialized for coding tasks".to_string(),
            icon: Some("code".to_string()),
            is_local: false,
            requires_api_key: true,
            website_url: "https://openai.com/codex".to_string(),
            documentation_url: "https://platform.openai.com/docs/guides/codex".to_string(),
        };

        let capabilities = ProviderCapabilities {
            supports_chat: true,
            supports_vision: true,
            supports_tools: true,
            supports_streaming: true,
            supports_reasoning: true,
            supports_embeddings: false,
            max_context_length: Some(200_000),
        };

        Self {
            inner: OpenAiCompatibleProvider::new(
                config,
                metadata,
                capabilities,
                default_codex_models(),
            ),
        }
    }

    /// Create with custom configuration
    pub fn with_config(config: ProviderConfig) -> Self {
        let metadata = ProviderMetadata {
            id: ProviderId::new("codex"),
            name: "OpenAI Codex".to_string(),
            description: "AI models specialized for coding tasks".to_string(),
            icon: Some("code".to_string()),
            is_local: false,
            requires_api_key: true,
            website_url: "https://openai.com/codex".to_string(),
            documentation_url: "https://platform.openai.com/docs/guides/codex".to_string(),
        };

        let capabilities = ProviderCapabilities {
            supports_chat: true,
            supports_vision: true,
            supports_tools: true,
            supports_streaming: true,
            supports_reasoning: true,
            supports_embeddings: false,
            max_context_length: Some(200_000),
        };

        Self {
            inner: OpenAiCompatibleProvider::new(
                config,
                metadata,
                capabilities,
                default_codex_models(),
            ),
        }
    }

    /// Get the recommended model for a coding task
    pub fn recommended_model_for_task(&self, task: &str) -> ModelId {
        let task_lower = task.to_lowercase();

        if task_lower.contains("reasoning") || task_lower.contains("complex") {
            ModelId::new("o1")
        } else if task_lower.contains("fast") || task_lower.contains("quick") {
            ModelId::new("o3-mini")
        } else {
            ModelId::new("codex-latest")
        }
    }
}

use async_trait::async_trait;
use crate::provider::AiProvider;

#[async_trait]
impl AiProvider for CodexProvider {
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
