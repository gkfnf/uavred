# AI Provider 集成指南

本指南说明如何将 `ai_provider` crate 与项目的其他部分（settings_ui 和 kanban_ui）集成。

## 目录
1. [与 Settings UI 集成](#与-settings-ui-集成)
2. [与 Kanban Agent 集成](#与-kanban-agent-集成)
3. [配置管理](#配置管理)
4. [最佳实践](#最佳实践)

---

## 与 Settings UI 集成

### 1. 更新 Cargo.toml

`settings_ui` 已添加对 `ai_provider` 的依赖：

```toml
[dependencies]
ai_provider = { path = "../ai_provider" }
```

### 2. 替换现有的 AI 客户端

将 `settings_ui/src/ai_client.rs` 中的内容替换为使用 `ai_provider`：

```rust
use ai_provider::{ProviderRegistry, ProviderId};

pub struct AiProviderService {
    registry: ProviderRegistry,
}

impl AiProviderService {
    pub fn new() -> Self {
        Self {
            registry: ProviderRegistry::with_defaults(),
        }
    }
    
    pub async fn test_provider_connection(&self, provider_id: &str) -> anyhow::Result<String> {
        let id = ProviderId::new(provider_id);
        let provider = self.registry
            .get(&id)
            .ok_or_else(|| anyhow::anyhow!("Provider not found: {}", provider_id))?;
        
        let result = provider.test_connection().await?;
        
        if result.success {
            Ok(format!("✓ 连接成功 ({} ms, {} 个模型)", 
                result.latency_ms, 
                result.models_available.unwrap_or(0)))
        } else {
            Err(anyhow::anyhow!("连接失败: {}", result.message))
        }
    }
    
    pub async fn fetch_models(&self, provider_id: &str) -> anyhow::Result<Vec<ModelInfo>> {
        let id = ProviderId::new(provider_id);
        let provider = self.registry
            .get(&id)
            .ok_or_else(|| anyhow::anyhow!("Provider not found"))?;
        
        provider.fetch_models().await
            .map_err(|e| anyhow::anyhow!("Failed to fetch models: {}", e))
    }
}
```

### 3. 更新设置面板

修改 `settings_ui/src/ai_settings.rs`：

```rust
use ai_provider::{ProviderRegistry, ProviderId, ProviderMetadata};

pub struct AiSettingsPanel {
    registry: ProviderRegistry,
    selected_provider: ProviderId,
    // ... 其他字段
}

impl AiSettingsPanel {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            registry: ProviderRegistry::with_defaults(),
            selected_provider: ProviderId::new("kimi"),
            // ...
        }
    }
    
    fn test_connection(&mut self, cx: &mut Context<Self>) {
        if let Some(provider) = self.registry.get(&self.selected_provider) {
            cx.spawn(async move |this, cx| {
                let result = provider.test_connection().await;
                
                cx.update(|cx| {
                    this.update(cx, |this, cx| {
                        match result {
                            Ok(test_result) => {
                                this.status_message = Some(test_result.message);
                                this.status_is_error = !test_result.success;
                            }
                            Err(e) => {
                                this.status_message = Some(e.to_string());
                                this.status_is_error = true;
                            }
                        }
                        cx.notify();
                    }).ok();
                }).ok();
            }).detach();
        }
    }
    
    fn fetch_models(&mut self, cx: &mut Context<Self>) {
        if let Some(provider) = self.registry.get(&self.selected_provider) {
            self.is_loading = true;
            cx.notify();
            
            cx.spawn(async move |this, cx| {
                let result = provider.fetch_models().await;
                
                cx.update(|cx| {
                    this.update(cx, |this, cx| {
                        this.is_loading = false;
                        match result {
                            Ok(models) => {
                                // 更新配置中的模型列表
                                if let Some(config) = this.get_current_provider_config_mut() {
                                    config.models = models.iter().map(|m| AiModel {
                                        id: m.id.as_str().to_string(),
                                        name: m.name.clone(),
                                        description: m.description.clone(),
                                        enabled: true,
                                        token_limit: Some(m.max_tokens),
                                        supports_vision: Some(m.capabilities.supports_vision),
                                        supports_reasoning: Some(m.capabilities.supports_reasoning),
                                    }).collect();
                                }
                                this.status_message = Some(format!("已获取 {} 个模型", models.len()));
                                this.status_is_error = false;
                            }
                            Err(e) => {
                                this.status_message = Some(e.to_string());
                                this.status_is_error = true;
                            }
                        }
                        cx.notify();
                    }).ok();
                }).ok();
            }).detach();
        }
    }
}
```

---

## 与 Kanban Agent 集成

### 1. 更新 agent crate 的 Cargo.toml

```toml
[dependencies]
ai_provider = { path = "../ai_provider" }
```

### 2. 创建 AI Agent 服务

在 `agent/src/` 下创建 `ai_service.rs`：

```rust
use ai_provider::{
    ProviderRegistry, ChatCompletionRequest, ChatMessage,
    ModelId, ProviderId
};
use std::sync::Arc;

pub struct AiAgentService {
    registry: Arc<ProviderRegistry>,
}

impl AiAgentService {
    pub fn new() -> Self {
        Self {
            registry: Arc::new(ProviderRegistry::with_defaults()),
        }
    }
    
    /// 执行单个任务
    pub async fn execute_task(
        &self,
        provider_id: &str,
        model_id: &str,
        system_prompt: &str,
        user_prompt: &str,
    ) -> anyhow::Result<TaskResult> {
        let provider = self.registry
            .get(&ProviderId::new(provider_id))
            .ok_or_else(|| anyhow::anyhow!("Provider not found: {}", provider_id))?;
        
        let request = ChatCompletionRequest::new(
            ModelId::new(model_id),
            vec![
                ChatMessage::system(system_prompt),
                ChatMessage::user(user_prompt),
            ],
        ).with_temperature(0.7);
        
        let start = std::time::Instant::now();
        let response = provider.chat_completion(request).await
            .map_err(|e| anyhow::anyhow!("AI request failed: {}", e))?;
        
        Ok(TaskResult {
            content: response.content,
            tokens_used: response.usage.total_tokens,
            latency_ms: start.elapsed().as_millis() as u64,
        })
    }
    
    /// 选择最佳供应商（基于延迟）
    pub async fn select_best_provider(&self) -> Option<ProviderId> {
        let mut best_latency = u64::MAX;
        let mut best_provider = None;
        
        for provider in self.registry.list().iter() {
            if !provider.is_authenticated() {
                continue;
            }
            
            if let Ok(latency) = provider.get_latency().await {
                if latency < best_latency {
                    best_latency = latency;
                    best_provider = Some(provider.provider_id());
                }
            }
        }
        
        best_provider
    }
    
    /// 获取适合任务的模型
    pub fn get_model_for_task(&self, provider_id: &ProviderId, task_type: TaskType) -> Option<ModelId> {
        let provider = self.registry.get(provider_id)?;
        let models = provider.available_models();
        
        match task_type {
            TaskType::Code => models.iter()
                .find(|m| m.id.as_str().contains("code") || m.id.as_str().contains("coder"))
                .map(|m| m.id.clone()),
            TaskType::Vision => models.iter()
                .find(|m| m.capabilities.supports_vision)
                .map(|m| m.id.clone()),
            TaskType::Reasoning => models.iter()
                .find(|m| m.capabilities.supports_reasoning)
                .map(|m| m.id.clone()),
            _ => models.first().map(|m| m.id.clone()),
        }
    }
}

pub struct TaskResult {
    pub content: String,
    pub tokens_used: u32,
    pub latency_ms: u64,
}

pub enum TaskType {
    General,
    Code,
    Vision,
    Reasoning,
}
```

### 3. 在 Kanban 中使用

```rust
use agent::ai_service::{AiAgentService, TaskType};

pub struct KanbanTask {
    ai_service: Arc<AiAgentService>,
}

impl KanbanTask {
    pub async fn process_with_ai(&self, task: &Task) -> anyhow::Result<String> {
        // 自动选择最佳供应商
        let provider_id = self.ai_service.select_best_provider().await
            .ok_or_else(|| anyhow::anyhow!("No available AI provider"))?;
        
        // 获取适合的模型
        let model_id = self.ai_service.get_model_for_task(&provider_id, TaskType::General)
            .ok_or_else(|| anyhow::anyhow!("No suitable model found"))?;
        
        // 执行任务
        let result = self.ai_service.execute_task(
            provider_id.as_str(),
            model_id.as_str(),
            "You are a helpful task assistant.",
            &task.description,
        ).await?;
        
        Ok(result.content)
    }
}
```

---

## 配置管理

### 配置文件格式

```json
{
  "ai": {
    "active_provider": "kimi",
    "providers": {
      "kimi": {
        "enabled": true,
        "endpoint": "https://api.moonshot.cn",
        "api_key": "sk-xxx",
        "models": [...]
      },
      "deepseek": {
        "enabled": false,
        "endpoint": "https://api.deepseek.com",
        "api_key": null
      }
    }
  }
}
```

### 从配置创建 Provider

```rust
use ai_provider::{ProviderBuilder, ProviderConfig};

fn create_provider_from_config(config: &AiProviderConfig, id: &str) -> Box<dyn AiProvider> {
    let provider_config = ProviderBuilder::new(ProviderId::new(id))
        .with_endpoint(&config.endpoint)
        .with_api_key(config.api_key.clone().unwrap_or_default())
        .with_timeout(config.timeout_seconds.unwrap_or(60))
        .enabled(config.enabled)
        .build_config();
    
    match id {
        "kimi" => Box::new(KimiProvider::with_config(provider_config)),
        "deepseek" => Box::new(DeepSeekProvider::with_config(provider_config)),
        // ...
        _ => panic!("Unknown provider: {}", id),
    }
}
```

---

## 最佳实践

### 1. 错误处理

```rust
match provider.chat_completion(request).await {
    Ok(response) => response,
    Err(ApiError::Authentication(_)) => {
        // 提示用户配置 API 密钥
        show_api_key_dialog();
        return;
    }
    Err(ApiError::RateLimit(_)) => {
        // 等待后重试
        tokio::time::sleep(Duration::from_secs(5)).await;
        retry().await
    }
    Err(e) => {
        tracing::error!("AI request failed: {}", e);
        show_error_to_user(e.to_string());
    }
}
```

### 2. 超时设置

```rust
// 本地模型需要更长的超时
let local_timeout = 120; // 2 minutes for local models

// 云端模型正常超时
let cloud_timeout = 60;  // 1 minute for cloud APIs
```

### 3. 模型选择策略

```rust
pub fn select_model(models: &[ModelInfo], requirements: &Requirements) -> Option<ModelId> {
    models.iter()
        .filter(|m| m.capabilities.supports_chat)
        .filter(|m| !requirements.needs_vision || m.capabilities.supports_vision)
        .filter(|m| !requirements.needs_tools || m.capabilities.supports_tools)
        .max_by_key(|m| m.max_tokens)
        .map(|m| m.id.clone())
}
```

### 4. 连接池管理

`AiHttpClient` 内部使用 `reqwest::Client`，它已经自动管理连接池，无需额外配置。

---

## 故障排查

### 常见问题

1. **连接超时**
   - 检查网络连接
   - 增加超时设置
   - 检查防火墙设置

2. **认证失败**
   - 验证 API 密钥
   - 检查环境变量
   - 确认密钥格式正确

3. **模型不存在**
   - 先调用 `fetch_models()` 获取可用模型
   - 使用准确的模型 ID

4. **本地供应商连接失败**
   - 确认 Ollama/LMStudio 正在运行
   - 检查端口号是否正确
   - 验证模型已加载
