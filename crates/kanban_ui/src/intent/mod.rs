//! Kanban UI 意图解析集成模块
//!
//! 提供在 Kanban UI 中解析用户安全测试意图的功能

pub mod integration_example;
pub mod parser_panel;
pub mod preview_card;

pub use parser_panel::IntentParserPanel;
pub use preview_card::ParsedIntentPreview;
pub use integration_example::KanbanWithIntentParser;

use core::intent_parser::{
    security::{ParsedSecurityIntent, SecurityTestType},
    ConfidenceScore,
};
use gpui::*;

/// 意图解析事件
#[derive(Debug, Clone)]
pub enum IntentParseEvent {
    /// 解析完成
    ParseCompleted(ParsedSecurityIntent),
    /// 解析失败
    ParseFailed(String),
    /// 用户确认创建任务
    CreateTask(ParsedSecurityIntent),
    /// 用户取消
    Cancelled,
}

/// 解析状态
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseState {
    /// 空闲状态
    Idle,
    /// 正在解析
    Parsing,
    /// 解析成功
    Success,
    /// 解析失败
    Error(String),
}

/// 获取测试类型显示名称
pub fn test_type_display(test_type: SecurityTestType) -> &'static str {
    test_type.display_name()
}

/// 获取测试类型图标
pub fn test_type_icon(test_type: SecurityTestType) -> &'static str {
    match test_type {
        SecurityTestType::NetworkScan => "🌐",
        SecurityTestType::PortScan => "🔌",
        SecurityTestType::VulnerabilityScan => "🛡️",
        SecurityTestType::ProtocolAnalysis => "📡",
        SecurityTestType::FirmwareAnalysis => "🔧",
        SecurityTestType::Exploit => "⚡",
        SecurityTestType::WebAppTest => "🌐",
        SecurityTestType::ApiTest => "🔌",
        SecurityTestType::WirelessTest => "📶",
        SecurityTestType::SocialEngineering => "🎭",
        SecurityTestType::ConfigurationAudit => "⚙️",
        SecurityTestType::ComplianceCheck => "✅",
        SecurityTestType::Unknown => "❓",
    }
}

/// 格式化置信度分数
pub fn format_confidence(score: ConfidenceScore) -> String {
    format!("{:.0}%", score.overall * 100.0)
}

/// 获取置信度颜色
pub fn confidence_color(score: f64) -> u32 {
    if score >= 0.8 {
        0x22c55e // green
    } else if score >= 0.6 {
        0xeab308 // yellow
    } else {
        0xef4444 // red
    }
}
