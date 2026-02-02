//! AI Settings Panel - Full-featured provider configuration
//! 
//! Architecture based on Zed's language model provider pattern.
//! Supports multiple AI providers with unified interface.

use gpui::*;
use gpui_component::{
    v_flex, h_flex,
    label::Label,
    button::{Button, ButtonVariants},
    input::InputState,
    switch::Switch,
    scroll::ScrollableElement,
    IconName, Icon,
};

use crate::config::{Settings, AiProviderConfig, AiModel};
use crate::provider::{AiProvider, ProviderId};
use crate::provider::kimi::{KimiProvider, kimi_provider_id};

/// AI Provider UI definition
#[derive(Clone)]
pub struct ProviderUi {
    pub id: ProviderId,
    pub name: String,
    pub description: String,
    pub icon: IconName,
    pub is_local: bool,
}

impl ProviderUi {
    fn all() -> Vec<Self> {
        vec![
            ProviderUi {
                id: ProviderId::new("kimi"),
                name: "Kimi (Moonshot)".to_string(),
                description: "Moonshot AI with long context and K2.5 model".to_string(),
                icon: IconName::Bot,
                is_local: false,
            },
            ProviderUi {
                id: ProviderId::new("deepseek"),
                name: "DeepSeek".to_string(),
                description: "DeepSeek AI with reasoning capabilities".to_string(),
                icon: IconName::Bot,
                is_local: false,
            },
            ProviderUi {
                id: ProviderId::new("openai"),
                name: "OpenAI".to_string(),
                description: "GPT-4, GPT-3.5, and more".to_string(),
                icon: IconName::Bot,
                is_local: false,
            },
            ProviderUi {
                id: ProviderId::new("claude"),
                name: "Claude".to_string(),
                description: "Anthropic Claude models".to_string(),
                icon: IconName::Bot,
                is_local: false,
            },
            ProviderUi {
                id: ProviderId::new("ollama"),
                name: "Ollama".to_string(),
                description: "Run AI models locally".to_string(),
                icon: IconName::Cpu,
                is_local: true,
            },
        ]
    }
}

/// AI Settings Panel State
pub struct AiSettingsPanel {
    providers_ui: Vec<ProviderUi>,
    selected_provider_id: ProviderId,
    settings: Settings,
    status_message: Option<String>,
    status_is_error: bool,
    model_search_query: String,
    is_loading: bool,
    // Provider instances
    kimi_provider: KimiProvider,
}

impl AiSettingsPanel {
    pub fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        let settings = Settings::load().unwrap_or_default();
        let active_provider = ProviderId::new(&settings.ai.active_provider);
        
