//! Vulns UI State Management
//!
//! 管理漏洞列表的状态：
//! - 当前选中的漏洞
//! - 列表视图类型（Severity/Asset/MITRE）
//! - 分组展开/折叠状态
//! - 搜索过滤

use data::VulnData;
use gpui::*;
use std::collections::HashMap;

/// 列表视图类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ListViewType {
    #[default]
    Severity,
    Asset,
    Mitre,
}

impl ListViewType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ListViewType::Severity => "severity",
            ListViewType::Asset => "asset",
            ListViewType::Mitre => "mitre",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            ListViewType::Severity => "Severity",
            ListViewType::Asset => "Asset",
            ListViewType::Mitre => "MITRE",
        }
    }
}

/// 漏洞分组状态
#[derive(Debug, Clone)]
pub struct VulnGroup {
    pub name: String,
    pub expanded: bool,
    pub vulns: Vec<VulnData>,
}

impl VulnGroup {
    pub fn new(name: impl Into<String>, vulns: Vec<VulnData>) -> Self {
        Self {
            name: name.into(),
            expanded: true,
            vulns,
        }
    }
}

/// 漏洞选择事件
#[derive(Clone)]
pub struct VulnSelectedEvent {
    pub vuln_id: String,
}

impl EventEmitter<VulnSelectedEvent> for VulnState {}

/// 漏洞面板状态
pub struct VulnState {
    /// 所有漏洞数据
    vulns: Vec<VulnData>,
    /// 选中的漏洞 ID
    selected_id: Option<String>,
    /// 当前视图类型
    view_type: ListViewType,
    /// 分组展开状态
    group_states: HashMap<String, bool>,
}

impl VulnState {
    /// 创建新状态
    pub fn new() -> Self {
        Self {
            vulns: Vec::new(),
            selected_id: None,
            view_type: ListViewType::default(),
            group_states: HashMap::new(),
        }
    }

    /// 使用初始数据创建
    pub fn with_vulns(mut self, vulns: Vec<VulnData>) -> Self {
        self.vulns = vulns;
        // 默认选中第一个
        if let Some(first) = self.vulns.first() {
            self.selected_id = Some(first.id.clone());
        }
        self
    }

    /// 获取所有漏洞
    pub fn vulns(&self) -> &[VulnData] {
        &self.vulns
    }

    /// 获取选中的漏洞
    pub fn selected(&self) -> Option<&VulnData> {
        self.selected_id.as_ref().and_then(|id| {
            self.vulns
                .iter()
                .find(|v| &v.id == id || v.cve.as_ref() == Some(id))
        })
    }

    /// 获取选中的漏洞 ID
    pub fn selected_id(&self) -> Option<&str> {
        self.selected_id.as_deref()
    }

    /// 选择漏洞
    pub fn select(&mut self, vuln_id: impl Into<String>, cx: &mut Context<Self>) {
        let id = vuln_id.into();
        self.selected_id = Some(id.clone());
        cx.emit(VulnSelectedEvent { vuln_id: id });
        cx.notify();
    }

    /// 获取当前视图类型
    pub fn view_type(&self) -> ListViewType {
        self.view_type
    }

    /// 设置视图类型
    pub fn set_view_type(&mut self, view_type: ListViewType, cx: &mut Context<Self>) {
        self.view_type = view_type;
        cx.notify();
    }

    /// 切换分组展开状态
    pub fn toggle_group(&mut self, group_name: &str, cx: &mut Context<Self>) {
        let entry = self.group_states.entry(group_name.to_string()).or_insert(true);
        *entry = !*entry;
        cx.notify();
    }

    /// 获取分组展开状态
    pub fn is_group_expanded(&self, group_name: &str) -> bool {
        self.group_states.get(group_name).copied().unwrap_or(true)
    }

    /// 获取漏洞数量
    pub fn count(&self) -> usize {
        self.vulns.len()
    }

    /// 获取分组后的漏洞
    pub fn grouped_vulns(&self) -> Vec<VulnGroup> {
        match self.view_type {
            ListViewType::Severity => self.group_by_severity(),
            ListViewType::Asset => self.group_by_asset(),
            ListViewType::Mitre => self.group_by_mitre(),
        }
    }

    /// 按严重程度分组
    fn group_by_severity(&self) -> Vec<VulnGroup> {
        use std::collections::HashMap;

        let mut groups: HashMap<String, Vec<VulnData>> = HashMap::new();

        for vuln in &self.vulns {
            let severity = format!("{:?}", vuln.severity).to_uppercase();
            groups.entry(severity).or_default().push(vuln.clone());
        }

        let severity_order = ["CRITICAL", "HIGH", "MEDIUM", "LOW", "INFO"];
        severity_order
            .iter()
            .filter_map(|severity| {
                groups.remove(*severity).map(|vulns| VulnGroup::new(*severity, vulns))
            })
            .collect()
    }

