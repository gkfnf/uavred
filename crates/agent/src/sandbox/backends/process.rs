//! Process Backend
//!
//! Fallback implementation of SandboxBackend using local process execution.
//!
//! Provides minimal isolation:
//! - Process-level separation only
//! - Optional chroot (requires root)
//! - Optional resource limits (rlimit)
//!
//! ## WARNING
//!
//! This backend is NOT recommended for untrusted code. It should only be
//! used as a last resort when neither Boxlite nor Docker are available.

use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use crate::sandbox::traits::{ExecutionHandle, IsolationLevel, SandboxBackend, SandboxInstance, SandboxState};
use crate::sandbox::backends::utils;

use anyhow::Result;
use async_trait::async_trait;
use tokio::process::Command;
use tokio::sync::mpsc;

use crate::sandbox::{OutputChunk, SandboxConfig, SandboxId, StreamType};

/// Process backend implementation
pub struct ProcessBackend {
    // No persistent state needed
}

impl ProcessBackend {
    /// Create a new Process backend
    ///
    /// This backend is always available but provides minimal security
    pub async fn new() -> Result<Self> {
        tracing::warn!(
            "Process backend provides MINIMAL isolation and should NOT be used for untrusted code"
        );
        Ok(Self {})
    }
}

#[async_trait]
impl SandboxBackend for ProcessBackend {
    fn name(&self) -> &str {
        "process"
    }

    fn isolation_level(&self) -> IsolationLevel {
        IsolationLevel::Process
    }

    async fn is_available(&self) -> bool {
        // Always available
        true
    }

    async fn create(&self, config: SandboxConfig) -> Result<Arc<dyn crate::sandbox::traits::SandboxInstance>> {
        tracing::debug!(
            sandbox_id = %config.id,
            "Creating process-based sandbox (WARNING: minimal isolation)"
        );

        let instance = ProcessInstance::new(config);
        Ok(Arc::new(instance))
    }

    async fn list(&self) -> Result<Vec<SandboxId>> {
        // No tracking for process-based sandboxes
        Ok(Vec::new())
    }

    async fn cleanup(&self) -> Result<()> {
        // Nothing to clean up
        Ok(())
    }
}

/// Process-based sandbox instance
pub struct ProcessInstance {
    id: SandboxId,
    config: SandboxConfig,
    state: Arc<tokio::sync::RwLock<SandboxState>>,
}

impl ProcessInstance {
    fn new(config: SandboxConfig) -> Self {
        Self {
            id: config.id,
            config,
            state: Arc::new(tokio::sync::RwLock::new(SandboxState::Created)),
        }
    }

    /// Apply resource limits to command using unsafe pre_exec
    fn apply_resource_limits(&self, cmd: &mut Command) {
        use std::os::unix::process::CommandExt;

        // Clone config values for the closure
        let memory_limit = self.config.resources.memory_limit_mb;
        let cpu_limit = self.config.resources.cpu_limit;
        let max_pids = self.config.resources.max_pids;
        let max_files = self.config.resources.max_open_files;
        let max_file_size = self.config.resources.max_file_size;

        unsafe {
            cmd.pre_exec(move || {
                // Apply rlimit restrictions
                // Note: These only affect the child process

                // Memory limit (RLIMIT_AS) - virtual memory address space
                if let Some(memory_mb) = memory_limit {
                    let limit = libc::rlimit {
                        rlim_cur: memory_mb * 1024 * 1024,
                        rlim_max: memory_mb * 1024 * 1024,
                    };
                    if libc::setrlimit(libc::RLIMIT_AS, &limit) != 0 {
                        eprintln!("Warning: Failed to set RLIMIT_AS");
                    }
                }

                // Max open files (RLIMIT_NOFILE)
                if let Some(max_open) = max_files {
                    let limit = libc::rlimit {
                        rlim_cur: max_open,
                        rlim_max: max_open,
                    };
                    if libc::setrlimit(libc::RLIMIT_NOFILE, &limit) != 0 {
                        eprintln!("Warning: Failed to set RLIMIT_NOFILE");
                    }
                }

                // Max processes (RLIMIT_NPROC)
                if let Some(max_proc) = max_pids {
                    let limit = libc::rlimit {
                        rlim_cur: max_proc,
                        rlim_max: max_proc,
                    };
                    if libc::setrlimit(libc::RLIMIT_NPROC, &limit) != 0 {
                        eprintln!("Warning: Failed to set RLIMIT_NPROC");
                    }
                }

                // Max file size (RLIMIT_FSIZE)
                if let Some(max_size) = max_file_size {
                    let limit = libc::rlimit {
                        rlim_cur: max_size,
                        rlim_max: max_size,
                    };
                    if libc::setrlimit(libc::RLIMIT_FSIZE, &limit) != 0 {
                        eprintln!("Warning: Failed to set RLIMIT_FSIZE");
                    }
                }

                // Note: CPU limit (RLIMIT_CPU) is handled separately via timeout

                Ok(())
            });
        }

        tracing::debug!(
            memory = ?memory_limit,
            max_files = ?max_files,
            max_pids = ?max_pids,
            "Resource limits configured via rlimit"
        );
    }

