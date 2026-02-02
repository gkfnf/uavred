//! DbAssetRepository - Real database access for production
//!
//! This implementation uses the data crate's AssetStore to access
//! real asset data from the SQLite database.

use data::models::{AssetNode, ZoneType, Asset, AssetStatus, Severity, Connection, ScanProgress, ComplianceStandard, ComplianceStatus};
use data::{AssetStore, init_and_load_asset_store};
use gpui::App;
use std::sync::Arc;

use super::repository::AssetRepository;

/// Database-backed asset repository
pub struct DbAssetRepository {
    // We don't store the AssetStore directly because it's an Entity
    // Instead, we'll use a callback pattern or store assets locally after loading
    cached_assets: Vec<AssetNode>,
}

impl DbAssetRepository {
    /// Create a new database repository and load assets from database
    /// 
    /// Note: This should be called after init_asset_store() has been called in the App
    pub fn new(cx: &mut App) -> Self {
        // Initialize asset store if not already done
        init_and_load_asset_store(cx);
        
        // Load assets from the global store
        let asset_store = AssetStore::global(cx);
        let assets: Vec<Asset> = asset_store.read(cx).get_all_assets();
        
        // Convert to AssetNode
        let cached_assets: Vec<AssetNode> = assets.into_iter()
            .map(|asset| Self::asset_to_node(asset, &[]))
            .collect();
        
        Self { cached_assets }
    }

    /// Create a new database repository with explicit asset store entity
    /// 
    /// Use this when you have access to the AssetStore entity
    pub fn with_store(asset_store: &data::AssetStore) -> Self {
        let assets = asset_store.get_all_assets();
        let cached_assets: Vec<AssetNode> = assets.into_iter()
            .map(|asset| Self::asset_to_node(asset, &[]))
            .collect();
        
        Self { cached_assets }
    }

    /// Refresh assets from database
    pub fn refresh(&mut self, cx: &mut App) {
        let asset_store = AssetStore::global(cx);
        let assets: Vec<Asset> = asset_store.read(cx).get_all_assets();
        
        self.cached_assets = assets.into_iter()
            .map(|asset| Self::asset_to_node(asset, &[]))
            .collect();
    }

    /// Convert Asset to AssetNode for UI display
    fn asset_to_node(asset: Asset, all_assets: &[Asset]) -> AssetNode {
        // Determine severity based on risk score
        let severity = if asset.risk_score >= 70 {
            Severity::High
        } else if asset.risk_score >= 40 {
            Severity::Medium
        } else {
            Severity::Low
        };

        // Parse zone from zone_id
        let zone = asset.zone_id.as_deref()
            .map(ZoneType::from)
            .unwrap_or(ZoneType::Z1);

        // Convert services to strings
        let services: Vec<String> = asset.services.iter()
            .map(|s| format!("{}:{}", s.service_name, s.port))
            .collect();

        // Extract open ports
        let open_ports: Vec<u16> = asset.services.iter()
            .filter(|s| s.port > 0 && s.port <= 65535)
            .map(|s| s.port as u16)
            .collect();

        // Convert connections - need to look up target asset IDs
        let connections: Vec<Connection> = asset.connections.iter()
            .filter_map(|c| {
                // Determine if this asset is source or target
                let is_source = c.source_asset_id == asset.id;
                let other_asset_id = if is_source { c.target_asset_id } else { c.source_asset_id };
                
                // Look up the other asset to get its string ID
                // In database, we use integer IDs, but UI expects string IDs
                let target_id = all_assets.iter()
                    .find(|a| a.id == other_asset_id)
                    .map(|a| a.id.to_string())
                    .unwrap_or_else(|| other_asset_id.to_string());

                Some(Connection {
                    target_id,
                    connection_type: c.connection_type.clone(),
                    protocol: c.protocol.clone(),
                    port: 0, // Could look up from services
                })
            })
            .collect();

        // Build compliance standards
        let compliance_standards: Vec<ComplianceStandard> = asset.compliance_standards.iter()
            .map(|name| ComplianceStandard {
                name: name.clone(),
                status: ComplianceStatus::Compliant,
                last_audit: None,
            })
            .collect();

        AssetNode {
            id: asset.id.to_string(),
            name: asset.name,
            ip_address: asset.ip_address.unwrap_or_default(),
            mac_address: asset.mac_address,
            zone,
            severity,
            risk_score: asset.risk_score,
            vulnerabilities_count: asset.vuln_count,
            services,
            open_ports,
            credentials: Vec::new(),
            owner: asset.owner_team,
            business_purpose: asset.business_purpose,
            department: None,
            scan_progress: ScanProgress {
                percentage: 100,
                last_scan: asset.last_scan_at,
                next_scan: None,
                scan_type: "Full".to_string(),
                scanning: false,
            },
            compliance_standards,
            connections,
            status: asset.status,
            last_seen: asset.updated_at.format("%Y-%m-%d %H:%M").to_string(),
            asset_type: asset.asset_type,
            firmware_version: if asset.firmware_version.is_empty() { None } else { Some(asset.firmware_version) },
            manufacturer: if asset.model.is_empty() { None } else { Some(asset.model) },
            location: None,
        }
    }
}

impl AssetRepository for DbAssetRepository {
    fn get_all_assets(&self) -> Vec<AssetNode> {
        self.cached_assets.clone()
    }

    fn get_assets_by_zone(&self, zone: ZoneType) -> Vec<AssetNode> {
        let zone_str = zone.as_str();
        self.cached_assets
            .iter()
            .filter(|a| {
                // Match zone by comparing the zone field
                a.zone == zone
            })
            .cloned()
            .collect()
    }

    fn get_asset_by_id(&self, id: &str) -> Option<AssetNode> {
        self.cached_assets
            .iter()
            .find(|a| a.id == id)
            .cloned()
    }
}

/// Repository provider extension for database repository
impl super::repository::RepositoryProvider {
    /// Create a database-backed repository
    pub fn database(cx: &mut App) -> DbAssetRepository {
        DbAssetRepository::new(cx)
    }
}
