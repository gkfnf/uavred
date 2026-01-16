// 添加任务对话框组件 - 显示表单 UI
use gpui::*;
use gpui_component::{
    button::{Button, ButtonVariants as _},
    h_flex, v_flex,
    label::Label,
    IconName, Sizable,
};

pub fn render_add_task_dialog(
    title: &str,
    description: &str,
    auto_start: bool,
) -> impl IntoElement {
    let title_display = if title.is_empty() { "输入任务标题...".to_string() } else { title.to_string() };
    let desc_display = if description.is_empty() { "输入任务描述...".to_string() } else { description.to_string() };
    
    v_flex()
        .gap(px(16.0))
        .p(px(24.0))
        .w(px(600.0))
        .bg(rgb(0xffffff))
        .rounded(px(12.0))
        // 头部：标题 + 关闭按钮
        .child(
            h_flex()
                .justify_between()
                .items_center()
                .w_full()
                .child(
                    Label::new("创建新任务")
                        .text_lg()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(0x1f2937))
                )
                .child(
                    Button::new("close-dialog")
                        .ghost()
                        .icon(IconName::Close)
                        .small()
                )
        )
        // 分割线
        .child(
            div()
                .w_full()
                .h(px(1.0))
                .bg(rgb(0xe5e7eb))
        )
        // 任务标题输入框
        .child(
            v_flex()
                .gap(px(8.0))
                .w_full()
                .child(
                    h_flex()
                        .gap(px(4.0))
                        .child(
                            Label::new("任务标题")
                                .text_sm()
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(rgb(0x1f2937))
                        )
                        .child(
                            Label::new("*")
                                .text_sm()
                                .text_color(rgb(0xef4444))
                        )
                )
                .child(
                    div()
                        .w_full()
                        .h(px(40.0))
                        .border(px(1.0))
                        .border_color(rgb(0xd1d5db))
                        .rounded(px(6.0))
                        .px(px(12.0))
                        .py(px(8.0))
                        .bg(rgb(0xfafafa))
                        .child(
                            Label::new(title_display.clone())
                                .text_sm()
                                .text_color(if title.is_empty() { rgb(0x9ca3af) } else { rgb(0x1f2937) })
                        )
                )
        )
        // 任务描述输入框
        .child(
            v_flex()
                .gap(px(8.0))
                .w_full()
                .child(
                    Label::new("任务描述（可选）")
                        .text_sm()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(0x1f2937))
                )
                .child(
                    div()
                        .w_full()
                        .h(px(100.0))
                        .border(px(1.0))
                        .border_color(rgb(0xd1d5db))
                        .rounded(px(6.0))
                        .p(px(12.0))
                        .bg(rgb(0xfafafa))
                        .child(
                            Label::new(desc_display.clone())
                                .text_sm()
                                .text_color(if description.is_empty() { rgb(0x9ca3af) } else { rgb(0x1f2937) })
                        )
                )
        )
        // 三个下拉菜单占位符
        .child(
            v_flex()
                .gap(px(8.0))
                .w_full()
                .child(
                    Label::new("配置")
                        .text_sm()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(0x1f2937))
                )
                .child(
                    h_flex()
                        .gap(px(12.0))
                        .w_full()
                        .child(
                            div()
                                .flex_1()
                                .border(px(1.0))
                                .border_color(rgb(0xd1d5db))
                                .rounded(px(6.0))
                                .p(px(12.0))
                                .bg(rgb(0xfafafa))
                                .flex()
                                .items_center()
                                .justify_between()
                                .child(
                                    Label::new("Agent")
                                        .text_sm()
                                        .text_color(rgb(0x1f2937))
                                )
                                .child(
                                    Label::new("OPENCODE")
                                        .text_xs()
                                        .text_color(rgb(0x6b7280))
                                )
                        )
                        .child(
                            div()
                                .flex_1()
                                .border(px(1.0))
                                .border_color(rgb(0xd1d5db))
                                .rounded(px(6.0))
                                .p(px(12.0))
                                .bg(rgb(0xfafafa))
                                .flex()
                                .items_center()
                                .justify_between()
                                .child(
                                    Label::new("优先级")
                                        .text_sm()
                                        .text_color(rgb(0x1f2937))
                                )
                                .child(
                                    Label::new("Medium")
                                        .text_xs()
                                        .text_color(rgb(0x6b7280))
                                )
                        )
                        .child(
                            div()
                                .flex_1()
                                .border(px(1.0))
                                .border_color(rgb(0xd1d5db))
                                .rounded(px(6.0))
                                .p(px(12.0))
                                .bg(rgb(0xfafafa))
                                .flex()
                                .items_center()
                                .justify_between()
                                .child(
                                    Label::new("分支")
                                        .text_sm()
                                        .text_color(rgb(0x1f2937))
                                )
                                .child(
                                    Label::new("master")
                                        .text_xs()
                                        .text_color(rgb(0x6b7280))
                                )
                        )
                )
        )
        // 分割线
        .child(
            div()
                .w_full()
                .h(px(1.0))
                .bg(rgb(0xe5e7eb))
        )
        // 底部：开始开关 + 创建/取消按钮
        .child(
            h_flex()
                .justify_between()
                .items_center()
                .w_full()
                .child(
                    h_flex()
                        .gap(px(12.0))
                        .items_center()
                        // 开始开关（toggle）
                        .child(
                            div()
                                .w(px(44.0))
                                .h(px(24.0))
                                .rounded_full()
                                .bg(if auto_start { rgb(0x3b82f6) } else { rgb(0xd1d5db) })
                                .cursor_pointer()
                                .relative()
                                .child(
                                    div()
                                        .w(px(20.0))
                                        .h(px(20.0))
                                        .rounded_full()
                                        .bg(rgb(0xffffff))
                                        .absolute()
                                        .top(px(2.0))
                                        .left(if auto_start { px(22.0) } else { px(2.0) })
                                )
                        )
                        .child(
                            Label::new("立即开始")
                                .text_sm()
                                .text_color(rgb(0x1f2937))
                        )
                )
                .child(
                    h_flex()
                        .gap(px(12.0))
                        .child(
                            Button::new("cancel-task")
                                .ghost()
                                .label("取消")
                        )
                        .child(
                            Button::new("create-task-confirm")
                                .primary()
                                .label("创建")
                        )
                )
        )
}
