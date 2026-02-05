//! TaskDetailPanel 组件 - 任务详情面板
//!
//! 显示任务的完整信息，包括标题、状态、优先级、类型等
//! 集成 AI Agent 执行面板

use crate::agent_execution::{AgentExecutionPanel, AgentExecutionSession, MissionObjective, create_demo_session};
use data::TaskData;
use gpui::prelude::FluentBuilder as _;
use gpui::*;
use gpui_component::{
    IconName, Sizable, StyledExt as _,
    button::{Button, ButtonVariants as _},
    h_flex,
    scroll::ScrollableElement as _,
    v_flex,
};
use ui::theme::*;

/// 关闭回调类型
type CloseHandler = Box<dyn Fn(&mut Window, &mut App) + 'static>;

/// 任务详情面板组件
pub struct TaskDetailPanel {
    task: Option<TaskData>,
    on_close: Option<CloseHandler>,
    /// Agent 执行面板
    agent_panel: Option<Entity<AgentExecutionPanel>>,
}

impl TaskDetailPanel {
    /// 创建新的详情面板
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            task: None,
            on_close: None,
            agent_panel: None,
        }
    }

    /// 初始化 Agent 面板（当面板被创建后调用）
    pub fn init_agent_panel(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.agent_panel.is_none() {
            let session = create_demo_session();
            let agent_panel = cx.new(|cx| AgentExecutionPanel::new(window, cx, session));
            self.agent_panel = Some(agent_panel);
        }
    }

    /// 设置要显示的任务
    pub fn set_task(&mut self, task: Option<TaskData>, window: &mut Window, cx: &mut Context<Self>) {
        self.task = task.clone();
        
        // 当有新任务时，创建新的 Agent 执行面板
        if let Some(ref task) = task {
            // 创建 Agent 执行会话
            let objective = if let Some(ref obj) = task.mission_objective {
                MissionObjective::new(&task.title, task.id as u64)
                    .with_description(obj)
            } else {
                MissionObjective::new(&task.title, task.id as u64)
                    .with_description("分析目标系统的安全漏洞")
            };
            
            let session = AgentExecutionSession::new("PENLIGENT AGENT", objective);
            let new_panel = cx.new(|cx| AgentExecutionPanel::new(window, cx, session));
            self.agent_panel = Some(new_panel);
        }
        
        cx.notify();
    }

    /// 设置关闭回调
    pub fn on_close(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_close = Some(Box::new(handler));
        self
    }

    /// 渲染信息行
    fn render_info_row(label: &str, value: impl IntoElement) -> impl IntoElement {
        h_flex()
            .items_start()
            .gap(SPACING_MD)
            .child(
                div()
                    .w(px(80.0))
                    .text_sm()
                    .text_color(rgb(TEXT_SECONDARY))
                    .child(label.to_string()),
            )
            .child(
                div()
                    .flex_1()
                    .text_sm()
                    .text_color(rgb(TEXT_PRIMARY))
                    .child(value),
            )
    }

    /// 获取优先级颜色
    fn priority_color(priority: &str) -> u32 {
        match priority {
            "critical" => SEVERITY_CRITICAL,
            "high" => SEVERITY_HIGH,
            "medium" => SEVERITY_MEDIUM,
            "low" => SEVERITY_LOW,
            _ => TEXT_SECONDARY,
        }
    }

    /// 获取状态颜色
    fn status_color(status: &str) -> u32 {
        match status {
            "todo" => TEXT_SECONDARY,
            "in_progress" => ACCENT_BLUE,
            "in_review" => SEVERITY_MEDIUM,
            "done" => STATUS_SUCCESS,
            "canceled" => SEVERITY_CRITICAL,
            _ => TEXT_SECONDARY,
        }
    }
}

impl Default for TaskDetailPanel {
    fn default() -> Self {
        // Default 实现用于类型占位，实际应使用 TaskDetailPanel::new
        Self {
            task: None,
            on_close: None,
            agent_panel: None,
        }
    }
}

impl Render for TaskDetailPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let task = self.task.clone();

        v_flex()
            .size_full()
            .bg(rgb(BG_CARD))
            .border_l_1()
            .border_color(rgb(BORDER_COLOR))
            // 头部
            .child(
                h_flex()
                    .items_center()
                    .justify_between()
                    .px(PADDING_LG)
                    .py(PADDING_MD)
                    .border_b_1()
                    .border_color(rgb(BORDER_COLOR))
                    .child(
                        div()
                            .text_base()
                            .font_semibold()
                            .text_color(rgb(TEXT_PRIMARY))
                            .child("Task Details"),
                    )
                    .child(
                        Button::new("close-detail")
                            .ghost()
                            .small()
                            .icon(IconName::Close)
                            .on_click(cx.listener(|this, _, window, cx| {
                                if let Some(ref on_close) = this.on_close {
                                    on_close(window, cx);
                                }
                            })),
                    ),
            )
            // 内容区
            .child(
                v_flex()
                    .flex_1()
                    .overflow_y_scrollbar()
                    .p(PADDING_LG)
                    .gap(SPACING_LG)
                    .when_some(task, |this, task| {
                        let priority_color = Self::priority_color(&task.priority);
                        let status_color = Self::status_color(&task.status.to_string());

                        this
                            // 任务标题
                            .child(
                                div()
                                    .text_lg()
                                    .font_semibold()
                                    .text_color(rgb(TEXT_PRIMARY))
                                    .child(task.title.clone()),
                            )
                            // 分隔线
                            .child(div().h(px(1.0)).bg(rgb(BORDER_COLOR)))
                            // 状态
                            .child(Self::render_info_row(
                                "Status",
                                div()
                                    .px(PADDING_SM)
                                    .py(px(2.0))
                                    .rounded(BORDER_RADIUS_SM)
                                    .bg(rgb(status_color))
                                    .text_color(rgb(0xffffff))
                                    .child(task.status.to_string()),
                            ))
                            // 优先级
                            .child(Self::render_info_row(
                                "Priority",
                                div()
                                    .px(PADDING_SM)
                                    .py(px(2.0))
                                    .rounded(BORDER_RADIUS_SM)
                                    .bg(rgb(priority_color))
                                    .text_color(rgb(0xffffff))
                                    .child(task.priority.clone()),
                            ))
                            // 类型
                            .child(Self::render_info_row("Type", task.task_type.clone()))
                            // ID
                            .child(Self::render_info_row("ID", format!("#{}", task.id)))
                    })
                    .when(self.task.is_none(), |this| {
                        this.child(
                            div()
                                .flex_1()
                                .items_center()
                                .justify_center()
                                .text_color(rgb(TEXT_MUTED))
                                .child("Select a task to view details"),
                        )
                    }),
            )
            // Agent 执行面板（当有任务时显示）
            .when_some(self.task.clone(), |this, _task| {
                this.when_some(self.agent_panel.clone(), |this, panel| {
                    this.child(
                        div()
                            .flex_1()
                            .h(px(400.0))
                            .child(panel)
                    )
                })
            })
    }
}
