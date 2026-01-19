// 添加任务表单 - 处理对话框的表单数据和输入
use gpui::*;
use gpui_component::{
    h_flex,
    input::{Input, InputState},
    label::Label,
    v_flex,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TaskPriority {
    Low,
    Medium,
    High,
    Critical,
}

impl TaskPriority {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Low => "Low",
            Self::Medium => "Medium",
            Self::High => "High",
            Self::Critical => "Critical",
        }
    }

    pub fn all() -> &'static [TaskPriority] {
        &[Self::Low, Self::Medium, Self::High, Self::Critical]
    }
}

pub struct AddTaskForm {
    pub title_input: Entity<InputState>,
    pub description_input: Entity<InputState>,
    pub selected_priority: TaskPriority,
    pub selected_agent: String,
    pub validation_error: String,
    _subscriptions: Vec<Subscription>,
}

impl AddTaskForm {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let title_input = cx.new(|cx| InputState::new(window, cx));
        let description_input = cx.new(|cx| InputState::new(window, cx));

        Self {
            title_input,
            description_input,
            selected_priority: TaskPriority::Medium,
            selected_agent: "OPENCODE".to_string(),
            validation_error: String::new(),
            _subscriptions: Vec::new(),
        }
    }

    pub fn get_title(&self, cx: &App) -> String {
        self.title_input.read(cx).value().to_string()
    }

    pub fn get_description(&self, cx: &App) -> String {
        self.description_input.read(cx).value().to_string()
    }

    pub fn get_priority(&self) -> String {
        self.selected_priority.as_str().to_string()
    }

    pub fn get_agent(&self) -> String {
        self.selected_agent.clone()
    }

    pub fn set_priority(&mut self, priority: TaskPriority, cx: &mut Context<Self>) {
        self.selected_priority = priority;
        cx.notify();
    }

    pub fn set_agent(&mut self, agent: String, cx: &mut Context<Self>) {
        self.selected_agent = agent;
        cx.notify();
    }

    pub fn validate(&mut self, cx: &App) -> bool {
        let title = self.get_title(cx);
        if title.trim().is_empty() {
            self.validation_error = "任务标题不能为空".to_string();
            return false;
        }
        self.validation_error.clear();
        true
    }

    pub fn get_validation_error(&self) -> &str {
        &self.validation_error
    }
}

impl Render for AddTaskForm {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let error = self.validation_error.clone();
        let priority = self.selected_priority;
        let agent = self.selected_agent.clone();

        v_flex()
            .gap(px(16.0))
            .w_full()
            // 验证错误提示
            .child(if !error.is_empty() {
                div()
                    .px(px(12.0))
                    .py(px(8.0))
                    .bg(rgb(0xfef2f2))
                    .border(px(1.0))
                    .border_color(rgb(0xfecaca))
                    .rounded(px(4.0))
                    .child(Label::new(error).text_sm().text_color(rgb(0xdc2626)))
                    .into_any_element()
            } else {
                div().into_any_element()
            })
            // 任务标题区域
            .child(
                v_flex()
                    .gap(px(8.0))
                    .w_full()
                    .child(
                        h_flex()
                            .gap(px(4.0))
                            .child(
                                Label::new("任务标题")
                                    .text_sm()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(rgb(0x1f2937)),
                            )
                            .child(Label::new("*").text_sm().text_color(rgb(0xef4444))),
                    )
                    .child(Input::new(&self.title_input)),
            )
            // 任务描述区域
            .child(
                v_flex()
                    .gap(px(8.0))
                    .w_full()
                    .child(
                        Label::new("任务描述（可选）")
                            .text_sm()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(0x1f2937)),
                    )
                    .child(Input::new(&self.description_input).h(px(100.0))),
            )
            // 优先级和 Agent 选择
            .child(
                v_flex()
                    .gap(px(8.0))
                    .w_full()
                    .child(
                        Label::new("配置")
                            .text_sm()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(0x1f2937)),
                    )
                    .child(
                        h_flex()
                            .gap(px(12.0))
                            .w_full()
                            // Agent 显示
                            .child(
                                v_flex()
                                    .flex_1()
                                    .gap(px(4.0))
                                    .child(Label::new("Agent").text_xs().text_color(rgb(0x6b7280)))
                                    .child(
                                        div()
                                            .px(px(8.0))
                                            .py(px(4.0))
                                            .bg(rgb(0xf3f4f6))
                                            .rounded(px(4.0))
                                            .child(
                                                Label::new(agent.clone())
                                                    .text_sm()
                                                    .text_color(rgb(0x1f2937)),
                                            ),
                                    ),
                            )
                            // 优先级显示
                            .child(
                                v_flex()
                                    .flex_1()
                                    .gap(px(4.0))
                                    .child(Label::new("优先级").text_xs().text_color(rgb(0x6b7280)))
                                    .child(
                                        div()
                                            .px(px(8.0))
                                            .py(px(4.0))
                                            .bg(rgb(0xf3f4f6))
                                            .rounded(px(4.0))
                                            .child(
                                                Label::new(priority.as_str())
                                                    .text_sm()
                                                    .text_color(rgb(0x1f2937)),
                                            ),
                                    ),
                            ),
                    ),
            )
    }
}
