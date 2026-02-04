//! AI Provider Architecture
//! 
//! This module provides a unified interface for all AI providers.
//! 
//! ## Architecture
//! 
//! ```text
//! ┌─────────────────────────────────────┐
//! │       AiSettingsPanel               │
//! │  (UI component in ai_settings.rs)   │
//! └──────────────┬──────────────────────┘
//!                │ uses
//!                ▼
//! ┌─────────────────────────────────────┐
//! │  KimiProvider / DeepSeekProvider    │
//! │  (implement AiProvider trait)       │
//! └──────────────┬──────────────────────┘
//!                │ calls
//!                ▼
//! ┌─────────────────────────────────────┐
//! │       AiProviderClient              │
//! │  (unified HTTP client)              │
//! └──────────────┬──────────────────────┘
//!                │ HTTP requests
//!                ▼
//! ┌─────────────────────────────────────┐
//! │   DeepSeek API / Kimi API / etc     │
//! └─────────────────────────────────────┘
//! ```

pub mod ai_client;
pub mod deepseek;
pub mod kimi;

pub use ai_client::{AiProvider, AiProviderClient, ApiModel, ProviderApiConfig, ModelInfo, ProviderCapabilities};
pub use deepseek::DeepSeekProvider;
pub use kimi::KimiProvider;

use std::sync::Arc;

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

/// Provider registry - manages all available providers
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
            .find(|p| p.provider_id() == id.as_str())
            .cloned()
    }
    
    pub fn list(&self) -> &[Arc<dyn AiProvider>] {
        &self.providers
    }
}

impl Default for ProviderRegistry {
    fn default() -> Self {
        Self::new()
    }
}
