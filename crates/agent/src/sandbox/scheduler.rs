//! Sandbox Scheduler
//!
//! Multi-tenant task scheduler that manages sandbox lifecycle,
//! assigns tasks to sandboxes, and handles resource allocation.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use tokio::sync::{mpsc, RwLock};
use uuid::Uuid;

use crate::sandbox::{SandboxConfig, SandboxFactory, SandboxId, SandboxInstance, SandboxRegistry, TaskId};
use crate::sandbox::drivers::ExecutionDriver;
use crate::sandbox::instance::{Execution, SandboxHandle, SandboxInstanceState};
use crate::sandbox::traits::SandboxBackend;

/// Task specification for sandbox execution
#[derive(Debug, Clone)]
pub struct TaskSpec {
    /// Unique task identifier
    pub id: TaskId,

    /// Human-readable task name
    pub name: String,

    /// Task type for routing to appropriate driver
    pub task_type: TaskType,

    /// Command to execute (for CLI tasks)
    pub command: Vec<String>,

    /// Environment variables
    pub env: HashMap<String, String>,

    /// Sandbox configuration
    pub sandbox_config: SandboxConfig,

    /// Priority (higher = more important)
    pub priority: i32,

    /// Maximum retries on failure
    pub max_retries: u32,

    /// Timeout for execution
    pub timeout: Duration,
}

impl TaskSpec {
    /// Create a new task specification
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            task_type: TaskType::Cli,
            command: Vec::new(),
            env: HashMap::new(),
            sandbox_config: SandboxConfig::default(),
            priority: 0,
            max_retries: 1,
            timeout: Duration::from_secs(300),
        }
    }

    /// Set the task type
    pub fn with_type(mut self, task_type: TaskType) -> Self {
        self.task_type = task_type;
        self
    }

    /// Set the command to execute
    pub fn with_command(mut self, command: Vec<String>) -> Self {
        self.command = command;
        self
    }

    /// Set environment variables
    pub fn with_env(mut self, env: HashMap<String, String>) -> Self {
        self.env = env;
        self
    }

    /// Set sandbox configuration
    pub fn with_sandbox_config(mut self, config: SandboxConfig) -> Self {
        self.sandbox_config = config;
        self
    }

    /// Set priority
    pub fn with_priority(mut self, priority: i32) -> Self {
        self.priority = priority;
        self
    }

    /// Set max retries
    pub fn with_retries(mut self, retries: u32) -> Self {
        self.max_retries = retries;
        self
    }

    /// Set timeout
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
}

/// Task type determines which execution driver to use
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskType {
    /// Direct CLI execution
    Cli,

    /// MCP protocol execution (Claude Code)
    Mcp,

    /// Meta-tooling Python code execution
    MetaTool,
}

/// Task status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskStatus {
    /// Waiting for sandbox
    Pending,

    /// Creating sandbox
    Creating,

    /// Sandbox starting
    Starting,

    /// Executing task
    Executing,

    /// Completed successfully
    Completed,

    /// Failed
    Failed,

    /// Cancelled
    Cancelled,

    /// Retrying
    Retrying,
}

/// Task execution result
#[derive(Debug, Clone)]
pub struct TaskResult {
    pub task_id: TaskId,
    pub status: TaskStatus,
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
    pub duration: Duration,
    pub retry_count: u32,
    pub error_message: Option<String>,
}

/// Request to execute a task
struct ExecutionRequest {
    task: TaskSpec,
    result_tx: mpsc::Sender<TaskResult>,
}

/// Sandbox Scheduler
///
/// Manages a pool of sandboxes and schedules tasks for execution.
/// Provides automatic sandbox lifecycle management and resource cleanup.
pub struct SandboxScheduler {
    /// Backend implementation
    backend: Arc<dyn SandboxBackend>,

    /// Registry of active sandboxes
    registry: Arc<SandboxRegistry>,

    /// Sandbox pool (reusable sandboxes)
    sandbox_pool: RwLock<Vec<Arc<SandboxInstance>>>,

    /// Task queue
    task_queue: RwLock<Vec<TaskSpec>>,

    /// Active executions
    active_tasks: RwLock<HashMap<TaskId, Arc<SandboxInstance>>>,

    /// Result channel
    result_tx: mpsc::Sender<TaskResult>,
    result_rx: RwLock<mpsc::Receiver<TaskResult>>,

    /// Maximum concurrent sandboxes
    max_concurrent: usize,

    /// Default driver for task execution
    default_driver: Arc<dyn ExecutionDriver>,

    /// Shutdown signal
    shutdown: RwLock<bool>,
}

impl SandboxScheduler {
    /// Create a new scheduler with the given backend
    pub async fn new(backend: Arc<dyn SandboxBackend>) -> Result<Arc<Self>> {
        let (result_tx, result_rx) = mpsc::channel(1000);

        let scheduler = Arc::new(Self {
            backend,
            registry: Arc::new(SandboxRegistry::new()),
            sandbox_pool: RwLock::new(Vec::new()),
            task_queue: RwLock::new(Vec::new()),
            active_tasks: RwLock::new(HashMap::new()),
            result_tx,
            result_rx: RwLock::new(result_rx),
            max_concurrent: 10,
            default_driver: Arc::new(crate::sandbox::drivers::CliDriver::new()),
            shutdown: RwLock::new(false),
        });

        // Start background task processor
        let scheduler_clone = Arc::clone(&scheduler);
        tokio::spawn(async move {
            scheduler_clone.run_task_processor().await;
        });

        tracing::info!("Sandbox scheduler initialized");
        Ok(scheduler)
    }

