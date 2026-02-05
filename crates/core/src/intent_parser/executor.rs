//! 意图执行器 - 将解析后的意图转换为可执行任务

use super::{
    error::{ExecutionResult, IntentExecutionError},
    security::{ParsedSecurityIntent, SecurityTestType, ScanIntensity},
};
use crate::task::{Task, TaskPriority};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 意图执行器
pub struct IntentExecutor {
    config: ExecutorConfig,
}

/// 执行器配置
#[derive(Debug, Clone)]
pub struct ExecutorConfig {
    /// 默认任务超时（秒）
    pub default_timeout_seconds: u64,
    /// 是否自动执行任务
    pub auto_execute: bool,
    /// 默认 Agent 镜像
    pub default_agent_image: String,
}

impl Default for ExecutorConfig {
    fn default() -> Self {
        Self {
            default_timeout_seconds: 300,
            auto_execute: false,
            default_agent_image: "uavred/agent:latest".to_string(),
        }
    }
}

impl IntentExecutor {
    /// 创建新的执行器
    pub fn new() -> Self {
        Self {
            config: ExecutorConfig::default(),
        }
    }

    /// 创建带配置的执行器
    pub fn with_config(config: ExecutorConfig) -> Self {
        Self { config }
    }

    /// 从解析后的意图创建任务
    pub fn create_task(&self, parsed: &ParsedSecurityIntent) -> ExecutionResult<Task> {
        let task_type = parsed
            .security_intent
            .to_task_type()
            .ok_or_else(|| IntentExecutionError::execution_failed("Could not determine task type"))?;

        let priority = parsed.security_intent.suggested_priority();

        let mut task = Task::new(
            parsed.task_name(),
            task_type,
            priority,
        );

        // 注意：Task 结构体不包含 description 和 metadata 字段
        // 这些信息通过 task_type 编码
        // 如果需要额外信息，考虑使用 data::models::Task
        
        let _description = parsed.task_description();
        let _metadata = serde_json::json!({
            "intent_parser_version": super::INTENT_PARSER_VERSION,
            "parsed_confidence": parsed.confidence,
            "test_type": parsed.security_intent.test_type.as_str(),
            "targets": parsed.security_intent.targets,
            "scan_config": parsed.security_intent.scan_config,
        });

        // 如果置信度足够高且配置允许，可以直接开始
        if self.config.auto_execute && parsed.confidence.overall >= 0.8 {
            task.start();
        }

        Ok(task)
    }

    /// 创建执行任务（返回 data::models::Task 兼容格式）
    pub fn create_data_task(
        &self,
        parsed: &ParsedSecurityIntent,
        user_id: Option<i64>,
    ) -> ExecutionResult<data::models::Task> {
        use data::models::{Task as DataTask, TaskPriority as DataPriority, TaskStatus as DataStatus};

        let task_type_str = parsed.security_intent.test_type.as_str().to_string();
        let priority = match parsed.security_intent.suggested_priority() {
            TaskPriority::Critical => DataPriority::Critical,
            TaskPriority::High => DataPriority::High,
            TaskPriority::Medium => DataPriority::Medium,
            TaskPriority::Low => DataPriority::Low,
        };

        let task = DataTask {
            id: 0, // 将由数据库分配
            title: parsed.task_name(),
            description: parsed.task_description(),
            mission_objective: parsed.raw_intent.clone(),
            task_type: task_type_str,
            priority,
            status: DataStatus::Todo,
            assignee: user_id.map(|id| id.to_string()).unwrap_or_default(),
            estimated_minutes: self.estimate_duration(&parsed.security_intent.test_type),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            started_at: None,
            completed_at: None,
            closed_at: None,
            close_reason: String::new(),
            source: "intent_parser".to_string(),
            external_ref: String::new(),
            metadata: serde_json::json!({
                "intent_parser": {
                    "version": super::INTENT_PARSER_VERSION,
                    "confidence": parsed.confidence,
                    "test_type": parsed.security_intent.test_type.as_str(),
                    "targets": parsed.security_intent.targets,
                    "params": parsed.security_intent.params.params,
                    "scan_config": parsed.security_intent.scan_config,
                }
            }),
            labels: vec!["ai_parsed".to_string()],
            comments: Vec::new(),
            dependencies: Vec::new(),
        };

        Ok(task)
    }

