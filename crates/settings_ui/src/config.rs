//! Settings configuration management
//!
//! Stores settings in JSON file at ~/.uavred/settings.json

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

/// Settings version for migration
const SETTINGS_VERSION: &str = "1.1";

/// Main settings structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub version: String,
    pub ai: AiSettings,
    pub appearance: AppearanceSettings,
    pub network: NetworkSettings,
    pub scanner: ScannerSettings,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            version: SETTINGS_VERSION.to_string(),
            ai: AiSettings::default(),
            appearance: AppearanceSettings::default(),
            network: NetworkSettings::default(),
            scanner: ScannerSettings::default(),
        }
    }
}

/// AI Model configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiModel {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_vision: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_reasoning: Option<bool>,
}

impl AiModel {
    pub fn new(id: &str, name: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            description: None,
            enabled: true,
            token_limit: None,
            supports_vision: None,
            supports_reasoning: None,
        }
    }
}

/// API Endpoint format type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ApiFormat {
    OpenAi,
    Anthropic,
    ClaudeCode,
    Azure,
    Ollama,
    Custom,
}

impl Default for ApiFormat {
    fn default() -> Self {
        ApiFormat::OpenAi
    }
}

/// API Endpoint configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiEndpoint {
    pub format: ApiFormat,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// AI Provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiProviderConfig {
    pub enabled: bool,
    pub endpoint: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub models: Vec<AiModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// Alternative API endpoints for different formats
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub alt_endpoints: Vec<ApiEndpoint>,
    /// Claude Code integration settings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub claude_code: Option<ClaudeCodeConfig>,
}

/// Claude Code integration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeCodeConfig {
    pub enabled: bool,
    pub proxy_url: String,
    pub default_model: String,
    pub small_fast_model: String,
    pub subagent_model: String,
}

impl Default for ClaudeCodeConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            proxy_url: String::new(),
            default_model: String::new(),
            small_fast_model: String::new(),
            subagent_model: String::new(),
        }
    }
}

impl AiProviderConfig {
    /// Get enabled models count
    pub fn enabled_models_count(&self) -> usize {
        self.models.iter().filter(|m| m.enabled).count()
    }

    /// Get model by id
    pub fn get_model(&self, id: &str) -> Option<&AiModel> {
        self.models.iter().find(|m| m.id == id)
    }

    /// Update or add model
    pub fn upsert_model(&mut self, model: AiModel) {
        if let Some(idx) = self.models.iter().position(|m| m.id == model.id) {
            self.models[idx] = model;
        } else {
            self.models.push(model);
        }
    }

    /// Toggle model enabled state
    pub fn toggle_model(&mut self, model_id: &str) -> bool {
        if let Some(model) = self.models.iter_mut().find(|m| m.id == model_id) {
            model.enabled = !model.enabled;
            model.enabled
        } else {
            false
        }
    }
}

impl Default for AiProviderConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            endpoint: String::new(),
            api_key: None,
            models: vec![],
            region: None,
            alt_endpoints: vec![],
            claude_code: None,
        }
    }
}

/// AI settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiSettings {
    pub active_provider: String,
    pub providers: HashMap<String, AiProviderConfig>,
}

