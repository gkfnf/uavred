//! Sandbox Backend Implementations
//!
//! Provides multiple backend implementations for sandbox isolation:
//!
//! - **BoxliteBackend**: MicroVM isolation using libkrun (highest security)
//! - **DockerBackend**: Container isolation using Docker (good security, widely available)
//! - **ProcessBackend**: Process-level isolation (fallback, minimal security)

use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;

use crate::sandbox::{ExecutionResult, ExitStatus, OutputChunk, ResourceUsage};
use crate::sandbox::traits::{IsolationLevel};

pub mod boxlite;
pub mod docker;
pub mod process;

pub use boxlite::BoxliteBackend;
pub use docker::DockerBackend;
pub use process::ProcessBackend;

/// Common utilities for backend implementations
pub(crate) mod utils {
    use std::process::Stdio;
    use tokio::process::Command;

    /// Create a command with proper stdio configuration
    pub fn create_command(program: &str) -> Command {
        let mut cmd = Command::new(program);
        cmd.stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        cmd
    }

    /// Parse exit status from process output
    pub fn parse_exit_status(code: Option<i32>) -> crate::sandbox::ExitStatus {
        match code {
            Some(code) => crate::sandbox::ExitStatus {
                exit_code: code,
                signal: None,
                error_message: None,
            },
            None => crate::sandbox::ExitStatus {
                exit_code: -1,
                signal: Some(9),
                error_message: Some("Process terminated by signal".to_string()),
            },
        }
    }

    /// Stream output from a reader to a channel
    pub async fn stream_output<R>(
        mut reader: R,
        stream_type: crate::sandbox::StreamType,
        tx: tokio::sync::mpsc::UnboundedSender<crate::sandbox::OutputChunk>,
    ) where
        R: tokio::io::AsyncRead + Unpin,
    {
        use tokio::io::AsyncReadExt;

        let mut buffer = vec![0u8; 4096];
        loop {
            match reader.read(&mut buffer).await {
                Ok(0) => break,
                Ok(n) => {
                    let chunk = crate::sandbox::OutputChunk {
                        stream: stream_type,
                        data: buffer[..n].to_vec(),
                        timestamp: std::time::Instant::now(),
                    };
                    if tx.send(chunk).is_err() {
                        break;
                    }
                }
                Err(_) => break,
            }
        }
    }
}

/// Backend capability information
#[derive(Debug, Clone)]
pub struct BackendCapabilities {
    pub isolation_level: IsolationLevel,
    pub supports_network_isolation: bool,
    pub supports_resource_limits: bool,
    pub supports_file_copy: bool,
    pub supports_image_pull: bool,
    pub supports_snapshot: bool,
}

/// Backend health status
#[derive(Debug, Clone)]
pub enum BackendHealth {
    Healthy,
    Degraded(String),
    Unhealthy(String),
}

/// Base trait for backend implementations
#[async_trait]
pub trait Backend: Send + Sync {
    /// Get backend capabilities
    fn capabilities(&self) -> BackendCapabilities;

    /// Check backend health
    async fn health_check(&self) -> BackendHealth;

    /// Get backend version
    async fn version(&self) -> Result<String>;
}

/// Helper struct to wrap a generic execution result
pub struct ExecutionOutput {
    pub exit_status: ExitStatus,
    pub stdout: String,
    pub stderr: String,
    pub resource_usage: ResourceUsage,
}

impl ExecutionOutput {
    pub fn into_result(self, task_id: crate::sandbox::TaskId) -> ExecutionResult {
        ExecutionResult {
            task_id,
            exit_status: self.exit_status,
            stdout: self.stdout,
            stderr: self.stderr,
            duration: std::time::Duration::from_secs(0),
            resource_usage: self.resource_usage,
        }
    }
}
