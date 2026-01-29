//! Packet Info Panel - Right column showing packet details and actions

use gpui::*;
use gpui::prelude::FluentBuilder;
use gpui_component::{
    h_flex, v_flex,
    label::Label,
    button::{Button, ButtonVariants},
};
use data::{TrafficStore, Traffic};
use data::traffic_store::TrafficStats;
use ui::theme::*;

/// Render the right column packet info panel
pub fn render_packet_info(
    traffic: Option<Traffic>,
    stats: Option<TrafficStats>,
    traffic_store: &Entity<TrafficStore>,
) -> impl IntoElement {
    v_flex()
        .w(px(280.0))
        .h_full()
        .gap(SPACING_MD)
        .child(
            Label::new("Packet Info")
                .text_size(TEXT_SIZE_LG)
                .font_weight(FontWeight::SEMIBOLD)
        )
        // Packet details (if selected)
        .child(
            v_flex()
                .flex_1()
                .when(traffic.is_none(), |this| {
                    this.items_center()
                        .justify_center()
                        .child(
                            Label::new("Select a packet to view details")
                                .text_color(rgb(TEXT_MUTED))
                        )
                })
                .when_some(traffic.clone(), |this, t| {
                    this.child(render_packet_details(&t))
                })
        )
        // Statistics section
        .child(
            render_statistics(stats)
        )
        // Quick actions (if traffic selected)
        .children(traffic.map(|t| {
            render_quick_actions(&t, traffic_store)
        }))
}

/// Render packet details
fn render_packet_details(traffic: &Traffic) -> impl IntoElement {
    let captured_at = traffic.captured_at.format("%Y-%m-%d %H:%M:%S").to_string();

    v_flex()
        .gap(SPACING_LG)
        // Packet Info section
        .child(
            v_flex()
                .p(SPACING_MD)
                .rounded_md()
                .bg(rgb(BG_CARD))
                .gap(SPACING_MD)
                .child(
                    Label::new("Packet Details")
                        .text_size(TEXT_SIZE_BASE)
                        .font_weight(FontWeight::SEMIBOLD)
                )
                .child(
                    v_flex()
                        .gap(SPACING_XS)
                        .child(
                            Label::new("ID")
                                .text_size(TEXT_SIZE_SM)
                                .text_color(rgb(TEXT_MUTED))
                        )
                        .child(
                            Label::new(format!("{}", traffic.id))
                                .text_size(TEXT_SIZE_BASE)
                        )
                )
                .child(
                    v_flex()
                        .gap(SPACING_XS)
                        .child(
                            Label::new("Size")
                                .text_size(TEXT_SIZE_SM)
                                .text_color(rgb(TEXT_MUTED))
                        )
                        .child(
                            Label::new(crate::format_bytes(traffic.size_bytes))
                                .text_size(TEXT_SIZE_BASE)
                        )
                )
                .child(
                    v_flex()
                        .gap(SPACING_XS)
                        .child(
                            Label::new("Captured At")
                                .text_size(TEXT_SIZE_SM)
                                .text_color(rgb(TEXT_MUTED))
                        )
                        .child(
                            Label::new(captured_at)
                                .text_size(TEXT_SIZE_BASE)
                        )
                )
        )
        // Source/Destination
        .child(
            v_flex()
                .p(SPACING_MD)
                .rounded_md()
                .bg(rgb(BG_CARD))
                .gap(SPACING_MD)
                .child(
                    Label::new("Endpoints")
                        .text_size(TEXT_SIZE_BASE)
                        .font_weight(FontWeight::SEMIBOLD)
                )
                .child(
                    v_flex()
                        .gap(SPACING_XS)
                        .child(
                            Label::new("Source")
                                .text_size(TEXT_SIZE_SM)
                                .text_color(rgb(TEXT_MUTED))
                        )
                        .child(
                            Label::new(format!("{}:{}",
                                traffic.src_ip,
                                traffic.src_port.map(|p| p.to_string()).unwrap_or_default()
                            ))
                                .text_size(TEXT_SIZE_SM)
                        )
                )
                .child(
                    v_flex()
                        .gap(SPACING_XS)
                        .child(
                            Label::new("Destination")
                                .text_size(TEXT_SIZE_SM)
                                .text_color(rgb(TEXT_MUTED))
                        )
                        .child(
                            Label::new(format!("{}:{}",
                                traffic.dst_ip,
                                traffic.dst_port.map(|p| p.to_string()).unwrap_or_default()
                            ))
                                .text_size(TEXT_SIZE_SM)
                        )
                )
        )
        // Anomaly Detection
        .child(render_anomaly_info(traffic))
}

