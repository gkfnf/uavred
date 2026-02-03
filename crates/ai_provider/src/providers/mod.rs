//! AI Provider Implementations
//!
//! This module contains implementations for various AI providers:
//!
//! - **OpenAI-compatible providers** (via `openai_compatible.rs`):
//!   - Kimi (Moonshot)
//!   - DeepSeek
//!   - OpenAI
//!   - Ollama (local)
//!   - LMStudio (local)
//!   - Codex
//!   - Z.ai (ChatGLM)
//!
//! - **Native API providers**:
//!   - Claude (Anthropic) - native Anthropic API
//!   - Gemini (Google) - native Gemini API

// Base implementation for OpenAI-compatible providers
pub mod openai_compatible;

// Individual provider implementations
pub mod kimi;
pub mod deepseek;
pub mod openai;
pub mod claude;
pub mod gemini;
pub mod ollama;
pub mod lmstudio;
pub mod codex;
pub mod zai;

// Re-export all providers
pub use kimi::KimiProvider;
pub use deepseek::DeepSeekProvider;
pub use openai::OpenAiProvider;
pub use claude::ClaudeProvider;
pub use gemini::GeminiProvider;
pub use ollama::OllamaProvider;
pub use lmstudio::LMStudioProvider;
pub use codex::CodexProvider;
pub use zai::ZaiProvider;

use crate::provider::AiProvider;

/// Get all available provider metadata
pub fn all_provider_metadata() -> Vec<crate::types::ProviderMetadata> {
    vec![
        kimi::KimiProvider::default().metadata(),
        deepseek::DeepSeekProvider::default().metadata(),
        openai::OpenAiProvider::default().metadata(),
        claude::ClaudeProvider::default().metadata(),
        gemini::GeminiProvider::default().metadata(),
        ollama::OllamaProvider::default().metadata(),
        lmstudio::LMStudioProvider::default().metadata(),
        codex::CodexProvider::default().metadata(),
        zai::ZaiProvider::default().metadata(),
    ]
}

/// Get provider by ID
pub fn get_provider_by_id(id: &str) -> Option<Box<dyn crate::provider::AiProvider>> {
    match id {
        "kimi" => Some(Box::new(kimi::KimiProvider::default())),
        "deepseek" => Some(Box::new(deepseek::DeepSeekProvider::default())),
        "openai" => Some(Box::new(openai::OpenAiProvider::default())),
        "claude" => Some(Box::new(claude::ClaudeProvider::default())),
        "gemini" => Some(Box::new(gemini::GeminiProvider::default())),
        "ollama" => Some(Box::new(ollama::OllamaProvider::default())),
        "lmstudio" => Some(Box::new(lmstudio::LMStudioProvider::default())),
        "codex" => Some(Box::new(codex::CodexProvider::default())),
        "zai" => Some(Box::new(zai::ZaiProvider::default())),
        _ => None,
    }
}

/// Provider category
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderCategory {
    Cloud,
    Local,
    Enterprise,
}

impl ProviderCategory {
    /// Get the category for a provider ID
    pub fn for_provider(id: &str) -> Option<Self> {
        match id {
            "kimi" | "deepseek" | "openai" | "claude" | "gemini" | "codex" | "zai" => {
                Some(ProviderCategory::Cloud)
            }
            "ollama" | "lmstudio" => Some(ProviderCategory::Local),
            _ => None,
        }
    }
}

impl std::fmt::Display for ProviderCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProviderCategory::Cloud => write!(f, "Cloud"),
            ProviderCategory::Local => write!(f, "Local"),
            ProviderCategory::Enterprise => write!(f, "Enterprise"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_providers_exist() {
        let metadata = all_provider_metadata();
        assert_eq!(metadata.len(), 9);

        let ids: Vec<_> = metadata.iter().map(|m| m.id.as_str()).collect();
        assert!(ids.contains(&"kimi"));
        assert!(ids.contains(&"deepseek"));
        assert!(ids.contains(&"openai"));
        assert!(ids.contains(&"claude"));
        assert!(ids.contains(&"gemini"));
        assert!(ids.contains(&"ollama"));
        assert!(ids.contains(&"lmstudio"));
        assert!(ids.contains(&"codex"));
        assert!(ids.contains(&"zai"));
    }

    #[test]
    fn test_get_provider_by_id() {
        assert!(get_provider_by_id("kimi").is_some());
        assert!(get_provider_by_id("openai").is_some());
        assert!(get_provider_by_id("unknown").is_none());
    }

    #[test]
    fn test_provider_categories() {
        assert_eq!(ProviderCategory::for_provider("openai"), Some(ProviderCategory::Cloud));
        assert_eq!(ProviderCategory::for_provider("ollama"), Some(ProviderCategory::Local));
        assert_eq!(ProviderCategory::for_provider("unknown"), None);
    }
}
