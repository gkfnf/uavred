// TrafficQL 查询栏组件
// T1-8: Traffic 流量分析视图 - TrafficQL 查询栏

use gpui::*;
use gpui_component::{
    button::{Button, ButtonVariants as _},
    h_flex,
    input::InputState,
    label::Label,
    IconName, Sizable,
};
use ui::theme::*;

/// TrafficQL 查询栏组件
pub struct QueryBar {
    query_input: Option<Entity<InputState>>,
    is_capturing: bool,
    is_intercepting: bool,
    query_text: String,
}

impl QueryBar {
    pub fn new() -> Self {
        Self {
            query_input: None,
            is_capturing: false,
            is_intercepting: false,
            query_text: String::new(),
        }
    }

    pub fn init_input(&mut self, window: &mut Window, cx: &mut App) {
        if self.query_input.is_none() {
            self.query_input = Some(cx.new(|cx| InputState::new(window, cx)));
        }
    }

    pub fn toggle_capturing(&mut self, cx: &mut App) {
        self.is_capturing = !self.is_capturing;
        cx.notify();
    }

    pub fn toggle_intercepting(&mut self, cx: &mut App) {
        self.is_intercepting = !self.is_intercepting;
        cx.notify();
    }

    pub fn is_capturing(&self) -> bool {
        self.is_capturing
    }

    pub fn is_intercepting(&self) -> bool {
        self.is_intercepting
    }

    pub fn get_query(&self) -> &str {
        &self.query_text
    }
}

impl Render for QueryBar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // 初始化输入框
        self.init_input(window, cx);

        let query_input = self.query_input.as_ref().unwrap();
        let is_capturing = self.is_capturing;
        let is_intercepting = self.is_intercepting;

        h_flex()
            .w_full()
            .h(MIN_INPUT_HEIGHT + PADDING_SM * 2.0)
            .px(PADDING_MD)
            .py(PADDING_SM)
            .gap(SPACING_SM)
            .items_center()
            .bg(rgb(BG_CARD))
            .border_b(px(1.0))
            .border_color(rgb(BORDER_COLOR))
            .child(
                // TrafficQL 输入框
                h_flex()
                    .flex_1()
                    .h(MIN_INPUT_HEIGHT)
                    .px(PADDING_MD)
                    .items_center()
                    .bg(rgb(BG_SECONDARY))
                    .rounded(BORDER_RADIUS)
                    .border(px(1.0))
                    .border_color(rgb(BORDER_COLOR))
                    .child(
                        gpui_component::input::Input::new(query_input)
                            .placeholder("TrafficQL: method=GET status=200")
                            .w_full()
                            .h(MIN_INPUT_HEIGHT),
                    ),
            )
            .child(
                // Capturing 切换按钮
                Button::new("capturing-toggle")
                    .ghost()
                    .small()
                    .selected(is_capturing)
                    .icon(IconName::Circle)
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.toggle_capturing(cx);
                    })),
            )
            .child(
                // Intercept 切换按钮
                Button::new("intercept-toggle")
                    .ghost()
                    .small()
                    .selected(is_intercepting)
                    .icon(IconName::Shield)
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.toggle_intercepting(cx);
                    })),
            )
            .child(
                // 过滤图标按钮
                Button::new("filter-button")
                    .ghost()
                    .small()
                    .icon(IconName::Filter)
                    .on_click(cx.listener(|this, _, _, cx| {
                        // TODO: 打开过滤对话框
                    })),
            )
    }
}
