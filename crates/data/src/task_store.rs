// TaskStore V2 - 使用新的 UavredDatabase
// 提供全局状态管理 + 数据库持久化

use gpui::{App, AppContext, Context, Entity, EventEmitter, Global};
use std::sync::{Arc, Mutex};

use crate::models::{Task, TaskStatus};
use crate::repository::Database;
use workspace::TaskData;

/// TaskStore 事件
#[derive(Debug, Clone)]
pub enum TaskStoreEvent {
    TasksUpdated,
    TaskAdded(Task),
    TaskUpdated(Task),
    TaskDeleted(i64),
}

/// 全局 TaskStore 包装
struct GlobalTaskStore(Entity<TaskStore>);

impl Global for GlobalTaskStore {}

/// TaskStore Entity - 管理任务状态
pub struct TaskStore {
    db: Arc<Mutex<Database>>,
    tasks: Vec<Task>,
}

impl EventEmitter<TaskStoreEvent> for TaskStore {}

impl TaskStore {
    pub fn new() -> anyhow::Result<Self> {
        // 使用本地开发数据库
        let db = Database::open_local()?;
        
        Ok(Self {
            db: Arc::new(Mutex::new(db)),
            tasks: Vec::new(),
        })
    }

    /// 从数据库加载所有任务
    pub fn load_all_tasks(&mut self, cx: &mut Context<Self>) -> anyhow::Result<()> {
        let db = self.db.lock().unwrap();
        
        // 加载所有状态的任务
        let mut all_tasks = Vec::new();
        for status in [TaskStatus::Todo, TaskStatus::InProgress, TaskStatus::InReview, TaskStatus::Done, TaskStatus::Canceled] {
            let tasks = db.tasks().list_by_status(status)?;
            all_tasks.extend(tasks);
        }
        
        drop(db);
        
        self.tasks = all_tasks;
        cx.emit(TaskStoreEvent::TasksUpdated);
        cx.notify();
        
        Ok(())
    }

    /// 获取指定状态的任务（返回 TaskData 以兼容 UI）
    pub fn get_tasks(&self, status: TaskStatus) -> Vec<TaskData> {
        self.tasks
            .iter()
            .filter(|task| task.status == status)
            .map(|task| TaskData::from(task.clone()))
            .collect()
    }
    
    /// 获取指定状态的任务（返回原始 Task）
    pub fn get_tasks_raw(&self, status: TaskStatus) -> Vec<Task> {
        self.tasks
            .iter()
            .filter(|task| task.status == status)
            .cloned()
            .collect()
    }

    /// 获取单个任务
    pub fn get_task(&self, id: i64) -> Option<Task> {
        self.tasks.iter().find(|t| t.id == id).cloned()
    }

    /// 获取下一个任务 ID（用于 UI 创建新任务时）
    pub fn get_next_task_id(&self) -> usize {
        self.tasks.iter().map(|t| t.id).max().unwrap_or(0) as usize + 1
    }

    /// 添加任务（使用 TaskData）
    pub fn add_task(&mut self, task_data: TaskData, cx: &mut Context<Self>) {
        let task: Task = task_data.into();
        
        let db = self.db.lock().unwrap();
        if let Ok(id) = db.tasks().create(&task) {
            drop(db);
            
            let mut task_with_id = task.clone();
            task_with_id.id = id;
            self.tasks.push(task_with_id.clone());
            
            cx.emit(TaskStoreEvent::TaskAdded(task_with_id));
            cx.notify();
        }
    }
    
    /// 添加任务（使用原始 Task）
    pub fn add_task_raw(&mut self, mut task: Task, cx: &mut Context<Self>) -> anyhow::Result<i64> {
        let db = self.db.lock().unwrap();
        let id = db.tasks().create(&task)?;
        drop(db);
        
        task.id = id;
        self.tasks.push(task.clone());
        
        cx.emit(TaskStoreEvent::TaskAdded(task));
        cx.notify();
        
        Ok(id)
    }

