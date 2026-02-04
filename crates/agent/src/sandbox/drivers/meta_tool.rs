//! Meta-Tool Driver
//!
//! Meta-tooling execution driver for Python-based code execution.
//!
//! Inspired by TinyCTFer's approach: Agent intent → Python code → Execute → Result
//!
//! This driver enables AI agents to generate and execute Python code within
//! the sandbox, with access to a curated set of tools and libraries.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;

use anyhow::Result;
use async_trait::async_trait;

use crate::sandbox::{ExecutionResult, TaskSpec};
use crate::sandbox::drivers::ExecutionDriver;
use crate::sandbox::drivers::utils::error_result;
use crate::sandbox::traits::SandboxInstance;

/// Meta-tooling execution driver
///
/// Enables execution of Python code with a pre-configured toolset.
/// The agent generates Python code to accomplish tasks, which is then
/// executed safely within the sandbox.
pub struct MetaToolDriver;

impl MetaToolDriver {
    /// Create a new meta-tool driver
    pub fn new() -> Self {
        Self
    }

    /// Generate Python wrapper code for the task
    fn generate_python_wrapper(&self, task_code: &str) -> String {
        format!(
            r#"#!/usr/bin/env python3
# Auto-generated meta-tooling wrapper

import sys
import json
import os

# Add toolset to path
sys.path.insert(0, '/opt/toolset')

# Import available tools
try:
    from toolset.browser import Browser
    from toolset.terminal import Terminal
    from toolset.note import Note
    from toolset.proxy import Proxy
except ImportError as e:
    print(f"Warning: Could not import toolset: {{e}}", file=sys.stderr)

# Task execution context
class TaskContext:
    def __init__(self):
        self.results = {{}}
        self.notes = []
    
    def log(self, message):
        print(f"[LOG] {{message}}", file=sys.stderr)
    
    def save_result(self, key, value):
        self.results[key] = value
    
    def add_note(self, note):
        self.notes.append(note)

# Initialize context
ctx = TaskContext()

# Execute agent code
try:
{}
except Exception as e:
    print(f"[ERROR] {{e}}", file=sys.stderr)
    import traceback
    traceback.print_exc()
    sys.exit(1)

# Output results
if ctx.results:
    print("\n--- RESULTS ---")
    print(json.dumps(ctx.results, indent=2))
"#,
            // Indent the task code
            task_code.lines()
                .map(|line| format!("    {}", line))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }

    /// Build the execution command
    fn build_command(&self, script_path: &str) -> Vec<String> {
        vec![
            "python3".to_string(),
            script_path.to_string(),
        ]
    }

    /// Write task code to file in sandbox
    async fn write_task_file(
        &self,
        sandbox: &Arc<dyn SandboxInstance>,
        task_code: &str,
    ) -> Result<String> {
        let script_content = self.generate_python_wrapper(task_code);
        let script_path = "/tmp/task_script.py";

        // Write to temporary location first
        let temp_file = tempfile::NamedTempFile::new()?;
        tokio::fs::write(temp_file.path(), script_content).await?;

        // Copy into sandbox
        sandbox.copy_in(temp_file.path(), script_path).await?;

        Ok(script_path.to_string())
    }
}

impl Default for MetaToolDriver {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ExecutionDriver for MetaToolDriver {
    fn name(&self) -> &str {
        "meta_tool"
    }

    fn can_handle(&self, task: &TaskSpec) -> bool {
        // Handle meta-tool type tasks
        matches!(task.task_type, crate::sandbox::scheduler::TaskType::MetaTool)
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
            "Executing meta-tool task"
        );

        // The task command is the Python code to execute
        let task_code = if task.command.is_empty() {
            return Ok(error_result(
                task.id,
                "No Python code provided for meta-tool execution",
                start_time.elapsed(),
            ));
        } else {
            task.command.join("\n")
        };

        // Write task file to sandbox
        let script_path = match self.write_task_file(sandbox, &task_code).await {
            Ok(path) => path,
            Err(e) => {
                return Ok(error_result(
                    task.id,
                    &format!("Failed to write task file: {}", e),
                    start_time.elapsed(),
                ));
            }
        };

        // Build and execute command
        let command = self.build_command(&script_path);

        tracing::debug!(command = ?command, "Meta-tool command built");

        let env: Vec<(String, String)> = task.env.iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

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
            Ok(Ok(r)) => r,
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
            "Meta-tool task completed"
        );

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sandbox::scheduler::TaskType;

    #[test]
    fn test_meta_tool_driver_identification() {
        let driver = MetaToolDriver::new();
        assert_eq!(driver.name(), "meta_tool");

        let meta_task = TaskSpec::new("test").with_type(TaskType::MetaTool);
        let cli_task = TaskSpec::new("test").with_type(TaskType::Cli);

        assert!(driver.can_handle(&meta_task));
        assert!(!driver.can_handle(&cli_task));
    }

    #[test]
    fn test_python_wrapper_generation() {
        let driver = MetaToolDriver::new();
        let code = "print('Hello')\nctx.save_result('greeting', 'Hello World')";

        let wrapper = driver.generate_python_wrapper(code);

        assert!(wrapper.contains("#!/usr/bin/env python3"));
        assert!(wrapper.contains("from toolset.browser import Browser"));
        assert!(wrapper.contains("class TaskContext"));
        assert!(wrapper.contains("print('Hello')"));
    }

    #[test]
    fn test_command_building() {
        let driver = MetaToolDriver::new();
        let command = driver.build_command("/tmp/task.py");

        assert_eq!(command, vec!["python3", "/tmp/task.py"]);
    }
}
