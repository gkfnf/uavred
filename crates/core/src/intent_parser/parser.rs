//! 意图解析器实现

use super::{
    error::{IntentParseError, IntentResult},
    intent::{Constraint, ContextType},
    security::*,
    ConfidenceScore, Intent, IntentCategory, ParseMetadata,
    Suggestion, SuggestionType, TokenUsage,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// AI 提供者接口（抽象）
#[async_trait::async_trait]
pub trait AiProvider: Send + Sync {
    /// 发送聊天完成请求
    async fn chat_completion(&self, request: ChatCompletionRequest) -> Result<String, String>;
    
    /// 获取默认模型名称
    fn default_model(&self) -> String;
}

/// 聊天完成请求
#[derive(Debug, Clone, Serialize)]
pub struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub temperature: f32,
    pub max_tokens: u32,
}

impl ChatCompletionRequest {
    pub fn new(messages: Vec<ChatMessage>) -> Self {
        Self {
            model: String::new(),
            messages,
            temperature: 0.3,
            max_tokens: 2048,
        }
    }

    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.model = model.into();
        self
    }
}

/// 聊天消息
#[derive(Debug, Clone, Serialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

impl ChatMessage {
    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: "system".to_string(),
            content: content.into(),
        }
    }

    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: "user".to_string(),
            content: content.into(),
        }
    }
}

/// 解析器配置
#[derive(Debug, Clone)]
pub struct ParserConfig {
    /// 置信度阈值
    pub confidence_threshold: f64,
    /// 默认模型
    pub default_model: String,
    /// 超时时间（毫秒）
    pub timeout_ms: u64,
    /// 最大重试次数
    pub max_retries: u32,
}

impl Default for ParserConfig {
    fn default() -> Self {
        Self {
            confidence_threshold: 0.7,
            default_model: "kimi-k2.5".to_string(),
            timeout_ms: 30000,
            max_retries: 3,
        }
    }
}

/// 意图解析器
pub struct IntentParser {
    ai_provider: Arc<dyn AiProvider>,
    config: ParserConfig,
}

impl IntentParser {
    /// 创建新的解析器
    pub fn new(ai_provider: Arc<dyn AiProvider>) -> Self {
        Self {
            ai_provider,
            config: ParserConfig::default(),
        }
    }

    /// 创建带配置的解析器
    pub fn with_config(ai_provider: Arc<dyn AiProvider>, config: ParserConfig) -> Self {
        Self {
            ai_provider,
            config,
        }
    }

    /// 解析安全测试意图
    pub async fn parse_security_test(&self, intent: Intent) -> IntentResult<ParseResult> {
        let start_time = std::time::Instant::now();

        // 构建系统提示词
        let system_prompt = self.build_system_prompt();

        // 构建用户提示词
        let user_prompt = self.build_user_prompt(&intent);

        // 发送请求到 AI
        let request = ChatCompletionRequest::new(vec![
            ChatMessage::system(system_prompt),
            ChatMessage::user(user_prompt),
        ])
        .with_model(&self.config.default_model);

        let response = self
            .ai_provider
            .chat_completion(request)
            .await
            .map_err(IntentParseError::ai_provider)?;

        // 解析响应
        let parsed: AiParsedResponse = serde_json::from_str(&response)
            .map_err(|e| IntentParseError::parse_failed(format!("Failed to parse AI response: {}", e)))?;

        let duration = start_time.elapsed();

        // 构建结果
        let result = self.build_parse_result(intent, parsed, duration.as_millis() as u64)?;

        // 检查置信度
        if !result.confidence.is_executable(self.config.confidence_threshold) {
            return Err(IntentParseError::LowConfidence {
                score: result.confidence.overall,
                threshold: self.config.confidence_threshold,
            });
        }

        Ok(result)
    }

    /// 快速分类意图类型
    pub async fn classify_intent(&self, text: &str) -> IntentResult<IntentCategory> {
        let prompt = format!(
            r#"请将以下用户输入分类为以下类别之一：security_test, information_query, configuration, unknown

用户输入: "{}"

只返回类别名称，不要其他解释。"#,
            text
        );

        let request = ChatCompletionRequest::new(vec![
            ChatMessage::system("你是一个意图分类器。将用户输入分类为预定义的类别。".to_string()),
            ChatMessage::user(prompt),
        ])
        .with_model(&self.config.default_model);

        let response = self
            .ai_provider
            .chat_completion(request)
            .await
            .map_err(IntentParseError::ai_provider)?;

        let category = match response.trim().to_lowercase().as_str() {
            "security_test" | "security" => IntentCategory::SecurityTest,
            "information_query" | "query" => IntentCategory::InformationQuery,
            "configuration" | "config" => IntentCategory::Configuration,
            _ => IntentCategory::Unknown,
        };

        Ok(category)
    }

