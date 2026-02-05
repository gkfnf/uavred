//! KanbanBoard 组件 - 主容器
//!
//! 5 列看板加 squeeze-style 详情面板的主容器

use crate::kanban_column::KanbanColumn;
use crate::agent_execution::{AgentExecutionPanel, AgentExecutionSession, MissionObjective, create_demo_session};
use data::{TaskData, TaskStatus};
use gpui::prelude::FluentBuilder as _;
use gpui::*;
use gpui_component::{h_flex, v_flex};
use ui::theme::*;

/// 看板事件
#[derive(Debug, Clone)]
pub enum KanbanEvent {
    TaskSelected(Option<usize>),
    TaskMoved {
        task_id: usize,
        from: TaskStatus,
        to: TaskStatus,
    },
    DetailPanelToggled(bool),
}

impl EventEmitter<KanbanEvent> for KanbanBoard {}

/// 主看板组件
pub struct KanbanBoard {
    tasks: Vec<TaskData>,
    selected_task_id: Option<usize>,
    detail_panel_visible: bool,
    /// Agent 执行面板
    agent_panel: Option<Entity<AgentExecutionPanel>>,
}

impl KanbanBoard {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            tasks: Vec::new(),
            selected_task_id: None,
            detail_panel_visible: false,
            agent_panel: None,
        }
    }

    pub fn tasks(mut self, tasks: Vec<TaskData>) -> Self {
        self.tasks = tasks;
        self
    }

    pub fn selected_task_id(mut self, id: Option<usize>) -> Self {
        self.selected_task_id = id;
        self
    }

    pub fn detail_panel_visible(mut self, visible: bool) -> Self {
        self.detail_panel_visible = visible;
        self
    }

    fn tasks_by_status(&self, status: TaskStatus) -> Vec<TaskData> {
        let status_str = status.as_str();
        self.tasks
            .iter()
            .filter(|t| t.status == status_str)
            .cloned()
            .collect()
    }

    fn get_selected_task(&self) -> Option<TaskData> {
        self.selected_task_id
            .and_then(|id| self.tasks.iter().find(|t| t.id == id).cloned())
    }

    /// 初始化或更新 Agent 面板
    fn update_agent_panel(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(ref task) = self.get_selected_task() {
            // 创建新的 Agent 执行会话
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
    }
}

impl Render for KanbanBoard {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let all_statuses = vec![
            TaskStatus::Todo,
            TaskStatus::InProgress,
            TaskStatus::InReview,
            TaskStatus::Done,
            TaskStatus::Canceled,
        ];

        // 确保 Agent 面板已初始化
        if self.agent_panel.is_none() || self.selected_task_id.is_some() {
            self.update_agent_panel(window, cx);
        }

        h_flex()
            .id("kanban-board")
            .w_full()
            .h_full()
            .bg(rgb(BG_PRIMARY))
            .p(PADDING_MD)
            .gap(SPACING_SM)
            .child(
                h_flex()
                    .flex_1()
                    .gap(SPACING_SM)
                    .children(all_statuses.into_iter().map(|status| {
                        let tasks = self.tasks_by_status(status);
                        let selected = self.selected_task_id;
                        KanbanColumn::new(status)
                            .tasks(tasks)
                            .selected_task_id(selected)
                    })),
            )
            .when_some(self.get_selected_task(), |this, task| {
                this.when(self.detail_panel_visible, |this| {
                    // 显示任务详情和 Agent 执行面板
                    this.child(
                        div()
                            .w(px(450.0))
                            .h_full()
                            .flex()
                            .flex_col()
                            .gap(SPACING_SM)
                            .child(
                                // 任务基本信息
                                div()
                                    .p(PADDING_MD)
                                    .bg(rgb(BG_CARD))
                                    .rounded(BORDER_RADIUS)
                                    .child(
                                        v_flex()
                                            .gap(SPACING_SM)
                                            .child(
                                                div()
                                                    .text_lg()
                                                    .font_weight(FontWeight::SEMIBOLD)
                                                    .text_color(rgb(TEXT_PRIMARY))
                                                    .child(task.title.clone())
                                            )
                                            .child(div().h(px(1.0)).bg(rgb(BORDER_COLOR)))
                                            .child(
                                                h_flex()
                                                    .gap(SPACING_SM)
                                                    .child(div().text_sm().text_color(rgb(TEXT_SECONDARY)).child("ID:"))
                                                    .child(div().text_sm().child(format!("#{}", task.id)))
                                            )
                                            .child(
                                                h_flex()
                                                    .gap(SPACING_SM)
                                                    .child(div().text_sm().text_color(rgb(TEXT_SECONDARY)).child("Status:"))
                                                    .child(div().text_sm().child(task.status.clone()))
                                            )
                                            .child(
                                                h_flex()
                                                    .gap(SPACING_SM)
                                                    .child(div().text_sm().text_color(rgb(TEXT_SECONDARY)).child("Priority:"))
                                                    .child(div().text_sm().child(task.priority.clone()))
                                            )
                                            .child(
                                                h_flex()
                                                    .gap(SPACING_SM)
                                                    .child(div().text_sm().text_color(rgb(TEXT_SECONDARY)).child("Type:"))
                                                    .child(div().text_sm().child(task.task_type.clone()))
                                            )
                                    )
                            )
                            .when_some(self.agent_panel.clone(), |this, panel| {
                                this.child(
                                    div()
                                        .flex_1()
                                        .child(panel)
                                )
                            })
                    )
                })
            })
    }
}
