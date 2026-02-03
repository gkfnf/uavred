// UAVRed Database Models
// Auto-generated from database schema

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// ============================================
// 1. CORE MODELS - Tasks
// ============================================

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum TaskStatus {
    Todo,
    InProgress,
    InReview,
    Done,
    Canceled,
}

impl TaskStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            TaskStatus::Todo => "todo",
            TaskStatus::InProgress => "in_progress",
            TaskStatus::InReview => "in_review",
            TaskStatus::Done => "done",
            TaskStatus::Canceled => "canceled",
        }
    }
}

impl From<&str> for TaskStatus {
    fn from(s: &str) -> Self {
        match s {
            "todo" => TaskStatus::Todo,
            "in_progress" => TaskStatus::InProgress,
            "in_review" => TaskStatus::InReview,
            "done" => TaskStatus::Done,
            "canceled" => TaskStatus::Canceled,
            _ => TaskStatus::Todo,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskPriority {
    Low,
    Medium,
    High,
    Critical,
}

impl TaskPriority {
    pub fn as_str(&self) -> &'static str {
        match self {
            TaskPriority::Low => "low",
            TaskPriority::Medium => "medium",
            TaskPriority::High => "high",
            TaskPriority::Critical => "critical",
        }
    }
}

impl From<&str> for TaskPriority {
    fn from(s: &str) -> Self {
        match s {
            "low" => TaskPriority::Low,
            "medium" => TaskPriority::Medium,
            "high" => TaskPriority::High,
            "critical" => TaskPriority::Critical,
            _ => TaskPriority::Medium,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub mission_objective: String,
    pub task_type: String,
    pub priority: TaskPriority,
    pub status: TaskStatus,
    pub assignee: String,
    pub estimated_minutes: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub closed_at: Option<DateTime<Utc>>,
    pub close_reason: String,
    pub source: String,
    pub external_ref: String,
    pub metadata: serde_json::Value,
    // Related data (not stored in DB)
    #[serde(skip)]
    pub labels: Vec<String>,
    #[serde(skip)]
    pub comments: Vec<TaskComment>,
    #[serde(skip)]
    pub dependencies: Vec<TaskDependency>,
}

impl Default for Task {
    fn default() -> Self {
        Self {
            id: 0,
            title: String::new(),
            description: String::new(),
            mission_objective: String::new(),
            task_type: "task".to_string(),
            priority: TaskPriority::Medium,
            status: TaskStatus::Todo,
            assignee: String::new(),
            estimated_minutes: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            started_at: None,
            completed_at: None,
            closed_at: None,
            close_reason: String::new(),
            source: "manual".to_string(),
            external_ref: String::new(),
            metadata: serde_json::Value::Object(serde_json::Map::new()),
            labels: Vec::new(),
            comments: Vec::new(),
            dependencies: Vec::new(),
        }
    }
}

// 转换到 workspace::TaskData (用于兼容旧 UI)
impl From<Task> for workspace::TaskData {
    fn from(task: Task) -> Self {
        Self {
            id: task.id as usize,
            title: task.title,
            task_type: task.task_type,
            priority: task.priority.as_str().to_string(),
            status: task.status.as_str().to_string(),
        }
    }
}

// 从 workspace::TaskData 转换 (用于兼容旧 UI)
impl From<workspace::TaskData> for Task {
    fn from(data: workspace::TaskData) -> Self {
        Self {
            id: data.id as i64,
            title: data.title,
            task_type: data.task_type,
            priority: TaskPriority::from(data.priority.as_str()),
            status: TaskStatus::from(data.status.as_str()),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskDependency {
    pub task_id: i64,
    pub depends_on_id: i64,
    pub dependency_type: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskComment {
    pub id: i64,
    pub task_id: i64,
    pub author: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

// ============================================
// 2. ASSETS MODELS
// ============================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AssetStatus {
    Online,
    Offline,
    Busy,
    Error,
    Maintenance,
    Unknown,
}

impl AssetStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            AssetStatus::Online => "online",
            AssetStatus::Offline => "offline",
            AssetStatus::Busy => "busy",
            AssetStatus::Error => "error",
            AssetStatus::Maintenance => "maintenance",
            AssetStatus::Unknown => "unknown",
        }
    }
}

impl From<&str> for AssetStatus {
    fn from(s: &str) -> Self {
        match s {
            "online" => AssetStatus::Online,
            "offline" => AssetStatus::Offline,
            "busy" => AssetStatus::Busy,
            "error" => AssetStatus::Error,
            "maintenance" => AssetStatus::Maintenance,
            "unknown" => AssetStatus::Unknown,
            _ => AssetStatus::Unknown,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetZone {
    pub id: String,
    pub name: String,
    pub description: String,
    pub level: i32,
    pub color: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asset {
    pub id: i64,
    pub name: String,
    pub asset_type: String,
    pub zone_id: Option<String>,
    pub ip_address: Option<String>,
    pub mac_address: Option<String>,
    pub status: AssetStatus,
    pub risk_score: i32,
    pub vuln_count: i32,
    pub model: String,
    pub firmware_version: String,
    pub protocol: String,
    pub auth_type: String,
    pub auth_status: String,
    pub auth_credential: String,
    pub business_purpose: String,
    pub owner_team: String,
    pub compliance_standards: Vec<String>,
    pub last_scan_at: Option<DateTime<Utc>>,
    pub scan_interval_minutes: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    // Network segmentation fields
    pub network_segment: String, // 所在网段，如 "192.168.1.0/24"
    pub accessible_networks: Vec<String>, // 可访问的网段列表
    // Related data
    #[serde(skip)]
    pub services: Vec<AssetService>,
    #[serde(skip)]
    pub connections: Vec<AssetConnection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetService {
    pub id: i64,
    pub asset_id: i64,
    pub port: i32,
    pub protocol: String,
    pub service_name: String,
    pub service_version: String,
    pub banner: String,
    pub is_vulnerable: bool,
    pub detected_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetConnection {
    pub id: i64,
    pub source_asset_id: i64,
    pub target_asset_id: i64,
    pub connection_type: String,
    pub protocol: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

/// Network ACL Action
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AclAction {
    Allow,
    Deny,
    Drop,
}

impl AclAction {
    pub fn as_str(&self) -> &'static str {
        match self {
            AclAction::Allow => "allow",
            AclAction::Deny => "deny",
            AclAction::Drop => "drop",
        }
    }
}

impl From<&str> for AclAction {
    fn from(s: &str) -> Self {
        match s {
            "allow" => AclAction::Allow,
            "deny" => AclAction::Deny,
            "drop" => AclAction::Drop,
            _ => AclAction::Deny,
        }
    }
}

/// Network ACL Direction
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AclDirection {
    Outbound,
    Inbound,
    Bidirectional,
}

impl AclDirection {
    pub fn as_str(&self) -> &'static str {
        match self {
            AclDirection::Outbound => "outbound",
            AclDirection::Inbound => "inbound",
            AclDirection::Bidirectional => "bidirectional",
        }
    }
}

impl From<&str> for AclDirection {
    fn from(s: &str) -> Self {
        match s {
            "outbound" => AclDirection::Outbound,
            "inbound" => AclDirection::Inbound,
            "bidirectional" => AclDirection::Bidirectional,
            _ => AclDirection::Bidirectional,
        }
    }
}

/// Network ACL - Access Control List for asset communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkAcl {
    pub id: i64,
    pub source_asset_id: i64,
    pub target_asset_id: i64,
    pub protocol: String,
    pub port_range: String,
    pub action: AclAction,
    pub direction: AclDirection,
    pub description: String,
    pub priority: i32,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

/// Network link with ACL information for topology visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkLink {
    pub source_id: i64,
    pub target_id: i64,
    pub protocol: String,
    pub port_range: String,
    pub action: AclAction,
    pub direction: AclDirection,
    pub description: String,
}

// ============================================
// 3. VULNERABILITIES MODELS
// ============================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub enum Severity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

impl Severity {
    pub fn as_str(&self) -> &'static str {
        match self {
            Severity::Info => "info",
            Severity::Low => "low",
            Severity::Medium => "medium",
            Severity::High => "high",
            Severity::Critical => "critical",
        }
    }
    
    pub fn color(&self) -> &'static str {
        match self {
            Severity::Info => "#6B7280",
            Severity::Low => "#10B981",
            Severity::Medium => "#F59E0B",
            Severity::High => "#EF4444",
            Severity::Critical => "#DC2626",
        }
    }
}

impl From<&str> for Severity {
    fn from(s: &str) -> Self {
        match s {
            "info" => Severity::Info,
            "low" => Severity::Low,
            "medium" => Severity::Medium,
            "high" => Severity::High,
            "critical" => Severity::Critical,
            _ => Severity::Medium,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vulnerability {
    pub id: String,
    pub name: String,
    pub description: String,
    pub vuln_type: String,
    pub severity: Severity,
    pub cvss_score: Option<f64>,
    pub cvss_vector: String,
    pub cve_id: String,
    pub cwe_id: String,
    pub mitre_techniques: Vec<String>,
    pub affected_systems: Vec<String>,
    pub affected_versions: String,
    pub exploit_available: bool,
    pub exploit_complexity: String,
    pub disclosure_date: Option<chrono::NaiveDate>,
    pub solution: String,
    pub ref_urls: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    pub id: i64,
    pub vuln_id: Option<String>,
    pub asset_id: i64,
    pub service_id: Option<i64>,
    pub task_id: Option<i64>,
    pub title: String,
    pub description: String,
    pub evidence: String,
    pub severity: Severity,
    pub cvss_score: Option<f64>,
    pub status: FindingStatus,
    pub ai_confidence: Option<i32>,
    pub ai_analysis: String,
    pub ai_recommendation: String,
    pub poc_code: String,
    pub poc_language: String,
    pub mitre_techniques: Vec<String>,
    pub remediation_steps: String,
    pub remediation_eta: Option<chrono::NaiveDate>,
    pub remediated_at: Option<DateTime<Utc>>,
    pub remediated_by: String,
    pub detected_at: DateTime<Utc>,
    pub detected_by: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FindingStatus {
    New,
    Validating,
    Confirmed,
    FalsePositive,
    Remediated,
    Accepted,
}

impl FindingStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            FindingStatus::New => "new",
            FindingStatus::Validating => "validating",
            FindingStatus::Confirmed => "confirmed",
            FindingStatus::FalsePositive => "false_positive",
            FindingStatus::Remediated => "remediated",
            FindingStatus::Accepted => "accepted",
        }
    }
}

impl From<&str> for FindingStatus {
    fn from(s: &str) -> Self {
        match s {
            "new" => FindingStatus::New,
            "validating" => FindingStatus::Validating,
            "confirmed" => FindingStatus::Confirmed,
            "false_positive" => FindingStatus::FalsePositive,
            "remediated" => FindingStatus::Remediated,
            "accepted" => FindingStatus::Accepted,
            _ => FindingStatus::New,
        }
    }
}

// ============================================
// 4. TRAFFIC MODELS
// ============================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Traffic {
    pub id: i64,
    pub protocol: String,
    pub method: Option<String>,
    pub path: String,
    pub src_ip: String,
    pub src_port: Option<i32>,
    pub dst_ip: String,
    pub dst_port: Option<i32>,
    pub request_headers: String,
    pub request_body: Option<Vec<u8>>,
    pub response_headers: String,
    pub response_body: Option<Vec<u8>>,
    pub response_status: Option<i32>,
    pub size_bytes: i64,
    pub duration_ms: i32,
    pub asset_id: Option<i64>,
    pub is_anomaly: bool,
    pub anomaly_type: String,
    pub anomaly_score: f64,
    pub tags: Vec<String>,
    pub captured_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficAnomaly {
    pub id: i64,
    pub traffic_id: i64,
    pub anomaly_type: String,
    pub confidence: i32,
    pub description: String,
    pub payload_sample: String,
    pub detected_at: DateTime<Utc>,
}

// ============================================
// 5. WORKFLOW MODELS
// ============================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub workflow_type: String,
    pub category: String,
    pub node_count: i32,
    pub max_parallel: i32,
    pub estimated_duration_seconds: Option<i32>,
    pub success_rate: i32,
    pub total_executions: i32,
    pub is_active: bool,
    pub is_template: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    // Related data
    #[serde(skip)]
    pub nodes: Vec<WorkflowNode>,
    #[serde(skip)]
    pub edges: Vec<WorkflowNodeEdge>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowNode {
    pub id: i64,
    pub workflow_id: i64,
    pub node_id: String,
    pub name: String,
    pub node_type: String,
    pub action: String,
    pub estimated_duration_seconds: Option<i32>,
    pub max_retries: i32,
    pub config: serde_json::Value,
    pub position_x: f64,
    pub position_y: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowNodeEdge {
    pub id: i64,
    pub workflow_id: i64,
    pub source_node_id: String,
    pub target_node_id: String,
    pub edge_type: String,
    pub condition: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowExecution {
    pub id: i64,
    pub workflow_id: i64,
    pub name: String,
    pub target_assets: Vec<i64>,
    pub status: WorkflowExecutionStatus,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub duration_seconds: Option<i32>,
    pub progress_percent: i32,
    pub nodes_completed: i32,
    pub nodes_total: i32,
    pub findings_count: i32,
    pub report_path: String,
    pub error_message: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WorkflowExecutionStatus {
    Pending,
    Running,
    Paused,
    Completed,
    Failed,
    Canceled,
}

impl WorkflowExecutionStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            WorkflowExecutionStatus::Pending => "pending",
            WorkflowExecutionStatus::Running => "running",
            WorkflowExecutionStatus::Paused => "paused",
            WorkflowExecutionStatus::Completed => "completed",
            WorkflowExecutionStatus::Failed => "failed",
            WorkflowExecutionStatus::Canceled => "canceled",
        }
    }
}

impl From<&str> for WorkflowExecutionStatus {
    fn from(s: &str) -> Self {
        match s {
            "pending" => WorkflowExecutionStatus::Pending,
            "running" => WorkflowExecutionStatus::Running,
            "paused" => WorkflowExecutionStatus::Paused,
            "completed" => WorkflowExecutionStatus::Completed,
            "failed" => WorkflowExecutionStatus::Failed,
            "canceled" => WorkflowExecutionStatus::Canceled,
            _ => WorkflowExecutionStatus::Pending,
        }
    }
}

// ============================================
// 6. AGENTS MODELS
// ============================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentImage {
    pub id: i64,
    pub name: String,
    pub version: String,
    pub description: String,
    pub image_type: String,
    pub docker_image: String,
    pub capabilities: Vec<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub id: i64,
    pub name: String,
    pub image_id: Option<i64>,
    pub container_id: String,
    pub docker_exec_command: String,
    pub status: AgentStatus,
    pub current_task_id: Option<i64>,
    pub current_task_name: String,
    pub cpu_percent: f64,
    pub memory_percent: f64,
    pub memory_mb: i64,
    pub exposed_ports: Vec<i32>,
    pub started_at: Option<DateTime<Utc>>,
    pub running_duration_seconds: i64,
    pub tasks_completed: i32,
    pub live_trace: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AgentStatus {
    Running,
    Stopped,
    Building,
    Error,
}

impl AgentStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            AgentStatus::Running => "running",
            AgentStatus::Stopped => "stopped",
            AgentStatus::Building => "building",
            AgentStatus::Error => "error",
        }
    }
}

impl From<&str> for AgentStatus {
    fn from(s: &str) -> Self {
        match s {
            "running" => AgentStatus::Running,
            "stopped" => AgentStatus::Stopped,
            "building" => AgentStatus::Building,
            "error" => AgentStatus::Error,
            _ => AgentStatus::Stopped,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentLog {
    pub id: i64,
    pub agent_id: i64,
    pub log_level: String,
    pub message: String,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

// ============================================
// 7. DEVICES MODELS
// ============================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    pub id: i64,
    pub name: String,
    pub device_type: String,
    pub serial_number: String,
    pub firmware_version: String,
    pub device_path: String,
    pub status: DeviceStatus,
    pub frequency_hz: i64,
    pub sample_rate: i64,
    pub bandwidth_hz: i64,
    pub gain_db: i32,
    pub temperature_celsius: Option<f64>,
    pub total_runtime_seconds: i64,
    pub tasks_completed: i32,
    pub last_used_at: Option<DateTime<Utc>>,
    pub current_operation: String,
    pub current_task_id: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DeviceStatus {
    Connected,
    Busy,
    Ready,
    Error,
    Disconnected,
}

impl DeviceStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            DeviceStatus::Connected => "connected",
            DeviceStatus::Busy => "busy",
            DeviceStatus::Ready => "ready",
            DeviceStatus::Error => "error",
            DeviceStatus::Disconnected => "disconnected",
        }
    }
}

impl From<&str> for DeviceStatus {
    fn from(s: &str) -> Self {
        match s {
            "connected" => DeviceStatus::Connected,
            "busy" => DeviceStatus::Busy,
            "ready" => DeviceStatus::Ready,
            "error" => DeviceStatus::Error,
            "disconnected" => DeviceStatus::Disconnected,
            _ => DeviceStatus::Disconnected,
        }
    }
}

// ============================================
// 8. SETTINGS MODELS
// ============================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Setting {
    pub key: String,
    pub value: String,
    pub value_type: String,
    pub description: String,
    pub category: String,
    pub is_editable: bool,
    pub updated_at: DateTime<Utc>,
}

// ============================================
// 9. AUDIT LOG MODELS
// ============================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLog {
    pub id: i64,
    pub action: String,
    pub entity_type: String,
    pub entity_id: Option<String>,
    pub actor_type: String,
    pub actor_id: String,
    pub actor_name: String,
    pub description: String,
    pub old_value: String,
    pub new_value: String,
    pub ip_address: String,
    pub user_agent: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemEvent {
    pub id: i64,
    pub event_type: String,
    pub source: String,
    pub message: String,
    pub details: String,
    pub stack_trace: String,
    pub created_at: DateTime<Utc>,
}

// ============================================
// 10. VIEW MODELS (Aggregated Data)
// ============================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetRiskSummary {
    pub id: i64,
    pub name: String,
    pub asset_type: String,
    pub zone_id: Option<String>,
    pub risk_score: i32,
    pub total_findings: i32,
    pub critical_count: i32,
    pub high_count: i32,
    pub medium_count: i32,
    pub service_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecentActivity {
    pub entity_type: String,
    pub entity_id: String,
    pub description: String,
    pub status: String,
    pub activity_time: DateTime<Utc>,
}

// ============================================
// 11. ASSETS UI SPECIFIC MODELS
// ============================================

/// Zone type for topology view
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum ZoneType {
    Z1,
    Z2,
    Z3,
    Z4,
    Z5,
}

impl ZoneType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ZoneType::Z1 => "Z1",
            ZoneType::Z2 => "Z2",
            ZoneType::Z3 => "Z3",
            ZoneType::Z4 => "Z4",
            ZoneType::Z5 => "Z5",
        }
    }
}

impl From<&str> for ZoneType {
    fn from(s: &str) -> Self {
        match s {
            "Z1" => ZoneType::Z1,
            "Z2" => ZoneType::Z2,
            "Z3" => ZoneType::Z3,
            "Z4" => ZoneType::Z4,
            "Z5" => ZoneType::Z5,
            _ => ZoneType::Z1,
        }
    }
}

/// Compliance status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ComplianceStatus {
    Compliant,
    NonCompliant,
    Pending,
    NotApplicable,
}

impl ComplianceStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            ComplianceStatus::Compliant => "compliant",
            ComplianceStatus::NonCompliant => "non_compliant",
            ComplianceStatus::Pending => "pending",
            ComplianceStatus::NotApplicable => "not_applicable",
        }
    }
}

