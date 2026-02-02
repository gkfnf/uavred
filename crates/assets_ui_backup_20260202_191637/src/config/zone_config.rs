//! Zone configuration - centralized zone metadata
//!
//! This module provides a single source of truth for zone-related
//! display properties, separating configuration from data models.

use data::models::ZoneType;

/// Complete configuration for a security zone
#[derive(Clone, Copy, Debug)]
pub struct ZoneConfig {
    /// Short identifier (Z1, Z2, etc.)
    pub short_name: &'static str,
    /// Chinese display name for the zone layer
    pub layer_name: &'static str,
    /// Network classification description
    pub network_type: &'static str,
    /// Primary brand color for the zone
    pub primary_color: u32,
    /// Background color for zone areas
    pub bg_color: u32,
    /// Light variant of primary color for highlights
    pub light_color: u32,
}

/// Get configuration for a specific zone type
pub fn get_zone_config(zone: &ZoneType) -> &'static ZoneConfig {
    match zone {
        ZoneType::Z1 => &ZoneConfig {
            short_name: "Z1",
            layer_name: "地面指挥中心",
            network_type: "外部网络",
            primary_color: 0x2563eb, // Blue
            bg_color: 0xe8f4ff,
            light_color: 0x60a5fa,
        },
        ZoneType::Z2 => &ZoneConfig {
            short_name: "Z2",
            layer_name: "通信网关层",
            network_type: "DMZ",
            primary_color: 0x10b981, // Green
            bg_color: 0xe8f8ed,
            light_color: 0x34d399,
        },
        ZoneType::Z3 => &ZoneConfig {
            short_name: "Z3",
            layer_name: "任务控制层",
            network_type: "业务网络",
            primary_color: 0x7c3aed, // Purple
            bg_color: 0xf0ebff,
            light_color: 0xa78bfa,
        },
        ZoneType::Z4 => &ZoneConfig {
            short_name: "Z4",
            layer_name: "飞控设备层",
            network_type: "设备通信层",
            primary_color: 0xf97316, // Orange
            bg_color: 0xfff8e1,
            light_color: 0xfb923c,
        },
        ZoneType::Z5 => &ZoneConfig {
            short_name: "Z5",
            layer_name: "安全应急系统",
            network_type: "应急系统",
            primary_color: 0xef4444, // Red
            bg_color: 0xffebee,
            light_color: 0xf87171,
        },
    }
}

/// Extension trait for ZoneType to access config
pub trait ZoneTypeExt {
    fn config(&self) -> &'static ZoneConfig;
    fn short_name(&self) -> &'static str;
    fn layer_name(&self) -> &'static str;
    fn network_type(&self) -> &'static str;
    fn primary_color(&self) -> u32;
    fn bg_color(&self) -> u32;
}

impl ZoneTypeExt for ZoneType {
    fn config(&self) -> &'static ZoneConfig {
        get_zone_config(self)
    }

    fn short_name(&self) -> &'static str {
        self.config().short_name
    }

    fn layer_name(&self) -> &'static str {
        self.config().layer_name
    }

    fn network_type(&self) -> &'static str {
        self.config().network_type
    }

    fn primary_color(&self) -> u32 {
        self.config().primary_color
    }

    fn bg_color(&self) -> u32 {
        self.config().bg_color
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zone_config_consistency() {
        for zone in [ZoneType::Z1, ZoneType::Z2, ZoneType::Z3, ZoneType::Z4, ZoneType::Z5] {
            let config = zone.config();
            assert!(!config.short_name.is_empty());
            assert!(!config.layer_name.is_empty());
            assert!(!config.network_type.is_empty());
            assert!(config.primary_color > 0);
            assert!(config.bg_color > 0);
        }
    }

    #[test]
    fn test_zone_ext_trait() {
        let z1 = ZoneType::Z1;
        assert_eq!(z1.short_name(), "Z1");
        assert_eq!(z1.layer_name(), "地面指挥中心");
        assert_eq!(z1.primary_color(), 0x2563eb);
    }
}
