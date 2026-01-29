//! Traffic UI - Network traffic analysis panel with 3-column layout
//!
//! Layout:
//! - Top: Search bar with TrafficQL and capture toggle
//! - Left: Traffic capture list
//! - Middle: Request/Response inspector
//! - Right: Packet info and actions

use gpui::*;
use gpui::prelude::FluentBuilder;
use gpui_component::{h_flex, v_flex, label::Label, button::{Button, ButtonVariants}};
use data::{TrafficStore, TrafficStoreEvent, init_and_load_traffic_store};
use ui::theme::*;

mod traffic_list;
mod request_response;
mod packet_info;

pub use traffic_list::*;
pub use request_response::*;
pub use packet_info::*;

/// Main TrafficPanel with 3-column layout
pub struct TrafficPanel {
    traffic_store: Entity<TrafficStore>,
    search_query: String,
    _subscription: Subscription,
}

impl TrafficPanel {
    pub fn new(cx: &mut Context<Self>) -> Self {
        init_and_load_traffic_store(cx);
        let traffic_store = TrafficStore::global(cx);

        // Subscribe to TrafficStore events
        let _subscription = cx.subscribe(&traffic_store, |_this, _store, event: &TrafficStoreEvent, cx| {
            match event {
                TrafficStoreEvent::TrafficUpdated => {
                    cx.notify();
                }
                TrafficStoreEvent::TrafficSelected(_) => {
                    cx.notify();
                }
                TrafficStoreEvent::AnomalyDetected(_) => {
                    cx.notify();
                }
                TrafficStoreEvent::CaptureStarted => {
                    cx.notify();
                }
                TrafficStoreEvent::CaptureStopped => {
                    cx.notify();
                }
            }
        });

        Self {
            traffic_store,
            search_query: String::new(),
            _subscription,
        }
    }
}

impl Render for TrafficPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let traffic = self.traffic_store.read(cx).traffic().to_vec();
        let selected_traffic = self.traffic_store.read(cx).selected_traffic().cloned();
        let is_capturing = self.traffic_store.read(cx).is_capturing();
        let stats = self.traffic_store.read(cx).stats().cloned();

        v_flex()
            .size_full()
            .gap(SPACING_MD)
            .p(SPACING_MD)
            .bg(rgb(BG_PRIMARY))
            // Top bar with search and capture toggle
            .child(self.render_top_bar(is_capturing, cx))
            // Main content area
            .child(
                h_flex()
                    .flex_1()
                    .gap(SPACING_MD)
                    // Left column: Traffic list
                    .child(traffic_list::render_traffic_list(&traffic, &self.traffic_store))
                    // Middle column: Request/Response inspector
                    .child(request_response::render_request_response(selected_traffic.clone()))
                    // Right column: Packet info and actions
                    .child(packet_info::render_packet_info(
                        selected_traffic,
                        stats,
                        &self.traffic_store,
                    ))
            )
    }
}

impl TrafficPanel {
    /// Render the top search bar with capture toggle
    fn render_top_bar(&self, is_capturing: bool, cx: &Context<Self>) -> impl IntoElement {
        let traffic_store = self.traffic_store.clone();

        h_flex()
            .h(px(48.0))
            .px(SPACING_MD)
            .gap(SPACING_MD)
            .items_center()
            .bg(rgb(BG_CARD))
            .rounded_md()
            // Search input placeholder
            .child(
                h_flex()
                    .flex_1()
                    .px(SPACING_MD)
                    .py(SPACING_SM)
                    .bg(rgb(BG_SECONDARY))
                    .rounded_md()
                    .child(
                        Label::new("TrafficQL: protocol=http AND status=200")
                            .text_size(TEXT_SIZE_SM)
                            .text_color(rgb(TEXT_MUTED))
                    )
            )
            // Capture toggle button
            .child(
                Button::new("capture-toggle")
                    .label(if is_capturing { "Stop Capture" } else { "Start Capture" })
                    .when(is_capturing, |b| b.danger())
                    .when(!is_capturing, |b| b.primary())
                    .on_click(move |_event, _window, cx| {
                        traffic_store.update(cx, |store, cx| {
                            store.toggle_capture(cx);
                        });
                    })
            )
    }
}

/// Helper function to get protocol color
pub fn protocol_color(protocol: &str) -> u32 {
    match protocol.to_lowercase().as_str() {
        "http" | "https" => ACCENT_BLUE,
        "mavlink" => ACCENT_PURPLE,
        "rtsp" => STATUS_WARNING,
        "websocket" | "ws" | "wss" => STATUS_SUCCESS,
        "dns" => 0x8b5cf6, // Violet
        "tcp" => 0x6b7280, // Gray
        "udp" => 0x9ca3af, // Light gray
        _ => TEXT_MUTED,
    }
}

/// Helper function to format protocol name
pub fn format_protocol(protocol: &str) -> String {
    protocol.to_uppercase()
}

/// Helper function to format status code color
pub fn status_code_color(status: Option<i32>) -> u32 {
    match status {
        Some(s) if s >= 200 && s < 300 => STATUS_SUCCESS,
        Some(s) if s >= 300 && s < 400 => STATUS_WARNING,
        Some(s) if s >= 400 => STATUS_ERROR,
        _ => TEXT_MUTED,
    }
}

/// Helper function to format bytes to human readable
pub fn format_bytes(bytes: i64) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{:.2} MB", bytes as f64 / (1024.0 * 1024.0))
    }
}

/// Helper function to format duration
pub fn format_duration(ms: i32) -> String {
    if ms < 1000 {
        format!("{} ms", ms)
    } else {
        format!("{:.1} s", ms as f64 / 1000.0)
    }
}

pub fn traffic_panel(cx: &mut App) -> Entity<TrafficPanel> {
    cx.new(|cx| TrafficPanel::new(cx))
}
