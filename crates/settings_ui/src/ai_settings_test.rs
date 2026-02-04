//! AI Settings Panel Tests
//! 
//! Tests for AI provider configuration interface interactions

#[cfg(test)]
mod tests {
    use gpui::*;
    use gpui_component::input::InputState;
    
    use crate::ai_settings::{AiSettingsPanel, ProviderUi};
    use crate::config::{Settings, AiProviderConfig, AiModel};
    use crate::provider::ProviderId;

    /// Test setup helper - creates a test window context
    fn init_test(cx: &mut TestAppContext) -> (Entity<AiSettingsPanel>, WindowHandle<()>) {
        cx.update(|cx| {
            let window = cx.open_window(WindowOptions::default(), |window, cx| {
                let panel = cx.new(|cx| AiSettingsPanel::new(window, cx));
                cx.new(|_cx| ())
            }).unwrap();
            
            let panel = cx.update_window(window, |_, cx| {
                cx.new(|cx| {
                    AiSettingsPanel::new(cx.windows().first().unwrap().clone(), cx)
                })
            }).unwrap();
            
            (panel, window)
        })
    }

    #[gpui::test]
    async fn test_provider_list_initialization(cx: &mut TestAppContext) {
        let (panel, _window) = init_test(cx);
        
        cx.update(|cx| {
            panel.read_with(cx, |panel, _cx| {
                // Verify all 5 providers are loaded
                let providers = vec![
                    ("kimi", "Kimi (Moonshot)"),
                    ("deepseek", "DeepSeek"),
                    ("openai", "OpenAI"),
                    ("claude", "Claude"),
                    ("ollama", "Ollama"),
                ];
                
                for (id, name) in providers {
                    // Providers should be available in the UI list
                    assert!(panel.filtered_providers().iter().any(|p| {
                        p.id.as_str() == id && p.name == name
                    }), "Provider {} should be in the list", name);
                }
            });
        });
    }

    #[gpui::test]
    async fn test_provider_selection(cx: &mut TestAppContext) {
        let (panel, _window) = init_test(cx);
        
        cx.update(|cx| {
            panel.update(cx, |panel, cx| {
                // Initially should have a selected provider
                let initial_id = panel.selected_provider_id.clone();
                
                // Select DeepSeek
                let deepseek_id = ProviderId::new("deepseek");
                panel.select_provider(deepseek_id.clone(), cx);
                
                // Verify selection changed
                assert_eq!(panel.selected_provider_id, deepseek_id);
                assert_ne!(panel.selected_provider_id, initial_id);
                
                // Select Kimi
                let kimi_id = ProviderId::new("kimi");
                panel.select_provider(kimi_id.clone(), cx);
                assert_eq!(panel.selected_provider_id, kimi_id);
            });
        });
    }

    #[gpui::test]
    async fn test_provider_enable_toggle(cx: &mut TestAppContext) {
        let (panel, _window) = init_test(cx);
        
        cx.update(|cx| {
            panel.update(cx, |panel, cx| {
                // Select a provider
                let kimi_id = ProviderId::new("kimi");
                panel.select_provider(kimi_id.clone(), cx);
                
                // Initially should be disabled
                let initial_enabled = panel.is_provider_active();
                
                // Toggle enable
                panel.toggle_provider_enabled(cx);
                
                // Should now be enabled
                assert!(panel.is_provider_active(), "Provider should be enabled after toggle");
                assert!(!initial_enabled || panel.is_provider_active() != initial_enabled);
                
                // Toggle again
                panel.toggle_provider_enabled(cx);
                assert!(!panel.is_provider_active(), "Provider should be disabled after second toggle");
            });
        });
    }

    #[gpui::test]
    async fn test_provider_search_filter(cx: &mut TestAppContext) {
        let (panel, _window) = init_test(cx);
        
        cx.update(|cx| {
            panel.update(cx, |panel, _cx| {
                // All providers visible initially
                let all_count = panel.filtered_providers().len();
                assert_eq!(all_count, 5, "Should have 5 providers initially");
                
                // Filter for "deep"
                panel.provider_search_query = "deep".to_string();
                let filtered = panel.filtered_providers();
                assert_eq!(filtered.len(), 1, "Should have 1 provider matching 'deep'");
                assert_eq!(filtered[0].id.as_str(), "deepseek");
                
                // Filter for "ai"
                panel.provider_search_query = "ai".to_string();
                let ai_filtered = panel.filtered_providers();
                assert!(ai_filtered.len() >= 2, "Should have multiple providers with 'ai' in name or description");
                
                // Clear filter
                panel.provider_search_query = String::new();
                assert_eq!(panel.filtered_providers().len(), 5, "Should show all providers after clearing filter");
            });
        });
    }

    #[gpui::test]
    async fn test_default_endpoints(cx: &mut TestAppContext) {
        let (panel, _window) = init_test(cx);
        
        cx.update(|cx| {
            panel.read_with(cx, |panel, _cx| {
                // Verify default endpoints for each provider
                assert_eq!(panel.get_default_endpoint("kimi"), "https://api.moonshot.cn");
                assert_eq!(panel.get_default_endpoint("deepseek"), "https://api.deepseek.com");
                assert_eq!(panel.get_default_endpoint("openai"), "https://api.openai.com/v1");
                assert_eq!(panel.get_default_endpoint("claude"), "https://api.anthropic.com");
                assert_eq!(panel.get_default_endpoint("ollama"), "http://localhost:11434");
            });
        });
    }

