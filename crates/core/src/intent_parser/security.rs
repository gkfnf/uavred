//! 安全测试意图定义

use super::{ConfidenceScore, Intent, ParseMetadata, Suggestion};
use crate::task::{TaskPriority, TaskType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 安全测试意图
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityTestIntent {
    /// 基础意图
    pub base: Intent,
    /// 测试类型
    pub test_type: SecurityTestType,
    /// 测试参数
    pub params: SecurityTestParams,
    /// 目标信息
    pub targets: Vec<Target>,
    /// 扫描配置
    pub scan_config: ScanConfig,
}

impl SecurityTestIntent {
    /// 从基础意图创建
    pub fn from_intent(intent: Intent) -> Self {
        Self {
            base: intent,
            test_type: SecurityTestType::Unknown,
            params: SecurityTestParams::default(),
            targets: Vec::new(),
            scan_config: ScanConfig::default(),
        }
    }

    /// 转换为 TaskType
    pub fn to_task_type(&self) -> Option<TaskType> {
        match &self.test_type {
            SecurityTestType::NetworkScan => {
                let target = self.targets.first()?;
                Some(TaskType::NetworkScan {
                    target: target.address.clone(),
                })
            }
            SecurityTestType::PortScan => {
                let target = self.targets.first()?;
                Some(TaskType::NetworkScan {
                    target: target.address.clone(),
                })
            }
            SecurityTestType::ProtocolAnalysis => {
                let target = self.targets.first()?;
                let protocol = self.params.get_string("protocol").unwrap_or_default();
                Some(TaskType::ProtocolAnalysis {
                    target: target.address.clone(),
                    protocol,
                })
            }
            SecurityTestType::FirmwareAnalysis => {
                let path = self.params.get_string("firmware_path").unwrap_or_default();
                Some(TaskType::FirmwareAnalysis { path })
            }
            SecurityTestType::VulnerabilityScan => {
                let target = self.targets.first()?;
                Some(TaskType::NetworkScan {
                    target: target.address.clone(),
                })
            }
            SecurityTestType::Exploit => {
                let target = self.targets.first()?;
                let exploit_id = self.params.get_string("exploit_id").unwrap_or_default();
                Some(TaskType::Exploit {
                    target: target.address.clone(),
                    exploit_id,
                })
            }
            _ => None,
        }
    }

    /// 获取建议的优先级
    pub fn suggested_priority(&self) -> TaskPriority {
        match self.params.get_string("priority").as_deref() {
            Some("critical") => TaskPriority::Critical,
            Some("high") => TaskPriority::High,
            Some("low") => TaskPriority::Low,
            _ => TaskPriority::Medium,
        }
    }
}

/// 安全测试类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecurityTestType {
    /// 网络扫描
    NetworkScan,
    /// 端口扫描
    PortScan,
    /// 协议分析
    ProtocolAnalysis,
    /// 固件分析
    FirmwareAnalysis,
    /// 漏洞扫描
    VulnerabilityScan,
    /// 渗透测试/漏洞利用
    Exploit,
    /// Web 应用测试
    WebAppTest,
    /// API 测试
    ApiTest,
    /// 无线测试
    WirelessTest,
    /// 社会工程测试
    SocialEngineering,
    /// 配置审计
    ConfigurationAudit,
    /// 合规检查
    ComplianceCheck,
    /// 未知
    Unknown,
}

