use data::models::AssetNode;
use gpui::*;
use gpui_component::{label::Label, v_flex};
use ui::theme::*;

use crate::config::{theme_ext::*, ui_labels::asset_detail};

/// Vulnerability statistics card
pub struct VulnStatsCard;

impl VulnStatsCard {
    pub fn render(node: &AssetNode) -> impl IntoElement {
        let has_vulns = node.vulnerabilities_count > 0;
        let count_color = if has_vulns { 0xdc2626 } else { 0x10b981 };

        v_flex()
            .mt_4()
            .p_3()
            .rounded_lg()
            .bg(rgb(CARD_VULN_BG))
            .border_1()
            .border_color(rgb(CARD_VULN_BORDER))
            .gap_2()
            .child(
                Label::new(asset_detail::VULNERABILITY_STATS)
                    .text_xs()
                    .text_color(rgb(TEXT_SECONDARY)),
            )
            .child(
                Label::new(node.vulnerabilities_count.to_string())
                    .text_2xl()
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(count_color)),
            )
            .child(
                Label::new(asset_detail::DETECTED_VULNS)
                    .text_xs()
                    .text_color(rgb(TEXT_MUTED)),
            )
    }
}
