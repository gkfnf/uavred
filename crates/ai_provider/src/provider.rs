//! AI Provider Core Trait
//!
//! Defines the unified interface that all AI providers must implement.

use async_trait::async_trait;
use futures::stream::BoxStream;
use std::sync::Arc;

use crate::types::*;

/// Core trait for AI providers
///
/// This trait defines the unified interface that all AI providers (OpenAI, Claude, Kimi, etc.)
/// must implement. It provides methods for chat completions, embeddings, model management,
/// and connection testing.
#[async_trait]
pub trait AiProvider: Send + Sync + std::any::Any {
    /// Get provider unique identifier
    fn provider_id(&self) -> ProviderId;

    /// Get provider display name
    fn name(&self) -> &str;

    /// Get provider description
    fn description(&self) -> &str;

    /// Get provider metadata for UI
    fn metadata(&self) -> ProviderMetadata;

    /// Check if provider is properly configured and authenticated
    fn is_authenticated(&self) -> bool;

    /// Get current API key state
    fn api_key_state(&self) -> ApiKeyState;

    /// Set or update API key
    fn set_api_key(&mut self, key: Option<String>);

    /// Get provider capabilities
    fn capabilities(&self) -> ProviderCapabilities;

    /// Get available models
    fn available_models(&self) -> Vec<ModelInfo>;

    /// Fetch models from API (may require network call)
    async fn fetch_models(&self) -> Result<Vec<ModelInfo>, ApiError>;

    /// Test connection to the provider
    async fn test_connection(&self) -> Result<ConnectionTestResult, ApiError>;

    /// Get connection latency (ping test)
    async fn get_latency(&self) -> Result<u64, ApiError>;