impl SecurityTestType {
    pub fn as_str(&self) -> &'static str {
        match self {
            SecurityTestType::NetworkScan => "network_scan",
            SecurityTestType::PortScan => "port_scan",
            SecurityTestType::ProtocolAnalysis => "protocol_analysis",
            SecurityTestType::FirmwareAnalysis => "firmware_analysis",
            SecurityTestType::VulnerabilityScan => "vulnerability_scan",
            SecurityTestType::Exploit => "exploit",
            SecurityTestType::WebAppTest => "web_app_test",
            SecurityTestType::ApiTest => "api_test",
            SecurityTestType::WirelessTest => "wireless_test",
            SecurityTestType::SocialEngineering => "social_engineering",
            SecurityTestType::ConfigurationAudit => "configuration_audit",
            SecurityTestType::ComplianceCheck => "compliance_check",
            SecurityTestType::Unknown => "unknown",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            SecurityTestType::NetworkScan => "网络扫描",
            SecurityTestType::PortScan => "端口扫描",
            SecurityTestType::ProtocolAnalysis => "协议分析",
            SecurityTestType::FirmwareAnalysis => "固件分析",
            SecurityTestType::VulnerabilityScan => "漏洞扫描",
            SecurityTestType::Exploit => "漏洞利用",
            SecurityTestType::WebAppTest => "Web应用测试",
            SecurityTestType::ApiTest => "API测试",
            SecurityTestType::WirelessTest => "无线测试",
            SecurityTestType::SocialEngineering => "社会工程测试",
            SecurityTestType::ConfigurationAudit => "配置审计",
            SecurityTestType::ComplianceCheck => "合规检查",
            SecurityTestType::Unknown => "未知",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            SecurityTestType::NetworkScan => "扫描目标网络，发现存活主机和网络拓扑",
            SecurityTestType::PortScan => "扫描目标主机的开放端口和服务",
            SecurityTestType::ProtocolAnalysis => "分析特定协议的安全性和实现",
            SecurityTestType::FirmwareAnalysis => "分析固件文件中的安全漏洞",
            SecurityTestType::VulnerabilityScan => "扫描目标系统的已知漏洞",
            SecurityTestType::Exploit => "尝试利用发现的漏洞",
            SecurityTestType::WebAppTest => "测试Web应用程序的安全性",
            SecurityTestType::ApiTest => "测试API端点的安全性",
            SecurityTestType::WirelessTest => "测试无线网络的安全性",
            SecurityTestType::SocialEngineering => "测试人员安全意识",
            SecurityTestType::ConfigurationAudit => "审计系统配置的安全性",
            SecurityTestType::ComplianceCheck => "检查合规性要求",
            SecurityTestType::Unknown => "未知类型的安全测试",
        }
    }

    /// 获取所需的 Agent 能力
    pub fn required_capabilities(&self) -> Vec<&'static str> {
        match self {
            SecurityTestType::NetworkScan => vec!["network_scan", "host_discovery"],
            SecurityTestType::PortScan => vec!["port_scan", "service_detection"],
            SecurityTestType::ProtocolAnalysis => vec!["protocol_analysis", "packet_capture"],
            SecurityTestType::FirmwareAnalysis => vec!["firmware_analysis", "binary_analysis"],
            SecurityTestType::VulnerabilityScan => vec!["vuln_scan", "cve_lookup"],
            SecurityTestType::Exploit => vec!["exploit_execution", "payload_generation"],
            SecurityTestType::WebAppTest => vec!["web_scan", "sql_injection", "xss_detection"],
            SecurityTestType::ApiTest => vec!["api_testing", "fuzzing"],
            SecurityTestType::WirelessTest => vec!["wireless_scan", "packet_injection"],
            SecurityTestType::SocialEngineering => vec!["phishing", "osint"],
            SecurityTestType::ConfigurationAudit => vec!["config_audit", "compliance_check"],
            SecurityTestType::ComplianceCheck => vec!["compliance_check", "policy_validation"],
            SecurityTestType::Unknown => vec![],
        }
    }
}

impl From<&str> for SecurityTestType {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "network_scan" | "network" | "netscan" | "网络扫描" => SecurityTestType::NetworkScan,
            "port_scan" | "port" | "端口扫描" => SecurityTestType::PortScan,
            "protocol_analysis" | "protocol" | "协议分析" => SecurityTestType::ProtocolAnalysis,
            "firmware_analysis" | "firmware" | "固件分析" => SecurityTestType::FirmwareAnalysis,
            "vulnerability_scan" | "vuln_scan" | "漏洞扫描" => SecurityTestType::VulnerabilityScan,
            "exploit" | "penetration" | "漏洞利用" | "渗透测试" => SecurityTestType::Exploit,
            "web_app_test" | "web" | "webscan" | "web应用测试" => SecurityTestType::WebAppTest,
            "api_test" | "api" | "api测试" => SecurityTestType::ApiTest,
            "wireless_test" | "wireless" | "wifi" | "无线测试" => SecurityTestType::WirelessTest,
            "social_engineering" | "social" | "社会工程" => SecurityTestType::SocialEngineering,
            "configuration_audit" | "config" | "配置审计" => SecurityTestType::ConfigurationAudit,
            "compliance_check" | "compliance" | "合规检查" => SecurityTestType::ComplianceCheck,
            _ => SecurityTestType::Unknown,
        }
    }
}

/// 安全测试参数
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SecurityTestParams {
    /// 参数映射
    pub params: HashMap<String, serde_json::Value>,
}

impl SecurityTestParams {
    pub fn new() -> Self {
        Self {
            params: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: impl Into<String>, value: impl Into<serde_json::Value>) {
        self.params.insert(key.into(), value.into());
    }

    pub fn get(&self, key: &str) -> Option<&serde_json::Value> {
        self.params.get(key)
    }

    pub fn get_string(&self, key: &str) -> Option<String> {
        self.params.get(key).and_then(|v| {
            if let Some(s) = v.as_str() {
                Some(s.to_string())
            } else {
                Some(v.to_string())
            }
        })
    }

    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.params.get(key).and_then(|v| v.as_bool())
    }

