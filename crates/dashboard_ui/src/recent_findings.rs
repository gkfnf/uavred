// Recent findings preview component

use gpui::*;
use gpui_component::{h_flex, label::Label, tag::Tag, v_flex, Sizable};
use ui::theme::*;

/// Recent finding item for preview
pub struct RecentFinding {
    pub title: String,
    pub severity: String,
    pub asset: String,
    pub time: String,
}

impl RecentFinding {
    pub fn new(
        title: impl Into<String>,
        severity: impl Into<String>,
        asset: impl Into<String>,
        time: impl Into<String>,
    ) -> Self {
        Self {
            title: title.into(),
            severity: severity.into(),
            asset: asset.into(),
            time: time.into(),
        }
    }
}

/// Render a single finding row
fn render_finding_row(finding: &RecentFinding) -> impl IntoElement {
    let severity_color = match finding.severity.as_str() {
        "critical" => SEVERITY_CRITICAL,
        "high" => SEVERITY_HIGH,
        "medium" => SEVERITY_MEDIUM,
        _ => SEVERITY_LOW,
    };

    h_flex()
        .gap(SPACING_MD)
        .items_center()
        .justify_between()
        .p(PADDING_MD)
        .border_b(px(1.0))
        .border_color(rgb(BORDER_COLOR))
        .child(
            v_flex()
                .gap(SPACING_SM)
                .flex_1()
                .child(
                    Label::new(finding.title.clone())
                        .text_sm()
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(rgb(TEXT_PRIMARY))
                )
                .child(
                    h_flex()
                        .gap(SPACING_SM)
                        .items_center()
                        .child(
                            Label::new(finding.asset.clone())
                                .text_xs()
                                .text_color(rgb(TEXT_SECONDARY))
                        )
                        .child(
                            Label::new(format!("• {}", finding.time))
                                .text_xs()
                                .text_color(rgb(TEXT_SECONDARY))
                        )
                )
        )
        .child(
            Tag::new()
                .small()
                .bg(rgb(severity_color))
                .text_color(rgb(0xffffff))
                .child(Label::new(finding.severity.clone()).text_xs())
        )
}

/// Render recent findings section
pub fn render_recent_findings(findings: &[RecentFinding]) -> impl IntoElement {
    v_flex()
        .gap(px(0.0))
        .w_full()
        .bg(rgb(BG_CARD))
        .border(px(1.0))
        .border_color(rgb(BORDER_COLOR))
        .rounded(BORDER_RADIUS)
        .child(
            h_flex()
                .justify_between()
                .items_center()
                .p(PADDING_LG)
                .border_b(px(1.0))
                .border_color(rgb(BORDER_COLOR))
                .child(
                    Label::new("Recent Findings")
                        .text_sm()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(TEXT_PRIMARY))
                )
                .child(
                    Label::new(format!("{} total", findings.len()))
                        .text_xs()
                        .text_color(rgb(TEXT_SECONDARY))
                )
        )
        .children(
            if findings.is_empty() {
                vec![
                    div()
                        .p(PADDING_LG)
                        .child(
                            Label::new("No recent findings")
                                .text_sm()
                                .text_color(rgb(TEXT_SECONDARY))
                        )
                        .into_any_element()
                ]
            } else {
                findings
                    .iter()
                    .take(5)
                    .map(|f| render_finding_row(f).into_any_element())
                    .collect()
            }
        )
}
