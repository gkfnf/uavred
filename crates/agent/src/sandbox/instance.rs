//! Sandbox Instance
//!
//! Manages the lifecycle of a single sandbox instance including
//! state transitions, execution, and resource tracking.

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

use anyhow::Result;
use async_trait::async_trait;
use tokio::sync::{mpsc, Mutex, RwLock};
use uuid::Uuid;

use crate::sandbox::{
    ExecutionResult, ExitStatus, OutputChunk, ResourceUsage,
    SandboxConfig, SandboxId, StreamType, TaskId,
};
use crate::sandbox::traits::{ExecutionHandle, SandboxState};
use crate::sandbox::traits::SandboxInstance as SandboxInstanceTrait;

/// Handle to a running sandbox instance
pub struct SandboxInstance {
    id: SandboxId,
    config: SandboxConfig,
    state: RwLock<SandboxState>,
    created_at: Instant,
    started_at: RwLock<Option<Instant>>,
    task_id: RwLock<Option<TaskId>>,

    // Resource tracking
    cpu_time_ms: AtomicU64,
    memory_peak_mb: AtomicU64,

    // Backend-specific inner implementation
    inner: Arc<dyn SandboxInstanceTrait>,
}

impl SandboxInstance {
    /// Create a new sandbox instance
    pub fn new(id: SandboxId, config: SandboxConfig, inner: Arc<dyn SandboxInstanceTrait>) -> Arc<Self> {
        Arc::new(Self {
            id,
            config,
            state: RwLock::new(SandboxState::Created),
            created_at: Instant::now(),
            started_at: RwLock::new(None),
            task_id: RwLock::new(None),
            cpu_time_ms: AtomicU64::new(0),
            memory_peak_mb: AtomicU64::new(0),
            inner,
        })
    }

    /// Get the sandbox ID
    pub fn id(&self) -> SandboxId {
        self.id
    }

    /// Get the configuration
    pub fn config(&self) -> &SandboxConfig {
        &self.config
    }

    /// Get the current state
    pub async fn state(&self) -> SandboxState {
        *self.state.read().await
    }

    /// Get creation time
    pub fn created_at(&self) -> Instant {
        self.created_at
    }

    /// Get start time (if started)
    pub async fn started_at(&self) -> Option<Instant> {
        *self.started_at.read().await
    }

    /// Get uptime
    pub async fn uptime(&self) -> Option<Duration> {
        self.started_at().await.map(|start| start.elapsed())
    }

    /// Get assigned task ID
    pub async fn task_id(&self) -> Option<TaskId> {
        *self.task_id.read().await
    }

    /// Set assigned task ID
    pub async fn set_task_id(&self, task_id: Option<TaskId>) {
        *self.task_id.write().await = task_id;
    }

    /// Start the sandbox
    pub async fn start(self: &Arc<Self>) -> Result<()> {
        let mut state = self.state.write().await;

        match *state {
            SandboxState::Running | SandboxState::Executing => {
                // Already running
                return Ok(());
            }
            SandboxState::Created | SandboxState::Stopped => {
                // Can start
            }
            _ => {
                anyhow::bail!("Cannot start sandbox in {:?} state", *state);
            }
        }

        *state = SandboxState::Starting;
        drop(state);

        // Call backend start
        self.inner.start().await?;

        // Update state
        let mut state = self.state.write().await;
        *state = SandboxState::Running;
        drop(state);

        *self.started_at.write().await = Some(Instant::now());

        tracing::info!(sandbox_id = %self.id, "Sandbox started");
        Ok(())
    }

    /// Stop the sandbox gracefully
    pub async fn stop(self: &Arc<Self>) -> Result<()> {
        let mut state = self.state.write().await;

        match *state {
            SandboxState::Running | SandboxState::Executing => {
                *state = SandboxState::Stopping;
            }
            SandboxState::Stopped | SandboxState::Destroyed => {
                return Ok(());
            }
            _ => {}
        }
        drop(state);

        // Call backend stop
        self.inner.stop().await?;

        let mut state = self.state.write().await;
        *state = SandboxState::Stopped;

        tracing::info!(sandbox_id = %self.id, "Sandbox stopped");
        Ok(())
    }

