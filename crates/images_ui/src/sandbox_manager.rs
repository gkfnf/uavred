use data::{ContainerExecutionStatus, ContainerStatus};
use futures::FutureExt;
use gpui::*;
use std::collections::HashMap;
use std::time::Duration;
use tokio::sync::mpsc;

/// Events emitted by the sandbox manager
#[derive(Clone, Debug)]
pub enum SandboxEvent {
    ContainersUpdated(Vec<ContainerStatus>),
    ContainerStarted(String),
    ContainerStopped(String),
    Error(String),
}

impl EventEmitter<SandboxEvent> for SandboxManager {}

/// Sandbox/Container manager - interfaces with BoxLite or Docker
pub struct SandboxManager {
    containers: Vec<ContainerStatus>,
    command_tx: mpsc::UnboundedSender<SandboxCommand>,
    _task: Option<Task<()>>,
}

#[derive(Debug)]
enum SandboxCommand {
    StartContainer {
        image_name: String,
        agent_name: String,
        ports: Vec<i32>,
    },
    StopContainer(String),
    GetContainers,
    ExecuteTask {
        container_id: String,
        task_command: String,
    },
}

impl SandboxManager {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let (command_tx, mut command_rx) = mpsc::unbounded_channel::<SandboxCommand>();
        
        // Create initial sample containers
        let initial_containers = vec![
            ContainerStatus {
                container_id: "abc123def456".to_string(),
                agent: "Agent-Alpha".to_string(),
                task_name: "CVE-2024-1234 PoC 生成".to_string(),
                docker_exec_command: "img-1".to_string(),
                status: ContainerExecutionStatus::Running,
                running_duration: "2h 34m".to_string(),
                cpu_usage_percent: 67.0,
                memory_usage_mb: 920,
                memory_limit_mb: 2048,
                exposed_ports: vec!["8080".to_string(), "5900".to_string()],
            },
            ContainerStatus {
                container_id: "def789ghi012".to_string(),
                agent: "Agent-Beta".to_string(),
                task_name: "网络拓扑扫描".to_string(),
                docker_exec_command: "img-2".to_string(),
                status: ContainerExecutionStatus::Running,
                running_duration: "45m".to_string(),
                cpu_usage_percent: 23.0,
                memory_usage_mb: 640,
                memory_limit_mb: 2048,
                exposed_ports: vec!["8081".to_string(), "5901".to_string()],
            },
            ContainerStatus {
                container_id: "ghi345jkl678".to_string(),
                agent: "Agent-Gamma".to_string(),
                task_name: "DJI Mavic 漏洞利用开发".to_string(),
                docker_exec_command: "img-3".to_string(),
                status: ContainerExecutionStatus::Running,
                running_duration: "1h 12m".to_string(),
                cpu_usage_percent: 89.0,
                memory_usage_mb: 1580,
                memory_limit_mb: 2048,
                exposed_ports: vec!["8082".to_string(), "5902".to_string()],
            },
            ContainerStatus {
                container_id: "stopped123456".to_string(),
                agent: "Agent-Delta".to_string(),
                task_name: "Payload 模糊测试".to_string(),
                docker_exec_command: "img-5".to_string(),
                status: ContainerExecutionStatus::Running,
                running_duration: "3h 21m".to_string(),
                cpu_usage_percent: 45.0,
                memory_usage_mb: 890,
                memory_limit_mb: 2048,
                exposed_ports: vec!["8083".to_string(), "5903".to_string()],
            },
        ];

        // Emit initial containers (subscribers may or may not receive this depending on timing)
        cx.emit(SandboxEvent::ContainersUpdated(initial_containers.clone()));
        
