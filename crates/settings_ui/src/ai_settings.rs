//! AI Settings Panel - Alma-style provider configuration

use gpui::*;
use gpui_component::{
    v_flex, h_flex,
    button::{Button, ButtonVariants},
    input::InputState,
    label::Label,
    IconName, Icon, Sizable,
};

use crate::components::{
    ApiConfigSection, CodeBlockStyle, ConfigActions, IntegrationSection, ModelsSection,
    ProviderConfigHeader,
};
use crate::config::{Settings, AiProviderConfig, AiModel};
use crate::provider::{AiProvider, ProviderId};
use crate::provider::kimi::KimiProvider;
use crate::provider::deepseek::DeepSeekProvider;

/// AI Provider UI definition
#[derive(Clone)]
pub struct ProviderUi {
    pub id: ProviderId,
    pub name: String,
    pub description: String,
    pub icon: IconName,
    #[allow(dead_code)]
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
    provider_search_query: String,
    is_loading: bool,
    // Connection test result
    connection_latency: Option<u32>,
    // Provider instances
    kimi_provider: KimiProvider,
    deepseek_provider: DeepSeekProvider,
    // Input states for editable fields
    endpoint_input: Entity<InputState>,
    api_key_input: Entity<InputState>,
    model_search_input: Entity<InputState>,
    provider_search_input: Entity<InputState>,
}

impl AiSettingsPanel {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let settings = Settings::load().unwrap_or_default();
        let active_provider = ProviderId::new(&settings.ai.active_provider);

        // Initialize input states with values from current provider config
        let endpoint_input = cx.new(|cx| {
            InputState::new(window, cx)
        });
        let api_key_input = cx.new(|cx| {
            InputState::new(window, cx)
        });
        let model_search_input = cx.new(|cx| InputState::new(window, cx));
        let provider_search_input = cx.new(|cx| InputState::new(window, cx));
        
        // Subscribe to provider search input changes
        let provider_search_input_clone = provider_search_input.clone();
        cx.subscribe(&provider_search_input, move |this, _input_state, event: &gpui_component::input::InputEvent, cx| {
            if matches!(event, gpui_component::input::InputEvent::Change) {
                this.provider_search_query = provider_search_input_clone.read(cx).text().to_string();
                cx.notify();
            }
        }).detach();
        
        // Subscribe to endpoint input changes - save to config
        let endpoint_input_clone = endpoint_input.clone();
        cx.subscribe(&endpoint_input, move |this, _input_state, event: &gpui_component::input::InputEvent, cx| {
            if matches!(event, gpui_component::input::InputEvent::Change) {
                let endpoint = endpoint_input_clone.read(cx).text().to_string();
                if let Some(config) = this.get_current_provider_config_mut() {
                    config.endpoint = endpoint;
                }
            }
        }).detach();
        
        // Subscribe to API key input changes - save to config
        let api_key_input_clone = api_key_input.clone();
        cx.subscribe(&api_key_input, move |this, _input_state, event: &gpui_component::input::InputEvent, cx| {
            if matches!(event, gpui_component::input::InputEvent::Change) {
                let api_key = api_key_input_clone.read(cx).text().to_string();
                if let Some(config) = this.get_current_provider_config_mut() {
                    config.api_key = if api_key.is_empty() { None } else { Some(api_key) };
                }
            }
        }).detach();