impl From<&str> for ComplianceStatus {
    fn from(s: &str) -> Self {
        match s {
            "compliant" => ComplianceStatus::Compliant,
            "non_compliant" => ComplianceStatus::NonCompliant,
            "pending" => ComplianceStatus::Pending,
            "not_applicable" => ComplianceStatus::NotApplicable,
            _ => ComplianceStatus::Pending,
        }
    }
}

/// Compliance standard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceStandard {
    pub name: String,
    pub status: ComplianceStatus,
    pub last_audit: Option<DateTime<Utc>>,
}

/// Scan progress information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanProgress {
    pub percentage: i32,
    pub last_scan: Option<DateTime<Utc>>,
    pub next_scan: Option<DateTime<Utc>>,
    pub scan_type: String,
    pub scanning: bool,
}

/// Connection information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connection {
    pub target_id: String,
    pub connection_type: String,
    pub protocol: String,
    pub port: u16,
}

/// AssetNode - UI-specific asset representation for topology view
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetNode {
    pub id: String,
    pub name: String,
    pub ip_address: String,
    pub mac_address: Option<String>,
    pub zone: ZoneType,
    pub severity: Severity,
    pub risk_score: i32,
    pub vulnerabilities_count: i32,
    pub services: Vec<String>,
    pub open_ports: Vec<u16>,
    pub credentials: Vec<String>,
    pub owner: String,
    pub business_purpose: String,
    pub department: Option<String>,
    pub scan_progress: ScanProgress,
    pub compliance_standards: Vec<ComplianceStandard>,
    pub connections: Vec<Connection>,
    pub status: AssetStatus,
    pub last_seen: String,
    pub asset_type: String,
    pub firmware_version: Option<String>,
    pub manufacturer: Option<String>,
    pub location: Option<String>,
}

