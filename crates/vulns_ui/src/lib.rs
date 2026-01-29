//! Vulns UI - Vulnerability findings management panel with 3-column layout
//!
//! Layout:
//! - Left: Finding list grouped by severity
//! - Middle: Finding detail with AI analysis and PoC
//! - Right: CVE reference info and quick actions

use gpui::*;
use gpui::prelude::FluentBuilder;
use gpui_component::{h_flex, v_flex, label::Label, button::Button, button::ButtonVariants};
use data::{VulnStore, VulnStoreEvent, init_and_load_vuln_store};
use ui::theme::*;

mod finding_list;
mod finding_detail;
mod cve_info;

pub use finding_list::*;
pub use finding_detail::*;
pub use cve_info::*;

/// Main VulnsPanel with 3-column layout
pub struct VulnsPanel {
    vuln_store: Entity<VulnStore>,
    _subscription: Subscription,
}

impl VulnsPanel {
    pub fn new(cx: &mut Context<Self>) -> Self {
        init_and_load_vuln_store(cx);
        let vuln_store = VulnStore::global(cx);

        // Subscribe to VulnStore events
        let _subscription = cx.subscribe(&vuln_store, |_this, _store, event: &VulnStoreEvent, cx| {
            match event {
                VulnStoreEvent::FindingsUpdated => {
                    cx.notify();
                }
                VulnStoreEvent::FindingSelected(_) => {
                    cx.notify();
                }
                VulnStoreEvent::VulnReferenceLoaded(_) => {
                    cx.notify();
                }
            }
        });

        Self {
            vuln_store,
            _subscription,
        }
    }
}

impl Render for VulnsPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let findings = self.vuln_store.read(cx).findings().to_vec();
        let selected_finding = self.vuln_store.read(cx).selected_finding().cloned();
        let vuln_reference = self.vuln_store.read(cx).selected_vuln_reference().cloned();
        let is_loading = self.vuln_store.read(cx).is_loading();
        let error = self.vuln_store.read(cx).last_error().map(|e| e.to_string());

        v_flex()
            .size_full()
            .gap(SPACING_MD)
            .p(SPACING_MD)
            .bg(rgb(BG_PRIMARY))
            // Header with title and refresh button
            .child(self.render_header(cx))
            // Error banner (if any)
            .children(error.map(|e| render_error_banner(&e, &self.vuln_store)))
            // Main content
            .child(
                h_flex()
                    .flex_1()
                    .gap(SPACING_MD)
                    // Left column: Finding list
                    .child(
                        v_flex()
                            .w(px(300.0))
                            .h_full()
                            .child(finding_list::render_finding_list(&findings, &self.vuln_store))
                            .when(is_loading, |this| {
                                this.child(render_loading_overlay("Loading vulnerabilities..."))
                            })
                            .when(!is_loading && findings.is_empty(), |this| {
                                this.child(render_empty_state(
                                    "No vulnerabilities found",
                                    "Vulnerabilities will appear here when detected by scanners or AI analysis",
                                ))
                            })
                    )
                    // Middle column: Finding detail (if selected)
                    .child(finding_detail::render_finding_detail(
                        selected_finding.clone(),
                    ))
                    // Right column: CVE info and actions
                    .child(cve_info::render_cve_info(
                        selected_finding,
                        vuln_reference,
                        &self.vuln_store,
                    ))
            )
    }
}

/// Helper function to get severity color
pub fn severity_color(severity: &data::models::Severity) -> u32 {
    use data::models::Severity::*;
    match severity {
        Critical => SEVERITY_CRITICAL,
        High => SEVERITY_HIGH,
        Medium => SEVERITY_MEDIUM,
        Low => SEVERITY_LOW,
        _ => TEXT_MUTED,
    }
}

/// Helper function to format severity as string
pub fn severity_label(severity: &data::models::Severity) -> &'static str {
    use data::models::Severity::*;
    match severity {
        Critical => "Critical",
        High => "High",
        Medium => "Medium",
        Low => "Low",
        _ => "Info",
    }
}

