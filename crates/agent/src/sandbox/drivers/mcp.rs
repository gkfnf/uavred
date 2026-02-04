//! MCP Driver
//!
//! Model Context Protocol execution driver for Claude Code integration.
//!
//! The MCP driver prepares the sandbox environment for running Claude Code
//! with the appropriate configuration and MCP server setup.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;

use anyhow::Result;
use async_trait::async_trait;

use crate::sandbox::{ExecutionResult, TaskSpec};
use crate::sandbox::drivers::ExecutionDriver;
use crate::sandbox::drivers::utils::error_result;
use crate::sandbox::traits::SandboxInstance;

/// MCP (Model Context Protocol) execution driver
///
/// This driver sets up the environment for Claude Code to run with MCP
/// servers enabled for secure tool execution within the sandbox.
pub struct McpDriver;

impl McpDriver {
    /// Create a new MCP driver
    pub fn new() -> Self {
        Self
    }

    /// Prepare MCP configuration for the sandbox
    async fn prepare_mcp_config(
        &self,
        _sandbox: &Arc<dyn SandboxInstance>,
    ) -> Result<McpConfig> {
        // TODO:
        // 1. Generate MCP server configuration
        // 2. Set up tool permissions
        // 3. Configure allowed tools based on task

        Ok(McpConfig {
            server_url: "http://localhost:8080".to_string(),
            tools_allowed: vec![
                "read_file".to_string(),
                "write_file".to_string(),
                "execute_command".to_string(),
                "search_files".to_string(),
            ],
        })
    }

    /// Build Claude Code command with MCP configuration
    fn build_claude_command(
        &self,
        task: &TaskSpec,
        mcp_config: &McpConfig,
    ) -> Vec<String> {
        let mut command = vec!["claude".to_string()];

        // Add MCP configuration
        command.push("--mcp-config".to_string());
        command.push(mcp_config.server_url.clone());

        // Skip permissions prompt (running in automated mode)
        command.push("--dangerously-skip-permissions".to_string());

        // Add task as prompt
        command.push("--print".to_string());

        // The actual task description
        let task_description = format!(
            "Task: {}\n\n{}",
            task.name,
            task.command.join(" ")
        );
        command.push(task_description);

        command
    }
}

impl Default for McpDriver {
    fn default() -> Self {
        Self::new()
    }
}

/// MCP configuration
#[derive(Debug, Clone)]
struct McpConfig {
    server_url: String,
    tools_allowed: Vec<String>,
}

#[async_trait]
impl ExecutionDriver for McpDriver {
    fn name(&self) -> &str {
        "mcp"
    }

    fn can_handle(&self, task: &TaskSpec) -> bool {
        // Handle MCP-type tasks
        matches!(task.task_type, crate::sandbox::scheduler::TaskType::Mcp)
    }

    async fn execute(
        &self,
        task: &TaskSpec,
        sandbox: &Arc<dyn SandboxInstance>,
    ) -> Result<ExecutionResult> {
        let start_time = Instant::now();

        tracing::info!(
            task_id = %task.id,
            task_name = %task.name,
            "Executing MCP task"
        );

        // Prepare MCP configuration
        let mcp_config = match self.prepare_mcp_config(sandbox).await {
            Ok(config) => config,
            Err(e) => {
                return Ok(error_result(
                    task.id,
                    &format!("Failed to prepare MCP config: {}", e),
                    start_time.elapsed(),
                ));
            }
        };

        // Build Claude Code command
        let command = self.build_claude_command(task, &mcp_config);

        tracing::debug!(command = ?command, "MCP command built");

        // Set up MCP environment variables
        let mut env: Vec<(String, String)> = task.env.iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        env.push(("CLAUDE_MCP_SERVER".to_string(), mcp_config.server_url));
        env.push(("CLAUDE_MCP_TOOLS".to_string(), mcp_config.tools_allowed.join(",")));

        // Execute in sandbox using trait method
        let handle = match sandbox
            .exec(command, Some(env), Some(task.timeout))
            .await
        {
            Ok(h) => h,
            Err(e) => {
                return Ok(error_result(
                    task.id,
                    &format!("Failed to start execution: {}", e),
                    start_time.elapsed(),
                ));
            }
        };

        // Wait for completion
        let mut result = match handle.completion.await {
            Ok(Ok(r)) => {
                r
            }
            Ok(Err(e)) => {
                return Ok(error_result(
                    task.id,
                    &format!("Execution failed: {}", e),
                    start_time.elapsed(),
                ));
            }
            Err(e) => {
                return Ok(error_result(
                    task.id,
                    &format!("Task panicked: {}", e),
                    start_time.elapsed(),
                ));
            }
        };
        result.duration = start_time.elapsed();

        tracing::info!(
            task_id = %task.id,
            exit_code = result.exit_status.exit_code,
            duration = ?result.duration,
            "MCP task completed"
        );

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sandbox::scheduler::TaskType;

    #[test]
    fn test_mcp_driver_identification() {
        let driver = McpDriver::new();
        assert_eq!(driver.name(), "mcp");

        let mcp_task = TaskSpec::new("test").with_type(TaskType::Mcp);
        let cli_task = TaskSpec::new("test").with_type(TaskType::Cli);

        assert!(driver.can_handle(&mcp_task));
        assert!(!driver.can_handle(&cli_task));
    }

    #[test]
    fn test_command_building() {
        let driver = McpDriver::new();
        let task = TaskSpec::new("security-scan")
            .with_type(TaskType::Mcp)
            .with_command(vec!["scan".to_string(), "target.com".to_string()]);

        let mcp_config = McpConfig {
            server_url: "http://localhost:8080".to_string(),
            tools_allowed: vec!["execute_command".to_string()],
        };

        let command = driver.build_claude_command(&task, &mcp_config);

        assert!(command.contains(&"claude".to_string()));
        assert!(command.contains(&"--dangerously-skip-permissions".to_string()));
        assert!(command.contains(&"--print".to_string()));
    }
}
