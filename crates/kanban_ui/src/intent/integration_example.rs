//! Kanban UI + Intent Parser 集成示例
//!
//! 这个文件展示了如何将 IntentParserPanel 与 KanbanBoard 集成
//! 在实际应用中，你可以参考这个实现

use crate::intent::{IntentParserPanel, IntentParseEvent};
use crate::KanbanBoard;
use core::intent_parser::parser::AiProvider;
use gpui::*;
use std::sync::Arc;

/// 示例：集成看板和意图解析的主组件
pub struct KanbanWithIntentParser {
    /// 看板组件
    kanban_board: Entity<KanbanBoard>,
    /// 意图解析面板（可选，当添加任务时显示）
    intent_parser_panel: Option<Entity<IntentParserPanel>>,
    /// AI Provider
    ai_provider: Option<Arc<dyn AiProvider>>,
    /// 当前选中的状态（用于添加任务）
    current_status: Option<data::TaskStatus>,
}

impl KanbanWithIntentParser {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        // 创建看板
        let kanban_board = cx.new(|_cx| KanbanBoard::new());
        
        // 创建意图解析面板
        let intent_parser_panel = cx.new(|cx| IntentParserPanel::new(window, cx));

        Self {
            kanban_board,
            intent_parser_panel: Some(intent_parser_panel),
            ai_provider: None,
            current_status: None,
        }
    }

    /// 设置 AI Provider
    pub fn with_ai_provider(mut self, provider: Arc<dyn AiProvider>) -> Self {
        self.ai_provider = Some(provider);
        self
    }
}

impl Render for KanbanWithIntentParser {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        // 这里应该渲染看板和意图解析面板
        // 实际使用时，IntentParserPanel 会在对话框中显示
        div()
            .size_full()
            .child(self.kanban_board.clone())
    }
}

impl EventEmitter<IntentParseEvent> for KanbanWithIntentParser {}
