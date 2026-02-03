//! AI Provider Integration Crate
//!
//! A unified interface layer for integrating multiple AI providers including:
//! - LMStudio (local)
//! - Ollama (local)
//! - Z.ai (ChatGLM)
//! - Kimi (Moonshot)
//! - DeepSeek
//! - Codex (OpenAI)
//! - Gemini (Google)
//! - Claude (Anthropic)
//! - OpenAI
//!
//! ## Quick Start
//!
//! ```no_run
//! use ai_provider::{ProviderRegistry, ChatCompletionRequest, ChatMessage};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     // Create registry with all default providers
//!     let registry = ProviderRegistry::with_defaults();
//!
//!     // Get a provider
//!     let provider = registry.get(&"kimi".into())
//!         .expect("Provider not found");
//!
//!     // Test connection
//!     let result = provider.test_connection().await?;
//!     println!("Connection: {:?}", result);
//!
//!     // Chat completion
//!     let request = ChatCompletionRequest::new(
//!         "kimi-k2.5".into(),
//!         vec![
//!             ChatMessage::system("You are a helpful assistant."),
//!             ChatMessage::user("Hello!"),
//!         ],
//!     );
//!
//!     let response = provider.chat_completion(request).await?;
//!     println!("Response: {}", response.content);
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Architecture
//!
//! The crate is organized into several modules:
//!
//! - `types`: Core types for messages, requests, responses, etc.
//! - `provider`: The `AiProvider` trait and `ProviderRegistry`
//! - `providers`: Individual provider implementations
//! - `http_client`: HTTP client with retry logic and error handling

// Core modules
pub mod types;
pub mod provider;
pub mod http_client;
pub mod providers;

// Re-export main types
pub use types::*;
pub use provider::{AiProvider, ProviderRegistry, ProviderBuilder};
pub use http_client::AiHttpClient;

// Re-export providers
pub use providers::{
    KimiProvider, DeepSeekProvider, OpenAiProvider, ClaudeProvider,
    GeminiProvider, OllamaProvider, LMStudioProvider, CodexProvider, ZaiProvider,
};

/// Crate version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Get a list of all supported provider IDs
pub fn supported_providers() -> Vec<&'static str> {
    vec![
        "kimi",
        "deepseek",
        "openai",
        "claude",
        "gemini",
        "ollama",
        "lmstudio",
        "codex",
        "zai",
    ]
}

/// Check if a provider ID is supported
pub fn is_provider_supported(id: &str) -> bool {
    supported_providers().contains(&id)
}

/// Create a default provider registry with all built-in providers
pub fn default_registry() -> ProviderRegistry {
    ProviderRegistry::with_defaults()
}

/// Utility functions
pub mod utils {
    /// Format latency for display
    pub fn format_latency(latency_ms: u64) -> String {
        if latency_ms < 1000 {
            format!("{} ms", latency_ms)
        } else {
            format!("{:.1} s", latency_ms as f64 / 1000.0)
        }
    }

    /// Estimate token count (rough approximation)
    pub fn estimate_token_count(text: &str) -> u32 {
        // Rough estimate: 1 token ≈ 4 characters for English
        // For mixed content, this is a conservative estimate
        (text.len() as f64 / 3.5).ceil() as u32
    }

    /// Calculate cost based on token usage and pricing
    pub fn calculate_cost(
        prompt_tokens: u32,
        completion_tokens: u32,
        input_price: f64,
        output_price: f64,
    ) -> f64 {
        let input_cost = (prompt_tokens as f64 / 1000.0) * input_price;
        let output_cost = (completion_tokens as f64 / 1000.0) * output_price;
        input_cost + output_cost
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_supported_providers() {
        let providers = supported_providers();
        assert_eq!(providers.len(), 9);
        assert!(providers.contains(&"kimi"));
        assert!(providers.contains(&"openai"));
        assert!(providers.contains(&"claude"));
    }

    #[test]
    fn test_is_provider_supported() {
        assert!(is_provider_supported("kimi"));
        assert!(is_provider_supported("openai"));
        assert!(!is_provider_supported("unknown"));
    }

    #[test]
    fn test_format_latency() {
        assert_eq!(utils::format_latency(500), "500 ms");
        assert_eq!(utils::format_latency(1500), "1.5 s");
    }

    #[test]
    fn test_estimate_token_count() {
        // Rough estimate test
        let text = "Hello world";
        let tokens = utils::estimate_token_count(text);
        assert!(tokens > 0);
    }
}