    /// 生成执行计划
    pub fn generate_execution_plan(
        &self,
        parsed: &ParsedSecurityIntent,
    ) -> ExecutionResult<ExecutionPlan> {
        let mut steps = Vec::new();

        // 根据测试类型生成步骤
        match parsed.security_intent.test_type {
            SecurityTestType::NetworkScan => {
                steps.push(ExecutionStep {
                    name: "host_discovery".to_string(),
                    description: "发现存活主机".to_string(),
                    step_type: StepType::Scan,
                    depends_on: Vec::new(),
                    estimated_duration_seconds: 60,
                });
                steps.push(ExecutionStep {
                    name: "service_detection".to_string(),
                    description: "检测开放服务".to_string(),
                    step_type: StepType::Scan,
                    depends_on: vec!["host_discovery".to_string()],
                    estimated_duration_seconds: 120,
                });
            }
            SecurityTestType::PortScan => {
                steps.push(ExecutionStep {
                    name: "port_scan".to_string(),
                    description: "扫描目标端口".to_string(),
                    step_type: StepType::Scan,
                    depends_on: Vec::new(),
                    estimated_duration_seconds: 60,
                });
            }
            SecurityTestType::VulnerabilityScan => {
                steps.push(ExecutionStep {
                    name: "service_detection".to_string(),
                    description: "检测服务版本".to_string(),
                    step_type: StepType::Scan,
                    depends_on: Vec::new(),
                    estimated_duration_seconds: 60,
                });
                steps.push(ExecutionStep {
                    name: "vulnerability_check".to_string(),
                    description: "检查已知漏洞".to_string(),
                    step_type: StepType::Analysis,
                    depends_on: vec!["service_detection".to_string()],
                    estimated_duration_seconds: 180,
                });
            }
            SecurityTestType::ProtocolAnalysis => {
                steps.push(ExecutionStep {
                    name: "traffic_capture".to_string(),
                    description: "捕获协议流量".to_string(),
                    step_type: StepType::Capture,
                    depends_on: Vec::new(),
                    estimated_duration_seconds: 300,
                });
                steps.push(ExecutionStep {
                    name: "protocol_analysis".to_string(),
                    description: "分析协议行为".to_string(),
                    step_type: StepType::Analysis,
                    depends_on: vec!["traffic_capture".to_string()],
                    estimated_duration_seconds: 300,
                });
            }
            SecurityTestType::FirmwareAnalysis => {
                steps.push(ExecutionStep {
                    name: "firmware_extraction".to_string(),
                    description: "提取固件内容".to_string(),
                    step_type: StepType::Extraction,
                    depends_on: Vec::new(),
                    estimated_duration_seconds: 120,
                });
                steps.push(ExecutionStep {
                    name: "binary_analysis".to_string(),
                    description: "分析二进制文件".to_string(),
                    step_type: StepType::Analysis,
                    depends_on: vec!["firmware_extraction".to_string()],
                    estimated_duration_seconds: 300,
                });
            }
            SecurityTestType::Exploit => {
                steps.push(ExecutionStep {
                    name: "target_validation".to_string(),
                    description: "验证目标可达性".to_string(),
                    step_type: StepType::Validation,
                    depends_on: Vec::new(),
                    estimated_duration_seconds: 30,
                });
                steps.push(ExecutionStep {
                    name: "exploit_execution".to_string(),
                    description: "执行漏洞利用".to_string(),
                    step_type: StepType::Exploit,
                    depends_on: vec!["target_validation".to_string()],
                    estimated_duration_seconds: 60,
                });
            }
            SecurityTestType::WebAppTest => {
                steps.push(ExecutionStep {
                    name: "reconnaissance".to_string(),
                    description: "Web 应用侦察".to_string(),
                    step_type: StepType::Scan,
                    depends_on: Vec::new(),
                    estimated_duration_seconds: 120,
                });
                steps.push(ExecutionStep {
                    name: "vulnerability_scan".to_string(),
                    description: "扫描 Web 漏洞".to_string(),
                    step_type: StepType::Scan,
                    depends_on: vec!["reconnaissance".to_string()],
                    estimated_duration_seconds: 300,
                });
            }
            SecurityTestType::ApiTest => {
                steps.push(ExecutionStep {
                    name: "api_discovery".to_string(),
                    description: "发现 API 端点".to_string(),
                    step_type: StepType::Scan,
                    depends_on: Vec::new(),
                    estimated_duration_seconds: 60,
                });
                steps.push(ExecutionStep {
                    name: "api_testing".to_string(),
                    description: "测试 API 安全性".to_string(),
                    step_type: StepType::Scan,
                    depends_on: vec!["api_discovery".to_string()],
                    estimated_duration_seconds: 180,
                });
            }
            _ => {
                steps.push(ExecutionStep {
                    name: "generic_scan".to_string(),
                    description: "执行通用扫描".to_string(),
                    step_type: StepType::Scan,
                    depends_on: Vec::new(),
                    estimated_duration_seconds: 300,
                });
            }
        }

        // 添加报告生成步骤
        steps.push(ExecutionStep {
            name: "report_generation".to_string(),
            description: "生成测试报告".to_string(),
            step_type: StepType::Report,
            depends_on: steps.iter().map(|s| s.name.clone()).collect(),
            estimated_duration_seconds: 30,
        });

        let total_duration = steps.iter().map(|s| s.estimated_duration_seconds).sum();

        Ok(ExecutionPlan {
            plan_id: Uuid::new_v4(),
            test_type: parsed.security_intent.test_type,
            steps,
            total_estimated_duration_seconds: total_duration,
            required_capabilities: parsed
                .security_intent
                .test_type
                .required_capabilities()
                .into_iter()
                .map(|s| s.to_string())
                .collect(),
        })
    }

