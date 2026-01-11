// 数据模型定义

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use workspace::TaskData as WorkspaceTaskData;

/// 任务状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus {
    Todo,
    InProgress,
    Done,
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskStatus::Todo => write!(f, "todo"),
            TaskStatus::InProgress => write!(f, "in_progress"),
            TaskStatus::Done => write!(f, "done"),
        }
    }
}

/// 任务数据模型（扩展 workspace::TaskData，添加 status 字段）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskData {
    pub id: usize,
    pub title: String,
    pub task_type: String,
    pub priority: String,
    pub status: TaskStatus,
}

impl TaskData {
    pub fn new(
        id: usize,
        title: String,
        task_type: String,
        priority: String,
        status: TaskStatus,
    ) -> Self {
        Self {
            id,
            title,
            task_type,
            priority,
            status,
        }
    }

    /// 从 workspace::TaskData 转换
    pub fn from_workspace(task: &WorkspaceTaskData, status: TaskStatus) -> Self {
        Self {
            id: task.id,
            title: task.title.clone(),
            task_type: task.task_type.clone(),
            priority: task.priority.clone(),
            status,
        }
    }

    /// 转换为 workspace::TaskData
    pub fn to_workspace(&self) -> WorkspaceTaskData {
        WorkspaceTaskData::new(
            self.id,
            self.title.clone(),
            self.task_type.clone(),
            self.priority.clone(),
            self.status.to_string(),
        )
    }
}

