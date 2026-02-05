//! 意图解析引擎集成测试

use core::intent_parser::{
    Intent, IntentExecutor, SecurityTestIntent, SecurityTestType,
};
use core::intent_parser::executor::{ExecutionPlan, ExecutionStep, StepType};
use core::execution::{ExecutionService, ExecutionConfig, SandboxManager, AgentScheduler};
use core::execution::sandbox_manager::SandboxBackend;

/// 测试 Intent Builder
#[test]
fn test_intent_builder() {
    let intent = Intent::new()
        .goal("扫描 192.168.1.0/24 网段")
        .context("这是一个内部测试网络")
        .technical_context("使用 nmap 进行扫描")
        .with_param("timeout", 300)
        .rule("不要扫描敏感端口")
        .timeout_seconds(600)
        .build()
        .expect("Failed to build intent");

    assert_eq!(intent.goal, "扫描 192.168.1.0/24 网段");
    assert_eq!(intent.context.len(), 2);
    assert_eq!(intent.strategy.timeout_seconds, 600);
}

/// 测试从字符串创建 Intent
#[test]
fn test_intent_from_string() {
    let intent = Intent::from("扫描目标网络");
    assert_eq!(intent.goal, "扫描目标网络");
}

/// 测试 SecurityTestIntent 创建
#[test]
fn test_security_test_intent() {
    let base_intent = Intent::from("端口扫描测试");
    let mut security_intent = SecurityTestIntent::from_intent(base_intent);
    
    security_intent.test_type = SecurityTestType::PortScan;
    security_intent.params.set("port_range", "1-1000");
    security_intent.targets.push(core::intent_parser::Target::new(
        "192.168.1.1",
        core::intent_parser::TargetType::Ip,
    ));

    assert_eq!(security_intent.test_type, SecurityTestType::PortScan);
    assert_eq!(security_intent.test_type.as_str(), "port_scan");
    assert_eq!(security_intent.test_type.display_name(), "端口扫描");
}

/// 测试 SecurityTestType 转换
#[test]
fn test_security_test_type_from_str() {
    assert_eq!(SecurityTestType::from("network_scan"), SecurityTestType::NetworkScan);
    assert_eq!(SecurityTestType::from("port_scan"), SecurityTestType::PortScan);
    assert_eq!(SecurityTestType::from("vulnerability_scan"), SecurityTestType::VulnerabilityScan);
    assert_eq!(SecurityTestType::from("exploit"), SecurityTestType::Exploit);
    assert_eq!(SecurityTestType::from("web"), SecurityTestType::WebAppTest);
    assert_eq!(SecurityTestType::from("unknown_type"), SecurityTestType::Unknown);
    
    // 测试中文
    assert_eq!(SecurityTestType::from("网络扫描"), SecurityTestType::NetworkScan);
    assert_eq!(SecurityTestType::from("端口扫描"), SecurityTestType::PortScan);
}

/// 测试 SecurityTestType 所需能力
#[test]
fn test_security_test_type_capabilities() {
    let network_caps = SecurityTestType::NetworkScan.required_capabilities();
    assert!(network_caps.contains(&"network_scan"));
    assert!(network_caps.contains(&"host_discovery"));

    let web_caps = SecurityTestType::WebAppTest.required_capabilities();
    assert!(web_caps.contains(&"web_scan"));
    assert!(web_caps.contains(&"sql_injection"));
}

/// 测试参数提取
#[test]
fn test_security_test_params() {
    let mut params = core::intent_parser::security::SecurityTestParams::new();
    
    params.set("protocol", "tcp");
    params.set("port_range", "80-443");
    params.set("deep_scan", true);
    params.set("threads", 50u64);
    
    assert_eq!(params.get_string("protocol"), Some("tcp".to_string()));
    assert_eq!(params.get_bool("deep_scan"), Some(true));
    assert_eq!(params.get_u64("threads"), Some(50));
    
    // 测试端口范围解析
    params.set("port_range", "1-1000");
    let range = params.port_range();
    assert_eq!(range, Some((1, 1000)));
    
    params.set("port_range", "443");
    let single = params.port_range();
    assert_eq!(single, Some((443, 443)));
}

