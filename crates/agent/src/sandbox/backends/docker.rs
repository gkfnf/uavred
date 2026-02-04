//! Docker Backend
//!
//! Implementation of SandboxBackend using Docker containers.
//!
//! Provides good isolation and is widely available:
//! - OS-level containerization
//! - cgroup resource limits
//! - Network isolation
//! - Image management

use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use async_trait::async_trait;
use bollard::Docker;
use bollard::container::{Config, CreateContainerOptions, StartContainerOptions, StopContainerOptions, RemoveContainerOptions, ListContainersOptions, WaitContainerOptions};
use bollard::exec::CreateExecOptions;
use bollard::image::CreateImageOptions;
use bollard::models::{HostConfig, Resources};
use futures::StreamExt;
use tokio::sync::mpsc;
use tokio::process::Command;

use crate::sandbox::{OutputChunk, SandboxConfig, SandboxId, StreamType};
use crate::sandbox::traits::{ExecutionHandle, IsolationLevel, SandboxBackend, SandboxInstance, SandboxState};

/// Docker backend implementation
pub struct DockerBackend {
    client: Docker,
}

impl DockerBackend {
    /// Create a new Docker backend
    pub async fn new() -> Result<Self> {
        let client = Docker::connect_with_local_defaults()
            .map_err(|e| anyhow::anyhow!("Failed to connect to Docker: {}", e))?;

        // Verify connection
        client.version().await
            .map_err(|e| anyhow::anyhow!("Docker daemon not responding: {}", e))?;

        tracing::info!("Docker backend initialized");
        Ok(Self { client })
    }

    /// Check if Docker is available
    async fn check_availability() -> bool {
        match Docker::connect_with_local_defaults() {
            Ok(client) => client.version().await.is_ok(),
            Err(_) => false,
        }
    }
}

#[async_trait]
impl SandboxBackend for DockerBackend {
    fn name(&self) -> &str {
        "docker"
    }

    fn isolation_level(&self) -> IsolationLevel {
        IsolationLevel::Container
    }

    async fn is_available(&self) -> bool {
        Self::check_availability().await
    }

    async fn create(&self, config: SandboxConfig) -> Result<Arc<dyn SandboxInstance>> {
        tracing::debug!(sandbox_id = %config.id, image = %config.image, "Creating Docker container");

        let instance = DockerInstance::new(config, self.client.clone());
        Ok(Arc::new(instance))
    }

    async fn list(&self) -> Result<Vec<SandboxId>> {
        let options = ListContainersOptions {
            all: true,
            filters: vec![
                ("label", vec!["uavred.sandbox=true"]),
            ].into_iter().collect(),
            ..Default::default()
        };

        let containers = self.client.list_containers(Some(options)).await?;
        let ids = containers.iter()
            .filter_map(|c| c.id.as_ref())
            .filter_map(|id| SandboxId::parse_str(&id[..32]).ok())
            .collect();

        Ok(ids)
    }

    async fn cleanup(&self) -> Result<()> {
        // Remove stopped containers with our label
        let options = ListContainersOptions {
            all: true,
            filters: vec![
                ("label", vec!["uavred.sandbox=true"]),
                ("status", vec!["exited"]),
            ].into_iter().collect(),
            ..Default::default()
        };

        let containers = self.client.list_containers(Some(options)).await?;
        for container in containers {
            if let Some(id) = container.id {
                let _ = self.client.remove_container(&id, Some(RemoveContainerOptions {
                    force: true,
                    ..Default::default()
                })).await;
            }
        }

        Ok(())
    }
}

/// Docker container instance
pub struct DockerInstance {
    id: SandboxId,
    config: SandboxConfig,
    state: tokio::sync::RwLock<SandboxState>,
    client: Docker,
    container_id: tokio::sync::RwLock<Option<String>>,
}

impl DockerInstance {
    fn new(config: SandboxConfig, client: Docker) -> Self {
        Self {
            id: config.id,
            config,
            state: tokio::sync::RwLock::new(SandboxState::Created),
            client,
            container_id: tokio::sync::RwLock::new(None),
        }
    }

    /// Generate Docker container name
    fn container_name(&self) -> String {
        format!("uavred-sandbox-{}", self.id)
    }

