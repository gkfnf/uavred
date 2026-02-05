// Add Task with Intent Parser - AI 意图解析添加任务
//
// 这个模块提供了在添加任务时使用 AI 意图解析的功能
// 使用示例：
// ```rust
// let modal = cx.new(|cx| AddTaskWithIntentModal::new(window, cx, TaskStatus::Todo));
// ```

use data::{TaskData, TaskStatus};
use gpui::*;
use gpui::prelude::FluentBuilder as _;
use gpui_component::{
    button::{Button, ButtonVariants as _},
    h_flex,
    input::{Input, InputState},
    label::Label,
    v_flex, Sizable,
};

use core::intent_parser::{
    Intent, IntentExecutor,
};

/// 可用的 AI 模型选项
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AiModelOption {
    Kimi,
    DeepSeek,
    OpenAI,
    Claude,
    Ollama,
}

impl AiModelOption {
    pub fn as_str(&self) -> &'static str {
        match self {
            AiModelOption::Kimi => "kimi",
            AiModelOption::DeepSeek => "deepseek",
            AiModelOption::OpenAI => "openai",
            AiModelOption::Claude => "claude",
            AiModelOption::Ollama => "ollama",
        }
    }
    
    pub fn display_name(&self) -> &'static str {
        match self {
            AiModelOption::Kimi => "Kimi",
            AiModelOption::DeepSeek => "DeepSeek",
            AiModelOption::OpenAI => "OpenAI",
            AiModelOption::Claude => "Claude",
            AiModelOption::Ollama => "Ollama",
        }
    }
    
    pub fn all() -> Vec<Self> {
        vec![
            AiModelOption::Kimi,
            AiModelOption::DeepSeek,
            AiModelOption::OpenAI,
            AiModelOption::Claude,
            AiModelOption::Ollama,
        ]
    }
}

/// AI 解析状态
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AiParseState {
    Idle,
    Parsing,
    Success,
    Error(String),
}

/// AddTaskWithIntentModal - 带有 AI 意图解析的添加任务对话框
pub struct AddTaskWithIntentModal {
    title_input: Entity<InputState>,
    description_input: Entity<InputState>,
    auto_start: bool,
    status: TaskStatus,
    use_ai_parse: bool,
    selected_model: AiModelOption,
    parse_state: AiParseState,
    parsed_result: Option<core::intent_parser::security::ParsedSecurityIntent>,
}

/// AddTaskWithIntentModal 事件
#[derive(Debug, Clone)]
pub enum AddTaskIntentEvent {
    TaskCreated(TaskData),
    AiTaskCreated {
        task_data: TaskData,
        parsed_intent: core::intent_parser::security::ParsedSecurityIntent,
    },
}

impl EventEmitter<AddTaskIntentEvent> for AddTaskWithIntentModal {}

impl AddTaskWithIntentModal {
    pub fn new(window: &mut Window, cx: &mut Context<Self>, status: TaskStatus) -> Self {
        let title_input = cx.new(|cx| InputState::new(window, cx));
        let description_input = cx.new(|cx| InputState::new(window, cx));

        Self {
            title_input,
            description_input,
            auto_start: false,
            status,
            use_ai_parse: false,
            selected_model: AiModelOption::Kimi,
            parse_state: AiParseState::Idle,
            parsed_result: None,
        }
    }

    pub fn get_title(&self, cx: &App) -> String {
        self.title_input.read(cx).value().to_string()
    }

    pub fn get_description(&self, cx: &App) -> String {
        self.description_input.read(cx).value().to_string()
    }

    pub fn is_auto_start(&self) -> bool {
        self.auto_start
    }

    pub fn toggle_ai_parse(&mut self, cx: &mut Context<Self>) {
        self.use_ai_parse = !self.use_ai_parse;
        cx.notify();
    }

    pub fn select_model(&mut self, model: AiModelOption, cx: &mut Context<Self>) {
        self.selected_model = model;
        cx.notify();
    }

    /// 确认创建任务
    pub fn confirm_create(&mut self, cx: &mut Context<Self>) {
        let title = self.get_title(cx);
        if title.trim().is_empty() {
            return;
        }

        if self.use_ai_parse {
            // TODO: 实际的 AI 解析
            // 这里暂时创建普通任务
            let task_data = TaskData::new(
                0,
                title,
                "task".to_string(),
                "medium".to_string(),
                self.status.as_str().to_string(),
            );
            cx.emit(AddTaskIntentEvent::TaskCreated(task_data));
        } else {
            // 创建普通任务
            let task_data = TaskData::new(
                0,
                title,
                "task".to_string(),
                "medium".to_string(),
                self.status.as_str().to_string(),
            );
            cx.emit(AddTaskIntentEvent::TaskCreated(task_data));
        }
    }
}

