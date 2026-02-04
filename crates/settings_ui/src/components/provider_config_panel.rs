//! Provider Configuration Panel Component
//!
//! A reusable configuration panel for AI providers following Alma's design.

use gpui::*;
use gpui::prelude::FluentBuilder;
use gpui_component::{
    v_flex, h_flex,
    button::{Button, ButtonVariants},
    input::{Input, InputState},
    label::Label,
    IconName, Icon, Sizable,
};

use crate::config::AiModel;
use crate::components::{CodeBlock, CodeBlockStyle, ModelCard, EmptyModelsState};

use std::rc::Rc;

/// Header section for provider configuration with Alma-style design
#[derive(IntoElement)]
pub struct ProviderConfigHeader {
    name: SharedString,
    description: SharedString,
    is_enabled: bool,
    connection_status: Option<(bool, u32)>, // (success, latency_ms)
    on_toggle: Option<Rc<dyn Fn(&mut Window, &mut App) + 'static>>,
}

impl ProviderConfigHeader {
    pub fn new(name: impl Into<SharedString>, description: impl Into<SharedString>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            is_enabled: false,
            connection_status: None,
            on_toggle: None,
        }
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.is_enabled = enabled;
        self
    }

    pub fn connection_status(mut self, success: bool, latency_ms: u32) -> Self {
        self.connection_status = Some((success, latency_ms));
        self
    }

    pub fn on_toggle(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_toggle = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for ProviderConfigHeader {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        v_flex()
            .gap_2()
            .child(
                h_flex()
                    .gap_3()
                    .items_center()
                    .child(
                        Label::new(self.name.clone())
                            .text_xl()
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0x1a1a1a))
                    )
                    .child(
                        div()
                            .px_2()
                            .py_1()
                            .rounded_full()
                            .bg(if self.is_enabled { rgb(0xdcfce7) } else { rgb(0xf3f4f6) })
                            .border_1()
                            .border_color(if self.is_enabled { rgb(0x86efac) } else { rgb(0xe5e7eb) })
                            .child(
                                Label::new(if self.is_enabled { "Active" } else { "Inactive" })
                                    .text_xs()
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(if self.is_enabled { rgb(0x16a34a) } else { rgb(0x6b7280) })
                            )
                    )
            )
            .child(
                Label::new(self.description.clone())
                    .text_sm()
                    .text_color(rgb(0x6b7280))
            )
            .when_some(self.connection_status, |this, (success, latency)| {
                this.child(
                    h_flex()
                        .gap_2()
                        .items_center()
                        .child(
                            div()
                                .size(px(6.0))
                                .rounded_full()
                                .bg(if success { rgb(0x22c55e) } else { rgb(0xef4444) })
                        )
                        .child(
                            Label::new(if success { 
                                format!("Connection successful ({}ms)", latency)
                            } else { 
                                "Connection failed".into() 
                            })
                            .text_sm()
                            .text_color(if success { rgb(0x16a34a) } else { rgb(0xdc2626) })
                        )
                )
            })
    }
}

/// Collapsible API Configuration section (Alma-style accordion)
#[derive(IntoElement)]
pub struct ApiConfigSection {
    endpoint: SharedString,
    api_key: SharedString,
    is_expanded: bool,
    endpoint_input: Option<Entity<InputState>>,
    api_key_input: Option<Entity<InputState>>,
}

impl ApiConfigSection {
    pub fn new(endpoint: impl Into<SharedString>, api_key: impl Into<SharedString>) -> Self {
        Self {
            endpoint: endpoint.into(),
            api_key: api_key.into(),
            is_expanded: true,
            endpoint_input: None,
            api_key_input: None,
        }
    }

    pub fn with_inputs(
        mut self,
        endpoint_input: &Entity<InputState>,
        api_key_input: &Entity<InputState>,
    ) -> Self {
        self.endpoint_input = Some(endpoint_input.clone());
        self.api_key_input = Some(api_key_input.clone());
        self
    }
}

