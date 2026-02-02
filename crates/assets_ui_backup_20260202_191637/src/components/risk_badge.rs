use data::models::Severity;
use gpui::*;
use gpui_component::{h_flex, label::Label};
use ui::theme::*;

pub fn render_risk_badge(severity: &Severity, risk_score: u8) -> impl IntoElement {
    let color = match severity {
        Severity::Critical => SEVERITY_CRITICAL,
        Severity::High => SEVERITY_HIGH,
        Severity::Medium => SEVERITY_MEDIUM,
        Severity::Low => SEVERITY_LOW,
        Severity::Info => STATUS_SUCCESS,
    };

    let text = match severity {
        Severity::Critical => "Critical",
        Severity::High => "High",
        Severity::Medium => "Medium",
        Severity::Low => "Low",
        Severity::Info => "Info",
    };

    let risk_text = format!("{}/100", risk_score);

    h_flex()
        .gap_2()
        .items_center()
        .px_3()
        .py_2()
        .rounded_lg()
        .border_1()
        .border_color(rgb(color))
        .bg(rgb(BG_SECONDARY))
        .child(
            Label::new(text)
                .text_xs()
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(rgb(color)),
        )
        .child(
            Label::new(risk_text)
                .text_xs()
                .text_color(rgb(TEXT_SECONDARY)),
        )
}
