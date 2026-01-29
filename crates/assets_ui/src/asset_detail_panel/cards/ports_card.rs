use data::models::AssetNode;
use gpui::*;
use gpui_component::{h_flex, label::Label, v_flex};
use ui::theme::*;

use crate::config::ui_labels::asset_detail;

/// Open ports display card
pub struct PortsCard;

impl PortsCard {
    pub fn render(node: &AssetNode) -> impl IntoElement {
        v_flex()
            .gap_2()
            .child(
                Label::new(asset_detail::OPEN_PORTS)
                    .text_xs()
                    .text_color(rgb(TEXT_SECONDARY))
                    .font_weight(FontWeight::BOLD),
            )
            .child(
                h_flex()
                    .gap_2()
                    .flex_wrap()
                    .children(node.open_ports.iter().map(|port| {
                        div()
                            .px_2()
                            .py_1()
                            .bg(rgb(0xf3f4f6))
                            .rounded_md()
                            .border_1()
                            .border_color(rgb(0xe5e7eb))
                            .child(Label::new(port.to_string()).text_xs())
                    })),
            )
            // Protocol info
            .child(
                v_flex()
                    .gap_1()
                    .child(
                        Label::new(format!("{}: HTTPS", asset_detail::PROTOCOL_LABEL))
                            .text_xs()
                            .text_color(rgb(TEXT_MUTED)),
                    )
                    .child(
                        Label::new(format!(
                            "{}: {}",
                            asset_detail::LAST_SCAN_LABEL,
                            node.scan_progress
                                .last_scan
                                .as_ref()
                                .map(|s| s.format("%Y-%m-%d").to_string())
                                .unwrap_or_else(|| asset_detail::NEVER_SCANNED.to_string())
                        ))
                        .text_xs()
                        .text_color(rgb(TEXT_MUTED)),
                    ),
            )
    }
}
