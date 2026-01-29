// TaskStore - 管理任务状态的 Entity，类似 Zed 的 Store 模式

use gpui::{App, AppContext, Context, Entity, EventEmitter, Global};
use std::sync::{Arc, Mutex};

use crate::database::TasksDatabase;
use crate::models::{TaskData, TaskStatus};

/// TaskStore 事件
#[derive(Debug, Clone)]
pub enum TaskStoreEvent {
    TasksUpdated,
    TaskAdded(TaskData),
    TaskUpdated(TaskData),
    TaskDeleted(usize),
}

/// 全局 TaskStore 包装
struct GlobalTaskStore(Entity<TaskStore>);

impl Global for GlobalTaskStore {}

/// TaskStore Entity - 管理任务状态
pub struct TaskStore {
    database: Arc<Mutex<TasksDatabase>>,
    tasks: Vec<TaskData>,
    _subscriptions: Vec<gpui::Subscription>,
}

impl EventEmitter<TaskStoreEvent> for TaskStore {}

impl TaskStore {
    pub fn new() -> Self {
        Self {
            database: Arc::new(Mutex::new(
                TasksDatabase::new().expect("Failed to initialize database"),
            )),
            tasks: Vec::new(),
            _subscriptions: Vec::new(),
        }
    }

    pub fn get_tasks(&self, status: TaskStatus) -> Vec<TaskData> {
        self.tasks
            .iter()
            .filter(|task| task.status == status)
            .cloned()
            .collect()
    }

    pub fn get_next_task_id(&self) -> usize {
        let database = self.database.lock().unwrap();
        database
            .get_next_task_id()
            .unwrap_or_else(|_| self.tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1)
    }

    pub fn add_task(&mut self, task: TaskData, cx: &mut Context<Self>) {
        self.tasks.push(task.clone());

        // 同步保存到数据库
        let database = self.database.lock().unwrap();
        if let Err(e) = database.save_task(&task) {
            eprintln!("Failed to save task: {}", e);
        }
        drop(database);

        cx.emit(TaskStoreEvent::TaskAdded(task));
        cx.notify();
    }

    pub fn update_task(&mut self, task: TaskData, cx: &mut Context<Self>) {
        // 找到任务并更新
        if let Some(pos) = self.tasks.iter().position(|t| t.id == task.id) {
            self.tasks[pos] = task.clone();
        }

        // 同步保存到数据库
        let database = self.database.lock().unwrap();
        if let Err(e) = database.save_task(&task) {
            eprintln!("Failed to update task: {}", e);
        }
        drop(database);

        cx.emit(TaskStoreEvent::TaskUpdated(task));
        cx.notify();
    }

    pub fn delete_task(&mut self, task_id: usize, cx: &mut Context<Self>) {
        self.tasks.retain(|t| t.id != task_id);
        let database = self.database.clone();

        cx.spawn({
            let database = database.clone();
            async move |_this, _cx| {
                if let Ok(db) = database.lock() {
                    let _ = db.delete_task(task_id);
                }
                Ok::<(), anyhow::Error>(())
            }
        })
        .detach_and_log_err(cx);

        cx.emit(TaskStoreEvent::TaskDeleted(task_id));
        cx.notify();
    }

    pub fn reload_sync(&mut self, cx: &mut Context<Self>) {
        let database = self.database.lock().unwrap();

        if let Ok(tasks) = database.list_tasks(TaskStatus::Todo) {
            self.tasks = tasks;
        }

        drop(database);
        cx.notify();
    }

    pub fn reload(&mut self, cx: &mut Context<Self>) {
        let database = self.database.clone();

        cx.spawn(async move |this, cx| {
            let db = database.lock().unwrap();
            if let Ok(tasks) = db.list_tasks(TaskStatus::Todo) {
                drop(db);
                let _ = this.update(cx, |this, cx| {
                    this.tasks = tasks;
                    cx.emit(TaskStoreEvent::TasksUpdated);
                    cx.notify();
                });
            }

            Ok::<(), anyhow::Error>(())
        })
        .detach_and_log_err(cx);
    }
}

impl TaskStore {
    pub fn global(cx: &mut App) -> Entity<Self> {
        if cx.has_global::<GlobalTaskStore>() {
            return cx.global::<GlobalTaskStore>().0.clone();
        }

        panic!(
            "TaskStore::global() called but no global TaskStore exists. Initialize from workspace initialization."
        )
    }
}

pub fn init_task_store(cx: &mut App) {
    if !cx.has_global::<GlobalTaskStore>() {
        let task_store = cx.new(|_cx| TaskStore::new());
        cx.set_global(GlobalTaskStore(task_store));
    }
}
