//! 意图解析面板 - 用于在 Kanban UI 中解析用户意图

use super::{confidence_color, format_confidence, test_type_display, test_type_icon, IntentParseEvent, ParseState};
use core::intent_parser::{
    parser::{AiProvider, IntentParser, ParserConfig},
    Intent,
};
use gpui::*;
use gpui_component::{
    button::{Button, ButtonVariants as _},
    h_flex,
    input::{Input, InputState},
    label::Label,
    v_flex, Disableable, IconName,
};
use std::sync::Arc;

/// 意图解析面板
pub struct IntentParserPanel {
    /// 输入状态
    input_state: Entity<InputState>,
    /// 解析状态
    parse_state: ParseState,
    /// 解析结果
    parsed_result: Option<core::intent_parser::security::ParsedSecurityIntent>,
    /// AI Provider
    ai_provider: Option<Arc<dyn AiProvider>>,
    /// 是否显示预览
    show_preview: bool,
    /// 订阅
    _subscriptions: Vec<Subscription>,
}

impl IntentParserPanel {
    /// 创建新的解析面板
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let input_state = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("例如：扫描 192.168.1.0/24 网段的所有开放端口和服务")
        });

        Self {
            input_state,
            parse_state: ParseState::Idle,
            parsed_result: None,
            ai_provider: None,
            show_preview: false,
            _subscriptions: Vec::new(),
        }
    }

    /// 设置 AI Provider
    pub fn with_ai_provider(mut self, provider: Arc<dyn AiProvider>) -> Self {
        self.ai_provider = Some(provider);
        self
    }

    /// 获取当前输入文本
    pub fn get_input(&self, cx: &App) -> String {
        self.input_state.read(cx).value().to_string()
    }

    /// 设置输入文本
    pub fn set_input(&mut self, text: impl Into<String>, window: &mut Window, cx: &mut Context<Self>) {
        let text = text.into();
        self.input_state.update(cx, |state, cx| {
            state.set_value(&text, window, cx);
        });
    }

    /// 开始解析
    pub fn start_parse(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let text = self.get_input(cx);
        
        if text.trim().is_empty() {
            self.parse_state = ParseState::Error("请输入安全测试意图".to_string());
            cx.notify();
            return;
        }

        let Some(ai_provider) = self.ai_provider.clone() else {
            self.parse_state = ParseState::Error("AI Provider 未配置".to_string());
            cx.notify();
            return;
        };

        self.parse_state = ParseState::Parsing;
        self.show_preview = false;
        cx.notify();

        // 创建解析器
        let parser = IntentParser::new(ai_provider);
        let raw_text = text.clone();
        let intent = Intent::from(text);

        // 异步解析
        cx.spawn(async move |this, cx| {
            match parser.parse_security_test(intent).await {
                Ok(result) => {
                    // 构造 ParsedSecurityIntent
                    let parsed = core::intent_parser::security::ParsedSecurityIntent {
                        raw_intent: raw_text,
                        security_intent: result.security_intent,
                        confidence: result.confidence,
                        metadata: result.metadata,
                        suggestions: result.suggestions,
                    };
                    cx.update(|cx| {
                        this.update(cx, |panel, cx| {
                            panel.parse_state = ParseState::Success;
                            panel.parsed_result = Some(parsed.clone());
                            panel.show_preview = true;
                            cx.emit(IntentParseEvent::ParseCompleted(parsed));
                            cx.notify();
                        }).ok();
                    }).ok();
                }
                Err(e) => {
                    cx.update(|cx| {
                        this.update(cx, |panel, cx| {
                            panel.parse_state = ParseState::Error(e.to_string());
                            cx.emit(IntentParseEvent::ParseFailed(e.to_string()));
                            cx.notify();
                        }).ok();
                    }).ok();
                }
            }
        }).detach();
    }

    /// 确认创建任务
    pub fn confirm_create(&mut self, cx: &mut Context<Self>) {
        if let Some(result) = self.parsed_result.clone() {
            cx.emit(IntentParseEvent::CreateTask(result));
        }
    }

    /// 取消
    pub fn cancel(&mut self, cx: &mut Context<Self>) {
        self.parse_state = ParseState::Idle;
        self.parsed_result = None;
        self.show_preview = false;
        cx.emit(IntentParseEvent::Cancelled);
        cx.notify();
    }

    /// 获取解析结果
    pub fn parsed_result(&self) -> Option<&core::intent_parser::security::ParsedSecurityIntent> {
        self.parsed_result.as_ref()
    }

    /// 是否正在解析
    pub fn is_parsing(&self) -> bool {
        matches!(self.parse_state, ParseState::Parsing)
    }
}

impl Render for IntentParserPanel {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let parse_state = self.parse_state.clone();
        let has_result = self.parsed_result.is_some();
        let is_parsing = self.is_parsing();