impl Default for AiSettings {
    fn default() -> Self {
        let mut providers = HashMap::new();
        
        // DeepSeek with complete configuration
        let mut deepseek_models = vec![
            AiModel {
                id: "deepseek-chat".to_string(),
                name: "DeepSeek Chat".to_string(),
                description: Some("General purpose chat model".to_string()),
                enabled: true,
                token_limit: Some(64000),
                supports_vision: Some(false),
                supports_reasoning: Some(false),
            },
            AiModel {
                id: "deepseek-reasoner".to_string(),
                name: "DeepSeek Reasoner".to_string(),
                description: Some("Reasoning model with step-by-step thinking".to_string()),
                enabled: true,
                token_limit: Some(64000),
                supports_vision: Some(false),
                supports_reasoning: Some(true),
            },
        ];
        
        let deepseek_endpoints = vec![
            ApiEndpoint {
                format: ApiFormat::OpenAi,
                url: "https://api.deepseek.com/v1/chat/completions".to_string(),
                description: Some("OpenAI compatible endpoint".to_string()),
            },
        ];
        
        providers.insert("deepseek".to_string(), AiProviderConfig {
            enabled: false,
            endpoint: "https://api.deepseek.com".to_string(),
            api_key: None,
            models: deepseek_models,
            region: Some("international".to_string()),
            alt_endpoints: deepseek_endpoints,
            claude_code: Some(ClaudeCodeConfig::default()),
        });
        
        // GLM with complete configuration
        let mut glm_models = vec![
            AiModel {
                id: "glm-4".to_string(),
                name: "GLM-4".to_string(),
                description: Some("General purpose model".to_string()),
                enabled: true,
                token_limit: Some(128000),
                supports_vision: Some(true),
                supports_reasoning: Some(false),
            },
            AiModel {
                id: "glm-4-flash".to_string(),
                name: "GLM-4 Flash".to_string(),
                description: Some("Fast and cost-effective".to_string()),
                enabled: false,
                token_limit: Some(128000),
                supports_vision: Some(false),
                supports_reasoning: Some(false),
            },
            AiModel {
                id: "chatglm3-6b".to_string(),
                name: "ChatGLM3-6B".to_string(),
                description: Some("Open source model".to_string()),
                enabled: false,
                token_limit: Some(32000),
                supports_vision: Some(false),
                supports_reasoning: Some(false),
            },
        ];
        
        providers.insert("glm".to_string(), AiProviderConfig {
            enabled: false,
            endpoint: "https://open.bigmodel.cn/api/paas".to_string(),
            api_key: None,
            models: glm_models,
            region: Some("china".to_string()),
            alt_endpoints: vec![],
            claude_code: None,
        });
        
        // Ollama (local)
        providers.insert("ollama".to_string(), AiProviderConfig {
            enabled: true,
            endpoint: "http://localhost:11434".to_string(),
            api_key: None,
            models: vec![
                AiModel {
                    id: "llama3.2".to_string(),
                    name: "Llama 3.2".to_string(),
                    description: None,
                    enabled: true,
                    token_limit: Some(128000),
                    supports_vision: Some(true),
                    supports_reasoning: Some(false),
                },
                AiModel {
                    id: "qwen2.5".to_string(),
                    name: "Qwen 2.5".to_string(),
                    description: None,
                    enabled: true,
                    token_limit: Some(128000),
                    supports_vision: Some(false),
                    supports_reasoning: Some(false),
                },
            ],
            region: None,
            alt_endpoints: vec![],
            claude_code: None,
        });
        
        // OpenAI
        providers.insert("openai".to_string(), AiProviderConfig {
            enabled: false,
            endpoint: "https://api.openai.com/v1".to_string(),
            api_key: None,
            models: vec![
                AiModel {
                    id: "gpt-4o".to_string(),
                    name: "GPT-4o".to_string(),
                    description: Some("Most capable multimodal model".to_string()),
                    enabled: true,
                    token_limit: Some(128000),
                    supports_vision: Some(true),
                    supports_reasoning: Some(false),
                },
                AiModel {
                    id: "gpt-4o-mini".to_string(),
                    name: "GPT-4o Mini".to_string(),
                    description: Some("Fast and affordable".to_string()),
                    enabled: true,
                    token_limit: Some(128000),
                    supports_vision: Some(true),
                    supports_reasoning: Some(false),
                },
            ],
            region: Some("international".to_string()),
            alt_endpoints: vec![],
            claude_code: None,
        });
        
        // Claude
        providers.insert("claude".to_string(), AiProviderConfig {
            enabled: false,
            endpoint: "https://api.anthropic.com".to_string(),
            api_key: None,
            models: vec![
                AiModel {
                    id: "claude-3-5-sonnet-20241022".to_string(),
                    name: "Claude 3.5 Sonnet".to_string(),
                    description: Some("Most capable Claude model".to_string()),
                    enabled: true,
                    token_limit: Some(200000),
                    supports_vision: Some(true),
                    supports_reasoning: Some(true),
                },
            ],
            region: Some("international".to_string()),
            alt_endpoints: vec![],
            claude_code: None,
        });
        
        // Kimi (Moonshot) with complete configuration including Kimi Code
        let kimi_models = vec![
            AiModel {
                id: "kimi-k2.5".to_string(),
                name: "Kimi K2.5".to_string(),
                description: Some("Latest Kimi model with superior performance".to_string()),
                enabled: true,
                token_limit: Some(256000),
                supports_vision: Some(true),
                supports_reasoning: Some(true),
            },
            AiModel {
                id: "moonshot-v1-128k".to_string(),
                name: "Moonshot v1 128K".to_string(),
                description: Some("Long context model (128K tokens)".to_string()),
                enabled: true,
                token_limit: Some(128000),
                supports_vision: Some(false),
                supports_reasoning: Some(false),
            },
            AiModel {
                id: "moonshot-v1-32k".to_string(),
                name: "Moonshot v1 32K".to_string(),
                description: Some("Standard model (32K tokens)".to_string()),
                enabled: false,
                token_limit: Some(32000),
                supports_vision: Some(false),
                supports_reasoning: Some(false),
            },
            AiModel {
                id: "moonshot-v1-8k".to_string(),
                name: "Moonshot v1 8K".to_string(),
                description: Some("Fast and economical (8K tokens)".to_string()),
                enabled: false,
                token_limit: Some(8000),
                supports_vision: Some(false),
                supports_reasoning: Some(false),
            },
        ];
        
        let kimi_endpoints = vec![
            ApiEndpoint {
                format: ApiFormat::OpenAi,
                url: "https://api.moonshot.cn/v1/chat/completions".to_string(),
                description: Some("OpenAI compatible chat completions".to_string()),
            },
        ];
        
        providers.insert("kimi".to_string(), AiProviderConfig {
            enabled: false,
            endpoint: "https://api.moonshot.cn".to_string(),
            api_key: None,
            models: kimi_models,
            region: Some("china".to_string()),
            alt_endpoints: kimi_endpoints,
            claude_code: Some(ClaudeCodeConfig {
                enabled: true,
                proxy_url: "https://api.moonshot.cn".to_string(),
                default_model: "kimi-k2.5".to_string(),
                small_fast_model: "moonshot-v1-8k".to_string(),
                subagent_model: "kimi-k2.5".to_string(),
            }),
        });
        
        providers.insert("gemini".to_string(), AiProviderConfig {
            enabled: false,
            endpoint: "https://generativelanguage.googleapis.com".to_string(),
            api_key: None,
            models: vec![
                AiModel {
                    id: "gemini-1.5-pro".to_string(),
                    name: "Gemini 1.5 Pro".to_string(),
                    description: Some("Most capable Gemini model".to_string()),
                    enabled: true,
                    token_limit: Some(2000000),
                    supports_vision: Some(true),
                    supports_reasoning: Some(true),
                },
            ],
            region: Some("international".to_string()),
            alt_endpoints: vec![],
            claude_code: None,
        });
        
        providers.insert("azure".to_string(), AiProviderConfig {
            enabled: false,
            endpoint: "https://{your-resource}.openai.azure.com".to_string(),
            api_key: None,
            models: vec![],
            region: None,
            alt_endpoints: vec![],
            claude_code: None,
        });
        
        providers.insert("lmstudio".to_string(), AiProviderConfig {
            enabled: false,
            endpoint: "http://localhost:1234".to_string(),
            api_key: None,
            models: vec![],
            region: None,
            alt_endpoints: vec![],
            claude_code: None,
        });

        Self {
            active_provider: "ollama".to_string(),
            providers,
        }
    }
}

