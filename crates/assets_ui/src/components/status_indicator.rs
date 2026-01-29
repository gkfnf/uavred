use data::models::AssetStatus;
use gpui::*;
use gpui_component::{h_flex, label::Label};
use ui::theme::*;

pub fn render_status_indicator(status: &AssetStatus) -> impl IntoElement {
    let color = match status {
        AssetStatus::Online => STATUS_SUCCESS,
        AssetStatus::Offline => TEXT_MUTED,
        AssetStatus::Unknown => STATUS_WARNING,
        AssetStatus::Maintenance => ACCENT_BLUE,
        AssetStatus::Busy => STATUS_WARNING,
        AssetStatus::Error => 0xef4444,
    };

    let text = match status {
        AssetStatus::Online => "Online",
        AssetStatus::Offline => "Offline",
        AssetStatus::Unknown => "Unknown",
        AssetStatus::Maintenance => "Maintenance",
        AssetStatus::Busy => "Busy",
        AssetStatus::Error => "Error",
    };

    h_flex()
        .gap_2()
        .items_center()
        .px_2()
        .py_1()
        .rounded_md()
        .child(div().w_2().h_2().rounded_full().bg(rgb(color)))
        .child(
            Label::new(text)
                .text_xs()
                .font_weight(FontWeight::MEDIUM)
                .text_color(rgb(color)),
        )
}
