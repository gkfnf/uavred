//! Kimi (Moonshot) Provider Implementation
//! 
//! Kimi API is compatible with OpenAI's API format.
//! Base URL: https://api.moonshot.cn
//! Docs: https://platform.moonshot.cn/docs

use gpui::*;
use gpui_component::{label::Label, button::Button, IconName};
use serde::Deserialize;

use super::{AiProvider, ApiKeyState, ModelInfo, ProviderCapabilities, ProviderId};
use crate::config::{AiProviderConfig, Settings};

pub const KIMI_API_URL: &str = "https://api.moonshot.cn";
pub fn kimi_provider_id() -> ProviderId {
    ProviderId::new("kimi")
}

/// Placeholder configuration view
struct ConfigurationPlaceholder;

impl Render for ConfigurationPlaceholder {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        gpui::div().child("Configuration View")
    }
}

/// Kimi provider implementation
pub struct KimiProvider {
    api_key_state: ApiKeyState,
    config: AiProviderConfig,
    http_client: reqwest::Client,
}

impl KimiProvider {
    pub fn new() -> Self {
        let config = Self::load_config();
        let api_key_state = Self::load_api_key(&config);
        
        Self {
            api_key_state,
            config,
            http_client: reqwest::Client::new(),
        }
    }
    
    fn load_config() -> AiProviderConfig {
        Settings::load()
            .ok()
            .and_then(|s| s.ai.providers.get("kimi").cloned())
            .unwrap_or_else(|| default_kimi_config())
    }
    
    fn load_api_key(config: &AiProviderConfig) -> ApiKeyState {
        // Check environment variable first
        if let Ok(key) = std::env::var("MOONSHOT_API_KEY") {
            if !key.is_empty() {
                return ApiKeyState::FromEnv(key);
            }
        }
        
        // Then check saved config
        if let Some(key) = &config.api_key {
            if !key.is_empty() {
                return ApiKeyState::Configured(key.clone());
            }
        }
        
        ApiKeyState::NotConfigured
    }
    
    /// Save API key to settings
    fn save_api_key(&mut self, key: String) -> anyhow::Result<()> {
        let mut settings = Settings::load()?;
        
        if let Some(config) = settings.ai.providers.get_mut("kimi") {
            config.api_key = Some(key.clone());
        } else {
            let mut config = default_kimi_config();
            config.api_key = Some(key.clone());
            settings.ai.providers.insert("kimi".to_string(), config);
        }
        
        settings.save()?;
        self.api_key_state = ApiKeyState::Configured(key);
        Ok(())
    }
    
    /// Clear API key
    fn clear_api_key(&mut self) -> anyhow::Result<()> {
        let mut settings = Settings::load()?;
        
        if let Some(config) = settings.ai.providers.get_mut("kimi") {
            config.api_key = None;
            settings.save()?;
        }
        
        self.api_key_state = ApiKeyState::NotConfigured;
        Ok(())
    }
    
    /// Get default models for Kimi
    fn default_models() -> Vec<ModelInfo> {
        vec![
            ModelInfo {
                id: "kimi-k2.5".to_string(),
                name: "Kimi K2.5".to_string(),
                description: "Latest Kimi model with superior performance and reasoning".to_string(),
                max_tokens: 256_000,
                capabilities: ProviderCapabilities {
                    supports_chat: true,
                    supports_vision: true,
                    supports_tools: true,
                    supports_streaming: true,
                },
            },
            ModelInfo {
                id: "moonshot-v1-128k".to_string(),
                name: "Moonshot v1 128K".to_string(),
                description: "Long context model with 128K token limit".to_string(),
                max_tokens: 128_000,
                capabilities: ProviderCapabilities {
                    supports_chat: true,
                    supports_vision: false,
                    supports_tools: true,
                    supports_streaming: true,
                },
            },
            ModelInfo {
                id: "moonshot-v1-32k".to_string(),
                name: "Moonshot v1 32K".to_string(),
                description: "Standard model with 32K token limit".to_string(),
                max_tokens: 32_000,
                capabilities: ProviderCapabilities {
                    supports_chat: true,
                    supports_vision: false,
                    supports_tools: true,
                    supports_streaming: true,
                },
            },
            ModelInfo {
                id: "moonshot-v1-8k".to_string(),
                name: "Moonshot v1 8K".to_string(),
                description: "Fast and economical model with 8K token limit".to_string(),
                max_tokens: 8_000,
                capabilities: ProviderCapabilities {
                    supports_chat: true,
                    supports_vision: false,
                    supports_tools: false,
                    supports_streaming: true,
                },
            },
        ]
    }
}

impl AiProvider for KimiProvider {
    fn id(&self) -> ProviderId {
        kimi_provider_id()
    }
    
    fn name(&self) -> &str {
        "Kimi (Moonshot)"
    }
    
