//! Kanban UI 模块 - 5 列看板 + Squeeze-style 详情面板
//!
//! ## 组件结构
//!
//! - `KanbanBoard`: 主容器，管理 5 列和详情面板
//! - `KanbanColumn`: 单个状态列，显示任务卡片
//! - `TaskCard`: 可拖拽任务卡片，显示任务摘要
//! - `TaskDetailPanel`: Squeeze-style 详情面板，显示任务完整信息
//!
//! ## 功能特性
//!
//! - 5 列布局（Todo, In Progress, In Review, Done, Canceled）
//! - 拖拽移动任务到不同状态
//! - 任务选择和高亮
//! - Squeeze-style 详情面板动画
//! - 实时搜索和过滤
//! - 键盘导航（箭头键、Enter、Tab、ESC）

pub mod agent_execution;
pub mod animations;
pub mod intent;
pub mod kanban_board;
pub mod kanban_column;
pub mod task_card;
pub mod task_detail;

pub use agent_execution::{AgentExecutionPanel, AgentExecutionSession, AgentExecutionStatus};
pub use kanban_board::{KanbanBoard, KanbanEvent};
pub use kanban_column::KanbanColumn;
pub use intent::{IntentParserPanel, IntentParseEvent, ParseState, KanbanWithIntentParser};
pub use task_card::{DragTask, TaskCard};
pub use task_detail::TaskDetailPanel;