        // Spawn background task for sandbox operations
        let _task = Some(cx.background_spawn({
            let mut containers: HashMap<String, ContainerStatus> = initial_containers
                .clone()
                .into_iter()
                .map(|c| (c.docker_exec_command.clone(), c))
                .collect();
            
            async move {
                // Simulate resource updates using smol timer
                let mut tick_count: u64 = 0;
                
                loop {
                    // Create a fused timer for this iteration
                    let timer_fut = smol::Timer::after(Duration::from_secs(5)).fuse();
                    futures::pin_mut!(timer_fut);
                    
                    // Wait for either timer tick or command using futures::select
                    futures::select! {
                        _ = timer_fut => {
                            // Timer tick - update metrics
                            tick_count += 1;
                            for (idx, (_, container)) in containers.iter_mut().enumerate() {
                                if matches!(container.status, ContainerExecutionStatus::Running) {
                                    // Simulate CPU fluctuation using tick_count
                                    let fluctuation = (((tick_count + idx as u64) % 10) as f64 - 5.0);
                                    container.cpu_usage_percent = 
                                        (container.cpu_usage_percent + fluctuation).clamp(5.0, 95.0);
                                    
                                    // Simulate memory fluctuation
                                    let mem_fluctuation = (((tick_count + idx as u64) % 20) as f64 - 10.0) * 5.0;
                                    container.memory_usage_mb = 
                                        ((container.memory_usage_mb as f64 + mem_fluctuation) as u64)
                                            .clamp(100, container.memory_limit_mb);
                                }
                            }
                        }
                        
                        cmd = command_rx.recv().fuse() => {
                            match cmd {
                                Some(SandboxCommand::StartContainer { image_name, agent_name, ports: _ }) => {
                                    tracing::info!(
                                        "Starting container: {} for agent {}",
                                        image_name, agent_name
                                    );
                                    // TODO: Integrate with BoxLite or Docker
                                }
                                Some(SandboxCommand::StopContainer(container_id)) => {
                                    tracing::info!("Stopping container: {}", container_id);
                                    if let Some(container) = containers.get_mut(&container_id) {
                                        container.status = ContainerExecutionStatus::Stopped;
                                    }
                                }
                                Some(SandboxCommand::GetContainers) => {
                                    // Return current container list
                                }
                                Some(SandboxCommand::ExecuteTask { container_id, task_command }) => {
                                    tracing::info!(
                                        "Executing task in {}: {}",
                                        container_id, task_command
                                    );
                                    // TODO: Execute task in container
                                }
                                None => {
                                    // Channel closed, exit loop
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }));

        Self {
            containers: initial_containers,
            command_tx,
            _task,
        }
    }

    /// Start a new container
    pub fn start_container(
        &self,
        image_name: String,
        agent_name: String,
        ports: Vec<i32>,
    ) -> anyhow::Result<()> {
        self.command_tx
            .send(SandboxCommand::StartContainer {
                image_name,
                agent_name,
                ports,
            })
            .map_err(|_| anyhow::anyhow!("Failed to send start command"))
    }

    /// Stop a container
    pub fn stop_container(&self, container_id: String) -> anyhow::Result<()> {
        self.command_tx
            .send(SandboxCommand::StopContainer(container_id))
            .map_err(|_| anyhow::anyhow!("Failed to send stop command"))
    }

    /// Execute a task in a container
    pub fn execute_task(
        &self,
        container_id: String,
        task_command: String,
    ) -> anyhow::Result<()> {
        self.command_tx
            .send(SandboxCommand::ExecuteTask {
                container_id,
                task_command,
            })
            .map_err(|_| anyhow::anyhow!("Failed to send execute command"))
    }

    /// Get all containers (for initial load)
    pub fn containers(&self) -> &[ContainerStatus] {
        &self.containers
    }

    /// Update containers list (called by background task)
    pub fn update_containers(&mut self, containers: Vec<ContainerStatus>, cx: &mut Context<Self>) {
        self.containers = containers;
        cx.emit(SandboxEvent::ContainersUpdated(self.containers.clone()));
        cx.notify();
    }
}

/// Integration trait for BoxLite
#[async_trait::async_trait]
pub trait BoxLiteIntegration: Send + Sync {
    async fn create_box(&self, image: &str) -> anyhow::Result<String>;
    async fn execute_command(&self, box_id: &str, command: &[&str]) -> anyhow::Result<String>;
    async fn get_metrics(&self, box_id: &str) -> anyhow::Result<ContainerMetrics>;
    async fn stop_box(&self, box_id: &str) -> anyhow::Result<()>;
}

/// Container metrics from BoxLite/Docker
#[derive(Debug, Clone)]
pub struct ContainerMetrics {
    pub cpu_percent: f64,
    pub memory_usage_mb: u64,
    pub memory_limit_mb: u64,
}

/// Docker integration implementation
pub struct DockerIntegration;

#[async_trait::async_trait]
impl BoxLiteIntegration for DockerIntegration {
    async fn create_box(&self, image: &str) -> anyhow::Result<String> {
        // TODO: Implement Docker container creation
        tracing::info!("Creating Docker container from image: {}", image);
        Ok("container-id".to_string())
    }

    async fn execute_command(&self, box_id: &str, command: &[&str]) -> anyhow::Result<String> {
        tracing::info!("Executing command in {}: {:?}", box_id, command);
        Ok("output".to_string())
    }

    async fn get_metrics(&self, box_id: &str) -> anyhow::Result<ContainerMetrics> {
        tracing::info!("Getting metrics for {}", box_id);
        Ok(ContainerMetrics {
            cpu_percent: 0.0,
            memory_usage_mb: 0,
            memory_limit_mb: 2048,
        })
    }

    async fn stop_box(&self, box_id: &str) -> anyhow::Result<()> {
        tracing::info!("Stopping container {}", box_id);
        Ok(())
    }
}
