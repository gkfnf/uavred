//! UI Labels - Centralized text constants for i18n
//!
//! All user-facing text should be defined here to enable:
//! - Consistent terminology
//! - Easy localization
//! - Single source of truth

/// Panel titles and headers
pub mod panel {
    pub const TOPOLOGY_TITLE: &str = "网络拓扑 - 业务层级视图";
    pub const DETAIL_TITLE: &str = "资产详情";
    pub const NO_SELECTION_MESSAGE: &str = "选择一个资产来查看详情";
    pub const ASSETS_COUNT_LABEL: &str = "资产";
    pub const CONNECTIONS_COUNT_LABEL: &str = "连接";
}

/// Severity level labels
pub mod severity {
    pub const LOW: &str = "低危";
    pub const MEDIUM: &str = "中危";
    pub const HIGH: &str = "高危";
    pub const CRITICAL: &str = "严重";
    pub const INFO: &str = "信息";
}

/// Asset detail card labels
pub mod asset_detail {
    // Card titles
    pub const RISK_SCORE: &str = "风险评分";
    pub const STATUS: &str = "状态";
    pub const OPEN_PORTS: &str = "开放端口";
    pub const DETECTED_SERVICES: &str = "检测到的服务";
    pub const CREDENTIALS: &str = "认证凭证";
    pub const BUSINESS_PURPOSE: &str = "业务用途";
    pub const OWNER_TEAM: &str = "负责人/团队";
    pub const COMPLIANCE_STANDARDS: &str = "合规标准";
    pub const VULNERABILITY_STATS: &str = "漏洞统计";
    pub const DETECTED_VULNS: &str = "已检测漏洞";

    // Status values
    pub const STATUS_ONLINE: &str = "ONLINE";
    pub const STATUS_OFFLINE: &str = "OFFLINE";
    pub const STATUS_UNKNOWN: &str = "UNKNOWN";
    pub const STATUS_MAINTENANCE: &str = "MAINTENANCE";

    // Credential info
    pub const CRED_TYPE: &str = "类型: Certificate";
    pub const CRED_VALID: &str = "有效";

    // Protocol info
    pub const PROTOCOL_LABEL: &str = "协议";
    pub const LAST_SCAN_LABEL: &str = "最后扫描";
    pub const NEVER_SCANNED: &str = "从未";

    // Default services (when none detected)
    pub const DEFAULT_SERVICES: &[&str] = &["TLS Gateway", "API Router", "Load Balancer"];
}

/// Action button labels
pub mod actions {
    pub const AI_ANALYSIS: &str = "AI 分析";
    pub const SCAN_ASSET: &str = "扫描资产";
    pub const CONFIGURE: &str = "配置";
    pub const DELETE: &str = "删除";
}

/// Compliance status labels
pub mod compliance {
    pub const COMPLIANT: &str = "合规";
    pub const NON_COMPLIANT: &str = "不合规";
    pub const PENDING: &str = "待审核";
    pub const NOT_APPLICABLE: &str = "不适用";
}

/// Asset type display names
pub mod asset_types {
    pub const UAV: &str = "无人机";
    pub const GCS: &str = "地面站";
    pub const ROUTER: &str = "路由器";
    pub const SERVER: &str = "服务器";
    pub const UNKNOWN: &str = "未知设备";
}

/// Helper function to get severity label
pub fn severity_label(severity: &data::models::Severity) -> &'static str {
    use data::models::Severity;
    match severity {
        Severity::Low => severity::LOW,
        Severity::Medium => severity::MEDIUM,
        Severity::High => severity::HIGH,
        Severity::Critical => severity::CRITICAL,
        Severity::Info => severity::INFO,
    }
}

/// Helper function to get asset status label
pub fn status_label(status: &data::models::AssetStatus) -> &'static str {
    use data::models::AssetStatus;
    match status {
        AssetStatus::Online => asset_detail::STATUS_ONLINE,
        AssetStatus::Offline => asset_detail::STATUS_OFFLINE,
        AssetStatus::Unknown => asset_detail::STATUS_UNKNOWN,
        AssetStatus::Maintenance => asset_detail::STATUS_MAINTENANCE,
        AssetStatus::Busy => "BUSY",
        AssetStatus::Error => "ERROR",
    }
}

/// Helper function to get asset type display name
pub fn asset_type_label(asset_type: &str) -> &'static str {
    match asset_type {
        "UAV" => asset_types::UAV,
        "GCS" => asset_types::GCS,
        "Router" => asset_types::ROUTER,
        "Server" => asset_types::SERVER,
        _ => asset_types::UNKNOWN,
    }
}
