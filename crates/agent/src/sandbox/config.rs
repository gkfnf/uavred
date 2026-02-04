//! Sandbox Configuration
//!
//! Configuration types for sandbox instances including resource limits,
//! network policies, and security options.

use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::sandbox::SandboxId;

/// Configuration for a sandbox instance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxConfig {
    /// Unique identifier for this sandbox
    pub id: SandboxId,

    /// Base image to use (OCI image reference or path)
    pub image: String,

    /// Working directory inside the sandbox
    pub working_dir: String,

    /// Environment variables
    pub env: HashMap<String, String>,

    /// Resource limits
    pub resources: ResourceLimits,

    /// Network policy
    pub network: NetworkPolicy,

    /// Volume mounts (host path -> guest path)
    pub mounts: Vec<MountSpec>,

    /// Security options
    pub security: SecurityOptions,

    /// Execution timeout
    pub timeout: Duration,

    /// Auto-remove sandbox after execution
    pub auto_remove: bool,

    /// Enable PTY for interactive sessions
    pub tty: bool,

    /// User to run as (username or uid:gid)
    pub user: Option<String>,

    /// Entrypoint override
    pub entrypoint: Option<Vec<String>>,

    /// Command override
    pub cmd: Option<Vec<String>>,
}

impl SandboxConfig {
    /// Create a new configuration with defaults
    pub fn new(image: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            image: image.into(),
            working_dir: "/workspace".to_string(),
            env: HashMap::new(),
            resources: ResourceLimits::default(),
            network: NetworkPolicy::default(),
            mounts: Vec::new(),
            security: SecurityOptions::default(),
            timeout: Duration::from_secs(300),
            auto_remove: true,
            tty: false,
            user: None,
            entrypoint: None,
            cmd: None,
        }
    }

    /// Create a configuration builder
    pub fn builder() -> SandboxConfigBuilder {
        SandboxConfigBuilder::default()
    }

    /// Validate the configuration
    pub fn validate(&self) -> anyhow::Result<()> {
        // Validate image reference
        if self.image.is_empty() {
            anyhow::bail!("Image reference cannot be empty");
        }

        // Validate resource limits
        if let Some(memory) = self.resources.memory_limit_mb {
            if memory == 0 {
                anyhow::bail!("Memory limit cannot be zero");
            }
        }

        // Validate mounts
        for mount in &self.mounts {
            if mount.host_path.as_os_str().is_empty() {
                anyhow::bail!("Mount host path cannot be empty");
            }
            if mount.guest_path.is_empty() {
                anyhow::bail!("Mount guest path cannot be empty");
            }
        }

        Ok(())
    }
}

impl Default for SandboxConfig {
    fn default() -> Self {
        Self::new("alpine:latest")
    }
}

/// Builder for SandboxConfig
#[derive(Debug, Default)]
pub struct SandboxConfigBuilder {
    image: Option<String>,
    working_dir: Option<String>,
    env: HashMap<String, String>,
    resources: ResourceLimits,
    network: NetworkPolicy,
    mounts: Vec<MountSpec>,
    security: SecurityOptions,
    timeout: Option<Duration>,
    auto_remove: bool,
    tty: bool,
    user: Option<String>,
    entrypoint: Option<Vec<String>>,
    cmd: Option<Vec<String>>,
}

impl SandboxConfigBuilder {
    pub fn image(mut self, image: impl Into<String>) -> Self {
        self.image = Some(image.into());
        self
    }

    pub fn working_dir(mut self, dir: impl Into<String>) -> Self {
        self.working_dir = Some(dir.into());
        self
    }

    pub fn env(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.env.insert(key.into(), value.into());
        self
    }

    pub fn envs(mut self, envs: HashMap<String, String>) -> Self {
        self.env.extend(envs);
        self
    }

    pub fn memory_limit_mb(mut self, limit: u64) -> Self {
        self.resources.memory_limit_mb = Some(limit);
        self
    }

    pub fn cpu_limit(mut self, limit: f64) -> Self {
        self.resources.cpu_limit = Some(limit);
        self
    }

