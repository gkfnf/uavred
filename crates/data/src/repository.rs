// Minimal Repository - 只包含核心 Task 操作
// 其他 repository 可以按需添加

use anyhow::Result;
use sqlez::connection::Connection;
use std::path::Path;
use std::sync::{Arc, Mutex};

use crate::models::*;

/// Main database connection pool
pub struct Database {
    pub(crate) connection: Arc<Mutex<Connection>>,
}

impl Database {
    /// Open or create database at the given path
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let conn = Connection::open_file(path.as_ref().to_string_lossy().as_ref());
        
        conn.exec("PRAGMA foreign_keys = ON;")?()
            .map_err(|e| anyhow::anyhow!("Failed to enable foreign keys: {}", e))?;
        conn.exec("PRAGMA journal_mode = WAL;")?()
            .map_err(|e| anyhow::anyhow!("Failed to set WAL mode: {}", e))?;
        
        Ok(Self {
            connection: Arc::new(Mutex::new(conn)),
        })
    }
    
    pub fn default_path() -> Result<std::path::PathBuf> {
        if let Ok(db_path) = std::env::var("UAVRED_DB_PATH") {
            return Ok(db_path.into());
        }
        let local_db = std::path::PathBuf::from("database/uavred.db");
        if local_db.exists() {
            return Ok(local_db);
        }
        let data_dir = dirs::data_dir()
            .ok_or_else(|| anyhow::anyhow!("Cannot get data directory"))?
            .join("uavred");
        std::fs::create_dir_all(&data_dir)?;
        Ok(data_dir.join("uavred.db"))
    }
    
    pub fn local_path() -> std::path::PathBuf {
        std::path::PathBuf::from("database/uavred.db")
    }
    
    pub fn open_default() -> Result<Self> {
        let path = Self::default_path()?;
        Self::open(path)
    }
    
    pub fn open_local() -> Result<Self> {
        let path = Self::local_path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        Self::open(path)
    }
    
    pub fn initialize_schema(&self) -> Result<()> {
        let conn = self.connection.lock().unwrap();
        let schema = include_str!("../../../database/schema.sql");
        conn.exec(schema)?()
            .map_err(|e| anyhow::anyhow!("Failed to initialize schema: {}", e))?;
        Ok(())
    }
    
    pub fn tasks(&self) -> TaskRepository {
        TaskRepository::new(self.connection.clone())
    }
}

pub struct TaskRepository {
    connection: Arc<Mutex<Connection>>,
}

impl TaskRepository {
    pub fn new(connection: Arc<Mutex<Connection>>) -> Self {
        Self { connection }
    }
    
    pub fn create(&self, task: &Task) -> Result<i64> {
        let conn = self.connection.lock().unwrap();
        let sql = r#"
            INSERT INTO tasks (title, description, mission_objective, task_type, priority, status, 
                              assignee, estimated_minutes, source, external_ref, metadata)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
        "#;
        
        conn.exec_bound::<(&str, &str, &str, &str, &str, &str, &str, Option<i64>, &str, &str, String)>(sql)?((
            &task.title,
            &task.description,
            &task.mission_objective,
            &task.task_type,
            task.priority.as_str(),
            task.status.as_str(),
            &task.assignee,
            task.estimated_minutes,
            &task.source,
            &task.external_ref,
            task.metadata.to_string(),
        ))?;
        
        let id: i64 = conn.select::<i64>("SELECT last_insert_rowid()")?()?
            .into_iter().next().unwrap_or(0);
        Ok(id)
    }
    
    pub fn get_by_id(&self, id: i64) -> Result<Option<Task>> {
        let conn = self.connection.lock().unwrap();
        let sql = r#"
            SELECT id, title, description, mission_objective, task_type, priority, status,
                   assignee, estimated_minutes, created_at, updated_at, started_at, 
                   completed_at, closed_at, close_reason, source, external_ref, metadata
            FROM tasks WHERE id = ?1
        "#;
        
        let rows = conn.select_bound::<i64, (i64, String, String, String, String, String, String, 
                                   String, Option<i64>, String, String, Option<String>, 
                                   Option<String>, Option<String>, String, String, String, String)>(sql)?(id)?;
        
        Ok(rows.into_iter().next().map(|row| self.row_to_task(row).ok()).flatten())
    }
    