    /// Kill the sandbox immediately
    pub async fn kill(self: &Arc<Self>) -> Result<()> {
        self.inner.kill().await?;

        let mut state = self.state.write().await;
        *state = SandboxState::Stopped;

        tracing::warn!(sandbox_id = %self.id, "Sandbox killed");
        Ok(())
    }

    /// Execute a command in the sandbox
    pub async fn execute(
        self: &Arc<Self>,
        command: Vec<String>,
        env: Option<Vec<(String, String)>>,
        timeout: Option<Duration>,
    ) -> Result<Execution> {
        // Ensure sandbox is running
        self.start().await?;

        // Set state to executing
        {
            let mut state = self.state.write().await;
            if *state != SandboxState::Running {
                anyhow::bail!("Sandbox not in running state: {:?}", *state);
            }
            *state = SandboxState::Executing;
        }

        let execution_timeout = timeout.unwrap_or(self.config.timeout);

        // Get handle from backend
        let ExecutionHandle { output_rx, completion } =
            self.inner.exec(command.clone(), env, Some(execution_timeout)).await?;

        let instance = Arc::clone(self);
        let execution_id = Uuid::new_v4();

        // Create execution result receiver
        let (result_tx, result_rx) = tokio::sync::oneshot::channel();

        // Spawn monitoring task
        tokio::spawn(async move {
            let start_time = Instant::now();

            // Wait for completion with timeout
            let result = tokio::time::timeout(execution_timeout, completion).await;

            // Update state back to running
            {
                let mut state = instance.state.write().await;
                if *state == SandboxState::Executing {
                    *state = SandboxState::Running;
                }
            }

            let exec_result = match result {
                Ok(Ok(Ok(result))) => result,
                Ok(Ok(Err(e))) => {
                    ExecutionResult {
                        task_id: execution_id,
                        exit_status: ExitStatus {
                            exit_code: -1,
                            signal: None,
                            error_message: Some(format!("Execution error: {}", e)),
                        },
                        stdout: String::new(),
                        stderr: format!("Execution error: {}", e),
                        duration: start_time.elapsed(),
                        resource_usage: ResourceUsage::default(),
                    }
                }
                Ok(Err(e)) => {
                    ExecutionResult {
                        task_id: execution_id,
                        exit_status: ExitStatus {
                            exit_code: -1,
                            signal: None,
                            error_message: Some(format!("Task panicked: {}", e)),
                        },
                        stdout: String::new(),
                        stderr: format!("Task panicked: {}", e),
                        duration: start_time.elapsed(),
                        resource_usage: ResourceUsage::default(),
                    }
                }
                Err(_) => {
                    // Timeout
                    let _ = instance.kill().await;
                    ExecutionResult {
                        task_id: execution_id,
                        exit_status: ExitStatus {
                            exit_code: -1,
                            signal: Some(9), // SIGKILL
                            error_message: Some("Execution timed out".to_string()),
                        },
                        stdout: String::new(),
                        stderr: "Execution timed out".to_string(),
                        duration: execution_timeout,
                        resource_usage: ResourceUsage::default(),
                    }
                }
            };

            let _ = result_tx.send(exec_result);
        });

        Ok(Execution::new(execution_id, output_rx, result_rx))
    }

    /// Copy file into sandbox
    pub async fn copy_in(&self, source: &std::path::Path, dest: &str) -> Result<()> {
        self.inner.copy_in(source, dest).await
    }

    /// Copy file out of sandbox
    pub async fn copy_out(&self, source: &str, dest: &std::path::Path) -> Result<()> {
        self.inner.copy_out(source, dest).await
    }

    /// Get resource usage
    pub async fn resource_usage(&self) -> Result<ResourceUsage> {
        self.inner.resource_usage().await
    }

    /// Update resource metrics (called periodically)
    pub fn update_metrics(&self, cpu_ms: u64, memory_mb: u64) {
        self.cpu_time_ms.fetch_add(cpu_ms, Ordering::Relaxed);

        // Update peak memory
        let current_peak = self.memory_peak_mb.load(Ordering::Relaxed);
        if memory_mb > current_peak {
            self.memory_peak_mb.store(memory_mb, Ordering::Relaxed);
        }
    }

    /// Destroy the sandbox
    pub async fn destroy(self: &Arc<Self>) -> Result<()> {
        self.stop().await.ok();

        let mut state = self.state.write().await;
        *state = SandboxState::Destroyed;

        tracing::info!(sandbox_id = %self.id, "Sandbox destroyed");
        Ok(())
    }
}