    pub fn network_enabled(mut self, enabled: bool) -> Self {
        self.network.enabled = enabled;
        self
    }

    pub fn network_policy(mut self, policy: NetworkPolicy) -> Self {
        self.network = policy;
        self
    }

    pub fn mount(mut self, host: impl Into<PathBuf>, guest: impl Into<String>, read_only: bool) -> Self {
        self.mounts.push(MountSpec {
            host_path: host.into(),
            guest_path: guest.into(),
            read_only,
        });
        self
    }

    pub fn timeout_seconds(mut self, seconds: u64) -> Self {
        self.timeout = Some(Duration::from_secs(seconds));
        self
    }

    pub fn timeout(mut self, duration: Duration) -> Self {
        self.timeout = Some(duration);
        self
    }

    pub fn auto_remove(mut self, auto_remove: bool) -> Self {
        self.auto_remove = auto_remove;
        self
    }

    pub fn tty(mut self, tty: bool) -> Self {
        self.tty = tty;
        self
    }

    pub fn user(mut self, user: impl Into<String>) -> Self {
        self.user = Some(user.into());
        self
    }

    pub fn entrypoint(mut self, entrypoint: Vec<String>) -> Self {
        self.entrypoint = Some(entrypoint);
        self
    }

    pub fn cmd(mut self, cmd: Vec<String>) -> Self {
        self.cmd = Some(cmd);
        self
    }

    pub fn security(mut self, security: SecurityOptions) -> Self {
        self.security = security;
        self
    }

    pub fn resources(mut self, resources: ResourceLimits) -> Self {
        self.resources = resources;
        self
    }

    pub fn build(self) -> SandboxConfig {
        SandboxConfig {
            id: Uuid::new_v4(),
            image: self.image.unwrap_or_else(|| "alpine:latest".to_string()),
            working_dir: self.working_dir.unwrap_or_else(|| "/workspace".to_string()),
            env: self.env,
            resources: self.resources,
            network: self.network,
            mounts: self.mounts,
            security: self.security,
            timeout: self.timeout.unwrap_or_else(|| Duration::from_secs(300)),
            auto_remove: self.auto_remove,
            tty: self.tty,
            user: self.user,
            entrypoint: self.entrypoint,
            cmd: self.cmd,
        }
    }
}

/// Resource limits for sandbox execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    /// Memory limit in MB
    pub memory_limit_mb: Option<u64>,

    /// CPU limit (1.0 = 1 core)
    pub cpu_limit: Option<f64>,

    /// Maximum number of PIDs
    pub max_pids: Option<u64>,

    /// Disk I/O bandwidth limit (MB/s)
    pub disk_io_limit_mbps: Option<u64>,

    /// Maximum file size (bytes)
    pub max_file_size: Option<u64>,

    /// Maximum number of open files
    pub max_open_files: Option<u64>,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            memory_limit_mb: Some(512),
            cpu_limit: Some(1.0),
            max_pids: Some(64),
            disk_io_limit_mbps: None,
            max_file_size: Some(1024 * 1024 * 100), // 100MB
            max_open_files: Some(1024),
        }
    }
}

/// Network policy for sandbox
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkPolicy {
    /// Enable network access
    pub enabled: bool,

    /// Allowed outbound ports (empty = all if enabled)
    pub allowed_ports: Vec<u16>,

    /// Allowed IP ranges (CIDR notation)
    pub allowed_ips: Vec<String>,

    /// Blocked IP ranges (CIDR notation)
    pub blocked_ips: Vec<String>,

    /// DNS servers to use
    pub dns_servers: Vec<String>,

    /// Enable outbound connections
    pub outbound_allowed: bool,

    /// Enable inbound connections
    pub inbound_allowed: bool,
}

impl Default for NetworkPolicy {
    fn default() -> Self {
        Self {
            enabled: false,
            allowed_ports: Vec::new(),
            allowed_ips: Vec::new(),
            blocked_ips: Vec::new(),
            dns_servers: vec!["8.8.8.8".to_string()],
            outbound_allowed: false,
            inbound_allowed: false,
        }
    }
}

