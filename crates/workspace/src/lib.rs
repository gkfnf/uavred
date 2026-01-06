// 工作区核心逻辑 - 共享数据结构和工作区状态管理

use serde::{Deserialize, Serialize};

/// 任务数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskData {
    pub id: usize,
    pub title: String,
    pub task_type: String,
    pub priority: String,
}

impl TaskData {
    pub fn new(id: usize, title: String, task_type: String, priority: String) -> Self {
        Self {
            id,
            title,
            task_type,
            priority,
        }
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
