//! CLI Driver
//!
//! Direct command execution driver. Executes tasks as shell commands
//! within the sandbox.

use std::sync::Arc;
use std::time::Instant;

use anyhow::Result;
use async_trait::async_trait;

use crate::sandbox::{ExecutionResult, TaskSpec};
use crate::sandbox::drivers::ExecutionDriver;
use crate::sandbox::traits::SandboxInstance;

/// CLI execution driver
pub struct CliDriver;

impl CliDriver {
    /// Create a new CLI driver
    pub fn new() -> Self {
        Self
    }
}

impl Default for CliDriver {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ExecutionDriver for CliDriver {
    fn name(&self) -> &str {
        "cli"
    }

    fn can_handle(&self, task: &TaskSpec) -> bool {
        // Can handle any task with a command
        !task.command.is_empty()
    }

    async fn execute(
        &self,
        task: &TaskSpec,
        sandbox: &Arc<dyn SandboxInstance>,
    ) -> Result<ExecutionResult> {
        let start_time = Instant::now();

        tracing::info!(
            task_id = %task.id,
            command = ?task.command,
            "Executing CLI task"
        );

        // Convert env map to vec
        let env: Vec<(String, String)> = task.env.iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

        // Execute in sandbox using trait method
        let handle = sandbox
            .exec(task.command.clone(), Some(env), Some(task.timeout))
            .await?;

        // Wait for completion
        let mut result = handle.completion.await??;
        result.duration = start_time.elapsed();

        tracing::info!(
            task_id = %task.id,
            exit_code = result.exit_status.exit_code,
            duration = ?result.duration,
            "CLI task completed"
        );

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sandbox::backends::ProcessBackend;
    use crate::sandbox::traits::SandboxBackend;
    use crate::sandbox::SandboxConfig;

    #[tokio::test]
    async fn test_cli_driver() {
        let driver = CliDriver::new();
        assert_eq!(driver.name(), "cli");

        // Create a test task
        let task = TaskSpec::new("test")
            .with_command(vec!["echo".to_string(), "hello".to_string()]);

        assert!(driver.can_handle(&task));

        // Create sandbox and execute with temp directory
        let backend = ProcessBackend::new().await.unwrap();
        let temp_dir = tempfile::tempdir().unwrap();
        let config = SandboxConfig::builder()
            .image("alpine:latest")
            .working_dir(temp_dir.path().to_str().unwrap())
            .build();
        let sandbox = backend.create(config).await.unwrap();
        sandbox.start().await.unwrap();

        let result = driver.execute(&task, &sandbox).await.unwrap();
        assert!(result.exit_status.success());
    }
}