/// Appearance settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppearanceSettings {
    pub theme: String,
    pub font_size: String,
    pub ui_density: String,
    pub accent_color: String,
}

impl Default for AppearanceSettings {
    fn default() -> Self {
        Self {
            theme: "light".to_string(),
            font_size: "medium".to_string(),
            ui_density: "comfortable".to_string(),
            accent_color: "purple".to_string(),
        }
    }
}

/// Network settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkSettings {
    pub proxy_enabled: bool,
    pub proxy_url: String,
    pub verify_ssl: bool,
    pub request_timeout: u64,
}

impl Default for NetworkSettings {
    fn default() -> Self {
        Self {
            proxy_enabled: false,
            proxy_url: String::new(),
            verify_ssl: true,
            request_timeout: 60,
        }
    }
}

/// Scanner settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScannerSettings {
    pub scan_threads: u32,
    pub scan_timeout: u64,
    pub default_scan_type: String,
    pub port_range: String,
    pub rate_limit: bool,
}

impl Default for ScannerSettings {
    fn default() -> Self {
        Self {
            scan_threads: 50,
            scan_timeout: 10,
            default_scan_type: "standard".to_string(),
            port_range: "1-1000".to_string(),
            rate_limit: true,
        }
    }
}

impl Settings {
    /// Get settings file path
    pub fn settings_path() -> PathBuf {
        let home = dirs::home_dir().expect("Failed to get home directory");
        home.join(".uavred").join("settings.json")
    }

