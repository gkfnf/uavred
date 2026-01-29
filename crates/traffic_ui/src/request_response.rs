//! Request/Response Inspector - Middle section showing request and response details

use gpui::*;
use gpui::prelude::FluentBuilder;
use gpui_component::{
    h_flex, v_flex,
    label::Label,
};
use data::Traffic;
use ui::theme::*;
use crate::format_bytes;

/// Render the middle section request/response inspector
pub fn render_request_response(
    traffic: Option<Traffic>,
) -> impl IntoElement {
    v_flex()
        .flex_1()
        .h_full()
        .gap(SPACING_MD)
        .child(
            Label::new("Request / Response")
                .text_size(TEXT_SIZE_LG)
                .font_weight(FontWeight::SEMIBOLD)
        )
        .child(
            v_flex()
                .flex_1()
                .when(traffic.is_none(), |this| {
                    this.items_center()
                        .justify_center()
                        .child(
                            Label::new("Select a traffic entry to view details")
                                .text_color(rgb(TEXT_MUTED))
                        )
                })
                .when_some(traffic, |this, t| {
                    this.child(render_traffic_inspector(&t))
                })
        )
}

/// Render the traffic inspector with request and response tabs
fn render_traffic_inspector(traffic: &Traffic) -> impl IntoElement {
    v_flex()
        .flex_1()
        .gap(SPACING_MD)
        // Request section
        .child(render_request_section(traffic))
        // Response section
        .child(render_response_section(traffic))
}

/// Render the request section
fn render_request_section(traffic: &Traffic) -> impl IntoElement {
    let method = traffic.method.clone().unwrap_or_else(|| "GET".to_string());
    let path = traffic.path.clone();

    v_flex()
        .gap(SPACING_SM)
        .child(
            h_flex()
                .gap(SPACING_SM)
                .items_center()
                .child(
                    Label::new("Request")
                        .text_size(TEXT_SIZE_BASE)
                        .font_weight(FontWeight::SEMIBOLD)
                )
                .child(
                    h_flex()
                        .px(SPACING_SM)
                        .py(SPACING_XS)
                        .rounded_md()
                        .bg(rgb(ACCENT_BLUE))
                        .child(
                            Label::new(method)
                                .text_size(TEXT_SIZE_SM)
                                .text_color(gpui::white())
                        )
                )
                .child(
                    Label::new(path)
                        .text_size(TEXT_SIZE_SM)
                        .text_color(rgb(TEXT_SECONDARY))
                )
        )
        .child(
            v_flex()
                .p(SPACING_MD)
                .rounded_md()
                .bg(rgb(BG_CARD))
                .gap(SPACING_SM)
                .child(
                    Label::new("Headers")
                        .text_size(TEXT_SIZE_SM)
                        .font_weight(FontWeight::MEDIUM)
                )
                .child(
                    render_headers(&traffic.request_headers)
                )
                .children(traffic.request_body.as_ref().map(|body| {
                    v_flex()
                        .mt(SPACING_SM)
                        .gap(SPACING_SM)
                        .child(
                            Label::new(format!("Body ({})", format_bytes(body.len() as i64)))
                                .text_size(TEXT_SIZE_SM)
                                .font_weight(FontWeight::MEDIUM)
                        )
                        .child(
                            v_flex()
                                .p(SPACING_SM)
                                .rounded_md()
                                .bg(rgb(BG_DARK))
                                .child(
                                    Label::new(String::from_utf8_lossy(body).to_string())
                                        .text_size(TEXT_SIZE_SM)
                                        .text_color(rgb(0x10b981)) // Green code color
                                )
                        )
                }))
        )
}

/// Render the response section
fn render_response_section(traffic: &Traffic) -> impl IntoElement {
    let status = traffic.response_status.unwrap_or(0);
    let status_color = crate::status_code_color(traffic.response_status);

    v_flex()
        .gap(SPACING_SM)
        .child(
            h_flex()
                .gap(SPACING_SM)
                .items_center()
                .child(
                    Label::new("Response")
                        .text_size(TEXT_SIZE_BASE)
                        .font_weight(FontWeight::SEMIBOLD)
                )
                .child(
                    h_flex()
                        .px(SPACING_SM)
                        .py(SPACING_XS)
                        .rounded_md()
                        .bg(rgb(status_color))
                        .child(
                            Label::new(format!("{}", status))
                                .text_size(TEXT_SIZE_SM)
                                .text_color(gpui::white())
                        )
                )
                .child(
                    Label::new(format!("{} bytes", format_bytes(traffic.size_bytes)))
                        .text_size(TEXT_SIZE_SM)
                        .text_color(rgb(TEXT_SECONDARY))
                )
                .child(
                    Label::new(format!("{} ms", traffic.duration_ms))
                        .text_size(TEXT_SIZE_SM)
                        .text_color(rgb(TEXT_SECONDARY))
                )
        )
        .child(
            v_flex()
                .p(SPACING_MD)
                .rounded_md()
                .bg(rgb(BG_CARD))
                .gap(SPACING_SM)
                .child(
                    Label::new("Headers")
                        .text_size(TEXT_SIZE_SM)
                        .font_weight(FontWeight::MEDIUM)
                )
                .child(
                    render_headers(&traffic.response_headers)
                )
                .children(traffic.response_body.as_ref().map(|body| {
                    v_flex()
                        .mt(SPACING_SM)
                        .gap(SPACING_SM)
                        .child(
                            Label::new(format!("Body ({})", format_bytes(body.len() as i64)))
                                .text_size(TEXT_SIZE_SM)
                                .font_weight(FontWeight::MEDIUM)
                        )
                        .child(
                            v_flex()
                                .p(SPACING_SM)
                                .rounded_md()
                                .bg(rgb(BG_DARK))
                                .child(
                                    Label::new(String::from_utf8_lossy(body).to_string())
                                        .text_size(TEXT_SIZE_SM)
                                        .text_color(rgb(0x10b981)) // Green code color
                                )
                        )
                }))
        )
}

/// Render headers as key-value pairs
fn render_headers(headers: &str) -> impl IntoElement {
    let lines: Vec<String> = headers.lines().map(|l| l.to_string()).collect();
    v_flex()
        .gap(px(2.0))
        .children(lines.into_iter().map(|line| {
            if let Some((key, value)) = line.split_once(':') {
                h_flex()
                    .gap(SPACING_XS)
                    .child(
                        Label::new(format!("{}:", key.trim()))
                            .text_size(TEXT_SIZE_SM)
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgb(TEXT_SECONDARY))
                    )
                    .child(
                        Label::new(value.trim().to_string())
                            .text_size(TEXT_SIZE_SM)
                            .text_color(rgb(TEXT_PRIMARY))
                    )
            } else {
                h_flex()
                    .child(
                        Label::new(line)
                            .text_size(TEXT_SIZE_SM)
                            .text_color(rgb(TEXT_SECONDARY))
                    )
            }
        }))
}
