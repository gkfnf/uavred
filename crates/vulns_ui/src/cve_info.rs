//! CVE Info Panel - Right column showing CVE reference info and quick actions

use gpui::*;
use gpui::prelude::FluentBuilder;
use gpui_component::{
    h_flex, v_flex,
    label::Label,
    button::{Button, ButtonVariants},
};
use data::{VulnStore, models::{Finding, Vulnerability}};
use ui::theme::*;
use crate::status_color;

/// Render the right column CVE info panel
pub fn render_cve_info(
    finding: Option<Finding>,
    vuln_reference: Option<Vulnerability>,
    vuln_store: &Entity<VulnStore>,
) -> impl IntoElement {
    v_flex()
        .w(px(280.0))
        .h_full()
        .gap(SPACING_MD)
        .child(
            Label::new("CVE Database")
                .text_size(TEXT_SIZE_LG)
                .font_weight(FontWeight::SEMIBOLD)
        )
        .child(
            v_flex()
                .flex_1()
                .when(finding.is_none(), |this| {
                    this.items_center()
                        .justify_center()
                        .child(
                            Label::new("Select a finding to view CVE info")
                                .text_color(rgb(TEXT_MUTED))
                        )
                })
                .when_some(finding.clone(), |this, f| {
                    match vuln_reference {
                        Some(v) => this.child(render_cve_content(&f, &v)),
                        None => this.child(render_finding_only_content(&f)),
                    }
                })
        )
        // Quick actions (only shown when finding is selected)
        .children(finding.as_ref().map(|f| {
            render_quick_actions(f, vuln_store)
        }))
}

/// Render CVE content when both finding and vulnerability reference exist
fn render_cve_content(finding: &Finding, vuln: &Vulnerability) -> impl IntoElement {
    v_flex()
        .gap(SPACING_LG)
        // CVSS Score section
        .child(render_cvss_section(vuln))
        // Detection info
        .child(render_detection_info(finding))
        // Affected systems
        .child(render_affected_systems(vuln))
        // References
        .child(render_references(vuln))
}

/// Render content when only finding exists (no CVE reference)
fn render_finding_only_content(finding: &Finding) -> impl IntoElement {
    v_flex()
        .gap(SPACING_LG)
        // Severity badge
        .child(
            v_flex()
                .gap(SPACING_SM)
                .child(
                    Label::new("Severity")
                        .text_size(TEXT_SIZE_SM)
                        .text_color(rgb(TEXT_MUTED))
                )
                .child(
                    h_flex()
                        .px(SPACING_MD)
                        .py(SPACING_SM)
                        .rounded_md()
                        .bg(rgb(crate::severity_color(&finding.severity)))
                        .child(
                            Label::new(format!("{:?}", finding.severity))
                                .text_size(TEXT_SIZE_LG)
                                .font_weight(FontWeight::BOLD)
                                .text_color(gpui::white())
                        )
                )
        )
        // Detection info
        .child(render_detection_info(finding))
}

/// Render CVSS score section
fn render_cvss_section(vuln: &Vulnerability) -> impl IntoElement {
    let cvss_score = vuln.cvss_score.unwrap_or(0.0);
    let score_text = format!("{:.1}", cvss_score);

    // Determine color based on score
    let color = if cvss_score >= 9.0 {
        SEVERITY_CRITICAL
    } else if cvss_score >= 7.0 {
        SEVERITY_HIGH
    } else if cvss_score >= 4.0 {
        SEVERITY_MEDIUM
    } else {
        SEVERITY_LOW
    };

    v_flex()
        .gap(SPACING_SM)
        .child(
            Label::new("CVSS Score")
                .text_size(TEXT_SIZE_SM)
                .text_color(rgb(TEXT_MUTED))
        )
        .child(
            h_flex()
                .gap(SPACING_MD)
                .items_center()
                .child(
                    v_flex()
                        .w(px(80.0))
                        .h(px(80.0))
                        .items_center()
                        .justify_center()
                        .rounded_lg()
                        .bg(rgb(color))
                        .child(
                            Label::new(score_text)
                                .text_size(TEXT_SIZE_XL)
                                .font_weight(FontWeight::BOLD)
                                .text_color(gpui::white())
                        )
                )
                .child(
                    v_flex()
                        .gap(SPACING_XS)
                        .child(
                            Label::new(format!("CVE: {}", vuln.cve_id))
                                .text_size(TEXT_SIZE_BASE)
                                .font_weight(FontWeight::MEDIUM)
                        )
                        .child(
                            Label::new(format!("CWE: {}", vuln.cwe_id))
                                .text_size(TEXT_SIZE_SM)
                                .text_color(rgb(TEXT_MUTED))
                        )
                )
        )
        // CVSS Vector
        .children(if vuln.cvss_vector.is_empty() {
            None
        } else {
            Some(
                Label::new(vuln.cvss_vector.clone())
                    .text_size(TEXT_SIZE_XS)
                    .text_color(rgb(TEXT_MUTED))
            )
        })
        // Exploit availability
        .child(
            h_flex()
                .mt(SPACING_SM)
                .gap(SPACING_SM)
                .items_center()
                .child(
                    h_flex()
                        .w(px(8.0))
                        .h(px(8.0))
                        .rounded_full()
                        .bg(rgb(if vuln.exploit_available {
                            STATUS_ERROR
                        } else {
                            STATUS_SUCCESS
                        }))
                )
                .child(
                    Label::new(if vuln.exploit_available {
                        "Exploit available"
                    } else {
                        "No known exploit"
                    })
                        .text_size(TEXT_SIZE_SM)
                        .text_color(rgb(TEXT_SECONDARY))
                )
        )
}

