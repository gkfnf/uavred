//! DeepSeek Provider Implementation
//!
//! Uses the unified AiProviderClient for all API calls.

use gpui::*;

use super::ai_client::{AiProvider, ApiModel, ProviderApiConfig, AiProviderClient, ModelInfo, ProviderCapabilities};
use crate::config::{AiProviderConfig, Settings};

pub struct DeepSeekProvider {
    config: AiProviderConfig,
    api_key: Option<String>,
}

impl DeepSeekProvider {
    pub fn new() -> Self {
        let config = Settings::load()
            .ok()
            .and_then(|s| s.ai.providers.get("deepseek").cloned())
            .unwrap_or_else(default_deepseek_config);
        
        let api_key = std::env::var("DEEPSEEK_API_KEY")
            .ok()
            .or_else(|| config.api_key.clone());
        
        Self { config, api_key }
    }
    
    /// Convert ApiModel to ModelInfo for UI
    fn to_model_info(model: &ApiModel) -> ModelInfo {
        ModelInfo {
            id: model.id.clone(),
            name: model.name.clone().unwrap_or_else(|| model.id.clone()),
            description: model.description.clone().unwrap_or_default(),
            max_tokens: model.context_length.unwrap_or(64000),
            capabilities: ProviderCapabilities {
                supports_chat: true,
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
            },
        }
    }
    
    /// Get default models for DeepSeek
    pub fn default_models() -> Vec<ApiModel> {
        vec![
            ApiModel {
                id: "deepseek-chat".to_string(),
                name: Some("DeepSeek Chat".to_string()),
                description: Some("General purpose chat model".to_string()),
                context_length: Some(64000),
            },
            ApiModel {
                id: "deepseek-reasoner".to_string(),
                name: Some("DeepSeek Reasoner".to_string()),
                description: Some("Reasoning model with step-by-step thinking".to_string()),
                context_length: Some(64000),
            },
        ]
    }
    
    /// Save API key to settings
    fn save_api_key(&mut self, key: String) -> anyhow::Result<()> {
        let mut settings = Settings::load()?;
        
        if let Some(config) = settings.ai.providers.get_mut("deepseek") {
            config.api_key = Some(key.clone());
        } else {
            let mut config = default_deepseek_config();
            config.api_key = Some(key.clone());
            settings.ai.providers.insert("deepseek".to_string(), config);
        }
        
        settings.save()?;
        self.api_key = Some(key);
        Ok(())
    }
}

impl AiProvider for DeepSeekProvider {
    fn provider_id(&self) -> &str {
        "deepseek"
    }
    
    fn provider_name(&self) -> &str {
        "DeepSeek"
    }
    
    fn default_base_url(&self) -> &str {
        "https://api.deepseek.com"
    }
    
    fn get_config(&self) -> ProviderApiConfig {
        ProviderApiConfig {
            base_url: self.config.endpoint.clone(),
            api_key: self.api_key.clone().unwrap_or_default(),
        }
    }
    
    fn set_api_key(&mut self, key: Option<String>) {
        match key {
            Some(k) if !k.is_empty() => {
                let _ = self.save_api_key(k);
            }
            _ => {
                self.api_key = None;
            }
        }
    }
    
    fn set_base_url(&mut self, base_url: String) {
        self.config.endpoint = base_url;
    }
    
    fn fetch_models(&self, cx: &App) -> Task<Result<Vec<ModelInfo>, String>> {
        let config = self.get_config();
        
        if config.api_key.is_empty() {
            return Task::ready(Err("API key not configured".to_string()));
        }
        
        cx.background_spawn(async move {
            let client = AiProviderClient::new();
            match client.fetch_models(&config).await {
                Ok(models) => {
                    let model_infos: Vec<ModelInfo> = models.iter()
                        .map(Self::to_model_info)
                        .collect();
                    Ok(model_infos)
                }
                Err(e) => Err(format!("Failed to fetch models: {}", e)),
            }
        })
    }
    
    fn test_connection(&self, cx: &App) -> Task<Result<String, String>> {
        let config = self.get_config();
        
        if config.api_key.is_empty() {
            return Task::ready(Err("API key not configured".to_string()));
        }
        
        cx.background_spawn(async move {
            let client = AiProviderClient::new();
            match client.test_connection(&config).await {
                Ok(()) => Ok("Connection successful".to_string()),
                Err(e) => Err(format!("Connection failed: {}", e)),
            }
        })
    }
}

fn default_deepseek_config() -> AiProviderConfig {
    AiProviderConfig {
        enabled: false,
        endpoint: "https://api.deepseek.com".to_string(),
        api_key: None,
        models: vec![],
        region: Some("international".to_string()),
        alt_endpoints: vec![],
        claude_code: None,
    }
}