// ============================================
// 12. VULNERABILITY UI SPECIFIC MODELS
// ============================================

/// Vulnerability severity for UI
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, PartialOrd, Ord, Eq)]
pub enum VulnSeverity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

impl VulnSeverity {
    pub fn as_str(&self) -> &'static str {
        match self {
            VulnSeverity::Info => "info",
            VulnSeverity::Low => "low",
            VulnSeverity::Medium => "medium",
            VulnSeverity::High => "high",
            VulnSeverity::Critical => "critical",
        }
    }

    pub fn color(&self) -> &'static str {
        match self {
            VulnSeverity::Info => "#6B7280",
            VulnSeverity::Low => "#10B981",
            VulnSeverity::Medium => "#F59E0B",
            VulnSeverity::High => "#EF4444",
            VulnSeverity::Critical => "#DC2626",
        }
    }
}

impl From<&str> for VulnSeverity {
    fn from(s: &str) -> Self {
        match s {
            "info" => VulnSeverity::Info,
            "low" => VulnSeverity::Low,
            "medium" => VulnSeverity::Medium,
            "high" => VulnSeverity::High,
            "critical" => VulnSeverity::Critical,
            _ => VulnSeverity::Medium,
        }
    }
}

/// Vulnerability status
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum VulnStatus {
    New,
    Confirmed,
    FalsePositive,
    Remediated,
    Accepted,
}