    /// 构建系统提示词
    fn build_system_prompt(&self) -> String {
        r#"你是一个专业的网络安全测试意图解析器。你的任务是将用户的自然语言描述转换为结构化的安全测试配置。

你需要提取以下信息并以JSON格式返回：
{
    "test_type": "网络扫描类型，如 network_scan, port_scan, vulnerability_scan, protocol_analysis, firmware_analysis, exploit, web_app_test",
    "confidence": {
        "overall": 0.0-1.0,
        "category": 0.0-1.0,
        "parameters": 0.0-1.0,
        "target": 0.0-1.0
    },
    "targets": [
        {
            "address": "目标地址",
            "target_type": "ip|cidr|domain|hostname|url|file",
            "ports": [端口列表，可选]
        }
    ],
    "parameters": {
        "protocol": "协议类型",
        "port_range": "端口范围，如 '1-1000' 或 '80,443,8080'",
        "intensity": "light|normal|aggressive",
        "deep_scan": true/false,
        "priority": "low|medium|high|critical",
        "threads": 线程数,
        "timeout_seconds": 超时秒数,
        "其他参数..."
    },
    "scan_config": {
        "intensity": "light|normal|aggressive",
        "deep_scan": true/false,
        "threads": 线程数,
        "timeout_seconds": 超时秒数
    },
    "suggestions": [
        {
            "type": "clarification|parameter_suggestion|target_confirmation|risk_warning",
            "message": "建议消息",
            "possible_values": ["可选值1", "可选值2"]
        }
    ],
    "constraints": [
        {
            "type": "rule|time_limit|resource_limit|network_restriction",
            "description": "约束描述"
        }
    ],
    "context_analysis": {
        "risk_level": "low|medium|high|critical",
        "compliance_requirements": ["合规要求1", "合规要求2"],
        "special_considerations": ["特殊考虑1", "特殊考虑2"]
    }
}

注意事项：
1. 准确识别测试类型（网络扫描、端口扫描、漏洞扫描、协议分析、固件分析、漏洞利用等）
2. 提取所有目标地址（IP、CIDR、域名、URL等）
3. 识别端口范围、协议类型等参数
4. 评估请求的风险等级
5. 如果信息不完整，提供澄清建议
6. 置信度评分要客观准确
7. 考虑中文描述的安全测试意图

安全测试类型说明：
- network_scan: 网络扫描，发现存活主机
- port_scan: 端口扫描，检测开放端口
- vulnerability_scan: 漏洞扫描，检查已知漏洞
- protocol_analysis: 协议分析，分析通信协议
- firmware_analysis: 固件分析，分析固件文件
- exploit: 漏洞利用，尝试利用漏洞
- web_app_test: Web应用测试
- api_test: API测试
- configuration_audit: 配置审计"#.to_string()
    }

    /// 构建用户提示词
    fn build_user_prompt(&self, intent: &Intent) -> String {
        let mut prompt = format!("用户意图: {}\n\n", intent.goal);

        if !intent.context.is_empty() {
            prompt.push_str("上下文信息:\n");
            for ctx in &intent.context {
                let type_str = match ctx.context_type {
                    ContextType::Technical => "[技术]",
                    ContextType::Security => "[安全]",
                    ContextType::Background => "[背景]",
                    ContextType::Business => "[业务]",
                    ContextType::Other => "[其他]",
                };
                prompt.push_str(&format!("{} {}\n", type_str, ctx.content));
            }
            prompt.push('\n');
        }

        if !intent.constraints.is_empty() {
            prompt.push_str("约束条件:\n");
            for constraint in &intent.constraints {
                match constraint {
                    Constraint::Rule(rule) => {
                        prompt.push_str(&format!("- 规则: {}\n", rule));
                    }
                    Constraint::TimeLimit { seconds } => {
                        prompt.push_str(&format!("- 时间限制: {}秒\n", seconds));
                    }
                    Constraint::ResourceLimit { cpu_percent, memory_mb } => {
                        prompt.push_str(&format!(
                            "- 资源限制: CPU {}%, 内存 {}MB\n",
                            cpu_percent, memory_mb
                        ));
                    }
                    Constraint::NetworkRestriction { allowed_hosts } => {
                        prompt.push_str(&format!("- 网络限制: 仅允许 {}\n", allowed_hosts.join(", ")));
                    }
                    Constraint::Custom { name, value } => {
                        prompt.push_str(&format!("- {}: {}\n", name, value));
                    }
                }
            }
        }

        prompt.push_str("\n请解析以上安全测试意图，返回JSON格式的结构化数据。");

        prompt
    }