        Self {
            providers_ui: ProviderUi::all(),
            selected_provider_id: active_provider,
            settings: settings.clone(),
            status_message: None,
            status_is_error: false,
            model_search_query: String::new(),
            is_loading: false,
            kimi_provider: KimiProvider::new(),
        }
    }
    
    fn select_provider(&mut self, id: ProviderId, cx: &mut Context<Self>) {
        self.selected_provider_id = id.clone();
        self.status_message = None;
        self.model_search_query = String::new();
        cx.notify();
    }
    
    fn get_current_provider_config(&self) -> Option<&AiProviderConfig> {
        self.settings.ai.providers.get(self.selected_provider_id.as_str())
    }
    
    fn get_current_provider_config_mut(&mut self) -> Option<&mut AiProviderConfig> {
        self.settings.ai.providers.get_mut(self.selected_provider_id.as_str())
    }
    
    fn toggle_provider_enabled(&mut self, cx: &mut Context<Self>) {
        let id = self.selected_provider_id.clone();
        
        if let Some(config) = self.get_current_provider_config_mut() {
            config.enabled = !config.enabled;
            if config.enabled {
                self.settings.ai.active_provider = id.as_str().to_string();
            }
        } else {
            // Create new config
            let mut config = AiProviderConfig::default();
            config.enabled = true;
            self.settings.ai.active_provider = id.as_str().to_string();
            self.settings.ai.providers.insert(id.as_str().to_string(), config);
        }
        
        self.save_settings(cx);
    }
    
    fn toggle_model_enabled(&mut self, model_id: &str, cx: &mut Context<Self>) {
        if let Some(config) = self.get_current_provider_config_mut() {
            config.toggle_model(model_id);
            self.save_settings(cx);
        }
    }
    
    fn test_connection(&mut self, cx: &mut Context<Self>) {
        self.is_loading = true;
        self.status_message = Some("Testing connection...".to_string());
        self.status_is_error = false;
        cx.notify();
        
        match self.selected_provider_id.as_str() {
            "kimi" => {
                let task = self.kimi_provider.test_connection(cx);
                cx.spawn(async move |this, cx| {
                    let result = task.await;
                    cx.update(|cx| {
                        this.update(cx, |this, cx| {
                            this.is_loading = false;
                            match result {
                                Ok(msg) => {
                                    this.status_message = Some(msg);
                                    this.status_is_error = false;
                                }
                                Err(e) => {
                                    this.status_message = Some(e);
                                    this.status_is_error = true;
                                }
                            }
                            cx.notify();
                        }).ok();
                    }).ok();
                }).detach();
            }
            _ => {
                self.is_loading = false;
                self.status_message = Some("Connection test not implemented for this provider".to_string());
                self.status_is_error = false;
                cx.notify();
            }
        }
    }
    
    fn fetch_models(&mut self, cx: &mut Context<Self>) {
        self.is_loading = true;
        self.status_message = Some("Fetching models...".to_string());
        self.status_is_error = false;
        cx.notify();
        
        match self.selected_provider_id.as_str() {
            "kimi" => {
                let task = self.kimi_provider.fetch_models(cx);
                cx.spawn(async move |this, cx| {
                    let result = task.await;
                    cx.update(|cx| {
                        this.update(cx, |this, cx| {
                            this.is_loading = false;
                            match result {
                                Ok(models) => {
                                    // Update config with fetched models
                                    if let Some(config) = this.get_current_provider_config_mut() {
                                        for model_info in models {
                                            let model = AiModel {
                                                id: model_info.id.clone(),
                                                name: model_info.name.clone(),
                                                description: Some(model_info.description),
                                                enabled: true,
                                                token_limit: Some(model_info.max_tokens),
                                                supports_vision: Some(model_info.capabilities.supports_vision),
                                                supports_reasoning: Some(false),
                                            };
                                            config.upsert_model(model);
                                        }
                                        this.save_settings(cx);
                                    }
                                    this.status_message = Some(format!("Successfully fetched models"));
                                    this.status_is_error = false;
                                }
                                Err(e) => {
                                    this.status_message = Some(e);
                                    this.status_is_error = true;
                                }
                            }
                            cx.notify();
                        }).ok();
                    }).ok();
                }).detach();
            }
            _ => {
                self.is_loading = false;
                self.status_message = Some("Fetch models not implemented for this provider".to_string());
                self.status_is_error = false;
                cx.notify();
            }
        }
    }
    
    fn save_settings(&mut self, cx: &mut Context<Self>) {
        match self.settings.save() {
            Ok(()) => {
                self.status_message = Some("Settings saved".to_string());
                self.status_is_error = false;
            }
            Err(e) => {
                self.status_message = Some(format!("Failed to save: {}", e));
                self.status_is_error = true;
            }
        }
        cx.notify();
    }
    
    fn selected_provider_ui(&self) -> Option<&ProviderUi> {
        self.providers_ui.iter().find(|p| p.id == self.selected_provider_id)
    }
    
    fn is_provider_active(&self) -> bool {
        self.get_current_provider_config()
            .map(|c| c.enabled)
            .unwrap_or(false)
    }
    
    fn filtered_models(&self) -> Vec<AiModel> {
        let config = match self.get_current_provider_config() {
            Some(c) => c,
            None => return vec![],
        };
        
        if self.model_search_query.is_empty() {
            config.models.clone()
        } else {
            let query = self.model_search_query.to_lowercase();
            config.models
                .iter()
                .filter(|m| {
                    m.id.to_lowercase().contains(&query) || 
                    m.name.to_lowercase().contains(&query)
                })
                .cloned()
                .collect()
        }
    }
}