impl VulnStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            VulnStatus::New => "new",
            VulnStatus::Confirmed => "confirmed",
            VulnStatus::FalsePositive => "false_positive",
            VulnStatus::Remediated => "remediated",
            VulnStatus::Accepted => "accepted",
        }
    }
}

/// CVSS Score
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CvssScore {
    pub score: f64,
    pub base_score: f64,
    pub vector: String,
}

impl CvssScore {
    pub fn new(score: f64, vector: String) -> Self {
        Self { score, base_score: score, vector }
    }
}

/// Detection source
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum DetectionSource {
    AiAnalysis,
    AutomatedScanner,
    DynamicAnalysis,
    StaticAnalysis,
    Manual,
}

/// Scan type
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum ScanType {
    StaticCodeAnalysis,
    Network,
    Protocol,
    Firmware,
}

/// Exploit maturity
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum ExploitMaturity {
    Unproven,
    ProofOfConcept,
    Functional,
    High,
}

/// AI Security Analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiSecurityAnalysis {
    pub confidence_score: f64,
    pub risk_score: f64,
    pub analysis_type: String,
    pub reasoning: String,
    pub recommendations: Vec<String>,
    pub false_positive_probability: f64,
    pub model_version: String,
    pub analyzed_at: String,
}

/// Detection location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectionLocation {
    pub component: String,
    pub file_path: Option<String>,
    pub line_number: Option<i32>,
    pub function: Option<String>,
    pub source: DetectionSource,
}

