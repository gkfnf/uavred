// Dashboard 通用组件

use gpui::*;
use gpui_component::{
    button::{Button, ButtonVariants as _},
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
    
    // 根据列的状态选择指示器颜色
    let indicator_color = match column_index {
        0 => rgb(0x374151), // To Do: Dark Grey
        1 => rgb(0x3b82f6), // In Progress: Blue
        2 => rgb(0xf97316), // In Review: Orange
        3 => rgb(0x10b981), // Done: Green
        4 => rgb(0xef4444), // Cancelled: Red
        _ => rgb(0x6b7280), // Default: Grey
    };

    h_flex()
        .w_full()
        .h(px(32.0))
        .flex_none()
        .px(header_padding)
        .py(px(4.0))
        .items_center()
        .justify_between()
        .bg(rgb(0xf9fafb))
        .rounded(px(6.0))
        .border_b(px(1.0))
        .border_color(rgb(0xe5e7eb))
        .child(
            h_flex()
                .gap(px(6.0))
                .items_center()
                .child(div().w(px(6.0)).h(px(6.0)).rounded_full().bg(indicator_color))
                .child(
                    Label::new(title_str)
                        .text_sm()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(0x1f2937)),
                )
                .child(
                    Label::new(format!("({})", count))
                        .text_xs()
                        .text_color(rgb(0x6b7280)),
                ),
        )
        .child(
            Button::new(format!("add-task-{}", column_index))
                .ghost()
                .icon(IconName::Plus)
                .xsmall()
                .on_click(cx.listener(move |this, _, window, cx| {
                    on_add(this, window, cx, column_index);
                })),
        )
}

/// 任务卡片 - 简洁紧凑的设计，符合 Kanban 风格
pub fn render_task_card<T: 'static>(
    cx: &mut Context<T>,
    task: &TaskData,
    is_selected: bool,
    on_select: impl Fn(&mut T, &mut Context<T>, usize) + 'static,
) -> impl IntoElement {
    let task_id = task.id;

    let mut card = div()
        .bg(rgb(0xffffff))
        .rounded(px(6.0))
        .border(px(1.0))
        .border_color(rgb(0xe5e7eb))
        .p(px(10.0))
        .gap(px(6.0))
        .flex_col()
        .w_full()
        .mb(px(8.0))
        .child(
            // 卡片头部：标题 + 菜单
            h_flex()
                .justify_between()
                .items_center()
                .w_full()
                .child(
                    Label::new(&task.title)
                        .text_sm()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(0x1f2937)),
                )
                .child(
                    Button::new(format!("task-menu-{}", task_id))
                        .ghost()
                        .icon(IconName::Ellipsis)
                        .xsmall()
                )
        );
    
    // 如果有任务类型，显示它
    if !task.task_type.is_empty() && task.task_type != "TASK" {
        card = card.child(
            Label::new(&task.task_type)
                .text_xs()
                .text_color(rgb(0x6b7280))
        );
    }
    
    // 如果选中，更新边框
    if is_selected {
        card = card.border_color(rgb(0x3b82f6)).border(px(2.0));
    }

    // 卡片容器 - 可点击选择
    div()
        .id(("task-card", task_id))
        .w_full()
        .cursor_pointer()
        .child(card)
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