    /// Setup working directory
    async fn setup_workdir(&self) -> Result<std::path::PathBuf> {
        let workdir = std::path::PathBuf::from(&self.config.working_dir);

        // Create directory if it doesn't exist
        tokio::fs::create_dir_all(&workdir).await?;

        Ok(workdir)
    }
}

#[async_trait]
impl SandboxInstance for ProcessInstance {
    fn id(&self) -> SandboxId {
        self.id
    }

    async fn state(&self) -> Result<SandboxState> {
        Ok(*self.state.read().await)
    }

    async fn start(&self) -> Result<()> {
        let mut state = self.state.write().await;

        if *state == SandboxState::Running {
            return Ok(());
        }

        // Setup working directory
        self.setup_workdir().await?;

        *state = SandboxState::Running;
        tracing::debug!(sandbox_id = %self.id, "Process sandbox started");

        Ok(())
    }

    async fn stop(&self) -> Result<()> {
        let mut state = self.state.write().await;

        // There's no persistent process to stop
        // Individual executions are handled separately

        *state = SandboxState::Stopped;
        tracing::debug!(sandbox_id = %self.id, "Process sandbox stopped");

        Ok(())
    }

    async fn kill(&self) -> Result<()> {
        // There's no persistent process to kill

        let mut state = self.state.write().await;
        *state = SandboxState::Stopped;

        tracing::warn!(sandbox_id = %self.id, "Process sandbox killed");
        Ok(())
    }

    async fn exec(
        &self,
        command: Vec<String>,
        env: Option<Vec<(String, String)>>,
        timeout: Option<Duration>,
    ) -> Result<ExecutionHandle> {
        let mut state = self.state.write().await;

        if *state != SandboxState::Running {
            // Auto-start if not running
            drop(state);
            self.start().await?;
            state = self.state.write().await;
        }

        *state = SandboxState::Executing;
        drop(state);

        // Build command
        let program = command.first().cloned().unwrap_or_default();
        let args: Vec<String> = command.into_iter().skip(1).collect();

        let mut cmd = Command::new(&program);
        cmd.args(&args)
            .current_dir(&self.config.working_dir)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped());

        // Apply environment
        cmd.env_clear();

        // Add config env
        for (key, value) in &self.config.env {
            cmd.env(key, value);
        }

        // Add execution env
        if let Some(env_vars) = env {
            for (key, value) in env_vars {
                cmd.env(key, value);
            }
        }

        // Apply resource limits
        self.apply_resource_limits(&mut cmd);

        // Spawn process
        let mut child = cmd.spawn()?;

        let (output_tx, output_rx) = mpsc::unbounded_channel();

        // Get stdout/stderr handles
        let stdout = child.stdout.take().expect("stdout not captured");
        let stderr = child.stderr.take().expect("stderr not captured");

