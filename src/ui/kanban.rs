use gpui::*;
use crate::ui::prelude::*;
use crate::ui::{badge, card};
use crate::core::task::{Task, TaskStatus, TaskPriority};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KanbanColumn {
    Todo,
    InProgress,
    Done,
}

pub struct TaskCard {
    task: Task,
}

impl TaskCard {
    pub fn new(task: Task) -> Self {
        Self { task }
    }

    fn render_priority_badge(&self) -> impl IntoElement {
        let (text, color) = match self.task.priority {
            TaskPriority::Critical => ("critical", gpui::rgb(0xef4444)),
            TaskPriority::High => ("high", gpui::rgb(0xf97316)),
            TaskPriority::Medium => ("medium", gpui::rgb(0xfbbf24)),
            TaskPriority::Low => ("low", gpui::rgb(0x10b981)),
        };

        div()
            .px_2()
            .py_0p5()
            .rounded_md()
            .bg(color.opacity(0.2))
            .child(
                label(text)
                    .text_size(ui::TextSize::XSmall)
                    .text_color(color)
            )
    }

    fn render_task_type_badge(&self) -> impl IntoElement {
        let (text, color) = match &self.task.task_type {
            crate::core::task::TaskType::NetworkScan { .. } => ("SCAN", gpui::rgb(0x60a5fa)),
            crate::core::task::TaskType::ProtocolAnalysis { .. } => ("ANALYSIS", gpui::rgb(0xa78bfa)),
            crate::core::task::TaskType::FirmwareAnalysis { .. } => ("RECON", gpui::rgb(0x34d399)),
            crate::core::task::TaskType::Exploit { .. } => ("EXPLOIT", gpui::rgb(0xf87171)),
        };

        div()
            .px_2()
            .py_0p5()
            .rounded_md()
            .border_1()
            .border_color(color)
            .child(
                label(text)
                    .text_size(ui::TextSize::XSmall)
                    .text_color(color)
            )
    }
}

impl Render for TaskCard {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        card()
            .p_3()
            .child(
                v_flex()
                    .gap_2()
                    .child(
                        label(&self.task.name)
                            .text_size(ui::TextSize::Medium)
                            .text_color(gpui::white())
                    )
                    .child(
                        h_flex()
                            .gap_2()
                            .child(self.render_task_type_badge())
                            .child(self.render_priority_badge())
                    )
            )
    }
}

pub struct KanbanBoard {
    todo_tasks: Vec<Task>,
    in_progress_tasks: Vec<Task>,
    done_tasks: Vec<Task>,
    selected_task: Option<usize>,
}

impl KanbanBoard {
    pub fn new(_cx: &mut ViewContext<Self>) -> Self {
        Self {
            todo_tasks: vec![],
            in_progress_tasks: vec![],
            done_tasks: vec![],
            selected_task: None,
        }
    }

    pub fn add_task(&mut self, task: Task) {
        match task.status {
            TaskStatus::Pending => self.todo_tasks.push(task),
            TaskStatus::Running => self.in_progress_tasks.push(task),
            TaskStatus::Completed => self.done_tasks.push(task),
            _ => {}
        }
    }

    fn render_column(&self, title: &str, count: usize, tasks: &[Task], _cx: &mut ViewContext<Self>) -> impl IntoElement {
        v_flex()
            .flex_1()
            .gap_3()
            .child(
                h_flex()
                    .items_center()
                    .justify_between()
                    .pb_2()
                    .child(
                        h_flex()
                            .gap_2()
                            .items_center()
                            .child(
                                div()
                                    .w_2()
                                    .h_2()
                                    .rounded_full()
                                    .bg(match title {
                                        "To Do" => gpui::rgb(0x6b7280),
                                        "In Progress" => gpui::rgb(0x60a5fa),
                                        "Done" => gpui::rgb(0x10b981),
                                        _ => gpui::rgb(0x6b7280),
                                    })
                            )
                            .child(
                                label(title)
                                    .text_size(ui::TextSize::Medium)
                                    .text_color(gpui::white())
                            )
                            .child(
                                label(&count.to_string())
                                    .text_size(ui::TextSize::Small)
                                    .text_color(gpui::rgb(0x9ca3af))
                            )
                    )
                    .child(
                        div()
                            .w_6()
                            .h_6()
                            .flex()
                            .items_center()
                            .justify_center()
                            .rounded_md()
                            .hover(|style| style.bg(gpui::rgb(0x2d2d2d)))
                            .child(label("+").text_color(gpui::rgb(0x9ca3af)))
                    )
            )
            .child(
                v_flex()
                    .gap_2()
                    .children(tasks.iter().map(|task| {
                        TaskCard::new(task.clone())
                    }))
            )
    }
}

impl Render for KanbanBoard {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        h_flex()
            .w_full()
            .gap_4()
            .p_4()
            .child(self.render_column("To Do", self.todo_tasks.len(), &self.todo_tasks, cx))
            .child(self.render_column("In Progress", self.in_progress_tasks.len(), &self.in_progress_tasks, cx))
            .child(self.render_column("Done", self.done_tasks.len(), &self.done_tasks, cx))
    }
}