    #[gpui::test]
    async fn test_model_toggle(cx: &mut TestAppContext) {
        let (panel, _window) = init_test(cx);
        
        cx.update(|cx| {
            panel.update(cx, |panel, cx| {
                // Select Kimi provider
                let kimi_id = ProviderId::new("kimi");
                panel.select_provider(kimi_id.clone(), cx);
                
                // Enable the provider first
                panel.toggle_provider_enabled(cx);
                
                // Add a test model to the config
                if let Some(config) = panel.get_current_provider_config_mut() {
                    config.models.push(AiModel {
                        id: "test-model".to_string(),
                        name: "Test Model".to_string(),
                        description: None,
                        enabled: true,
                        token_limit: None,
                        supports_vision: None,
                        supports_reasoning: None,
                    });
                }
                
                // Toggle model
                panel.toggle_model_enabled("test-model", cx);
                
                // Verify model is disabled
                if let Some(config) = panel.get_current_provider_config() {
                    let model = config.models.iter().find(|m| m.id == "test-model");
                    assert!(model.is_some(), "Model should exist");
                    assert!(!model.unwrap().enabled, "Model should be disabled after toggle");
                }
            });
        });
    }

    #[gpui::test]
    async fn test_model_search_filter(cx: &mut TestAppContext) {
        let (panel, _window) = init_test(cx);
        
        cx.update(|cx| {
            panel.update(cx, |panel, cx| {
                // Select Kimi and enable it
                let kimi_id = ProviderId::new("kimi");
                panel.select_provider(kimi_id.clone(), cx);
                panel.toggle_provider_enabled(cx);
                
                // Add test models
                if let Some(config) = panel.get_current_provider_config_mut() {
                    config.models = vec![
                        AiModel {
                            id: "kimi-k2".to_string(),
                            name: "Kimi K2".to_string(),
                            description: None,
                            enabled: true,
                            token_limit: None,
                            supports_vision: None,
                            supports_reasoning: None,
                        },
                        AiModel {
                            id: "moonshot-v1".to_string(),
                            name: "Moonshot V1".to_string(),
                            description: None,
                            enabled: true,
                            token_limit: None,
                            supports_vision: None,
                            supports_reasoning: None,
                        },
                    ];
                }
                
                // Search for "k2"
                panel.model_search_query = "k2".to_string();
                let filtered = panel.filtered_models();
                assert_eq!(filtered.len(), 1, "Should have 1 model matching 'k2'");
                assert_eq!(filtered[0].id, "kimi-k2");
                
                // Search for "moonshot"
                panel.model_search_query = "moonshot".to_string();
                let moonshot_filtered = panel.filtered_models();
                assert_eq!(moonshot_filtered.len(), 1);
                assert_eq!(moonshot_filtered[0].id, "moonshot-v1");
            });
        });
    }

    #[gpui::test]
    async fn test_save_settings(cx: &mut TestAppContext) {
        let (panel, _window) = init_test(cx);
        
        cx.update(|cx| {
            panel.update(cx, |panel, cx| {
                // Make some changes
                let kimi_id = ProviderId::new("kimi");
                panel.select_provider(kimi_id.clone(), cx);
                panel.toggle_provider_enabled(cx);
                
                // Save settings
                panel.save_settings(cx);
                
                // Verify success status
                assert!(panel.status_message.is_some(), "Status message should be set after save");
                assert!(!panel.status_is_error, "Save should not produce error");
            });
        });
    }

    #[gpui::test]
    async fn test_connection_test_state(cx: &mut TestAppContext) {
        let (panel, _window) = init_test(cx);
        
        cx.update(|cx| {
            panel.update(cx, |panel, cx| {
                let kimi_id = ProviderId::new("kimi");
                panel.select_provider(kimi_id.clone(), cx);
                
                // Initially not loading
                assert!(!panel.is_loading);
                
                // Note: We can't easily test the actual connection test without mocking
                // the HTTP client, but we can verify the loading state is managed
            });
        });
    }

    #[gpui::test]
    async fn test_integration_section_rendering(cx: &mut TestAppContext) {
        let (panel, _window) = init_test(cx);
        
        cx.update(|cx| {
            panel.update(cx, |panel, cx| {
                // Test Kimi integration section
                let kimi_id = ProviderId::new("kimi");
                panel.select_provider(kimi_id, cx);
                
                let kimi_config = AiProviderConfig {
                    enabled: true,
                    endpoint: "https://api.moonshot.cn".to_string(),
                    api_key: Some("test-key".to_string()),
                    models: vec![],
                    region: None,
                    alt_endpoints: vec![],
                    claude_code: None,
                };
                
                // Kimi should have integration section
                assert!(
                    panel.render_integration_section("kimi", &kimi_config).is_some(),
                    "Kimi should have integration section"
                );
                
                // Test DeepSeek integration section
                let deepseek_id = ProviderId::new("deepseek");
                panel.select_provider(deepseek_id, cx);
                
                let deepseek_config = AiProviderConfig {
                    enabled: true,
                    endpoint: "https://api.deepseek.com".to_string(),
                    api_key: Some("test-key".to_string()),
                    models: vec![],
                    region: None,
                    alt_endpoints: vec![],
                    claude_code: None,
                };
                
                assert!(
                    panel.render_integration_section("deepseek", &deepseek_config).is_some(),
                    "DeepSeek should have integration section"
                );
                
                // OpenAI should NOT have integration section (not implemented)
                let openai_config = AiProviderConfig::default();
                assert!(
                    panel.render_integration_section("openai", &openai_config).is_none(),
                    "OpenAI should not have integration section yet"
                );
            });
        });
    }
}
