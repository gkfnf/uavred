//! Technique Tag Component
//!
//! MITRE ATT&CK 技术标签组件

use gpui::*;
use ui::theme::*;

/// 技术标签样式
#[derive(Clone, Copy, Debug, Default)]
pub enum TechniqueTagStyle {
    #[default]
    Default,
    Primary,
    Success,
    Warning,
    Danger,
}

/// MITRE ATT&CK 技术标签组件
#[derive(IntoElement)]
pub struct TechniqueTag {
    label: SharedString,
    style: TechniqueTagStyle,
    custom_bg: Option<u32>,
    custom_text: Option<u32>,
    custom_border: Option<u32>,
}

impl TechniqueTag {
    /// 创建新的技术标签
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            style: TechniqueTagStyle::Default,
            custom_bg: None,
            custom_text: None,
            custom_border: None,
        }
    }

    /// 设置样式
    pub fn with_style(mut self, style: TechniqueTagStyle) -> Self {
        self.style = style;
        self
    }

    /// 设置自定义颜色
    pub fn custom_colors(
        mut self,
        bg: u32,
        text: u32,
        border: u32,
    ) -> Self {
        self.custom_bg = Some(bg);
        self.custom_text = Some(text);
        self.custom_border = Some(border);
        self
    }

    fn get_colors(&self) -> (u32, u32, u32) {
        if let (Some(bg), Some(text), Some(border)) =
            (self.custom_bg, self.custom_text, self.custom_border)
        {
            return (bg, text, border);
        }

        match self.style {
            TechniqueTagStyle::Default => (0xfff7ed, 0xc2410c, 0xfdba74),
            TechniqueTagStyle::Primary => (0xeff6ff, 0x1e40af, 0xbfdbfe),
            TechniqueTagStyle::Success => (0xd1fae5, 0x065f46, 0x6ee7b7),
            TechniqueTagStyle::Warning => (0xfef3c7, 0x92400e, 0xfcd34d),
            TechniqueTagStyle::Danger => (0xfee2e2, 0x991b1b, 0xfca5a5),
        }
    }
}

impl RenderOnce for TechniqueTag {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let (bg, text, border) = self.get_colors();

        div()
            .px(PADDING_MD)
            .py(px(6.0))
            .rounded(BORDER_RADIUS_SM)
            .bg(rgb(bg))
            .border_1()
            .border_color(rgb(border))
            .text_sm()
            .font_weight(FontWeight::SEMIBOLD)
            .text_color(rgb(text))
            .child(self.label)
    }
}

/// 创建一组技术标签
#[derive(IntoElement)]
pub struct TechniqueTagGroup {
    tags: Vec<SharedString>,
    style: TechniqueTagStyle,
}

impl TechniqueTagGroup {
    pub fn new() -> Self {
        Self {
            tags: Vec::new(),
            style: TechniqueTagStyle::Default,
        }
    }

    pub fn tag(mut self, tag: impl Into<SharedString>) -> Self {
        self.tags.push(tag.into());
        self
    }

    pub fn tags(mut self, tags: impl Iterator<Item = impl Into<SharedString>>) -> Self {
        self.tags.extend(tags.map(|t| t.into()));
        self
    }

    pub fn with_style(mut self, style: TechniqueTagStyle) -> Self {
        self.style = style;
        self
    }
}

impl RenderOnce for TechniqueTagGroup {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        use gpui_component::h_flex;

        h_flex()
            .flex_wrap()
            .gap(SPACING_SM)
            .children(self.tags.into_iter().map(|tag| {
                TechniqueTag::new(tag).with_style(self.style)
            }))
    }
}
