// Mission Control 子面板 - 完整的 Kanban 看板和任务详情面板

use crate::components::{
    render_ai_activity, render_ai_tool, render_kanban_column_header, render_task_card,
};
use crate::dashboard_panel::DashboardPanel;
use data::{TaskData, TaskStatus};
use gpui::*;
use gpui_component::{
    button::{Button, ButtonVariants as _},
    h_flex,
    label::Label,
    v_flex, IconName, Sizable,
};

/// Mission Control 主视图
pub fn render_mission_control(
    panel: &mut DashboardPanel,
    window: &mut Window,
    cx: &mut Context<DashboardPanel>,
) -> impl IntoElement {
    let selected_task_id = panel.selected_task_id;
    let header_padding = px(16.0);

    // 克隆任务列表以避免借用问题
    let todo_tasks = panel.todo_tasks.clone();
    let in_progress_tasks = panel.in_progress_tasks.clone();
    let done_tasks = panel.done_tasks.clone();

    h_flex()
        .flex_1()
        .pt(px(0.0))
        .px(px(24.0))
        .pb(px(24.0))
        .gap(px(16.0))
        .items_start()
        .justify_start()
        .child(
            // 看板三列 - 有详情面板时 flex_1，无详情面板时占满
            h_flex()
                .flex_1()
                .gap(px(16.0))
                .items_start()
                .justify_start()
                .h(px(0.0))
                .min_h_full()
                .child(render_kanban_column(
                    panel,
                    window,
                    cx,
                    "To Do",
                    &todo_tasks,
                    0,
                    header_padding,
                    TaskStatus::Todo,
                    move |this, _window, cx, _idx| {
                        let new_id = this.get_next_task_id(cx);
                        let new_task = TaskData::new(
                            new_id,
                            format!("New Task {}", new_id),
                            "TASK".to_string(),
                            "medium".to_string(),
                            TaskStatus::Todo,
                        );
                        this.add_task(new_task, cx);
                    },
                ))
                .child(render_kanban_column(
                    panel,
                    window,
                    cx,
                    "In Progress",
                    &in_progress_tasks,
                    1,
                    header_padding,
                    TaskStatus::InProgress,
                    move |this, _window, cx, _idx| {
                        let new_id = this.get_next_task_id(cx);
                        let new_task = TaskData::new(
                            new_id,
                            format!("New Task {}", new_id),
                            "TASK".to_string(),
                            "medium".to_string(),
                            TaskStatus::InProgress,
                        );
                        this.add_task(new_task, cx);
                    },
                ))
                .child(render_kanban_column(
                    panel,
                    window,
                    cx,
                    "Done",
                    &done_tasks,
                    2,
                    header_padding,
                    TaskStatus::Done,
                    move |this, _window, cx, _idx| {
                        let new_id = this.get_next_task_id(cx);
                        let new_task = TaskData::new(
                            new_id,
                            format!("New Task {}", new_id),
                            "TASK".to_string(),
                            "medium".to_string(),
                            TaskStatus::Done,
                        );
                        this.add_task(new_task, cx);
                    },
                )),
        )
        .child(
            // 右侧：详情面板（可选）
            if let Some(task_id) = selected_task_id {
                render_task_detail_panel(panel, window, cx, task_id).into_any_element()
            } else {
                div().into_any_element()
            },
        )
}

/// Kanban 列 - 包含标题栏和任务卡片列表
fn render_kanban_column(
    panel: &mut DashboardPanel,
    _window: &mut Window,
    cx: &mut Context<DashboardPanel>,
    title: &str,
    tasks: &[TaskData],
    column_index: usize,
    header_padding: Pixels,
    _default_status: TaskStatus,
    on_add: impl Fn(&mut DashboardPanel, &mut Window, &mut Context<DashboardPanel>, usize) + 'static,
) -> impl IntoElement {
    let selected_task_id = panel.selected_task_id;
    let task_count = tasks.len();

    v_flex()
        .flex_1()
        .h(px(0.0))
        .min_h_full()
        .bg(rgb(0xf9fafb))
        .rounded(px(8.0))
        .overflow_hidden()
        .items_start()
        .justify_start()
        .content_start()
        .child(
            // 标题栏 - 固定高度
            render_kanban_column_header(
                cx,
                title,
                task_count,
                column_index,
                header_padding,
                move |this, window, cx, idx| {
                    on_add(this, window, cx, idx);
                },
            ),
        )
        .child(
            // 任务卡片列表 - flex_1，支持滚动
            v_flex()
                .flex_1()
                .px(header_padding)
                .pt(px(12.0))
                .gap(px(8.0))
                .w_full()
                .h_full()
                .items_start()
                .justify_start()
                .content_start()
                .children(tasks.iter().map(|task| {
                    let task_id = task.id;
                    let is_selected = selected_task_id == Some(task_id);
                    render_task_card(
                        cx,
                        task,
                        is_selected,
                        move |this: &mut DashboardPanel, cx: &mut Context<DashboardPanel>, _| {
                            this.select_task(Some(task_id), cx);
                        },
                    )
                })),
        )
}

