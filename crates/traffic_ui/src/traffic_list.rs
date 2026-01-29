//! Traffic List Panel - Left column showing captured traffic entries

use gpui::*;
use gpui::prelude::FluentBuilder;
use gpui_component::{
    h_flex, v_flex,
    label::Label,
    scroll::ScrollableElement,
};
use data::{TrafficStore, Traffic};
use ui::theme::*;
use crate::{protocol_color, format_protocol, status_code_color, format_bytes, format_duration};

/// Render the left column traffic list
pub fn render_traffic_list(
    traffic: &[Traffic],
    traffic_store: &Entity<TrafficStore>,
) -> impl IntoElement {
    v_flex()
        .w(px(400.0))
        .h_full()
        .gap(SPACING_SM)
        .child(render_header(traffic.len()))
        .child(render_column_headers())
        .child(
            v_flex()
                .flex_1()
                .overflow_y_scrollbar()
                .gap(px(1.0))
                .children(traffic.iter().enumerate().map(|(idx, t)| {
                    render_traffic_row(idx, t, traffic_store)
                }))
        )
}

/// Render the list header with count
fn render_header(total_count: usize) -> impl IntoElement {
    h_flex()
        .px(SPACING_MD)
        .py(SPACING_SM)
        .justify_between()
        .items_center()
        .child(
            Label::new("Captured Traffic")
                .text_size(TEXT_SIZE_LG)
                .font_weight(FontWeight::SEMIBOLD)
        )
        .child(
            Label::new(format!("{} entries", total_count))
                .text_color(rgb(TEXT_MUTED))
                .text_size(TEXT_SIZE_SM)
        )
}

/// Render column headers
fn render_column_headers() -> impl IntoElement {
    h_flex()
        .px(SPACING_MD)
        .py(SPACING_XS)
        .bg(rgb(BG_SECONDARY))
        .child(Label::new("#").w(px(30.0)).text_size(TEXT_SIZE_XS).text_color(rgb(TEXT_MUTED)))
        .child(Label::new("Time").w(px(60.0)).text_size(TEXT_SIZE_XS).text_color(rgb(TEXT_MUTED)))
        .child(Label::new("Proto").w(px(50.0)).text_size(TEXT_SIZE_XS).text_color(rgb(TEXT_MUTED)))
        .child(Label::new("Method").w(px(50.0)).text_size(TEXT_SIZE_XS).text_color(rgb(TEXT_MUTED)))
        .child(Label::new("Path").flex_1().text_size(TEXT_SIZE_XS).text_color(rgb(TEXT_MUTED)))
        .child(Label::new("Status").w(px(50.0)).text_size(TEXT_SIZE_XS).text_color(rgb(TEXT_MUTED)))
        .child(Label::new("Size").w(px(60.0)).text_size(TEXT_SIZE_XS).text_color(rgb(TEXT_MUTED)))
        .child(Label::new("Duration").w(px(60.0)).text_size(TEXT_SIZE_XS).text_color(rgb(TEXT_MUTED)))
}

/// Render a single traffic row
fn render_traffic_row(
    idx: usize,
    traffic: &Traffic,
    traffic_store: &Entity<TrafficStore>,
) -> impl IntoElement {
    let traffic_id = traffic.id;
    let is_anomaly = traffic.is_anomaly;
    let bg_color = if idx % 2 == 0 { BG_CARD } else { BG_PRIMARY };

    let time_str = traffic.captured_at.format("%H:%M:%S").to_string();
    let protocol = format_protocol(&traffic.protocol);
    let method = traffic.method.clone().unwrap_or_default();
    let path = if traffic.path.len() > 25 {
        format!("{}...", &traffic.path[..25])
    } else {
        traffic.path.clone()
    };
    let status = traffic.response_status.map(|s| s.to_string()).unwrap_or_default();
    let size = format_bytes(traffic.size_bytes);
    let duration = format_duration(traffic.duration_ms);

    let proto_color = protocol_color(&traffic.protocol);
    let status_color = status_code_color(traffic.response_status);

    let traffic_store_clone = traffic_store.clone();

    h_flex()
        .px(SPACING_MD)
        .py(SPACING_XS)
        .gap(SPACING_XS)
        .items_center()
        .bg(rgb(bg_color))
        .when(is_anomaly, |s| {
            s.border_l(px(3.0)).border_color(rgb(STATUS_ERROR))
        })
        .cursor_pointer()
        .hover(|s| s.bg(rgb(BG_CARD_HOVER)))
        .on_mouse_down(gpui::MouseButton::Left, move |_event, _window, cx| {
            traffic_store_clone.update(cx, |store, cx| {
                store.select_traffic(traffic_id, cx);
            });
        })
        // Row number
        .child(
            Label::new(format!("{}", idx + 1))
                .w(px(30.0))
                .text_size(TEXT_SIZE_XS)
                .text_color(rgb(TEXT_MUTED))
        )
        // Time
        .child(
            Label::new(time_str)
                .w(px(60.0))
                .text_size(TEXT_SIZE_XS)
                .text_color(rgb(TEXT_SECONDARY))
        )
        // Protocol
        .child(
            h_flex()
                .w(px(50.0))
                .child(
                    Label::new(protocol)
                        .text_size(TEXT_SIZE_XS)
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(proto_color))
                )
        )
        // Method
        .child(
            Label::new(method)
                .w(px(50.0))
                .text_size(TEXT_SIZE_XS)
                .text_color(rgb(TEXT_SECONDARY))
        )
        // Path
        .child(
            Label::new(path)
                .flex_1()
                .text_size(TEXT_SIZE_XS)
                .text_color(rgb(TEXT_PRIMARY))
                .line_clamp(1)
        )
        // Status
        .child(
            h_flex()
                .w(px(50.0))
                .child(
                    Label::new(status)
                        .text_size(TEXT_SIZE_XS)
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(rgb(status_color))
                )
        )
        // Size
        .child(
            Label::new(size)
                .w(px(60.0))
                .text_size(TEXT_SIZE_XS)
                .text_color(rgb(TEXT_SECONDARY))
        )
        // Duration
        .child(
            Label::new(duration)
                .w(px(60.0))
                .text_size(TEXT_SIZE_XS)
                .text_color(rgb(TEXT_SECONDARY))
        )
        // Anomaly indicator
        .children(if is_anomaly {
            Some(
                h_flex()
                    .w(px(8.0))
                    .h(px(8.0))
                    .rounded_full()
                    .bg(rgb(STATUS_ERROR))
            )
        } else {
            None
        })
}
