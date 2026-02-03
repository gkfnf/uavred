# AI Provider Integration Crate

统一的 AI 供应商集成模块，为 UAVRed 项目提供标准化的 AI 服务接口。

## 支持的供应商

### 云端供应商
- **Kimi (Moonshot)** - 中国大模型供应商，支持长上下文和推理
- **DeepSeek** - 具有强大推理能力的 AI 模型
- **OpenAI** - GPT-4 系列模型
- **Claude (Anthropic)** - 大上下文窗口模型
- **Gemini (Google)** - Google 多模态模型
- **Codex** - OpenAI 代码专用模型
- **Z.ai (ChatGLM)** - 智谱 AI 中文模型

### 本地供应商
- **Ollama** - 本地运行开源模型
- **LMStudio** - 本地 AI 模型管理器

## 架构设计

```
ai_provider/
├── src/
│   ├── lib.rs              # 主模块入口
│   ├── types/              # 核心类型定义
│   │   └── mod.rs          # ProviderId, ModelId, ChatMessage 等
│   ├── provider.rs         # AiProvider trait 和 ProviderRegistry
│   ├── http_client.rs      # HTTP 客户端和重试逻辑
│   └── providers/          # 供应商实现
│       ├── openai_compatible.rs  # OpenAI 兼容 API 基础
│       ├── kimi.rs
│       ├── deepseek.rs
│       ├── openai.rs
│       ├── claude.rs
│       ├── gemini.rs
│       ├── ollama.rs
│       ├── lmstudio.rs
│       ├── codex.rs
│       └── zai.rs
└── examples/
    ├── basic_usage.rs      # 基础使用示例
    └── advanced_usage.rs   # 高级功能示例
```

## 核心特性

### 1. 统一接口 (AiProvider Trait)

```rust
#[async_trait]
pub trait AiProvider: Send + Sync {
    fn provider_id(&self) -> ProviderId;
    fn name(&self) -> &str;
    fn is_authenticated(&self) -> bool;
    fn capabilities(&self) -> ProviderCapabilities;
    fn available_models(&self) -> Vec<ModelInfo>;
    
    async fn fetch_models(&self) -> Result<Vec<ModelInfo>, ApiError>;
    async fn test_connection(&self) -> Result<ConnectionTestResult, ApiError>;
    async fn get_latency(&self) -> Result<u64, ApiError>;
    
    async fn chat_completion(&self, request: ChatCompletionRequest) 
        -> Result<ChatCompletionResponse, ApiError>;
    async fn chat_completion_stream(&self, request: ChatCompletionRequest)
        -> Result<BoxStream<'static, Result<ChatCompletionChunk, ApiError>>, ApiError>;
    async fn create_embeddings(&self, request: EmbeddingRequest)
        -> Result<EmbeddingResponse, ApiError>;
}
```

### 2. 供应商注册中心 (ProviderRegistry)

```rust
// 创建包含所有默认供应商的注册中心
let registry = ProviderRegistry::with_defaults();

// 获取特定供应商
let kimi = registry.get(&"kimi".into()).expect("Kimi provider not found");

// 列出所有已认证的供应商
let authenticated = registry.list_authenticated();

// 测试所有连接
let results = registry.test_all_connections().await;

// 获取所有可用模型
let all_models = registry.get_all_models().await;
```

### 3. 连接测试与延迟测量

```rust
// 测试单个供应商连接
let result = provider.test_connection().await?;
println!("Success: {}", result.success);
println!("Latency: {} ms", result.latency_ms);
println!("Models: {:?}", result.models_available);

// 测量延迟
let latency = provider.get_latency().await?;
```

### 4. 聊天补全

```rust
use ai_provider::{ChatCompletionRequest, ChatMessage};

let request = ChatCompletionRequest::new(
    "kimi-k2.5".into(),
    vec![
        ChatMessage::system("You are a helpful assistant."),
        ChatMessage::user("Hello!"),
    ],
).with_temperature(0.7)
 .with_max_tokens(2048);

let response = provider.chat_completion(request).await?;
println!("Response: {}", response.content);
println!("Tokens used: {}", response.usage.total_tokens);
```

## 使用示例

### 基础用法

