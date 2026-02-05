//! AI Provider 适配器
//!
//! 将 ai_provider crate 与 intent_parser 集成

use super::parser::{AiProvider, ChatCompletionRequest};
use ai_provider::{AiProvider as ExternalAiProvider, ChatCompletionRequest as ExternalRequest};
use std::sync::Arc;

/// AI Provider 适配器包装
pub struct AiProviderAdapter {
    provider: Arc<dyn ExternalAiProvider>,
    model: String,
}

impl AiProviderAdapter {
    /// 从外部 AI Provider 创建适配器
    pub fn new(provider: Arc<dyn ExternalAiProvider>) -> Self {
        let model = provider.available_models()
            .first()
            .map(|m| m.id.0.clone())
            .unwrap_or_else(|| "default".to_string());
        
        Self { provider, model }
    }

    /// 指定模型创建适配器
    pub fn with_model(provider: Arc<dyn ExternalAiProvider>, model: impl Into<String>) -> Self {
        Self {
            provider,
            model: model.into(),
        }
    }

    /// 从 ProviderRegistry 创建适配器（使用默认或指定 provider）
    pub fn from_registry(
        registry: &ai_provider::ProviderRegistry,
        provider_id: Option<&str>,
    ) -> Option<Self> {
        let provider = if let Some(id) = provider_id {
            registry.get(&ai_provider::ProviderId::new(id))?
        } else {
            registry.get_default()?
        };

        Some(Self::new(provider))
    }
}

#[async_trait::async_trait]
impl AiProvider for AiProviderAdapter {
    async fn chat_completion(&self, request: ChatCompletionRequest) -> Result<String, String> {
        // 转换消息格式
        let messages: Vec<ai_provider::ChatMessage> = request
            .messages
            .into_iter()
            .map(|msg| match msg.role.as_str() {
                "system" => ai_provider::ChatMessage::system(msg.content),
                "user" => ai_provider::ChatMessage::user(msg.content),
                "assistant" => ai_provider::ChatMessage::assistant(msg.content),
                _ => ai_provider::ChatMessage::user(msg.content),
            })
            .collect();

        // 构建外部请求
        let model_id = if request.model.is_empty() {
            ai_provider::ModelId::new(&self.model)
        } else {
            ai_provider::ModelId::new(request.model)
        };

        let external_request = ExternalRequest::new(model_id, messages)
            .with_temperature(request.temperature)
            .with_max_tokens(request.max_tokens);

        // 发送请求
        match self.provider.chat_completion(external_request).await {
            Ok(response) => Ok(response.content),
            Err(e) => Err(format!("AI Provider error: {}", e)),
        }
    }

    fn default_model(&self) -> String {
        self.model.clone()
    }
}

/// 创建适配器的便捷函数
pub fn create_adapter(provider: Arc<dyn ExternalAiProvider>) -> Arc<dyn AiProvider> {
    Arc::new(AiProviderAdapter::new(provider))
}

/// 从 registry 创建适配器的便捷函数
pub fn create_adapter_from_registry(
    registry: &ai_provider::ProviderRegistry,
    provider_id: Option<&str>,
) -> Option<Arc<dyn AiProvider>> {
    AiProviderAdapter::from_registry(registry, provider_id).map(|a| Arc::new(a) as Arc<dyn AiProvider>)
}
