//! Vulns UI - Vulnerability findings management panel with 3-column layout
//!
//! Layout:
//! - Left: Finding list grouped by severity
//! - Middle: Finding detail with AI analysis and PoC
//! - Right: CVE reference info and quick actions

use gpui::*;
use gpui_component::h_flex;
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
        let _is_loading = self.vuln_store.read(cx).is_loading();

        h_flex()
            .size_full()
            .gap(SPACING_MD)
            .p(SPACING_MD)
            .bg(rgb(BG_PRIMARY))
            // Left column: Finding list
            .child(finding_list::render_finding_list(&findings,
                &self.vuln_store,
            ))
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

pub fn vulns_panel(cx: &mut App) -> Entity<VulnsPanel> {
    cx.new(|cx| VulnsPanel::new(cx))
}
