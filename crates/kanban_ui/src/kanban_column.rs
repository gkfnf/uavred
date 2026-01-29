//! KanbanColumn 组件 - 看板列
//!
//! 显示单个状态列，包含该状态的所有任务卡片

use crate::task_card::{DragTask, TaskCard};
use data::{TaskData, TaskStatus};
use gpui::prelude::FluentBuilder as _;
use gpui::*;
use gpui_component::{
    IconName, Sizable, StyledExt as _,
    button::{Button, ButtonVariants as _},
    h_flex,
    scroll::ScrollableElement as _,
    v_flex,
};
use std::rc::Rc;
use ui::theme::*;

type TaskClickHandler = Rc<dyn Fn(usize, &mut Window, &mut App) + 'static>;

type TaskDropHandler = Box<dyn Fn(usize, String, String, &mut Window, &mut App) + 'static>;

/// 看板列组件
#[derive(IntoElement)]
pub struct KanbanColumn {
    status: TaskStatus,
    title: String,
    tasks: Vec<TaskData>,
    selected_task_id: Option<usize>,
    collapsed: bool,
    on_task_click: Option<TaskClickHandler>,
    on_task_drop: Option<TaskDropHandler>,
}

impl KanbanColumn {
    pub fn new(status: TaskStatus) -> Self {
        let title = match status {
            TaskStatus::Todo => "Todo",
            TaskStatus::InProgress => "In Progress",
            TaskStatus::InReview => "In Review",
            TaskStatus::Done => "Done",
            TaskStatus::Canceled => "Canceled",
        }
        .to_string();

        Self {
            status,
            title,
            tasks: Vec::new(),
            selected_task_id: None,
            collapsed: false,
            on_task_click: None,
            on_task_drop: None,
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn tasks(mut self, tasks: Vec<TaskData>) -> Self {
        self.tasks = tasks;
        self
    }

    pub fn selected_task_id(mut self, id: Option<usize>) -> Self {
        self.selected_task_id = id;
        self
    }

    pub fn on_task_click(
        mut self,
        handler: impl Fn(usize, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_task_click = Some(Rc::new(handler));
        self
    }

    pub fn collapsed(mut self, collapsed: bool) -> Self {
        self.collapsed = collapsed;
        self
    }

    pub fn on_task_drop(
        mut self,
        handler: impl Fn(usize, String, String, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_task_drop = Some(Box::new(handler));
        self
    }

    fn status_bg_color(&self) -> u32 {
        match self.status {
            TaskStatus::Todo => STATUS_TODO_BG,
            TaskStatus::InProgress => STATUS_IN_PROGRESS_BG,
            TaskStatus::InReview => STATUS_IN_REVIEW_BG,
            TaskStatus::Done => STATUS_DONE_BG,
            TaskStatus::Canceled => STATUS_CANCELED_BG,
        }
    }

    fn drag_over_color(&self) -> u32 {
        match self.status {
            TaskStatus::Todo => 0xe5e7eb,
            TaskStatus::InProgress => 0xbfdbfe,
            TaskStatus::InReview => 0xfde68a,
            TaskStatus::Done => 0xa7f3d0,
            TaskStatus::Canceled => 0xfecaca,
        }
    }
}

impl RenderOnce for KanbanColumn {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let status_bg = self.status_bg_color();
        let drag_over_bg = self.drag_over_color();
        let task_count = self.tasks.len();
        let selected_id = self.selected_task_id;
        let target_status = self.status.as_str().to_string();

        v_flex()
            .id(format!("kanban-column-{}", target_status))
            .w(KANBAN_COLUMN_WIDTH)
            .h_full()
            .flex_shrink_0()
            .rounded(BORDER_RADIUS)
            .bg(rgb(status_bg))
            .p(PADDING_SM)
            .gap(SPACING_SM)
            .drag_over::<DragTask>(move |this, _, _, _cx| {
                this.bg(rgb(drag_over_bg))
                    .border_2()
                    .border_color(rgb(ACCENT_PURPLE))
            })
            .when_some(self.on_task_drop, |this, handler| {
                this.on_drop(move |drag: &DragTask, window, cx| {
                    if drag.from_status != target_status {
                        handler(drag.task_id, drag.from_status.clone(), target_status.clone(), window, cx);
                    }
                })
            })
            .child(
                h_flex()
                    .items_center()
                    .justify_between()
                    .px(PADDING_XS)
                    .py(PADDING_SM)
                    .child(
                        h_flex()
                            .items_center()
                            .gap(SPACING_SM)
                            .child(
                                div()
                                    .text_sm()
                                    .font_semibold()
                                    .text_color(rgb(TEXT_PRIMARY))
                                    .child(self.title.clone()),
                            )
                            .child(
                                div()
                                    .px(PADDING_XS)
                                    .py(px(2.0))
                                    .rounded_full()
                                    .bg(rgb(TEXT_MUTED))
                                    .text_xs()
                                    .text_color(rgb(0xffffff))
                                    .child(format!("{}", task_count)),
                            ),
                    )
                    .child(
                        Button::new(format!("collapse-{:?}", self.status))
                            .ghost()
                            .xsmall()
                            .icon(if self.collapsed {
                                IconName::ChevronRight
                            } else {
                                IconName::ChevronDown
                            }),
                    ),
            )
            .when(!self.collapsed, |this| {
                this.child(
                    v_flex()
                        .flex_1()
                        .overflow_y_scrollbar()
                        .gap(SPACING_SM)
                        .when(self.tasks.is_empty(), |inner| {
                            inner.child(
                                div()
                                    .flex_1()
                                    .items_center()
                                    .justify_center()
                                    .p(PADDING_LG)
                                    .text_color(rgb(TEXT_MUTED))
                                    .text_sm()
                                    .child("No tasks"),
                            )
                        })
                        .when(!self.tasks.is_empty(), |inner| {
                            inner.children(self.tasks.into_iter().map(|task| {
                                let task_id = task.id;
                                let is_selected = selected_id == Some(task_id);
                                let on_click = self.on_task_click.clone();

                                TaskCard::new(task).selected(is_selected).on_click(
                                    move |_event, window, cx| {
                                        if let Some(ref handler) = on_click {
                                            handler(task_id, window, cx);
                                        }
                                    },
                                )
                            }))
                        }),
                )
            })
    }
}
