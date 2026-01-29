use data::models::AssetNode;
use gpui::*;
use gpui_component::{label::Label, v_flex};
use ui::theme::*;

use crate::config::{theme_ext::*, ui_labels::asset_detail};

/// Business purpose card
pub struct BusinessCard;

impl BusinessCard {
    pub fn render(node: &AssetNode) -> impl IntoElement {
        v_flex()
            .p_3()
            .rounded_lg()
            .bg(rgb(CARD_BUSINESS_BG))
            .border_1()
            .border_color(rgb(CARD_BUSINESS_BORDER))
            .gap_2()
            .child(
                Label::new(asset_detail::BUSINESS_PURPOSE)
                    .text_xs()
                    .text_color(rgb(0x0284c7))
                    .font_weight(FontWeight::BOLD),
            )
            .child(
                Label::new(node.business_purpose.clone())
                    .text_xs()
                    .text_color(rgb(TEXT_PRIMARY)),
            )
    }
}
