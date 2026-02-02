//! CVE Info Panel - Right column showing CVE database info and quick actions

use gpui::*;
use gpui::prelude::FluentBuilder;
use gpui_component::{
    h_flex, v_flex,
    label::Label,
    button::Button,
};
use data::{VulnStore, VulnerabilityWithFindings};
use ui::theme::*;


/// Render the right column CVE info and actions
pub fn render_cve_info(
    vuln: Option<VulnerabilityWithFindings>,
    vuln_store: &Entity<VulnStore>,
) -> impl IntoElement {
    let has_vuln = vuln.is_some();
    
    v_flex()
        .w(px(300.0))
        .h_full()
        .gap(SPACING_MD)
        .when(has_vuln, |this| {
            let vuln = vuln.unwrap();
            this.child(render_cve_details(&vuln))
                .child(render_quick_actions(&vuln, vuln_store))
                .child(render_detection_info(&vuln))
        })
        .when(!has_vuln, |this| {
            this.flex_1()
                .items_center()
                .justify_center()
                .child(
                    v_flex()
                        .gap(SPACING_MD)
                        .items_center()
                        .child(
                            Label::new("No selection")
                                .text_size(TEXT_SIZE_BASE)
                                .text_color(rgb(TEXT_MUTED))
                        )
                )
        })
}

/// Render CVE database details
fn render_cve_details(vuln: &VulnerabilityWithFindings) -> impl IntoElement {
    v_flex()
        .gap(SPACING_MD)
        .p(SPACING_MD)
        .bg(rgb(BG_CARD))
        .rounded_md()
        .child(
            Label::new("CVE Database")
                .text_size(TEXT_SIZE_LG)
                .font_weight(FontWeight::SEMIBOLD)
        )
        .child(
            v_flex()
                .gap(SPACING_SM)
                .child(render_info_row("CVE ID", &vuln.vulnerability.cve_id))
                .child(render_info_row("Internal ID", &vuln.vulnerability.id))
                .children(vuln.vulnerability.cvss_score.map(|score| {
                    render_info_row("CVSS Score", &format!("{:.1}", score))
                }))
                .children(if vuln.vulnerability.cvss_vector.is_empty() {
                    None
                } else {
                    Some(render_info_row("CVSS Vector", &vuln.vulnerability.cvss_vector))
                })
                .children(if vuln.vulnerability.cwe_id.is_empty() {
                    None
                } else {
                    Some(render_info_row("CWE ID", &vuln.vulnerability.cwe_id))
                })
                .children(if vuln.vulnerability.vuln_type.is_empty() {
                    None
                } else {
                    Some(render_info_row("Type", &vuln.vulnerability.vuln_type))
                })
        )
}

/// Render quick action buttons
fn render_quick_actions(
    vuln: &VulnerabilityWithFindings,
    _vuln_store: &Entity<VulnStore>,
) -> impl IntoElement {
    let vuln_id_1 = vuln.vulnerability.id.clone();
    let vuln_id_2 = vuln.vulnerability.id.clone();
    let vuln_id_3 = vuln.vulnerability.id.clone();
    let vuln_id_4 = vuln.vulnerability.id.clone();
    
    v_flex()
        .gap(SPACING_MD)
        .p(SPACING_MD)
        .bg(rgb(BG_CARD))
        .rounded_md()
        .child(
            Label::new("Quick Actions")
                .text_size(TEXT_SIZE_LG)
                .font_weight(FontWeight::SEMIBOLD)
        )
        .child(
            v_flex()
                .gap(SPACING_SM)
                .child(
                    h_flex()
                        .child(Button::new("test_traffic")
                            .label("Test in Traffic")
                            .on_click(move |_event, _window, _cx| {
                                println!("Test in Traffic clicked for vuln: {}", vuln_id_1);
                            }))
                )
                .child(
                    h_flex()
                        .child(Button::new("fuzz_test")
                            .label("FUZZ Test")
                            .on_click(move |_event, _window, _cx| {
                                println!("FUZZ Test clicked for vuln: {}", vuln_id_2);
                            }))
                )
                .child(
                    h_flex()
                        .child(Button::new("view_poc")
                            .label("View PoC")
                            .on_click(move |_event, _window, _cx| {
                                println!("View PoC clicked for vuln: {}", vuln_id_3);
                            }))
                )
                .child(
                    h_flex()
                        .child(Button::new("export_report")
                            .label("Export Report")
                            .on_click(move |_event, _window, _cx| {
                                println!("Export Report clicked for vuln: {}", vuln_id_4);
                            }))
                )
        )
}

/// Render detection/remediation info
fn render_detection_info(vuln: &VulnerabilityWithFindings) -> impl IntoElement {
    v_flex()
        .gap(SPACING_MD)
        .p(SPACING_MD)
        .bg(rgb(BG_CARD))
        .rounded_md()
        .child(
            Label::new("Detection & Remediation")
                .text_size(TEXT_SIZE_LG)
                .font_weight(FontWeight::SEMIBOLD)
        )
        .child(
            v_flex()
                .gap(SPACING_SM)
                .child(render_info_row("Total Findings", &vuln.finding_count().to_string()))
                .child(render_info_row("Confirmed", &vuln.confirmed_count().to_string()))
                .child(render_info_row("Affected Assets", &vuln.affected_assets.len().to_string()))
                .children(if vuln.vulnerability.exploit_available {
                    Some(render_info_row("Exploit", "Available"))
                } else {
                    None
                })
                .children(if vuln.vulnerability.exploit_complexity.is_empty() {
                    None
                } else {
                    Some(render_info_row("Complexity", &vuln.vulnerability.exploit_complexity))
                })
        )
        .children(if !vuln.vulnerability.solution.is_empty() {
            Some(
                v_flex()
                    .mt(SPACING_SM)
                    .gap(SPACING_XS)
                    .child(
                        Label::new("Solution")
                            .text_size(TEXT_SIZE_SM)
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgb(TEXT_SECONDARY))
                    )
                    .child(
                        Label::new(&vuln.vulnerability.solution)
                            .text_size(TEXT_SIZE_SM)
                            .text_color(rgb(TEXT_PRIMARY))
                    )
            )
        } else {
            None
        })
}

/// Render an info row with label and value
fn render_info_row(label: &str, value: &str) -> impl IntoElement {
    let label = label.to_string();
    let value = value.to_string();
    h_flex()
        .justify_between()
        .child(
            Label::new(label)
                .text_size(TEXT_SIZE_SM)
                .text_color(rgb(TEXT_MUTED))
        )
        .child(
            Label::new(value)
                .text_size(TEXT_SIZE_SM)
                .font_weight(FontWeight::MEDIUM)
        )
}
