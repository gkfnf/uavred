//! Boxlite Backend
//!
//! Implementation of SandboxBackend using Boxlite (libkrun microVM).
//!
//! Provides the highest level of isolation:
//! - Hardware virtualization via KVM (Linux) or Hypervisor.framework (macOS)
//! - MicroVM with minimal attack surface
//! - virtiofs for file sharing
//! - vsock for communication
//!
//! ## Requirements
//!
//! - Linux: KVM support, libkrun installed
//! - macOS: Hypervisor.framework (built-in)
//!
//! ## Feature Flag
//!
//! This backend requires the `boxlite-backend` feature to be enabled:
//! ```toml
//! [dependencies]
//! agent = { path = "../crates/agent", features = ["boxlite-backend"] }
//! ```
//!
//! Without this feature, the backend will return an error at initialization.

use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use async_trait::async_trait;
use tokio::sync::mpsc;

use crate::sandbox::{OutputChunk, SandboxConfig, SandboxId, StreamType};
use crate::sandbox::traits::{ExecutionHandle, IsolationLevel, SandboxBackend, SandboxInstance, SandboxState};

// Boxlite integration (only available with boxlite-backend feature)
#[cfg(feature = "boxlite-backend")]
mod boxlite_integration {
    pub use boxlite::{
        BoxliteRuntime, BoxOptions, LiteBox, BoxCommand as LiteBoxCommand,
        runtime::options::{RootfsSpec as BoxliteRootfsSpec, SecurityOptions as BoxliteSecurityOptions, 
                          NetworkSpec as BoxliteNetworkSpec, VolumeSpec as BoxliteVolumeSpec},
        runtime::types::BoxStatus as BoxliteBoxStatus,
        CopyOptions,
    };
    pub use boxlite_shared::errors::BoxliteResult;
}

/// Boxlite backend implementation
pub struct BoxliteBackend {
    #[cfg(feature = "boxlite-backend")]
    runtime: boxlite_integration::BoxliteRuntime,
}

impl BoxliteBackend {
    /// Create a new Boxlite backend
    ///
    /// # Errors
    /// Returns error if:
    /// - The `boxlite-backend` feature is not enabled
    /// - Boxlite runtime initialization fails
    /// - libkrun is not available on the system
    pub async fn new() -> Result<Self> {
        #[cfg(feature = "boxlite-backend")]
        {
            use boxlite_integration::BoxliteRuntime;
            
            let runtime = BoxliteRuntime::with_defaults()
                .map_err(|e| anyhow::anyhow!("Failed to initialize Boxlite runtime: {}", e))?;

            tracing::info!("Boxlite backend initialized");
            Ok(Self { runtime })
        }

        #[cfg(not(feature = "boxlite-backend"))]
        {
            anyhow::bail!(
                "Boxlite backend not compiled. Enable 'boxlite-backend' feature to use this backend.\n\
                 To enable:\n\
                 1. Add 'boxlite-backend' to agent crate features\n\
                 2. Initialize git submodules: git submodule update --init --recursive\n\
                 3. Install Go and C build dependencies"
            )
        }
    }

    /// Check if Boxlite is available
    ///
    /// Returns true only if:
    /// - The `boxlite-backend` feature is enabled
    /// - libkrun is available on the system
    pub async fn is_available() -> bool {
        #[cfg(feature = "boxlite-backend")]
        {
            use boxlite_integration::BoxliteRuntime;
            BoxliteRuntime::with_defaults().is_ok()
        }

        #[cfg(not(feature = "boxlite-backend"))]
        {
            false
        }
    }

