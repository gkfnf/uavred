//! AI Provider API Client
//! 
//! Supports fetching models from various AI providers including:
//! - Kimi (Moonshot)
//! - DeepSeek
//! - OpenAI
//! - ChatGLM

use serde::{Deserialize, Serialize};

/// Model information from API
#[derive(Debug, Clone, Deserialize)]
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

/// AI Provider API Client
pub struct AiProviderClient {
    client: reqwest::Client,
}

impl AiProviderClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    /// Fetch models from Kimi (Moonshot)
    pub async fn fetch_kimi_models(&self, api_key: &str) -> anyhow::Result<Vec<ApiModel>> {
        let url = "https://api.moonshot.cn/v1/models";
        
        let response = self.client
            .get(url)
            .header("Authorization", format!("Bearer {}", api_key))
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

    /// Fetch models from DeepSeek
    pub async fn fetch_deepseek_models(&self, api_key: &str) -> anyhow::Result<Vec<ApiModel>> {
        let url = "https://api.deepseek.com/v1/models";
        
        let response = self.client
            .get(url)
            .header("Authorization", format!("Bearer {}", api_key))
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

    /// Fetch models from OpenAI
    pub async fn fetch_openai_models(&self, api_key: &str) -> anyhow::Result<Vec<ApiModel>> {
        let url = "https://api.openai.com/v1/models";
        
        let response = self.client
            .get(url)
            .header("Authorization", format!("Bearer {}", api_key))
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
    pub async fn test_connection(&self, provider_id: &str, endpoint: &str, api_key: &str) -> anyhow::Result<String> {
        match provider_id {
            "kimi" => {
                let models = self.fetch_kimi_models(api_key).await?;
                Ok(format!("Connected! Found {} models", models.len()))
            }
            "deepseek" => {
                let models = self.fetch_deepseek_models(api_key).await?;
                Ok(format!("Connected! Found {} models", models.len()))
            }
            "openai" => {
                let models = self.fetch_openai_models(api_key).await?;
                Ok(format!("Connected! Found {} models", models.len()))
            }
            _ => Err(anyhow::anyhow!("Unsupported provider: {}", provider_id)),
        }
    }
}

impl Default for AiProviderClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_client_creation() {
        let client = AiProviderClient::new();
        // Just verify it can be created
    }
}
