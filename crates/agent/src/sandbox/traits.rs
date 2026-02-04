//! Sandbox Backend Traits
//!
//! Defines the core abstractions that all sandbox backends must implement.
//! This allows the scheduler to work with any backend (Boxlite, Docker, Process)
//! without knowing the implementation details.

use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use tokio::sync::mpsc;

use crate::sandbox::{ExecutionResult, OutputChunk, SandboxConfig, SandboxId};

/// Core trait for sandbox backend implementations
///
/// All backends (Boxlite, Docker, Process) must implement this trait to be
/// usable by the scheduler.
#[async_trait]
pub trait SandboxBackend: Send + Sync {
    /// Backend name for logging and identification
    fn name(&self) -> &str;

    /// Isolation level provided by this backend
    fn isolation_level(&self) -> IsolationLevel;

    /// Check if the backend is available on this system
    async fn is_available(&self) -> bool;

    /// Create a new sandbox instance with the given configuration
    async fn create(&self, config: SandboxConfig) -> Result<Arc<dyn SandboxInstance>>;

    /// List all sandboxes managed by this backend
    async fn list(&self) -> Result<Vec<SandboxId>>;

    /// Clean up backend resources
    async fn cleanup(&self) -> Result<()>;
}

/// A running sandbox instance
#[async_trait]
pub trait SandboxInstance: Send + Sync {
    /// Get the unique ID of this sandbox
    fn id(&self) -> SandboxId;

    /// Get the current state of the sandbox
    async fn state(&self) -> Result<SandboxState>;

    /// Start the sandbox (if not already running)
    async fn start(&self) -> Result<()>;

    /// Stop the sandbox gracefully
    async fn stop(&self) -> Result<()>;

    /// Force kill the sandbox immediately
    async fn kill(&self) -> Result<()>;

    /// Execute a command inside the sandbox
    async fn exec(
        &self,
        command: Vec<String>,
        env: Option<Vec<(String, String)>>,
        timeout: Option<std::time::Duration>,
    ) -> Result<ExecutionHandle>;

    /// Copy file into the sandbox
    async fn copy_in(&self, source: &std::path::Path, dest: &str) -> Result<()>;

    /// Copy file out of the sandbox
    async fn copy_out(&self, source: &str, dest: &std::path::Path) -> Result<()>;

    /// Get resource usage statistics
    async fn resource_usage(&self) -> Result<crate::sandbox::ResourceUsage>;

    /// Wait for the sandbox to exit
    async fn wait(&self) -> Result<crate::sandbox::ExitStatus>;
}

/// Handle to an ongoing execution
pub struct ExecutionHandle {
    /// Channel for receiving real-time output
    pub output_rx: mpsc::UnboundedReceiver<OutputChunk>,

    /// Join handle to await completion
    pub completion: tokio::task::JoinHandle<Result<ExecutionResult>>,
}

/// Factory trait for creating backends
#[async_trait]
pub trait SandboxBackendFactory: Send + Sync {
    /// Backend name
    fn name(&self) -> &str;

    /// Create a new backend instance
    async fn create(&self) -> Result<Arc<dyn SandboxBackend>>;
}

/// Isolation levels from weakest to strongest
/// 
/// Note: Order matters for PartialOrd/Ord - stronger isolation has higher ordinal
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum IsolationLevel {
    /// No isolation (direct execution)
    None,

    /// Process-level isolation (chroot, seccomp)
    Process,

    /// OS-level containers (Docker, containerd)
    Container,

    /// MicroVM (libkrun, Firecracker)
    MicroVM,

    /// Hardware virtualization (KVM, Hyper-V)
    HardwareVirtualization,
}

impl std::fmt::Display for IsolationLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IsolationLevel::HardwareVirtualization => write!(f, "hardware-virtualization"),
            IsolationLevel::MicroVM => write!(f, "microvm"),
            IsolationLevel::Container => write!(f, "container"),
            IsolationLevel::Process => write!(f, "process"),
            IsolationLevel::None => write!(f, "none"),
        }
    }
}

/// Current state of a sandbox instance
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SandboxState {
    /// Created but not started
    Created,

    /// Starting up
    Starting,

    /// Running and ready for commands
    Running,

    /// Executing a command
    Executing,

    /// Stopping gracefully
    Stopping,

    /// Stopped but not cleaned up
    Stopped,

    /// Cleaned up and destroyed
    Destroyed,

    /// Error state
    Error,
}

impl std::fmt::Display for SandboxState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SandboxState::Created => write!(f, "created"),
            SandboxState::Starting => write!(f, "starting"),
            SandboxState::Running => write!(f, "running"),
            SandboxState::Executing => write!(f, "executing"),
            SandboxState::Stopping => write!(f, "stopping"),
            SandboxState::Stopped => write!(f, "stopped"),
            SandboxState::Destroyed => write!(f, "destroyed"),
            SandboxState::Error => write!(f, "error"),
        }
    }
}
