//! Score Bar Component
//!
//! 分数进度条组件，用于 AI 安全分析

use gpui::prelude::FluentBuilder as _;
use gpui::*;
use gpui_component::{h_flex, v_flex};
use ui::theme::*;

/// 分数条组件
#[derive(IntoElement)]
pub struct ScoreBar {
    label: SharedString,
    percentage: f64,
    color: u32,
    show_value: bool,
}

impl ScoreBar {
    /// 创建新的分数条
    pub fn new(label: impl Into<SharedString>, percentage: f64) -> Self {
        Self {
            label: label.into(),
            percentage: percentage.clamp(0.0, 100.0),
            color: ACCENT_PURPLE,
            show_value: true,
        }
    }

    /// 设置颜色
    pub fn color(mut self, color: u32) -> Self {
        self.color = color;
        self
    }

    /// 设置是否显示数值
    pub fn show_value(mut self, show: bool) -> Self {
        self.show_value = show;
        self
    }
}

impl RenderOnce for ScoreBar {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let width_pct = self.percentage as f32;

        v_flex()
            .w_full()
            .gap(SPACING_XS)
            // 标签行
            .child(
                h_flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(TEXT_SECONDARY))
                            .child(self.label),
                    )
                    .when(self.show_value, |this| {
                        this.child(
                            div()
                                .text_sm()
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(rgb(self.color))
                                .child(format!("{:.0}%", self.percentage)),
                        )
                    }),
            )
            // 进度条
            .child(
                h_flex()
                    .w_full()
                    .h(px(6.0))
                    .rounded_full()
                    .bg(rgb(BG_SECONDARY))
                    .overflow_hidden()
                    .child(
                        div()
                            .h_full()
                            .w(px(width_pct * 3.0))
                            .bg(rgb(self.color))
                            .rounded_full(),
                    ),
            )
    }
}