/// 漏洞数据模型
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum VulnSeverity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum VulnStatus {
    New,
    Validating,
    Confirmed,
    FalsePositive,
    Mitigated,
    Resolved,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CvssScore {
    pub base_score: f64,
    pub base_severity: VulnSeverity,
    pub temporal_score: Option<f64>,
    pub environmental_score: Option<f64>,
    pub vector_string: String,
    pub exploitability: Option<f64>,
    pub impact: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectionLocation {
    pub component: String,
    pub file_path: Option<String>,
    pub line_number: Option<usize>,
    pub function: Option<String>,
    pub source: DetectionSource,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DetectionSource {
    StaticAnalysis,
    DynamicAnalysis,
    ManualReview,
    AutomatedScanner,
    AiAnalysis,
    ThreatIntelligence,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScanType {
    Network,
    Protocol,
    Firmware,
    StaticCodeAnalysis,
    DynamicAnalysis,
    PenetrationTest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExploitMaturity {
    None,
    High,
    Functional,
    ProofOfConcept,
    Unreported,
}

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

impl VulnData {
    pub fn new(id: String, title: String, description: String, severity: VulnSeverity) -> Self {
        Self {
            id,
            title,
            description,
            severity,
            status: VulnStatus::New,
            cve: None,
            cwe: None,
            cvss: None,
            affected: String::new(),
            affected_systems: Vec::new(),
            detection_time: String::new(),
            detection_location: DetectionLocation {
                component: String::new(),
                file_path: None,
                line_number: None,
                function: None,
                source: DetectionSource::AutomatedScanner,
            },
            scan_type: ScanType::Network,
            exploit_available: false,
            poc_available: false,
            exploit_maturity: None,
            attack_tactics: Vec::new(),
            attack_techniques: Vec::new(),
            attack_subtechniques: Vec::new(),
            ai_analysis: None,
            references: Vec::new(),
            tags: Vec::new(),
        }
    }

    pub fn is_high_severity(&self) -> bool {
        matches!(self.severity, VulnSeverity::Critical | VulnSeverity::High)
    }

    pub fn has_exploit(&self) -> bool {
        self.exploit_available && self.exploit_maturity.is_some()
    }
}

impl CvssScore {
    pub fn new(base_score: f64, vector_string: String) -> Self {
        let base_severity = match base_score {
            s if s >= 9.0 => VulnSeverity::Critical,
            s if s >= 7.0 => VulnSeverity::High,
            s if s >= 4.0 => VulnSeverity::Medium,
            s if s > 0.0 => VulnSeverity::Low,
            _ => VulnSeverity::Info,
        };

        Self {
            base_score,
            base_severity,
            temporal_score: None,
            environmental_score: None,
            vector_string,
            exploitability: None,
            impact: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Protocol {
    HTTP,
    HTTPS,
    FTP,
    TELNET,
    MAVLink,
    DJI,
    Unknown,
}

impl std::fmt::Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Protocol::HTTP => write!(f, "HTTP"),
            Protocol::HTTPS => write!(f, "HTTPS"),
            Protocol::FTP => write!(f, "FTP"),
            Protocol::TELNET => write!(f, "TELNET"),
            Protocol::MAVLink => write!(f, "MAVLink"),
            Protocol::DJI => write!(f, "DJI"),
            Protocol::Unknown => write!(f, "UNKNOWN"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    OPTIONS,
    CONNECT,
    TRACE,
    Unknown,
}

impl std::fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpMethod::GET => write!(f, "GET"),
            HttpMethod::POST => write!(f, "POST"),
            HttpMethod::PUT => write!(f, "PUT"),
            HttpMethod::DELETE => write!(f, "DELETE"),
            HttpMethod::PATCH => write!(f, "PATCH"),
            HttpMethod::HEAD => write!(f, "HEAD"),
            HttpMethod::OPTIONS => write!(f, "OPTIONS"),
            HttpMethod::CONNECT => write!(f, "CONNECT"),
            HttpMethod::TRACE => write!(f, "TRACE"),
            HttpMethod::Unknown => write!(f, "UNKNOWN"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AnomalyType {
    StatusAnomaly,
    LatencyAnomaly,
    SizeAnomaly,
    PatternAnomaly,
    ResponseAnomaly,
    TLSIssue,
    ProtocolViolation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TrafficSource {
    Intercept,
    Replay,
    Workflow,
    Plugin,
    Import,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficEntry {
    pub id: i64,
    pub created_at: String,
    pub method: Option<HttpMethod>,
    pub path: String,
    pub query: Option<String>,
    pub host: String,
    pub port: u16,
    pub protocol: Protocol,
    pub tls: bool,
    pub extension: Option<String>,
    pub status: u16,
    pub status_text: Option<String>,
    pub duration_ms: u64,
    pub request_size: u64,
    pub response_size: u64,
    pub asset_id: String,
    pub asset_name: String,
    pub target_ip: Option<String>,
    pub source_ip: Option<String>,
    pub source_port: Option<u16>,
    pub request_raw: Option<String>,
    pub request_headers: Vec<(String, String)>,
    pub request_body: Option<String>,
    pub response_raw: Option<String>,
    pub response_headers: Vec<(String, String)>,
    pub response_body: Option<String>,
    pub source: TrafficSource,
    pub task_id: Option<i64>,
    pub workspace_id: Option<String>,
    pub flow_id: Option<String>,
    pub anomalies: Vec<AnomalyType>,
    pub anomaly_description: Option<String>,
    pub is_live: bool,
    pub finding_ids: Vec<i64>,
    pub artifact_ids: Vec<i64>,
    pub notes: Option<String>,
    pub is_exported: bool,
}

impl TrafficEntry {
    pub fn new(id: i64, created_at: String) -> Self {
        Self {
            id,
            created_at,
            method: None,
            path: String::new(),
            query: None,
            host: String::new(),
            port: 80,
            protocol: Protocol::Unknown,
            tls: false,
            extension: None,
            status: 0,
            status_text: None,
            duration_ms: 0,
            request_size: 0,
            response_size: 0,
            asset_id: String::new(),
            asset_name: String::new(),
            target_ip: None,
            source_ip: None,
            source_port: None,
            request_raw: None,
            request_headers: Vec::new(),
            request_body: None,
            response_raw: None,
            response_headers: Vec::new(),
            response_body: None,
            source: TrafficSource::Intercept,
            task_id: None,
            workspace_id: None,
            flow_id: None,
            anomalies: Vec::new(),
            anomaly_description: None,
            is_live: false,
            finding_ids: Vec::new(),
            artifact_ids: Vec::new(),
            notes: None,
            is_exported: false,
        }
    }

    pub fn get_field(&self, field: &str) -> Option<String> {
        match field {
            "method" | "req.method" => self.method.map(|m| m.to_string()),
            "path" | "req.path" => Some(self.path.clone()),
            "status" | "resp.code" => Some(self.status.to_string()),
            "proto" | "protocol" => Some(self.protocol.to_string()),
            "time" => Some(self.created_at.clone()),
            "asset" => Some(self.asset_name.clone()),
            "size" => Some(self.response_size.to_string()),
            "duration" | "resp.roundtrip" => Some(self.duration_ms.to_string()),
            "req.host" => Some(self.host.clone()),
            "req.port" => Some(self.port.to_string()),
            "req.query" => self.query.clone(),
            "req.tls" => Some(self.tls.to_string()),
            "req.ext" => self.extension.clone(),
            "req.len" => Some(self.request_size.to_string()),
            "resp.len" => Some(self.response_size.to_string()),
            "row.id" => Some(self.id.to_string()),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NodeType {
    Atomic,
    Composite,
    Task,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ExecutionStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Skipped,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConnectionType {
    Dependency,
    OnSuccess,
    OnFailure,
    Conditional,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionMetrics {
    pub total_executions: u32,
    pub successful_executions: u32,
    pub failed_executions: u32,
    pub success_rate: f64,
    pub estimated_duration_ms: u64,
    pub actual_duration_ms: Option<u64>,
    pub average_duration_ms: Option<u64>,
}

impl ExecutionMetrics {
    pub fn new(estimated_duration_ms: u64) -> Self {
        Self {
            total_executions: 0,
            successful_executions: 0,
            failed_executions: 0,
            success_rate: 0.0,
            estimated_duration_ms,
            actual_duration_ms: None,
            average_duration_ms: None,
        }
    }

    pub fn record_execution(&mut self, duration_ms: u64, success: bool) {
        self.total_executions += 1;
        if success {
            self.successful_executions += 1;
        } else {
            self.failed_executions += 1;
        }

        if self.total_executions > 0 {
            self.success_rate =
                (self.successful_executions as f64 / self.total_executions as f64) * 100.0;
        }
        self.actual_duration_ms = Some(duration_ms);

        self.average_duration_ms = if self.total_executions == 1 {
            Some(duration_ms)
        } else if let Some(avg) = self.average_duration_ms {
            Some(
                (avg * (self.total_executions - 1) as u64 + duration_ms)
                    / self.total_executions as u64,
            )
        } else {
            Some(duration_ms)
        };
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParallelExecution {
    pub max_parallel: u32,
    pub current_parallel: u32,
    pub total_instances: u32,
}

impl ParallelExecution {
    pub fn new(max_parallel: u32) -> Self {
        Self {
            max_parallel,
            current_parallel: 0,
            total_instances: 1,
        }
    }

    pub fn with_instances(max_parallel: u32, total_instances: u32) -> Self {
        Self {
            max_parallel,
            current_parallel: 0,
            total_instances,
        }
    }

    pub fn can_start_more(&self) -> bool {
        self.current_parallel < self.max_parallel
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CriticalPath {
    pub nodes: Vec<String>,
    pub is_critical: bool,
}

impl CriticalPath {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            is_critical: false,
        }
    }

    pub fn with_nodes(nodes: Vec<String>) -> Self {
        Self {
            nodes,
            is_critical: true,
        }
    }
}

impl Default for CriticalPath {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepInfo {
    pub step_number: u32,
    pub description: String,
    pub sub_step_count: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
    pub max_retries: u32,
    pub retry_count: u32,
    pub backoff_strategy: BackoffStrategy,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum BackoffStrategy {
    None,
    Fixed {
        delay_ms: u64,
    },
    Exponential {
        base_delay_ms: u64,
        max_delay_ms: u64,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimingInfo {
    pub created_at: String,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
    pub expected_duration_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowNode {
    pub id: String,
    pub name: String,
    pub node_type: NodeType,
    pub status: ExecutionStatus,
    pub dependencies: Vec<String>,
    pub connections: HashMap<String, ConnectionType>,
    pub step_info: StepInfo,
    pub metrics: ExecutionMetrics,
    pub parallel: ParallelExecution,
    pub critical_path: CriticalPath,
    pub targets: Vec<String>,
    pub parameters: HashMap<String, String>,
    pub retry_config: RetryConfig,
    pub timing: TimingInfo,
    pub metadata: HashMap<String, String>,
}

impl FlowNode {
    pub fn new(name: String, node_type: NodeType, step_number: u32) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            node_type,
            status: ExecutionStatus::Pending,
            dependencies: Vec::new(),
            connections: HashMap::new(),
            step_info: StepInfo {
                step_number,
                description: String::new(),
                sub_step_count: None,
            },
            metrics: ExecutionMetrics::new(0),
            parallel: ParallelExecution::new(1),
            critical_path: CriticalPath::new(),
            targets: Vec::new(),
            parameters: HashMap::new(),
            retry_config: RetryConfig {
                max_retries: 3,
                retry_count: 0,
                backoff_strategy: BackoffStrategy::None,
            },
            timing: TimingInfo {
                created_at: chrono::Utc::now().to_rfc3339(),
                started_at: None,
                completed_at: None,
                expected_duration_ms: 0,
            },
            metadata: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeviceStatus {
    Online,
    Offline,
    Busy,
    Error,
    Scanning,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TemperatureStatus {
    Normal,
    Warning,
    Critical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeviceIcon {
    Antenna,
    Radio,
    Satellite,
    Usb,
    Network,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DeviceCapability {
    RFTransmit,
    RFReceive,
    SpectrumAnalysis,
    ProtocolAnalysis,
    MAVLink,
    DJI,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub id: String,
    pub name: String,
    pub path: String,
    pub device_type: String,
    pub status: DeviceStatus,
    pub connection_status: String,
    pub icon: DeviceIcon,
    pub current_task: Option<String>,
    pub task_run_id: Option<String>,
    pub serial_number: String,
    pub firmware_version: String,
    pub port: String,
    pub frequency: String,
    pub sampling_rate: String,
    pub bandwidth: String,
    pub gain: String,
    pub temperature: f32,
    pub temperature_status: TemperatureStatus,
    pub last_seen: Option<String>,
    pub uptime: Option<String>,
    pub protocol: String,
    pub capabilities: Vec<DeviceCapability>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContainerExecutionStatus {
    Running,
    Stopped,
    Building,
}

impl std::fmt::Display for ContainerExecutionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContainerExecutionStatus::Running => write!(f, "RUNNING"),
            ContainerExecutionStatus::Stopped => write!(f, "STOPPED"),
            ContainerExecutionStatus::Building => write!(f, "BUILDING"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerStatus {
    pub container_id: String,
    pub docker_exec_command: String,
    pub agent: String,
    pub task_name: String,
    pub status: ContainerExecutionStatus,
    pub running_duration: String,
    pub cpu_usage_percent: f64,
    pub memory_usage_mb: f64,
    pub memory_limit_mb: f64,
    pub exposed_ports: Vec<String>,
}

impl ContainerStatus {
    pub fn new(
        container_id: String,
        docker_exec_command: String,
        agent: String,
        task_name: String,
        status: ContainerExecutionStatus,
        running_duration: String,
        cpu_usage_percent: f64,
        memory_usage_mb: f64,
        memory_limit_mb: f64,
        exposed_ports: Vec<String>,
    ) -> Self {
        Self {
            container_id,
            docker_exec_command,
            agent,
            task_name,
            status,
            running_duration,
            cpu_usage_percent,
            memory_usage_mb,
            memory_limit_mb,
            exposed_ports,
        }
    }

    pub fn memory_usage_percent(&self) -> f64 {
        if self.memory_limit_mb > 0.0 {
            (self.memory_usage_mb / self.memory_limit_mb) * 100.0
        } else {
            0.0
        }
    }
}

/// 资产数据模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetData {
    pub id: String,
    pub name: String,
    pub asset_type: String,
    pub status: String,
}

/// Risk severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

impl Severity {
    pub fn color_hex(&self) -> u32 {
        match self {
            Severity::Critical => 0xef4444,
            Severity::High => 0xf97316,
            Severity::Medium => 0xfbbf24,
            Severity::Low => 0x10b981,
            Severity::Info => 0x6b7280,
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Severity::Critical => "CRITICAL",
            Severity::High => "HIGH",
            Severity::Medium => "MEDIUM",
            Severity::Low => "LOW",
            Severity::Info => "INFO",
        }
    }
}

/// Zone classification (Z1-Z5)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ZoneType {
    Z1, // 外部网络
    Z2, // DMZ
    Z3, // 业务网络
    Z4, // 飞控设备层
    Z5, // 设备通信层
}

impl ZoneType {
    pub fn display_name(&self) -> &'static str {
        match self {
            ZoneType::Z1 => "外部网络",
            ZoneType::Z2 => "DMZ",
            ZoneType::Z3 => "业务网络",
            ZoneType::Z4 => "飞控设备层",
            ZoneType::Z5 => "设备通信层",
        }
    }
}

/// Network service information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Service {
    pub port: u16,
    pub protocol: String,
    pub service_name: String,
    pub version: Option<String>,
    pub banner: Option<String>,
}

/// Authentication credentials
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credential {
    pub username: String,
    pub password: String,
    pub auth_type: String,
    pub last_used: Option<String>,
}

/// Scan progress information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanProgress {
    pub percentage: u8,
    pub last_scan: Option<String>,
    pub next_scan: Option<String>,
    pub scan_type: String,
    pub scanning: bool,
}

/// Compliance standard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceStandard {
    pub name: String,
    pub status: ComplianceStatus,
    pub last_audit: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComplianceStatus {
    Compliant,
    NonCompliant,
    Pending,
    NotApplicable,
}

/// Network connection information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connection {
    pub target_id: String,
    pub connection_type: String,
    pub protocol: String,
    pub port: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssetStatus {
    Online,
    Offline,
    Unknown,
    Maintenance,
}

/// Asset node data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetNode {
    pub id: String,
    pub name: String,
    pub ip_address: String,
    pub mac_address: Option<String>,
    pub zone: ZoneType,
    pub severity: Severity,
    pub risk_score: u8,
    pub vulnerabilities_count: usize,
    pub services: Vec<Service>,
    pub open_ports: Vec<u16>,
    pub credentials: Vec<Credential>,
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

impl AssetNode {
    pub fn new(id: String, name: String, ip_address: String, zone: ZoneType) -> Self {
        Self {
            id,
            name,
            ip_address,
            mac_address: None,
            zone,
            severity: Severity::Info,
            risk_score: 0,
            vulnerabilities_count: 0,
            services: Vec::new(),
            open_ports: Vec::new(),
            credentials: Vec::new(),
            owner: String::new(),
            business_purpose: String::new(),
            department: None,
            scan_progress: ScanProgress {
                percentage: 0,
                last_scan: None,
                next_scan: None,
                scan_type: String::from("Quick"),
                scanning: false,
            },
            compliance_standards: Vec::new(),
            connections: Vec::new(),
            status: AssetStatus::Unknown,
            last_seen: String::new(),
            asset_type: String::new(),
            firmware_version: None,
            manufacturer: None,
            location: None,
        }
    }

    pub fn is_critical(&self) -> bool {
        self.severity == Severity::Critical
    }

    pub fn open_ports_count(&self) -> usize {
        self.open_ports.len()
    }

    pub fn services_count(&self) -> usize {
        self.services.len()
    }

    pub fn is_scanning(&self) -> bool {
        self.scan_progress.scanning
    }

    pub fn severity_color(&self) -> u32 {
        self.severity.color_hex()
    }
}