impl Render for AddTaskWithIntentModal {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let auto_start = self.auto_start;
        let use_ai = self.use_ai_parse;

        v_flex()
            .gap(px(16.0))
            .p(px(16.0))
            // 任务标题
            .child(
                v_flex()
                    .gap(px(8.0))
                    .child(Label::new("任务标题").text_sm().font_weight(FontWeight::SEMIBOLD))
                    .child(Input::new(&self.title_input)),
            )
            // 任务描述
            .child(
                v_flex()
                    .gap(px(8.0))
                    .child(Label::new("任务描述").text_sm().font_weight(FontWeight::SEMIBOLD))
                    .child(Input::new(&self.description_input).h(px(100.0))),
            )
            // AI 解析开关
            .child(
                h_flex()
                    .gap(px(12.0))
                    .items_center()
                    .child(
                        div()
                            .w(px(44.0))
                            .h(px(24.0))
                            .rounded_full()
                            .bg(if use_ai { rgb(0x3b82f6) } else { rgb(0xd1d5db) })
                            .cursor_pointer()
                            .relative()
                            .on_mouse_down(MouseButton::Left, cx.listener(|this: &mut Self, _, _window, cx| {
                                this.toggle_ai_parse(cx);
                            }))
                            .child(
                                div()
                                    .w(px(20.0))
                                    .h(px(20.0))
                                    .rounded_full()
                                    .bg(rgb(0xffffff))
                                    .absolute()
                                    .top(px(2.0))
                                    .left(if use_ai { px(22.0) } else { px(2.0) }),
                            ),
                    )
                    .child(Label::new("使用 AI 解析").text_sm()),
            )
            // AI 模型选择
            .when(use_ai, |this| {
                this.child(
                    v_flex()
                        .gap(px(8.0))
                        .child(Label::new("选择 AI 模型").text_sm().font_weight(FontWeight::MEDIUM))
                        .child(
                            h_flex()
                                .gap(px(8.0))
                                .children(AiModelOption::all().into_iter().map(|model| {
                                    let is_selected = self.selected_model == model;
                                    let btn = Button::new(format!("model-{}", model.as_str()))
                                        .label(model.display_name())
                                        .small();
                                    if is_selected {
                                        btn.primary()
                                    } else {
                                        btn.ghost()
                                    }
                                }))
                        )
                )
            })
            // 配置区域
            .child(
                v_flex()
                    .gap(px(8.0))
                    .child(Label::new("配置").text_sm().font_weight(FontWeight::SEMIBOLD))
                    .child(
                        h_flex()
                            .gap(px(12.0))
                            .child(
                                div()
                                    .flex_1()
                                    .border(px(1.0))
                                    .border_color(rgb(0xd1d5db))
                                    .rounded(px(6.0))
                                    .p(px(12.0))
                                    .bg(rgb(0xfafafa))
                                    .child(Label::new("Agent: OPENCODE").text_sm()),
                            )
                            .child(
                                div()
                                    .flex_1()
                                    .border(px(1.0))
                                    .border_color(rgb(0xd1d5db))
                                    .rounded(px(6.0))
                                    .p(px(12.0))
                                    .bg(rgb(0xfafafa))
                                    .child(Label::new("优先级: Medium").text_sm()),
                            ),
                    ),
            )
            // 底部按钮
            .child(
                h_flex()
                    .justify_between()
                    .items_center()
                    .child(
                        h_flex()
                            .gap(px(12.0))
                            .items_center()
                            .child(
                                div()
                                    .w(px(44.0))
                                    .h(px(24.0))
                                    .rounded_full()
                                    .bg(if auto_start { rgb(0x3b82f6) } else { rgb(0xd1d5db) })
                                    .cursor_pointer()
                                    .relative()
                                    .on_mouse_down(MouseButton::Left, cx.listener(|this: &mut Self, _, _window, cx| {
                                        this.auto_start = !this.auto_start;
                                        cx.notify();
                                    }))
                                    .child(
                                        div()
                                            .w(px(20.0))
                                            .h(px(20.0))
                                            .rounded_full()
                                            .bg(rgb(0xffffff))
                                            .absolute()
                                            .top(px(2.0))
                                            .left(if auto_start { px(22.0) } else { px(2.0) }),
                                    ),
                            )
                            .child(Label::new("立即开始").text_sm()),
                    )
                    .child(
                        h_flex()
                            .gap(px(8.0))
                            .child(Button::new("cancel").label("取消").ghost())
                            .child(
                                Button::new("create")
                                    .label(if use_ai { "AI 解析并创建" } else { "创建任务" })
                                    .primary()
                                    .on_click(cx.listener(|this, _, _window, cx| {
                                        this.confirm_create(cx);
                                    }))
                            ),
                    ),
            )
    }
}
