// 工作区核心逻辑 - 共享数据结构和工作区状态管理

use serde::{Deserialize, Serialize};

/// 任务数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskData {
    pub id: usize,
    pub title: String,
    pub task_type: String,
    pub priority: String,
    pub status: String,
    /// 任务目标/描述
    pub mission_objective: Option<String>,
    /// 元数据 JSON
    pub metadata: Option<String>,
    /// 任务来源
    pub source: String,
}

impl TaskData {
    pub fn new(
        id: usize,
        title: String,
        task_type: String,
        priority: String,
        status: String,
    ) -> Self {
        Self {
            id,
            title,
            task_type,
            priority,
            status,
            mission_objective: None,
            metadata: None,
            source: "manual".to_string(),
        }
    }

    /// 是否是 Agent 创建的任务
    pub fn is_agent_task(&self) -> bool {
        self.source == "agent"
    }

    /// 获取元数据 JSON
    pub fn get_metadata(&self) -> Option<serde_json::Value> {
        self.metadata.as_ref()
            .and_then(|m| serde_json::from_str(m).ok())
    }

    /// 设置元数据
    pub fn set_metadata(&mut self, value: serde_json::Value) {
        self.metadata = Some(value.to_string());
    }
}

/// 漏洞过滤器
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum VulnFilter {
    All,
    Critical,
    High,
    Medium,
    Low,
}

/// Dashboard 视图
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum DashboardView {
    MissionControl,
    Findings,
}

/// 应用视图
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum AppView {
    Dashboard,
    Assets,
    Images,
    Vulns,
    Traffic,
    Flows,
    Devices,
    Settings,
}
