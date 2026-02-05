# UAVRed 意图解析引擎集成指南

## 概述

本文档介绍如何将意图解析引擎与 Kanban UI 集成，实现从自然语言输入到安全测试任务执行的完整流程。

## 集成架构

```
┌─────────────────────────────────────────────────────────────────┐
│                      Kanban UI (kanban_ui)                       │
│  ┌──────────────┐  ┌──────────────────┐  ┌──────────────────┐  │
│  │KanbanBoard   │  │ IntentParserPanel│  │KanbanWithIntent  │  │
│  └──────────────┘  └──────────────────┘  └──────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────┐
│                  Core Intent Parser (core)                       │
│  ┌──────────────┐  ┌──────────────────┐  ┌──────────────────┐  │
│  │IntentParser  │  │SecurityTestIntent│  │IntentExecutor    │  │
│  └──────────────┘  └──────────────────┘  └──────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────┐
│                  Execution Service (core)                        │
│  ┌──────────────┐  ┌──────────────────┐  ┌──────────────────┐  │
│  │ExecutionSvc  │  │SandboxManager    │  │AgentScheduler    │  │
│  └──────────────┘  └──────────────────┘  └──────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

## 使用方式

### 1. 基本使用流程

在 Kanban 列中添加任务按钮，点击后打开 Intent Parser 面板：

```rust
use kanban_ui::{KanbanColumn, IntentParserPanel, IntentParseEvent};
use core::intent_parser::ai_adapter::create_adapter_from_registry;
use ai_provider::ProviderRegistry;

// 1. 创建 AI Provider
let registry = ProviderRegistry::with_defaults();
let ai_provider = create_adapter_from_registry(&registry, Some("kimi"));

// 2. 创建 Kanban 列，传入添加任务回调
let column = KanbanColumn::new(TaskStatus::Todo)
    .tasks(tasks)
    .on_add_task(|status, window, cx| {
        // 3. 打开 IntentParserPanel 对话框
        let panel = cx.new(|cx, window| {
            IntentParserPanel::new(window, cx)
                .with_ai_provider(ai_provider.clone())
        });
        
        // 4. 订阅解析事件
        cx.subscribe(&panel, |this, panel, event: &IntentParseEvent, cx| {
            match event {
                IntentParseEvent::CreateTask(parsed_intent) => {
                    // 5. 创建任务
                    let task_data = TaskData::new(
                        next_id,
                        parsed_intent.task_name(),
                        parsed_intent.security_intent.test_type.as_str().to_string(),
                        parsed_intent.security_intent.suggested_priority().as_str().to_string(),
                        status.as_str().to_string(),
                    );
                    
                    // 6. 添加到 TaskStore
                    TaskStore::global(cx).update(cx, |store, cx| {
                        store.add_task(task_data, cx);
                    });
                }
                _ => {}
            }
        }).detach();
        
        // 7. 打开对话框
        window.open_dialog(cx, |dialog, _window, _cx| {
            dialog
                .title("添加安全测试任务")
                .w(px(700.0))
                .child(panel)
        });
    });
```

### 2. 在 Dashboard 中集成

```rust
// crates/dashboard_ui/src/dashboard_panel.rs

use kanban_ui::{KanbanColumn, IntentParserPanel, IntentParseEvent};
use core::execution::ExecutionService;
use core::intent_parser::ai_adapter::create_adapter_from_registry;

impl DashboardPanel {
    pub fn open_add_task_dialog(
        &mut self,
        status: TaskStatus,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // 创建 Intent Parser Panel
        let panel = cx.new(|cx, window| {
            let ai_provider = create_adapter_from_registry(
                &self.ai_provider_registry,
                Some("kimi")
            ).expect("AI Provider not found");
            
            IntentParserPanel::new(window, cx)
                .with_ai_provider(ai_provider)
        });
        
        // 订阅解析事件
        cx.subscribe(&panel, |this, panel, event: &IntentParseEvent, cx| {
            match event {
                IntentParseEvent::CreateTask(parsed_intent) => {
                    // 创建任务数据
                    let task_data = this.create_task_from_parsed(parsed_intent, status);
                    
                    // 添加到看板
                    this.add_task(task_data.clone(), cx);
                    
                    // 执行安全测试（如果设置了自动执行）
                    cx.spawn(async move |this, cx| {
                        let result = this.update(cx, |this, _cx| {
                            this.execution_service.execute_intent(
                                &parsed_intent, 
                                this.current_user_id
                            ).await
                        });
                        
                        match result {
                            Ok(execution) => {
                                eprintln!("安全测试执行中: {:?}", execution.execution_id);
                            }
                            Err(e) => {
                                eprintln!("执行失败: {:?}", e);
                            }
                        }
                    }).detach();
                }
                IntentParseEvent::ParseFailed(error) => {
                    eprintln!("解析失败: {}", error);
                }
                _ => {}
            }
        }).detach();
        
        // 打开对话框
        window.open_dialog(cx, |dialog, _window, _cx| {
            dialog
                .title("AI 意图解析 - 创建安全测试任务")
                .w(px(700.0))
                .child(panel)
        });
    }
    
