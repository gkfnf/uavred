use data::models::AssetNode;
use gpui::*;
use gpui_component::{label::Label, v_flex};
use ui::theme::*;

use crate::config::{theme_ext::*, ui_labels::asset_detail};

/// Owner/team card
pub struct OwnerCard;

impl OwnerCard {
    pub fn render(node: &AssetNode) -> impl IntoElement {
        v_flex()
            .p_3()
            .rounded_lg()
            .bg(rgb(CARD_STATUS_BG))
            .border_1()
            .border_color(rgb(CARD_DEFAULT_BORDER))
            .gap_2()
            .child(
                Label::new(asset_detail::OWNER_TEAM)
                    .text_xs()
                    .text_color(rgb(TEXT_SECONDARY))
                    .font_weight(FontWeight::BOLD),
            )
            .child(
                Label::new(node.owner.clone())
                    .text_xs()
                    .text_color(rgb(TEXT_PRIMARY)),
            )
    }
}