/// Render anomaly detection info
fn render_anomaly_info(traffic: &Traffic) -> impl IntoElement {
    if !traffic.is_anomaly {
        return v_flex().into_any_element();
    }

    v_flex()
        .p(SPACING_MD)
        .rounded_md()
        .bg(rgb(STATUS_ERROR))
        .gap(SPACING_MD)
        .child(
            Label::new("Anomaly Detected")
                .text_size(TEXT_SIZE_BASE)
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(gpui::white())
        )
        .child(
            v_flex()
                .gap(SPACING_XS)
                .child(
                    Label::new("Type")
                        .text_size(TEXT_SIZE_SM)
                        .text_color(gpui::white())
                )
                .child(
                    Label::new(traffic.anomaly_type.clone())
                        .text_size(TEXT_SIZE_BASE)
                        .text_color(gpui::white())
                )
        )
        .child(
            v_flex()
                .gap(SPACING_XS)
                .child(
                    Label::new("Score")
                        .text_size(TEXT_SIZE_SM)
                        .text_color(gpui::white())
                )
                .child(
                    Label::new(format!("{:.1}%", traffic.anomaly_score * 100.0))
                        .text_size(TEXT_SIZE_BASE)
                        .text_color(gpui::white())
                )
        )
        .into_any_element()
}

/// Render statistics section
fn render_statistics(stats: Option<TrafficStats>) -> impl IntoElement {
    let stats = match stats {
        Some(s) => s,
        None => {
            return v_flex()
                .p(SPACING_MD)
                .rounded_md()
                .bg(rgb(BG_CARD))
                .child(Label::new("No statistics available").text_color(rgb(TEXT_MUTED)))
                .into_any_element();
        }
    };

    let success_rate = stats.success_rate;

    v_flex()
        .p(SPACING_MD)
        .rounded_md()
        .bg(rgb(BG_CARD))
        .gap(SPACING_MD)
        .child(
            Label::new("Statistics")
                .text_size(TEXT_SIZE_BASE)
                .font_weight(FontWeight::SEMIBOLD)
        )
        .child(
            v_flex()
                .gap(SPACING_SM)
                .child(
                    h_flex()
                        .justify_between()
                        .child(Label::new("Total").text_size(TEXT_SIZE_SM))
                        .child(Label::new(format!("{}", stats.total_requests)).text_size(TEXT_SIZE_SM))
                )
                .child(
                    h_flex()
                        .justify_between()
                        .child(Label::new("Anomalies").text_size(TEXT_SIZE_SM))
                        .child(
                            Label::new(format!("{}", stats.anomalies))
                                .text_size(TEXT_SIZE_SM)
                                .text_color(rgb(if stats.anomalies > 0 { STATUS_ERROR } else { TEXT_SECONDARY }))
                        )
                )
                .child(
                    h_flex()
                        .justify_between()
                        .child(Label::new("Success Rate").text_size(TEXT_SIZE_SM))
                        .child(
                            Label::new(format!("{:.1}%", success_rate))
                                .text_size(TEXT_SIZE_SM)
                                .text_color(rgb(if success_rate >= 95.0 { STATUS_SUCCESS } else { STATUS_WARNING }))
                        )
                )
                .child(
                    h_flex()
                        .justify_between()
                        .child(Label::new("Avg Time").text_size(TEXT_SIZE_SM))
                        .child(Label::new(format!("{} ms", stats.avg_duration_ms)).text_size(TEXT_SIZE_SM))
                )
        )
        // Protocol breakdown
        .child(
            v_flex()
                .mt(SPACING_SM)
                .gap(SPACING_XS)
                .child(
                    Label::new("Protocols")
                        .text_size(TEXT_SIZE_SM)
                        .text_color(rgb(TEXT_MUTED))
                )
                .children(stats.by_protocol.iter().map(|(proto, count)| {
                    h_flex()
                        .justify_between()
                        .child(Label::new(proto.to_uppercase()).text_size(TEXT_SIZE_SM))
                        .child(Label::new(format!("{}", count)).text_size(TEXT_SIZE_SM))
                }))
        )
        .into_any_element()
}

/// Render quick action buttons
fn render_quick_actions(
    traffic: &Traffic,
    traffic_store: &Entity<TrafficStore>,
) -> impl IntoElement {
    let traffic_id = traffic.id;
    let traffic_store_clone = traffic_store.clone();

    v_flex()
        .mt(SPACING_MD)
        .gap(SPACING_SM)
        .child(
            Label::new("Actions")
                .text_size(TEXT_SIZE_BASE)
                .font_weight(FontWeight::SEMIBOLD)
        )
        .child(
            Button::new("replay")
                .label("Replay Request")
                .primary()
                .on_click(move |_event, _window, _cx| {
                    println!("Replay request for traffic {}", traffic_id);
                })
        )
        .child(
            Button::new("fuzz")
                .label("Fuzz Test")
                .on_click(move |_event, _window, _cx| {
                    println!("Fuzz test for traffic {}", traffic_id);
                })
        )
        .child(
            Button::new("export-curl")
                .label("Export as cURL")
                .on_click(move |_event, _window, cx| {
                    traffic_store_clone.update(cx, |store, _cx| {
                        if let Some(t) = store.selected_traffic() {
                            let curl = store.format_as_curl(t);
                            println!("cURL: {}", curl);
                        }
                    });
                })
        )
}