    fn create_task_from_parsed(
        &self,
        parsed: &ParsedSecurityIntent,
        status: TaskStatus
    ) -> TaskData {
        TaskData::new(
            self.get_next_task_id(),
            parsed.task_name(),
            parsed.security_intent.test_type.as_str().to_string(),
            parsed.security_intent.suggested_priority().as_str().to_string(),
            status.as_str().to_string(),
        )
    }
}
```

### 3. 完整的 Intent Parser 工作流程

```rust
// 1. 用户输入意图
let user_input = "扫描 192.168.1.0/24 网段的所有开放端口";

// 2. 创建意图
let intent = core::intent_parser::Intent::from(user_input);

// 3. 创建解析器并解析
let parser = IntentParser::new(ai_provider);
let result = parser.parse_security_test(intent).await?;

// 4. 检查置信度
if result.confidence.is_executable(0.7) {
    // 5. 创建任务
    let executor = IntentExecutor::new();
    let task = executor.create_task(&result)?;
    
    // 6. 生成执行计划
    let plan = executor.generate_execution_plan(&result)?;
    
    // 7. 执行（创建 Sandbox，分配 Agent）
    let execution = ExecutionService::new()
        .execute_intent(&result, user_id)
        .await?;
    
    println!("执行ID: {:?}", execution.execution_id);
}
```

## API 参考

### KanbanColumn

```rust
impl KanbanColumn {
    /// 创建新列
    pub fn new(status: TaskStatus) -> Self;
    
    /// 设置任务列表
    pub fn tasks(mut self, tasks: Vec<TaskData>) -> Self;
    
    /// 设置添加任务回调
    pub fn on_add_task(
        mut self, 
        handler: impl Fn(TaskStatus, &mut Window, &mut App) + 'static
    ) -> Self;
}
```

### IntentParserPanel

```rust
impl IntentParserPanel {
    /// 创建新面板
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self;
    
    /// 设置 AI Provider
    pub fn with_ai_provider(mut self, provider: Arc<dyn AiProvider>) -> Self;
    
    /// 开始解析
    pub fn start_parse(&mut self, window: &mut Window, cx: &mut Context<Self>);
    
    /// 获取解析结果
    pub fn parsed_result(&self) -> Option<&ParsedSecurityIntent>;
}
```

### IntentParseEvent

```rust
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
```

## 配置 AI Provider

```rust
use core::intent_parser::ai_adapter::{create_adapter, create_adapter_from_registry};

// 方式1: 从 registry 创建
let registry = ai_provider::ProviderRegistry::with_defaults();
let adapter = create_adapter_from_registry(&registry, Some("kimi"));

// 方式2: 从已配置的 provider 创建
let kimi_provider = registry.get(&"kimi".into()).unwrap();
let adapter = create_adapter(kimi_provider);
```

## 执行配置

```rust
use core::execution::{ExecutionService, ExecutionConfig};

let config = ExecutionConfig {
    auto_create_sandbox: true,
    auto_assign_agent: true,
    default_sandbox_image: "uavred/agent:latest".to_string(),
    execution_timeout_seconds: 3600,
    keep_sandbox: false,
};

let service = ExecutionService::with_config(config)
    .with_sandbox_manager(SandboxManager::new())
    .with_agent_scheduler(AgentScheduler::new());
```

## 注意事项

1. **AI Provider 配置**: 确保在设置中配置了 AI Provider (API key 等)
2. **Sandbox 环境**: 生产环境建议使用 BoxLite 或 Docker 后端
3. **错误处理**: 解析失败时会返回 `IntentParseEvent::ParseFailed`
4. **置信度阈值**: 默认需要 0.7 以上的置信度才会建议自动执行

## 示例代码

参见:
- `crates/kanban_ui/src/intent/integration_example.rs` - 完整集成示例
- `crates/core/tests/intent_parser_integration.rs` - 测试用例

## 下一步

1. 配置 AI Provider 设置
2. 在 Dashboard 中实现 `open_add_task_dialog` 方法
3. 添加任务执行状态监控
4. 集成任务执行结果回调
