// Dashboard 通用组件

use gpui::*;
use gpui_component::{
    button::{Button, ButtonVariants as _},
    group_box::{GroupBox, GroupBoxVariants as _},
    h_flex,
    label::Label,
    tag::Tag,
    v_flex, IconName, Sizable,
};

use data::TaskData;

/// Kanban 列标题
pub fn render_kanban_column_header<T: 'static>(
    cx: &mut Context<T>,
    title: &str,
    count: usize,
    column_index: usize,
    header_padding: Pixels,
    on_add: impl Fn(&mut T, &mut Window, &mut Context<T>, usize) + 'static,
) -> impl IntoElement {
    let title_str = title.to_string();

    h_flex()
        .flex_1()
        .h(px(48.0))
        .px(header_padding)
        .items_center()
        .justify_between()
        .bg(rgb(0xf9fafb))
        .rounded(px(8.0))
        .items_start()
        .child(
            h_flex()
                .gap(px(8.0))
                .items_center()
                .child(div().w(px(8.0)).h(px(8.0)).rounded_full().bg(rgb(0x6b7280)))
                .child(
                    Label::new(title_str)
                        .text_base()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(0x1f2937)),
                )
                .child(
                    Label::new(format!("({})", count))
                        .text_sm()
                        .text_color(rgb(0x6b7280)),
                ),
        )
        .child(
            Button::new(format!("add-task-{}", column_index))
                .ghost()
                .icon(IconName::Plus)
                .small()
                .on_click(cx.listener(move |this, _, window, cx| {
                    on_add(this, window, cx, column_index);
                })),
        )
}

/// 任务卡片
pub fn render_task_card<T: 'static>(
    cx: &mut Context<T>,
    task: &TaskData,
    is_selected: bool,
    on_select: impl Fn(&mut T, &mut Context<T>, usize) + 'static,
) -> impl IntoElement {
    let task_id = task.id;

    // 类型标签颜色（蓝色背景）
    let type_bg = rgb(0x3b82f6);
    let type_text = rgb(0xffffff);

    // 优先级标签颜色
    let (priority_bg, priority_text) = match task.priority.as_str() {
        "critical" => (rgb(0xef4444), rgb(0xffffff)),
        "high" => (rgb(0xef4444), rgb(0xffffff)),
        "medium" => (rgb(0xf97316), rgb(0xffffff)),
        _ => (rgb(0x10b981), rgb(0xffffff)),
    };

    let mut card_content = GroupBox::new().outline().child(
        v_flex()
            .gap(px(8.0))
            .p(px(12.0))
            .child(
                Label::new(&task.title)
                    .text_sm()
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(rgb(0x1f2937)),
            )
            .child(
                h_flex()
                    .gap(px(4.0))
                    .items_center()
                    .child(
                        Tag::new()
                            .small()
                            .bg(type_bg)
                            .text_color(type_text)
                            .child(Label::new(&task.task_type).text_xs()),
                    )
                    .child(
                        Tag::new()
                            .small()
                            .bg(priority_bg)
                            .text_color(priority_text)
                            .child(Label::new(&task.priority).text_xs()),
                    ),
            ),
    );

    // 选中状态：紫色边框
    if is_selected {
        card_content = card_content.border(px(2.0)).border_color(rgb(0x7c3aed));
    }

    // 使用稳定的 ElementId（元组格式，参考 zed 的模式）
    div()
        .id(("task-card", task_id))
        .w_full()
        .cursor_pointer()
        .child(card_content)
        .on_mouse_down(
            MouseButton::Left,
            cx.listener(move |this: &mut T, _, _, cx: &mut Context<T>| {
                on_select(this, cx, task_id);
            }),
        )
}

/// AI Activity 条目
pub fn render_ai_activity(activity_type: &str, timestamp: &str, content: &str) -> impl IntoElement {
    let activity_type_str = activity_type.to_string();
    let timestamp_str = timestamp.to_string();
    let content_str = content.to_string();

    let (bg_color, text_color) = match activity_type {
        "HISTORY" => (rgb(0xf3f4f6), rgb(0x6b7280)),
        "THOUGHT" => (rgb(0xf3e8ff), rgb(0x7c3aed)),
        "PLAN" => (rgb(0xfffbeb), rgb(0xf59e0b)),
        "TOOL" => (rgb(0xeff6ff), rgb(0x2563eb)),
        "ANALYSIS" => (rgb(0xf0fdf4), rgb(0x16a34a)),
        _ => (rgb(0xf3f4f6), rgb(0x6b7280)),
    };

    v_flex()
        .gap(px(4.0))
        .p(px(12.0))
        .bg(bg_color)
        .rounded(px(6.0))
        .child(
            h_flex()
                .gap(px(8.0))
                .items_center()
                .child(
                    Label::new(activity_type_str)
                        .text_xs()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(text_color),
                )
                .child(Label::new(timestamp_str).text_xs().text_color(text_color)),
        )
        .child(Label::new(content_str).text_sm().text_color(rgb(0x1f2937)))
}

/// AI Tool 执行条目（特殊格式，显示命令和输出）
pub fn render_ai_tool(
    tool_name: &str,
    timestamp: &str,
    command: &str,
    output: &str,
    status: &str,
) -> impl IntoElement {
    let tool_name_str = tool_name.to_string();
    let timestamp_str = timestamp.to_string();
    let command_str = command.to_string();
    let output_str = output.to_string();
    let status_str = status.to_string();

    let status_bg = if status == "Success" {
        rgb(0xdcfce7)
    } else {
        rgb(0xfee2e2)
    };
    let status_text = if status == "Success" {
        rgb(0x166534)
    } else {
        rgb(0x991b1b)
    };

    v_flex()
        .gap(px(8.0))
        .p(px(12.0))
        .bg(rgb(0xeff6ff))
        .rounded(px(6.0))
        .child(
            h_flex()
                .gap(px(8.0))
                .items_center()
                .child(
                    Label::new("TOOL")
                        .text_xs()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(0x2563eb)),
                )
                .child(
                    Label::new(timestamp_str)
                        .text_xs()
                        .text_color(rgb(0x2563eb)),
                ),
        )
        .child(
            h_flex()
                .gap(px(8.0))
                .items_center()
                .child(
                    Label::new(format!("Tool: {}", tool_name_str))
                        .text_sm()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(0x1f2937)),
                )
                .child(
                    Tag::new()
                        .small()
                        .bg(status_bg)
                        .text_color(status_text)
                        .child(Label::new(status_str).text_xs()),
                ),
        )
        .child(
            v_flex()
                .gap(px(4.0))
                .child(
                    Label::new("Command:")
                        .text_xs()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(0x6b7280)),
                )
                .child(
                    div()
                        .px(px(8.0))
                        .py(px(4.0))
                        .bg(rgb(0x1f2937))
                        .rounded(px(4.0))
                        .child(
                            Label::new(command_str)
                                .text_xs()
                                .font_family("monospace")
                                .text_color(rgb(0xffffff)),
                        ),
                ),
        )
        .child(
            v_flex()
                .gap(px(4.0))
                .child(
                    Label::new("Output:")
                        .text_xs()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(0x6b7280)),
                )
                .child(
                    div()
                        .px(px(8.0))
                        .py(px(4.0))
                        .bg(rgb(0xf9fafb))
                        .rounded(px(4.0))
                        .border(px(1.0))
                        .border_color(rgb(0xe5e7eb))
                        .child(
                            Label::new(output_str)
                                .text_xs()
                                .font_family("monospace")
                                .text_color(rgb(0x1f2937)),
                        ),
                ),
        )
}