    /// Pull image if needed
    async fn pull_image(&self, image: &str) -> Result<()> {
        tracing::info!("Pulling image: {}", image);

        let options = CreateImageOptions {
            from_image: image,
            ..Default::default()
        };

        let mut stream = self.client.create_image(Some(options), None, None);
        while let Some(result) = stream.next().await {
            match result {
                Ok(progress) => {
                    if let Some(status) = progress.status {
                        tracing::debug!("Pull progress: {}", status);
                    }
                }
                Err(e) => {
                    tracing::warn!("Pull warning: {}", e);
                }
            }
        }

        Ok(())
    }

    /// Build container configuration
    fn build_container_config(&self) -> Config<String> {
        let mut env = Vec::new();
        for (key, value) in &self.config.env {
            env.push(format!("{}={}", key, value));
        }

        let mut labels = std::collections::HashMap::new();
        labels.insert("uavred.sandbox".to_string(), "true".to_string());
        labels.insert("uavred.sandbox.id".to_string(), self.id.to_string());

        // Build binds (volume mounts)
        let mut binds = Vec::new();
        for mount in &self.config.mounts {
            let bind = format!(
                "{}:{}{}",
                mount.host_path.display(),
                mount.guest_path,
                if mount.read_only { ":ro" } else { ":rw" }
            );
            binds.push(bind);
        }

        // Build resource limits
        let mut resources = Resources::default();
        if let Some(memory_mb) = self.config.resources.memory_limit_mb {
            resources.memory = Some((memory_mb * 1024 * 1024) as i64);
            resources.memory_swap = Some((memory_mb * 1024 * 1024) as i64); // Disable swap
        }
        if let Some(cpu_limit) = self.config.resources.cpu_limit {
            resources.cpu_quota = Some((cpu_limit * 100000.0) as i64); // Quota in microseconds
            resources.cpu_period = Some(100000); // Period in microseconds
        }
        if let Some(max_pids) = self.config.resources.max_pids {
            resources.pids_limit = Some(max_pids as i64);
        }

        // Network mode
        let network_mode = if self.config.network.enabled {
            None
        } else {
            Some("none".to_string())
        };

        Config {
            image: Some(self.config.image.clone()),
            env: Some(env),
            working_dir: Some(self.config.working_dir.clone()),
            labels: Some(labels),
            host_config: Some(HostConfig {
                binds: if binds.is_empty() { None } else { Some(binds) },
                network_mode,
                cpu_quota: resources.cpu_quota,
                cpu_period: resources.cpu_period,
                memory: resources.memory,
                memory_swap: resources.memory_swap,
                pids_limit: resources.pids_limit,
                ..Default::default()
            }),
            ..Default::default()
        }
    }
}

#[async_trait]
impl SandboxInstance for DockerInstance {
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

        *state = SandboxState::Starting;
        drop(state);

        // Pull image if needed
        self.pull_image(&self.config.image).await?;

        // Create container
        let config = self.build_container_config();
        let options = CreateContainerOptions {
            name: self.container_name(),
            ..Default::default()
        };

        let container = self.client.create_container(Some(options), config).await?;
        let container_id = container.id.clone();
        *self.container_id.write().await = Some(container_id.clone());

        // Start container
        self.client.start_container::<String>(&container_id, None).await?;

        let mut state = self.state.write().await;
        *state = SandboxState::Running;

