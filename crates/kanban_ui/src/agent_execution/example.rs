//! Agent Execution Panel 使用示例
//!
//! 展示如何在 Kanban UI 中集成 AI Agent 执行面板

use super::{AgentExecutionPanel, AgentExecutionSession, create_demo_session};
use crate::kanban_board::KanbanBoard;
use gpui::*;

/// 带有 Agent 执行面板的 Kanban 示例
pub struct KanbanWithAgentExecution {
    kanban_board: Entity<KanbanBoard>,
    agent_panel: Option<Entity<AgentExecutionPanel>>,
}

impl KanbanWithAgentExecution {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let kanban_board = cx.new(|_cx| KanbanBoard::new());
        
        // 创建带有示例数据的 Agent 执行面板
        let session = create_demo_session();
        let agent_panel = cx.new(|cx| AgentExecutionPanel::new(window, cx, session));

        Self {
            kanban_board,
            agent_panel: Some(agent_panel),
        }
    }

    /// 从任务创建 Agent 执行面板
    pub fn from_task(
        window: &mut Window,
        cx: &mut Context<Self>,
        task_id: u64,
        task_title: &str,
    ) -> Self {
        let kanban_board = cx.new(|_cx| KanbanBoard::new());
        let agent_panel = cx.new(|cx| {
            AgentExecutionPanel::from_task(window, cx, task_id, task_title)
        });

        Self {
            kanban_board,
            agent_panel: Some(agent_panel),
        }
    }
}

impl Render for KanbanWithAgentExecution {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .child(self.kanban_board.clone())
    }
}

// 在 TaskDetailPanel 中显示 Agent 执行的示例:
// let agent_panel = cx.new(|cx| {
//     AgentExecutionPanel::from_task(window, cx, task.id, &task.title)
// });

// 实时更新 Agent 执行的示例代码：
// panel.update(cx, |panel, cx| {
//     panel.add_message(new_message, cx);
// });
