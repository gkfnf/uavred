use data::models::AssetNode;
use gpui::*;
use gpui_component::{label::Label, v_flex};
use ui::theme::*;

use crate::config::{theme_ext::*, ui_labels::asset_detail};

/// Asset status card
pub struct StatusCard;

impl StatusCard {
    pub fn render(node: &AssetNode) -> impl IntoElement {
        use data::models::AssetStatus;

        let (status_text, status_color) = match node.status {
            AssetStatus::Online => (asset_detail::STATUS_ONLINE, 0x10b981),
            AssetStatus::Offline => (asset_detail::STATUS_OFFLINE, TEXT_MUTED),
            AssetStatus::Unknown => (asset_detail::STATUS_UNKNOWN, TEXT_MUTED),
            AssetStatus::Maintenance => (asset_detail::STATUS_MAINTENANCE, TEXT_MUTED),
        };

        v_flex()
            .p_3()
            .rounded_lg()
            .bg(rgb(CARD_STATUS_BG))
            .border_1()
            .border_color(rgb(CARD_DEFAULT_BORDER))
            .gap_2()
            .child(
                Label::new(asset_detail::STATUS)
                    .text_xs()
                    .text_color(rgb(TEXT_SECONDARY))
                    .font_weight(FontWeight::BOLD),
            )
            .child(
                Label::new(status_text)
                    .text_sm()
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(status_color)),
            )
    }
}
