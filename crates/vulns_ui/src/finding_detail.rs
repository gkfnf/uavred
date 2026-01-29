//! Finding Detail Panel - Middle column showing finding details, AI analysis, and PoC

use gpui::*;
use gpui::prelude::FluentBuilder;
use gpui_component::{
    h_flex, v_flex,
    label::Label,
};
use data::models::Finding;
use ui::theme::*;

/// Render the middle column finding detail
pub fn render_finding_detail(
    finding: Option<Finding>,
) -> impl IntoElement {
    v_flex()
        .flex_1()
        .h_full()
        .gap(SPACING_MD)
        .child(render_detail_header())
        .child(
            v_flex()
                .flex_1()
                .when(finding.is_none(), |this| {
                    this.items_center()
                        .justify_center()
                        .child(
                            Label::new("Select a vulnerability to view details")
                                .text_color(rgb(TEXT_MUTED))
                        )
                })
                .when_some(finding, |this, f| {
                    this.child(render_finding_content(&f))
                })
        )
}

/// Render the detail column header
fn render_detail_header() -> impl IntoElement {
    h_flex()
        .px(SPACING_MD)
        .py(SPACING_SM)
        .child(
            Label::new("Details & PoC")
                .text_size(TEXT_SIZE_LG)
                .font_weight(FontWeight::SEMIBOLD)
        )
}

/// Render the finding content with all sections
fn render_finding_content(finding: &Finding) -> impl IntoElement {
    v_flex()
        .gap(SPACING_LG)
        // Description section
        .child(render_description_section(finding))
        // AI Analysis section (if available)
        .child(render_ai_analysis_section(finding))
        // PoC section (if available)
        .child(render_poc_section(finding))
        // MITRE ATT&CK section (if available)
        .child(render_mitre_section(finding))
}

/// Render the description section
fn render_description_section(finding: &Finding) -> impl IntoElement {
    v_flex()
        .gap(SPACING_SM)
        .child(
            Label::new("Description")
                .text_size(TEXT_SIZE_BASE)
                .font_weight(FontWeight::SEMIBOLD)
        )
        .child(
            v_flex()
                .p(SPACING_MD)
                .rounded_md()
                .bg(rgb(BG_CARD))
                .child(
                    Label::new(finding.description.clone())
                        .text_size(TEXT_SIZE_BASE)
                        .text_color(rgb(TEXT_SECONDARY))
                ),
        )
        // Evidence if present
        .children(if finding.evidence.is_empty() {
            None
        } else {
            Some(
                v_flex()
                    .mt(SPACING_MD)
                    .gap(SPACING_SM)
                    .child(
                        Label::new("Evidence")
                            .text_size(TEXT_SIZE_BASE)
                            .font_weight(FontWeight::SEMIBOLD)
                    )
                    .child(
                        v_flex()
                            .p(SPACING_MD)
                            .rounded_md()
                            .bg(rgb(BG_SECONDARY))
                            .child(
                                Label::new(finding.evidence.clone())
                                    .text_size(TEXT_SIZE_SM)
                                    .text_color(rgb(TEXT_SECONDARY))
                            ),
                    )
            )
        })
}

/// Render the AI Analysis section
fn render_ai_analysis_section(finding: &Finding) -> impl IntoElement {
    let ai_confidence = match finding.ai_confidence {
        Some(conf) => conf,
        None => return v_flex().into_any_element(),
    };

    let ai_analysis = if finding.ai_analysis.is_empty() {
        "No detailed AI analysis available."
    } else {
        &finding.ai_analysis
    };

    let ai_recommendation = if finding.ai_recommendation.is_empty() {
        None
    } else {
        Some(finding.ai_recommendation.clone())
    };

    v_flex()
        .gap(SPACING_SM)
        .child(
            h_flex()
                .gap(SPACING_SM)
                .items_center()
                .child(
                    Label::new("AI Security Analysis")
                        .text_size(TEXT_SIZE_BASE)
                        .font_weight(FontWeight::SEMIBOLD)
                )
                .child(
                    h_flex()
                        .px(SPACING_SM)
                        .py(SPACING_XS)
                        .rounded_md()
                        .bg(rgb(STATUS_AI))
                        .child(
                            Label::new(format!("{}% confidence", ai_confidence))
                                .text_size(TEXT_SIZE_SM)
                                .text_color(gpui::white())
                        )
                ),
        )
        .child(
            v_flex()
                .p(SPACING_MD)
                .rounded_md()
                .bg(rgb(BG_CARD))
                .gap(SPACING_MD)
                .child(
                    Label::new(ai_analysis.to_string())
                        .text_size(TEXT_SIZE_BASE)
                        .text_color(rgb(TEXT_SECONDARY))
                )
                .children(ai_recommendation.map(|rec| {
                    v_flex()
                        .mt(SPACING_SM)
                        .gap(SPACING_XS)
                        .child(
                            Label::new("Recommendation")
                                .text_size(TEXT_SIZE_SM)
                                .font_weight(FontWeight::SEMIBOLD)
                        )
                        .child(
                            Label::new(rec)
                                .text_size(TEXT_SIZE_BASE)
                                .text_color(rgb(TEXT_SECONDARY))
                        )
                }))
        )
        .into_any_element()
}

/// Render the PoC code section
fn render_poc_section(finding: &Finding) -> impl IntoElement {
    if finding.poc_code.is_empty() {
        return v_flex().into_any_element();
    }

    let poc_code = finding.poc_code.clone();
    let poc_language = if finding.poc_language.is_empty() {
        "python".to_string()
    } else {
        finding.poc_language.clone()
    };

    v_flex()
        .gap(SPACING_SM)
        .child(
            h_flex()
                .gap(SPACING_SM)
                .items_center()
                .child(
                    Label::new("AI-Generated PoC")
                        .text_size(TEXT_SIZE_BASE)
                        .font_weight(FontWeight::SEMIBOLD)
                )
                .child(
                    h_flex()
                        .px(SPACING_SM)
                        .py(SPACING_XS)
                        .rounded_md()
                        .bg(rgb(BG_DARK))
                        .child(
                            Label::new(poc_language.to_uppercase())
                                .text_size(TEXT_SIZE_XS)
                                .text_color(gpui::white())
                        )
                ),
        )
        .child(
            v_flex()
                .p(SPACING_MD)
                .rounded_md()
                .bg(rgb(BG_DARK))
                .child(
                    Label::new(poc_code)
                        .text_size(TEXT_SIZE_SM)
                        .text_color(rgb(0x10b981)) // Green code color
                ),
        )
        .into_any_element()
}

/// Render the MITRE ATT&CK section
fn render_mitre_section(finding: &Finding) -> impl IntoElement {
    if finding.mitre_techniques.is_empty() {
        return v_flex().into_any_element();
    }

    v_flex()
        .gap(SPACING_SM)
        .child(
            Label::new("MITRE ATT&CK")
                .text_size(TEXT_SIZE_BASE)
                .font_weight(FontWeight::SEMIBOLD)
        )
        .child(
            h_flex()
                .flex_wrap()
                .gap(SPACING_SM)
                .children(finding.mitre_techniques.iter().map(|technique| {
                    h_flex()
                        .px(SPACING_SM)
                        .py(SPACING_XS)
                        .rounded_md()
                        .bg(rgb(ACCENT_BLUE))
                        .child(
                            Label::new(technique.clone())
                                .text_size(TEXT_SIZE_SM)
                                .text_color(gpui::white())
                        )
                })),
        )
        .into_any_element()
}
