//! Info Card Component
//!
//! 信息卡片组件，用于显示 CVSS 评分、检测时间等信息

use gpui::prelude::FluentBuilder as _;
use gpui::*;
use gpui_component::{h_flex, v_flex};
use ui::theme::*;

/// 信息卡片变体
#[derive(Clone, Copy, Debug, Default)]
pub enum InfoCardVariant {
    #[default]
    Default,
    Primary,
    Success,
    Warning,
    Danger,
}

/// 信息卡片组件
#[derive(IntoElement)]
pub struct InfoCard {
    label: SharedString,
    value: SharedString,
    variant: InfoCardVariant,
    custom_value_color: Option<u32>,
    subtitle: Option<SharedString>,
    icon: Option<SharedString>,
}

impl InfoCard {
    /// 创建新的信息卡片
    pub fn new(
        label: impl Into<SharedString>,
        value: impl Into<SharedString>,
    ) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            variant: InfoCardVariant::Default,
            custom_value_color: None,
            subtitle: None,
            icon: None,
        }
    }

    /// 设置变体
    pub fn variant(mut self, variant: InfoCardVariant) -> Self {
        self.variant = variant;
        self
    }

    /// 设置自定义值颜色
    pub fn value_color(mut self, color: u32) -> Self {
        self.custom_value_color = Some(color);
        self
    }

    /// 设置副标题
    pub fn subtitle(mut self, subtitle: impl Into<SharedString>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    /// 设置图标
    pub fn icon(mut self, icon: impl Into<SharedString>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    fn get_value_color(&self) -> u32 {
        if let Some(color) = self.custom_value_color {
            return color;
        }
        match self.variant {
            InfoCardVariant::Default => TEXT_PRIMARY,
            InfoCardVariant::Primary => ACCENT_BLUE,
            InfoCardVariant::Success => SEVERITY_LOW,
            InfoCardVariant::Warning => SEVERITY_MEDIUM,
            InfoCardVariant::Danger => SEVERITY_CRITICAL,
        }
    }
}

impl RenderOnce for InfoCard {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let value_color = self.get_value_color();
        
        v_flex()
            .p(PADDING_LG)
            .gap(SPACING_SM)
            .rounded(BORDER_RADIUS)
            .bg(rgb(BG_SECONDARY))
            .border_1()
            .border_color(rgb(BORDER_COLOR))
            // 标签行（可能包含图标）
            .child(
                h_flex()
                    .items_center()
                    .gap(SPACING_XS)
                    .when_some(self.icon, |this, icon| {
                        this.child(div().text_sm().text_color(rgb(TEXT_SECONDARY)).child(icon))
                    })
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(TEXT_SECONDARY))
                            .child(self.label),
                    ),
            )
            // 值
            .child(
                div()
                    .text_2xl()
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(value_color))
                    .child(self.value),
            )
            // 副标题
            .when_some(self.subtitle, |this, subtitle| {
                this.child(
                    div()
                        .text_xs()
                        .text_color(rgb(TEXT_MUTED))
                        .child(subtitle),
                )
            })
    }
}

/// 带标签的信息行组件
#[derive(IntoElement)]
pub struct InfoRow {
    label: SharedString,
    value: AnyElement,
    label_width: Pixels,
}

impl InfoRow {
    /// 创建新的信息行
    pub fn new(label: impl Into<SharedString>, value: impl IntoElement) -> Self {
        Self {
            label: label.into(),
            value: value.into_any_element(),
            label_width: px(100.0),
        }
    }

    /// 设置标签宽度
    pub fn label_width(mut self, width: Pixels) -> Self {
        self.label_width = width;
        self
    }
}

impl RenderOnce for InfoRow {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        h_flex()
            .items_start()
            .gap(SPACING_MD)
            .child(
                div()
                    .w(self.label_width)
                    .text_sm()
                    .text_color(rgb(TEXT_SECONDARY))
                    .child(self.label),
            )
            .child(
                div()
                    .flex_1()
                    .text_sm()
                    .text_color(rgb(TEXT_PRIMARY))
                    .child(self.value),
            )
    }
}
