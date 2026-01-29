use data::models::AssetNode;
use gpui::*;
use gpui_component::{h_flex, label::Label, v_flex, Icon, IconName};
use ui::theme::*;

use crate::config::{theme_ext::*, ZoneTypeExt};

/// Zone information card
pub struct ZoneCard;

impl ZoneCard {
    pub fn render(node: &AssetNode) -> impl IntoElement {
        let zone_config = node.zone.config();

        v_flex()
            .p_3()
            .rounded_lg()
            .bg(rgb(CARD_ZONE_BG))
            .border_1()
            .border_color(rgb(CARD_ZONE_BORDER))
            .gap_2()
            .child(
                h_flex()
                    .gap_2()
                    .items_center()
                    .child(
                        Icon::new(IconName::CircleCheck)
                            .size(px(20.0))
                            .text_color(rgb(zone_config.primary_color)),
                    )
                    .child(
                        Label::new(zone_config.short_name.to_string())
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(zone_config.primary_color)),
                    ),
            )
            .child(
                Label::new(zone_config.layer_name.to_string())
                    .text_sm()
                    .text_color(rgb(TEXT_PRIMARY)),
            )
    }
}
