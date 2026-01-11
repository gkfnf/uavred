// T1-15: Flows 工作流视图 - 底部操作栏
// 参考设计: WorkFlows.png 底部

use gpui::*;
use gpui_component::{
    button::{Button, ButtonVariants as _},
    h_flex,
    label::Label,
    tag::Tag,
    v_flex, IconName, Sizable,
};
use ui::theme::*;

/// 工作流统计信息
#[derive(Debug, Clone, Default)]
pub struct FlowStats {
    pub atomic_count: usize,
    pub composite_count: usize,
    pub task_count: usize,
}

/// 底部操作栏组件
pub struct ActionBar {
    pub stats: FlowStats,
    pub has_selected_flow: bool,
}

impl ActionBar {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            stats: FlowStats::default(),
            has_selected_flow: false,
        }
    }

    pub fn set_stats(&mut self, stats: FlowStats, cx: &mut Context<Self>) {
        self.stats = stats;
        cx.notify();
    }

    pub fn set_has_selected_flow(&mut self, has_selected: bool, cx: &mut Context<Self>) {
        self.has_selected_flow = has_selected;
        cx.notify();
    }
}

impl Render for ActionBar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .w_full()
            .h(px(56.0))
            .px(PADDING_LG)
            .py(PADDING_MD)
            .items_center()
            .justify_between()
            .border_t(px(1.0))
            .border_color(rgb(BORDER_COLOR))
            .bg(rgb(BG_CARD))
            .child(self.render_actions(cx))
            .child(self.render_stats(cx))
    }
}

impl ActionBar {
    fn render_actions(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .gap(PADDING_SM)
            .items_center()
            .child(
                Button::new("run-flow-btn")
                    .primary()
                    .medium()
                    .icon(IconName::Play)
                    .label("运行工作流")
                    .disabled(!self.has_selected_flow)
                    .on_click(cx.listener(|this: &mut Self, _, _, cx| {
                        // TODO: 实现运行工作流逻辑
                        cx.notify();
                    })),
            )
            .child(
                Button::new("simulate-flow-btn")
                    .outline()
                    .medium()
                    .icon(IconName::TestTube)
                    .label("模拟测试")
                    .disabled(!self.has_selected_flow)
                    .on_click(cx.listener(|this: &mut Self, _, _, cx| {
                        // TODO: 实现模拟测试逻辑
                        cx.notify();
                    })),
            )
            .child(
                Button::new("delete-flow-btn")
                    .ghost()
                    .medium()
                    .icon(IconName::Trash)
                    .label("删除")
                    .disabled(!self.has_selected_flow)
                    .on_click(cx.listener(|this: &mut Self, _, _, cx| {
                        // TODO: 实现删除工作流逻辑
                        cx.notify();
                    })),
            )
    }

    fn render_stats(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .gap(PADDING_LG)
            .items_center()
            .child(self.render_stat_badge("原子", self.stats.atomic_count, rgb(0x3b82f6)))
            .child(self.render_stat_badge("组合", self.stats.composite_count, rgb(0x10b981)))
            .child(self.render_stat_badge("任务", self.stats.task_count, rgb(0x7c3aed)))
    }

    fn render_stat_badge(&self, label: &str, count: usize, color: Rgb) -> impl IntoElement {
        h_flex()
            .gap(PADDING_XS)
            .items_center()
            .child(
                Label::new(label)
                    .text_sm()
                    .text_color(rgb(TEXT_SECONDARY)),
            )
            .child(
                Tag::new()
                    .small()
                    .bg(color)
                    .text_color(rgb(0xffffff))
                    .child(Label::new(format!("{}", count)).text_xs()),
            )
    }
}