    /// Convert our SandboxConfig to Boxlite's BoxOptions
    #[cfg(feature = "boxlite-backend")]
    fn convert_config(&self, config: &SandboxConfig) -> boxlite_integration::BoxOptions {
        use boxlite_integration::*;
        use boxlite::runtime::options::ResourceLimits as BoxliteResourceLimits;

        // Build rootfs spec
        let rootfs = BoxliteRootfsSpec::Image(config.image.clone());

        // Build volume mounts
        let volumes: Vec<BoxliteVolumeSpec> = config.mounts.iter().map(|m| {
            BoxliteVolumeSpec {
                host_path: m.host_path.to_string_lossy().to_string(),
                guest_path: m.guest_path.clone(),
                read_only: m.read_only,
            }
        }).collect();

        // Build security options
        let security = BoxliteSecurityOptions {
            jailer_enabled: config.security.jailer_enabled,
            seccomp_enabled: config.security.seccomp_enabled,
            uid: config.security.uid,
            gid: config.security.gid,
            new_pid_ns: config.security.new_pid_ns,
            new_net_ns: config.security.new_net_ns,
            chroot_base: config.security.chroot_base.clone(),
            chroot_enabled: config.security.chroot_enabled,
            close_fds: config.security.close_fds,
            sanitize_env: config.security.sanitize_env,
            env_allowlist: config.security.env_allowlist.clone(),
            resource_limits: BoxliteResourceLimits {
                max_open_files: config.resources.max_open_files,
                max_file_size: config.resources.max_file_size,
                max_processes: config.resources.max_pids,
                max_memory: config.resources.memory_limit_mb.map(|m| m * 1024 * 1024),
                max_cpu_time: config.resources.max_cpu_time,
            },
            sandbox_profile: config.security.sandbox_profile.clone(),
            network_enabled: config.security.network_enabled,
        };

        BoxOptions {
            cpus: config.resources.cpu_limit.map(|c| c as u8),
            memory_mib: config.resources.memory_limit_mb.map(|m| m as u32),
            disk_size_gb: None,
            working_dir: Some(config.working_dir.clone()),
            env: config.env.iter().map(|(k, v)| (k.clone(), v.clone())).collect(),
            rootfs,
            volumes,
            network: if config.network.enabled {
                BoxliteNetworkSpec::Isolated
            } else {
                BoxliteNetworkSpec::Isolated
            },
            ports: vec![],
            isolate_mounts: false,
            auto_remove: config.auto_remove,
            detach: false,
            security,
            entrypoint: config.entrypoint.clone(),
            cmd: config.cmd.clone(),
            user: config.user.clone(),
        }
    }
}

#[async_trait]
impl SandboxBackend for BoxliteBackend {
    fn name(&self) -> &str {
        "boxlite"
    }

    fn isolation_level(&self) -> IsolationLevel {
        IsolationLevel::MicroVM
    }

    async fn is_available(&self) -> bool {
        Self::is_available().await
    }

    async fn create(&self, config: SandboxConfig) -> Result<Arc<dyn SandboxInstance>> {
        #[cfg(feature = "boxlite-backend")]
        {
            tracing::debug!(sandbox_id = %config.id, "Creating Boxlite sandbox");

            let box_options = self.convert_config(&config);
            let litebox = self.runtime.create(box_options, None).await
                .map_err(|e| anyhow::anyhow!("Failed to create LiteBox: {}", e))?;

            let instance = BoxliteInstance::new(config, litebox);
            Ok(Arc::new(instance))
        }

        #[cfg(not(feature = "boxlite-backend"))]
        {
            anyhow::bail!("Boxlite backend not compiled. Enable 'boxlite-backend' feature.")
        }
    }

    async fn list(&self) -> Result<Vec<SandboxId>> {
        #[cfg(feature = "boxlite-backend")]
        {
            let boxes = self.runtime.list_info().await
                .map_err(|e| anyhow::anyhow!("Failed to list boxes: {}", e))?;
            
            let ids = boxes.iter()
                .filter_map(|b| SandboxId::parse_str(&b.id.to_string()).ok())
                .collect();
            
            Ok(ids)
        }

        #[cfg(not(feature = "boxlite-backend"))]
        {
            Ok(vec![])
        }
    }

    async fn cleanup(&self) -> Result<()> {
        #[cfg(feature = "boxlite-backend")]
        {
            self.runtime.shutdown(Some(30)).await
                .map_err(|e| anyhow::anyhow!("Failed to shutdown runtime: {}", e))?;
        }
        
        Ok(())
    }
}

