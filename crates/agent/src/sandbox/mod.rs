//! Agent Sandbox Module
//!
//! Provides isolated execution environments for AI agents using multiple backend implementations.
//!
//! # Architecture
//!
//! ```text
//! sandbox/
//! ├── mod.rs           (public API - this file)
//! ├── traits.rs        (Sandbox, SandboxBackend, TaskExecutor traits)
//! ├── config.rs        (SandboxConfiguration, ResourceLimits, NetworkPolicy)
//! ├── instance.rs      (SandboxInstance - lifecycle management)
//! ├── scheduler.rs     (SandboxScheduler - multi-tenant task scheduling)
//! ├── backends/        (backend implementations)
//! │   ├── mod.rs
//! │   ├── boxlite.rs   (BoxliteBackend - microVM isolation)
//! │   ├── docker.rs    (DockerBackend - container isolation)
//! │   └── process.rs   (ProcessBackend - local process fallback)
//! └── drivers/         (agent execution drivers)
//!     ├── mod.rs
//!     ├── mcp.rs       (MCP protocol driver for Claude Code)
//!     ├── cli.rs       (Direct CLI execution driver)
//!     └── meta_tool.rs (Meta-tooling Python code execution)
//! ```
//!
//! # Design Principles
//!
//! 1. **Backend Abstraction**: Support multiple isolation levels (VM > Container > Process)
//! 2. **Task-Centric**: Each task runs in its own sandbox instance
//! 3. **Resource Control**: CPU, memory, network, and time limits enforced
//! 4. **Async Streaming**: Real-time stdout/stderr streaming during execution
//! 5. **Secure by Default**: Deny network by default, minimal privileges
//!
//! # Usage Example
//!
//! ```rust,no_run
//! use std::sync::Arc;
//! use agent::sandbox::{SandboxScheduler, TaskSpec, SandboxConfig};
//! use agent::sandbox::backends::ProcessBackend;
//!
//! # async fn example() -> anyhow::Result<()> {
//! // Create scheduler with Process backend (fallback)
//! let backend = Arc::new(ProcessBackend::new().await?);
//! let scheduler = SandboxScheduler::new(backend).await?;
//!
//! // Create a task
//! let task = TaskSpec::new("security-scan")
//!     .with_command(vec!["echo".to_string(), "hello".to_string()]);
//!
//! // Execute task in sandbox
//! let result = scheduler.execute(task).await?;
//!
//! println!("Exit code: {}", result.exit_code);
//! println!("Output: {}", result.stdout);
//! # Ok(())
//! # }
//! ```

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
// Re-export async_trait for use in submodules
pub use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc, RwLock};
use uuid::Uuid;

pub mod backends;
pub mod config;
pub mod drivers;
pub mod instance;
pub mod scheduler;
pub mod traits;

pub use config::{NetworkPolicy, ResourceLimits, SandboxConfig, SandboxConfigBuilder, SecurityOptions};
pub use drivers::{ExecutionDriver, McpDriver, CliDriver, MetaToolDriver};
pub use instance::{SandboxInstance, SandboxInstanceState, SandboxHandle};
pub use scheduler::{SandboxScheduler, TaskSpec, TaskResult, TaskStatus, TaskType};
pub use traits::{SandboxBackend, SandboxBackendFactory, ExecutionHandle, SandboxState};

/// Unique identifier for a sandbox instance
pub type SandboxId = Uuid;

/// Unique identifier for an execution task
pub type TaskId = Uuid;

/// Exit status of a sandboxed execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExitStatus {
    pub exit_code: i32,
    pub signal: Option<i32>,
    pub error_message: Option<String>,
}

impl ExitStatus {
    pub fn success(&self) -> bool {
        self.exit_code == 0 && self.signal.is_none()
    }
}

/// Output stream type for sandbox execution
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StreamType {
    Stdout,
    Stderr,
}

