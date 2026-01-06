// 数据模型定义

use serde::{Deserialize, Serialize};
use workspace::TaskData as WorkspaceTaskData;

/// 任务状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus {
    Todo,
    InProgress,
    Done,
}

/// 任务数据模型（扩展 workspace::TaskData，添加 status 字段）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskData {
    pub id: usize,
    pub title: String,
    pub task_type: String,
    pub priority: String,
    pub status: TaskStatus,
}

impl TaskData {
    pub fn new(id: usize, title: String, task_type: String, priority: String, status: TaskStatus) -> Self {
        Self {
            id,
            title,
            task_type,
            priority,
            status,
        }
    }

    /// 从 workspace::TaskData 转换
    pub fn from_workspace(task: &WorkspaceTaskData, status: TaskStatus) -> Self {
        Self {
            id: task.id,
            title: task.title.clone(),
            task_type: task.task_type.clone(),
            priority: task.priority.clone(),
            status,
        }
    }

    /// 转换为 workspace::TaskData
    pub fn to_workspace(&self) -> WorkspaceTaskData {
        WorkspaceTaskData::new(
            self.id,
            self.title.clone(),
            self.task_type.clone(),
            self.priority.clone(),
        )
    }
}

/// 漏洞数据模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VulnData {
    pub id: usize,
    pub title: String,
    pub description: String,
    pub severity: String,
    pub cve: Option<String>,
    pub affected: String,
    pub status: String,
}

/// 资产数据模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetData {
    pub id: String,
    pub name: String,
    pub asset_type: String,
    pub status: String,
}