impl Render for AiSettingsPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let provider_ui = self.selected_provider_ui().cloned();
        let is_active = self.is_provider_active();
        let status_message = self.status_message.clone();
        let status_is_error = self.status_is_error;
        let config = self.get_current_provider_config().cloned();
        
        // Left sidebar - Provider list
        let mut list = v_flex()
            .w(px(300.0))
            .h_full()
            .gap_2()
            ;
        
        for provider in self.providers_ui.clone() {
            let is_selected = provider.id == self.selected_provider_id;
            let provider_config = self.settings.ai.providers.get(provider.id.as_str());
            let is_enabled = provider_config.map(|c| c.enabled).unwrap_or(false);
            let enabled_count = provider_config.map(|c| c.enabled_models_count()).unwrap_or(0);
            let provider_id = provider.id.clone();
            
            let card = div()
                .id(ElementId::Name(format!("provider-{}", provider.id.as_str()).into()))
                .w_full()
                .p_3()
                .rounded_lg()
                .cursor_pointer()
                .bg(if is_selected { rgb(0xf1f5f9) } else { rgb(0xffffff) })
                .border_1()
                .border_color(if is_selected { rgb(0x7c3aed) } else { rgb(0xe2e8f0) })
                .shadow_sm()
                .hover(|style| style.bg(rgb(0xf8fafc)).border_color(rgb(0xcbd5e1)))
                .child(
                    h_flex()
                        .w_full()
                        .gap_3()
                        .items_center()
                        .child(
                            div()
                                .size(px(36.0))
                                .rounded_md()
                                .bg(if is_selected { rgb(0x7c3aed) } else { rgb(0xf1f5f9) })
                                .flex()
                                .items_center()
                                .justify_center()
                                .child(Icon::new(provider.icon.clone()).size(px(20.0)))
                        )
                        .child(
                            v_flex()
                                .flex_1()
                                .gap_1()
                                .child(
                                    Label::new(provider.name.clone())
                                        .text_sm()
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .text_color(if is_selected { rgb(0x1e293b) } else { rgb(0x334155) })
                                )
                        )
                        .child(
                            h_flex()
                                .gap_2()
                                .items_center()
                                .child(
                                    div()
                                        .px_2()
                                        .py(px(1.0))
                                        .rounded_full()
                                        .bg(if enabled_count > 0 { rgb(0xd1fae5) } else { rgb(0xf3f4f6) })
                                        .child(
                                            Label::new(format!("{}", enabled_count))
                                                .text_xs()
                                                .text_color(if enabled_count > 0 { rgb(0x059669) } else { rgb(0x6b7280) })
                                        )
                                )
                                .child(
                                    div()
                                        .size(px(8.0))
                                        .rounded_full()
                                        .bg(if is_enabled { rgb(0x10b981) } else { rgb(0xd1d5db) })
                                )
                        )
                )
                .on_click(cx.listener(move |this, _, _window, cx| {
                    this.select_provider(provider_id.clone(), cx);
                }));
            
            list = list.child(card);
        }
        
        // Right panel - Configuration
        let config_panel = if let Some(provider) = provider_ui {
            let config = config.unwrap_or_else(|| AiProviderConfig {
                enabled: false,
                endpoint: match provider.id.as_str() {
                    "kimi" => "https://api.moonshot.cn".to_string(),
                    "deepseek" => "https://api.deepseek.com".to_string(),
                    "openai" => "https://api.openai.com/v1".to_string(),
                    "claude" => "https://api.anthropic.com".to_string(),
                    "ollama" => "http://localhost:11434".to_string(),
                    _ => String::new(),
                },
                api_key: None,
                models: vec![],
                region: None,
                alt_endpoints: vec![],
                claude_code: None,
            });
            self.render_provider_config(&provider, &config, cx)
        } else {
            div().flex_1().into_any_element()
        };
        
        // Main layout
        v_flex()
            .flex_1()
            .h_full()
            .gap_4()
            .child(
                h_flex()
                    .gap_4()
                    .items_center()
                    .child(
                        div()
                            .flex_1()
                            .child(Label::new("AI Providers").text_xl().font_weight(FontWeight::BOLD))
                    )
                    .child(
                        h_flex()
                            .gap_2()
                            .child(
                                Button::new("add-custom-acp")
                                    .label("Add Custom ACP Provider")
                                    .ghost()
                            )
                            .child(
                                Button::new("add-custom-provider")
                                    .label("Add Custom Provider")
                                    .primary()
                            )
                    ),
            )
            .child(
                h_flex()
                    .flex_1()
                    .gap_6()
                    .overflow_hidden()
                    .child(list)
                    .child(config_panel),
            )
            .child(render_status_message(status_message, status_is_error))
    }
}

