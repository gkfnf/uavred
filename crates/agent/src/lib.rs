pub mod executor;
pub mod sandbox;
pub mod scheduler;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Agent representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub id: Uuid,
    pub name: String,
    pub status: AgentStatus,
    pub capabilities: Vec<AgentCapability>,
}

/// Agent status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AgentStatus {
    Idle,
    Running,
    Paused,
    Completed,
    Failed,
}

/// Agent capabilities
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

/// Re-export sandbox types for convenience
pub use sandbox::{
    SandboxConfig, SandboxConfigBuilder, SandboxScheduler, TaskSpec, TaskResult, TaskStatus,
    TaskType, ResourceLimits, NetworkPolicy, SecurityOptions,
    ExecutionHandle, SandboxState,
};

/// Re-export backend types
pub use sandbox::backends::{BoxliteBackend, DockerBackend, ProcessBackend};

/// Re-export driver types
pub use sandbox::drivers::{CliDriver, McpDriver, MetaToolDriver};