    /// Load settings from file, create default if not exists
    pub fn load() -> anyhow::Result<Self> {
        let path = Self::settings_path();
        
        if !path.exists() {
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            
            let settings = Self::default();
            settings.save()?;
            return Ok(settings);
        }
        
        let content = std::fs::read_to_string(&path)?;
        let settings: Settings = serde_json::from_str(&content)?;
        
        // Migrate if needed
        if settings.version != SETTINGS_VERSION {
            // TODO: Handle migration
        }
        
        Ok(settings)
    }

    /// Save settings to file
    pub fn save(&self) -> anyhow::Result<()> {
        let path = Self::settings_path();
        
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(&path, content)?;
        
        Ok(())
    }

    /// Get AI provider config
    pub fn get_ai_provider(&self, id: &str) -> Option<&AiProviderConfig> {
        self.ai.providers.get(id)
    }

    /// Update AI provider config
    pub fn update_ai_provider(&mut self, id: &str, config: AiProviderConfig) {
        self.ai.providers.insert(id.to_string(), config);
    }

    /// Set active AI provider
    pub fn set_active_provider(&mut self, id: &str) {
        self.ai.active_provider = id.to_string();
    }
}

/// Global settings manager
pub struct SettingsManager {
    settings: Arc<std::sync::RwLock<Settings>>,
}

impl SettingsManager {
    pub fn new() -> anyhow::Result<Self> {
        let settings = Settings::load()?;
        Ok(Self {
            settings: Arc::new(std::sync::RwLock::new(settings)),
        })
    }

    pub fn get(&self) -> anyhow::Result<Settings> {
        let settings = self.settings.read().map_err(|e| {
            anyhow::anyhow!("Failed to read settings: {}", e)
        })?;
        Ok(settings.clone())
    }

    pub fn update<F>(&self, f: F) -> anyhow::Result<()>
    where
        F: FnOnce(&mut Settings),
    {
        let mut settings = self.settings.write().map_err(|e| {
            anyhow::anyhow!("Failed to write settings: {}", e)
        })?;
        f(&mut settings);
        settings.save()?;
        Ok(())
    }
}

impl Default for SettingsManager {
    fn default() -> Self {
        Self::new().expect("Failed to initialize settings")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_settings() {
        let settings = Settings::default();
        assert_eq!(settings.version, SETTINGS_VERSION);
        assert!(settings.ai.providers.contains_key("deepseek"));
        assert!(settings.ai.providers.contains_key("glm"));
        assert!(settings.ai.providers.contains_key("ollama"));
    }
    
    #[test]
    fn test_deepseek_models() {
        let settings = Settings::default();
        let deepseek = settings.ai.providers.get("deepseek").unwrap();
        assert_eq!(deepseek.models.len(), 2);
        assert!(deepseek.models.iter().any(|m| m.id == "deepseek-chat"));
        assert!(deepseek.models.iter().any(|m| m.id == "deepseek-reasoner"));
    }
    
    #[test]
    fn test_glm_models() {
        let settings = Settings::default();
        let glm = settings.ai.providers.get("glm").unwrap();
        assert_eq!(glm.models.len(), 3);
        assert!(glm.models.iter().any(|m| m.id == "glm-4"));
    }
}