    /// Send chat completion request
    async fn chat_completion(
        &self,
        request: ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse, ApiError>;

    /// Send streaming chat completion request
    async fn chat_completion_stream(
        &self,
        request: ChatCompletionRequest,
    ) -> Result<BoxStream<'static, Result<ChatCompletionChunk, ApiError>>, ApiError>;

    /// Create embeddings
    async fn create_embeddings(
        &self,
        request: EmbeddingRequest,
    ) -> Result<EmbeddingResponse, ApiError>;

    /// Get provider configuration
    fn config(&self) -> &ProviderConfig;

    /// Update provider configuration
    fn update_config(&mut self, config: ProviderConfig);

    /// Clone the provider (as trait object)
    fn clone_box(&self) -> Box<dyn AiProvider>;
}

impl Clone for Box<dyn AiProvider> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

/// Provider registry for managing multiple AI providers
pub struct ProviderRegistry {
    providers: parking_lot::RwLock<Arc<Vec<Arc<dyn AiProvider>>>>,
    default_provider: parking_lot::RwLock<Option<ProviderId>>,
}

impl ProviderRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self {
            providers: parking_lot::RwLock::new(Arc::new(Vec::new())),
            default_provider: parking_lot::RwLock::new(None),
        }
    }

    /// Create a registry with all built-in providers
    pub fn with_defaults() -> Self {
        let registry = Self::new();
        registry.register_defaults();
        registry
    }

    /// Register a provider
    pub fn register(&self, provider: Arc<dyn AiProvider>) {
        let mut providers = self.providers.write();
        let list = Arc::make_mut(&mut providers);

        // Replace if exists
        if let Some(idx) = list.iter().position(|p| p.provider_id() == provider.provider_id()) {
            list[idx] = provider;
        } else {
            list.push(provider);
        }
    }

    /// Register default providers
    pub fn register_defaults(&self) {
        use crate::providers::*;

        // Register all built-in providers
        self.register(Arc::new(KimiProvider::default()));
        self.register(Arc::new(DeepSeekProvider::default()));
        self.register(Arc::new(OpenAiProvider::default()));
        self.register(Arc::new(ClaudeProvider::default()));
        self.register(Arc::new(GeminiProvider::default()));
        self.register(Arc::new(OllamaProvider::default()));
        self.register(Arc::new(LMStudioProvider::default()));
        self.register(Arc::new(CodexProvider::default()));
        self.register(Arc::new(ZaiProvider::default()));
    }

    /// Get a provider by ID
    pub fn get(&self, id: &ProviderId) -> Option<Arc<dyn AiProvider>> {
        self.providers
            .read()
            .iter()
            .find(|p| p.provider_id() == *id)
            .cloned()
    }

    /// Get all providers
    pub fn get_all(&self) -> Vec<Arc<dyn AiProvider>> {
        self.providers.read().iter().cloned().collect()
    }

    /// List all registered providers
    pub fn list(&self) -> Arc<Vec<Arc<dyn AiProvider>>> {
        self.providers.read().clone()
    }

    /// List authenticated providers
    pub fn list_authenticated(&self) -> Vec<Arc<dyn AiProvider>> {
        self.providers
            .read()
            .iter()
            .filter(|p| p.is_authenticated())
            .cloned()
            .collect()
    }

    /// List enabled providers
    pub fn list_enabled(&self) -> Vec<Arc<dyn AiProvider>> {
        self.providers
            .read()
            .iter()
            .filter(|p| p.config().enabled)
            .cloned()
            .collect()
    }

    /// Remove a provider
    pub fn remove(&self, id: &ProviderId) -> Option<Arc<dyn AiProvider>> {
        let mut providers = self.providers.write();
        let list = Arc::make_mut(&mut providers);
        list.iter()
            .position(|p| p.provider_id() == *id)
            .map(|idx| list.remove(idx))
    }

    /// Set default provider
    pub fn set_default(&self, id: Option<ProviderId>) {
        *self.default_provider.write() = id;
    }

    /// Get default provider
    pub fn get_default(&self) -> Option<Arc<dyn AiProvider>> {
        let default_id = self.default_provider.read().clone()?;
        self.get(&default_id)
    }

    /// Test all connections and return results
    pub async fn test_all_connections(&self) -> Vec<(ProviderId, Result<ConnectionTestResult, ApiError>)> {
        let providers = self.list();
        let mut results = Vec::new();

        for provider in providers.iter() {
            let result = provider.test_connection().await;
            results.push((provider.provider_id(), result));
        }

        results
    }

    /// Get all available models from all authenticated providers
    pub async fn get_all_models(&self) -> Vec<ModelInfo> {
        let mut all_models = Vec::new();
        let providers = self.list_authenticated();

        for provider in providers {
            match provider.fetch_models().await {
                Ok(models) => all_models.extend(models),
                Err(e) => {
                    tracing::warn!(
                        "Failed to fetch models from {}: {}",
                        provider.name(),
                        e
                    );
                }
            }
        }

        all_models
    }
}

impl Default for ProviderRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Provider builder for creating configured providers
pub struct ProviderBuilder {
    config: ProviderConfig,
}

impl ProviderBuilder {
    pub fn new(provider_id: ProviderId) -> Self {
        Self {
            config: ProviderConfig {
                provider_id,
                ..Default::default()
            },
        }
    }

    pub fn with_endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.config.endpoint = endpoint.into();
        self
    }

    pub fn with_api_key(mut self, api_key: impl Into<String>) -> Self {
        self.config.api_key = Some(api_key.into());
        self
    }

    pub fn with_timeout(mut self, seconds: u64) -> Self {
        self.config.timeout_seconds = seconds;
        self
    }

    pub fn with_region(mut self, region: impl Into<String>) -> Self {
        self.config.region = Some(region.into());
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.config.enabled = enabled;
        self
    }

    pub fn build_config(self) -> ProviderConfig {
        self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_id() {
        let id = ProviderId::new("test");
        assert_eq!(id.as_str(), "test");
    }

    #[test]
    fn test_model_id() {
        let id = ModelId::new("gpt-4");
        assert_eq!(id.as_str(), "gpt-4");
    }

    #[test]
    fn test_api_key_state() {
        let not_configured = ApiKeyState::NotConfigured;
        assert!(!not_configured.is_configured());

        let configured = ApiKeyState::Configured("key".to_string());
        assert!(configured.is_configured());
        assert_eq!(configured.key(), Some("key"));
    }
}
