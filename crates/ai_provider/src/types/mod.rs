//! AI Provider Types
//!
//! Core types and structures for AI provider integration.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Provider identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProviderId(pub String);

impl ProviderId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for ProviderId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for ProviderId {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

/// Model identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct ModelId(pub String);

impl ModelId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for ModelId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for ModelId {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

/// Provider capabilities
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProviderCapabilities {
    pub supports_chat: bool,
    pub supports_vision: bool,
    pub supports_tools: bool,
    pub supports_streaming: bool,
    pub supports_reasoning: bool,
    pub supports_embeddings: bool,
    pub max_context_length: Option<u32>,
}

/// Model information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub id: ModelId,
    pub name: String,
    pub description: Option<String>,
    pub provider: ProviderId,
    pub max_tokens: u32,
    pub capabilities: ProviderCapabilities,
    pub pricing: Option<ModelPricing>,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Model pricing information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelPricing {
    pub input_price_per_1k: f64,
    pub output_price_per_1k: f64,
    pub currency: String,
}

/// API key state
#[derive(Debug, Clone)]
pub enum ApiKeyState {
    NotConfigured,
    Configured(String),
    FromEnv(String),
}

impl ApiKeyState {
    pub fn is_configured(&self) -> bool {
        matches!(self, Self::Configured(_) | Self::FromEnv(_))
    }

    pub fn key(&self) -> Option<&str> {
        match self {
            Self::Configured(key) | Self::FromEnv(key) => Some(key),
            _ => None,
        }
    }
}

/// Message role for chat completions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MessageRole {
    System,
    User,
    Assistant,
    Tool,
    Model,
}

/// Chat message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: MessageRole,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
}

impl ChatMessage {
    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: MessageRole::System,
            content: content.into(),
            name: None,
            tool_calls: None,
        }
    }

    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: MessageRole::User,
            content: content.into(),
            name: None,
            tool_calls: None,
        }
    }

    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: MessageRole::Assistant,
            content: content.into(),
            name: None,
            tool_calls: None,
        }
    }
}

/// Tool call
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub id: String,
    pub function: FunctionCall,
}

/// Function call
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: String,
}

/// Tool definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

/// Chat completion request
#[derive(Debug, Clone, Default)]
pub struct ChatCompletionRequest {
    pub model: ModelId,
    pub messages: Vec<ChatMessage>,
    pub tools: Vec<Tool>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
    pub top_p: Option<f32>,
    pub stream: bool,
    pub stop: Vec<String>,
}

impl ChatCompletionRequest {
    pub fn new(model: ModelId, messages: Vec<ChatMessage>) -> Self {
        Self {
            model,
            messages,
            tools: Vec::new(),
            temperature: None,
            max_tokens: None,
            top_p: None,
            stream: false,
            stop: Vec::new(),
        }
    }

    pub fn with_temperature(mut self, temp: f32) -> Self {
        self.temperature = Some(temp);
        self
    }

    pub fn with_max_tokens(mut self, tokens: u32) -> Self {
        self.max_tokens = Some(tokens);
        self
    }

    pub fn with_streaming(mut self) -> Self {
        self.stream = true;
        self
    }

    pub fn with_tools(mut self, tools: Vec<Tool>) -> Self {
        self.tools = tools;
        self
    }
}

/// Chat completion response
#[derive(Debug, Clone)]
pub struct ChatCompletionResponse {
    pub id: String,
    pub model: ModelId,
    pub content: String,
    pub usage: TokenUsage,
    pub finish_reason: FinishReason,
    pub tool_calls: Vec<ToolCall>,
}

/// Streaming chunk for chat completions
#[derive(Debug, Clone)]
pub struct ChatCompletionChunk {
    pub id: String,
    pub model: ModelId,
    pub content_delta: String,
    pub finish_reason: Option<FinishReason>,
}

/// Token usage information
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TokenUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

/// Finish reason for chat completion
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FinishReason {
    Stop,
    Length,
    ToolCalls,
    ContentFilter,
    Other,
}

/// Embedding request
#[derive(Debug, Clone)]
pub struct EmbeddingRequest {
    pub model: ModelId,
    pub input: Vec<String>,
}

/// Embedding response
#[derive(Debug, Clone)]
pub struct EmbeddingResponse {
    pub model: ModelId,
    pub embeddings: Vec<Vec<f32>>,
    pub usage: TokenUsage,
}

/// Connection test result
#[derive(Debug, Clone)]
pub struct ConnectionTestResult {
    pub success: bool,
    pub latency_ms: u64,
    pub message: String,
    pub models_available: Option<usize>,
    pub error: Option<String>,
}

/// Provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub provider_id: ProviderId,
    pub name: String,
    pub endpoint: String,
    pub api_key: Option<String>,
    pub organization_id: Option<String>,
    pub project_id: Option<String>,
    pub region: Option<String>,
    pub timeout_seconds: u64,
    pub max_retries: u32,
    pub custom_headers: HashMap<String, String>,
    pub enabled: bool,
}

impl Default for ProviderConfig {
    fn default() -> Self {
        Self {
            provider_id: ProviderId::new("unknown"),
            name: String::new(),
            endpoint: String::new(),
            api_key: None,
            organization_id: None,
            project_id: None,
            region: None,
            timeout_seconds: 60,
            max_retries: 3,
            custom_headers: HashMap::new(),
            enabled: false,
        }
    }
}

/// API error types
#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Authentication failed: {0}")]
    Authentication(String),

    #[error("Rate limit exceeded: {0}")]
    RateLimit(String),

    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    #[error("Model not found: {0}")]
    ModelNotFound(String),

    #[error("Server error: {0}")]
    ServerError(String),

    #[error("Network error: {0}")]
    Network(String),

    #[error("Timeout")]
    Timeout,

    #[error("Other: {0}")]
    Other(String),
}

/// Provider metadata for UI display
#[derive(Debug, Clone)]
pub struct ProviderMetadata {
    pub id: ProviderId,
    pub name: String,
    pub description: String,
    pub icon: Option<String>,
    pub is_local: bool,
    pub requires_api_key: bool,
    pub website_url: String,
    pub documentation_url: String,
}
