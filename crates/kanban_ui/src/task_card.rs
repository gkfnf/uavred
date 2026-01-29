//! TaskCard 组件 - 任务卡片
//!
//! 显示任务摘要信息，包括标题、优先级、类型标签

use data::TaskData;
use gpui::prelude::FluentBuilder as _;
use gpui::*;
use gpui_component::{
    ActiveTheme as _, StyledExt as _, h_flex,
    menu::{ContextMenuExt as _, PopupMenuItem},
    v_flex,
};
use std::rc::Rc;
use ui::theme::*;

/// 任务卡片点击回调类型
type TaskCardClickHandler = Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;

/// 任务编辑回调类型
type TaskEditHandler = Rc<dyn Fn(usize, &mut Window, &mut App) + 'static>;

/// 任务删除回调类型
type TaskDeleteHandler = Rc<dyn Fn(usize, &mut Window, &mut App) + 'static>;

/// 任务移动状态回调类型
type TaskMoveStatusHandler = Rc<dyn Fn(usize, String, &mut Window, &mut App) + 'static>;

/// 拖拽任务数据结构
#[derive(Clone)]
pub struct DragTask {
    /// 被拖拽的任务 ID
    pub task_id: usize,
    /// 任务标题 (用于显示拖拽预览)
    pub title: String,
    /// 原始状态 (String for compatibility)
    pub from_status: String,
}

impl DragTask {
    /// 创建新的拖拽任务
    pub fn new(task: &TaskData) -> Self {
        Self {
            task_id: task.id,
            title: task.title.clone(),
            from_status: task.status.clone(),
        }
    }
}

impl Render for DragTask {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("drag-task-preview")
            .cursor_grabbing()
            .py(PADDING_SM)
            .px(PADDING_MD)
            .w(px(200.0))
            .overflow_hidden()
            .whitespace_nowrap()
            .border_1()
            .border_color(cx.theme().border)
            .rounded(BORDER_RADIUS)
            .text_color(rgb(TEXT_PRIMARY))
            .bg(rgb(BG_CARD))
            .shadow_lg()
            .opacity(0.9)
            .text_sm()
            .child(self.title.clone())
    }
}

/// 任务卡片组件
#[derive(IntoElement)]
pub struct TaskCard {
    task: TaskData,
    selected: bool,
    on_click: Option<TaskCardClickHandler>,
    on_edit: Option<TaskEditHandler>,
    on_delete: Option<TaskDeleteHandler>,
    on_move_status: Option<TaskMoveStatusHandler>,
}

impl TaskCard {
    /// 创建新的任务卡片
    pub fn new(task: TaskData) -> Self {
        Self {
            task,
            selected: false,
            on_click: None,
            on_edit: None,
            on_delete: None,
            on_move_status: None,
        }
    }

