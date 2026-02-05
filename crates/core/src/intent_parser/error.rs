//! 意图解析错误类型

use thiserror::Error;

/// 意图解析错误
#[derive(Debug, Error, Clone)]
pub enum IntentParseError {
    /// AI 提供者错误
    #[error("AI provider error: {0}")]
    AiProvider(String),

    /// 解析失败
    #[error("Failed to parse intent: {0}")]
    ParseFailed(String),

    /// 无效的输入
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// 不支持的意图类型
    #[error("Unsupported intent type: {0}")]
    UnsupportedIntentType(String),

    /// 缺少必要参数
    #[error("Missing required parameter: {0}")]
    MissingParameter(String),

    /// 参数验证失败
    #[error("Parameter validation failed: {0}")]
    ParameterValidation(String),

    /// 置信度不足
    #[error("Low confidence score: {score:.2} < {threshold:.2}")]
    LowConfidence { score: f64, threshold: f64 },

    /// 超时
    #[error("Parse timeout after {0}ms")]
    Timeout(u64),

    /// 序列化错误
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// 其他错误
    #[error("Intent parse error: {0}")]
    Other(String),
}

impl IntentParseError {
    /// 创建 AI 提供者错误
    pub fn ai_provider(msg: impl Into<String>) -> Self {
        Self::AiProvider(msg.into())
    }

    /// 创建解析失败错误
    pub fn parse_failed(msg: impl Into<String>) -> Self {
        Self::ParseFailed(msg.into())
    }

    /// 创建无效输入错误
    pub fn invalid_input(msg: impl Into<String>) -> Self {
        Self::InvalidInput(msg.into())
    }

    /// 创建缺少参数错误
    pub fn missing_parameter(param: impl Into<String>) -> Self {
        Self::MissingParameter(param.into())
    }

    /// 是否可重试
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            IntentParseError::AiProvider(_)
                | IntentParseError::Timeout(_)
                | IntentParseError::Other(_)
        )
    }
}

/// 意图执行错误
#[derive(Debug, Error, Clone)]
pub enum IntentExecutionError {
    /// 执行失败
    #[error("Execution failed: {0}")]
    ExecutionFailed(String),

    /// Sandbox 错误
    #[error("Sandbox error: {0}")]
    Sandbox(String),

    /// Agent 错误
    #[error("Agent error: {0}")]
    Agent(String),

    /// 任务创建失败
    #[error("Failed to create task: {0}")]
    TaskCreation(String),

    /// 超时
    #[error("Execution timeout after {0}s")]
    Timeout(u64),

    /// 被用户取消
    #[error("Execution cancelled by user")]
    Cancelled,

    /// 依赖缺失
    #[error("Missing dependency: {0}")]
    MissingDependency(String),

    /// 其他错误
    #[error("Intent execution error: {0}")]
    Other(String),
}

impl IntentExecutionError {
    /// 创建执行失败错误
    pub fn execution_failed(msg: impl Into<String>) -> Self {
        Self::ExecutionFailed(msg.into())
    }

    /// 创建 sandbox 错误
    pub fn sandbox(msg: impl Into<String>) -> Self {
        Self::Sandbox(msg.into())
    }

    /// 创建 agent 错误
    pub fn agent(msg: impl Into<String>) -> Self {
        Self::Agent(msg.into())
    }

    /// 是否可重试
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            IntentExecutionError::ExecutionFailed(_)
                | IntentExecutionError::Sandbox(_)
                | IntentExecutionError::Agent(_)
                | IntentExecutionError::Timeout(_)
                | IntentExecutionError::Other(_)
        )
    }
}

/// 结果类型别名
pub type IntentResult<T> = Result<T, IntentParseError>;
pub type ExecutionResult<T> = Result<T, IntentExecutionError>;