```rust
use ai_provider::{ProviderRegistry, ChatCompletionRequest, ChatMessage};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 创建注册中心
    let registry = ProviderRegistry::with_defaults();
    
    // 获取 Kimi 供应商
    let kimi = registry.get(&"kimi".into()).unwrap();
    
    // 测试连接
    let result = kimi.test_connection().await?;
    println!("Connection test: {:?}", result);
    
    // 获取可用模型
    let models = kimi.fetch_models().await?;
    for model in models {
        println!("- {} ({} tokens)", model.name, model.max_tokens);
    }
    
    Ok(())
}
```

### 高级用法 - 延迟比较

```rust
use ai_provider::ProviderRegistry;

async fn find_fastest_provider() {
    let registry = ProviderRegistry::with_defaults();
    
    let mut latencies = Vec::new();
    for provider in registry.list().iter() {
        if let Ok(latency) = provider.get_latency().await {
            latencies.push((provider.name().to_string(), latency));
        }
    }
    
    latencies.sort_by_key(|(_, l)| *l);
    
    for (name, latency) in latencies {
        println!("{}: {} ms", name, latency);
    }
}
```

### 自定义配置

```rust
use ai_provider::{ProviderBuilder, ProviderConfig};

let config = ProviderBuilder::new("custom".into())
    .with_endpoint("https://api.example.com")
    .with_api_key("sk-xxx")
    .with_timeout(60)
    .with_region("us-east-1")
    .enabled(true)
    .build_config();
```

## 环境变量

每个供应商支持从环境变量读取 API 密钥：

| 供应商 | 环境变量 |
|--------|----------|
| Kimi | `MOONSHOT_API_KEY` |
| DeepSeek | `DEEPSEEK_API_KEY` |
| OpenAI | `OPENAI_API_KEY` |
| Claude | `ANTHROPIC_API_KEY` |
| Gemini | `GOOGLE_API_KEY` |
| Codex | `OPENAI_API_KEY` |
| Z.ai | `ZAI_API_KEY` |

## 与 Settings UI 集成

`ai_provider` crate 可与 `settings_ui` crate 联动：

```rust
// 在 settings_ui 中使用
use ai_provider::ProviderRegistry;

pub struct AiSettingsPanel {
    registry: ProviderRegistry,
    selected_provider: ProviderId,
}

impl AiSettingsPanel {
    pub fn test_selected_connection(&self, cx: &App) {
        if let Some(provider) = self.registry.get(&self.selected_provider) {
            cx.spawn(async move {
                let result = provider.test_connection().await;
                // 更新 UI 显示结果
            }).detach();
        }
    }
}
```

## 与 Kanban Agent 集成

```rust
use ai_provider::{ProviderRegistry, ChatCompletionRequest, ChatMessage};

pub struct TaskAgent {
    registry: ProviderRegistry,
}

impl TaskAgent {
    pub async fn execute_task(&self, task_description: &str) -> anyhow::Result<String> {
        // 选择默认供应商
        let provider = self.registry.get_default()
            .ok_or_else(|| anyhow::anyhow!("No default provider"))?;
        
        let request = ChatCompletionRequest::new(
            provider.available_models()[0].id.clone(),
            vec![
                ChatMessage::system("You are a task execution agent."),
                ChatMessage::user(task_description),
            ],
        );
        
        let response = provider.chat_completion(request).await?;
        Ok(response.content)
    }
}
```

## API 错误处理

```rust
use ai_provider::types::ApiError;

match result {
    Err(ApiError::Authentication(msg)) => eprintln!("Auth failed: {}", msg),
    Err(ApiError::RateLimit(msg)) => eprintln!("Rate limited: {}", msg),
    Err(ApiError::Network(msg)) => eprintln!("Network error: {}", msg),
    Err(ApiError::Timeout) => eprintln!("Request timeout"),
    _ => {}
}
```

## 开发计划

### 已实现
- [x] 统一的 AiProvider trait
- [x] ProviderRegistry 注册中心
- [x] 9 个供应商实现
- [x] 连接测试和延迟测量
- [x] 聊天补全 API
- [x] HTTP 客户端和重试逻辑
- [x] 模型获取和缓存

### 待实现
- [ ] 流式响应 (SSE)
- [ ] Embedding API
- [ ] 工具/函数调用
- [ ] 图像生成 (DALL-E, etc.)
- [ ] 语音转文字
- [ ] 配置持久化
- [ ] 使用量统计

## 测试

```bash
# 运行单元测试
cargo test -p ai_provider

# 运行示例
cargo run --example basic_usage -p ai_provider
cargo run --example advanced_usage -p ai_provider
```

## License

MIT
