//! Agent Execution Data Model
//!
//! 定义 AI Agent 执行过程中的数据结构和事件类型

use chrono::{DateTime, Local};
use gpui::SharedString;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

/// Agent 执行状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentExecutionStatus {
    /// 等待开始
    Pending,
    /// 正在执行
    Running,
    /// 已完成
    Completed,
    /// 已暂停
    Paused,
    /// 出错
    Error,
}

impl AgentExecutionStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pending => "等待中",
            Self::Running => "执行中",
            Self::Completed => "已完成",
            Self::Paused => "已暂停",
            Self::Error => "出错",
        }
    }

    pub fn color(&self) -> u32 {
        match self {
            Self::Pending => 0x9ca3af,    // gray
            Self::Running => 0x3b82f6,    // blue
            Self::Completed => 0x22c55e,  // green
            Self::Paused => 0xf59e0b,     // yellow
            Self::Error => 0xef4444,      // red
        }
    }
}

/// 消息类型（对应截图中的各类标签）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentMessageType {
    /// 历史记录
    History,
    /// 思考过程
    Thought,
    /// 计划
    Plan,
    /// 工具执行
    Tool,
    /// 分析结果
    Analysis,
    /// 系统消息
    System,
}

impl AgentMessageType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::History => "HISTORY",
            Self::Thought => "THOUGHT",
            Self::Plan => "PLAN",
            Self::Tool => "TOOL",
            Self::Analysis => "ANALYSIS",
            Self::System => "SYSTEM",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Self::History => "历史",
            Self::Thought => "思考",
            Self::Plan => "计划",
            Self::Tool => "工具",
            Self::Analysis => "分析",
            Self::System => "系统",
        }
    }

    /// 标签颜色
    pub fn tag_color(&self) -> (u32, u32) {
        match self {
            // (background, text)
            Self::History => (0xe5e7eb, 0x374151),      // gray
            Self::Thought => (0xe9d5ff, 0x7c3aed),      // purple
            Self::Plan => (0xfef3c7, 0xd97706),         // amber
            Self::Tool => (0xcffafe, 0x0891b2),         // cyan
            Self::Analysis => (0xfecaca, 0xdc2626),     // red
            Self::System => (0xd1d5db, 0x4b5563),       // gray
        }
    }
}

/// 工具执行状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToolExecutionStatus {
    /// 正在执行
    Running,
    /// 成功
    Success,
    /// 失败
    Failed,
}

/// Agent 消息项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMessage {
    /// 唯一ID
    pub id: Uuid,
    /// 消息类型
    pub message_type: AgentMessageType,
    /// 时间戳
    pub timestamp: DateTime<Local>,
    /// 内容
    pub content: String,
    /// 附加数据（工具输出、分析结果等）
    pub metadata: Option<AgentMessageMetadata>,
}

impl AgentMessage {
    pub fn new(message_type: AgentMessageType, content: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            message_type,
            timestamp: Local::now(),
            content: content.into(),
            metadata: None,
        }
    }

    pub fn with_metadata(mut self, metadata: AgentMessageMetadata) -> Self {
        self.metadata = Some(metadata);
        self
    }

    /// 格式化时间显示
    pub fn formatted_time(&self) -> String {
        self.timestamp.format("%H:%M:%S").to_string()
    }
}

/// 消息附加元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgentMessageMetadata {
    /// 工具执行信息
    Tool {
        /// 工具名称
        tool_name: String,
        /// 命令
        command: String,
        /// 输出
        output: String,
        /// 状态
        status: ToolExecutionStatus,
    },
    /// 分析结果
    Analysis {
        /// 严重级别 (1-10)
        severity: u8,
        /// 发现的漏洞
        findings: Vec<String>,
        /// 建议
        recommendations: Vec<String>,
    },
    /// 计划步骤
    Plan {
        /// 步骤序号
        step_number: usize,
        /// 总步骤数
        total_steps: usize,
    },
}

/// 任务目标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionObjective {
    /// 目标标题
    pub title: String,
    /// 详细描述（支持多行）
    pub descriptions: Vec<String>,
    /// 任务ID
    pub task_id: u64,
}