        tracing::debug!(sandbox_id = %self.id, container_id = %container_id, "Docker container started");
        Ok(())
    }

    async fn stop(&self) -> Result<()> {
        let container_id = match self.container_id.read().await.as_ref() {
            Some(id) => id.clone(),
            None => return Ok(()),
        };

        let options = StopContainerOptions {
            t: 10, // 10 second timeout
        };
        let _ = self.client.stop_container(&container_id, Some(options)).await;

        let mut state = self.state.write().await;
        *state = SandboxState::Stopped;
        
        tracing::debug!(sandbox_id = %self.id, "Docker container stopped");
        Ok(())
    }

    async fn kill(&self) -> Result<()> {
        let container_id = match self.container_id.read().await.as_ref() {
            Some(id) => id.clone(),
            None => return Ok(()),
        };

        let _ = self.client.kill_container::<&str>(&container_id, None).await;

        let mut state = self.state.write().await;
        *state = SandboxState::Stopped;

        tracing::warn!(sandbox_id = %self.id, "Docker container killed");
        Ok(())
    }

    async fn exec(
        &self,
        command: Vec<String>,
        env: Option<Vec<(String, String)>>,
        _timeout: Option<Duration>,
    ) -> Result<ExecutionHandle> {
        let container_id = self.container_id.read().await.clone()
            .ok_or_else(|| anyhow::anyhow!("Container not created"))?;

        // Use docker exec command directly for simplicity
        let (output_tx, output_rx) = mpsc::unbounded_channel();
        
        let completion = tokio::spawn(async move {
            let start_time = std::time::Instant::now();

            // Build docker exec command
            let mut cmd = Command::new("docker");
            cmd.arg("exec").arg(&container_id);
            
            // Add environment variables
            if let Some(env_vars) = env {
                for (key, value) in env_vars {
                    cmd.arg("-e").arg(format!("{}={}", key, value));
                }
            }
            
            // Add command
            for arg in &command {
                cmd.arg(arg);
            }

            // Execute and capture output
            let output = cmd.output().await;
            
            let exit_code = match &output {
                Ok(o) => {
                    if !o.stdout.is_empty() {
                        let _ = output_tx.send(OutputChunk {
                            stream: StreamType::Stdout,
                            data: o.stdout.clone(),
                            timestamp: std::time::Instant::now(),
                        });
                    }
                    if !o.stderr.is_empty() {
                        let _ = output_tx.send(OutputChunk {
                            stream: StreamType::Stderr,
                            data: o.stderr.clone(),
                            timestamp: std::time::Instant::now(),
                        });
                    }
                    o.status.code().unwrap_or(-1)
                }
                Err(_) => -1,
            };

            Ok(crate::sandbox::ExecutionResult {
                task_id: crate::sandbox::TaskId::new_v4(),
                exit_status: crate::sandbox::ExitStatus {
                    exit_code,
                    signal: None,
                    error_message: None,
                },
                stdout: String::new(),
                stderr: String::new(),
                duration: start_time.elapsed(),
                resource_usage: crate::sandbox::ResourceUsage::default(),
            })
        });

        Ok(ExecutionHandle {
            output_rx,
            completion,
        })
    }

    async fn copy_in(&self, _source: &Path, _dest: &str) -> Result<()> {
        // TODO: Implement using docker cp command
        tracing::warn!("Docker copy_in not yet implemented");
        Ok(())
    }

    async fn copy_out(&self, _source: &str, _dest: &Path) -> Result<()> {
        // TODO: Implement using docker cp command
        tracing::warn!("Docker copy_out not yet implemented");
        Ok(())
    }

    async fn resource_usage(&self) -> Result<crate::sandbox::ResourceUsage> {
        // TODO: Get stats from Docker
        Ok(crate::sandbox::ResourceUsage::default())
    }

    async fn wait(&self) -> Result<crate::sandbox::ExitStatus> {
        // Wait for container to exit
        if let Some(container_id) = self.container_id.read().await.as_ref() {
            let options = WaitContainerOptions {
                condition: "not-running",
            };
            let _ = self.client.wait_container(container_id, Some(options)).next().await;
        }

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
    use std::path::PathBuf;

    #[tokio::test]
    async fn test_docker_backend_creation() {
        // This will fail if Docker is not available
        let backend = DockerBackend::new().await;

        if let Ok(backend) = backend {
            assert_eq!(backend.name(), "docker");
            assert_eq!(backend.isolation_level(), IsolationLevel::Container);
        }
    }

    #[test]
    fn test_docker_config() {
        let config = SandboxConfig::builder()
            .image("alpine:latest")
            .memory_limit_mb(512)
            .cpu_limit(1.0)
            .network_enabled(false)
            .mount(PathBuf::from("/tmp"), "/workspace", false)
            .build();

        assert_eq!(config.image, "alpine:latest");
        assert_eq!(config.resources.memory_limit_mb, Some(512));
    }
}
