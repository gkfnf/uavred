// 添加任务对话框组件
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
    let title_str = title.to_string();
    let description_str = description.to_string();

    v_flex()
        .gap(px(16.0))
        .p(px(24.0))
        .w(px(600.0))
        .bg(rgb(0xffffff))
        .rounded(px(12.0))
        // 头部
        .child(
            h_flex()
                .justify_between()
                .items_center()
                .child(
                    Label::new("任务标题")
                        .text_lg()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(0x1f2937))
                )
                .child(
                    Label::new("")
                        .text_xs()
                )
        )
        // 标题输入框
        .child(
            v_flex()
                .gap(px(8.0))
                .child(
                    Label::new("标题（必填）")
                        .text_sm()
                        .text_color(rgb(0x6b7280))
                )
                .child(
                    div()
                        .w_full()
                        .h(px(36.0))
                        .border(px(1.0))
                        .border_color(rgb(0xd1d5db))
                        .rounded(px(6.0))
                        .px(px(12.0))
                        .flex()
                        .items_center()
                        .bg(rgb(0xfafafa))
                        .child(
                            Label::new(title_str.clone())
                                .text_sm()
                                .text_color(if title.is_empty() { rgb(0x9ca3af) } else { rgb(0x1f2937) })
                        )
                )
        )
        // 描述输入框
        .child(
            v_flex()
                .gap(px(8.0))
                .child(
                    Label::new("描述（可选）")
                        .text_sm()
                        .text_color(rgb(0x6b7280))
                )
                .child(
                    div()
                        .w_full()
                        .h(px(120.0))
                        .border(px(1.0))
                        .border_color(rgb(0xd1d5db))
                        .rounded(px(6.0))
                        .p(px(12.0))
                        .bg(rgb(0xfafafa))
                        .child(
                            Label::new(description_str.clone())
                                .text_sm()
                                .text_color(if description.is_empty() { rgb(0x9ca3af) } else { rgb(0x1f2937) })
                        )
                )
        )
        // 三个下拉菜单占位
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
                        .child(Label::new("OPENCODE").text_sm().text_color(rgb(0x6b7280)))
                )
                .child(
                    div()
                        .flex_1()
                        .border(px(1.0))
                        .border_color(rgb(0xd1d5db))
                        .rounded(px(6.0))
                        .p(px(12.0))
                        .bg(rgb(0xfafafa))
                        .child(Label::new("DEFAULT").text_sm().text_color(rgb(0x6b7280)))
                )
                .child(
                    div()
                        .flex_1()
                        .border(px(1.0))
                        .border_color(rgb(0xd1d5db))
                        .rounded(px(6.0))
                        .p(px(12.0))
                        .bg(rgb(0xfafafa))
                        .child(Label::new("master").text_sm().text_color(rgb(0x6b7280)))
                )
        )
        // 底部：图片按钮、开始开关、创建按钮
        .child(
            h_flex()
                .justify_between()
                .items_center()
                .w_full()
                .child(
                    Button::new("upload-image")
                        .ghost()
                        .icon(IconName::File)
                        .small()
                )
                .child(
                    h_flex()
                        .gap(px(12.0))
                        .items_center()
                        // 开始开关
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
                            Label::new("开始")
                                .text_sm()
                                .text_color(rgb(0x1f2937))
                        )
                        .child(
                            Button::new("create-task")
                                .primary()
                                .label("创建")
                        )
                )
        )
}
