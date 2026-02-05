//! 意图执行服务 - 协调解析后的意图执行
//!
//! 负责：
//! 1. 将解析后的意图转换为可执行任务
//! 2. 创建和管理 sandbox
//! 3. 分配 agent 执行任务
//! 4. 监控执行状态和结果

pub mod service;
pub mod sandbox_manager;
pub mod agent_scheduler;

pub use service::ExecutionService;
pub use sandbox_manager::SandboxManager;
pub use agent_scheduler::AgentScheduler;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 执行状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExecutionStatus {
    /// 待执行
    Pending,
    /// 准备中（创建 sandbox 等）
    Preparing,
    /// 运行中
    Running,
    /// 已完成
    Completed,
    /// 失败
    Failed,
    /// 已取消
    Cancelled,
}

impl ExecutionStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            ExecutionStatus::Pending => "pending",
            ExecutionStatus::Preparing => "preparing",
            ExecutionStatus::Running => "running",
            ExecutionStatus::Completed => "completed",
            ExecutionStatus::Failed => "failed",
            ExecutionStatus::Cancelled => "cancelled",
        }
    }

    pub fn is_active(&self) -> bool {
        matches!(
            self,
            ExecutionStatus::Pending | ExecutionStatus::Preparing | ExecutionStatus::Running
        )
    }
}

/// 执行上下文
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionContext {
    /// 执行 ID
    pub execution_id: Uuid,
    /// 任务 ID
    pub task_id: i64,
    /// 用户 ID
    pub user_id: Option<i64>,
    /// 执行状态
    pub status: ExecutionStatus,
    /// Sandbox ID
    pub sandbox_id: Option<String>,
    /// Agent ID
    pub agent_id: Option<String>,
    /// 开始时间
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    /// 完成时间
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    /// 执行结果
    pub result: Option<ExecutionResult>,
}

/// 执行结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    /// 是否成功
    pub success: bool,
    /// 退出码
    pub exit_code: Option<i32>,
    /// 输出
    pub output: String,
    /// 错误信息
    pub error_message: Option<String>,
    /// 发现数量
    pub findings_count: u32,
    /// 报告路径
    pub report_path: Option<String>,
}

/// 执行配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionConfig {
    /// 自动创建 sandbox
    pub auto_create_sandbox: bool,
    /// 自动分配 agent
    pub auto_assign_agent: bool,
    /// 默认 sandbox 镜像
    pub default_sandbox_image: String,
    /// 执行超时（秒）
    pub execution_timeout_seconds: u64,
    /// 是否保留 sandbox
    pub keep_sandbox: bool,
}

impl Default for ExecutionConfig {
    fn default() -> Self {
        Self {
            auto_create_sandbox: true,
            auto_assign_agent: true,
            default_sandbox_image: "uavred/agent:latest".to_string(),
            execution_timeout_seconds: 3600,
            keep_sandbox: false,
        }
    }
}