    pub fn get_u64(&self, key: &str) -> Option<u64> {
        self.params.get(key).and_then(|v| {
            if let Some(n) = v.as_u64() {
                Some(n)
            } else if let Some(n) = v.as_str() {
                n.parse().ok()
            } else {
                None
            }
        })
    }

    pub fn port_range(&self) -> Option<(u16, u16)> {
        let range = self.get_string("port_range")?;
        if range.contains('-') {
            let parts: Vec<&str> = range.split('-').collect();
            if parts.len() == 2 {
                let start: u16 = parts[0].trim().parse().ok()?;
                let end: u16 = parts[1].trim().parse().ok()?;
                return Some((start, end));
            }
        } else if let Ok(port) = range.parse::<u16>() {
            return Some((port, port));
        }
        None
    }
}

/// 目标定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Target {
    /// 目标地址（IP、域名、CIDR 等）
    pub address: String,
    /// 目标类型
    pub target_type: TargetType,
    /// 端口列表（可选）
    pub ports: Option<Vec<u16>>,
    /// 额外信息
    pub metadata: HashMap<String, serde_json::Value>,
}

impl Target {
    pub fn new(address: impl Into<String>, target_type: TargetType) -> Self {
        Self {
            address: address.into(),
            target_type,
            ports: None,
            metadata: HashMap::new(),
        }
    }

    pub fn with_ports(mut self, ports: Vec<u16>) -> Self {
        self.ports = Some(ports);
        self
    }
}

/// 目标类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TargetType {
    Ip,
    Cidr,
    Domain,
    Hostname,
    Url,
    File,
    Range,
    Unknown,
}

impl TargetType {
    pub fn as_str(&self) -> &'static str {
        match self {
            TargetType::Ip => "ip",
            TargetType::Cidr => "cidr",
            TargetType::Domain => "domain",
            TargetType::Hostname => "hostname",
            TargetType::Url => "url",
            TargetType::File => "file",
            TargetType::Range => "range",
            TargetType::Unknown => "unknown",
        }
    }
}

/// 扫描配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanConfig {
    /// 扫描强度
    pub intensity: ScanIntensity,
    /// 是否进行深层扫描
    pub deep_scan: bool,
    /// 并发线程数
    pub threads: u32,
    /// 超时时间（秒）
    pub timeout_seconds: u64,
    /// 额外选项
    pub options: HashMap<String, serde_json::Value>,
}

impl Default for ScanConfig {
    fn default() -> Self {
        Self {
            intensity: ScanIntensity::Normal,
            deep_scan: false,
            threads: 10,
            timeout_seconds: 300,
            options: HashMap::new(),
        }
    }
}

/// 扫描强度
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScanIntensity {
    /// 轻度 - 快速扫描，较少请求
    Light,
    /// 正常 - 平衡速度和质量
    Normal,
    /// 激进 - 深度扫描，更多请求
    Aggressive,
    /// 定制
    Custom,
}

impl ScanIntensity {
    pub fn as_str(&self) -> &'static str {
        match self {
            ScanIntensity::Light => "light",
            ScanIntensity::Normal => "normal",
            ScanIntensity::Aggressive => "aggressive",
            ScanIntensity::Custom => "custom",
        }
    }
}

/// 解析后的安全测试意图
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedSecurityIntent {
    /// 原始意图文本
    pub raw_intent: String,
    /// 解析后的安全测试意图
    pub security_intent: SecurityTestIntent,
    /// 置信度评分
    pub confidence: ConfidenceScore,
    /// 解析元数据
    pub metadata: ParseMetadata,
    /// 建议/澄清请求
    pub suggestions: Vec<Suggestion>,
}

impl ParsedSecurityIntent {
    /// 是否需要澄清
    pub fn needs_clarification(&self) -> bool {
        !self.suggestions.is_empty()
            || self.confidence.overall < 0.7
            || self.security_intent.test_type == SecurityTestType::Unknown
    }

    /// 获取任务名称
    pub fn task_name(&self) -> String {
        format!(
            "{} - {}",
            self.security_intent.test_type.display_name(),
            self.security_intent
                .targets
                .first()
                .map(|t| t.address.as_str())
                .unwrap_or("未知目标")
        )
    }

    /// 获取任务描述
    pub fn task_description(&self) -> String {
        let mut desc = format!(
            "类型: {}\n",
            self.security_intent.test_type.description()
        );
        desc.push_str(&format!(
            "目标: {}\n",
            self.security_intent
                .targets
                .iter()
                .map(|t| t.address.clone())
                .collect::<Vec<_>>()
                .join(", ")
        ));
        if !self.security_intent.params.params.is_empty() {
            desc.push_str("参数:\n");
            for (k, v) in &self.security_intent.params.params {
                desc.push_str(&format!("  - {}: {}\n", k, v));
            }
        }
        desc
    }
}