/// Task Detail Panel - 显示任务详情和 AI Agent 对话
fn render_task_detail_panel(
    panel: &DashboardPanel,
    _window: &mut Window,
    cx: &mut Context<DashboardPanel>,
    task_id: usize,
) -> impl IntoElement {
    // 根据 task_id 获取任务数据
    let task = panel
        .todo_tasks
        .iter()
        .chain(panel.in_progress_tasks.iter())
        .chain(panel.done_tasks.iter())
        .find(|t| t.id == task_id);

    // 任务详情数据（根据设计图）
    let (title, mission_objectives, ai_activities): (String, Vec<String>, Vec<(String, String, String)>) = match task_id {
        1 => (
            "Analyze Flight Logs".to_string(),
            vec![
                "Analyze the target drone communication interface for injection vulnerabilities.".to_string(),
                "Focus on legacy PHP endpoints.".to_string(),
            ],
            vec![
                ("HISTORY".to_string(), "14:30:05".to_string(), "Initial reconnaissance completed. Target appears to be running OpenResty + PHP 5.6.40. Several potentially vulnerable parameters identified.".to_string()),
                ("THOUGHT".to_string(), "14:31:12".to_string(), "Detected suspicious parameter `?ip=` in the URL. This pattern suggests a potential Command Injection vulnerability. The legacy PHP version (5.6.40) increases the likelihood of unpatched security flaws.".to_string()),
                ("PLAN".to_string(), "14:31:15".to_string(), "1. Verify connection to target.\n2. Fuzz the `ip` parameter with common injection payloads.\n3. Analyze response time and content for execution indicators.".to_string()),
            ],
        ),
        _ => (
            task.map(|t| t.title.clone()).unwrap_or_else(|| "Task".to_string()),
            vec!["No objectives defined.".to_string()],
            vec![],
        ),
    };

    v_flex()
        .w(px(400.0))
        .h_full()
        .bg(rgb(0xffffff))
        .border_l(px(1.0))
        .border_color(rgb(0xe5e7eb))
        .p(px(24.0))
        .gap(px(16.0))
        .child(
            // Panel Header
            h_flex()
                .items_center()
                .justify_between()
                .child(
                    h_flex()
                        .gap(px(8.0))
                        .items_center()
                        .child(
                            Label::new(title)
                                .text_base()
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(rgb(0x1f2937))
                        )
                        .child(
                            Label::new(format!("ID: {}", task_id))
                                .text_sm()
                                .text_color(rgb(0x6b7280))
                        )
                )
                .child(
                    h_flex()
                        .gap(px(8.0))
                        .child(
                            Button::new("view-task")
                                .ghost()
                                .icon(IconName::Eye)
                                .small()
                        )
                        .child(
                            Button::new("export-task")
                                .ghost()
                                .icon(IconName::ArrowDown)
                                .small()
                        )
                        .child(
                            Button::new("close-task")
                                .ghost()
                                .icon(IconName::Close)
                                .small()
                                .on_click(cx.listener(move |this, _, _, cx| {
                                    this.select_task(None, cx);
                                }))
                        )
                )
        )
        .child(
            // MISSION OBJECTIVE
            v_flex()
                .gap(px(8.0))
                .child(
                    Label::new("MISSION OBJECTIVE")
                        .text_sm()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(0x6b7280))
                )
                .children(mission_objectives.iter().map(|obj| {
                    let text = format!("> {}", obj);
                    Label::new(text.clone())
                        .text_sm()
                        .text_color(rgb(0x1f2937))
                }))
        )
        .child(
            // AI PENLIGENT AGENT
            v_flex()
                .gap(px(8.0))
                .child(
                    Label::new("AI PENLIGENT AGENT")
                        .text_sm()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(0x6b7280))
                )
                .children(ai_activities.iter().map(|(activity_type, timestamp, content)| {
                    render_ai_activity(
                        activity_type.as_str(),
                        timestamp.as_str(),
                        content.as_str(),
                    )
                }))
        )
        // 如果是 task_id == 1，添加 TOOL 和 ANALYSIS 示例
        .children(if task_id == 1 {
            vec![
                render_ai_tool(
                    "curl",
                    "14:32:00",
                    "$ curl -s -I -L --max-time 10 'http://target-drone-api:8080/?ip=127.0.0.1;id'",
                    "uid=33(www-data) gid=33(www-data) groups=33(www-data)",
                    "Success",
                ).into_any_element(),
                render_ai_activity(
                    "ANALYSIS",
                    "14:32:05",
                    "Command execution confirmed. The server returned the output of the `id` command. This is a CRITICAL vulnerability allowing remote code execution.",
                ).into_any_element(),
            ]
        } else {
            vec![]
        })
}