        Self {
            providers_ui: ProviderUi::all(),
            selected_provider_id: active_provider.clone(),
            settings: settings.clone(),
            status_message: None,
            status_is_error: false,
            model_search_query: String::new(),
            provider_search_query: String::new(),
            is_loading: false,
            connection_latency: None,
            kimi_provider: KimiProvider::new(),
            deepseek_provider: DeepSeekProvider::new(),
            endpoint_input,
            api_key_input,
            model_search_input,
            provider_search_input,
        }
    }

    fn select_provider(&mut self, id: ProviderId, window: &mut Window, cx: &mut Context<Self>) {
        self.selected_provider_id = id.clone();
        self.status_message = None;
        self.model_search_query = String::new();
        self.connection_latency = None;
        
        // Update input fields with current provider config
        if let Some(config) = self.get_current_provider_config() {
            self.endpoint_input.update(cx, |state, cx| {
                state.set_value(&config.endpoint, window, cx);
            });
            if let Some(api_key) = &config.api_key {
                self.api_key_input.update(cx, |state, cx| {
                    state.set_value(api_key, window, cx);
                });
            } else {
                self.api_key_input.update(cx, |state, cx| {
                    state.set_value("", window, cx);
                });
            }
        }
        
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
        self.connection_latency = None;
        cx.notify();

        let start_time = std::time::Instant::now();

        let provider_id = self.selected_provider_id.as_str();
        let task = match provider_id {
            "kimi" => self.kimi_provider.test_connection(cx),
            "deepseek" => self.deepseek_provider.test_connection(cx),
            _ => {
                self.is_loading = false;
                self.status_message = Some("Connection test not implemented for this provider".to_string());
                self.status_is_error = false;
                cx.notify();
                return;
            }
        };
        
        cx.spawn(async move |this, cx| {
            let result = task.await;
            let latency = start_time.elapsed().as_millis() as u32;
            cx.update(|cx| {
                this.update(cx, |this, cx| {
                    this.is_loading = false;
                    this.connection_latency = Some(latency);
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

    fn fetch_models(&mut self, cx: &mut Context<Self>) {
        self.is_loading = true;
        self.status_message = Some("Fetching models...".to_string());
        self.status_is_error = false;
        cx.notify();

        // Get current values from input fields
        let endpoint = self.endpoint_input.read(cx).text().to_string();
        let api_key = self.api_key_input.read(cx).text().to_string();
        
        // Update config with current input values
        if let Some(config) = self.get_current_provider_config_mut() {
            config.endpoint = endpoint.clone();
            config.api_key = if api_key.is_empty() { None } else { Some(api_key.clone()) };
        }

        let provider_id = self.selected_provider_id.as_str();
        
        // Update provider instances with current API key before fetching
        match provider_id {
            "deepseek" => {
                self.deepseek_provider.set_api_key(if api_key.is_empty() { None } else { Some(api_key) });
            }
            "kimi" => {
                self.kimi_provider.set_api_key(if api_key.is_empty() { None } else { Some(api_key) });
            }
            _ => {}
        }
        
        let task = match provider_id {
            "kimi" => self.kimi_provider.fetch_models(cx),
            "deepseek" => self.deepseek_provider.fetch_models(cx),
            _ => {
                self.is_loading = false;
                self.status_message = Some("Fetch models not implemented for this provider".to_string());
                self.status_is_error = false;
                cx.notify();
                return;
            }
        };
        
        cx.spawn(async move |this, cx| {
            let result = task.await;
            cx.update(|cx| {
                this.update(cx, |this, cx| {
                    this.is_loading = false;
                    match result {
                        Ok(models) => {
                            // Update config with fetched models
                            let model_count = models.len();
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
                            this.status_message = Some(format!("Successfully fetched {} models", model_count));
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
                    m.id.to_lowercase().contains(&query)
                        || m.name.to_lowercase().contains(&query)
                })
                .cloned()
                .collect()
        }
    }

    fn filtered_providers(&self) -> Vec<ProviderUi> {
        if self.provider_search_query.is_empty() {
            self.providers_ui.clone()
        } else {
            let query = self.provider_search_query.to_lowercase();
            self.providers_ui
                .iter()
                .filter(|p| {
                    p.name.to_lowercase().contains(&query)
                        || p.description.to_lowercase().contains(&query)
                })
                .cloned()
                .collect()
        }
    }

    fn get_default_endpoint(&self, provider_id: &str) -> String {
        match provider_id {
            "kimi" => "https://api.moonshot.cn".to_string(),
            "deepseek" => "https://api.deepseek.com".to_string(),
            "openai" => "https://api.openai.com/v1".to_string(),
            "claude" => "https://api.anthropic.com".to_string(),
            "ollama" => "http://localhost:11434".to_string(),
            _ => String::new(),
        }
    }

    fn render_integration_section(&self, provider_id: &str, config: &AiProviderConfig) -> Option<impl IntoElement> {
        match provider_id {
            "kimi" => {
                Some(
                    IntegrationSection::new("Kimi Code Integration")
                        .description("Use Kimi with Claude Code by setting these environment variables:")
                        .add_code_block(
                            format!(
                                "export ANTHROPIC_BASE_URL={}/v1\nexport ANTHROPIC_MODEL=kimi-k2.5\nexport ANTHROPIC_SMALL_FAST_MODEL=moonshot-v1-8k",
                                config.endpoint
                            ),
                            CodeBlockStyle::Green,
                        )
                        .add_code_block(
                            format!(
                                "export OPENAI_API_KEY=your-api-key\nexport OPENAI_BASE_URL={}/v1",
                                config.endpoint
                            ),
                            CodeBlockStyle::Blue,
                        )
                )
            }
            "deepseek" => {
                Some(
                    IntegrationSection::new("DeepSeek Integration")
                        .description("Configure DeepSeek for use with compatible tools:")
                        .add_code_block(
                            format!(
                                "export OPENAI_API_KEY=your-api-key\nexport OPENAI_BASE_URL={}/v1",
                                config.endpoint
                            ),
                            CodeBlockStyle::Blue,
                        )
                )
            }
            _ => None,
        }
    }
}

impl Render for AiSettingsPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let provider_ui = self.selected_provider_ui().cloned();
        let status_message = self.status_message.clone();
        let status_is_error = self.status_is_error;
        let filtered_providers = self.filtered_providers();

        // Left sidebar - Provider list with Alma-style search
        let mut list = v_flex()
            .w(px(280.0))
            .h_full()
            .gap_2();

        // Provider search box - use Input component
        list = list.child(
            div()
                .px_2()
                .child(
                    gpui_component::input::Input::new(&self.provider_search_input)
                        .prefix(Icon::new(IconName::Search).size(px(14.0)))
                )
        );

        // Provider list
        for provider in filtered_providers {
            let is_selected = provider.id == self.selected_provider_id;
            let provider_config = self.settings.ai.providers.get(provider.id.as_str());
            let is_enabled = provider_config.map(|c| c.enabled).unwrap_or(false);
            let enabled_count = provider_config.map(|c| c.enabled_models_count()).unwrap_or(0);
            let provider_id = provider.id.clone();

            let card = div()
                .id(ElementId::Name(format!("provider-{}", provider.id.as_str()).into()))
                .w_full()
                .p_2()
                .rounded_md()
                .cursor_pointer()
                .bg(if is_selected { rgb(0xf3f4f6) } else { rgb(0xffffff) })
                .border_1()
                .border_color(if is_selected { rgb(0x6b7280) } else { rgb(0xe5e7eb) })
                .hover(|style| style.bg(rgb(0xf9fafb)).border_color(rgb(0xd1d5db)))
                .child(
                    h_flex()
                        .w_full()
                        .gap_2()
                        .items_center()
                        .child(
                            div()
                                .size(px(28.0))
                                .rounded_md()
                                .bg(if is_selected { rgb(0x6b7280) } else { rgb(0xf3f4f6) })
                                .flex()
                                .items_center()
                                .justify_center()
                                .child(Icon::new(provider.icon.clone()).size(px(14.0)))
                        )
                        .child(
                            v_flex()
                                .flex_1()
                                .gap_0()
                                .child(
                                    Label::new(provider.name.clone())
                                        .text_sm()
                                        .font_weight(FontWeight::MEDIUM)
                                        .text_color(if is_selected { rgb(0x1a1a1a) } else { rgb(0x4b5563) })
                                )
                        )
                        .child(
                            h_flex()
                                .gap_2()
                                .items_center()
                                .child(
                                    div()
                                        .px_1_5()
                                        .py_0()
                                        .rounded_full()
                                        .bg(if enabled_count > 0 { rgb(0xdcfce7) } else { rgb(0xf3f4f6) })
                                        .child(
                                            Label::new(format!("{}", enabled_count))
                                                .text_xs()
                                                .text_color(if enabled_count > 0 { rgb(0x16a34a) } else { rgb(0x9ca3af) })
                                        )
                                )
                                .child(
                                    div()
                                        .size(px(6.0))
                                        .rounded_full()
                                        .bg(if is_enabled { rgb(0x22c55e) } else { rgb(0xd1d5db) })
                                )
                        )
                )
                .on_click(cx.listener(move |this, _, window, cx| {
                    this.select_provider(provider_id.clone(), window, cx);
                }));

            list = list.child(card);
        }

        // Right panel - Configuration
        let config_panel = if let Some(provider) = provider_ui {
            let config = self.get_current_provider_config()
                .cloned()
                .unwrap_or_else(|| AiProviderConfig {
                    enabled: false,
                    endpoint: self.get_default_endpoint(provider.id.as_str()),
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

        // Main layout with Alma-style header
        v_flex()
            .flex_1()
            .h_full()
            .gap_4()
            .px_6()
            .py_4()
            .bg(rgb(0xf9fafb))
            .child(
                h_flex()
                    .gap_4()
                    .items_center()
                    .child(
                        div()
                            .flex_1()
                            .child(
                                Label::new("AI Providers")
                                    .text_xl()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0x1a1a1a))
                            )
                    )
                    .child(
                        h_flex()
                            .gap_2()
                            .child(
                                Button::new("add-custom-acp")
                                    .label("Add Custom ACP Provider")
                                    .ghost()
                                    .small()
                            )
                            .child(
                                Button::new("add-custom-provider")
                                    .label("Add Custom Provider")
                                    .primary()
                                    .small()
                            )
                    ),
            )
            .child(
                h_flex()
                    .flex_1()
                    .gap_4()
                    .overflow_hidden()
                    .child(list)
                    .child(config_panel),
            )
            .child(render_status_message(status_message, status_is_error))
    }
}

impl AiSettingsPanel {
    fn render_provider_config(&mut self, provider: &ProviderUi, config: &AiProviderConfig, cx: &mut Context<Self>) -> AnyElement {
        let is_active = config.enabled;
        let models = self.filtered_models();
        let provider_id = provider.id.as_str();
        
        // Build header with connection status
        let mut header = ProviderConfigHeader::new(&provider.name, &provider.description)
            .enabled(is_active);
        
        if let Some(latency) = self.connection_latency {
            header = header.connection_status(!self.status_is_error, latency);
        }

        let weak = cx.weak_entity();
        let header_with_toggle = header.on_toggle(move |_window, cx| {
            if let Some(this) = weak.upgrade() {
                this.update(cx, |this, cx| {
                    this.toggle_provider_enabled(cx);
                });
            }
        });

        let mut panel = v_flex()
            .flex_1()
            .h_full()
            .p_5()
            .gap_4()
            .rounded_xl()
            .bg(rgb(0xffffff))
            .border_1()
            .border_color(rgb(0xe5e7eb));

        // Header
        panel = panel.child(header_with_toggle);

        // API Configuration section
        panel = panel.child(
            ApiConfigSection::new(config.endpoint.clone(), config.api_key.clone().unwrap_or_default())
                .with_inputs(&self.endpoint_input, &self.api_key_input)
        );

        // Integration section (provider-specific)
        if let Some(integration) = self.render_integration_section(provider_id, config) {
            panel = panel.child(integration);
        }

        // Models section
        let weak_fetch = cx.weak_entity();
        let weak_toggle = cx.weak_entity();
        panel = panel.child(
            ModelsSection::new(models)
                .search_query(&self.model_search_query)
                .loading(self.is_loading)
                .on_fetch(move |_window, cx| {
                    if let Some(this) = weak_fetch.upgrade() {
                        this.update(cx, |this, cx| {
                            if !this.is_loading {
                                this.fetch_models(cx);
                            }
                        });
                    }
                })
                .on_model_toggle(move |model_id: String, _window, cx| {
                    if let Some(this) = weak_toggle.upgrade() {
                        this.update(cx, |this, cx| {
                            this.toggle_model_enabled(&model_id, cx);
                        });
                    }
                })
        );

        // Action buttons
        let weak_test = cx.weak_entity();
        let weak_save = cx.weak_entity();
        panel = panel.child(
            ConfigActions::new()
                .loading(self.is_loading)
                .on_test(move |_window, cx| {
                    if let Some(this) = weak_test.upgrade() {
                        this.update(cx, |this, cx| {
                            if !this.is_loading {
                                this.test_connection(cx);
                            }
                        });
                    }
                })
                .on_save(move |_window, cx| {
                    if let Some(this) = weak_save.upgrade() {
                        this.update(cx, |this, cx| {
                            this.save_settings(cx);
                        });
                    }
                })
        );

        panel.into_any_element()
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
