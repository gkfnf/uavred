//! Agent Execution UI Module
//!
//! 提供 AI Agent 执行过程的可视化界面，包括：
//! - 任务目标显示 (Mission Objective)
//! - 实时执行追踪 (Live Trace)
//! - 历史消息时间线 (History Timeline)
//! - 思考过程展示 (Thought)
//! - 计划步骤 (Plan)
//! - 工具执行 (Tool Execution)
//! - 分析结果 (Analysis)

pub mod example;
pub mod model;
pub mod panel;

pub use model::*;
pub use panel::{AgentExecutionPanel, create_demo_session};
pub use example::KanbanWithAgentExecution;

// 导出常用类型
pub use model::{
    AgentExecutionStatus,
    AgentMessageType,
    AgentMessage,
    AgentExecutionSession,
    MissionObjective,
    ToolExecutionStatus,
};