    /// 设置选中状态
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// 设置点击回调
    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }

    /// 设置编辑回调
    pub fn on_edit(mut self, handler: impl Fn(usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_edit = Some(Rc::new(handler));
        self
    }

    /// 设置删除回调
    pub fn on_delete(mut self, handler: impl Fn(usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_delete = Some(Rc::new(handler));
        self
    }

    /// 设置移动状态回调
    pub fn on_move_status(
        mut self,
        handler: impl Fn(usize, String, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_move_status = Some(Rc::new(handler));
        self
    }

    /// 获取优先级颜色
    fn priority_color(&self) -> u32 {
        match self.task.priority.as_str() {
            "critical" => SEVERITY_CRITICAL,
            "high" => SEVERITY_HIGH,
            "medium" => SEVERITY_MEDIUM,
            "low" => SEVERITY_LOW,
            _ => TEXT_SECONDARY,
        }
    }
}

impl RenderOnce for TaskCard {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let task_id = self.task.id;
        let priority_color = self.priority_color();
        let is_selected = self.selected;
        let on_edit = self.on_edit.clone();
        let on_delete = self.on_delete.clone();
        let on_move_status = self.on_move_status.clone();

        let drag_task = DragTask::new(&self.task);

        let mut card = v_flex()
            .id(("task-card", task_id))
            .w_full()
            .min_h(KANBAN_CARD_MIN_HEIGHT)
            .p(PADDING_MD)
            .bg(rgb(BG_CARD))
            .rounded(BORDER_RADIUS)
            .border_1()
            .cursor_grab()
            .gap(SPACING_SM);

        card = if is_selected {
            card.border_color(rgb(ACCENT_PURPLE)).shadow_md()
        } else {
            card.border_color(rgb(BORDER_COLOR)).hover(|style| {
                style
                    .border_color(rgb(ACCENT_PURPLE))
                    .shadow_md()
                    .bg(rgb(BG_CARD_HOVER))
            })
        };

        let card_element = card
            .on_drag(drag_task, |drag, _, _, cx| cx.new(|_| drag.clone()))
            .context_menu({
                move |menu, window, cx| {
                    menu.when_some(on_edit.clone(), |this, handler| {
                        this.item(PopupMenuItem::new("Edit").on_click({
                            let handler = handler.clone();
                            move |_event, window, cx| {
                                handler(task_id, window, cx);
                            }
                        }))
                    })
                    .when_some(on_delete.clone(), |this, handler| {
                        this.item(PopupMenuItem::new("Delete").on_click({
                            let handler = handler.clone();
                            move |_event, window, cx| {
                                handler(task_id, window, cx);
                            }
                        }))
                    })
                    .separator()
                    .when_some(on_move_status.clone(), |this, handler| {
                        this.submenu("Move to", window, cx, move |submenu, _window, _cx| {
                            let handler = handler.clone();
                            submenu
                                .item(PopupMenuItem::new("Todo").on_click({
                                    let handler = handler.clone();
                                    move |_event, window, cx| {
                                        handler(task_id, "todo".to_string(), window, cx);
                                    }
                                }))
                                .item(PopupMenuItem::new("In Progress").on_click({
                                    let handler = handler.clone();
                                    move |_event, window, cx| {
                                        handler(task_id, "in_progress".to_string(), window, cx);
                                    }
                                }))
                                .item(PopupMenuItem::new("In Review").on_click({
                                    let handler = handler.clone();
                                    move |_event, window, cx| {
                                        handler(task_id, "in_review".to_string(), window, cx);
                                    }
                                }))
                                .item(PopupMenuItem::new("Done").on_click({
                                    let handler = handler.clone();
                                    move |_event, window, cx| {
                                        handler(task_id, "done".to_string(), window, cx);
                                    }
                                }))
                                .item(PopupMenuItem::new("Canceled").on_click({
                                    let handler = handler.clone();
                                    move |_event, window, cx| {
                                        handler(task_id, "canceled".to_string(), window, cx);
                                    }
                                }))
                        })
                    })
                }
            })
            .child(
                div()
                    .text_sm()
                    .font_medium()
                    .text_color(rgb(TEXT_PRIMARY))
                    .child(self.task.title.clone()),
            )
            .child(
                h_flex()
                    .items_center()
                    .justify_between()
                    .mt_auto()
                    .child(
                        div()
                            .px(PADDING_XS)
                            .py(px(2.0))
                            .rounded(BORDER_RADIUS_SM)
                            .bg(rgb(BG_SECONDARY))
                            .text_xs()
                            .text_color(rgb(TEXT_SECONDARY))
                            .child(self.task.task_type.clone()),
                    )
                    .child(
                        div()
                            .px(PADDING_XS)
                            .py(px(2.0))
                            .rounded(BORDER_RADIUS_SM)
                            .bg(rgb(priority_color))
                            .text_xs()
                            .text_color(rgb(0xffffff))
                            .child(self.task.priority.clone()),
                    ),
            );

        card_element
    }
}