/// Render detection information
fn render_detection_info(finding: &Finding) -> impl IntoElement {
    let detected_at = finding.detected_at.format("%Y-%m-%d %H:%M").to_string();

    v_flex()
        .p(SPACING_MD)
        .rounded_md()
        .bg(rgb(BG_CARD))
        .gap(SPACING_MD)
        .child(
            Label::new("Detection Info")
                .text_size(TEXT_SIZE_BASE)
                .font_weight(FontWeight::SEMIBOLD)
        )
        .child(
            v_flex()
                .gap(SPACING_XS)
                .child(
                    Label::new("Status")
                        .text_size(TEXT_SIZE_SM)
                        .text_color(rgb(TEXT_MUTED))
                )
                .child(
                    h_flex()
                        .px(SPACING_SM)
                        .py(SPACING_XS)
                        .rounded_md()
                        .bg(rgb(status_color(&finding.status)))
                        .child(
                            Label::new(crate::status_label(&finding.status))
                                .text_size(TEXT_SIZE_SM)
                                .text_color(gpui::white())
                        )
                )
        )
        .child(
            v_flex()
                .gap(SPACING_XS)
                .child(
                    Label::new("Detected At")
                        .text_size(TEXT_SIZE_SM)
                        .text_color(rgb(TEXT_MUTED))
                )
                .child(
                    Label::new(detected_at)
                        .text_size(TEXT_SIZE_BASE)
                )
        )
        .child(
            v_flex()
                .gap(SPACING_XS)
                .child(
                    Label::new("Detected By")
                        .text_size(TEXT_SIZE_SM)
                        .text_color(rgb(TEXT_MUTED))
                )
                .child(
                    Label::new(finding.detected_by.clone())
                        .text_size(TEXT_SIZE_BASE)
                )
        )
        .child(
            v_flex()
                .gap(SPACING_XS)
                .child(
                    Label::new("Asset ID")
                        .text_size(TEXT_SIZE_SM)
                        .text_color(rgb(TEXT_MUTED))
                )
                .child(
                    Label::new(format!("{}", finding.asset_id))
                        .text_size(TEXT_SIZE_BASE)
                )
        )
}

/// Render affected systems
fn render_affected_systems(vuln: &Vulnerability) -> impl IntoElement {
    if vuln.affected_systems.is_empty() {
        return v_flex().into_any_element();
    }

    v_flex()
        .gap(SPACING_SM)
        .child(
            Label::new("Affected Systems")
                .text_size(TEXT_SIZE_BASE)
                .font_weight(FontWeight::SEMIBOLD)
        )
        .child(
            v_flex()
                .gap(SPACING_XS)
                .children(vuln.affected_systems.iter().map(|system| {
                    h_flex()
                        .px(SPACING_SM)
                        .py(SPACING_XS)
                        .rounded_md()
                        .bg(rgb(BG_SECONDARY))
                        .child(
                            Label::new(system.clone())
                                .text_size(TEXT_SIZE_SM)
                        )
                }))
        )
        .children(if vuln.affected_versions.is_empty() {
            None
        } else {
            Some(
                v_flex()
                    .mt(SPACING_SM)
                    .gap(SPACING_XS)
                    .child(
                        Label::new("Affected Versions")
                            .text_size(TEXT_SIZE_SM)
                            .text_color(rgb(TEXT_MUTED))
                    )
                    .child(
                        Label::new(vuln.affected_versions.clone())
                            .text_size(TEXT_SIZE_SM)
                    )
            )
        })
        .into_any_element()
}

/// Render references
fn render_references(vuln: &Vulnerability) -> impl IntoElement {
    if vuln.ref_urls.is_empty() {
        return v_flex().into_any_element();
    }

    v_flex()
        .gap(SPACING_SM)
        .child(
            Label::new("References")
                .text_size(TEXT_SIZE_BASE)
                .font_weight(FontWeight::SEMIBOLD)
        )
        .child(
            v_flex()
                .gap(SPACING_XS)
                .children(vuln.ref_urls.iter().take(5).map(|url| {
                    Label::new(url.clone())
                        .text_size(TEXT_SIZE_XS)
                        .text_color(rgb(ACCENT_BLUE))
                        .line_clamp(1)
                }))
        )
        .into_any_element()
}

/// Render quick action buttons
fn render_quick_actions(
    finding: &Finding,
    vuln_store: &Entity<VulnStore>,
) -> impl IntoElement {
    let finding_id = finding.id;
    let vuln_store_clone = vuln_store.clone();

    v_flex()
        .mt(SPACING_MD)
        .gap(SPACING_SM)
        .child(
            Label::new("Quick Actions")
                .text_size(TEXT_SIZE_BASE)
                .font_weight(FontWeight::SEMIBOLD)
        )
        .child(
            Button::new("verify")
                .label("Verify Finding")
                .primary()
                .on_click(move |_event, _window, cx| {
                    vuln_store_clone.update(cx, |store, cx| {
                        use data::models::FindingStatus::Validating;
                        let _ = store.update_finding_status(finding_id, Validating, cx);
                    });
                })
        )
        .child(
            Button::new("export")
                .label("Export Report")
                .on_click(move |_event, _window, _cx| {
                    // Export functionality placeholder
                    println!("Export report for finding {}", finding_id);
                })
        )
        .child(
            Button::new("create-task")
                .label("Create Task")
                .on_click(move |_event, _window, _cx| {
                    // Create task functionality placeholder
                    println!("Create task for finding {}", finding_id);
                })
        )
}
