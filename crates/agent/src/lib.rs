pub mod scheduler;
pub mod executor;

use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub id: Uuid,
    pub name: String,
    pub status: AgentStatus,
    pub capabilities: Vec<AgentCapability>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AgentStatus {
    Idle,
    Running,
    Paused,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgentCapability {
    NetworkScan,
    ProtocolAnalysis,
    FirmwareAnalysis,
    ExploitExecution,
}

impl Agent {
    pub fn new(name: String, capabilities: Vec<AgentCapability>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            status: AgentStatus::Idle,
            capabilities,
        }
    }
}
