// Task progress visualization component

use gpui::*;
use gpui_component::{h_flex, label::Label, v_flex};
use ui::theme::*;

/// Render a progress ring chart showing task completion percentage
pub fn render_task_progress_ring(
    total_tasks: usize,
    done_tasks: usize,
) -> impl IntoElement {
    let percentage = if total_tasks > 0 {
        ((done_tasks as f32 / total_tasks as f32) * 100.0) as usize
    } else {
        0
    };

    let remaining = total_tasks.saturating_sub(done_tasks);

    v_flex()
        .gap(SPACING_MD)
        .p(PADDING_LG)
        .bg(rgb(BG_CARD))
        .border(px(1.0))
        .border_color(rgb(BORDER_COLOR))
        .rounded(BORDER_RADIUS)
        .child(
            Label::new("Task Progress")
                .text_sm()
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(rgb(TEXT_PRIMARY))
        )
        .child(
            v_flex()
                .gap(SPACING_SM)
                .w_full()
                .child({
                    let width_px = (percentage as f32 / 100.0) * 300.0;
                    div()
                        .w_full()
                        .h(px(8.0))
                        .bg(rgb(BG_SECONDARY))
                        .rounded(px(4.0))
                        .overflow_hidden()
                        .child(
                            div()
                                .h_full()
                                .bg(rgb(STATUS_SUCCESS))
                                .w(px(width_px))
                        )
                })
                .child(
                    h_flex()
                        .justify_between()
                        .w_full()
                        .child(
                            Label::new(format!("{}% Complete", percentage))
                                .text_xs()
                                .text_color(rgb(TEXT_PRIMARY))
                                .font_weight(FontWeight::SEMIBOLD)
                        )
                        .child(
                            Label::new(format!("{} remaining", remaining))
                                .text_xs()
                                .text_color(rgb(TEXT_SECONDARY))
                        )
                )
        )
}
