use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub name: String,
    pub task_type: TaskType,
    pub status: TaskStatus,
    pub priority: TaskPriority,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskType {
    NetworkScan { target: String },
    ProtocolAnalysis { target: String, protocol: String },
    FirmwareAnalysis { path: String },
    Exploit { target: String, exploit_id: String },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum TaskPriority {
    Low,
    Medium,
    High,
    Critical,
}

impl Task {
    pub fn new(name: String, task_type: TaskType, priority: TaskPriority) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            task_type,
            status: TaskStatus::Pending,
            priority,
            created_at: Utc::now(),
            started_at: None,
            completed_at: None,
        }
    }

    pub fn start(&mut self) {
        self.status = TaskStatus::Running;
        self.started_at = Some(Utc::now());
    }

    pub fn complete(&mut self) {
        self.status = TaskStatus::Completed;
        self.completed_at = Some(Utc::now());
    }

    pub fn fail(&mut self) {
        self.status = TaskStatus::Failed;
        self.completed_at = Some(Utc::now());
    }
}
