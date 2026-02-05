//! Agent 调度器 - 分配 agent 执行安全测试任务

use crate::intent_parser::security::ParsedSecurityIntent;

/// Agent 调度器
pub struct AgentScheduler {
    /// Agent 注册表
    agents: std::collections::HashMap<String, AgentInfo>,
    /// 默认 Agent 配置
    default_config: AgentConfig,
}

/// Agent 信息
#[derive(Debug, Clone)]
pub struct AgentInfo {
    /// Agent ID
    pub id: String,
    /// Agent 名称
    pub name: String,
    /// 状态
    pub status: AgentStatus,
    /// 能力列表
    pub capabilities: Vec<String>,
    /// 当前任务
    pub current_task: Option<String>,
    /// 配置
    pub config: AgentConfig,
}

/// Agent 状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentStatus {
    /// 空闲
    Idle,
    /// 忙碌
    Busy,
    /// 离线
    Offline,
    /// 错误
    Error,
}

impl AgentStatus {
    pub fn is_available(&self) -> bool {
        matches!(self, AgentStatus::Idle)
    }
}

/// Agent 配置
#[derive(Debug, Clone)]
pub struct AgentConfig {
    /// 最大并发任务数
    pub max_concurrent_tasks: usize,
    /// 任务超时（秒）
    pub task_timeout_seconds: u64,
    /// 资源限制
    pub resource_limits: ResourceLimits,
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            max_concurrent_tasks: 1,
            task_timeout_seconds: 3600,
            resource_limits: ResourceLimits::default(),
        }
    }
}

/// 资源限制
#[derive(Debug, Clone)]
pub struct ResourceLimits {
    /// 内存限制（MB）
    pub memory_mb: u64,
    /// CPU 限制（核心数）
    pub cpu_cores: f64,
    /// 磁盘限制（MB）
    pub disk_mb: u64,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            memory_mb: 2048,
            cpu_cores: 1.0,
            disk_mb: 10240,
        }
    }
}

/// 任务分配结果
#[derive(Debug, Clone)]
pub struct TaskAssignment {
    /// 任务 ID
    pub task_id: String,
    /// 分配的 Agent ID
    pub agent_id: String,
    /// 预期开始时间
    pub expected_start: chrono::DateTime<chrono::Utc>,
    /// 预期完成时间
    pub expected_completion: chrono::DateTime<chrono::Utc>,
}

impl AgentScheduler {
    /// 创建新的调度器
    pub fn new() -> Self {
        Self {
            agents: std::collections::HashMap::new(),
            default_config: AgentConfig::default(),
        }
    }

    /// 注册 Agent
    pub fn register_agent(&mut self, info: AgentInfo) {
        tracing::info!("Registering agent: {} ({})", info.name, info.id);
        self.agents.insert(info.id.clone(), info);
    }

    /// 注销 Agent
    pub fn unregister_agent(&mut self, agent_id: &str) {
        tracing::info!("Unregistering agent: {}", agent_id);
        self.agents.remove(agent_id);
    }

    /// 更新 Agent 状态
    pub fn update_agent_status(&mut self, agent_id: &str, status: AgentStatus) {
        if let Some(agent) = self.agents.get_mut(agent_id) {
            agent.status = status;
            tracing::debug!("Updated agent {} status to {:?}", agent_id, status);
        }
    }

    /// 分配 agent 执行意图
    pub async fn assign_agent(
        &self,
        parsed: &ParsedSecurityIntent,
        sandbox_id: Option<&str>,
    ) -> anyhow::Result<String> {
        let intent = &parsed.security_intent;
        
        // 1. 确定所需能力
        let required_capabilities = intent.test_type.required_capabilities();
        
        // 2. 查找匹配的可用 agent
        let available_agents: Vec<&AgentInfo> = self.agents
            .values()
            .filter(|a| {
                a.status.is_available()
                    && has_required_capabilities(&a.capabilities, &required_capabilities)
            })
            .collect();

        if available_agents.is_empty() {
            // 没有可用 agent，创建新的
            return self.create_agent_for_intent(parsed, sandbox_id).await;
        }

        // 3. 选择最佳 agent（这里选择第一个匹配的，可以扩展为更复杂的策略）
        let selected = available_agents[0];
        
        tracing::info!(
            "Assigned agent {} to task type {:?}",
            selected.id,
            intent.test_type
        );

        Ok(selected.id.clone())
    }

    /// 为意图创建新的 agent
    async fn create_agent_for_intent(
        &self,
        parsed: &ParsedSecurityIntent,
        sandbox_id: Option<&str>,
    ) -> anyhow::Result<String> {
        let intent = &parsed.security_intent;
        let agent_id = format!("agent-{}-auto", uuid::Uuid::new_v4());
        
        tracing::info!(
            "Creating new agent {} for task type {:?} in sandbox {:?}",
            agent_id,
            intent.test_type,
            sandbox_id
        );

        // TODO: 集成 agent crate 的 Agent 创建逻辑
        // 这里应该调用 agent crate 的 API 来创建新的 Agent

        Ok(agent_id)
    }

    /// 获取所有 agent
    pub fn list_agents(&self) -> Vec<&AgentInfo> {
        self.agents.values().collect()
    }

    /// 获取可用 agent 列表
    pub fn list_available_agents(&self) -> Vec<&AgentInfo> {
        self.agents
            .values()
            .filter(|a| a.status.is_available())
            .collect()
    }

    /// 获取特定能力的 agent
    pub fn find_agents_with_capability(&self, capability: &str) -> Vec<&AgentInfo> {
        self.agents
            .values()
            .filter(|a| a.capabilities.contains(&capability.to_string()))
            .collect()
    }

    /// 释放 agent
    pub fn release_agent(&mut self, agent_id: &str) {
        if let Some(agent) = self.agents.get_mut(agent_id) {
            agent.status = AgentStatus::Idle;
            agent.current_task = None;
            tracing::info!("Released agent: {}", agent_id);
        }
    }
}

impl Default for AgentScheduler {
    fn default() -> Self {
        Self::new()
    }
}

/// 检查 agent 是否有所需能力
fn has_required_capabilities(agent_caps: &[String], required: &[&str]) -> bool {
    required.iter().all(|req| {
        agent_caps.iter().any(|cap| cap.as_str() == *req)
    })
}

/// 创建默认的 Agent 注册
pub fn create_default_agents() -> Vec<AgentInfo> {
    vec![
        AgentInfo {
            id: "agent-network-1".to_string(),
            name: "Network Scanner".to_string(),
            status: AgentStatus::Idle,
            capabilities: vec![
                "network_scan".to_string(),
                "port_scan".to_string(),
                "host_discovery".to_string(),
            ],
            current_task: None,
            config: AgentConfig::default(),
        },
        AgentInfo {
            id: "agent-vuln-1".to_string(),
            name: "Vulnerability Scanner".to_string(),
            status: AgentStatus::Idle,
            capabilities: vec![
                "vuln_scan".to_string(),
                "cve_lookup".to_string(),
                "service_detection".to_string(),
            ],
            current_task: None,
            config: AgentConfig::default(),
        },
        AgentInfo {
            id: "agent-web-1".to_string(),
            name: "Web App Tester".to_string(),
            status: AgentStatus::Idle,
            capabilities: vec![
                "web_scan".to_string(),
                "sql_injection".to_string(),
                "xss_detection".to_string(),
            ],
            current_task: None,
            config: AgentConfig::default(),
        },
    ]
}
