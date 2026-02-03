//! Ollama Provider
//!
//! Local AI model provider using Ollama.
//! Docs: https://github.com/ollama/ollama/blob/main/docs/api.md

use crate::providers::openai_compatible::OpenAiCompatibleProvider;
use crate::types::*;

/// Ollama API base URL (local)
pub const OLLAMA_API_URL: &str = "http://localhost:11434";

/// Default models for Ollama
pub fn default_ollama_models() -> Vec<ModelInfo> {
    vec![
        ModelInfo {
            id: ModelId::new("llama3.2"),
            name: "Llama 3.2".to_string(),
            description: Some("Meta's Llama 3.2 model".to_string()),
            provider: ProviderId::new("ollama"),
            max_tokens: 128_000,
            capabilities: ProviderCapabilities {
                supports_chat: true,
                supports_vision: false,
                supports_tools: false,
                supports_streaming: true,
                supports_reasoning: false,
                supports_embeddings: true,
                max_context_length: Some(128_000),
            },
            pricing: None,
            metadata: Default::default(),
        },
        ModelInfo {
            id: ModelId::new("llama3.2-vision"),
            name: "Llama 3.2 Vision".to_string(),
            description: Some("Llama 3.2 with vision capabilities".to_string()),
            provider: ProviderId::new("ollama"),
            max_tokens: 128_000,
            capabilities: ProviderCapabilities {
                supports_chat: true,
                supports_vision: true,
                supports_tools: false,
                supports_streaming: true,
                supports_reasoning: false,
                supports_embeddings: true,
                max_context_length: Some(128_000),
            },
            pricing: None,
            metadata: Default::default(),
        },
        ModelInfo {
            id: ModelId::new("qwen2.5"),
            name: "Qwen 2.5".to_string(),
            description: Some("Alibaba's Qwen 2.5 model".to_string()),
            provider: ProviderId::new("ollama"),
            max_tokens: 128_000,
            capabilities: ProviderCapabilities {
                supports_chat: true,
                supports_vision: false,
                supports_tools: false,
                supports_streaming: true,
                supports_reasoning: false,
                supports_embeddings: true,
                max_context_length: Some(128_000),
            },
            pricing: None,
            metadata: Default::default(),
        },
        ModelInfo {
            id: ModelId::new("mistral"),
            name: "Mistral".to_string(),
            description: Some("Mistral AI's base model".to_string()),
            provider: ProviderId::new("ollama"),
            max_tokens: 32_000,
            capabilities: ProviderCapabilities {
                supports_chat: true,
                supports_vision: false,
                supports_tools: false,
                supports_streaming: true,
                supports_reasoning: false,
                supports_embeddings: true,
                max_context_length: Some(32_000),
            },
            pricing: None,
            metadata: Default::default(),
        },
        ModelInfo {
            id: ModelId::new("codellama"),
            name: "CodeLlama".to_string(),
            description: Some("Meta's code-focused model".to_string()),
            provider: ProviderId::new("ollama"),
            max_tokens: 16_000,
            capabilities: ProviderCapabilities {
                supports_chat: true,
                supports_vision: false,
                supports_tools: false,
                supports_streaming: true,
                supports_reasoning: false,
                supports_embeddings: true,
                max_context_length: Some(16_000),
            },
            pricing: None,
            metadata: Default::default(),
        },
    ]
}

/// Ollama provider
pub struct OllamaProvider {
    inner: OpenAiCompatibleProvider,
}

impl Default for OllamaProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl OllamaProvider {
    /// Create a new Ollama provider with default configuration
    pub fn new() -> Self {
        let config = ProviderConfig {
            provider_id: ProviderId::new("ollama"),
            name: "Ollama".to_string(),
            endpoint: OLLAMA_API_URL.to_string(),
            api_key: None, // Local, no API key needed
            organization_id: None,
            project_id: None,
            region: None,
            timeout_seconds: 120, // Longer timeout for local models
            max_retries: 1,
            custom_headers: Default::default(),
            enabled: false,
        };

        let metadata = ProviderMetadata {
            id: ProviderId::new("ollama"),
            name: "Ollama".to_string(),
            description: "Run AI models locally on your machine".to_string(),
            icon: Some("cpu".to_string()),
            is_local: true,
            requires_api_key: false,
            website_url: "https://ollama.com".to_string(),
            documentation_url: "https://github.com/ollama/ollama".to_string(),
        };

        let capabilities = ProviderCapabilities {
            supports_chat: true,
            supports_vision: true,
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
                default_ollama_models(),
            ),
        }
    }

    /// Create with custom configuration
    pub fn with_config(config: ProviderConfig) -> Self {
        let metadata = ProviderMetadata {
            id: ProviderId::new("ollama"),
            name: "Ollama".to_string(),
            description: "Run AI models locally on your machine".to_string(),
            icon: Some("cpu".to_string()),
            is_local: true,
            requires_api_key: false,
            website_url: "https://ollama.com".to_string(),
            documentation_url: "https://github.com/ollama/ollama".to_string(),
        };

        let capabilities = ProviderCapabilities {
            supports_chat: true,
            supports_vision: true,
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
                default_ollama_models(),
            ),
        }
    }

    /// Check if Ollama is running locally
    pub async fn is_running(&self) -> bool {
        match self.inner.get_latency().await {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    /// Pull a model from Ollama registry
    pub async fn pull_model(&self, model_name: &str) -> Result<(), ApiError> {
        let body = serde_json::json!({
            "name": model_name,
            "stream": false
        });

        self.inner
            .client
            .post("/api/pull", body)
            .await
            .map(|_| ())
    }
}

use async_trait::async_trait;
use crate::provider::AiProvider;

#[async_trait]
impl AiProvider for OllamaProvider {
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
