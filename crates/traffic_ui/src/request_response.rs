// Traffic 请求响应面板组件
// T1-10: Traffic 流量分析视图 - 请求响应面板

use gpui::*;
use gpui_component::{
    button::{Button, ButtonVariants as _},
    group_box::{GroupBox, GroupBoxVariants as _},
    h_flex,
    label::Label,
    v_flex, Sizable,
};

use data::TrafficEntry;
use ui::theme;

/// 请求响应面板组件
pub struct RequestResponsePanel {
    selected_traffic: Option<TrafficEntry>,
}

impl RequestResponsePanel {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self {
            selected_traffic: None,
        }
    }

    pub fn set_selected_traffic(&mut self, traffic: Option<TrafficEntry>, cx: &mut Context<Self>) {
        self.selected_traffic = traffic;
        cx.notify();
    }
}

impl Render for RequestResponsePanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .flex_1()
            .gap(theme::SPACING_MD)
            .child(render_request_panel(cx, &self.selected_traffic))
            .child(render_response_panel(cx, &self.selected_traffic))
    }
}

/// Request 面板
fn render_request_panel(
    cx: &mut Context<RequestResponsePanel>,
    selected_traffic: &Option<TrafficEntry>,
) -> impl IntoElement {
    let request_content = selected_traffic.as_ref().map(|traffic| {
        format_request_content(traffic)
    }).unwrap_or_else(|| "No request selected".to_string());

    v_flex()
        .flex_1()
        .gap(theme::SPACING_SM)
        .child(
            // 标题栏
            h_flex()
                .items_center()
                .justify_between()
                .h(px(32.0))
                .child(
                    Label::new("Request")
                        .text_base()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(theme::TEXT_PRIMARY)),
                )
                .child(
                    h_flex()
                        .gap(theme::SPACING_XS)
                        .items_center()
                        .child(
                            Button::new("edit-request")
                                .ghost()
                                .small()
                                .label("Edit")
                                .on_click(cx.listener(|_this, _, _, _| {
                                    // TODO: 实现编辑功能
                                })),
                        )
                        .child(
                            Button::new("copy-request")
                                .ghost()
                                .small()
                                .label("Copy")
                                .on_click(cx.listener(|_this, _, window, cx| {
                                    // TODO: 实现复制功能
                                    window.push_notification("Request copied to clipboard", cx);
                                })),
                        ),
                ),
        )
        .child(
            // Request 代码块
            GroupBox::new()
                .outline()
                .child(
                    div()
                        .w_full()
                        .h_full()
                        .min_h(px(200.0))
                        .p(theme::PADDING_MD)
                        .bg(rgb(theme::BG_DARK))
                        .rounded(theme::BORDER_RADIUS)
                        .child(
                            Label::new(request_content)
                                .text_sm()
                                .font_family("monospace")
                                .text_color(rgb(0xffffff))
                        ),
                ),
        )
}

/// Response 面板
fn render_response_panel(
    cx: &mut Context<RequestResponsePanel>,
    selected_traffic: &Option<TrafficEntry>,
) -> impl IntoElement {
    let response_content = selected_traffic.as_ref().map(|traffic| {
        format_response_content(traffic)
    }).unwrap_or_else(|| "No response selected".to_string());

    v_flex()
        .flex_1()
        .gap(theme::SPACING_SM)
        .child(
            // 标题栏
            h_flex()
                .items_center()
                .justify_between()
                .h(px(32.0))
                .child(
                    Label::new("Response")
                        .text_base()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(theme::TEXT_PRIMARY)),
                )
                .child(
                    Button::new("copy-response")
                        .ghost()
                        .small()
                        .label("Copy")
                        .on_click(cx.listener(|_this, _, window, cx| {
                            // TODO: 实现复制功能
                            window.push_notification("Response copied to clipboard", cx);
                        })),
                ),
        )
        .child(
            // Response 代码块
            GroupBox::new()
                .outline()
                .child(
                    div()
                        .w_full()
                        .h_full()
                        .min_h(px(200.0))
                        .p(theme::PADDING_MD)
                        .bg(rgb(theme::BG_CARD))
                        .rounded(theme::BORDER_RADIUS)
                        .border(px(1.0))
                        .border_color(rgb(theme::BORDER_COLOR))
                        .child(
                            Label::new(response_content)
                                .text_sm()
                                .font_family("monospace")
                                .text_color(rgb(theme::TEXT_PRIMARY))
                        ),
                ),
        )
}

/// 格式化请求内容
fn format_request_content(traffic: &TrafficEntry) -> String {
    let mut content = String::new();

    // 请求行
    if let Some(method) = traffic.method {
        let path = if let Some(ref query) = traffic.query {
            format!("{}?{}", traffic.path, query)
        } else {
            traffic.path.clone()
        };
        content.push_str(&format!("{} {} HTTP/1.1\n", method, path));
    } else {
        content.push_str(&format!("{} HTTP/1.1\n", traffic.path));
    }

    // Host 头
    content.push_str(&format!("Host: {}:{}\n", traffic.host, traffic.port));

    // 其他请求头
    for (key, value) in &traffic.request_headers {
        content.push_str(&format!("{}: {}\n", key, value));
    }

    // 空行
    content.push('\n');

    // 请求体
    if let Some(ref body) = traffic.request_body {
        content.push_str(body);
    } else if let Some(ref raw) = traffic.request_raw {
        content.push_str(raw);
    }

    content
}

/// 格式化响应内容
fn format_response_content(traffic: &TrafficEntry) -> String {
    let mut content = String::new();

    // 状态行
    let status_text = traffic.status_text.as_ref()
        .map(|s| s.as_str())
        .unwrap_or_else(|| {
            match traffic.status {
                200 => "OK",
                404 => "Not Found",
                500 => "Internal Server Error",
                _ => "Unknown",
            }
        });
    content.push_str(&format!("HTTP/1.1 {} {}\n", traffic.status, status_text));

    // 响应头
    for (key, value) in &traffic.response_headers {
        content.push_str(&format!("{}: {}\n", key, value));
    }

    // 空行
    content.push('\n');

    // 响应体
    if let Some(ref body) = traffic.response_body {
        content.push_str(body);
    } else if let Some(ref raw) = traffic.response_raw {
        content.push_str(raw);
    }

    content
}