    pub fn list_by_status(&self, status: TaskStatus) -> Result<Vec<Task>> {
        let conn = self.connection.lock().unwrap();
        let sql = r#"
            SELECT id, title, description, mission_objective, task_type, priority, status,
                   assignee, estimated_minutes, created_at, updated_at, started_at, 
                   completed_at, closed_at, close_reason, source, external_ref, metadata
            FROM tasks WHERE status = ?1
            ORDER BY priority DESC, created_at DESC
        "#;
        
        let rows = conn.select_bound::<&str, (i64, String, String, String, String, String, String, 
                                   String, Option<i64>, String, String, Option<String>, 
                                   Option<String>, Option<String>, String, String, String, String)>(sql)?(status.as_str())?;
        
        rows.into_iter().map(|row| self.row_to_task(row)).collect()
    }
    
    pub fn update(&self, task: &Task) -> Result<()> {
        let conn = self.connection.lock().unwrap();
        let sql = r#"
            UPDATE tasks SET
                title = ?1, description = ?2, mission_objective = ?3, task_type = ?4,
                priority = ?5, status = ?6, assignee = ?7, estimated_minutes = ?8,
                source = ?9, external_ref = ?10, metadata = ?11, updated_at = CURRENT_TIMESTAMP
            WHERE id = ?12
        "#;
        
        conn.exec_bound::<(&str, &str, &str, &str, &str, &str, &str, Option<i64>, &str, &str, String, i64)>(sql)?((
            &task.title, &task.description, &task.mission_objective, &task.task_type,
            task.priority.as_str(), task.status.as_str(), &task.assignee, task.estimated_minutes,
            &task.source, &task.external_ref, task.metadata.to_string(), task.id,
        ))?;
        Ok(())
    }
    
    pub fn delete(&self, id: i64) -> Result<()> {
        let conn = self.connection.lock().unwrap();
        conn.exec_bound::<i64>("DELETE FROM tasks WHERE id = ?1")?(id)?;
        Ok(())
    }
    
    pub fn update_status(&self, id: i64, status: &str) -> Result<()> {
        let conn = self.connection.lock().unwrap();
        let sql = "UPDATE tasks SET status = ?1, updated_at = CURRENT_TIMESTAMP WHERE id = ?2";
        conn.exec_bound::<(&str, i64)>(sql)?((status, id))?;
        Ok(())
    }
    
    fn row_to_task(&self, row: (i64, String, String, String, String, String, String, 
                               String, Option<i64>, String, String, Option<String>, 
                               Option<String>, Option<String>, String, String, String, String)) -> Result<Task> {
        let (id, title, description, mission_objective, task_type, priority_str, status_str,
             assignee, estimated_minutes, created_at_str, updated_at_str, started_at_str,
             completed_at_str, closed_at_str, close_reason, source, external_ref, metadata_str) = row;
        
        Ok(Task {
            id, title, description, mission_objective, task_type,
            priority: TaskPriority::from(priority_str.as_str()),
            status: TaskStatus::from(status_str.as_str()),
            assignee, estimated_minutes,
            created_at: parse_datetime(&created_at_str)?,
            updated_at: parse_datetime(&updated_at_str)?,
            started_at: started_at_str.and_then(|s| parse_datetime(&s).ok()),
            completed_at: completed_at_str.and_then(|s| parse_datetime(&s).ok()),
            closed_at: closed_at_str.and_then(|s| parse_datetime(&s).ok()),
            close_reason, source, external_ref,
            metadata: serde_json::from_str(&metadata_str).unwrap_or_default(),
            labels: Vec::new(), comments: Vec::new(), dependencies: Vec::new(),
        })
    }
}

fn parse_datetime(s: &str) -> Result<chrono::DateTime<chrono::Utc>> {
    chrono::DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&chrono::Utc))
        .or_else(|_| {
            chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S")
                .map(|naive| chrono::DateTime::from_naive_utc_and_offset(naive, chrono::Utc))
        })
        .map_err(|e| anyhow::anyhow!("Failed to parse datetime: {}", e))
}
