// Core data modules
pub mod models;
pub mod repository;
pub mod task_store;
pub mod vuln_store;
pub mod traffic_store;

// New UAVRed database
pub mod uavred_db;

// Re-export models
pub use models::*;
pub use repository::*;

// Re-export new database
pub use uavred_db::*;

// Re-export TaskStore (primary interface for UI)
pub use task_store::{
    TaskStore,
    TaskStoreEvent,
    init_task_store,
    init_and_load_task_store,
    DashboardStats
};

// Re-export VulnStore (for vulns_ui)
pub use vuln_store::{
    VulnStore,
    VulnStoreEvent,
    init_vuln_store,
    init_and_load_vuln_store,
};

// Re-export TrafficStore (for traffic_ui)
pub use traffic_store::{
    TrafficStore,
    TrafficStoreEvent,
    TrafficStats,
    init_traffic_store,
    init_and_load_traffic_store,
};

// Re-export workspace types for convenience
pub use workspace::TaskData;