/// A running execution in a sandbox
pub struct Execution {
    id: Uuid,
    output_rx: mpsc::UnboundedReceiver<OutputChunk>,
    result_rx: tokio::sync::oneshot::Receiver<ExecutionResult>,
}

impl Execution {
    fn new(
        id: Uuid,
        output_rx: mpsc::UnboundedReceiver<OutputChunk>,
        result_rx: tokio::sync::oneshot::Receiver<ExecutionResult>,
    ) -> Self {
        Self {
            id,
            output_rx,
            result_rx,
        }
    }

    /// Get execution ID
    pub fn id(&self) -> Uuid {
        self.id
    }

    /// Get the next output chunk (non-blocking)
    pub async fn next_output(&mut self) -> Option<OutputChunk> {
        self.output_rx.recv().await
    }

    /// Wait for execution to complete
    pub async fn wait(self) -> Result<ExecutionResult> {
        self.result_rx.await.map_err(|_| anyhow::anyhow!("Result channel closed"))
    }

    /// Stream all output to completion
    pub async fn stream_to_completion(mut self) -> Result<ExecutionResult> {
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();

        // Collect all output first
        while let Some(chunk) = self.next_output().await {
            match chunk.stream {
                StreamType::Stdout => stdout.extend_from_slice(&chunk.data),
                StreamType::Stderr => stderr.extend_from_slice(&chunk.data),
            }
        }

        // Wait for result
        let mut result = self.result_rx.await
            .map_err(|_| anyhow::anyhow!("Result channel closed"))?;

        // Append collected output
        result.stdout = String::from_utf8_lossy(&stdout).to_string();
        result.stderr = String::from_utf8_lossy(&stderr).to_string();

        Ok(result)
    }
}

/// Handle for external sandbox references
#[derive(Clone)]
pub struct SandboxHandle {
    pub id: SandboxId,
    pub inner: Arc<SandboxInstance>,
}

impl SandboxHandle {
    pub fn new(instance: Arc<SandboxInstance>) -> Self {
        Self {
            id: instance.id(),
            inner: instance,
        }
    }
}

/// Current state of a sandbox (simplified for external use)
#[derive(Debug, Clone)]
pub struct SandboxInstanceState {
    pub id: SandboxId,
    pub state: SandboxState,
    pub created_at: Instant,
    pub uptime: Option<Duration>,
    pub task_id: Option<TaskId>,
    pub resource_usage: ResourceUsage,
}

impl SandboxInstanceState {
    pub async fn from_instance(instance: &SandboxInstance) -> Self {
        let resource_usage = instance.resource_usage().await.unwrap_or_default();

        Self {
            id: instance.id(),
            state: instance.state().await,
            created_at: instance.created_at(),
            uptime: instance.uptime().await,
            task_id: instance.task_id().await,
            resource_usage,
        }
    }
}

// Implement the traits::SandboxInstance trait for SandboxInstance
#[async_trait]
impl crate::sandbox::traits::SandboxInstance for SandboxInstance {
    fn id(&self) -> SandboxId {
        self.id
    }

    async fn state(&self) -> Result<SandboxState> {
        Ok(*self.state.read().await)
    }

    async fn start(&self) -> Result<()> {
        self.start().await
    }

    async fn stop(&self) -> Result<()> {
        self.stop().await
    }

    async fn kill(&self) -> Result<()> {
        self.kill().await
    }

    async fn exec(
        &self,
        command: Vec<String>,
        env: Option<Vec<(String, String)>>,
        timeout: Option<Duration>,
    ) -> Result<crate::sandbox::traits::ExecutionHandle> {
        // Execute via inner backend
        self.inner.exec(command, env, timeout).await
    }

    async fn copy_in(&self, source: &std::path::Path, dest: &str) -> Result<()> {
        self.copy_in(source, dest).await
    }

    async fn copy_out(&self, source: &str, dest: &std::path::Path) -> Result<()> {
        self.copy_out(source, dest).await
    }

    async fn resource_usage(&self) -> Result<ResourceUsage> {
        self.resource_usage().await
    }

    async fn wait(&self) -> Result<crate::sandbox::ExitStatus> {
        self.inner.wait().await
    }
}
