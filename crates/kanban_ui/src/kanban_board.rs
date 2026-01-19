//! KanbanBoard 组件 - 主容器
//!
//! 5 列看板加 squeeze-style 详情面板的主容器

use crate::kanban_column::KanbanColumn;
use data::{TaskData, TaskStatus};
use gpui::prelude::FluentBuilder as _;
use gpui::*;
use gpui_component::h_flex;
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
}

impl KanbanBoard {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            selected_task_id: None,
            detail_panel_visible: false,
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
        self.tasks
            .iter()
            .filter(|t| t.status == status)
            .cloned()
            .collect()
    }

    fn get_selected_task(&self) -> Option<TaskData> {
        self.selected_task_id
            .and_then(|id| self.tasks.iter().find(|t| t.id == id).cloned())
    }
}

impl Render for KanbanBoard {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let all_statuses = vec![
            TaskStatus::Todo,
            TaskStatus::InProgress,
            TaskStatus::InReview,
            TaskStatus::Done,
            TaskStatus::Canceled,
        ];

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
            .when_some(self.get_selected_task(), |this, _task| {
                this.when(self.detail_panel_visible, |this| this.child(div()))
            })
    }
}
