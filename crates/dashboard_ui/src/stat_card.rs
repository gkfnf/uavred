// Stat card component for displaying dashboard metrics

use gpui::*;
use gpui_component::{h_flex, label::Label};
use ui::theme::*;

/// Render a single stat card
fn render_stat_card(title: &str, value: usize, color: u32) -> impl IntoElement {
    div()
        .bg(rgb(BG_CARD))
        .border(px(1.0))
        .border_color(rgb(BORDER_COLOR))
        .rounded(BORDER_RADIUS)
        .p(PADDING_LG)
        .flex_1()
        .child(
            h_flex()
                .flex_col()
                .gap(SPACING_MD)
                .w_full()
                .child(
                    Label::new(title.to_string())
                        .text_sm()
                        .text_color(rgb(TEXT_SECONDARY))
                )
                .child(
                    Label::new(value.to_string())
                        .text_3xl()
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(color))
                )
        )
}

/// Render stat cards row for dashboard
pub fn render_stat_cards(
    todo_count: usize,
    in_progress_count: usize,
    in_review_count: usize,
    done_count: usize,
    critical_vuln_count: usize,
) -> impl IntoElement {
    h_flex()
        .gap(SPACING_LG)
        .w_full()
        .child(render_stat_card("To Do", todo_count, SEVERITY_MEDIUM))
        .child(render_stat_card("In Progress", in_progress_count, ACCENT_BLUE))
        .child(render_stat_card("In Review", in_review_count, SEVERITY_HIGH))
        .child(render_stat_card("Done", done_count, STATUS_SUCCESS))
        .child(render_stat_card("Critical", critical_vuln_count, SEVERITY_CRITICAL))
}
