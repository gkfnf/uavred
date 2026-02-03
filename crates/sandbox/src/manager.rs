//! Sandbox manager

use crate::{
    config::SandboxConfig,
    error::{Result, SandboxError},
};
use data::models::SandboxStatus;
use std::collections::HashMap;

/// Manages multiple sandbox instances
pub struct SandboxManager {
    /// Sandboxes managed by this manager
    sandboxes: HashMap<String, SandboxInstance>,
}

/// Sandbox instance information
#[derive(Debug, Clone)]
pub struct SandboxInstance {
    /// Unique identifier
    pub id: String,
    /// Configuration
    pub config: SandboxConfig,
    /// Current status
    pub status: SandboxStatus,
    /// Creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Start timestamp (if started)
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl SandboxManager {
    /// Create a new sandbox manager
    pub fn new() -> Self {
        Self {
            sandboxes: HashMap::new(),
        }
    }

    /// Create a new sandbox instance
    pub fn create(&mut self, config: SandboxConfig) -> Result<SandboxInstance> {
        if self.sandboxes.contains_key(&config.id) {
            return Err(SandboxError::AlreadyExists(config.id));
        }

        let instance = SandboxInstance {
            id: config.id.clone(),
            config,
            status: SandboxStatus::Creating,
            created_at: chrono::Utc::now(),
            started_at: None,
        };

        self.sandboxes.insert(instance.id.clone(), instance.clone());
        Ok(instance)
    }

    /// Get a sandbox instance by ID
    pub fn get(&self, id: &str) -> Result<&SandboxInstance> {
        self.sandboxes
            .get(id)
            .ok_or_else(|| SandboxError::NotFound(id.to_string()))
    }

    /// List all sandboxes
    pub fn list(&self) -> Vec<&SandboxInstance> {
        self.sandboxes.values().collect()
    }

    /// Update sandbox status
    pub fn update_status(&mut self, id: &str, status: SandboxStatus) -> Result<()> {
        let instance = self
            .sandboxes
            .get_mut(id)
            .ok_or_else(|| SandboxError::NotFound(id.to_string()))?;
        
        instance.status = status;
        
        if status == SandboxStatus::Running {
            instance.started_at = Some(chrono::Utc::now());
        }
        
        Ok(())
    }

    /// Remove a sandbox instance
    pub fn remove(&mut self, id: &str) -> Result<()> {
        self.sandboxes
            .remove(id)
            .ok_or_else(|| SandboxError::NotFound(id.to_string()))?;
        Ok(())
    }
}

impl Default for SandboxManager {
    fn default() -> Self {
        Self::new()
    }
}