impl AiSettingsPanel {
    fn render_provider_config(&self, provider: &ProviderUi, config: &AiProviderConfig, cx: &Context<Self>) -> AnyElement {
        let is_active = config.enabled;
        let models = self.filtered_models();
        
        let mut panel = v_flex()
            .flex_1()
            .h_full()
            .p_6()
            .gap_5()
            .rounded_xl()
            .bg(rgb(0xffffff))
            .border_1()
            .border_color(rgb(0xe2e8f0))
            .shadow_sm();
        
        // Header
        panel = panel.child(
            h_flex()
                .items_start()
                .justify_between()
                .child(
                    v_flex()
                        .gap_2()
                        .child(
                            h_flex()
                                .gap_3()
                                .items_center()
                                .child(
                                    Label::new(provider.name.clone())
                                        .text_2xl()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(rgb(0x1e293b))
                                )
                                .child(
                                    div()
                                        .px_3()
                                        .py_1()
                                        .rounded_full()
                                        .bg(if is_active { rgb(0xd1fae5) } else { rgb(0xf3f4f6) })
                                        .child(
                                            Label::new(if is_active { "Active" } else { "Inactive" })
                                                .text_xs()
                                                .font_weight(FontWeight::SEMIBOLD)
                                                .text_color(if is_active { rgb(0x059669) } else { rgb(0x6b7280) })
                                        )
                                )
                        )
                        .child(
                            Label::new(provider.description.clone())
                                .text_color(rgb(0x64748b))
                        )
                )
                .child(
                    h_flex()
                        .gap_3()
                        .items_center()
                        .child(
                            Switch::new("provider-toggle").checked(is_active)
                                .on_click(cx.listener(|this, _, _, cx| {
                                    this.toggle_provider_enabled(cx);
                                }))
                        )
                )
        );
        
        // Model selector (placeholder)
        if !models.is_empty() {
            panel = panel.child(
                h_flex()
                    .gap_2()
                    .child(
                        div()
                            .flex_1()
                            .p_3()
                            .rounded_md()
                            .bg(rgb(0xf9fafb))
                            .border_1()
                            .border_color(rgb(0xe5e7eb))
                            .child(
                                h_flex()
                                    .gap_2()
                                    .items_center()
                                    .child(Icon::new(IconName::Search).size(px(16.0)))
                                    .child(
                                        Label::new("Select model to test...")
                                            .text_color(rgb(0x9ca3af))
                                    )
                            )
                    )
            );
        }
        
        // API Configuration
        panel = panel.child(
            v_flex()
                .gap_4()
                .p_4()
                .rounded_lg()
                .bg(rgb(0xf8fafc))
                .border_1()
                .border_color(rgb(0xe2e8f0))
                .child(
                    h_flex()
                        .gap_2()
                        .items_center()
                        .child(Icon::new(IconName::Settings).size(px(16.0)))
                        .child(
                            Label::new("API Configuration")
                                .text_sm()
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(rgb(0x374151))
                        )
                        .child(
                            div()
                                .px_2()
                                .py(px(1.0))
                                .rounded_md()
                                .bg(rgb(0xf3f4f6))
                                .child(Label::new("Advanced").text_xs().text_color(rgb(0x6b7280)))
                        )
                )
                .child(
                    Label::new(format!("Configure API endpoints for {}", provider.name))
                        .text_xs()
                        .text_color(rgb(0x6b7280))
                )
                .child(
                    v_flex()
                        .gap_2()
                        .child(
                            h_flex()
                                .gap_2()
                                .items_center()
                                .child(Label::new("API Endpoint").text_sm().font_weight(FontWeight::MEDIUM))
                        )
                        .child(
                            h_flex()
                                .gap_2()
                                .child(
                                    div()
                                        .flex_1()
                                        .p_2()
                                        .rounded_md()
                                        .bg(rgb(0xffffff))
                                        .border_1()
                                        .border_color(rgb(0xe5e7eb))
                                        .child(Label::new(&config.endpoint))
                                )
                                .child(
                                    Button::new("copy-endpoint")
                                        .ghost()
                                        .icon(Icon::new(IconName::Copy))
                                )
                        )
                )
        );
        
        // Kimi Code / Claude Code Integration
        if provider.id.as_str() == "kimi" {
            panel = panel.child(self.render_kimi_code_section(config));
        }
        
        // Models Section
        panel = panel.child(
            h_flex()
                .items_center()
                .justify_between()
                .child(
                    Label::new("Models")
                        .text_sm()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(0x1e293b))
                )
                .child(
                    h_flex()
                        .gap_2()
                        .child(
                            Button::new("fetch-models")
                                .ghost()
                                .icon(Icon::new(IconName::Bot))
                                .label("Fetch")
                                .on_click(cx.listener(|this, _, _, cx| {
                                    if !this.is_loading {
                                        this.fetch_models(cx);
                                    }
                                }))
                        )
                )
        );
        
        // Model Search
        panel = panel.child(
            div()
                .p_3()
                .rounded_md()
                .bg(rgb(0xf9fafb))
                .border_1()
                .border_color(rgb(0xe5e7eb))
                .child(
                    h_flex()
                        .gap_2()
                        .items_center()
                        .child(Icon::new(IconName::Search).size(px(16.0)))
                        .child(Label::new("Search models...").text_color(rgb(0x9ca3af)))
                )
        );
        
