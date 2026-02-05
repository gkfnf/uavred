# UAVRed 意图解析引擎使用文档

## 概述

UAVRed 意图解析引擎是一个基于 AI 的自然语言安全测试意图解析系统。它允许用户通过自然语言描述他们想要执行的安全测试，然后自动解析为结构化的任务定义并执行。

## 架构

意图解析引擎包含以下核心组件：

### 1. Core Intent Parser (`crates/core/src/intent_parser/`)

- **Intent**: 用户意图的完整表示
- **IntentParser**: 使用 AI 解析自然语言为结构化数据
- **SecurityTestIntent**: 安全测试特定的意图类型
- **IntentExecutor**: 将解析后的意图转换为可执行任务

### 2. Execution Service (`crates/core/src/execution/`)

- **ExecutionService**: 主服务，协调整个执行流程
- **SandboxManager**: 管理安全测试的 sandbox 环境
- **AgentScheduler**: 分配 agent 执行安全测试任务

### 3. Kanban UI Integration (`crates/kanban_ui/src/intent/`)

- **IntentParserPanel**: Kanban UI 中的意图解析面板
- **ParsedIntentPreview**: 解析结果预览卡片

## 使用示例

### 基础用法

```rust
use core::intent_parser::{Intent, IntentParser, SecurityTestType};

// 1. 创建意图
let intent = Intent::new()
    .goal("扫描 192.168.1.0/24 网段的所有开放端口")
    .context("这是一个内部测试网络")
    .build()
    .expect("Failed to build intent");

// 2. 创建解析器（需要 AI Provider）
let parser = IntentParser::new(ai_provider);

// 3. 解析意图
let result = parser.parse_security_test(intent).await?;

// 4. 获取解析结果
println!("测试类型: {}", result.security_intent.test_type.display_name());
println!("置信度: {:.0}%", result.confidence.overall * 100.0);
```

### 使用 IntentExecutor 创建任务

```rust
use core::intent_parser::IntentExecutor;

let executor = IntentExecutor::new();

// 从解析结果创建任务
let task = executor.create_task(&parsed_result)?;

// 生成执行计划
let plan = executor.generate_execution_plan(&parsed_result)?;
```

### 使用 ExecutionService 完整执行

```rust
use core::execution::{ExecutionService, ExecutionConfig, SandboxManager, AgentScheduler};

// 配置执行服务
let config = ExecutionConfig {
    auto_create_sandbox: true,
    auto_assign_agent: true,
    ..Default::default()
};

let service = ExecutionService::with_config(config)
    .with_sandbox_manager(SandboxManager::new())
    .with_agent_scheduler(AgentScheduler::new());

// 执行意图
let execution = service.execute_intent(&parsed, user_id).await?;
println!("执行 ID: {}", execution.execution_id);
```

### Kanban UI 集成

```rust
use kanban_ui::{IntentParserPanel, IntentParseEvent};

// 创建解析面板
let parser_panel = cx.new(|cx, window| {
    IntentParserPanel::new(window, cx)
        .with_ai_provider(ai_provider)
});

// 监听解析事件
cx.subscribe(&parser_panel, |this, panel, event: &IntentParseEvent, cx| {
    match event {
        IntentParseEvent::ParseCompleted(intent) => {
            // 显示预览
        }
        IntentParseEvent::CreateTask(intent) => {
            // 创建任务
        }
        _ => {}
    }
}).detach();
```

## 支持的测试类型

意图解析引擎支持以下安全测试类型：

| 测试类型 | 标识符 | 描述 |
|---------|--------|------|
| 网络扫描 | `network_scan` | 扫描目标网络，发现存活主机 |
| 端口扫描 | `port_scan` | 扫描目标主机的开放端口和服务 |
| 漏洞扫描 | `vulnerability_scan` | 扫描目标系统的已知漏洞 |
| 协议分析 | `protocol_analysis` | 分析特定协议的安全性和实现 |
| 固件分析 | `firmware_analysis` | 分析固件文件中的安全漏洞 |
| Web应用测试 | `web_app_test` | 测试 Web 应用程序的安全性 |
| API测试 | `api_test` | 测试 API 端点的安全性 |
| 漏洞利用 | `exploit` | 尝试利用发现的漏洞 |

## 置信度评分

解析结果的置信度评分包含以下维度：

- **overall**: 整体置信度 (0.0 - 1.0)
- **category**: 意图分类置信度
- **parameters**: 参数提取置信度
- **target**: 目标识别置信度

```rust
if result.confidence.is_executable(0.7) {
    // 置信度足够高，可以自动执行
}
```

## 配置 AI Provider

意图解析引擎需要配置 AI Provider 才能工作：

```rust
use core::intent_parser::ai_adapter::create_adapter_from_registry;
use ai_provider::ProviderRegistry;

// 从 Registry 创建适配器
let registry = ProviderRegistry::with_defaults();
let adapter = create_adapter_from_registry(&registry, Some("kimi"))
    .expect("Failed to create adapter");

// 创建解析器
let parser = IntentParser::new(adapter);
```

## 错误处理

意图解析引擎定义了两种主要错误类型：

```rust
use core::intent_parser::error::{IntentParseError, IntentExecutionError};

// 解析错误
match parser.parse_security_test(intent).await {
    Err(IntentParseError::LowConfidence { score, threshold }) => {
        // 置信度不足，需要用户确认
    }
    Err(e) => {
        // 其他解析错误
    }
    Ok(result) => {
        // 解析成功
    }
}

// 执行错误
match service.execute_intent(&parsed, user_id).await {
    Err(IntentExecutionError::Sandbox(msg)) => {
        // Sandbox 创建失败
    }
    Err(IntentExecutionError::Agent(msg)) => {
        // Agent 分配失败
    }
    Ok(execution) => {
        // 执行成功
    }
}
```

## 扩展开发

### 添加新的测试类型

1. 在 `security.rs` 中添加新的 `SecurityTestType` 变体
2. 实现 `as_str()`, `display_name()`, `description()` 方法
3. 定义所需的 Agent 能力
4. 在 `executor.rs` 中添加执行计划生成逻辑

### 自定义 Sandbox 后端

```rust
use core::execution::sandbox_manager::{SandboxManager, SandboxBackend};

let manager = SandboxManager::with_backend(SandboxBackend::BoxLite);
```

## API 参考

### IntentParser

```rust
impl IntentParser {
    pub fn new(ai_provider: Arc<dyn AiProvider>) -> Self;
    pub fn with_config(ai_provider: Arc<dyn AiProvider>, config: ParserConfig) -> Self;
    pub async fn parse_security_test(&self, intent: Intent) -> IntentResult<ParseResult>;
    pub async fn classify_intent(&self, text: &str) -> IntentResult<IntentCategory>;
}
```

### ExecutionService

```rust
impl ExecutionService {
    pub fn new() -> Self;
    pub fn with_config(config: ExecutionConfig) -> Self;
    pub async fn execute_intent(&self, parsed: &ParsedSecurityIntent, user_id: Option<i64>) -> IntentExecResult<ExecutionContext>;
    pub async fn cancel_execution(&self, execution_id: Uuid) -> IntentExecResult<()>;
}
```

## 测试

运行测试：

```bash
cargo test -p core --test intent_parser_integration
```

## 依赖

- `ai_provider`: AI 提供者集成
- `data`: 数据模型和持久化
- `tokio`: 异步运行时
- `serde`: 序列化

## 许可证

[与 UAVRed 项目相同]
