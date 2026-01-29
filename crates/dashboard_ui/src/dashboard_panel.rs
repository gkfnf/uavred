// Dashboard 面板 Entity - 类似 zed 的 Pane

use crate::add_task_modal::AddTaskModal;
use crate::findings::render_findings_view;
use crate::mission_control::render_mission_control;
use data::{TaskData, TaskStatus, TaskStore};
use gpui::EventEmitter;
use gpui::*;
use gpui_component::{
    Selectable, Sizable, WindowExt,
    button::{Button, ButtonVariants as _},
    h_flex, v_flex,
};
use ui::events::DashboardEvent;
use workspace::DashboardView;

/// Dashboard 面板 - 管理 Mission Control 和 Findings 两个子视图
pub struct DashboardPanel {
    pub view: DashboardView,
    pub selected_task_id: Option<usize>,
    pub todo_tasks: Vec<TaskData>,
    pub in_progress_tasks: Vec<TaskData>,
    pub in_review_tasks: Vec<TaskData>,
    pub done_tasks: Vec<TaskData>,
    pub canceled_tasks: Vec<TaskData>,
    task_store: Entity<TaskStore>,
    _subscriptions: Vec<Subscription>,
}

impl EventEmitter<DashboardEvent> for DashboardPanel {}

impl DashboardPanel {
    pub fn new(cx: &mut Context<Self>) -> Self {
        // 获取 App 上下文来访问全局 TaskStore
        // Context<T> 可以解引用为 App
        let task_store = TaskStore::global(&mut **cx);

        let mut panel = Self {
            view: DashboardView::MissionControl,
            selected_task_id: None,
            todo_tasks: Vec::new(),
            in_progress_tasks: Vec::new(),
            in_review_tasks: Vec::new(),
            done_tasks: Vec::new(),
            canceled_tasks: Vec::new(),
            task_store: task_store.clone(),
            _subscriptions: Vec::new(),
        };

        // 监听 TaskStore 的变化
        panel
            ._subscriptions
            .push(cx.subscribe(&task_store, |this, _store, _event, cx| {
                eprintln!("\n=== TaskStore subscription TRIGGERED ===");
                // 从 this.task_store 读取最新数据
                this.todo_tasks = this.task_store.read(cx).get_tasks(TaskStatus::Todo);
                eprintln!("Todo tasks: {}", this.todo_tasks.len());
                this.in_progress_tasks = this.task_store.read(cx).get_tasks(TaskStatus::InProgress);
                eprintln!("InProgress tasks: {}", this.in_progress_tasks.len());
                this.in_review_tasks = this.task_store.read(cx).get_tasks(TaskStatus::InReview);
                this.done_tasks = this.task_store.read(cx).get_tasks(TaskStatus::Done);
                this.canceled_tasks = this.task_store.read(cx).get_tasks(TaskStatus::Canceled);
                eprintln!("Calling cx.notify()");
                cx.notify();
                eprintln!("=== TaskStore subscription END ===\n");
            }));

        // 初始化任务列表
        panel.todo_tasks = task_store.read(cx).get_tasks(TaskStatus::Todo);
        panel.in_progress_tasks = task_store.read(cx).get_tasks(TaskStatus::InProgress);
        panel.in_review_tasks = task_store.read(cx).get_tasks(TaskStatus::InReview);
        panel.done_tasks = task_store.read(cx).get_tasks(TaskStatus::Done);
        panel.canceled_tasks = task_store.read(cx).get_tasks(TaskStatus::Canceled);

        panel
    }

    pub fn set_view(&mut self, view: DashboardView, cx: &mut Context<Self>) {
        self.view = view;
        cx.emit(DashboardEvent::ViewChanged(view));
    }

    pub fn select_task(&mut self, task_id: Option<usize>, cx: &mut Context<Self>) {
        if self.selected_task_id != task_id {
            self.selected_task_id = task_id;
            cx.emit(DashboardEvent::TaskSelected(task_id));
        }
    }

    pub fn get_next_task_id(&self, cx: &mut Context<Self>) -> usize {
        self.task_store.read(cx).get_next_task_id()
    }

    pub fn add_task(&mut self, task: TaskData, cx: &mut Context<Self>) {
        let task_clone = task.clone();
        let task_store = self.task_store.clone();

        // 保存到数据库
        task_store.update(cx, |store, cx| {
            store.add_task(task_clone.clone(), cx);
        });

        cx.emit(DashboardEvent::TaskAdded(task));
    }