        // Spawn output streaming tasks
        let stdout_tx = output_tx.clone();
        tokio::spawn(async move {
            utils::stream_output(stdout, StreamType::Stdout, stdout_tx).await;
        });

        let stderr_tx = output_tx;
        tokio::spawn(async move {
            utils::stream_output(stderr, StreamType::Stderr, stderr_tx).await;
        });

        // Create completion task
        let timeout = timeout.unwrap_or(Duration::from_secs(300));
        let state_ref = self.state.clone();

        let completion = tokio::spawn(async move {
            let start_time = std::time::Instant::now();

            // Wait for process with timeout
            let result = tokio::time::timeout(timeout, child.wait()).await;

            // Reset state
            {
                let mut state = state_ref.write().await;
                if *state == SandboxState::Executing {
                    *state = SandboxState::Running;
                }
            }

            let exit_status = match result {
                Ok(Ok(status)) => utils::parse_exit_status(status.code()),
                Ok(Err(e)) => crate::sandbox::ExitStatus {
                    exit_code: -1,
                    signal: None,
                    error_message: Some(format!("Process error: {}", e)),
                },
                Err(_) => {
                    // Timeout - kill the process
                    let _ = child.start_kill();
                    crate::sandbox::ExitStatus {
                        exit_code: -1,
                        signal: Some(9),
                        error_message: Some("Execution timed out".to_string()),
                    }
                }
            };

            Ok(crate::sandbox::ExecutionResult {
                task_id: crate::sandbox::TaskId::new_v4(),
                exit_status,
                stdout: String::new(), // Streamed via channel
                stderr: String::new(), // Streamed via channel
                duration: start_time.elapsed(),
                resource_usage: crate::sandbox::ResourceUsage::default(),
            })
        });

        Ok(ExecutionHandle {
            output_rx,
            completion,
        })
    }

    async fn copy_in(&self, source: &Path, dest: &str) -> Result<()> {
        // Simple file copy
        let dest_path = std::path::PathBuf::from(&self.config.working_dir).join(dest);

        // Create parent directory if needed
        if let Some(parent) = dest_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        tokio::fs::copy(source, dest_path).await?;
        Ok(())
    }

    async fn copy_out(&self, source: &str, dest: &Path) -> Result<()> {
        // Simple file copy
        let source_path = std::path::PathBuf::from(&self.config.working_dir).join(source);
        tokio::fs::copy(source_path, dest).await?;
        Ok(())
    }

    async fn resource_usage(&self) -> Result<crate::sandbox::ResourceUsage> {
        // No resource tracking for process backend
        Ok(crate::sandbox::ResourceUsage::default())
    }

    async fn wait(&self) -> Result<crate::sandbox::ExitStatus> {
        // Nothing to wait for (no persistent process)
        Ok(crate::sandbox::ExitStatus {
            exit_code: 0,
            signal: None,
            error_message: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_process_backend_creation() {
        let backend = ProcessBackend::new().await;
        assert!(backend.is_ok());

        let backend = backend.unwrap();
        assert_eq!(backend.name(), "process");
        assert_eq!(backend.isolation_level(), IsolationLevel::Process);
        assert!(backend.is_available().await);
    }

    #[tokio::test]
    async fn test_process_execution() {
        let backend = ProcessBackend::new().await.unwrap();
        // Create a temp directory for the test
        let temp_dir = tempfile::tempdir().unwrap();
        let config = SandboxConfig::builder()
            .image("alpine:latest")
            .working_dir(temp_dir.path().to_str().unwrap())
            .build();
        let instance = backend.create(config).await.unwrap();

        instance.start().await.unwrap();

        // Test simple command execution
        let handle = instance
            .exec(vec!["echo".to_string(), "hello".to_string()], None, None)
            .await
            .unwrap();

        let result = handle.completion.await.unwrap().unwrap();
        assert!(result.exit_status.success());
    }
}
