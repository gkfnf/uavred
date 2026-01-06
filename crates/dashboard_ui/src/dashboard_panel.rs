// Dashboard 面板 Entity - 类似 zed 的 Pane

use crate::findings::render_findings_view;
use crate::mission_control::render_mission_control;
use data::{TaskData, TaskStatus, TaskStore};
use gpui::EventEmitter;
use gpui::*;
use gpui_component::{
    button::{Button, ButtonVariants as _},
    h_flex, v_flex, Selectable, Sizable,
};
use ui::events::DashboardEvent;
use workspace::DashboardView;

/// Dashboard 面板 - 管理 Mission Control 和 Findings 两个子视图
pub struct DashboardPanel {
    pub view: DashboardView,
    pub selected_task_id: Option<usize>,
    pub todo_tasks: Vec<TaskData>,
    pub in_progress_tasks: Vec<TaskData>,
    pub done_tasks: Vec<TaskData>,
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
            done_tasks: Vec::new(),
            task_store: task_store.clone(),
            _subscriptions: Vec::new(),
        };

        // 监听 TaskStore 的变化
        panel
            ._subscriptions
            .push(cx.observe(&task_store, |this, _store, cx| {
                // 从 this.task_store 读取最新数据
                this.todo_tasks = this.task_store.read(cx).get_tasks(TaskStatus::Todo);
                this.in_progress_tasks = this.task_store.read(cx).get_tasks(TaskStatus::InProgress);
                this.done_tasks = this.task_store.read(cx).get_tasks(TaskStatus::Done);
                cx.notify();
            }));

        // 初始化任务列表
        panel.todo_tasks = task_store.read(cx).get_tasks(TaskStatus::Todo);
        panel.in_progress_tasks = task_store.read(cx).get_tasks(TaskStatus::InProgress);
        panel.done_tasks = task_store.read(cx).get_tasks(TaskStatus::Done);

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

        cx.emit(DashboardEvent::TaskAdded(task.to_workspace()));
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