impl MissionObjective {
    pub fn new(title: impl Into<String>, task_id: u64) -> Self {
        Self {
            title: title.into(),
            descriptions: Vec::new(),
            task_id,
        }
    }

    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.descriptions.push(desc.into());
        self
    }
}

/// Agent 执行会话
#[derive(Debug, Clone)]
pub struct AgentExecutionSession {
    /// 会话ID
    pub id: Uuid,
    /// Agent 名称
    pub agent_name: String,
    /// 执行状态
    pub status: AgentExecutionStatus,
    /// 任务目标
    pub objective: MissionObjective,
    /// 消息历史
    pub messages: Vec<AgentMessage>,
    /// 是否实时追踪
    pub live_trace: bool,
    /// 开始时间
    pub started_at: Option<DateTime<Local>>,
    /// 结束时间
    pub ended_at: Option<DateTime<Local>>,
}

impl AgentExecutionSession {
    pub fn new(agent_name: impl Into<String>, objective: MissionObjective) -> Self {
        Self {
            id: Uuid::new_v4(),
            agent_name: agent_name.into(),
            status: AgentExecutionStatus::Pending,
            objective,
            messages: Vec::new(),
            live_trace: true,
            started_at: None,
            ended_at: None,
        }
    }

    pub fn start(&mut self) {
        self.status = AgentExecutionStatus::Running;
        self.started_at = Some(Local::now());
    }

    pub fn complete(&mut self) {
        self.status = AgentExecutionStatus::Completed;
        self.ended_at = Some(Local::now());
    }

    pub fn add_message(&mut self, message: AgentMessage) {
        self.messages.push(message);
    }

    /// 添加历史消息
    pub fn add_history(&mut self, content: impl Into<String>) {
        self.add_message(AgentMessage::new(AgentMessageType::History, content));
    }

    /// 添加思考消息
    pub fn add_thought(&mut self, content: impl Into<String>) {
        self.add_message(AgentMessage::new(AgentMessageType::Thought, content));
    }

    /// 添加计划消息
    pub fn add_plan(&mut self, content: impl Into<String>, step: usize, total: usize) {
        let msg = AgentMessage::new(AgentMessageType::Plan, content)
            .with_metadata(AgentMessageMetadata::Plan {
                step_number: step,
                total_steps: total,
            });
        self.add_message(msg);
    }

    /// 添加工具执行消息
    pub fn add_tool_execution(
        &mut self,
        tool_name: impl Into<String>,
        command: impl Into<String>,
        output: impl Into<String>,
        status: ToolExecutionStatus,
    ) {
        let msg = AgentMessage::new(AgentMessageType::Tool, "")
            .with_metadata(AgentMessageMetadata::Tool {
                tool_name: tool_name.into(),
                command: command.into(),
                output: output.into(),
                status,
            });
        self.add_message(msg);
    }

    /// 添加分析结果
    pub fn add_analysis(
        &mut self,
        content: impl Into<String>,
        severity: u8,
        findings: Vec<String>,
        recommendations: Vec<String>,
    ) {
        let msg = AgentMessage::new(AgentMessageType::Analysis, content)
            .with_metadata(AgentMessageMetadata::Analysis {
                severity,
                findings,
                recommendations,
            });
        self.add_message(msg);
    }

    /// 获取最后一条消息
    pub fn last_message(&self) -> Option<&AgentMessage> {
        self.messages.last()
    }

    /// 获取执行时长（秒）
    pub fn duration_seconds(&self) -> i64 {
        match (self.started_at, self.ended_at) {
            (Some(start), Some(end)) => (end - start).num_seconds(),
            (Some(start), None) => (Local::now() - start).num_seconds(),
            _ => 0,
        }
    }
}

/// Agent 执行事件（用于与后端通信）
#[derive(Debug, Clone)]
pub enum AgentExecutionEvent {
    /// 新消息
    NewMessage(AgentMessage),
    /// 状态变更
    StatusChanged(AgentExecutionStatus),
    /// 执行完成
    Completed,
    /// 执行错误
    Error(String),
}