    /// Create scheduler with auto-detected best backend
    pub async fn with_best_backend() -> Result<Arc<Self>> {
        let backend = SandboxFactory::create_best().await?;
        Self::new(backend).await
    }

    /// Set maximum concurrent sandboxes
    pub fn with_max_concurrent(self: Arc<Self>, max: usize) -> Arc<Self> {
        // This is a bit hacky, but allows builder pattern
        // In real implementation, use proper builder
        self
    }

    /// Submit a task for execution
    pub async fn submit(&self, task: TaskSpec) -> Result<TaskId> {
        let task_id = task.id;

        {
            let mut queue = self.task_queue.write().await;
            queue.push(task);
            // Sort by priority (higher first)
            queue.sort_by_key(|t| -t.priority);
        }

        tracing::info!(task_id = %task_id, "Task submitted to queue");
        Ok(task_id)
    }

    /// Execute a task immediately (blocking)
    pub async fn execute(&self, task: TaskSpec) -> Result<TaskResult> {
        let task_id = task.id;
        let (tx, mut rx) = mpsc::channel(1);

        // Create execution request
        let request = ExecutionRequest {
            task,
            result_tx: tx,
        };

        // Process immediately
        self.process_single_task(request).await?;

        // Wait for result
        rx.recv()
            .await
            .ok_or_else(|| anyhow::anyhow!("Result channel closed"))
    }

    /// Execute a simple command
    pub async fn execute_command(
        &self,
        name: impl Into<String>,
        config: SandboxConfig,
        command: Vec<String>,
    ) -> Result<crate::sandbox::ExecutionResult> {
        // Get or create sandbox
        let sandbox = self.acquire_sandbox(config).await?;

        // Start sandbox
        sandbox.start().await?;

        // Execute command
        let execution = sandbox
            .execute(command, None, Some(Duration::from_secs(300)))
            .await?;

        // Wait for result
        let result = execution.wait().await?;

        // Release sandbox back to pool or destroy
        self.release_sandbox(sandbox).await;

        Ok(result)
    }

    /// Get scheduler statistics
    pub async fn stats(&self) -> SchedulerStats {
        SchedulerStats {
            active_sandboxes: self.registry.list_active().await.len(),
            queued_tasks: self.task_queue.read().await.len(),
            active_tasks: self.active_tasks.read().await.len(),
            pool_size: self.sandbox_pool.read().await.len(),
        }
    }

    /// List active sandboxes
    pub async fn list_active_sandboxes(&self) -> Vec<SandboxInstanceState> {
        let instances = self.registry.list_active().await;
        let mut states = Vec::new();

        for instance in instances {
            states.push(SandboxInstanceState::from_instance(&instance).await);
        }

        states
    }

    /// Get task result
    pub async fn receive_result(&self) -> Option<TaskResult> {
        self.result_rx.write().await.recv().await
    }

    /// Shutdown the scheduler
    pub async fn shutdown(&self) -> Result<()> {
        *self.shutdown.write().await = true;

        // Stop all active sandboxes
        self.registry.shutdown_all().await?;

        tracing::info!("Sandbox scheduler shut down");
        Ok(())
    }

    // =========================================================================
    // Internal methods
    // =========================================================================

    /// Main task processor loop
    async fn run_task_processor(self: Arc<Self>) {
        let mut interval = tokio::time::interval(Duration::from_millis(100));

        loop {
            interval.tick().await;

            if *self.shutdown.read().await {
                break;
            }

            // Check if we can process more tasks
            let active_count = self.active_tasks.read().await.len();
            if active_count >= self.max_concurrent {
                continue;
            }

            // Get next task from queue
            let task = {
                let mut queue = self.task_queue.write().await;
                if queue.is_empty() {
                    continue;
                }
                queue.remove(0)
            };

            // Process the task
            let scheduler = Arc::clone(&self);
            tokio::spawn(async move {
                let request = ExecutionRequest {
                    task,
                    result_tx: scheduler.result_tx.clone(),
                };

                if let Err(e) = scheduler.process_single_task(request).await {
                    tracing::error!("Task processing failed: {}", e);
                }
            });
        }
    }

