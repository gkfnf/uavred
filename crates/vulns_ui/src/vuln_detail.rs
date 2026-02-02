//! Vulnerability Detail Panel - Middle column showing vulnerability details and findings

use gpui::*;
use gpui::prelude::FluentBuilder;
use gpui_component::{
    h_flex, v_flex,
    label::Label,
    scroll::ScrollableElement,
};
use data::{VulnStore, VulnerabilityWithFindings, Finding};
use ui::theme::*;
use crate::{severity_color, severity_label, status_color, status_label};

/// Render the middle column vulnerability detail
pub fn render_vuln_detail(
    vuln: Option<VulnerabilityWithFindings>,
    selected_finding: Option<Finding>,
    vuln_store: &Entity<VulnStore>,
) -> impl IntoElement {
    v_flex()
        .flex_1()
        .h_full()
        .gap(SPACING_MD)
        .when_some(vuln.clone(), |this, vuln| {
            this.child(render_vuln_header(&vuln))
                .child(render_ai_analysis(&vuln))
                .child(render_associated_findings(&vuln, selected_finding, vuln_store))
        })
        .when(vuln.is_none(), |this| {
            this.flex_1()
                .items_center()
                .justify_center()
                .child(
                    v_flex()
                        .gap(SPACING_MD)
                        .items_center()
                        .child(
                            Label::new("Select a vulnerability")
                                .text_size(TEXT_SIZE_LG)
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(rgb(TEXT_SECONDARY))
                        )
                        .child(
                            Label::new("Choose a vulnerability from the list to view details")
                                .text_size(TEXT_SIZE_BASE)
                                .text_color(rgb(TEXT_MUTED))
                        )
                )
        })
}

/// Render vulnerability header with key info
fn render_vuln_header(vuln: &VulnerabilityWithFindings) -> impl IntoElement {
    let severity = &vuln.vulnerability.severity;
    let color = severity_color(severity);
    
    v_flex()
        .gap(SPACING_MD)
        .p(SPACING_MD)
        .bg(rgb(BG_CARD))
        .rounded_md()
        .child(
            h_flex()
                .gap(SPACING_SM)
                .items_center()
                .child(
                    h_flex()
                        .px(SPACING_SM)
                        .py(SPACING_XS)
                        .rounded_md()
                        .bg(rgb(color))
                        .child(
                            Label::new(severity_label(severity))
                                .text_size(TEXT_SIZE_SM)
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(gpui::white())
                        )
                )
                .child(
                    Label::new(&vuln.vulnerability.cve_id)
                        .text_size(TEXT_SIZE_BASE)
                        .font_weight(FontWeight::MEDIUM)
                )
                .when(vuln.vulnerability.cvss_score.is_some(), |this| {
                    let score = vuln.vulnerability.cvss_score.unwrap();
                    let score_color = if score >= 7.0 {
                        SEVERITY_HIGH
                    } else if score >= 4.0 {
                        SEVERITY_MEDIUM
                    } else {
                        SEVERITY_LOW
                    };
                    this.child(
                        h_flex()
                            .px(SPACING_SM)
                            .py(SPACING_XS)
                            .rounded_md()
                            .bg(rgb(score_color))
                            .child(
                                Label::new(format!("CVSS {:.1}", score))
                                    .text_size(TEXT_SIZE_SM)
                                    .text_color(gpui::white())
                            )
                    )
                })
        )
        .child(
            v_flex()
                .gap(SPACING_XS)
                .child(
                    Label::new(&vuln.vulnerability.name)
                        .text_size(TEXT_SIZE_XL)
                        .font_weight(FontWeight::SEMIBOLD)
                )
                .child(
                    Label::new(&vuln.vulnerability.description)
                        .text_size(TEXT_SIZE_BASE)
                        .text_color(rgb(TEXT_SECONDARY))
                        .line_clamp(5)
                )
        )
        .child(
            h_flex()
                .flex_wrap()
                .gap(SPACING_LG)
                .children(if !vuln.vulnerability.vuln_type.is_empty() {
                    Some(render_meta_item("Type", &vuln.vulnerability.vuln_type))
                } else {
                    None
                })
                .children(if !vuln.vulnerability.cwe_id.is_empty() {
                    Some(render_meta_item("CWE", &vuln.vulnerability.cwe_id))
                } else {
                    None
                })
                .children(if vuln.vulnerability.exploit_available {
                    Some(render_meta_item("Exploit", "Available"))
                } else {
                    None
                })
        )
}

