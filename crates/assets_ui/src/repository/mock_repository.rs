//! Mock Asset Repository - provides sample data for development
//!
//! This implementation generates realistic test data without
//! requiring a database connection.

use data::models::{
    AssetNode, AssetStatus, ComplianceStandard, Connection, ScanProgress, Severity, ZoneType,
};

use super::repository::AssetRepository;

/// Mock repository with sample data
pub struct MockAssetRepository {
    assets: Vec<AssetNode>,
}

impl MockAssetRepository {
    /// Create a new mock repository with sample data
    pub fn new() -> Self {
        Self {
            assets: Self::generate_sample_assets(),
        }
    }

    /// Generate sample assets for development/testing
    fn generate_sample_assets() -> Vec<AssetNode> {
        let mut assets = vec![
            Self::create_asset(
                "gcs-1",
                "GCS Primary",
                "10.0.1.10",
                ZoneType::Z1,
                Severity::Low,
                15,
                1,
                "GCS",
                "Command",
                "Ground Control",
                AssetStatus::Online,
                vec![80, 443],
                vec![Connection {
                    target_id: "router-1".to_string(),
                    connection_type: "Data".to_string(),
                    protocol: "TCP".to_string(),
                    port: 443,
                }],
            ),
            Self::create_asset(
                "router-1",
                "Router 1",
                "10.0.2.20",
                ZoneType::Z2,
                Severity::Low,
                10,
                0,
                "Router",
                "Network",
                "Routing",
                AssetStatus::Online,
                vec![443],
                vec![Connection {
                    target_id: "server-1".to_string(),
                    connection_type: "Data".to_string(),
                    protocol: "TCP".to_string(),
                    port: 8080,
                }],
            ),
            Self::create_asset(
                "server-1",
                "Mission Control",
                "10.0.3.30",
                ZoneType::Z3,
                Severity::Medium,
                55,
                2,
                "Server",
                "IT",
                "Mission",
                AssetStatus::Online,
                vec![443, 8080],
                vec![
                    Connection {
                        target_id: "uav-1".to_string(),
                        connection_type: "Control".to_string(),
                        protocol: "MAVLink".to_string(),
                        port: 0,
                    },
                    Connection {
                        target_id: "uav-2".to_string(),
                        connection_type: "Control".to_string(),
                        protocol: "MAVLink".to_string(),
                        port: 0,
                    },
                ],
            ),
            Self::create_asset(
                "uav-1",
                "UAV Alpha",
                "192.168.1.101",
                ZoneType::Z4,
                Severity::Medium,
                45,
                2,
                "UAV",
                "Flight",
                "Surveillance",
                AssetStatus::Online,
                vec![],
                vec![Connection {
                    target_id: "emergency-1".to_string(),
                    connection_type: "Alert".to_string(),
                    protocol: "UDP".to_string(),
                    port: 0,
                }],
            ),
            Self::create_asset(
                "uav-2",
                "UAV Beta",
                "192.168.1.102",
                ZoneType::Z4,
                Severity::Low,
                30,
                1,
                "UAV",
                "Flight",
                "Surveillance",
                AssetStatus::Online,
                vec![],
                vec![],
            ),
            Self::create_asset(
                "emergency-1",
                "Emergency System",
                "10.0.5.50",
                ZoneType::Z5,
                Severity::Low,
                5,
                0,
                "Server",
                "Safety",
                "Safety",
                AssetStatus::Online,
                vec![],
                vec![],
            ),
        ];

        // Add 200 test nodes to Z2 for stress testing
        for i in 0..200 {
            let id = format!("z2-test-{}", i);
            let ip = format!("10.0.2.{}", 100 + i % 155);

            assets.push(Self::create_asset(
                &id,
                &format!("Gateway Node {}", i),
                &ip,
                ZoneType::Z2,
                match i % 5 {
                    0 => Severity::Critical,
                    1 => Severity::High,
                    2 => Severity::Medium,
                    3 => Severity::Low,
                    _ => Severity::Info,
                },
                (i % 100) as u8,
                (i % 10) as usize,
                "Router",
                &format!("Team {}", i % 8),
                "Data Relay",
                AssetStatus::Online,
                vec![80, 443, 8080, 22, 3389][..(i % 5 + 1)].to_vec(),
                if i > 0 && i % 3 == 0 {
                    vec![Connection {
                        target_id: format!("z2-test-{}", i - 1),
                        connection_type: "Data".to_string(),
                        protocol: "TCP".to_string(),
                        port: 443,
                    }]
                } else {
                    vec![]
                },
            ));
        }

        assets
    }

    #[allow(clippy::too_many_arguments)]
    fn create_asset(
        id: &str,
        name: &str,
        ip: &str,
        zone: ZoneType,
        severity: Severity,
        risk_score: u8,
        vuln_count: usize,
        asset_type: &str,
        owner: &str,
        business_purpose: &str,
        status: AssetStatus,
        open_ports: Vec<u16>,
        connections: Vec<Connection>,
    ) -> AssetNode {
        AssetNode {
            id: id.to_string(),
            name: name.to_string(),
            ip_address: ip.to_string(),
            mac_address: None,
            zone,
            severity,
            risk_score,
            vulnerabilities_count: vuln_count,
            services: vec![],
            open_ports,
            credentials: vec![],
            owner: owner.to_string(),
            business_purpose: business_purpose.to_string(),
            department: None,
            scan_progress: ScanProgress {
                percentage: 100,
                last_scan: None,
                next_scan: None,
                scan_type: "Full".to_string(),
                scanning: false,
            },
            compliance_standards: vec![
                ComplianceStandard {
                    name: "ISO 27001".to_string(),
                    status: data::models::ComplianceStatus::Compliant,
                    last_audit: None,
                },
                ComplianceStandard {
                    name: "PCI DSS".to_string(),
                    status: data::models::ComplianceStatus::Compliant,
                    last_audit: None,
                },
            ],
            connections,
            status,
            last_seen: "2024-01-13".to_string(),
            asset_type: asset_type.to_string(),
            firmware_version: None,
            manufacturer: None,
            location: None,
        }
    }
}

impl Default for MockAssetRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl AssetRepository for MockAssetRepository {
    fn get_all_assets(&self) -> Vec<AssetNode> {
        self.assets.clone()
    }

    fn get_assets_by_zone(&self, zone: ZoneType) -> Vec<AssetNode> {
        self.assets
            .iter()
            .filter(|a| a.zone == zone)
            .cloned()
            .collect()
    }

    fn get_asset_by_id(&self, id: &str) -> Option<AssetNode> {
        self.assets.iter().find(|a| a.id == id).cloned()
    }
}