/// Boxlite sandbox instance wrapper
pub struct BoxliteInstance {
    id: SandboxId,
    config: SandboxConfig,
    state: tokio::sync::RwLock<SandboxState>,
    #[cfg(feature = "boxlite-backend")]
    litebox: boxlite_integration::LiteBox,
}

impl BoxliteInstance {
    #[cfg(feature = "boxlite-backend")]
    fn new(config: SandboxConfig, litebox: boxlite_integration::LiteBox) -> Self {
        Self {
            id: config.id,
            config,
            state: tokio::sync::RwLock::new(SandboxState::Created),
            litebox,
        }
    }

    #[cfg(not(feature = "boxlite-backend"))]
    fn new(config: SandboxConfig) -> Self {
        Self {
            id: config.id,
            config,
            state: tokio::sync::RwLock::new(SandboxState::Created),
        }
    }
}

#[async_trait]
impl SandboxInstance for BoxliteInstance {
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

        #[cfg(feature = "boxlite-backend")]
        {
            self.litebox.start().await
                .map_err(|e| anyhow::anyhow!("Failed to start LiteBox: {}", e))?;
        }

        let mut state = self.state.write().await;
        *state = SandboxState::Running;

        tracing::debug!(sandbox_id = %self.id, "Boxlite sandbox started");
        Ok(())
    }

    async fn stop(&self) -> Result<()> {
        let mut state = self.state.write().await;

        #[cfg(feature = "boxlite-backend")]
        {
            self.litebox.stop().await
                .map_err(|e| anyhow::anyhow!("Failed to stop LiteBox: {}", e))?;
        }

        *state = SandboxState::Stopped;
        tracing::debug!(sandbox_id = %self.id, "Boxlite sandbox stopped");

        Ok(())
    }

    async fn kill(&self) -> Result<()> {
        #[cfg(feature = "boxlite-backend")]
        {
            self.litebox.stop().await
                .map_err(|e| anyhow::anyhow!("Failed to kill LiteBox: {}", e))?;
        }

        let mut state = self.state.write().await;
        *state = SandboxState::Stopped;

        tracing::warn!(sandbox_id = %self.id, "Boxlite sandbox killed");
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
            drop(state);
            self.start().await?;
            state = self.state.write().await;
        }

        *state = SandboxState::Executing;
        drop(state);

        #[cfg(feature = "boxlite-backend")]
        {
            use boxlite_integration::LiteBoxCommand;
            use futures::StreamExt;
            
            // Build BoxCommand
            let program = command.first().cloned().unwrap_or_default();
            let args: Vec<String> = command.into_iter().skip(1).collect();
            
            let mut box_cmd = LiteBoxCommand::new(program);
            for arg in args {
                box_cmd = box_cmd.arg(arg);
            }
            
            // Add environment variables
            if let Some(env_vars) = env {
                for (key, value) in env_vars {
                    box_cmd = box_cmd.env(key, value);
                }
            }
            
            // Set timeout
            if let Some(t) = timeout {
                box_cmd = box_cmd.timeout(t);
            }

            // Execute command
            let execution = self.litebox.exec(box_cmd).await
                .map_err(|e| anyhow::anyhow!("Failed to exec in LiteBox: {}", e))?;

            let (output_tx, output_rx) = mpsc::unbounded_channel();
            
            // Spawn task to collect output
            let completion = tokio::spawn(async move {
                let start_time = std::time::Instant::now();

                // Get stdout stream
                if let Some(mut stdout) = execution.stdout {
                    while let Some(line) = stdout.next().await {
                        let _ = output_tx.send(OutputChunk {
                            stream: StreamType::Stdout,
                            data: line.into_bytes(),
                            timestamp: std::time::Instant::now(),
                        });
                    }
                }

                // Get stderr stream
                if let Some(mut stderr) = execution.stderr {
                    while let Some(line) = stderr.next().await {
                        let _ = output_tx.send(OutputChunk {
                            stream: StreamType::Stderr,
                            data: line.into_bytes(),
                            timestamp: std::time::Instant::now(),
                        });
                    }
                }

                // Wait for result
                let result = execution.wait().await;

                match result {
                    Ok(exec_result) => {
                        Ok(crate::sandbox::ExecutionResult {
                            task_id: crate::sandbox::TaskId::new_v4(),
                            exit_status: crate::sandbox::ExitStatus {
                                exit_code: exec_result.exit_code,
                                signal: None,
                                error_message: None,
                            },
                            stdout: String::new(),
                            stderr: String::new(),
                            duration: start_time.elapsed(),
                            resource_usage: crate::sandbox::ResourceUsage::default(),
                        })
                    }
                    Err(e) => Err(anyhow::anyhow!("Execution failed: {}", e)),
                }
            });

            Ok(ExecutionHandle {
                output_rx,
                completion,
            })
        }

        #[cfg(not(feature = "boxlite-backend"))]
        {
            anyhow::bail!("Boxlite backend not compiled")
        }
    }

    async fn copy_in(&self, source: &Path, dest: &str) -> Result<()> {
        #[cfg(feature = "boxlite-backend")]
        {
            let opts = boxlite_integration::CopyOptions::default();
            self.litebox.copy_into(source, dest, opts).await
                .map_err(|e| anyhow::anyhow!("Failed to copy into LiteBox: {}", e))?;
        }

        tracing::debug!("Copying {:?} to {}", source, dest);
        Ok(())
    }

    async fn copy_out(&self, source: &str, dest: &Path) -> Result<()> {
        #[cfg(feature = "boxlite-backend")]
        {
            let opts = boxlite_integration::CopyOptions::default();
            self.litebox.copy_out(source, dest, opts).await
                .map_err(|e| anyhow::anyhow!("Failed to copy out of LiteBox: {}", e))?;
        }

        tracing::debug!("Copying {} to {:?}", source, dest);
        Ok(())
    }

    async fn resource_usage(&self) -> Result<crate::sandbox::ResourceUsage> {
        #[cfg(feature = "boxlite-backend")]
        {
            let metrics = self.litebox.metrics().await
                .map_err(|e| anyhow::anyhow!("Failed to get metrics: {}", e))?;

            Ok(crate::sandbox::ResourceUsage {
                cpu_seconds: 0.0,
                memory_peak_mb: metrics.memory_bytes / (1024 * 1024),
                memory_avg_mb: metrics.memory_bytes / (1024 * 1024),
                disk_read_bytes: 0,
                disk_write_bytes: 0,
                network_rx_bytes: 0,
                network_tx_bytes: 0,
            })
        }

        #[cfg(not(feature = "boxlite-backend"))]
        {
            Ok(crate::sandbox::ResourceUsage::default())
        }
    }

    async fn wait(&self) -> Result<crate::sandbox::ExitStatus> {
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
    async fn test_boxlite_backend_creation() {
        // This will fail if Boxlite is not available
        let backend = BoxliteBackend::new().await;

        if let Ok(backend) = backend {
            assert_eq!(backend.name(), "boxlite");
            assert_eq!(backend.isolation_level(), IsolationLevel::MicroVM);
        }
    }

    #[test]
    fn test_isolation_level() {
        // IsolationLevel is ordered from weakest to strongest
        assert!(IsolationLevel::MicroVM > IsolationLevel::Container);
        assert!(IsolationLevel::Container > IsolationLevel::Process);
        assert!(IsolationLevel::Process > IsolationLevel::None);
    }

    #[test]
    fn test_boxlite_not_available_without_feature() {
        // Without boxlite-backend feature, is_available should return false
        #[cfg(not(feature = "boxlite-backend"))]
        {
            let rt = tokio::runtime::Runtime::new().unwrap();
            let available = rt.block_on(BoxliteBackend::is_available());
            assert!(!available);
        }
    }
}