    fn description(&self) -> &str {
        "Moonshot AI with long context and strong reasoning capabilities"
    }
    
    fn icon(&self) -> IconName {
        IconName::Bot
    }
    
    fn is_authenticated(&self) -> bool {
        self.api_key_state.is_configured()
    }
    
    fn api_key_state(&self) -> &ApiKeyState {
        &self.api_key_state
    }
    
    fn set_api_key(&mut self, key: Option<String>) {
        match key {
            Some(k) => {
                let _ = self.save_api_key(k);
            }
            None => {
                let _ = self.clear_api_key();
            }
        }
    }
    
    fn models(&self) -> Vec<ModelInfo> {
        if self.config.models.is_empty() {
            Self::default_models()
        } else {
            self.config.models.iter().map(|m| ModelInfo {
                id: m.id.clone(),
                name: m.name.clone(),
                description: m.description.clone().unwrap_or_default(),
                max_tokens: m.token_limit.unwrap_or(128_000),
                capabilities: ProviderCapabilities {
                    supports_chat: true,
                    supports_vision: m.supports_vision.unwrap_or(false),
                    supports_tools: true,
                    supports_streaming: true,
                },
            }).collect()
        }
    }
    
    fn capabilities(&self) -> ProviderCapabilities {
        ProviderCapabilities {
            supports_chat: true,
            supports_vision: true,
            supports_tools: true,
            supports_streaming: true,
        }
    }
    
    fn configuration_view(&self, window: &mut Window, cx: &mut App) -> AnyView {
        // Return a simple placeholder view
        cx.new(|_cx| ConfigurationPlaceholder).into()
    }
    
    fn test_connection(&self, cx: &App) -> Task<Result<String, String>> {
        let api_key = match self.api_key_state.key() {
            Some(key) => key.to_string(),
            None => return Task::ready(Err("API key not configured".to_string())),
        };
        
        let client = self.http_client.clone();
        
        cx.background_spawn(async move {
            match test_kimi_connection(&client, &api_key).await {
                Ok(()) => Ok("Successfully connected to Kimi API".to_string()),
                Err(e) => Err(format!("Connection failed: {}", e)),
            }
        })
    }
    
    fn fetch_models(&self, cx: &App) -> Task<Result<Vec<ModelInfo>, String>> {
        let api_key = match self.api_key_state.key() {
            Some(key) => key.to_string(),
            None => return Task::ready(Err("API key not configured".to_string())),
        };
        
        let client = self.http_client.clone();
        let default_models = Self::default_models();
        
        cx.background_spawn(async move {
            match fetch_kimi_models(&client, &api_key).await {
                Ok(models) => {
                    if models.is_empty() {
                        Ok(default_models)
                    } else {
                        Ok(models)
                    }
                }
                Err(e) => {
                    println!("Failed to fetch models: {}, using defaults", e);
                    Ok(default_models)
                }
            }
        })
    }
}

/// Default Kimi configuration
fn default_kimi_config() -> AiProviderConfig {
    AiProviderConfig {
        enabled: false,
        endpoint: KIMI_API_URL.to_string(),
        api_key: None,
        models: vec![],
        region: Some("china".to_string()),
        alt_endpoints: vec![],
        claude_code: None,
    }
}

/// Kimi API response structures
#[derive(Debug, Deserialize)]
struct ModelsResponse {
    data: Vec<ApiModel>,
}

#[derive(Debug, Deserialize)]
struct ApiModel {
    id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    context_length: Option<u32>,
}

/// Test connection to Kimi API
async fn test_kimi_connection(client: &reqwest::Client, api_key: &str) -> anyhow::Result<()> {
    let url = format!("{}/v1/models", KIMI_API_URL);
    
    let response = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await?;
    
    if response.status().is_success() {
        Ok(())
    } else {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        Err(anyhow::anyhow!("API error {}: {}", status, text))
    }
}

/// Fetch available models from Kimi API
async fn fetch_kimi_models(client: &reqwest::Client, api_key: &str) -> anyhow::Result<Vec<ModelInfo>> {
    let url = format!("{}/v1/models", KIMI_API_URL);
    
    let response = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await?;
    
    if !response.status().is_success() {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        return Err(anyhow::anyhow!("API error {}: {}", status, text));
    }
    
    let models_response: ModelsResponse = response.json().await?;
    
    let models = models_response.data.into_iter().map(|m| ModelInfo {
        id: m.id.clone(),
        name: m.name.unwrap_or_else(|| m.id.clone()),
        description: String::new(),
        max_tokens: m.context_length.unwrap_or(128_000),
        capabilities: ProviderCapabilities {
            supports_chat: true,
            supports_vision: m.id.contains("vision") || m.id.contains("k2.5"),
            supports_tools: true,
            supports_streaming: true,
        },
    }).collect();
    
    Ok(models)
}