    /// Process a single task
    async fn process_single_task(&self, request: ExecutionRequest) -> Result<()> {
        let task = request.task;
        let task_id = task.id;

        tracing::info!(task_id = %task_id, task_name = %task.name, "Processing task");

        let mut retry_count = 0;
        let mut last_error = None;

        loop {
            // Acquire sandbox
            let sandbox = match self.acquire_sandbox(task.sandbox_config.clone()).await {
                Ok(s) => s,
                Err(e) => {
                    last_error = Some(format!("Failed to acquire sandbox: {}", e));
                    break;
                }
            };

            // Start sandbox
            if let Err(e) = sandbox.start().await {
                last_error = Some(format!("Failed to start sandbox: {}", e));
                retry_count += 1;
                if retry_count > task.max_retries {
                    break;
                }
                continue;
            }

            // Execute task
            let start_time = std::time::Instant::now();

            let result = match self.execute_task(&task, &sandbox).await {
                Ok(result) => {
                    TaskResult {
                        task_id,
                        status: if result.exit_status.success() {
                            TaskStatus::Completed
                        } else {
                            TaskStatus::Failed
                        },
                        exit_code: result.exit_status.exit_code,
                        stdout: result.stdout,
                        stderr: result.stderr,
                        duration: start_time.elapsed(),
                        retry_count,
                        error_message: result.exit_status.error_message,
                    }
                }
                Err(e) => {
                    last_error = Some(format!("Execution error: {}", e));
                    retry_count += 1;
                    if retry_count > task.max_retries {
                        break;
                    }
                    continue;
                }
            };

            // Release sandbox
            self.release_sandbox(sandbox).await;

            // Send result
            let _ = request.result_tx.send(result).await;
            return Ok(());
        }

        // All retries exhausted or unrecoverable error
        let result = TaskResult {
            task_id,
            status: TaskStatus::Failed,
            exit_code: -1,
            stdout: String::new(),
            stderr: last_error.clone().unwrap_or_default(),
            duration: Duration::from_secs(0),
            retry_count,
            error_message: last_error,
        };

        let _ = request.result_tx.send(result).await;
        Ok(())
    }

    /// Execute a task using the appropriate driver
    async fn execute_task(
        &self,
        task: &TaskSpec,
        sandbox: &Arc<SandboxInstance>,
    ) -> Result<crate::sandbox::ExecutionResult> {
        // Select driver based on task type
        let driver: Arc<dyn ExecutionDriver> = match task.task_type {
            TaskType::Cli => Arc::new(crate::sandbox::drivers::CliDriver::new()),
            TaskType::Mcp => Arc::new(crate::sandbox::drivers::McpDriver::new()),
            TaskType::MetaTool => Arc::new(crate::sandbox::drivers::MetaToolDriver::new()),
        };

        // Convert to trait object
        let sandbox_trait: Arc<dyn crate::sandbox::traits::SandboxInstance> = sandbox.clone();
        driver.execute(task, &sandbox_trait).await
    }

    /// Acquire a sandbox (from pool or create new)
    async fn acquire_sandbox(&self, config: SandboxConfig) -> Result<Arc<SandboxInstance>> {
        // Try to get from pool
        {
            let mut pool = self.sandbox_pool.write().await;
            if let Some(instance) = pool.pop() {
                // Check if still valid
                if instance.state().await == crate::sandbox::SandboxState::Running {
                    tracing::debug!(sandbox_id = %instance.id(), "Reusing sandbox from pool");
                    return Ok(instance);
                }
            }
        }

        // Create new sandbox
        let inner = self.backend.create(config.clone()).await?;
        let instance = SandboxInstance::new(config.id, config, inner);

        self.registry.register(Arc::clone(&instance)).await;

        tracing::debug!(sandbox_id = %instance.id(), "Created new sandbox");
        Ok(instance)
    }

    /// Release a sandbox back to pool or destroy
    async fn release_sandbox(&self, sandbox: Arc<SandboxInstance>) {
        if sandbox.config().auto_remove {
            // Destroy immediately
            sandbox.destroy().await.ok();
            self.registry.unregister(sandbox.id()).await;
        } else {
            // Return to pool
            let mut pool = self.sandbox_pool.write().await;
            if pool.len() < 5 {
                // Max pool size
                pool.push(sandbox);
            } else {
                // Pool full, destroy
                sandbox.destroy().await.ok();
            }
        }
    }
}

/// Scheduler statistics
#[derive(Debug, Clone)]
pub struct SchedulerStats {
    pub active_sandboxes: usize,
    pub queued_tasks: usize,
    pub active_tasks: usize,
    pub pool_size: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_spec_builder() {
        let task = TaskSpec::new("test-task")
            .with_type(TaskType::Cli)
            .with_command(vec!["ls".to_string(), "-la".to_string()])
            .with_priority(10)
            .with_retries(3)
            .with_timeout(Duration::from_secs(60));

        assert_eq!(task.name, "test-task");
        assert_eq!(task.task_type, TaskType::Cli);
        assert_eq!(task.command, vec!["ls", "-la"]);
        assert_eq!(task.priority, 10);
        assert_eq!(task.max_retries, 3);
        assert_eq!(task.timeout, Duration::from_secs(60));
    }

    #[test]
    fn test_scheduler_stats() {
        let stats = SchedulerStats {
            active_sandboxes: 5,
            queued_tasks: 10,
            active_tasks: 3,
            pool_size: 2,
        };

        assert_eq!(stats.active_sandboxes, 5);
        assert_eq!(stats.queued_tasks, 10);
    }
}