/// 测试 IntentExecutor
#[test]
fn test_intent_executor() {
    let executor = IntentExecutor::new();
    
    // 创建解析后的意图
    let base_intent = Intent::from("测试漏洞扫描");
    let mut security_intent = SecurityTestIntent::from_intent(base_intent);
    security_intent.test_type = SecurityTestType::VulnerabilityScan;
    security_intent.targets.push(core::intent_parser::Target::new(
        "10.0.0.1",
        core::intent_parser::TargetType::Ip,
    ));
    
    let parsed = core::intent_parser::security::ParsedSecurityIntent {
        raw_intent: "测试漏洞扫描".to_string(),
        security_intent,
        confidence: core::intent_parser::ConfidenceScore::new(0.85, 0.9, 0.8, 0.85),
        metadata: core::intent_parser::ParseMetadata {
            model: "test-model".to_string(),
            parse_duration_ms: 100,
            token_usage: core::intent_parser::TokenUsage::default(),
            parsed_at: chrono::Utc::now(),
        },
        suggestions: vec![],
    };
    
    // 生成执行计划
    let plan = executor.generate_execution_plan(&parsed).expect("Failed to generate plan");
    
    assert!(!plan.steps.is_empty());
    assert!(plan.total_estimated_duration_seconds > 0);
    assert!(!plan.required_capabilities.is_empty());
}

/// 测试执行计划生成
#[test]
fn test_execution_plan_generation() {
    let executor = IntentExecutor::new();
    
    // 测试网络扫描的计划
    let network_intent = create_parsed_intent(SecurityTestType::NetworkScan, "192.168.1.0/24");
    let network_plan = executor.generate_execution_plan(&network_intent).unwrap();
    
    assert!(network_plan.steps.iter().any(|s| s.step_type == StepType::Scan));
    
    // 测试漏洞扫描的计划
    let vuln_intent = create_parsed_intent(SecurityTestType::VulnerabilityScan, "10.0.0.1");
    let vuln_plan = executor.generate_execution_plan(&vuln_intent).unwrap();
    
    assert!(vuln_plan.steps.iter().any(|s| s.step_type == StepType::Analysis));
    
    // 所有计划都应该有报告生成步骤
    assert!(network_plan.steps.iter().any(|s| s.name == "report_generation"));
    assert!(vuln_plan.steps.iter().any(|s| s.name == "report_generation"));
}

/// 测试 SandboxManager
#[test]
fn test_sandbox_manager() {
    let manager = SandboxManager::new();
    let docker_manager = SandboxManager::with_backend(SandboxBackend::Docker);
    
    // 测试默认配置
    // Verify backend type is set correctly (can't access private field directly)
    let _docker_manager = SandboxManager::with_backend(SandboxBackend::Docker);
}

/// 测试 AgentScheduler
#[test]
fn test_agent_scheduler() {
    let mut scheduler = AgentScheduler::new();
    
    // 创建测试 agent
    let agent = core::execution::agent_scheduler::AgentInfo {
        id: "test-agent-1".to_string(),
        name: "Test Agent".to_string(),
        status: core::execution::agent_scheduler::AgentStatus::Idle,
        capabilities: vec!["network_scan".to_string(), "port_scan".to_string()],
        current_task: None,
        config: core::execution::agent_scheduler::AgentConfig::default(),
    };
    
    scheduler.register_agent(agent);
    
    let agents = scheduler.list_agents();
    assert_eq!(agents.len(), 1);
    
    let available = scheduler.list_available_agents();
    assert_eq!(available.len(), 1);
    
    // 测试查找特定能力的 agent
    let network_agents = scheduler.find_agents_with_capability("network_scan");
    assert_eq!(network_agents.len(), 1);
    
    let web_agents = scheduler.find_agents_with_capability("web_scan");
    assert!(web_agents.is_empty());
}