/// Vulnerability Data - Main UI model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VulnData {
    pub id: String,
    pub title: String,
    pub description: String,
    pub cve: Option<String>,
    pub cwe: Option<String>,
    pub severity: VulnSeverity,
    pub status: VulnStatus,
    pub cvss: Option<CvssScore>,
    pub affected: String,
    pub affected_systems: Vec<String>,
    pub detection_time: String,
    pub detection_location: DetectionLocation,
    pub scan_type: ScanType,
    pub exploit_available: bool,
    pub poc_available: bool,
    pub exploit_maturity: Option<ExploitMaturity>,
    pub attack_tactics: Vec<String>,
    pub attack_techniques: Vec<String>,
    pub attack_subtechniques: Vec<String>,
    pub ai_analysis: Option<AiSecurityAnalysis>,
    pub references: Vec<String>,
    pub tags: Vec<String>,
}

// ============================================
// SANDBOX MODELS
// ============================================

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum SandboxStatus {
    Creating,
    Starting,
    Running,
    Stopping,
    Stopped,
    Error,
}

impl SandboxStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            SandboxStatus::Creating => "creating",
            SandboxStatus::Starting => "starting",
            SandboxStatus::Running => "running",
            SandboxStatus::Stopping => "stopping",
            SandboxStatus::Stopped => "stopped",
            SandboxStatus::Error => "error",
        }
    }
}