impl RenderOnce for ApiConfigSection {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        v_flex()
            .gap_4()
            .rounded_lg()
            .bg(rgb(0xf9fafb))
            .border_1()
            .border_color(rgb(0xe5e7eb))
            .child(
                // Accordion header
                h_flex()
                    .p_3()
                    .gap_2()
                    .items_center()
                    .rounded_t_lg()
                    .bg(rgb(0xf3f4f6))
                    .border_b_1()
                    .border_color(rgb(0xe5e7eb))
                    .child(Icon::new(IconName::Settings).size(px(16.0)).text_color(rgb(0x6b7280)))
                    .child(
                        Label::new("API Configuration")
                            .text_sm()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgb(0x374151))
                    )
                    .child(
                        div()
                            .px_2()
                            .py_1()
                            .rounded_full()
                            .bg(rgb(0xffffff))
                            .border_1()
                            .border_color(rgb(0xe5e7eb))
                            .child(Label::new("Advanced").text_xs().text_color(rgb(0x6b7280)))
                    )
            )
            .child(
                v_flex()
                    .p_4()
                    .gap_4()
                    // API Endpoint
                    .child(
                        v_flex()
                            .gap_2()
                            .child(
                                Label::new("API Endpoint")
                                    .text_sm()
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(rgb(0x374151))
                            )
                            .child(
                                h_flex()
                                    .gap_2()
                                    .child(
                                        if let Some(ref state) = self.endpoint_input {
                                            div()
                                                .flex_1()
                                                .child(Input::new(state))
                                                .into_any_element()
                                        } else {
                                            div()
                                                .flex_1()
                                                .px_3()
                                                .py_2()
                                                .rounded_md()
                                                .bg(rgb(0xffffff))
                                                .border_1()
                                                .border_color(rgb(0xe5e7eb))
                                                .child(
                                                    Label::new(self.endpoint.clone())
                                                        .text_sm()
                                                        .text_color(rgb(0x1f2937))
                                                )
                                                .into_any_element()
                                        }
                                    )
                                    .child(
                                        Button::new("copy-endpoint")
                                            .ghost()
                                            .icon(Icon::new(IconName::Copy))
                                            .on_click({
                                                let endpoint = self.endpoint.clone();
                                                move |_event, _window, cx| {
                                                    cx.write_to_clipboard(gpui::ClipboardItem::new_string(
                                                        endpoint.to_string(),
                                                    ));
                                                }
                                            })
                                    )
                            )
                    )
                    // API Key
                    .child(
                        v_flex()
                            .gap_2()
                            .child(
                                Label::new("API Key")
                                    .text_sm()
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(rgb(0x374151))
                            )
                            .child(
                                h_flex()
                                    .gap_2()
                                    .child(
                                        if let Some(ref state) = self.api_key_input {
                                            div()
                                                .flex_1()
                                                .child(Input::new(state).mask_toggle())
                                                .into_any_element()
                                        } else {
                                            div()
                                                .flex_1()
                                                .px_3()
                                                .py_2()
                                                .rounded_md()
                                                .bg(rgb(0xffffff))
                                                .border_1()
                                                .border_color(rgb(0xe5e7eb))
                                                .child(
                                                    Label::new(if self.api_key.is_empty() {
                                                        SharedString::from("Not configured")
                                                    } else {
                                                        SharedString::from("•".repeat(20))
                                                    })
                                                    .text_sm()
                                                    .text_color(if self.api_key.is_empty() {
                                                        rgb(0x9ca3af)
                                                    } else {
                                                        rgb(0x1f2937)
                                                    })
                                                )
                                                .into_any_element()
                                        }
                                    )
                                    .when(!self.api_key.is_empty(), |this| {
                                        this.child(
                                            Button::new("copy-apikey")
                                                .ghost()
                                                .icon(Icon::new(IconName::Copy))
                                                .on_click({
                                                    let api_key = self.api_key.clone();
                                                    move |_event, _window, cx| {
                                                        cx.write_to_clipboard(gpui::ClipboardItem::new_string(
                                                            api_key.to_string(),
                                                        ));
                                                    }
                                                })
                                        )
                                    })
                            )
                    )
            )
    }
}

