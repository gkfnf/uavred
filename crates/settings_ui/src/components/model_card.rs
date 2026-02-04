//! Model Card Component for AI Settings - Alma Style
//!
//! Displays an AI model with compact Alma-style design.

use gpui::*;
use gpui::prelude::FluentBuilder;
use gpui_component::{
    h_flex, v_flex,
    label::Label,
    switch::Switch,
    IconName, Icon,
};

use crate::config::AiModel;

use std::rc::Rc;

/// Model capability badge - Alma style small pill
#[derive(IntoElement)]
pub struct CapabilityBadge {
    #[allow(dead_code)]
    icon: IconName,
    label: SharedString,
    color: u32,
}

impl CapabilityBadge {
    pub fn vision() -> Self {
        Self {
            icon: IconName::Eye,
            label: "Vision".into(),
            color: 0x8b5cf6, // Purple
        }
    }

    pub fn reasoning() -> Self {
        Self {
            icon: IconName::Cpu,
            label: "Reasoning".into(),
            color: 0x06b6d4, // Cyan
        }
    }

    pub fn tools() -> Self {
        Self {
            icon: IconName::Settings,
            label: "Tools".into(),
            color: 0xf59e0b, // Amber
        }
    }
}

impl RenderOnce for CapabilityBadge {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .px_2()
            .py_1()
            .rounded_full()
            .bg(rgb(0xf3f4f6))
            .child(
                Label::new(self.label)
                    .text_xs()
                    .text_color(rgb(self.color))
            )
    }
}

/// Compact model card - Alma style
#[derive(IntoElement)]
pub struct ModelCard {
    model: AiModel,
    is_enabled: bool,
    on_toggle: Option<Rc<dyn Fn(&mut Window, &mut App) + 'static>>,
}

impl ModelCard {
    pub fn new(model: AiModel) -> Self {
        let is_enabled = model.enabled;
        Self {
            model,
            is_enabled,
            on_toggle: None,
        }
    }

    pub fn on_toggle(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_toggle = Some(Rc::new(handler));
        self
    }

    fn format_token_count(tokens: Option<u32>) -> String {
        match tokens {
            Some(t) if t >= 1_000_000 => format!("{:.1}M", t as f32 / 1_000_000.0),
            Some(t) if t >= 1_000 => format!("{}K", t / 1_000),
            Some(t) => format!("{}", t),
            None => "-".to_string(),
        }
    }
}

impl RenderOnce for ModelCard {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let model_id = self.model.id.clone();
        let on_toggle = self.on_toggle.clone();

        div()
            .p_3()
            .rounded_md()
            .bg(rgb(0xf9fafb))
            .border_1()
            .border_color(rgb(0xe5e7eb))
            .hover(|style| style.bg(rgb(0xf3f4f6)))
            .child(
                h_flex()
                    .items_center()
                    .justify_between()
                    .child(
                        h_flex()
                            .gap_3()
                            .items_center()
                            .child(
                                // Model icon based on capabilities
                                div()
                                    .size(px(8.0))
                                    .rounded_full()
                                    .bg(if self.model.supports_vision.unwrap_or(false) {
                                        rgb(0x8b5cf6)
                                    } else if self.model.supports_reasoning.unwrap_or(false) {
                                        rgb(0x06b6d4)
                                    } else {
                                        rgb(0x9ca3af)
                                    })
                            )
                            .child(
                                v_flex()
                                    .gap_1()
                                    .child(
                                        Label::new(self.model.name.clone())
                                            .text_sm()
                                            .font_weight(FontWeight::MEDIUM)
                                            .text_color(rgb(0x1f2937))
                                    )
                                    .child(
                                        h_flex()
                                            .gap_2()
                                            .items_center()
                                            .child(
                                                Label::new(format!("{} tokens", Self::format_token_count(self.model.token_limit)))
                                                    .text_xs()
                                                    .text_color(rgb(0x9ca3af))
                                            )
                                            .when(self.model.supports_vision.unwrap_or(false), |this| {
                                                this.child(CapabilityBadge::vision())
                                            })
                                            .when(self.model.supports_reasoning.unwrap_or(false), |this| {
                                                this.child(CapabilityBadge::reasoning())
                                            })
                                    )
                            )
                    )
                    .child(
                        h_flex()
                            .gap_3()
                            .items_center()
                            .child(
                                Label::new(model_id.clone())
                                    .text_xs()
                                    .text_color(rgb(0x9ca3af))
                                    .font_family("monospace")
                            )
                            .child(
                                Switch::new(format!("model-toggle-{}", model_id))
                                    .checked(self.is_enabled)
                                    .on_click(move |_event, window, cx| {
                                        if let Some(ref handler) = on_toggle {
                                            handler(window, cx);
                                        }
                                    })
                            )
                    )
            )
    }
}

/// Empty state for models list
#[derive(IntoElement)]
pub struct EmptyModelsState {
    message: SharedString,
}

impl EmptyModelsState {
    pub fn new(message: impl Into<SharedString>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl RenderOnce for EmptyModelsState {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        v_flex()
            .flex_1()
            .items_center()
            .justify_center()
            .p_8()
            .gap_3()
            .child(
                Icon::new(IconName::Bot)
                    .size(px(32.0))
                    .text_color(rgb(0xd1d5db))
            )
            .child(
                Label::new(self.message)
                    .text_sm()
                    .text_color(rgb(0x9ca3af))
            )
    }
}