impl From<&str> for SandboxStatus {
    fn from(s: &str) -> Self {
        match s {
            "creating" => SandboxStatus::Creating,
            "starting" => SandboxStatus::Starting,
            "running" => SandboxStatus::Running,
            "stopping" => SandboxStatus::Stopping,
            "stopped" => SandboxStatus::Stopped,
            "error" => SandboxStatus::Error,
            _ => SandboxStatus::Creating,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum SandboxImageType {
    KaliLinux,
    UbuntuTools,
    Custom,
}

impl SandboxImageType {
    pub fn as_str(&self) -> &'static str {
        match self {
            SandboxImageType::KaliLinux => "kali_linux",
            SandboxImageType::UbuntuTools => "ubuntu_tools",
            SandboxImageType::Custom => "custom",
        }
    }
}

impl From<&str> for SandboxImageType {
    fn from(s: &str) -> Self {
        match s {
            "kali_linux" => SandboxImageType::KaliLinux,
            "ubuntu_tools" => SandboxImageType::UbuntuTools,
            "custom" => SandboxImageType::Custom,
            _ => SandboxImageType::KaliLinux,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum SandboxImageStatus {
    Available,
    Downloading,
    Building,
    Error,
}

impl SandboxImageStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            SandboxImageStatus::Available => "available",
            SandboxImageStatus::Downloading => "downloading",
            SandboxImageStatus::Building => "building",
            SandboxImageStatus::Error => "error",
        }
    }
}

impl From<&str> for SandboxImageStatus {
    fn from(s: &str) -> Self {
        match s {
            "available" => SandboxImageStatus::Available,
            "downloading" => SandboxImageStatus::Downloading,
            "building" => SandboxImageStatus::Building,
            "error" => SandboxImageStatus::Error,
            _ => SandboxImageStatus::Available,
        }
    }
}
