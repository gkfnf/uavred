use gpui::*;
use gpui_component::{h_flex, label::Label, v_flex};
use ui::theme::*;

use crate::config::{theme_ext::*, ui_labels::asset_detail};

/// Credentials info card
pub struct CredentialsCard;

impl CredentialsCard {
    pub fn render() -> impl IntoElement {
        v_flex()
            .p_3()
            .rounded_lg()
            .bg(rgb(CARD_CREDENTIALS_BG))
            .border_1()
            .border_color(rgb(CARD_DEFAULT_BORDER))
            .gap_2()
            .child(
                Label::new(asset_detail::CREDENTIALS)
                    .text_xs()
                    .text_color(rgb(TEXT_SECONDARY))
                    .font_weight(FontWeight::BOLD),
            )
            .child(
                Label::new(asset_detail::CRED_TYPE)
                    .text_xs()
                    .text_color(rgb(TEXT_MUTED)),
            )
            .child(
                h_flex()
                    .gap_1()
                    .items_center()
                    .child(
                        div()
                            .size(px(6.0))
                            .rounded_full()
                            .bg(rgb(0x10b981)),
                    )
                    .child(
                        Label::new(asset_detail::CRED_VALID)
                            .text_xs()
                            .text_color(rgb(0x10b981)),
                    ),
            )
    }
}