/// Helper function to get status color
pub fn status_color(status: &data::models::FindingStatus) -> u32 {
    use data::models::FindingStatus::*;
    match status {
        New => STATUS_AI,
        Validating => STATUS_WARNING,
        Confirmed => STATUS_ERROR,
        FalsePositive => TEXT_MUTED,
        Remediated => STATUS_SUCCESS,
        Accepted => ACCENT_BLUE,
    }
}

/// Helper function to format status as string
pub fn status_label(status: &data::models::FindingStatus) -> &'static str {
    use data::models::FindingStatus::*;
    match status {
        New => "New",
        Validating => "Validating",
        Confirmed => "Confirmed",
        FalsePositive => "False Positive",
        Remediated => "Remediated",
        Accepted => "Accepted",
    }
}

impl VulnsPanel {
    /// Render the panel header with title and refresh button
    fn render_header(&self, cx: &Context<Self>) -> impl IntoElement {
        let vuln_store = self.vuln_store.clone();
        h_flex()
            .h(px(48.0))
            .px(SPACING_MD)
            .items_center()
            .justify_between()
            .bg(rgb(BG_CARD))
            .rounded_md()
            .child(
                Label::new("Vulnerabilities")
                    .text_size(TEXT_SIZE_XL)
                    .font_weight(FontWeight::SEMIBOLD)
            )
            .child(
                Button::new("refresh")
                    .label("Refresh")
                    .on_click(move |_event, _window, cx| {
                        vuln_store.update(cx, |store, cx| {
                            if let Err(e) = store.load_findings(cx) {
                                eprintln!("Failed to reload findings: {}", e);
                            }
                        });
                    })
            )
    }
}

/// Render error banner
fn render_error_banner(error: &str, vuln_store: &Entity<VulnStore>) -> impl IntoElement {
    let vuln_store = vuln_store.clone();
    let error_msg = error.to_string();
    h_flex()
        .px(SPACING_MD)
        .py(SPACING_SM)
        .gap(SPACING_MD)
        .bg(rgb(STATUS_ERROR))
        .rounded_md()
        .child(
            Label::new(format!("Error: {}", error_msg))
                .text_color(gpui::white())
        )
        .child(
            Button::new("dismiss")
                .label("Dismiss")
                .on_click(move |_event, _window, cx| {
                    vuln_store.update(cx, |store, cx| {
                        store.clear_error(cx);
                    });
                })
        )
}

/// Render loading overlay
fn render_loading_overlay(message: &str) -> impl IntoElement {
    let msg = message.to_string();
    v_flex()
        .absolute()
        .inset_0()
        .items_center()
        .justify_center()
        .bg(rgb(BG_PRIMARY))
        .child(
            v_flex()
                .gap(SPACING_MD)
                .items_center()
                .child(
                    // Simple spinner using a rotating element
                    h_flex()
                        .w(px(32.0))
                        .h(px(32.0))
                        .rounded_full()
                        .border(px(3.0))
                        .border_color(rgb(ACCENT_BLUE))
                )
                .child(
                    Label::new(msg)
                        .text_color(rgb(TEXT_MUTED))
                        .text_size(TEXT_SIZE_SM)
                )
        )
}

/// Render empty state
fn render_empty_state(title: &str, description: &str) -> impl IntoElement {
    let title_str = title.to_string();
    let desc_str = description.to_string();
    v_flex()
        .flex_1()
        .items_center()
        .justify_center()
        .gap(SPACING_MD)
        .p(SPACING_XL)
        .child(
            Label::new(title_str)
                .text_size(TEXT_SIZE_LG)
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(rgb(TEXT_SECONDARY))
        )
        .child(
            Label::new(desc_str)
                .text_size(TEXT_SIZE_BASE)
                .text_color(rgb(TEXT_MUTED))
        )
}

pub fn vulns_panel(cx: &mut App) -> Entity<VulnsPanel> {
    cx.new(|cx| VulnsPanel::new(cx))
}
