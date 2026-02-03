//! Sandbox crate - BoxLite integration for UAVRed
//!
//! This crate provides sandbox management functionality using BoxLite.
//! It is used by the monitor_ui crate (Images Tab) to manage agent execution environments.

pub mod config;
pub mod error;
pub mod manager;
pub mod runtime;
pub mod vnc;

pub use config::{NetworkMode, ResourceLimits, SandboxConfig, VolumeMount};
pub use error::{Result, SandboxError};
pub use manager::{SandboxInstance, SandboxManager};
pub use runtime::{BoxLiteRuntime, RuntimeConfig, SandboxRuntime};
pub use vnc::{VncConnectionInfo, VncFrame, VncQuality};
