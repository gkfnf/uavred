//! Code Block Component
//!
//! 代码块组件，用于显示 PoC 代码

use gpui::prelude::FluentBuilder as _;
use gpui::*;
use gpui_component::{h_flex, v_flex};
use ui::theme::*;

/// HTTP 方法类型
#[derive(Clone, Copy, Debug)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Patch,
}

impl HttpMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            HttpMethod::Get => "GET",
            HttpMethod::Post => "POST",
            HttpMethod::Put => "PUT",
            HttpMethod::Delete => "DELETE",
            HttpMethod::Patch => "PATCH",
        }
    }

    pub fn color(&self) -> u32 {
        match self {
            HttpMethod::Get => 0x4ade80,    // green
            HttpMethod::Post => 0xa855f7,   // purple
            HttpMethod::Put => 0x22d3ee,    // cyan
            HttpMethod::Delete => 0xef4444, // red
            HttpMethod::Patch => 0xf97316,  // orange
        }
    }
}

/// 代码块组件
#[derive(IntoElement)]
pub struct CodeBlock {
    title: Option<SharedString>,
    code: SharedString,
    language: Option<SharedString>,
    http_method: Option<HttpMethod>,
    http_path: Option<SharedString>,
    headers: Vec<(SharedString, SharedString)>,
    show_edit_button: bool,
    show_run_button: bool,
}

impl CodeBlock {
    /// 创建新的代码块
    pub fn new(code: impl Into<SharedString>) -> Self {
        Self {
            title: None,
            code: code.into(),
            language: None,
            http_method: None,
            http_path: None,
            headers: Vec::new(),
            show_edit_button: true,
            show_run_button: true,
        }
    }

    /// 设置标题
    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// 设置 HTTP 请求信息
    pub fn http_request(
        mut self,
        method: HttpMethod,
        path: impl Into<SharedString>,
    ) -> Self {
        self.http_method = Some(method);
        self.http_path = Some(path.into());
        self
    }

    /// 添加 HTTP 头
    pub fn header(
        mut self,
        key: impl Into<SharedString>,
        value: impl Into<SharedString>,
    ) -> Self {
        self.headers.push((key.into(), value.into()));
        self
    }

    /// 设置是否显示编辑按钮
    pub fn show_edit(mut self, show: bool) -> Self {
        self.show_edit_button = show;
        self
    }

    /// 设置是否显示运行按钮
    pub fn show_run(mut self, show: bool) -> Self {
        self.show_run_button = show;
        self
    }
}

impl RenderOnce for CodeBlock {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let has_headers = !self.headers.is_empty();
        let headers = self.headers;
        
        v_flex()
            .w_full()
            .rounded(BORDER_RADIUS)
            .overflow_hidden()
            // 标题栏
            .when_some(self.title, |this, title| {
                this.child(
                    h_flex()
                        .items_center()
                        .justify_between()
                        .px(PADDING_MD)
                        .py(px(10.0))
                        .rounded_t(BORDER_RADIUS)
                        .bg(rgb(0x374151))
                        // 左侧标题
                        .child(
                            h_flex()
                                .items_center()
                                .gap(SPACING_SM)
                                .child(
                                    div()
                                        .text_color(rgb(0xfbbf24))
                                        .child("⚡"),
                                )
                                .child(
                                    div()
                                        .text_sm()
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .text_color(rgb(0xf3f4f6))
                                        .child(title),
                                ),
                        )
                        // 右侧按钮
                        .child(
                            h_flex()
                                .items_center()
                                .gap(SPACING_MD)
                                .when(self.show_edit_button, |this| {
                                    this.child(
                                        h_flex()
                                            .items_center()
                                            .gap(px(4.0))
                                            .cursor_pointer()
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(rgb(0x9ca3af))
                                                    .child("✎"),
                                            )
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(rgb(0x9ca3af))
                                                    .child("Edit"),
                                            ),
                                    )
                                })
                                .when(self.show_run_button, |this| {
                                    this.child(
                                        div()
                                            .text_sm()
                                            .text_color(rgb(0x22d3ee))
                                            .cursor_pointer()
                                            .child("➤"),
                                    )
                                }),
                        ),
                )
            })
            // 代码内容
            .child(
                div()
                    .w_full()
                    .p(PADDING_MD)
                    .rounded_b(BORDER_RADIUS)
                    .bg(rgb(0x1f2937))
                    .child(
                        v_flex()
                            .gap(SPACING_SM)
                            // HTTP 方法行
                            .when_some(self.http_method, |this, method| {
                                this.child(
                                    h_flex()
                                        .gap(SPACING_SM)
                                        .child(
                                            div()
                                                .text_sm()
                                                .font_weight(FontWeight::SEMIBOLD)
                                                .text_color(rgb(method.color()))
                                                .child(method.as_str()),
                                        )
                                        .when_some(self.http_path, |this, path| {
                                            this.child(
                                                div()
                                                    .text_sm()
                                                    .text_color(rgb(0x4ade80))
                                                    .child(path),
                                            )
                                        }),
                                )
                            })
                            // Headers
                            .children(headers.into_iter().map(|(key, value)| {
                                h_flex()
                                    .gap(SPACING_SM)
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(rgb(0x22d3ee))
                                            .child(format!("{}:", key)),
                                    )
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(rgb(0xf3f4f6))
                                            .child(value),
                                    )
                            }))
                            // 分隔线
                            .when(
                                self.http_method.is_some() || has_headers,
                                |this| {
                                    this.child(div().w_full().h(px(1.0)).bg(rgb(0x4b5563)))
                                },
                            )
                            // 代码内容
                            .child(
                                div()
                                    .text_sm()
                                    .font_family("monospace")
                                    .text_color(rgb(0xf3f4f6))
                                    .line_height(px(20.0))
                                    .child(self.code),
                            ),
                    ),
            )
    }
}
