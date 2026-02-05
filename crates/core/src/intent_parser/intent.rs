//! Intent 结构体定义 - 用户意图的完整表示

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 用户意图
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intent {
    /// 意图目标（用户想要做什么）
    pub goal: String,
    /// 上下文信息
    pub context: Vec<IntentContext>,
    /// 输入数据
    pub input: IntentInput,
    /// 预期输出
    pub output: Option<IntentOutput>,
    /// 执行策略
    pub strategy: ExecutionStrategy,
    /// 约束条件
    pub constraints: Vec<Constraint>,
    /// 元数据
    pub metadata: HashMap<String, serde_json::Value>,
}

impl Intent {
    /// 创建新的意图构建器
    pub fn new() -> IntentBuilder {
        IntentBuilder::default()
    }

    /// 获取原始输入文本
    pub fn raw_text(&self) -> &str {
        &self.goal
    }

    /// 添加上下文
    pub fn add_context(&mut self, context: IntentContext) {
        self.context.push(context);
    }

    /// 添加约束
    pub fn add_constraint(&mut self, constraint: Constraint) {
        self.constraints.push(constraint);
    }
}

impl Default for Intent {
    fn default() -> Self {
        Self {
            goal: String::new(),
            context: Vec::new(),
            input: IntentInput::default(),
            output: None,
            strategy: ExecutionStrategy::default(),
            constraints: Vec::new(),
            metadata: HashMap::new(),
        }
    }
}

/// 意图构建器
#[derive(Debug, Default)]
pub struct IntentBuilder {
    goal: Option<String>,
    context: Vec<IntentContext>,
    input: IntentInput,
    output: Option<IntentOutput>,
    strategy: ExecutionStrategy,
    constraints: Vec<Constraint>,
    metadata: HashMap<String, serde_json::Value>,
}

impl IntentBuilder {
    /// 设置意图目标
    pub fn goal(mut self, goal: impl Into<String>) -> Self {
        self.goal = Some(goal.into());
        self
    }

    /// 添加上下文
    pub fn context(mut self, context: impl Into<String>) -> Self {
        self.context.push(IntentContext {
            content: context.into(),
            context_type: ContextType::Background,
        });
        self
    }

    /// 添加技术上下文
    pub fn technical_context(mut self, context: impl Into<String>) -> Self {
        self.context.push(IntentContext {
            content: context.into(),
            context_type: ContextType::Technical,
        });
        self
    }

    /// 添加安全上下文
    pub fn security_context(mut self, context: impl Into<String>) -> Self {
        self.context.push(IntentContext {
            content: context.into(),
            context_type: ContextType::Security,
        });
        self
    }

    /// 设置输入
    pub fn input(mut self, input: IntentInput) -> Self {
        self.input = input;
        self
    }

    /// 添加输入参数
    pub fn with_param(mut self, key: impl Into<String>, value: impl Into<serde_json::Value>) -> Self {
        self.input.parameters.insert(key.into(), value.into());
        self
    }

    /// 设置预期输出
    pub fn output(mut self, output: IntentOutput) -> Self {
        self.output = Some(output);
        self
    }

    /// 设置执行策略
    pub fn strategy(mut self, strategy: ExecutionStrategy) -> Self {
        self.strategy = strategy;
        self
    }

    /// 设置执行方式
    pub fn how(mut self, approach: impl Into<String>) -> Self {
        self.strategy.approach = approach.into();
        self
    }

    /// 添加约束
    pub fn constraint(mut self, constraint: Constraint) -> Self {
        self.constraints.push(constraint);
        self
    }

    /// 添加规则/约束
    pub fn rule(mut self, rule: impl Into<String>) -> Self {
        self.constraints.push(Constraint::Rule(rule.into()));
        self
    }

    /// 设置超时
    pub fn timeout_seconds(mut self, seconds: u64) -> Self {
        self.strategy.timeout_seconds = seconds;
        self
    }

    /// 添加元数据
    pub fn metadata(mut self, key: impl Into<String>, value: impl Into<serde_json::Value>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// 构建 Intent
    pub fn build(self) -> Result<Intent, String> {
        let goal = self.goal.ok_or("Goal is required")?;

        Ok(Intent {
            goal,
            context: self.context,
            input: self.input,
            output: self.output,
            strategy: self.strategy,
            constraints: self.constraints,
            metadata: self.metadata,
        })
    }
}

/// 上下文信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentContext {
    /// 上下文内容
    pub content: String,
    /// 上下文类型
    pub context_type: ContextType,
}

/// 上下文类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContextType {
    /// 背景信息
    Background,
    /// 技术细节
    Technical,
    /// 安全要求
    Security,
    /// 业务约束
    Business,
    /// 其他
    Other,
}

/// 意图输入
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IntentInput {
    /// 原始输入文本
    pub raw_text: String,
    /// 输入参数
    pub parameters: HashMap<String, serde_json::Value>,
    /// 附件/文件
    pub attachments: Vec<Attachment>,
}

/// 附件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment {
    /// 文件名
    pub filename: String,
    /// 内容类型
    pub content_type: String,
    /// 内容（Base64 编码或 URL）
    pub content: String,
    /// 是否是 URL
    pub is_url: bool,
}

/// 预期输出
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentOutput {
    /// 输出格式
    pub format: OutputFormat,
    /// 输出schema（用于结构化输出）
    pub schema: Option<serde_json::Value>,
    /// 预期字段
    pub expected_fields: Vec<String>,
}

/// 输出格式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OutputFormat {
    /// 结构化数据（JSON）
    Structured,
    /// 文本
    Text,
    /// 报告
    Report,
    /// 列表
    List,
    /// 表格
    Table,
}

impl Default for OutputFormat {
    fn default() -> Self {
        OutputFormat::Structured
    }
}

/// 执行策略
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionStrategy {
    /// 执行方法/方法
    pub approach: String,
    /// 最大迭代次数
    pub max_iterations: u32,
    /// 超时时间（秒）
    pub timeout_seconds: u64,
    /// 是否使用缓存
    pub use_cache: bool,
    /// 是否记录执行过程
    pub record_execution: bool,
}

impl Default for ExecutionStrategy {
    fn default() -> Self {
        Self {
            approach: "auto".to_string(),
            max_iterations: 10,
            timeout_seconds: 300,
            use_cache: false,
            record_execution: true,
        }
    }
}

/// 约束条件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Constraint {
    /// 规则（文本描述）
    Rule(String),
    /// 时间限制
    TimeLimit { seconds: u64 },
    /// 资源限制
    ResourceLimit { cpu_percent: u32, memory_mb: u64 },
    /// 网络限制
    NetworkRestriction { allowed_hosts: Vec<String> },
    /// 自定义约束
    Custom { name: String, value: serde_json::Value },
}

/// 从字符串创建 Intent
impl From<&str> for Intent {
    fn from(text: &str) -> Self {
        Self {
            goal: text.to_string(),
            ..Default::default()
        }
    }
}

impl From<String> for Intent {
    fn from(text: String) -> Self {
        Self {
            goal: text,
            ..Default::default()
        }
    }
}
