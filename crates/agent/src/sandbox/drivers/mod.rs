//! Execution Drivers
//!
//! Drivers implement different execution strategies for agent tasks:
//!
//! - **CliDriver**: Direct command execution
//! - **McpDriver**: Model Context Protocol execution (for Claude Code)
//! - **MetaToolDriver**: Meta-tooling Python code execution

use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;

use crate::sandbox::{ExecutionResult, SandboxInstance, TaskSpec};

pub mod cli;
pub mod mcp;
pub mod meta_tool;

pub use cli::CliDriver;
pub use mcp::McpDriver;
pub use meta_tool::MetaToolDriver;

/// Execution driver trait
///
/// Drivers translate task specifications into actual execution
/// within a sandbox instance.
#[async_trait]
pub trait ExecutionDriver: Send + Sync {
    /// Driver name
    fn name(&self) -> &str;

    /// Execute a task in the given sandbox
    async fn execute(
        &self,
        task: &TaskSpec,
        sandbox: &Arc<dyn crate::sandbox::traits::SandboxInstance>,
    ) -> Result<ExecutionResult>;

    /// Check if this driver can handle the task
    fn can_handle(&self, task: &TaskSpec) -> bool;
}

/// Driver registry for selecting appropriate driver
pub struct DriverRegistry {
    drivers: Vec<Arc<dyn ExecutionDriver>>,
}

impl DriverRegistry {
    /// Create a new registry with default drivers
    pub fn new() -> Self {
        let mut drivers: Vec<Arc<dyn ExecutionDriver>> = Vec::new();
        drivers.push(Arc::new(CliDriver::new()));
        drivers.push(Arc::new(McpDriver::new()));
        drivers.push(Arc::new(MetaToolDriver::new()));

        Self { drivers }
    }

    /// Find a driver for the given task
    pub fn find_driver(&self, task: &TaskSpec) -> Option<Arc<dyn ExecutionDriver>> {
        self.drivers
            .iter()
            .find(|d| d.can_handle(task))
            .cloned()
    }

    /// Register a custom driver
    pub fn register(&mut self, driver: Arc<dyn ExecutionDriver>) {
        self.drivers.push(driver);
    }

    /// List available drivers
    pub fn list_drivers(&self) -> Vec<&str> {
        self.drivers.iter().map(|d| d.name()).collect()
    }
}

impl Default for DriverRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Common utilities for drivers
pub(crate) mod utils {
    use crate::sandbox::{ExecutionResult, ExitStatus, ResourceUsage};

    /// Create a successful execution result
    pub fn success_result(
        task_id: crate::sandbox::TaskId,
        stdout: String,
        duration: std::time::Duration,
    ) -> ExecutionResult {
        ExecutionResult {
            task_id,
            exit_status: ExitStatus {
                exit_code: 0,
                signal: None,
                error_message: None,
            },
            stdout,
            stderr: String::new(),
            duration,
            resource_usage: ResourceUsage::default(),
        }
    }

    /// Create a failed execution result
    pub fn error_result(
        task_id: crate::sandbox::TaskId,
        error: &str,
        duration: std::time::Duration,
    ) -> ExecutionResult {
        ExecutionResult {
            task_id,
            exit_status: ExitStatus {
                exit_code: -1,
                signal: None,
                error_message: Some(error.to_string()),
            },
            stdout: String::new(),
            stderr: error.to_string(),
            duration,
            resource_usage: ResourceUsage::default(),
        }
    }
}
