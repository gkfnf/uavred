use data::models::AssetNode;
use gpui::*;
use gpui_component::{label::Label, v_flex};
use ui::theme::*;

use crate::config::ui_labels::asset_detail;

/// Detected services card
pub struct ServicesCard;

impl ServicesCard {
    pub fn render(node: &AssetNode) -> impl IntoElement {
        v_flex()
            .flex_1()
            .gap_3()
            .child(
                Label::new(asset_detail::DETECTED_SERVICES)
                    .text_xs()
                    .text_color(rgb(TEXT_SECONDARY))
                    .font_weight(FontWeight::BOLD),
            )
            .children(Self::get_services(node))
    }

    fn get_services(node: &AssetNode) -> Vec<AnyElement> {
        if node.services.is_empty() {
            // Show default services when none detected
            asset_detail::DEFAULT_SERVICES
                .iter()
                .map(|service| {
                    div()
                        .px_3()
                        .py_2()
                        .bg(rgb(0xf3f4f6))
                        .rounded_md()
                        .border_1()
                        .border_color(rgb(0xe5e7eb))
                        .child(Label::new(service.to_string()).text_xs())
                        .into_any_element()
                })
                .collect()
        } else {
            node.services
                .iter()
                .map(|s| {
                    div()
                        .px_3()
                        .py_2()
                        .bg(rgb(0xf3f4f6))
                        .rounded_md()
                        .border_1()
                        .border_color(rgb(0xe5e7eb))
                        .child(Label::new(s.service_name.clone()).text_xs())
                        .into_any_element()
                })
                .collect()
        }
    }
}