    pub fn delete_task(&mut self, task_id: usize, cx: &mut Context<Self>) {
        let task_store = self.task_store.clone();

        task_store.update(cx, |store, cx| {
            store.delete_task(task_id, cx);
        });

        // 只有当删除的是当前选中的任务时，才更新选中状态
        if self.selected_task_id == Some(task_id) {
            // 尝试在所有任务列表中找到另一个任务保持面板展开
            let remaining_task = self
                .todo_tasks
                .iter()
                .chain(self.in_progress_tasks.iter())
                .chain(self.in_review_tasks.iter())
                .chain(self.done_tasks.iter())
                .chain(self.canceled_tasks.iter())
                .find(|t| t.id != task_id)
                .map(|t| t.id);

            self.selected_task_id = remaining_task;
        }

        cx.emit(DashboardEvent::TaskRemoved(task_id));
    }

    pub fn start_task(&mut self, task_id: usize, cx: &mut Context<Self>) {
        eprintln!("DEBUG: start_task called for id={}", task_id);
        // 从所有任务列表中找到该任务
        if let Some(task) = self
            .todo_tasks
            .iter_mut()
            .find(|t| t.id == task_id)
        {
            eprintln!("DEBUG: Found task in todo_tasks, changing status to InProgress");
            task.status = "in_progress".to_string();
            
            let updated_task = task.clone();
            let task_store = self.task_store.clone();
            
            // 更新数据库中的任务状态
            task_store.update(cx, |store, cx| {
                eprintln!("DEBUG: Calling store.update_task with id={}", updated_task.id);
                store.update_task(updated_task.clone(), cx);
            });
        }
    }

    pub fn open_add_task_dialog(
        &mut self,
        status: TaskStatus,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // 创建表单实体 - InputState 必须在持久 Entity 中创建
        let modal = cx.new(|cx| AddTaskModal::new(window, cx, status));

        let this_handle = cx.entity().downgrade();
        
        window.open_dialog(cx, move |dialog, _window, _cx| {
            let modal_clone = modal.clone();
            dialog
                .title("创建新任务")
                .w(px(600.0))
                .child(modal.clone())
                .confirm()
                .on_ok({
                    let modal = modal_clone;
                    let this_handle = this_handle.clone();
                    move |_event, _window, cx| {
                        eprintln!("DEBUG: Dialog on_ok triggered");
                        let title = modal.read(cx).get_title(cx);
                        eprintln!("DEBUG: Title={}", title);
                        
                        if !title.is_empty() {
                            let description = modal.read(cx).get_description(cx);
                            let is_auto_start = modal.read(cx).is_auto_start();
                            eprintln!("DEBUG: Description={}, auto_start={}", description, is_auto_start);
                            
                            let task_status_str = if is_auto_start {
                                "in_progress"
                            } else {
                                "todo"
                            };
                            
                            if let Some(handle) = this_handle.upgrade() {
                                eprintln!("DEBUG: handle upgraded");
                                handle.update(cx, |this, cx| {
                                    eprintln!("DEBUG: inside handle.update");
                                    let task_id = this.get_next_task_id(cx);
                                    eprintln!("DEBUG: task_id={}", task_id);
                                    let task = TaskData {
                                        id: task_id,
                                        title,
                                        task_type: String::from("task"),
                                        priority: String::from("Medium"),
                                        status: task_status_str.to_string(),
                                    };
                                    eprintln!("DEBUG: calling add_task");
                                    this.add_task(task, cx);
                                });
                            } else {
                                eprintln!("DEBUG: handle upgrade failed");
                            }
                        }
                        true
                    }
                })
        });
    }
}

impl Render for DashboardPanel {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .gap(px(0.0))
            .child(self.render_header(cx))
            .child(match self.view {
                DashboardView::MissionControl => {
                    render_mission_control(self, window, cx).into_any_element()
                }
                DashboardView::Findings => {
                    render_findings_view(self, window, cx).into_any_element()
                }
            })
    }
}

impl DashboardPanel {
    fn render_header(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let mission_control_active = self.view == DashboardView::MissionControl;
        let findings_active = self.view == DashboardView::Findings;

        h_flex()
            .w_full()
            .min_h(px(48.0))
            .items_center()
            .px(px(24.0))
            .pt(px(12.0))
            .pb(px(0.0))
            .mb(px(0.0))
            .bg(rgb(0xffffff))
            .border_b(px(1.0))
            .border_color(rgb(0xe5e7eb))
            .gap(px(16.0))
            .child(h_flex().gap(px(8.0)).items_center().children(vec![
                        Button::new("dashboard-tab-mission-control")
                            .ghost()
                            .small()
                            .label("Mission Control")
                            .selected(mission_control_active)
                            .on_click(cx.listener(move |this: &mut Self, _, _, cx| {
                                this.set_view(DashboardView::MissionControl, cx);
                            })),
                        Button::new("dashboard-tab-findings")
                            .ghost()
                            .small()
                            .label("Findings 5")
                            .selected(findings_active)
                            .on_click(cx.listener(move |this: &mut Self, _, _, cx| {
                                this.set_view(DashboardView::Findings, cx);
                            })),
                    ]))
            .child(div().flex_1())
    }
}

