// Quick action buttons component

use gpui::*;
use gpui_component::{button::Button, h_flex, IconName};
use ui::theme::*;

/// Render quick action buttons
pub fn render_quick_actions<T: 'static>(
    cx: &mut Context<T>,
    on_new_task: impl Fn(&mut T, &mut Context<T>) + 'static,
    on_run_scan: impl Fn(&mut T, &mut Context<T>) + 'static,
    on_export: impl Fn(&mut T, &mut Context<T>) + 'static,
) -> impl IntoElement {
    h_flex()
        .gap(SPACING_MD)
        .w_full()
        .p(PADDING_LG)
        .bg(rgb(BG_SECONDARY))
        .rounded(BORDER_RADIUS)
        .child(
            Button::new("quick-action-new-task")
                .label("New Task")
                .icon(IconName::Plus)
                .on_click(cx.listener(move |this, _, _, cx| {
                    on_new_task(this, cx);
                }))
        )
        .child(
            Button::new("quick-action-run-scan")
                .label("Run Scan")
                .icon(IconName::SquareTerminal)
                .on_click(cx.listener(move |this, _, _, cx| {
                    on_run_scan(this, cx);
                }))
        )
        .child(
            Button::new("quick-action-export")
                .label("Export Report")
                .icon(IconName::ArrowDown)
                .on_click(cx.listener(move |this, _, _, cx| {
                    on_export(this, cx);
                }))
        )
        .child(div().flex_1())
}
