//! AI Provider API Client - Unified HTTP client for all AI providers
//!
//! This module provides a unified HTTP client for making API calls to various AI providers.
//! All provider implementations should use this client for consistency.

use gpui::*;
use serde::{Deserialize, Serialize};

/// Model information from API
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApiModel {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_length: Option<u32>,
}

/// API response for models list
#[derive(Debug, Clone, Deserialize)]
pub struct ModelsResponse {
    pub data: Vec<ApiModel>,
}

/// Provider API configuration
#[derive(Clone, Debug)]
pub struct ProviderApiConfig {
    pub base_url: String,
    pub api_key: String,
}

/// Unified AI Provider API Client
pub struct AiProviderClient {
    client: reqwest::Client,
}

impl AiProviderClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    /// Fetch models from any OpenAI-compatible API
    pub async fn fetch_models(&self, config: &ProviderApiConfig) -> anyhow::Result<Vec<ApiModel>> {
        let url = format!("{}/v1/models", config.base_url.trim_end_matches('/'));
        
        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", config.api_key))
            .send()
            .await?;
        
        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await?;
            return Err(anyhow::anyhow!("API error: {} - {}", status, text));
        }
        
        let models_response: ModelsResponse = response.json().await?;
        Ok(models_response.data)
    }

    /// Test connection to a provider
    pub async fn test_connection(&self, config: &ProviderApiConfig) -> anyhow::Result<()> {
        let url = format!("{}/v1/models", config.base_url.trim_end_matches('/'));
        
        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", config.api_key))
            .send()
            .await?;
        
        if response.status().is_success() {
            Ok(())
        } else {
            let status = response.status();
            let text = response.text().await?;
            Err(anyhow::anyhow!("API error: {} - {}", status, text))
        }
    }

    /// Chat completion request (for future use)
    pub async fn chat_completion(
        &self,
        config: &ProviderApiConfig,
        model: &str,
        messages: Vec<serde_json::Value>,
    ) -> anyhow::Result<serde_json::Value> {
        let url = format!("{}/v1/chat/completions", config.base_url.trim_end_matches('/'));
        
        let body = serde_json::json!({
            "model": model,
            "messages": messages,
        });
        
        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", config.api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;
        
        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await?;
            return Err(anyhow::anyhow!("API error: {} - {}", status, text));
        }
        
        let result: serde_json::Value = response.json().await?;
        Ok(result)
    }
}

impl Default for AiProviderClient {
    fn default() -> Self {
        Self::new()
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

/// Model information for UI
#[derive(Clone, Debug)]
pub struct ModelInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub max_tokens: u32,
    pub capabilities: ProviderCapabilities,
}

/// Trait for AI providers that use the unified client
/// 
/// Note: This trait integrates with GPUI's async system using Tasks
pub trait AiProvider: Send + Sync {
    /// Provider ID (e.g., "deepseek", "kimi")
    fn provider_id(&self) -> &str;
    
    /// Provider display name
    fn provider_name(&self) -> &str;
    
    /// Default base URL for this provider
    fn default_base_url(&self) -> &str;
    
    /// Get current API configuration
    fn get_config(&self) -> ProviderApiConfig;
    
    /// Update API key
    fn set_api_key(&mut self, api_key: Option<String>);
    
    /// Update base URL
    fn set_base_url(&mut self, base_url: String);
    
    /// Fetch available models from the API
    /// Returns a GPUI Task that resolves to a list of ModelInfo
    fn fetch_models(&self, cx: &App) -> Task<Result<Vec<ModelInfo>, String>>;
    
    /// Test connection to the provider
    /// Returns a GPUI Task that resolves to a success message or error
    fn test_connection(&self, cx: &App) -> Task<Result<String, String>>;
}


