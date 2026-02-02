//! AI Provider Architecture
//! 
//! Based on Zed's language model provider pattern.
//! Each provider implements the `AiProvider` trait for unified interface.

use gpui::*;
use gpui_component::IconName;
use std::sync::Arc;

pub mod kimi;

/// Provider ID for type identification
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ProviderId(pub String);

impl ProviderId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
    
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Provider capabilities
#[derive(Clone, Debug, Default)]
pub struct ProviderCapabilities {
    pub supports_chat: bool,
    pub supports_vision: bool,
    pub supports_tools: bool,
    pub supports_streaming: bool,
}

/// API Key state management
#[derive(Clone, Debug)]
pub enum ApiKeyState {
    NotConfigured,
    Configured(String),  // API key stored securely
    FromEnv(String),     // API key from environment variable
}

impl ApiKeyState {
    pub fn is_configured(&self) -> bool {
        matches!(self, Self::Configured(_) | Self::FromEnv(_))
    }
    
    pub fn key(&self) -> Option<&str> {
        match self {
            Self::Configured(key) | Self::FromEnv(key) => Some(key),
            _ => None,
        }
    }
}

/// Model information
#[derive(Clone, Debug)]
pub struct ModelInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub max_tokens: u32,
    pub capabilities: ProviderCapabilities,
}

/// Provider trait - main interface for AI providers
pub trait AiProvider: Send + Sync {
    /// Provider ID
    fn id(&self) -> ProviderId;
    
    /// Provider display name
    fn name(&self) -> &str;
    
    /// Provider description
    fn description(&self) -> &str;
    
    /// Icon name for UI
    fn icon(&self) -> IconName;
    
    /// Check if provider is authenticated
    fn is_authenticated(&self) -> bool;
    
    /// Get API key state
    fn api_key_state(&self) -> &ApiKeyState;
    
    /// Set API key
    fn set_api_key(&mut self, key: Option<String>);
    
    /// Get available models
    fn models(&self) -> Vec<ModelInfo>;
    
    /// Get provider capabilities
    fn capabilities(&self) -> ProviderCapabilities;
    
    /// Get configuration view
    fn configuration_view(&self, window: &mut Window, cx: &mut App) -> AnyView;
    
    /// Test connection to provider
    fn test_connection(&self, cx: &App) -> Task<Result<String, String>>;
    
    /// Fetch available models from API
    fn fetch_models(&self, cx: &App) -> Task<Result<Vec<ModelInfo>, String>>;
}

/// Provider registry
pub struct ProviderRegistry {
    providers: Vec<Arc<dyn AiProvider>>,
}

impl ProviderRegistry {
    pub fn new() -> Self {
        Self {
            providers: Vec::new(),
        }
    }
    
    pub fn register(&mut self, provider: Arc<dyn AiProvider>) {
        self.providers.push(provider);
    }
    
    pub fn get(&self, id: &ProviderId) -> Option<Arc<dyn AiProvider>> {
        self.providers.iter()
            .find(|p| p.id() == *id)
            .cloned()
    }
    
    pub fn list(&self) -> &[Arc<dyn AiProvider>] {
        &self.providers
    }
    
    pub fn authenticated_providers(&self) -> Vec<Arc<dyn AiProvider>> {
        self.providers.iter()
            .filter(|p| p.is_authenticated())
            .cloned()
            .collect()
    }
}

impl Default for ProviderRegistry {
    fn default() -> Self {
        Self::new()
    }
}
