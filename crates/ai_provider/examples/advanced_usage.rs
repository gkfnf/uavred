//! Advanced usage example for ai_provider crate
//!
//! Demonstrates:
//! - Provider registry management
//! - Connection testing with latency measurement
//! - Streaming chat completions
//! - Model selection

use ai_provider::{
    ChatCompletionRequest, ChatMessage, ProviderRegistry, ProviderId,
    types::FinishReason,
};
use futures::StreamExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("AI Provider Advanced Example");
    println!("=============================\n");

    // Initialize registry
    let registry = ProviderRegistry::with_defaults();

    // Test all connections and show latency
    println!("Connection Latency Test:");
    println!("------------------------");
    let results = registry.test_all_connections().await;
    for (provider_id, result) in results {
        match result {
            Ok(test_result) => {
                let status = if test_result.success { "✓" } else { "✗" };
                println!(
                    "  {} {}: {} ({} ms)",
                    status,
                    provider_id,
                    test_result.message,
                    test_result.latency_ms
                );
            }
            Err(e) => {
                println!("  ✗ {}: Error - {}", provider_id, e);
            }
        }
    }
    println!();

    // Get all available models from authenticated providers
    println!("Fetching Available Models:");
    println!("--------------------------");
    let all_models = registry.get_all_models().await;
    println!("Found {} models total\n", all_models.len());

    // Group by provider
    let mut by_provider: std::collections::HashMap<String, Vec<_>> = std::collections::HashMap::new();
    for model in all_models {
        by_provider
            .entry(model.provider.as_str().to_string())
            .or_default()
            .push(model);
    }

    for (provider_id, models) in by_provider {
        println!("  {} ({} models):", provider_id, models.len());
        for model in models.iter().take(3) {
            println!(
                "    - {} ({} tokens)",
                model.name, model.max_tokens
            );
        }
        if models.len() > 3 {
            println!("    ... and {} more", models.len() - 3);
        }
    }
    println!();

    // Example: Get provider with lowest latency
    println!("Provider Selection by Latency:");
    println!("-------------------------------");
    let mut latencies: Vec<(String, u64)> = Vec::new();
    for provider in registry.list().iter() {
        if let Ok(latency) = provider.get_latency().await {
            latencies.push((provider.name().to_string(), latency));
        }
    }
    latencies.sort_by_key(|(_, lat)| *lat);

    for (name, latency) in latencies.iter().take(3) {
        println!("  {}: {} ms", name, latency);
    }
    println!();

    // Example: Streaming completion (pseudo-code, requires actual API key)
    println!("Streaming Completion Example:");
    println!("-----------------------------");
    println!("  (Requires configured provider with API key)");
    println!();
    // if let Some(provider) = registry.get(&"kimi".into()) {
    //     let request = ChatCompletionRequest::new(
    //         "kimi-k2.5".into(),
    //         vec![ChatMessage::user("Count from 1 to 5")],
    //     ).with_streaming();
    //
    //     let mut stream = provider.chat_completion_stream(request).await?;
    //     print!("  Response: ");
    //     while let Some(chunk) = stream.next().await {
    //         match chunk {
    //             Ok(chunk) => {
    //                 print!("{}", chunk.content_delta);
    //                 if chunk.finish_reason == Some(FinishReason::Stop) {
    //                     println!();
    //                 }
    //             }
    //             Err(e) => eprintln!("  Error: {}", e),
    //         }
    //     }
    // }

    // Example: Custom provider configuration
    println!("Custom Provider Configuration:");
    println!("-------------------------------");

    // Create a custom OpenAI-compatible provider
    let custom_config = ai_provider::ProviderBuilder::new("custom".into())
        .with_endpoint("https://api.example.com")
        .with_api_key("sk-test-key")
        .with_timeout(30)
        .with_region("us-east-1")
        .enabled(true)
        .build_config();

    println!("  Custom provider config:");
    println!("    ID: {}", custom_config.provider_id);
    println!("    Endpoint: {}", custom_config.endpoint);
    println!("    Region: {:?}", custom_config.region);
    println!("    Timeout: {}s", custom_config.timeout_seconds);
    println!();

    // Example: Compare provider capabilities
    println!("Provider Capability Comparison:");
    println!("--------------------------------");
    let features = vec!["Vision", "Tools", "Streaming", "Embeddings"];
    print!("  {:<15}", "Provider");
    for feature in &features {
        print!(" {:>10}", feature);
    }
    println!();
    println!("  {}", "-".repeat(60));

    for provider in registry.list().iter() {
        let caps = provider.capabilities();
        print!("  {:<15}", provider.name());
        print!(" {:>10}", if caps.supports_vision { "✓" } else { "-" });
        print!(" {:>10}", if caps.supports_tools { "✓" } else { "-" });
        print!(" {:>10}", if caps.supports_streaming { "✓" } else { "-" });
        print!(" {:>10}", if caps.supports_embeddings { "✓" } else { "-" });
        println!();
    }

    println!("\nAdvanced example completed!");
    Ok(())
}
