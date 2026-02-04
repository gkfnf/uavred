//! Editable Text Field Component for Settings
//!
//! A text input field with label, copy button, and edit capability.

use gpui::*;
use gpui::prelude::FluentBuilder;
use gpui_component::{
    h_flex, v_flex,
    button::{Button, ButtonVariants},
    input::{Input, InputState},
    label::Label,
    IconName, Icon,
};

use std::rc::Rc;

/// Text field configuration
pub struct TextField {
    id: ElementId,
    label: SharedString,
    value: SharedString,
    placeholder: SharedString,
    is_editable: bool,
    is_password: bool,
    show_copy: bool,
    on_change: Option<Rc<dyn Fn(SharedString, &mut Window, &mut App) + 'static>>,
    on_copy: Option<Rc<dyn Fn(&mut Window, &mut App) + 'static>>,
}

impl TextField {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            value: SharedString::default(),
            placeholder: SharedString::default(),
            is_editable: true,
            is_password: false,
            show_copy: true,
            on_change: None,
            on_copy: None,
        }
    }

    pub fn value(mut self, value: impl Into<SharedString>) -> Self {
        self.value = value.into();
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn editable(mut self, editable: bool) -> Self {
        self.is_editable = editable;
        self
    }

    pub fn password(mut self, is_password: bool) -> Self {
        self.is_password = is_password;
        self
    }

    pub fn show_copy(mut self, show: bool) -> Self {
        self.show_copy = show;
        self
    }

    pub fn on_change(mut self, handler: impl Fn(SharedString, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }

    pub fn on_copy(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_copy = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for TextField {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let value = self.value.clone();
        let show_copy = self.show_copy && !self.value.is_empty();

        v_flex()
            .gap_2()
            .child(
                Label::new(self.label.clone())
                    .text_sm()
                    .font_weight(FontWeight::MEDIUM)
            )
            .child(
                h_flex()
                    .gap_2()
                    .child(
                        div()
                            .flex_1()
                            .px_3()
                            .py_2()
                            .rounded_md()
                            .bg(rgb(0xffffff))
                            .border_1()
                            .border_color(rgb(0xe5e7eb))
                            .when(self.is_editable, |this| {
                                this.hover(|style| style.border_color(rgb(0x9ca3af)))
                            })
                            .child(
                                Label::new(if self.is_password {
                                    "•".repeat(self.value.len().min(20)).into()
                                } else {
                                    self.value.clone()
                                })
                                .text_sm()
                                .text_color(if self.value.is_empty() {
                                    rgb(0x9ca3af)
                                } else {
                                    rgb(0x1f2937)
                                })
                            )
                    )
                    .when(show_copy, |this| {
                        this.child(
                            Button::new(format!("copy-{}", self.id.to_string()))
                                .ghost()
                                .icon(Icon::new(IconName::Copy))
                                .on_click(move |_event, _window, cx| {
                                    if let Some(ref handler) = self.on_copy {
                                        handler(_window, cx);
                                    }
                                    // Copy to clipboard
                                    cx.write_to_clipboard(gpui::ClipboardItem::new_string(value.to_string()));
                                })
                        )
                    })
            )
    }
}

/// Editable text field with actual input state
pub struct EditableTextField {
    id: ElementId,
    label: SharedString,
    state: Entity<InputState>,
    placeholder: SharedString,
    show_copy: bool,
    on_copy: Option<Rc<dyn Fn(&mut Window, &mut App) + 'static>>,
}

impl EditableTextField {
    pub fn new(
        id: impl Into<ElementId>,
        label: impl Into<SharedString>,
        state: &Entity<InputState>,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            state: state.clone(),
            placeholder: SharedString::default(),
            show_copy: true,
            on_copy: None,
        }
    }

    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn show_copy(mut self, show: bool) -> Self {
        self.show_copy = show;
        self
    }

    pub fn on_copy(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_copy = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for EditableTextField {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        v_flex()
            .gap_2()
            .child(
                Label::new(self.label.clone())
                    .text_sm()
                    .font_weight(FontWeight::MEDIUM)
            )
            .child(
                h_flex()
                    .gap_2()
                    .child(
                        Input::new(&self.state)
                            .cleanable(true)
                    )
                    .when(self.show_copy, |this| {
                        this.child(
                            Button::new(format!("copy-{}", self.id.to_string()))
                                .ghost()
                                .icon(Icon::new(IconName::Copy))
                                .on_click({
                                    let state = self.state.clone();
                                    move |_event, _window, cx| {
                                        if let Some(ref handler) = self.on_copy {
                                            handler(_window, cx);
                                        }
                                        let text = state.read(cx).text().to_string();
                                        cx.write_to_clipboard(gpui::ClipboardItem::new_string(text));
                                    }
                                })
                        )
                    })
            )
    }
}
