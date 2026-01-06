// TaskStore - 管理任务状态的 Entity，类似 Zed 的 Store 模式

use gpui::{App, AppContext, Context, Entity, EventEmitter, Global};
use std::sync::Arc;
use std::sync::Mutex;

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

/// 全局 TaskStore
struct GlobalTaskStore(Entity<TaskStore>);

impl Global for GlobalTaskStore {}

/// TaskStore Entity - 管理任务状态
pub struct TaskStore {
    database: Arc<Mutex<TasksDatabase>>,
    todo_tasks: Vec<TaskData>,
    in_progress_tasks: Vec<TaskData>,
    done_tasks: Vec<TaskData>,
    _subscriptions: Vec<gpui::Subscription>,
}

impl EventEmitter<TaskStoreEvent> for TaskStore {}

impl TaskStore {
    /// 获取或创建全局 TaskStore
    pub fn global(cx: &mut App) -> Entity<Self> {
        if cx.has_global::<GlobalTaskStore>() {
            return cx.global::<GlobalTaskStore>().0.clone();
        }

        let store = cx.new(|cx| {
            let database = Arc::new(Mutex::new(
                TasksDatabase::new().expect("无法初始化数据库")
            ));
            
            let mut store = Self {
                database,
                todo_tasks: Vec::new(),
                in_progress_tasks: Vec::new(),
                done_tasks: Vec::new(),
                _subscriptions: Vec::new(),
            };
            
            // 加载任务
            store.reload_sync(cx);
            
            store
        });

        cx.set_global(GlobalTaskStore(store.clone()));
        store
    }

    /// 同步重新加载任务（在初始化时使用）
    fn reload_sync(&mut self, cx: &mut Context<Self>) {
        let database = self.database.lock().unwrap();
        
        self.todo_tasks = database.list_tasks(TaskStatus::Todo)
            .unwrap_or_default();
        self.in_progress_tasks = database.list_tasks(TaskStatus::InProgress)
            .unwrap_or_default();
        self.done_tasks = database.list_tasks(TaskStatus::Done)
            .unwrap_or_default();
        
        drop(database);
        cx.notify();
    }

    /// 从数据库重新加载任务
    pub fn reload(&mut self, cx: &mut Context<Self>) {
        let database = self.database.clone();
        
        cx.spawn(async move |this, cx| {
            let db = database.lock().unwrap();
            let todo = db.list_tasks(TaskStatus::Todo)?;
            let in_progress = db.list_tasks(TaskStatus::InProgress)?;
            let done = db.list_tasks(TaskStatus::Done)?;
            drop(db);
            
            let _ = this.update(cx, |this, cx| {
                this.todo_tasks = todo;
                this.in_progress_tasks = in_progress;
                this.done_tasks = done;
                cx.emit(TaskStoreEvent::TasksUpdated);
                cx.notify();
            });
            
            Ok::<_, anyhow::Error>(())
        })
        .detach_and_log_err(cx);
    }

    /// 获取指定状态的任务列表
    pub fn get_tasks(&self, status: TaskStatus) -> Vec<TaskData> {
        match status {
            TaskStatus::Todo => self.todo_tasks.clone(),
            TaskStatus::InProgress => self.in_progress_tasks.clone(),
            TaskStatus::Done => self.done_tasks.clone(),
        }
    }

    /// 添加任务
    pub fn add_task(&mut self, task: TaskData, cx: &mut Context<Self>) {
        let database = self.database.clone();
        let task_clone = task.clone();
        let status = task.status;
        
        cx.spawn(async move |this, cx| {
            let db = database.lock().unwrap();
            db.save_task(&task_clone)?;
            drop(db);
            
            let _ = this.update(cx, |this, cx| {
                match status {
                    TaskStatus::Todo => this.todo_tasks.push(task_clone.clone()),
                    TaskStatus::InProgress => this.in_progress_tasks.push(task_clone.clone()),
                    TaskStatus::Done => this.done_tasks.push(task_clone.clone()),
                }
                cx.emit(TaskStoreEvent::TaskAdded(task_clone.clone()));
                cx.notify();
            });
            
            Ok::<_, anyhow::Error>(())
        })
        .detach_and_log_err(cx);
    }

    /// 更新任务
    pub fn update_task(&mut self, task: TaskData, cx: &mut Context<Self>) {
        let database = self.database.clone();
        let task_clone = task.clone();
        let old_status = self.get_task_status(task.id);
        let new_status = task.status;
        
        cx.spawn(async move |this, cx| {
            let db = database.lock().unwrap();
            db.update_task(&task_clone)?;
            drop(db);
            
            let _ = this.update(cx, |this, cx| {
                // 从旧状态列表中移除
                match old_status {
                    Some(TaskStatus::Todo) => this.todo_tasks.retain(|t| t.id != task_clone.id),
                    Some(TaskStatus::InProgress) => this.in_progress_tasks.retain(|t| t.id != task_clone.id),
                    Some(TaskStatus::Done) => this.done_tasks.retain(|t| t.id != task_clone.id),
                    None => {}
                }
                
                // 添加到新状态列表
                match new_status {
                    TaskStatus::Todo => this.todo_tasks.push(task_clone.clone()),
                    TaskStatus::InProgress => this.in_progress_tasks.push(task_clone.clone()),
                    TaskStatus::Done => this.done_tasks.push(task_clone.clone()),
                }
                
                cx.emit(TaskStoreEvent::TaskUpdated(task_clone.clone()));
                cx.notify();
            });
            
            Ok::<_, anyhow::Error>(())
        })
        .detach_and_log_err(cx);
    }

    /// 删除任务
    pub fn delete_task(&mut self, id: usize, cx: &mut Context<Self>) {
        let database = self.database.clone();
        let status = self.get_task_status(id);
        
        cx.spawn(async move |this, cx| {
            let db = database.lock().unwrap();
            db.delete_task(id)?;
            drop(db);
            
            if let Some(status) = status {
                let _ = this.update(cx, |this, cx| {
                    match status {
                        TaskStatus::Todo => this.todo_tasks.retain(|t| t.id != id),
                        TaskStatus::InProgress => this.in_progress_tasks.retain(|t| t.id != id),
                        TaskStatus::Done => this.done_tasks.retain(|t| t.id != id),
                    }
                    cx.emit(TaskStoreEvent::TaskDeleted(id));
                    cx.notify();
                });
            }
            
            Ok::<_, anyhow::Error>(())
        })
        .detach_and_log_err(cx);
    }

    /// 获取下一个可用的任务 ID
    pub fn get_next_task_id(&self) -> usize {
        let database = self.database.lock().unwrap();
        database.get_next_task_id().unwrap_or(1)
    }

    /// 获取任务的状态
    fn get_task_status(&self, id: usize) -> Option<TaskStatus> {
        if self.todo_tasks.iter().any(|t| t.id == id) {
            Some(TaskStatus::Todo)
        } else if self.in_progress_tasks.iter().any(|t| t.id == id) {
            Some(TaskStatus::InProgress)
        } else if self.done_tasks.iter().any(|t| t.id == id) {
            Some(TaskStatus::Done)
        } else {
            None
        }
    }
}