    /// 更新任务（使用 TaskData）
    pub fn update_task(&mut self, task_data: TaskData, cx: &mut Context<Self>) {
        let task: Task = task_data.into();
        
        let db = self.db.lock().unwrap();
        if db.tasks().update(&task).is_ok() {
            drop(db);
            
            // 更新内存中的任务
            if let Some(pos) = self.tasks.iter().position(|t| t.id == task.id) {
                self.tasks[pos] = task.clone();
            }
            
            cx.emit(TaskStoreEvent::TaskUpdated(task));
            cx.notify();
        }
    }
    
    /// 更新任务（使用原始 Task）
    pub fn update_task_raw(&mut self, task: Task, cx: &mut Context<Self>) -> anyhow::Result<()> {
        let db = self.db.lock().unwrap();
        db.tasks().update(&task)?;
        drop(db);
        
        // 更新内存中的任务
        if let Some(pos) = self.tasks.iter().position(|t| t.id == task.id) {
            self.tasks[pos] = task.clone();
        }
        
        cx.emit(TaskStoreEvent::TaskUpdated(task));
        cx.notify();
        
        Ok(())
    }

    /// 删除任务
    pub fn delete_task(&mut self, task_id: usize, cx: &mut Context<Self>) {
        let task_id_i64 = task_id as i64;
        let db = self.db.lock().unwrap();
        if db.tasks().delete(task_id_i64).is_ok() {
            drop(db);
            
            self.tasks.retain(|t| t.id != task_id_i64);
            
            cx.emit(TaskStoreEvent::TaskDeleted(task_id_i64));
            cx.notify();
        }
    }

    /// 移动任务状态（看板拖拽）
    pub fn move_task_status(&mut self, task_id: i64, new_status: TaskStatus, cx: &mut Context<Self>) -> anyhow::Result<()> {
        if let Some(pos) = self.tasks.iter().position(|t| t.id == task_id) {
            let task = &mut self.tasks[pos];
            task.status = new_status.clone();
            task.updated_at = chrono::Utc::now();
            
            let db = self.db.lock().unwrap();
            db.tasks().update_status(task_id, new_status.as_str())?;
            drop(db);
            
            cx.emit(TaskStoreEvent::TaskUpdated(task.clone()));
            cx.notify();
        }
        
        Ok(())
    }

    /// 获取仪表盘统计
    pub fn get_dashboard_stats(&self) -> anyhow::Result<DashboardStats> {
        let stats = DashboardStats {
            total_tasks: self.tasks.len() as i64,
            tasks_by_status: vec![
                (TaskStatus::Todo, self.get_tasks(TaskStatus::Todo).len()),
                (TaskStatus::InProgress, self.get_tasks(TaskStatus::InProgress).len()),
                (TaskStatus::InReview, self.get_tasks(TaskStatus::InReview).len()),
                (TaskStatus::Done, self.get_tasks(TaskStatus::Done).len()),
                (TaskStatus::Canceled, self.get_tasks(TaskStatus::Canceled).len()),
            ],
            total_assets: 0, // TODO: 添加 AssetRepository 后实现
            total_findings: 0, // TODO: 添加 FindingRepository 后实现
        };
        
        Ok(stats)
    }
}

impl TaskStore {
    /// 获取全局 TaskStore Entity
    pub fn global(cx: &mut App) -> Entity<Self> {
        if cx.has_global::<GlobalTaskStore>() {
            return cx.global::<GlobalTaskStore>().0.clone();
        }

        panic!("TaskStore::global() called but no global TaskStore exists. Call init_task_store() first.")
    }
}

/// 仪表盘统计数据
#[derive(Debug, Clone)]
pub struct DashboardStats {
    pub total_tasks: i64,
    pub tasks_by_status: Vec<(TaskStatus, usize)>,
    pub total_assets: i64,
    pub total_findings: i64,
}

/// 初始化全局 TaskStore
pub fn init_task_store(cx: &mut App) {
    if !cx.has_global::<GlobalTaskStore>() {
        let task_store = cx.new(|_cx| TaskStore::new().expect("Failed to initialize TaskStore"));
        cx.set_global(GlobalTaskStore(task_store));
    }
}

/// 初始化并加载数据
pub fn init_and_load_task_store(cx: &mut App) {
    init_task_store(cx);
    
    // 触发数据加载
    let store = TaskStore::global(cx);
    store.update(cx, |store, cx| {
        if let Err(e) = store.load_all_tasks(cx) {
            eprintln!("Failed to load tasks: {}", e);
        }
    });
}
