//! Asset Repository trait - defines data access interface
//!
//! This trait abstracts the data source (database, mock, API)
//! from the UI components.

use data::models::{AssetNode, ZoneType};

/// Repository trait for asset data access
///
/// Implementations:
/// - `MockAssetRepository` - Sample data for development/testing
/// - `DbAssetRepository` - Real database access (future)
pub trait AssetRepository {
    /// Get all assets
    fn get_all_assets(&self) -> Vec<AssetNode>;

    /// Get assets filtered by zone
    fn get_assets_by_zone(&self, zone: ZoneType) -> Vec<AssetNode>;

    /// Get a single asset by ID
    fn get_asset_by_id(&self, id: &str) -> Option<AssetNode>;

    /// Get total asset count
    fn get_asset_count(&self) -> usize {
        self.get_all_assets().len()
    }

    /// Get connection count between assets
    fn get_connection_count(&self) -> usize {
        self.get_all_assets()
            .iter()
            .map(|a| a.connections.len())
            .sum()
    }

    /// Search assets by name or IP
    fn search_assets(&self, query: &str) -> Vec<AssetNode> {
        let query = query.to_lowercase();
        self.get_all_assets()
            .into_iter()
            .filter(|a| {
                a.name.to_lowercase().contains(&query)
                    || a.ip_address.to_lowercase().contains(&query)
                    || a.asset_type.to_lowercase().contains(&query)
            })
            .collect()
    }
}

/// Repository provider - creates appropriate repository based on config
pub struct RepositoryProvider;

impl RepositoryProvider {
    /// Create a mock repository for development
    pub fn mock() -> MockAssetRepository {
        MockAssetRepository::new()
    }

    // TODO: Create a database repository for production
    // pub fn database() -> DbAssetRepository { ... }
}

use crate::repository::mock_repository::MockAssetRepository;