/// A chunk of output from sandbox execution
#[derive(Debug, Clone)]
pub struct OutputChunk {
    pub stream: StreamType,
    pub data: Vec<u8>,
    pub timestamp: std::time::Instant,
}

/// Complete execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    pub task_id: TaskId,
    pub exit_status: ExitStatus,
    pub stdout: String,
    pub stderr: String,
    pub duration: Duration,
    pub resource_usage: ResourceUsage,
}

/// Resource usage statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResourceUsage {
    pub cpu_seconds: f64,
    pub memory_peak_mb: u64,
    pub memory_avg_mb: u64,
    pub disk_read_bytes: u64,
    pub disk_write_bytes: u64,
    pub network_rx_bytes: u64,
    pub network_tx_bytes: u64,
}

/// Stream handle for real-time output
pub struct OutputStream {
    pub(crate) rx: mpsc::UnboundedReceiver<OutputChunk>,
}

impl OutputStream {
    pub async fn next(&mut self) -> Option<OutputChunk> {
        self.rx.recv().await
    }
}

/// Factory for creating sandbox backends
pub struct SandboxFactory;

impl SandboxFactory {
    /// Create the best available backend
    pub async fn create_best() -> Result<Arc<dyn SandboxBackend>> {
        // Priority: Boxlite (microVM) > Docker (container) > Process (fallback)
        if let Ok(backend) = backends::BoxliteBackend::new().await {
            tracing::info!("Using Boxlite backend (microVM isolation)");
            return Ok(Arc::new(backend));
        }

        if let Ok(backend) = backends::DockerBackend::new().await {
            tracing::info!("Using Docker backend (container isolation)");
            return Ok(Arc::new(backend));
        }

        tracing::warn!("Using Process backend (minimal isolation - NOT recommended for untrusted code)");
        Ok(Arc::new(backends::ProcessBackend::new().await?))
    }

    /// Create a specific backend by name
    pub async fn create(name: &str) -> Result<Arc<dyn SandboxBackend>> {
        match name {
            "boxlite" => Ok(Arc::new(backends::BoxliteBackend::new().await?)),
            "docker" => Ok(Arc::new(backends::DockerBackend::new().await?)),
            "process" => Ok(Arc::new(backends::ProcessBackend::new().await?)),
            _ => anyhow::bail!("Unknown backend: {}", name),
        }
    }
}

/// Global sandbox registry for tracking active instances
pub struct SandboxRegistry {
    instances: Arc<RwLock<HashMap<SandboxId, Arc<SandboxInstance>>>>,
}

impl SandboxRegistry {
    pub fn new() -> Self {
        Self {
            instances: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn register(&self, instance: Arc<SandboxInstance>) {
        let mut instances = self.instances.write().await;
        instances.insert(instance.id(), instance);
    }

    pub async fn unregister(&self, id: SandboxId) {
        let mut instances = self.instances.write().await;
        instances.remove(&id);
    }

    pub async fn get(&self, id: SandboxId) -> Option<Arc<SandboxInstance>> {
        let instances = self.instances.read().await;
        instances.get(&id).cloned()
    }

    pub async fn list_active(&self) -> Vec<Arc<SandboxInstance>> {
        let instances = self.instances.read().await;
        instances.values().cloned().collect()
    }

    pub async fn shutdown_all(&self) -> Result<()> {
        let instances = self.instances.read().await;
        for instance in instances.values() {
            let _ = instance.stop().await;
        }
        Ok(())
    }
}

impl Default for SandboxRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exit_status_success() {
        assert!(ExitStatus { exit_code: 0, signal: None, error_message: None }.success());
        assert!(!ExitStatus { exit_code: 1, signal: None, error_message: None }.success());
        assert!(!ExitStatus { exit_code: 0, signal: Some(9), error_message: None }.success());
    }

    #[test]
    fn test_sandbox_id_generation() {
        let id1 = Uuid::new_v4();
        let id2 = Uuid::new_v4();
        assert_ne!(id1, id2);
    }
}
