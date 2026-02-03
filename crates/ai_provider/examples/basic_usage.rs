//! Basic usage example for ai_provider crate
//!
//! Run with: cargo run --example basic_usage --features gpui-integration

use ai_provider::{
    ChatCompletionRequest, ChatMessage, ProviderRegistry, ProviderBuilder,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("AI Provider Integration Example");
    println!("================================\n");

    // Create a registry with all default providers
    let registry = ProviderRegistry::with_defaults();

    // List all available providers
    println!("Available Providers:");
    for provider in registry.list().iter() {
        let auth_status = if provider.is_authenticated() {
            "✓ authenticated"
        } else {
            "✗ not configured"
        };
        println!(
            "  - {} ({}): {}",
            provider.name(),
            provider.provider_id(),
            auth_status
        );
    }
    println!();

    // Test connection to local providers (Ollama, LMStudio)
    println!("Testing Local Providers:");
    for provider_id in ["ollama", "lmstudio"] {
        if let Some(provider) = registry.get(&provider_id.into()) {
            print!("  Testing {}... ", provider.name());
            match provider.test_connection().await {
                Ok(result) => {
                    if result.success {
                        println!(
                            "✓ connected ({} ms, {} models)",
                            result.latency_ms,
                            result.models_available.unwrap_or(0)
                        );
                    } else {
                        println!("✗ failed: {}", result.message);
                    }
                }
                Err(e) => println!("✗ error: {}", e),
            }
        }
    }
    println!();

    // Show provider capabilities
    println!("Provider Capabilities:");
    for provider in registry.list().iter() {
        let caps = provider.capabilities();
        println!("  {}:", provider.name());
        println!("    - Chat: {}", if caps.supports_chat { "✓" } else { "✗" });
        println!("    - Vision: {}", if caps.supports_vision { "✓" } else { "✗" });
        println!("    - Tools: {}", if caps.supports_tools { "✓" } else { "✗" });
        println!("    - Streaming: {}", if caps.supports_streaming { "✓" } else { "✗" });
    }
    println!();

    // Example: Create a configured provider
    println!("Creating a configured Kimi provider:");
    let config = ProviderBuilder::new("kimi".into())
        .with_endpoint("https://api.moonshot.cn")
        .with_api_key(std::env::var("MOONSHOT_API_KEY").unwrap_or_default())
        .with_timeout(60)
        .enabled(true)
        .build_config();

    println!("  Provider ID: {}", config.provider_id);
    println!("  Endpoint: {}", config.endpoint);
    println!("  Timeout: {}s", config.timeout_seconds);
    println!();

    // Example: Chat completion (requires API key)
    println!("Chat Completion Example:");
    println!("  (Skipped - requires API key)");
    // let request = ChatCompletionRequest::new(
    //     "kimi-k2.5".into(),
    //     vec![
    //         ChatMessage::system("You are a helpful assistant."),
    //         ChatMessage::user("Hello! What can you help me with?"),
    //     ],
    // );
    // let response = provider.chat_completion(request).await?;
    // println!("Response: {}", response.content);

    println!("\nExample completed successfully!");
    Ok(())
}
