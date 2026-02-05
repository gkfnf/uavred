//! Intent Parser Engine - 意图解析引擎
//!
//! 将自然语言安全测试意图解析为结构化的任务定义
//!
//! ## 设计灵感
//! 基于 intentlang 的意图驱动编程理念，将用户的自然语言输入转换为
//! 可执行的安全测试任务。
//!
//! ## 核心概念
//! - `Intent`: 用户意图的完整表示
//! - `IntentParser`: 解析引擎，使用 AI 将自然语言转换为结构化数据
//! - `SecurityTestIntent`: 安全测试特定的意图类型
//!
//! ## 使用示例
//! ```rust,no_run
//! use core::intent_parser::{Intent, IntentParser, SecurityTestType};
//!
//! let intent = Intent::new()
//!     .goal("扫描 192.168.1.0/24 网段的所有开放端口")
//!     .context("这是一个内部测试网络")
//!     .build();
//!
//! let parser = IntentParser::new(ai_provider);
//! let result = parser.parse_security_test(intent).await?;
//! ```

pub mod ai_adapter;
pub mod error;
pub mod executor;
pub mod intent;
pub mod parser;
pub mod security;

pub use ai_adapter::{AiProviderAdapter, create_adapter, create_adapter_from_registry};
pub use error::{IntentParseError, IntentExecutionError};
pub use executor::{IntentExecutor, ExecutionPlan, ExecutionStep};
pub use intent::{Intent, IntentBuilder, IntentInput, IntentOutput, IntentContext, ExecutionStrategy, Constraint, ContextType};
pub use parser::{IntentParser, ParseResult, ParserConfig, AiProvider};
pub use security::{SecurityTestIntent, SecurityTestType, SecurityTestParams, ParsedSecurityIntent, ScanIntensity, Target, TargetType, ScanConfig};

use serde::{Deserialize, Serialize};

/// 意图解析引擎版本
pub const INTENT_PARSER_VERSION: &str = "0.1.0";

/// 支持的意图类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IntentCategory {
    /// 安全测试意图
    SecurityTest,
    /// 信息查询意图
    InformationQuery,
    /// 配置管理意图
    Configuration,
    /// 未知/通用意图
    Unknown,
}

impl IntentCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            IntentCategory::SecurityTest => "security_test",
            IntentCategory::InformationQuery => "information_query",
            IntentCategory::Configuration => "configuration",
            IntentCategory::Unknown => "unknown",
        }
    }
}

/// 意图置信度评分
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ConfidenceScore {
    /// 整体置信度 (0.0 - 1.0)
    pub overall: f64,
    /// 意图分类置信度
    pub category: f64,
    /// 参数提取置信度
    pub parameters: f64,
    /// 目标识别置信度
    pub target: f64,
}

impl ConfidenceScore {
    pub fn new(overall: f64, category: f64, parameters: f64, target: f64) -> Self {
        Self {
            overall: overall.clamp(0.0, 1.0),
            category: category.clamp(0.0, 1.0),
            parameters: parameters.clamp(0.0, 1.0),
            target: target.clamp(0.0, 1.0),
        }
    }

    /// 是否达到可执行阈值
    pub fn is_executable(&self, threshold: f64) -> bool {
        self.overall >= threshold
            && self.category >= threshold
            && self.parameters >= threshold * 0.8
    }
}

impl Default for ConfidenceScore {
    fn default() -> Self {
        Self {
            overall: 0.0,
            category: 0.0,
            parameters: 0.0,
            target: 0.0,
        }
    }
}

/// 解析元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseMetadata {
    /// 使用的 AI 模型
    pub model: String,
    /// 解析耗时 (毫秒)
    pub parse_duration_ms: u64,
    /// Token 使用量
    pub token_usage: TokenUsage,
    /// 解析时间戳
    pub parsed_at: chrono::DateTime<chrono::Utc>,
}

/// Token 使用统计
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TokenUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

/// 建议的修正或澄清
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Suggestion {
    /// 建议类型
    pub suggestion_type: SuggestionType,
    /// 建议消息
    pub message: String,
    /// 可能的值（用于参数补全）
    pub possible_values: Option<Vec<String>>,
}

/// 建议类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SuggestionType {
    /// 需要澄清
    Clarification,
    /// 参数建议
    ParameterSuggestion,
    /// 目标确认
    TargetConfirmation,
    /// 风险警告
    RiskWarning,
}
