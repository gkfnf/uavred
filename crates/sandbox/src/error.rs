//! Sandbox error types

use std::fmt;

/// Result type alias for sandbox operations
pub type Result<T> = std::result::Result<T, SandboxError>;

/// Error types for sandbox operations
#[derive(Debug, Clone, PartialEq)]
pub enum SandboxError {
    /// Sandbox not found
    NotFound(String),
    /// Sandbox already exists
    AlreadyExists(String),
    /// Sandbox is in invalid state for operation
    InvalidState {
        /// Sandbox identifier
        id: String,
        /// Current state
        current: String,
        /// Expected state
        expected: String,
    },
    /// Configuration error
    ConfigError(String),
    /// Runtime error
    RuntimeError(String),
    /// VNC connection error
    VncError(String),
    /// Image not found or invalid
    ImageError(String),
    /// Resource limit exceeded
    ResourceLimitExceeded(String),
    /// Operation timed out
    Timeout(String),
    /// Internal error
    Internal(String),
}

impl fmt::Display for SandboxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SandboxError::NotFound(id) => write!(f, "Sandbox not found: {}", id),
            SandboxError::AlreadyExists(id) => write!(f, "Sandbox already exists: {}", id),
            SandboxError::InvalidState {
                id,
                current,
                expected,
            } => write!(
                f,
                "Sandbox {} is in invalid state: current={}, expected={}",
                id, current, expected
            ),
            SandboxError::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
            SandboxError::RuntimeError(msg) => write!(f, "Runtime error: {}", msg),
            SandboxError::VncError(msg) => write!(f, "VNC error: {}", msg),
            SandboxError::ImageError(msg) => write!(f, "Image error: {}", msg),
            SandboxError::ResourceLimitExceeded(msg) => {
                write!(f, "Resource limit exceeded: {}", msg)
            }
            SandboxError::Timeout(msg) => write!(f, "Operation timed out: {}", msg),
            SandboxError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for SandboxError {}