impl NetworkPolicy {
    /// Create an isolated network policy (no network access)
    pub fn isolated() -> Self {
        Self::default()
    }

    /// Create a restricted network policy (only allowed ports/IPs)
    pub fn restricted(allowed_ports: Vec<u16>) -> Self {
        Self {
            enabled: true,
            allowed_ports,
            outbound_allowed: true,
            ..Default::default()
        }
    }

    /// Create an unrestricted network policy (full access)
    pub fn unrestricted() -> Self {
        Self {
            enabled: true,
            outbound_allowed: true,
            inbound_allowed: true,
            ..Default::default()
        }
    }
}

/// Mount specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MountSpec {
    /// Host path
    pub host_path: PathBuf,

    /// Guest path
    pub guest_path: String,

    /// Read-only mount
    pub read_only: bool,
}

/// Security options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityOptions {
    /// Run as root (disabled by default for security)
    pub allow_root: bool,

    /// Enable seccomp syscall filtering
    pub seccomp_enabled: bool,

    /// Enable AppArmor/SELinux
    pub mac_enabled: bool,

    /// Capabilities to drop
    pub drop_capabilities: Vec<String>,

    /// Capabilities to add
    pub add_capabilities: Vec<String>,

    /// Read-only root filesystem
    pub read_only_rootfs: bool,

    /// No new privileges
    pub no_new_privileges: bool,
}

impl Default for SecurityOptions {
    fn default() -> Self {
        Self {
            allow_root: false,
            seccomp_enabled: true,
            mac_enabled: true,
            drop_capabilities: vec![
                "ALL".to_string(),
            ],
            add_capabilities: vec![],
            read_only_rootfs: true,
            no_new_privileges: true,
        }
    }
}

impl SecurityOptions {
    /// Maximum security (for untrusted code)
    pub fn maximum() -> Self {
        Self {
            allow_root: false,
            seccomp_enabled: true,
            mac_enabled: true,
            drop_capabilities: vec!["ALL".to_string()],
            add_capabilities: vec![],
            read_only_rootfs: true,
            no_new_privileges: true,
        }
    }

    /// Development mode (less restrictive)
    pub fn development() -> Self {
        Self {
            allow_root: true,
            seccomp_enabled: false,
            mac_enabled: false,
            drop_capabilities: vec![],
            add_capabilities: vec![],
            read_only_rootfs: false,
            no_new_privileges: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_builder() {
        let config = SandboxConfig::builder()
            .image("test:latest")
            .memory_limit_mb(1024)
            .cpu_limit(2.0)
            .network_enabled(true)
            .timeout_seconds(600)
            .build();

        assert_eq!(config.image, "test:latest");
        assert_eq!(config.resources.memory_limit_mb, Some(1024));
        assert_eq!(config.resources.cpu_limit, Some(2.0));
        assert!(config.network.enabled);
        assert_eq!(config.timeout, Duration::from_secs(600));
    }

    #[test]
    fn test_network_policies() {
        let isolated = NetworkPolicy::isolated();
        assert!(!isolated.enabled);

        let restricted = NetworkPolicy::restricted(vec![80, 443]);
        assert!(restricted.enabled);
        assert!(restricted.outbound_allowed);
        assert_eq!(restricted.allowed_ports, vec![80, 443]);

        let unrestricted = NetworkPolicy::unrestricted();
        assert!(unrestricted.enabled);
        assert!(unrestricted.inbound_allowed);
        assert!(unrestricted.outbound_allowed);
    }

    #[test]
    fn test_security_options() {
        let max_sec = SecurityOptions::maximum();
        assert!(!max_sec.allow_root);
        assert!(max_sec.seccomp_enabled);
        assert!(max_sec.read_only_rootfs);

        let dev_sec = SecurityOptions::development();
        assert!(dev_sec.allow_root);
        assert!(!dev_sec.seccomp_enabled);
    }

    #[test]
    fn test_config_validation() {
        let valid = SandboxConfig::new("alpine:latest");
        assert!(valid.validate().is_ok());

        let mut invalid = SandboxConfig::new("");
        invalid.image = "".to_string();
        assert!(invalid.validate().is_err());
    }
}