    /// 按资产分组
    fn group_by_asset(&self) -> Vec<VulnGroup> {
        use std::collections::HashMap;

        let mut groups: HashMap<String, Vec<VulnData>> = HashMap::new();

        for vuln in &self.vulns {
            let asset = if vuln.affected.is_empty() {
                "Unknown Asset".to_string()
            } else {
                vuln.affected.clone()
            };
            groups.entry(asset).or_default().push(vuln.clone());
        }

        let mut result: Vec<_> = groups
            .into_iter()
            .map(|(name, vulns)| VulnGroup::new(name, vulns))
            .collect();
        result.sort_by(|a, b| a.name.cmp(&b.name));
        result
    }

    /// 按 MITRE ATT&CK 技术分组
    fn group_by_mitre(&self) -> Vec<VulnGroup> {
        use std::collections::HashMap;

        let mut groups: HashMap<String, Vec<VulnData>> = HashMap::new();

        for vuln in &self.vulns {
            if vuln.attack_techniques.is_empty() {
                groups
                    .entry("No MITRE Technique".to_string())
                    .or_default()
                    .push(vuln.clone());
            } else {
                for tech in &vuln.attack_techniques {
                    groups.entry(tech.clone()).or_default().push(vuln.clone());
                }
            }
        }

        let mut result: Vec<_> = groups
            .into_iter()
            .map(|(name, vulns)| VulnGroup::new(name, vulns))
            .collect();
        result.sort_by(|a, b| a.name.cmp(&b.name));
        result
    }
}

impl Default for VulnState {
    fn default() -> Self {
        Self::new()
    }
}

/// 模拟数据生成（用于开发和测试）
pub mod mock {
    use data::{
        AiSecurityAnalysis, CvssScore, DetectionLocation, DetectionSource, ExploitMaturity,
        ScanType, VulnData, VulnSeverity, VulnStatus,
    };