        // Models List
        if !models.is_empty() {
            panel = panel.child(
                Label::new(format!("Showing {} models (enabled first)", models.len()))
                    .text_xs()
                    .text_color(rgb(0x9ca3af))
            );
            
            let mut models_list = v_flex().gap_2();
            
            for model in models {
                let model_id = model.id.clone();
                let is_model_enabled = model.enabled;
                
                let model_card = div()
                    .p_3()
                    .rounded_md()
                    .bg(rgb(0xf9fafb))
                    .border_1()
                    .border_color(rgb(0xe5e7eb))
                    .child(
                        h_flex()
                            .items_center()
                            .justify_between()
                            .child(
                                v_flex()
                                    .gap_1()
                                    .child(
                                        Label::new(model.name.clone())
                                            .text_sm()
                                            .font_weight(FontWeight::MEDIUM)
                                            .text_color(rgb(0x1f2937))
                                    )
                                    .child(
                                        Label::new(format!("{} tokens", model.token_limit.unwrap_or(0) / 1000))
                                            .text_xs()
                                            .text_color(rgb(0x9ca3af))
                                    )
                            )
                            .child(
                                h_flex()
                                    .gap_2()
                                    .items_center()
                                    .child(Label::new(model.id.clone()).text_xs().text_color(rgb(0x9ca3af)))
                                    .child(
                                        Switch::new(format!("model-{}", model.id))
                                            .checked(is_model_enabled)
                                            .on_click(cx.listener(move |this, _, _, cx| {
                                                this.toggle_model_enabled(&model_id, cx);
                                            }))
                                    )
                            )
                    );
                
                models_list = models_list.child(model_card);
            }
            
            panel = panel.child(models_list);
        }
        
        // Bottom actions
        panel = panel.child(
            h_flex()
                .gap_3()
                .pt_4()
                .border_t_1()
                .border_color(rgb(0xe2e8f0))
                .child(
                    Button::new("test-connection")
                        .label("Test Connection")
                        .on_click(cx.listener(|this, _, _, cx| {
                            if !this.is_loading {
                                this.test_connection(cx);
                            }
                        }))
                )
                .child(div().flex_1())
                .child(
                    Button::new("close")
                        .label("Close")
                        .ghost()
                )
                .child(
                    Button::new("save")
                        .label("Save")
                        .primary()
                )
        );
        
        panel.into_any_element()
    }
    
    fn render_kimi_code_section(&self, config: &AiProviderConfig) -> impl IntoElement {
        v_flex()
            .gap_3()
            .p_4()
            .rounded_lg()
            .bg(rgb(0xf8fafc))
            .border_1()
            .border_color(rgb(0xe2e8f0))
            .child(
                h_flex()
                    .gap_2()
                    .items_center()
                    .child(Icon::new(IconName::SquareTerminal).size(px(16.0)))
                    .child(
                        Label::new("Kimi Code Integration")
                            .text_sm()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(0x374151))
                    )
            )
            .child(
                Label::new("Use Kimi with Claude Code by setting these environment variables:")
                    .text_xs()
                    .text_color(rgb(0x6b7280))
            )
            .child(
                v_flex()
                    .p_3()
                    .rounded_md()
                    .bg(rgb(0x1f2937))
                    .child(
                        Label::new(format!(
                            "export ANTHROPIC_BASE_URL={}/v1\nexport ANTHROPIC_MODEL=kimi-k2.5\nexport ANTHROPIC_SMALL_FAST_MODEL=moonshot-v1-8k",
                            config.endpoint
                        ))
                            .text_xs()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgb(0x10b981))
                    )
            )
            .child(
                Label::new("Or use the OpenAI compatible endpoint directly:")
                    .text_xs()
                    .text_color(rgb(0x6b7280))
            )
            .child(
                v_flex()
                    .p_3()
                    .rounded_md()
                    .bg(rgb(0x1f2937))
                    .child(
                        Label::new(format!(
                            "export OPENAI_API_KEY=your-api-key\nexport OPENAI_BASE_URL={}/v1",
                            config.endpoint
                        ))
                            .text_xs()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgb(0x3b82f6))
                    )
            )
    }
}

fn render_status_message(msg: Option<String>, is_error: bool) -> impl IntoElement {
    if let Some(message) = msg {
        h_flex()
            .p_3()
            .rounded_lg()
            .bg(rgb(if is_error { 0xfef2f2 } else { 0xf0fdf4 }))
            .border_1()
            .border_color(rgb(if is_error { 0xfca5a5 } else { 0x86efac }))
            .child(
                Label::new(message)
                    .text_sm()
                    .text_color(rgb(if is_error { 0xef4444 } else { 0x15803d }))
            )
            .into_any_element()
    } else {
        div().into_any_element()
    }
}