/// Models section with Alma-style design
#[derive(IntoElement)]
pub struct ModelsSection {
    models: Vec<AiModel>,
    search_query: SharedString,
    is_loading: bool,
    on_search_change: Option<Rc<dyn Fn(SharedString, &mut Window, &mut App) + 'static>>,
    on_fetch: Option<Rc<dyn Fn(&mut Window, &mut App) + 'static>>,
    on_model_toggle: Option<Rc<dyn Fn(String, &mut Window, &mut App) + 'static>>,
}

impl ModelsSection {
    pub fn new(models: Vec<AiModel>) -> Self {
        Self {
            models,
            search_query: SharedString::default(),
            is_loading: false,
            on_search_change: None,
            on_fetch: None,
            on_model_toggle: None,
        }
    }

    pub fn search_query(mut self, query: impl Into<SharedString>) -> Self {
        self.search_query = query.into();
        self
    }

    pub fn loading(mut self, loading: bool) -> Self {
        self.is_loading = loading;
        self
    }

    pub fn on_search_change(
        mut self,
        handler: impl Fn(SharedString, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_search_change = Some(Rc::new(handler));
        self
    }

    pub fn on_fetch(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_fetch = Some(Rc::new(handler));
        self
    }

    pub fn on_model_toggle(
        mut self,
        handler: impl Fn(String, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_model_toggle = Some(Rc::new(handler));
        self
    }

    fn filter_models(&self) -> Vec<AiModel> {
        if self.search_query.is_empty() {
            let mut models = self.models.clone();
            models.sort_by(|a, b| {
                let a_enabled = if a.enabled { 0 } else { 1 };
                let b_enabled = if b.enabled { 0 } else { 1 };
                a_enabled.cmp(&b_enabled).then_with(|| a.name.cmp(&b.name))
            });
            models
        } else {
            let query = self.search_query.to_lowercase();
            self.models
                .iter()
                .filter(|m| {
                    m.id.to_lowercase().contains(&query)
                        || m.name.to_lowercase().contains(&query)
                })
                .cloned()
                .collect()
        }
    }
}

impl RenderOnce for ModelsSection {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let filtered_models = self.filter_models();
        let model_count = filtered_models.len();

        v_flex()
            .gap_3()
            // Header with Fetch button
            .child(
                h_flex()
                    .items_center()
                    .justify_between()
                    .child(
                        Label::new("Models")
                            .text_sm()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(0x374151))
                    )
                    .child(
                        Button::new("fetch-models")
                            .ghost()
                            .small()
                            .icon(Icon::new(IconName::Bot))
                            .label("Fetch")
                            .when(self.is_loading, |this| this.loading(true))
                            .on_click({
                                let on_fetch = self.on_fetch.clone();
                                move |_event, window, cx| {
                                    if let Some(ref handler) = on_fetch {
                                        handler(window, cx);
                                    }
                                }
                            })
                    )
            )
            // Search box with Alma styling
            .child(
                div()
                    .px_3()
                    .py_2()
                    .rounded_md()
                    .bg(rgb(0xf9fafb))
                    .border_1()
                    .border_color(rgb(0xe5e7eb))
                    .child(
                        h_flex()
                            .gap_2()
                            .items_center()
                            .child(Icon::new(IconName::Search).size(px(14.0)).text_color(rgb(0x9ca3af)))
                            .child(
                                Label::new(if self.search_query.is_empty() {
                                    SharedString::from("Search models...")
                                } else {
                                    SharedString::from(self.search_query.clone())
                                })
                                .text_sm()
                                .text_color(if self.search_query.is_empty() {
                                    rgb(0x9ca3af)
                                } else {
                                    rgb(0x1f2937)
                                })
                            )
                    )
            )
            // Model count
            .child(
                Label::new(format!("Showing {} models (enabled first)", model_count))
                    .text_xs()
                    .text_color(rgb(0x9ca3af))
            )
            // Model list - Alma style compact cards
            .child(
                if filtered_models.is_empty() {
                    EmptyModelsState::new("No models found. Click 'Fetch' to load models.")
                        .into_any_element()
                } else {
                    let mut list = v_flex().gap_2();
                    for model in filtered_models {
                        let model_id = model.id.clone();
                        let on_toggle = self.on_model_toggle.clone();
                        list = list.child(
                            ModelCard::new(model).on_toggle(move |window, cx| {
                                if let Some(ref handler) = on_toggle {
                                    handler(model_id.clone(), window, cx);
                                }
                            })
                        );
                    }
                    list.into_any_element()
                }
            )
    }
}

