//! Sandbox UI crate - UI components for sandbox management
//!
//! This crate provides UI components for managing sandboxes in the UAVRed application.
//! It is used by the monitor_ui crate (Images Tab) to display and control sandbox instances.

pub mod components;
pub mod modals;
pub mod panels;

pub use components::*;
pub use modals::*;
pub use panels::*;