        v_flex()
            .gap(px(16.0))
            .w_full()
            // 标题和说明
            .child(
                v_flex()
                    .gap(px(4.0))
                    .child(
                        Label::new("AI 意图解析")
                            .text_lg()
                            .font_weight(FontWeight::SEMIBOLD)
                    )
                    .child(
                        Label::new("用自然语言描述您想要执行的安全测试，AI 将自动解析并创建任务")
                            .text_sm()
                            .text_color(rgb(0x6b7280))
                    )
            )
            // 输入框
            .child(
                v_flex()
                    .gap(px(8.0))
                    .child(
                        Label::new("安全测试意图")
                            .text_sm()
                            .font_weight(FontWeight::MEDIUM)
                    )
                    .child(
                        Input::new(&self.input_state)
                            .h(px(80.0))
                    )
            )
            // 解析按钮
            .child(
                h_flex()
                    .gap(px(8.0))
                    .justify_between()
                    .child(
                        Button::new("parse_intent")
                            .label(if is_parsing { "解析中..." } else { "AI 解析" })
                            .primary()
                            .disabled(is_parsing)
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.start_parse(window, cx);
                            }))
                    )
                    .child(
                        Button::new("cancel")
                            .label("取消")
                            .ghost()
                            .on_click(cx.listener(|this, _, _window, cx| {
                                this.cancel(cx);
                            }))
                    )
            )
            // 状态显示
            .child(match parse_state {
                ParseState::Error(ref msg) => {
                    div()
                        .p(px(12.0))
                        .rounded_md()
                        .bg(rgb(0xfef2f2))
                        .border_1()
                        .border_color(rgb(0xfecaca))
                        .child(
                            Label::new(format!("解析失败: {}", msg))
                                .text_sm()
                                .text_color(rgb(0xdc2626))
                        )
                        .into_any_element()
                }
                ParseState::Success if has_result => {
                    self.render_preview(cx)
                }
                _ => div().into_any_element(),
            })
    }
}

impl IntentParserPanel {
    fn render_preview(&self, cx: &mut Context<Self>) -> AnyElement {
        let Some(result) = &self.parsed_result else {
            return div().into_any_element();
        };

        let intent = &result.security_intent;
        let test_type = intent.test_type;
        let icon = test_type_icon(test_type);
        let display_name = test_type_display(test_type);
        let confidence = result.confidence;
        let confidence_str = format_confidence(confidence);
        let conf_color = confidence_color(confidence.overall);

        v_flex()
            .gap(px(16.0))
            .p(px(16.0))
            .rounded_md()
            .bg(rgb(0xf9fafb))
            .border_1()
            .border_color(rgb(0xe5e7eb))
            // 预览标题
            .child(
                h_flex()
                    .gap(px(8.0))
                    .items_center()
                    .child(
                        div()
                            .text_xl()
                            .child(Label::new(icon))
                    )
                    .child(
                        Label::new("解析结果预览")
                            .text_base()
                            .font_weight(FontWeight::SEMIBOLD)
                    )
            )
            // 测试类型
            .child(
                v_flex()
                    .gap(px(4.0))
                    .child(
                        Label::new("测试类型")
                            .text_xs()
                            .text_color(rgb(0x6b7280))
                    )
                    .child(
                        Label::new(display_name)
                            .text_sm()
                            .font_weight(FontWeight::MEDIUM)
                    )
            )
            // 目标
            .child(
                v_flex()
                    .gap(px(4.0))
                    .child(
                        Label::new("目标")
                            .text_xs()
                            .text_color(rgb(0x6b7280))
                    )
                    .child(
                        Label::new(
                            intent.targets
                                .iter()
                                .map(|t| t.address.clone())
                                .collect::<Vec<_>>()
                                .join(", ")
                        )
                            .text_sm()
                    )
            )
            // 置信度
            .child(
                h_flex()
                    .gap(px(8.0))
                    .items_center()
                    .child(
                        Label::new("置信度:")
                            .text_xs()
                            .text_color(rgb(0x6b7280))
                    )
                    .child(
                        div()
                            .px(px(8.0))
                            .py(px(2.0))
                            .rounded_full()
                            .bg(rgb(conf_color))
                            .child(
                                Label::new(confidence_str)
                                    .text_xs()
                                    .text_color(rgb(0xffffff))
                            )
                    )
            )
            // 确认按钮
            .child(
                h_flex()
                    .gap(px(8.0))
                    .justify_end()
                    .mt(px(8.0))
                    .child(
                        Button::new("create_task")
                            .label("创建任务")
                            .primary()
                            .on_click(cx.listener(|this, _, _window, cx| {
                                this.confirm_create(cx);
                            }))
                    )
            )
            .into_any_element()
    }
}

impl EventEmitter<IntentParseEvent> for IntentParserPanel {}
