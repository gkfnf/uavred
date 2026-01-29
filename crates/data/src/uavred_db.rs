// UAVRed Database - Unified Access Layer
// Provides a clean API for all database operations

use std::sync::{Arc, Mutex};
use anyhow::Result;

use crate::models::*;
use crate::repository::*;

/// Main UAVRed database handle - 使用简化版 Repository
pub struct UavredDatabase {
    db: Database,
}

impl UavredDatabase {
    /// Open the database at the default location
    pub fn open_default() -> Result<Self> {
        let db = Database::open_default()?;
        Ok(Self { db })
    }
    
    /// Open or create the local development database
    pub fn open_local() -> Result<Self> {
        let db = Database::open_local()?;
        Ok(Self { db })
    }
    
    /// Open database at a specific path
    pub fn open<P: AsRef<std::path::Path>>(path: P) -> Result<Self> {
        let db = Database::open(path)?;
        Ok(Self { db })
    }
    
    /// Get task repository
    pub fn tasks(&self) -> TaskRepository {
        self.db.tasks()
    }
    
    /// Get raw connection for custom queries
    pub fn connection(&self) -> Arc<Mutex<sqlez::connection::Connection>> {
        self.db.connection.clone()
    }
}

// ============================================
// Dashboard Statistics (简化版)
// ============================================

pub struct DashboardStats {
    pub total_tasks: i64,
    pub tasks_by_status: std::collections::HashMap<String, i64>,
    pub total_assets: i64,
    pub assets_by_status: std::collections::HashMap<String, i64>,
    pub total_findings: i64,
    pub findings_by_severity: std::collections::HashMap<String, i64>,
    pub recent_findings: Vec<Finding>,
}

impl UavredDatabase {
    /// Get dashboard statistics
    pub fn get_dashboard_stats(&self) -> Result<DashboardStats> {
        let conn_arc = self.connection();
        let conn = conn_arc.lock().unwrap();
        
        // Task counts
        let total_tasks: i64 = conn.select::<i64>("SELECT COUNT(*) FROM tasks")?()?
            .into_iter().next().unwrap_or(0);
        
        let mut tasks_by_status = std::collections::HashMap::new();
        let task_rows = conn.select::<(String, i64)>(
            "SELECT status, COUNT(*) FROM tasks GROUP BY status"
        )?()?;
        for (status, count) in task_rows {
            tasks_by_status.insert(status, count);
        }
        
        // TODO: 添加其他表的统计
        
        Ok(DashboardStats {
            total_tasks,
            tasks_by_status,
            total_assets: 0,
            assets_by_status: std::collections::HashMap::new(),
            total_findings: 0,
            findings_by_severity: std::collections::HashMap::new(),
            recent_findings: Vec::new(),
        })
    }
}

// ============================================
// Predefined Queries (简化版)
// ============================================

impl UavredDatabase {
    /// Get ready tasks (not blocked by dependencies)
    pub fn get_ready_tasks(&self) -> Result<Vec<Task>> {
        // 简化实现：返回所有 Todo 状态的任务
        self.tasks().list_by_status(TaskStatus::Todo)
    }
    
    /// Get blocked tasks with their blockers
    pub fn get_blocked_tasks(&self) -> Result<Vec<(Task, Vec<String>)>> {
        // 简化实现：返回空列表
        Ok(Vec::new())
    }
}