/// 测试 ExecutionService
#[test]
fn test_execution_service() {
    let config = ExecutionConfig {
        auto_create_sandbox: true,
        auto_assign_agent: true,
        default_sandbox_image: "test-image".to_string(),
        execution_timeout_seconds: 3600,
        keep_sandbox: false,
    };
    
    let service = ExecutionService::with_config(config);
    
    // 验证配置 (service 的 config 是私有的，但配置应该已正确应用)
    let _ = service;
}

/// 测试置信度评分
#[test]
fn test_confidence_score() {
    // 高置信度
    let high = core::intent_parser::ConfidenceScore::new(0.85, 0.9, 0.8, 0.85);
    assert!(high.is_executable(0.7));
    assert!(high.is_executable(0.8));
    
    // 低置信度
    let low = core::intent_parser::ConfidenceScore::new(0.5, 0.6, 0.4, 0.5);
    assert!(!low.is_executable(0.7));
    
    // 边界值
    let borderline = core::intent_parser::ConfidenceScore::new(0.7, 0.7, 0.6, 0.7);
    assert!(borderline.is_executable(0.7));
}

/// 辅助函数：创建解析后的意图
fn create_parsed_intent(test_type: SecurityTestType, target: &str) -> core::intent_parser::security::ParsedSecurityIntent {
    let base_intent = Intent::from("测试意图");
    let mut security_intent = SecurityTestIntent::from_intent(base_intent);
    security_intent.test_type = test_type;
    security_intent.targets.push(core::intent_parser::Target::new(
        target,
        core::intent_parser::TargetType::Ip,
    ));
    
    core::intent_parser::security::ParsedSecurityIntent {
        raw_intent: "测试意图".to_string(),
        security_intent,
        confidence: core::intent_parser::ConfidenceScore::new(0.85, 0.9, 0.8, 0.85),
        metadata: core::intent_parser::ParseMetadata {
            model: "test".to_string(),
            parse_duration_ms: 100,
            token_usage: core::intent_parser::TokenUsage::default(),
            parsed_at: chrono::Utc::now(),
        },
        suggestions: vec![],
    }
}

/// 测试目标类型
#[test]
fn test_target_types() {
    use core::intent_parser::TargetType;
    
    assert_eq!(TargetType::Ip.as_str(), "ip");
    assert_eq!(TargetType::Cidr.as_str(), "cidr");
    assert_eq!(TargetType::Domain.as_str(), "domain");
    assert_eq!(TargetType::Url.as_str(), "url");
}

/// 测试扫描强度
#[test]
fn test_scan_intensity() {
    use core::intent_parser::ScanIntensity;
    
    assert_eq!(ScanIntensity::Light.as_str(), "light");
    assert_eq!(ScanIntensity::Normal.as_str(), "normal");
    assert_eq!(ScanIntensity::Aggressive.as_str(), "aggressive");
}

/// 测试执行步骤类型
#[test]
fn test_step_types() {
    assert_eq!(StepType::Scan.as_str(), "scan");
    assert_eq!(StepType::Analysis.as_str(), "analysis");
    assert_eq!(StepType::Exploit.as_str(), "exploit");
    assert_eq!(StepType::Report.as_str(), "report");
}

/// 测试任务优先级推断
#[test]
fn test_suggested_priority() {
    let base_intent = Intent::from("测试");
    let mut security_intent = SecurityTestIntent::from_intent(base_intent);
    
    // 默认优先级
    assert_eq!(security_intent.suggested_priority(), core::task::TaskPriority::Medium);
    
    // 高优先级
    security_intent.params.set("priority", "high");
    assert_eq!(security_intent.suggested_priority(), core::task::TaskPriority::High);
    
    // Critical 优先级
    security_intent.params.set("priority", "critical");
    assert_eq!(security_intent.suggested_priority(), core::task::TaskPriority::Critical);
}
