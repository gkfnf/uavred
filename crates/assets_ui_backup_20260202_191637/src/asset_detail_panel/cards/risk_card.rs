use data::models::AssetNode;
use gpui::*;
use gpui_component::{label::Label, v_flex};
use ui::theme::*;

use crate::config::{theme_ext::*, ui_labels::asset_detail};

/// Risk score card with progress bar
pub struct RiskCard;

impl RiskCard {
    pub fn render(node: &AssetNode) -> impl IntoElement {
        let risk_color = risk_color(node.risk_score as u8);

        v_flex()
            .p_3()
            .rounded_lg()
            .bg(rgb(CARD_RISK_BG))
            .border_1()
            .border_color(rgb(CARD_RISK_BORDER))
            .gap_2()
            .child(
                Label::new(asset_detail::RISK_SCORE)
                    .text_xs()
                    .text_color(rgb(TEXT_SECONDARY))
                    .font_weight(FontWeight::BOLD),
            )
            .child(
                Label::new(node.risk_score.to_string())
                    .text_2xl()
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0x7c3aed)),
            )
            .child(
                div()
                    .w_full()
                    .h(px(6.0))
                    .bg(rgb(0xe5e7eb))
                    .rounded_full()
                    .child(
                        div()
                            .w(relative(node.risk_score as f32 / 100.0))
                            .h_full()
                            .bg(rgb(risk_color))
                            .rounded_full(),
                    ),
            )
    }
}
