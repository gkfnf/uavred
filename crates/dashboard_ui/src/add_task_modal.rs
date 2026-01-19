// 添加任务 Modal - 使用 GPUI 的对话框系统
use data::{TaskData, TaskStatus};
use gpui::*;
use gpui_component::{
    button::{Button, ButtonVariants as _},
    h_flex,
    input::{Input, InputState},
    label::Label,
    v_flex, IconName, Sizable, WindowExt,
};

/// AddTaskModal 事件
#[derive(Debug, Clone)]
pub enum AddTaskModalEvent {
    TaskCreated(TaskData),
}

impl EventEmitter<AddTaskModalEvent> for AddTaskModal {}

/// 添加任务对话框状态
pub struct AddTaskModal {
    title_input: Entity<InputState>,
    description_input: Entity<InputState>,
    auto_start: bool,
    status: TaskStatus,
}

impl AddTaskModal {
    pub fn new(window: &mut Window, cx: &mut Context<Self>, status: TaskStatus) -> Self {
        let title_input = cx.new(|cx| InputState::new(window, cx));
        let description_input = cx.new(|cx| InputState::new(window, cx));

        Self {
            title_input,
            description_input,
            auto_start: false,
            status,
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
}

impl Render for AddTaskModal {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let auto_start = self.auto_start;

        v_flex()
            .gap(px(16.0))
            .p(px(16.0))
            // 任务标题输入框
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
            // 任务描述输入框
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
            // 配置部分（占位符）
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
                            .child(
                                div()
                                    .flex_1()
                                    .border(px(1.0))
                                    .border_color(rgb(0xd1d5db))
                                    .rounded(px(6.0))
                                    .p(px(12.0))
                                    .bg(rgb(0xfafafa))
                                    .flex()
                                    .items_center()
                                    .justify_between()
                                    .child(Label::new("Agent").text_sm().text_color(rgb(0x1f2937)))
                                    .child(
                                        Label::new("OPENCODE").text_xs().text_color(rgb(0x6b7280)),
                                    ),
                            )
                            .child(
                                div()
                                    .flex_1()
                                    .border(px(1.0))
                                    .border_color(rgb(0xd1d5db))
                                    .rounded(px(6.0))
                                    .p(px(12.0))
                                    .bg(rgb(0xfafafa))
                                    .flex()
                                    .items_center()
                                    .justify_between()
                                    .child(Label::new("优先级").text_sm().text_color(rgb(0x1f2937)))
                                    .child(
                                        Label::new("Medium").text_xs().text_color(rgb(0x6b7280)),
                                    ),
                            )
                            .child(
                                div()
                                    .flex_1()
                                    .border(px(1.0))
                                    .border_color(rgb(0xd1d5db))
                                    .rounded(px(6.0))
                                    .p(px(12.0))
                                    .bg(rgb(0xfafafa))
                                    .flex()
                                    .items_center()
                                    .justify_between()
                                    .child(Label::new("分支").text_sm().text_color(rgb(0x1f2937)))
                                    .child(
                                        Label::new("master").text_xs().text_color(rgb(0x6b7280)),
                                    ),
                            ),
                    ),
            )
            // 分割线
            .child(div().w_full().h(px(1.0)).bg(rgb(0xe5e7eb)))
            // 底部：开始开关 + 创建/取消按钮
            .child(
                h_flex()
                    .justify_between()
                    .items_center()
                    .w_full()
                    .px(px(0.0))
                    .child(
                        h_flex()
                            .gap(px(12.0))
                            .items_center()
                            // 开始开关（toggle）
                            .child(
                                div()
                                    .w(px(44.0))
                                    .h(px(24.0))
                                    .rounded_full()
                                    .bg(if auto_start {
                                        rgb(0x3b82f6)
                                    } else {
                                        rgb(0xd1d5db)
                                    })
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
                            .child(Label::new("立即开始").text_sm().text_color(rgb(0x1f2937))),
                    ),
            )
    }
}