    pub fn sample_vulns() -> Vec<VulnData> {
        vec![
            VulnData {
                id: "CVE-2024-1234".to_string(),
                title: "Buffer Overflow in Telemetry Parser".to_string(),
                description: "A buffer overflow vulnerability exists in the telemetry data parser allowing remote code execution through malformed telemetry packets.".to_string(),
                cve: Some("CVE-2024-1234".to_string()),
                cwe: Some("CWE-120".to_string()),
                severity: VulnSeverity::Critical,
                status: VulnStatus::Confirmed,
                cvss: Some(CvssScore::new(
                    9.8,
                    "CVSS:3.1/AV:N/AC:L/PR:N/UI:N/S:U/C:H/I:H/A:H".to_string(),
                )),
                affected: "DJI MAVIC 3 PRO".to_string(),
                affected_systems: vec!["telemetry_parser.c".to_string()],
                detection_time: "2024-11-05 14:32:45".to_string(),
                detection_location: DetectionLocation {
                    component: "DJI Mavic 3 Pro".to_string(),
                    file_path: Some("telemetry_parser.c".to_string()),
                    line_number: Some(247),
                    function: None,
                    source: DetectionSource::AiAnalysis,
                },
                scan_type: ScanType::StaticCodeAnalysis,
                exploit_available: true,
                poc_available: true,
                exploit_maturity: Some(ExploitMaturity::Functional),
                attack_tactics: vec!["Initial Access".to_string()],
                attack_techniques: vec!["T0806".to_string(), "T0868".to_string(), "T0885".to_string()],
                attack_subtechniques: vec![],
                ai_analysis: Some(AiSecurityAnalysis {
                    confidence_score: 0.98,
                    risk_score: 9.5,
                    analysis_type: "Static Analysis".to_string(),
                    reasoning: "Buffer overflow detected in telemetry parser".to_string(),
                    recommendations: vec!["Update firmware to v2.4.3 immediately. Implement input validation and bounds checking.".to_string()],
                    false_positive_probability: 0.02,
                    model_version: "v1.0".to_string(),
                    analyzed_at: "2024-11-05 14:32:45".to_string(),
                }),
                references: vec![],
                tags: vec![],
            },
            VulnData {
                id: "SEC-UAV-002".to_string(),
                title: "Default Admin Credentials".to_string(),
                description: "The device uses default administrative credentials that can be easily guessed.".to_string(),
                cve: None,
                cwe: Some("CWE-798".to_string()),
                severity: VulnSeverity::High,
                status: VulnStatus::Confirmed,
                cvss: Some(CvssScore::new(
                    8.1,
                    "CVSS:3.1/AV:N/AC:H/PR:N/UI:N/S:U/C:H/I:H/A:H".to_string(),
                )),
                affected: "GCS PRIMARY STATION".to_string(),
                affected_systems: vec![],
                detection_time: "2024-11-05 10:15:22".to_string(),
                detection_location: DetectionLocation {
                    component: "GCS Primary Station".to_string(),
                    file_path: None,
                    line_number: None,
                    function: None,
                    source: DetectionSource::AutomatedScanner,
                },
                scan_type: ScanType::Network,
                exploit_available: true,
                poc_available: true,
                exploit_maturity: Some(ExploitMaturity::ProofOfConcept),
                attack_tactics: vec![],
                attack_techniques: vec![],
                attack_subtechniques: vec![],
                ai_analysis: Some(AiSecurityAnalysis {
                    confidence_score: 1.0,
                    risk_score: 8.0,
                    analysis_type: "Credential Analysis".to_string(),
                    reasoning: "Default credentials found".to_string(),
                    recommendations: vec!["Change default credentials immediately".to_string()],
                    false_positive_probability: 0.0,
                    model_version: "v1.0".to_string(),
                    analyzed_at: "2024-11-05 10:15:22".to_string(),
                }),
                references: vec![],
                tags: vec![],
            },
            VulnData {
                id: "SEC-UAV-003".to_string(),
                title: "MAVLink Command Injection".to_string(),
                description: "Improper input validation allows command injection in MAVLink protocol handler.".to_string(),
                cve: None,
                cwe: Some("CWE-77".to_string()),
                severity: VulnSeverity::High,
                status: VulnStatus::Confirmed,
                cvss: Some(CvssScore::new(
                    7.5,
                    "CVSS:3.1/AV:N/AC:L/PR:N/UI:N/S:U/C:N/I:H/A:N".to_string(),
                )),
                affected: "FLIGHT CONTROLLER".to_string(),
                affected_systems: vec![],
                detection_time: "2024-11-04 16:45:10".to_string(),
                detection_location: DetectionLocation {
                    component: "Flight Controller".to_string(),
                    file_path: None,
                    line_number: None,
                    function: None,
                    source: DetectionSource::DynamicAnalysis,
                },
                scan_type: ScanType::Protocol,
                exploit_available: false,
                poc_available: true,
                exploit_maturity: Some(ExploitMaturity::ProofOfConcept),
                attack_tactics: vec![],
                attack_techniques: vec![],
                attack_subtechniques: vec![],
                ai_analysis: Some(AiSecurityAnalysis {
                    confidence_score: 0.91,
                    risk_score: 7.2,
                    analysis_type: "Protocol Analysis".to_string(),
                    reasoning: "Command injection vulnerability detected".to_string(),
                    recommendations: vec!["Implement input validation".to_string()],
                    false_positive_probability: 0.09,
                    model_version: "v1.0".to_string(),
                    analyzed_at: "2024-11-04 16:45:10".to_string(),
                }),
                references: vec![],
                tags: vec![],
            },
            VulnData {
                id: "SEC-UAV-004".to_string(),
                title: "SQL Injection in Flight Logs".to_string(),
                description: "SQL injection vulnerability in flight log query interface.".to_string(),
                cve: None,
                cwe: Some("CWE-89".to_string()),
                severity: VulnSeverity::Medium,
                status: VulnStatus::New,
                cvss: Some(CvssScore::new(
                    6.5,
                    "CVSS:3.1/AV:N/AC:L/PR:L/UI:N/S:U/C:H/I:N/A:N".to_string(),
                )),
                affected: "TELEMETRY SERVER".to_string(),
                affected_systems: vec![],
                detection_time: "2024-11-03 09:20:15".to_string(),
                detection_location: DetectionLocation {
                    component: "Telemetry Server".to_string(),
                    file_path: None,
                    line_number: None,
                    function: None,
                    source: DetectionSource::StaticAnalysis,
                },
                scan_type: ScanType::StaticCodeAnalysis,
                exploit_available: false,
                poc_available: true,
                exploit_maturity: Some(ExploitMaturity::ProofOfConcept),
                attack_tactics: vec![],
                attack_techniques: vec![],
                attack_subtechniques: vec![],
                ai_analysis: Some(AiSecurityAnalysis {
                    confidence_score: 0.89,
                    risk_score: 6.0,
                    analysis_type: "Code Analysis".to_string(),
                    reasoning: "SQL injection detected in query construction".to_string(),
                    recommendations: vec!["Use parameterized queries".to_string()],
                    false_positive_probability: 0.11,
                    model_version: "v1.0".to_string(),
                    analyzed_at: "2024-11-03 09:20:15".to_string(),
                }),
                references: vec![],
                tags: vec![],
            },
        ]
    }
}
