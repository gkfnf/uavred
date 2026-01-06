// 任务数据库 - 管理 SQLite 连接和查询

use anyhow::{Context, Result};
use chrono::Utc;
use sqlez::connection::Connection;
use std::sync::{Arc, Mutex};

use crate::models::{TaskData, TaskStatus};

/// 任务数据库连接
pub struct TasksDatabase {
    connection: Arc<Mutex<Connection>>,
}

impl TasksDatabase {
    /// 创建新的数据库连接
    pub fn new() -> Result<Self> {
        // 获取数据目录
        let data_dir = dirs::data_dir()
            .ok_or_else(|| anyhow::anyhow!("无法获取数据目录"))?
            .join("uavred");

        std::fs::create_dir_all(&data_dir).context("无法创建数据目录")?;

        let db_path = data_dir.join("tasks.db");
        let connection = Connection::open_file(&db_path.to_string_lossy());

        // 创建表
        connection.exec(indoc::indoc! {"
            CREATE TABLE IF NOT EXISTS tasks (
                id INTEGER PRIMARY KEY,
                title TEXT NOT NULL,
                task_type TEXT NOT NULL,
                priority TEXT NOT NULL,
                status TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )
        "})?()
        .map_err(|e| anyhow::anyhow!("无法创建表: {}", e))?;

        Ok(Self {
            connection: Arc::new(Mutex::new(connection)),
        })
    }

    /// 列出指定状态的任务
    pub fn list_tasks(&self, status: TaskStatus) -> Result<Vec<TaskData>> {
        let connection = self.connection.lock().unwrap();
        let status_str = status_to_string(status);

        // 转义单引号并使用原始 SQL
        let status_str_escaped = status_str.replace("'", "''");
        let sql = format!(
            "SELECT id, title, task_type, priority, status, created_at, updated_at \
             FROM tasks WHERE status = '{}' ORDER BY id ASC",
            status_str_escaped
        );
        let rows =
            connection.select::<(i64, String, String, String, String, String, String)>(&sql)?()?;

        let mut tasks = Vec::new();
        for (id, title, task_type, priority, status_str, _created_at, _updated_at) in rows {
            let status = string_to_status(&status_str)?;
            tasks.push(TaskData {
                id: id as usize,
                title,
                task_type,
                priority,
                status,
            });
        }

        Ok(tasks)
    }

    /// 保存任务（插入或更新）
    pub fn save_task(&self, task: &TaskData) -> Result<()> {
        let connection = self.connection.lock().unwrap();
        let now = Utc::now().to_rfc3339();
        let status_str = status_to_string(task.status);

        // 使用原始 SQL，转义单引号
        let title_escaped = task.title.replace("'", "''");
        let task_type_escaped = task.task_type.replace("'", "''");
        let priority_escaped = task.priority.replace("'", "''");

        let sql = format!(
            "INSERT OR REPLACE INTO tasks (id, title, task_type, priority, status, created_at, updated_at) \
             VALUES ({}, '{}', '{}', '{}', '{}', COALESCE((SELECT created_at FROM tasks WHERE id = {}), '{}'), '{}')",
            task.id, title_escaped, task_type_escaped, priority_escaped, status_str, task.id, now, now
        );

        connection.exec(&sql)?()?;

        Ok(())
    }

    /// 删除任务
    pub fn delete_task(&self, id: usize) -> Result<()> {
        let connection = self.connection.lock().unwrap();

        let sql = format!("DELETE FROM tasks WHERE id = {}", id);
        connection.exec(&sql)?()?;

        Ok(())
    }

    /// 更新任务
    pub fn update_task(&self, task: &TaskData) -> Result<()> {
        self.save_task(task)
    }

    /// 获取下一个可用的任务 ID
    pub fn get_next_task_id(&self) -> Result<usize> {
        let connection = self.connection.lock().unwrap();

        let rows = connection.select::<i64>("SELECT COALESCE(MAX(id), 0) FROM tasks")?()?;
        let max_id = rows.into_iter().next().unwrap_or(0) as usize;

        Ok(max_id + 1)
    }
}

fn status_to_string(status: TaskStatus) -> String {
    match status {
        TaskStatus::Todo => "todo".to_string(),
        TaskStatus::InProgress => "in_progress".to_string(),
        TaskStatus::Done => "done".to_string(),
    }
}

fn string_to_status(s: &str) -> Result<TaskStatus> {
    match s {
        "todo" => Ok(TaskStatus::Todo),
        "in_progress" => Ok(TaskStatus::InProgress),
        "done" => Ok(TaskStatus::Done),
        _ => Err(anyhow::anyhow!("无效的任务状态: {}", s)),
    }
}