/// Render AI Security Analysis section (from findings)
fn render_ai_analysis(vuln: &VulnerabilityWithFindings) -> impl IntoElement + '_ {
    // Aggregate AI analysis from all findings - clone to owned strings
    let analyses: Vec<String> = vuln.findings.iter()
        .filter(|f| !f.ai_analysis.is_empty())
        .map(|f| f.ai_analysis.clone())
        .collect();
    
    let recommendations: Vec<String> = vuln.findings.iter()
        .filter(|f| !f.ai_recommendation.is_empty())
        .map(|f| f.ai_recommendation.clone())
        .collect();
    
    let avg_confidence: Option<i32> = if vuln.findings.is_empty() {
        None
    } else {
        let total: i32 = vuln.findings.iter()
            .filter_map(|f| f.ai_confidence)
            .sum();
        let count = vuln.findings.iter()
            .filter(|f| f.ai_confidence.is_some())
            .count() as i32;
        if count > 0 { Some(total / count) } else { None }
    };

    v_flex()
        .gap(SPACING_MD)
        .p(SPACING_MD)
        .bg(rgb(BG_CARD))
        .rounded_md()
        .child(
            h_flex()
                .gap(SPACING_SM)
                .items_center()
                .child(
                    Label::new("AI Security Analysis")
                        .text_size(TEXT_SIZE_LG)
                        .font_weight(FontWeight::SEMIBOLD)
                )
                .children(avg_confidence.map(|conf| {
                    h_flex()
                        .px(SPACING_SM)
                        .py(SPACING_XS)
                        .rounded_md()
                        .bg(rgb(STATUS_AI))
                        .child(
                            Label::new(format!("{}% confidence", conf))
                                .text_size(TEXT_SIZE_SM)
                                .text_color(gpui::white())
                        )
                }))
        )
        .children(if !analyses.is_empty() {
            Some(
                v_flex()
                    .gap(SPACING_SM)
                    .child(
                        Label::new("Analysis")
                            .text_size(TEXT_SIZE_SM)
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgb(TEXT_SECONDARY))
                    )
                    .child(
                        Label::new(analyses[0].clone())
                            .text_size(TEXT_SIZE_BASE)
                            .text_color(rgb(TEXT_PRIMARY))
                    )
            )
        } else {
            None
        })
        .children(if !recommendations.is_empty() {
            Some(
                v_flex()
                    .gap(SPACING_SM)
                    .mt(SPACING_SM)
                    .child(
                        Label::new("Recommendation")
                            .text_size(TEXT_SIZE_SM)
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgb(TEXT_SECONDARY))
                    )
                    .child(
                        Label::new(recommendations[0].clone())
                            .text_size(TEXT_SIZE_BASE)
                            .text_color(rgb(TEXT_PRIMARY))
                    )
            )
        } else {
            None
        })
}

/// Render associated findings section
fn render_associated_findings(
    vuln: &VulnerabilityWithFindings,
    selected_finding: Option<Finding>,
    vuln_store: &Entity<VulnStore>,
) -> impl IntoElement {
    let selected_finding_id = selected_finding.map(|f| f.id);

    v_flex()
        .flex_1()
        .gap(SPACING_MD)
        .p(SPACING_MD)
        .bg(rgb(BG_CARD))
        .rounded_md()
        .child(
            h_flex()
                .justify_between()
                .items_center()
                .child(
                    Label::new(format!("Associated Findings ({})", vuln.finding_count()))
                        .text_size(TEXT_SIZE_LG)
                        .font_weight(FontWeight::SEMIBOLD)
                )
        )
        .child(
            v_flex()
                .flex_1()
                .overflow_y_scrollbar()
                .gap(SPACING_SM)
                .children(vuln.findings.iter().map(|finding| {
                    let is_selected = selected_finding_id == Some(finding.id);
                    render_finding_item(finding, vuln_store, is_selected)
                }))
                .when(vuln.findings.is_empty(), |this| {
                    this.child(
                        v_flex()
                            .flex_1()
                            .items_center()
                            .justify_center()
                            .child(
                                Label::new("No findings yet")
                                    .text_size(TEXT_SIZE_BASE)
                                    .text_color(rgb(TEXT_MUTED))
                            )
                    )
                })
        )
}

/// Render a single finding item
fn render_finding_item(
    finding: &Finding,
    vuln_store: &Entity<VulnStore>,
    is_selected: bool,
) -> impl IntoElement {
    let finding_id = finding.id;
    let title = finding.title.clone();
    let status = finding.status.clone();
    let asset_id = finding.asset_id;
    let has_poc = !finding.poc_code.is_empty();
    
    let vuln_store_clone = vuln_store.clone();
    let status_color_val = status_color(&status);

    h_flex()
        .px(SPACING_MD)
        .py(SPACING_SM)
        .gap(SPACING_SM)
        .items_center()
        .cursor_pointer()
        .rounded_md()
        .when(is_selected, |this| {
            this.bg(rgb(BG_CARD_HOVER))
                .border(px(1.0))
                .border_color(rgb(ACCENT_BLUE))
        })
        .when(!is_selected, |this| {
            this.hover(|s| s.bg(rgb(BG_CARD_HOVER)))
        })
        .on_mouse_down(gpui::MouseButton::Left, move |_event, _window, cx| {
            vuln_store_clone.update(cx, |store, cx| {
                store.select_finding(finding_id, cx);
            });
        })
        .child(
            h_flex()
                .w(px(8.0))
                .h(px(8.0))
                .rounded_full()
                .bg(rgb(status_color_val))
        )
        .child(
            v_flex()
                .flex_1()
                .gap(SPACING_XS)
                .child(
                    Label::new(title)
                        .text_size(TEXT_SIZE_SM)
                        .font_weight(FontWeight::MEDIUM)
                )
                .child(
                    h_flex()
                        .gap(SPACING_MD)
                        .child(
                            Label::new(format!("Asset {}", asset_id))
                                .text_size(TEXT_SIZE_XS)
                                .text_color(rgb(TEXT_MUTED))
                        )
                        .child(
                            Label::new(status_label(&status))
                                .text_size(TEXT_SIZE_XS)
                                .text_color(rgb(status_color_val))
                        )
                )
        )
        .children(if has_poc {
            Some(
                h_flex()
                    .px(SPACING_XS)
                    .py(px(2.0))
                    .rounded_sm()
                    .bg(rgb(ACCENT_BLUE))
                    .child(
                        Label::new("PoC")
                            .text_size(TEXT_SIZE_XS)
                            .text_color(gpui::white())
                    )
            )
        } else {
            None
        })
}

/// Render a metadata item
fn render_meta_item(label: &str, value: &str) -> impl IntoElement {
    let label = label.to_string();
    let value = value.to_string();
    h_flex()
        .gap(SPACING_XS)
        .items_center()
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
