//! VNC connection handling

use serde::{Deserialize, Serialize};

/// VNC connection information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VncConnectionInfo {
    /// Host address
    pub host: String,
    /// Port number
    pub port: u16,
    /// Password (optional)
    pub password: Option<String>,
    /// Connection quality settings
    pub quality: VncQuality,
}

impl Default for VncConnectionInfo {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 5900,
            password: None,
            quality: VncQuality::default(),
        }
    }
}

/// VNC connection quality settings
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum VncQuality {
    /// Low quality (fast)
    Low,
    /// Medium quality (balanced)
    Medium,
    /// High quality (slower)
    High,
}

impl Default for VncQuality {
    fn default() -> Self {
        VncQuality::Medium
    }
}

/// VNC frame data
#[derive(Debug, Clone)]
pub struct VncFrame {
    /// Frame width
    pub width: u32,
    /// Frame height
    pub height: u32,
    /// Raw pixel data (RGBA)
    pub data: Vec<u8>,
    /// Timestamp when frame was captured
    pub timestamp: u64,
}

impl VncFrame {
    /// Create a new VNC frame
    pub fn new(width: u32, height: u32, data: Vec<u8>) -> Self {
        Self {
            width,
            height,
            data,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
        }
    }

    /// Calculate data size in bytes
    pub fn data_size(&self) -> usize {
        self.data.len()
    }
}
