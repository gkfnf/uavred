//! Data access layer for assets_ui
//!
//! Provides repository pattern for asset data access, enabling:
//! - Clear separation between UI and data sources
//! - Easy switching between mock and real database
//! - Async data loading patterns

pub mod mock_repository;
pub mod repository;

// Re-export commonly used types


pub use mock_repository::MockAssetRepository;
pub use repository::AssetRepository;
