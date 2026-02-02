//! Vulns UI - Vulnerability Knowledge Base Management Panel
//!
//! Architecture:
//! - Left Panel: Vulnerability list with filter tabs (Severity/Asset/MITRE)
//! - Middle Panel: Selected vulnerability details + associated findings
//! - Right Panel: CVE database info and quick actions
//!
//! Vulnerability = User-defined knowledge base entry
//! Finding = AI Agent discovery linked to a vulnerability

use gpui::*;
use gpui::prelude::FluentBuilder;
use gpui_component::{h_flex, v_flex, label::Label, button::Button};
use data::{VulnStore, VulnStoreEvent, init_and_load_vuln_store};
use ui::theme::*;

mod vuln_list;
mod vuln_detail;
mod cve_info;

pub use vuln_list::*;
pub use vuln_detail::*;
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
                VulnStoreEvent::VulnerabilitiesUpdated => {
                    cx.notify();
                }
                VulnStoreEvent::VulnerabilitySelected(_) => {
                    cx.notify();
                }
                VulnStoreEvent::FindingsUpdated => {
                    cx.notify();
                }
                VulnStoreEvent::FindingSelected(_) => {
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
        // Use filtered vulnerabilities based on search query
        let vulnerabilities: Vec<_> = self.vuln_store.read(cx).filtered_vulnerabilities();
        let selected_vuln = self.vuln_store.read(cx).selected_vulnerability().cloned();
        let selected_vuln_id = selected_vuln.as_ref().map(|v| v.vulnerability.id.clone());
        let selected_finding = self.vuln_store.read(cx).selected_finding().cloned();
        let group_by = self.vuln_store.read(cx).group_by();
        let search_query = self.vuln_store.read(cx).search_query().to_string();
        let is_loading = self.vuln_store.read(cx).is_loading();
        let error = self.vuln_store.read(cx).last_error().map(|e| e.to_string());

        v_flex()
            .size_full()
            .gap(SPACING_MD)
            .p(SPACING_MD)
            .bg(rgb(BG_PRIMARY))
            // Error banner (if any)
            .children(error.map(|e| render_error_banner(&e, &self.vuln_store)))
            // Main content - three column layout with fixed widths
            .child(
                h_flex()
                    .flex_1()
                    .gap(SPACING_MD)
                    // Left column: Fixed 320px, no shrink
                    .child(
                        v_flex()
                            .w(px(320.0))
                            .h_full()
                            .flex_shrink_0()
                            .child(vuln_list::render_vuln_list(
                                &vulnerabilities,
                                group_by,
                                &self.vuln_store,
                                selected_vuln_id,
                                &search_query,
                                cx,
                            ))
                            .when(is_loading, |this| {
                                this.child(render_loading_overlay("Loading vulnerabilities..."))
                            })
                            .when(!is_loading && vulnerabilities.is_empty(), |this| {
                                this.child(render_empty_state(
                                    "No vulnerabilities defined",
                                    "Vulnerabilities will appear here when defined by security experts or imported from CVE databases",
                                ))
                            })
                    )
                    // Middle column: Flexible, but with min/max constraints
                    .child(
                        v_flex()
                            .flex_1()
                            .h_full()
                            .min_w(px(400.0))
                            .child(vuln_detail::render_vuln_detail(
                                selected_vuln.clone(),
                                selected_finding,
                                &self.vuln_store,
                            ))
                    )
                    // Right column: Fixed 260px, no shrink
                    .child(
                        v_flex()
                            .w(px(260.0))
                            .h_full()
                            .flex_shrink_0()
                            .child(cve_info::render_cve_info(
                                selected_vuln,
                                &self.vuln_store,
                            ))
                    )
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
                    // Simple spinner
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