    /// 构建解析结果
    fn build_parse_result(
        &self,
        intent: Intent,
        parsed: AiParsedResponse,
        duration_ms: u64,
    ) -> IntentResult<ParseResult> {
        let security_intent = SecurityTestIntent {
            base: intent.clone(),
            test_type: SecurityTestType::from(parsed.test_type.as_str()),
            params: SecurityTestParams {
                params: parsed.parameters.clone().into_iter().collect(),
            },
            targets: parsed
                .targets
                .into_iter()
                .map(|t| Target {
                    address: t.address,
                    target_type: match t.target_type.as_str() {
                        "ip" => TargetType::Ip,
                        "cidr" => TargetType::Cidr,
                        "domain" => TargetType::Domain,
                        "hostname" => TargetType::Hostname,
                        "url" => TargetType::Url,
                        "file" => TargetType::File,
                        "range" => TargetType::Range,
                        _ => TargetType::Unknown,
                    },
                    ports: t.ports,
                    metadata: t.metadata.unwrap_or_default(),
                })
                .collect(),
            scan_config: ScanConfig {
                intensity: match parsed.scan_config.intensity.as_str() {
                    "light" => ScanIntensity::Light,
                    "aggressive" => ScanIntensity::Aggressive,
                    "custom" => ScanIntensity::Custom,
                    _ => ScanIntensity::Normal,
                },
                deep_scan: parsed.scan_config.deep_scan,
                threads: parsed.scan_config.threads.unwrap_or(10),
                timeout_seconds: parsed.scan_config.timeout_seconds.unwrap_or(300),
                options: parsed.scan_config.options.unwrap_or_default(),
            },
        };

        let suggestions = parsed
            .suggestions
            .into_iter()
            .map(|s| Suggestion {
                suggestion_type: match s.suggestion_type.as_str() {
                    "clarification" => SuggestionType::Clarification,
                    "parameter_suggestion" => SuggestionType::ParameterSuggestion,
                    "target_confirmation" => SuggestionType::TargetConfirmation,
                    "risk_warning" => SuggestionType::RiskWarning,
                    _ => SuggestionType::Clarification,
                },
                message: s.message,
                possible_values: s.possible_values,
            })
            .collect();

        let confidence = ConfidenceScore::new(
            parsed.confidence.overall,
            parsed.confidence.category,
            parsed.confidence.parameters,
            parsed.confidence.target,
        );

        let metadata = ParseMetadata {
            model: self.config.default_model.clone(),
            parse_duration_ms: duration_ms,
            token_usage: TokenUsage::default(), // 可由调用者填充
            parsed_at: chrono::Utc::now(),
        };

        Ok(ParseResult {
            category: IntentCategory::SecurityTest,
            security_intent,
            confidence,
            metadata,
            suggestions,
            raw_response: None,
        })
    }
}

/// AI 解析响应结构
#[derive(Debug, Deserialize)]
struct AiParsedResponse {
    test_type: String,
    confidence: AiConfidence,
    targets: Vec<AiTarget>,
    #[serde(default)]
    parameters: serde_json::Map<String, serde_json::Value>,
    scan_config: AiScanConfig,
    #[serde(default)]
    suggestions: Vec<AiSuggestion>,
    #[serde(default)]
    constraints: Vec<AiConstraint>,
    #[serde(default)]
    context_analysis: Option<AiContextAnalysis>,
}

#[derive(Debug, Deserialize)]
struct AiConfidence {
    overall: f64,
    category: f64,
    parameters: f64,
    target: f64,
}

#[derive(Debug, Deserialize)]
struct AiTarget {
    address: String,
    target_type: String,
    #[serde(default)]
    ports: Option<Vec<u16>>,
    #[serde(default)]
    metadata: Option<std::collections::HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Deserialize)]
struct AiScanConfig {
    intensity: String,
    #[serde(default)]
    deep_scan: bool,
    #[serde(default)]
    threads: Option<u32>,
    #[serde(default)]
    timeout_seconds: Option<u64>,
    #[serde(default)]
    options: Option<std::collections::HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Deserialize)]
struct AiSuggestion {
    #[serde(rename = "type")]
    suggestion_type: String,
    message: String,
    #[serde(default)]
    possible_values: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct AiConstraint {
    #[serde(rename = "type")]
    constraint_type: String,
    description: String,
}

#[derive(Debug, Deserialize)]
struct AiContextAnalysis {
    risk_level: String,
    #[serde(default)]
    compliance_requirements: Vec<String>,
    #[serde(default)]
    special_considerations: Vec<String>,
}

/// 解析结果
#[derive(Debug, Clone)]
pub struct ParseResult {
    /// 意图类别
    pub category: IntentCategory,
    /// 安全测试意图
    pub security_intent: SecurityTestIntent,
    /// 置信度评分
    pub confidence: ConfidenceScore,
    /// 解析元数据
    pub metadata: ParseMetadata,
    /// 建议
    pub suggestions: Vec<Suggestion>,
    /// 原始 AI 响应
    pub raw_response: Option<String>,
}
