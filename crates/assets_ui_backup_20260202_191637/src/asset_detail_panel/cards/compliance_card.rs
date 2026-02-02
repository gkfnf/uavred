use data::models::{AssetNode, ComplianceStatus};
use gpui::*;
use gpui_component::{h_flex, label::Label, v_flex};
use ui::theme::*;

use crate::config::{theme_ext::*, ui_labels::asset_detail};

/// Compliance standards card
pub struct ComplianceCard;

impl ComplianceCard {
    pub fn render(node: &AssetNode) -> impl IntoElement {
        v_flex()
            .p_3()
            .rounded_lg()
            .bg(rgb(CARD_COMPLIANCE_BG))
            .border_1()
            .border_color(rgb(CARD_COMPLIANCE_BORDER))
            .gap_2()
            .child(
                Label::new(asset_detail::COMPLIANCE_STANDARDS)
                    .text_xs()
                    .text_color(rgb(TEXT_SECONDARY))
                    .font_weight(FontWeight::BOLD),
            )
            .child(Self::render_badges(&node.compliance_standards))
    }

    fn render_badges(standards: &[data::models::ComplianceStandard]) -> impl IntoElement {
        // Default standards if none exist
        let default_standards = [
            data::models::ComplianceStandard {
                name: "ISO 27001".to_string(),
                status: ComplianceStatus::Compliant,
                last_audit: None,
            },
            data::models::ComplianceStandard {
                name: "PCI DSS".to_string(),
                status: ComplianceStatus::Compliant,
                last_audit: None,
            },
        ];

        let standards_to_render = if standards.is_empty() {
            &default_standards[..]
        } else {
            standards
        };

        h_flex()
            .gap_2()
            .flex_wrap()
            .children(standards_to_render.iter().map(|std| {
                let (bg_color, text_color) = match std.status {
                    ComplianceStatus::Compliant => (0xd1fae5, 0x059669),
                    ComplianceStatus::NonCompliant => (0xfecaca, 0xdc2626),
                    ComplianceStatus::Pending => (0xfef3c7, 0xd97706),
                    ComplianceStatus::NotApplicable => (0xe5e7eb, 0x6b7280),
                };

                div()
                    .px_2()
                    .py_1()
                    .bg(rgb(bg_color))
                    .rounded_md()
                    .border_1()
                    .border_color(rgb(text_color))
                    .child(
                        Label::new(std.name.clone())
                            .text_xs()
                            .text_color(rgb(text_color))
                            .font_weight(FontWeight::MEDIUM),
                    )
            }))
    }
}
