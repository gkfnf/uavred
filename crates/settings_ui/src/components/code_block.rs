//! Code Block Component for Settings - Alma Style
//!
//! A styled code block with copy functionality for environment variables.

use gpui::*;
use gpui::prelude::FluentBuilder;
use gpui_component::{
    v_flex, h_flex,
    button::{Button, ButtonVariants},
    label::Label,
    IconName, Icon, Sizable,
};

use std::rc::Rc;

/// Code block style variant
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum CodeBlockStyle {
    #[default]
    Dark,
    Green,
    Blue,
}

/// A styled code block with copy functionality - Alma style compact design
#[derive(IntoElement)]
pub struct CodeBlock {
    code: SharedString,
    style: CodeBlockStyle,
    show_copy: bool,
    on_copy: Option<Rc<dyn Fn(&mut Window, &mut App) + 'static>>,
}

impl CodeBlock {
    pub fn new(code: impl Into<SharedString>) -> Self {
        Self {
            code: code.into(),
            style: CodeBlockStyle::Dark,
            show_copy: true,
            on_copy: None,
        }
    }

    pub fn style(mut self, style: CodeBlockStyle) -> Self {
        self.style = style;
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

    fn bg_color(&self) -> u32 {
        match self.style {
            CodeBlockStyle::Dark => 0x1e293b,
            CodeBlockStyle::Green => 0x064e3b,
            CodeBlockStyle::Blue => 0x1e3a8a,
        }
    }

    fn text_color(&self) -> u32 {
        match self.style {
            CodeBlockStyle::Dark => 0xe2e8f0,
            CodeBlockStyle::Green => 0x6ee7b7,
            CodeBlockStyle::Blue => 0x93c5fd,
        }
    }
}

impl RenderOnce for CodeBlock {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let code = self.code.clone();
        let bg = self.bg_color();
        let text = self.text_color();

        v_flex()
            .relative()
            .rounded_md()
            .bg(rgb(bg))
            .p_3()
            .child(
                v_flex()
                    .gap_1()
                    .child(
                        Label::new(self.code.clone())
                            .text_xs()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgb(text))
                    )
            )
            .when(self.show_copy, |this| {
                this.child(
                    div()
                        .absolute()
                        .top(px(6.0))
                        .right(px(6.0))
                        .child(
                            Button::new("copy-code")
                                .ghost()
                                .small()
                                .icon(Icon::new(IconName::Copy))
                                .on_click(move |_event, _window, cx| {
                                    if let Some(ref handler) = self.on_copy {
                                        handler(_window, cx);
                                    }
                                    cx.write_to_clipboard(gpui::ClipboardItem::new_string(code.to_string()));
                                })
                        )
                )
            })
    }
}

/// Section containing title and code block
#[derive(IntoElement)]
pub struct CodeSection {
    title: SharedString,
    description: SharedString,
    code: SharedString,
    style: CodeBlockStyle,
}

impl CodeSection {
    pub fn new(title: impl Into<SharedString>, code: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            description: SharedString::default(),
            code: code.into(),
            style: CodeBlockStyle::Dark,
        }
    }

    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = description.into();
        self
    }

    pub fn style(mut self, style: CodeBlockStyle) -> Self {
        self.style = style;
        self
    }
}

impl RenderOnce for CodeSection {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        v_flex()
            .gap_3()
            .child(
                h_flex()
                    .gap_2()
                    .items_center()
                    .child(Icon::new(IconName::SquareTerminal).size(px(14.0)).text_color(rgb(0x6b7280)))
                    .child(
                        Label::new(self.title.clone())
                            .text_sm()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(0x374151))
                    )
            )
            .when(!self.description.is_empty(), |this| {
                this.child(
                    Label::new(self.description.clone())
                        .text_xs()
                        .text_color(rgb(0x6b7280))
                )
            })
            .child(
                CodeBlock::new(self.code.clone())
                    .style(self.style)
            )
    }
}
