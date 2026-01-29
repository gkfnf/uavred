//! Shared UI Components for assets_ui
//!
//! These are reusable components used across multiple panels.
//! Panel-specific components have been moved to their respective modules.

pub mod asset_header;
pub mod port_list;
pub mod risk_badge;
pub mod status_indicator;
pub mod topology_zone;

pub use asset_header::render_asset_header;
pub use port_list::{render_port_list, PortItem};
pub use risk_badge::render_risk_badge;
pub use status_indicator::render_status_indicator;
pub use topology_zone::{render_topology_zone_bg, TopologyZone};

// Note: Card components have been moved to asset_detail_panel/cards/
// as they are specific to the asset detail view.
