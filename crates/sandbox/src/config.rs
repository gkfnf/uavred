//! Sandbox configuration

use serde::{Deserialize, Serialize};

/// Sandbox configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxConfig {
    /// Unique sandbox identifier
    pub id: String,
    /// Display name
    pub name: String,
    /// Image type
    pub image: String,
    /// Network mode
    pub network_mode: NetworkMode,
    /// Volume mounts
    pub volumes: Vec<VolumeMount>,
    /// Environment variables
    pub env_vars: Vec<(String, String)>,
    /// Resource limits
    pub resource_limits: Option<ResourceLimits>,
}

impl Default for SandboxConfig {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            image: "uavred/kali-pentest:latest".to_string(),
            network_mode: NetworkMode::default(),
            volumes: Vec::new(),
            env_vars: Vec::new(),
            resource_limits: None,
        }
    }
}

/// Network mode for sandbox
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum NetworkMode {
    /// Isolated network
    Isolated,
    /// Bridge network with NAT
    Bridge,
    /// Host network (less secure)
    Host,
}

impl Default for NetworkMode {
    fn default() -> Self {
        NetworkMode::Bridge
    }
}

/// Volume mount configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeMount {
    /// Source path on host (or volume name)
    pub source: String,
    /// Destination path in sandbox
    pub destination: String,
    /// Read-only mount
    pub read_only: bool,
}

/// Resource limits for sandbox
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    /// Maximum CPU cores
    pub cpu_cores: Option<u32>,
    /// Maximum memory in MB
    pub memory_mb: Option<u64>,
    /// Maximum disk space in MB
    pub disk_mb: Option<u64>,
}