    /// 估算执行时长
    fn estimate_duration(&self, test_type: &SecurityTestType) -> Option<i64> {
        let seconds = match test_type {
            SecurityTestType::PortScan => 300,
            SecurityTestType::NetworkScan => 600,
            SecurityTestType::VulnerabilityScan => 1800,
            SecurityTestType::ProtocolAnalysis => 3600,
            SecurityTestType::FirmwareAnalysis => 1800,
            SecurityTestType::WebAppTest => 2400,
            SecurityTestType::ApiTest => 1800,
            SecurityTestType::Exploit => 600,
            _ => 900,
        };
        Some(seconds / 60) // 转换为分钟
    }

    /// 选择适合的 Agent 镜像
    pub fn select_agent_image(&self, parsed: &ParsedSecurityIntent) -> String {
        match parsed.security_intent.test_type {
            SecurityTestType::NetworkScan | SecurityTestType::PortScan => {
                "uavred/agent:nmap".to_string()
            }
            SecurityTestType::VulnerabilityScan => "uavred/agent:openvas".to_string(),
            SecurityTestType::WebAppTest => "uavred/agent:burp".to_string(),
            SecurityTestType::ProtocolAnalysis => "uavred/agent:wireshark".to_string(),
            SecurityTestType::FirmwareAnalysis => "uavred/agent:binwalk".to_string(),
            SecurityTestType::ApiTest => "uavred/agent:postman".to_string(),
            _ => self.config.default_agent_image.clone(),
        }
    }

    /// 生成 Sandbox 配置
    pub fn generate_sandbox_config(
        &self,
        parsed: &ParsedSecurityIntent,
    ) -> serde_json::Value {
        let network_mode = if parsed
            .security_intent
            .params
            .get_bool("isolate_network")
            .unwrap_or(false)
        {
            "none"
        } else {
            "bridge"
        };

        let memory_limit = match parsed.security_intent.scan_config.intensity {
            ScanIntensity::Light => "512m",
            ScanIntensity::Aggressive => "4g",
            _ => "2g",
        };

        let cpu_limit = match parsed.security_intent.scan_config.intensity {
            ScanIntensity::Light => "0.5",
            ScanIntensity::Aggressive => "2.0",
            _ => "1.0",
        };

        serde_json::json!({
            "network_mode": network_mode,
            "memory_limit": memory_limit,
            "cpu_limit": cpu_limit,
            "timeout_seconds": parsed.security_intent.scan_config.timeout_seconds,
            "volumes": [
                {
                    "source": "/tmp/scans",
                    "target": "/output",
                    "read_only": false
                }
            ],
            "environment": {
                "SCAN_INTENSITY": parsed.security_intent.scan_config.intensity.as_str(),
                "DEEP_SCAN": parsed.security_intent.scan_config.deep_scan,
                "THREADS": parsed.security_intent.scan_config.threads,
            }
        })
    }
}

impl Default for IntentExecutor {
    fn default() -> Self {
        Self::new()
    }
}

/// 执行计划
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionPlan {
    /// 计划 ID
    pub plan_id: Uuid,
    /// 测试类型
    pub test_type: SecurityTestType,
    /// 执行步骤
    pub steps: Vec<ExecutionStep>,
    /// 预计总时长（秒）
    pub total_estimated_duration_seconds: u64,
    /// 所需 Agent 能力
    pub required_capabilities: Vec<String>,
}

/// 执行步骤
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionStep {
    /// 步骤名称
    pub name: String,
    /// 步骤描述
    pub description: String,
    /// 步骤类型
    pub step_type: StepType,
    /// 依赖步骤
    pub depends_on: Vec<String>,
    /// 预计时长（秒）
    pub estimated_duration_seconds: u64,
}

/// 步骤类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StepType {
    Scan,
    Analysis,
    Capture,
    Extraction,
    Validation,
    Exploit,
    Report,
    Other,
}

impl StepType {
    pub fn as_str(&self) -> &'static str {
        match self {
            StepType::Scan => "scan",
            StepType::Analysis => "analysis",
            StepType::Capture => "capture",
            StepType::Extraction => "extraction",
            StepType::Validation => "validation",
            StepType::Exploit => "exploit",
            StepType::Report => "report",
            StepType::Other => "other",
        }
    }
}
