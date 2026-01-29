//! Theme extensions - Additional color constants specific to assets_ui
//!
//! This module extends the base theme from `ui::theme` with
//! assets_ui-specific color definitions.

// Card background colors (from design system)
pub const CARD_ZONE_BG: u32 = 0xf0fdf4; // Zone card background
pub const CARD_RISK_BG: u32 = 0xfdf4ff; // Risk score card background
pub const CARD_STATUS_BG: u32 = 0xf8fafc; // Status card background
pub const CARD_BUSINESS_BG: u32 = 0xf0f9ff; // Business purpose card background
pub const CARD_COMPLIANCE_BG: u32 = 0xf0fdf4; // Compliance card background
pub const CARD_VULN_BG: u32 = 0xfef2f2; // Vulnerability card background
pub const CARD_CREDENTIALS_BG: u32 = 0xf8fafc; // Credentials card background

// Card border colors
pub const CARD_ZONE_BORDER: u32 = 0x86efac;
pub const CARD_RISK_BORDER: u32 = 0xf5d0fe;
pub const CARD_BUSINESS_BORDER: u32 = 0xbae6fd;
pub const CARD_COMPLIANCE_BORDER: u32 = 0x86efac;
pub const CARD_VULN_BORDER: u32 = 0xfecaca;
pub const CARD_DEFAULT_BORDER: u32 = 0xe2e8f0;

// Button colors
pub const BUTTON_AI_BG: u32 = 0xfdf4ff;
pub const BUTTON_AI_BORDER: u32 = 0xf5d0fe;
pub const BUTTON_AI_TEXT: u32 = 0x7c3aed;
pub const BUTTON_SCAN_BG: u32 = 0x7c3aed; // Primary purple
pub const BUTTON_SCAN_TEXT: u32 = 0xffffff;
pub const BUTTON_CONFIG_BG: u32 = 0xf1f5f9;
pub const BUTTON_CONFIG_BORDER: u32 = 0xe2e8f0;
pub const BUTTON_CONFIG_TEXT: u32 = 0x64748b;

// Severity colors (semantic aliases)
pub const SEVERITY_LOW: u32 = 0x10b981;
pub const SEVERITY_MEDIUM: u32 = 0xfbbf24;
pub const SEVERITY_HIGH: u32 = 0xf97316;
pub const SEVERITY_CRITICAL: u32 = 0xef4444;

// Selection/highlight colors
pub const SELECTION_RING: u32 = 0x7c3aed; // Purple for selected nodes
pub const CONNECTION_HIGHLIGHT: u32 = 0x7c3aed;
pub const CONNECTION_DEFAULT: u32 = 0xc0c0c0;

/// Get severity color
pub fn severity_color(severity: &data::models::Severity) -> u32 {
    use data::models::Severity;
    match severity {
        Severity::Critical => SEVERITY_CRITICAL,
        Severity::High => SEVERITY_HIGH,
        Severity::Medium => SEVERITY_MEDIUM,
        Severity::Low => SEVERITY_LOW,
        Severity::Info => 0x6b7280,
    }
}

/// Get risk color based on score (0-100)
pub fn risk_color(score: u8) -> u32 {
    match score {
        0..=20 => SEVERITY_LOW,
        21..=40 => SEVERITY_MEDIUM,
        41..=60 => SEVERITY_HIGH,
        _ => SEVERITY_CRITICAL,
    }
}

/// Get node color by asset type
pub fn node_color(asset_type: &str) -> u32 {
    match asset_type {
        "UAV" => 0x10b981,    // Green
        "GCS" => 0x2563eb,    // Blue
        "Router" => 0x10b981, // Green
        "Server" => 0x7c3aed, // Purple
        _ => 0x6b7280,        // Gray
    }
}
