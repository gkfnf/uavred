//! Finding List Panel - Left column showing findings grouped by severity

use gpui::*;
use gpui_component::{
    h_flex, v_flex,
    label::Label,
    scroll::ScrollableElement,
};
use data::{VulnStore, Finding};
use ui::theme::*;
use crate::{severity_color, severity_label};

/// Render the left column finding list
pub fn render_finding_list(
    findings: &[Finding],
    vuln_store: &Entity<VulnStore>,
) -> impl IntoElement {
    let grouped = group_findings_by_severity(findings);

    v_flex()
        .w(px(300.0))
        .h_full()
        .gap(SPACING_SM)
        .child(render_header(findings.len()))
        .child(
            v_flex()
                .flex_1()
                .overflow_y_scrollbar()
                .gap(SPACING_MD)
                .children(grouped.into_iter().map(|(severity, findings)| {
                    render_severity_group(severity, findings, vuln_store)
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
            Label::new("Vulnerabilities")
                .text_size(TEXT_SIZE_LG)
                .font_weight(FontWeight::SEMIBOLD)
        )
        .child(
            Label::new(format!("{}", total_count))
                .text_color(rgb(TEXT_MUTED))
                .text_size(TEXT_SIZE_SM)
        )
}

/// Group findings by severity (Critical, High, Medium, Low)
fn group_findings_by_severity(
    findings: &[Finding],
) -> Vec<(data::models::Severity, Vec<&Finding>)> {
    use data::models::Severity::*;
    let order = vec![Critical, High, Medium, Low];

    order
        .into_iter()
        .filter_map(|sev| {
            let items: Vec<&Finding> = findings.iter().filter(|f| f.severity == sev).collect();
            if items.is_empty() {
                None
            } else {
                Some((sev, items))
            }
        })
        .collect()
}

/// Render a severity group section
fn render_severity_group(
    severity: data::models::Severity,
    findings: Vec<&Finding>,
    vuln_store: &Entity<VulnStore>,
) -> impl IntoElement {
    let color = severity_color(&severity);
    let label = severity_label(&severity);
    let count = findings.len();

    v_flex()
        .gap(SPACING_XS)
        .child(
            // Group header
            h_flex()
                .px(SPACING_MD)
                .py(SPACING_XS)
                .gap(SPACING_SM)
                .items_center()
                .child(
                    h_flex()
                        .w(px(8.0))
                        .h(px(8.0))
                        .rounded_full()
                        .bg(rgb(color))
                )
                .child(
                    Label::new(label)
                        .text_size(TEXT_SIZE_SM)
                        .font_weight(FontWeight::SEMIBOLD)
                )
                .child(
                    Label::new(format!("({})", count))
                        .text_color(rgb(TEXT_MUTED))
                        .text_size(TEXT_SIZE_SM)
                ),
        )
        .children(findings.into_iter().map(|finding| {
            render_finding_item(finding, vuln_store)
        }))
}

/// Render a single finding item in the list
fn render_finding_item(
    finding: &Finding,
    vuln_store: &Entity<VulnStore>,
) -> impl IntoElement {
    let finding_id = finding.id;
    let cve_id = finding.vuln_id.clone().unwrap_or_else(|| "Unknown".to_string());
    let title = finding.title.clone();
    let has_ai = finding.ai_confidence.is_some();
    let has_poc = !finding.poc_code.is_empty();

    let vuln_store_clone = vuln_store.clone();

    h_flex()
        .px(SPACING_MD)
        .py(SPACING_SM)
        .gap(SPACING_SM)
        .items_start()
        .cursor_pointer()
        .hover(|s| s.bg(rgb(BG_CARD_HOVER)))
        .cursor_pointer()
        .on_mouse_down(gpui::MouseButton::Left, move |_event, _window, cx| {
            vuln_store_clone.update(cx, |store, cx| {
                store.select_finding(finding_id, cx);
            });
        })
        .child(
            // CVE ID and indicators column
            v_flex()
                .gap(SPACING_XS)
                .flex_1()
                .child(
                    h_flex()
                        .gap(SPACING_XS)
                        .items_center()
                        .child(
                            Label::new(cve_id)
                                .text_size(TEXT_SIZE_SM)
                                .font_weight(FontWeight::MEDIUM)
                        )
                        .children(if has_ai {
                            Some(
                                h_flex()
                                    .px(SPACING_XS)
                                    .py(px(2.0))
                                    .rounded_sm()
                                    .bg(rgb(STATUS_AI))
                                    .child(
                                        Label::new("AI")
                                            .text_size(TEXT_SIZE_XS)
                                            .text_color(gpui::white())
                                    )
                            )
                        } else {
                            None
                        })
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
                        }),
                )
                .child(
                    Label::new(title)
                        .text_size(TEXT_SIZE_SM)
                        .text_color(rgb(TEXT_SECONDARY))
                        .line_clamp(2)
                ),
        )
}