/// Integration code section (Alma-style)
#[derive(IntoElement)]
pub struct IntegrationSection {
    title: SharedString,
    description: SharedString,
    code_blocks: Vec<(SharedString, CodeBlockStyle)>,
}

impl IntegrationSection {
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            description: SharedString::default(),
            code_blocks: Vec::new(),
        }
    }

    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = description.into();
        self
    }

    pub fn add_code_block(
        mut self,
        code: impl Into<SharedString>,
        style: CodeBlockStyle,
    ) -> Self {
        self.code_blocks.push((code.into(), style));
        self
    }
}

impl RenderOnce for IntegrationSection {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut section = v_flex()
            .gap_3()
            .p_4()
            .rounded_lg()
            .bg(rgb(0xf9fafb))
            .border_1()
            .border_color(rgb(0xe5e7eb))
            .child(
                h_flex()
                    .gap_2()
                    .items_center()
                    .child(Icon::new(IconName::SquareTerminal).size(px(16.0)).text_color(rgb(0x6b7280)))
                    .child(
                        Label::new(self.title.clone())
                            .text_sm()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(0x374151))
                    )
            );

        if !self.description.is_empty() {
            section = section.child(
                Label::new(self.description.clone())
                    .text_xs()
                    .text_color(rgb(0x6b7280)),
            );
        }

        for (code, style) in self.code_blocks {
            section = section.child(CodeBlock::new(code).style(style));
        }

        section
    }
}

/// Bottom action bar (Alma-style fixed footer)
#[derive(IntoElement)]
pub struct ConfigActions {
    on_test: Option<Rc<dyn Fn(&mut Window, &mut App) + 'static>>,
    on_save: Option<Rc<dyn Fn(&mut Window, &mut App) + 'static>>,
    on_cancel: Option<Rc<dyn Fn(&mut Window, &mut App) + 'static>>,
    is_loading: bool,
}

impl ConfigActions {
    pub fn new() -> Self {
        Self {
            on_test: None,
            on_save: None,
            on_cancel: None,
            is_loading: false,
        }
    }

    pub fn loading(mut self, loading: bool) -> Self {
        self.is_loading = loading;
        self
    }

    pub fn on_test(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_test = Some(Rc::new(handler));
        self
    }

    pub fn on_save(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_save = Some(Rc::new(handler));
        self
    }

    pub fn on_cancel(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_cancel = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for ConfigActions {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        h_flex()
            .gap_3()
            .pt_4()
            .border_t_1()
            .border_color(rgb(0xe5e7eb))
            .child(
                Button::new("test-connection")
                    .label("Test Connection")
                    .when(self.is_loading, |this| this.loading(true))
                    .on_click({
                        let on_test = self.on_test.clone();
                        move |_event, window, cx| {
                            if let Some(ref handler) = on_test {
                                handler(window, cx);
                            }
                        }
                    })
            )
            .child(div().flex_1())
            .child(
                Button::new("cancel")
                    .label("Cancel")
                    .ghost()
                    .on_click({
                        let on_cancel = self.on_cancel.clone();
                        move |_event, window, cx| {
                            if let Some(ref handler) = on_cancel {
                                handler(window, cx);
                            }
                        }
                    })
            )
            .child(
                Button::new("save")
                    .label("Save")
                    .primary()
                    .on_click({
                        let on_save = self.on_save.clone();
                        move |_event, window, cx| {
                            if let Some(ref handler) = on_save {
                                handler(window, cx);
                            }
                        }
                    })
            )
    }
}
