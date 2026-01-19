use gpui::*;
use gpui_component::{h_flex, label::Label, v_flex, IconName};
use ui::theme::*;
use data::models::AssetNode;

use crate::components::{
    render_asset_header, render_risk_badge, render_status_indicator, render_port_list, PortItem,
    render_info_card,
};

pub struct AssetDetailPanel {
    selected_node: Option<AssetNode>,
}

impl AssetDetailPanel {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self {
            selected_node: None,
        }
    }

    pub fn set_node(&mut self, node: AssetNode, cx: &mut Context<Self>) {
        self.selected_node = Some(node);
        cx.notify();
    }

    pub fn clear_node(&mut self, cx: &mut Context<Self>) {
        self.selected_node = None;
        cx.notify();
    }

    fn render_section_title(title: impl Into<SharedString>) -> Div {
        let title: SharedString = title.into();
        div()
            .mt_4()
            .mb_2()
            .child(
                Label::new(title)
                    .text_sm()
                    .text_color(rgb(TEXT_SECONDARY))
                    .font_weight(FontWeight::SEMIBOLD),
            )
    }
}

impl Render for AssetDetailPanel {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if let Some(node) = self.selected_node.clone() {
            v_flex()
                .size_full()
                .gap_0()
                .bg(rgb(BG_CARD))
                .rounded_lg()
                .overflow_hidden()
                .child(render_asset_header(&node).into_any_element())
                .child(
                    v_flex()
                        .flex_1()
                        .gap_3()
                        .p_4()
                        .overflow_hidden()
                        .children(vec![
                            // Basic Information Section
                            Self::render_section_title("Basic Information").into_any_element(),
                            h_flex()
                                .gap_2()
                                .child(render_info_card("ID", node.id.clone()))
                                .child(render_info_card("Type", node.asset_type.clone()))
                                .into_any_element(),
                            h_flex()
                                .gap_2()
                                .child(render_info_card("MAC Address", node.mac_address.clone().unwrap_or_else(|| "N/A".to_string())))
                                .child(render_info_card("Manufacturer", node.manufacturer.clone().unwrap_or_else(|| "N/A".to_string())))
                                .into_any_element(),
                            h_flex()
                                .gap_2()
                                .child(render_info_card("Firmware", node.firmware_version.clone().unwrap_or_else(|| "N/A".to_string())))
                                .child(render_info_card("Location", node.location.clone().unwrap_or_else(|| "N/A".to_string())))
                                .into_any_element(),
                            
                            // Security Section
                            Self::render_section_title("Security & Risk").into_any_element(),
                            h_flex()
                                .gap_2()
                                .items_center()
                                .child(render_risk_badge(&node.severity, node.risk_score).into_any_element())
                                .child(render_status_indicator(&node.status).into_any_element())
                                .into_any_element(),
                            render_info_card("Vulnerabilities", &format!("{} found", node.vulnerabilities_count))
                                .into_any_element(),

                            // Network Information Section
                            Self::render_section_title("Network Information").into_any_element(),
                            h_flex()
                                .gap_2()
                                .child(render_info_card("Zone", format!("{:?}", node.zone)))
                                .child(render_info_card("Open Ports", format!("{}", node.open_ports_count())))
                                .into_any_element(),
                            render_info_card("Services", &format!("{} available", node.services_count()))
                                .into_any_element(),

                            // Open Ports
                            Self::render_section_title("Open Ports").into_any_element(),
                            {
                                let ports: Vec<_> = node.open_ports.iter().map(|port| PortItem {
                                    port: *port,
                                    protocol: "TCP".to_string(),
                                    service: None,
                                }).collect();
                                render_port_list(&ports).into_any_element()
                            },

                            // Owner & Department Section
                            Self::render_section_title("Owner & Department").into_any_element(),
                            h_flex()
                                .gap_2()
                                .child(render_info_card("Owner", node.owner.clone()))
                                .child(render_info_card("Department", node.department.clone().unwrap_or_else(|| "N/A".to_string())))
                                .into_any_element(),
                            render_info_card("Business Purpose", node.business_purpose.clone())
                                .into_any_element(),

                            // Scan Status Section
                            Self::render_section_title("Scan Status").into_any_element(),
                            h_flex()
                                .gap_2()
                                .child(render_info_card("Last Scan", node.scan_progress.last_scan.clone().unwrap_or_else(|| "Never".to_string())))
                                .child(render_info_card("Next Scan", node.scan_progress.next_scan.clone().unwrap_or_else(|| "Not Scheduled".to_string())))
                                .into_any_element(),
                            h_flex()
                                .gap_2()
                                .child(render_info_card("Scan Type", &node.scan_progress.scan_type))
                                .child(render_info_card("Progress", &format!("{}%", node.scan_progress.percentage)))
                                .into_any_element(),

                            // Action Buttons
                            h_flex()
                                .gap_2()
                                .mt_4()
                                .pt_4()
                                .border_t_1()
                                .border_color(rgb(BORDER_COLOR))
                                .children(vec![
                                    div()
                                        .flex_1()
                                        .px_3()
                                        .py_2()
                                        .rounded_md()
                                        .bg(rgb(ACCENT_BLUE))
                                        .items_center()
                                        .justify_center()
                                        .child(
                                            Label::new("Scan")
                                                .text_sm()
                                                .font_weight(FontWeight::SEMIBOLD)
                                                .text_color(rgb(0xffffff)),
                                        ),
                                    div()
                                        .flex_1()
                                        .px_3()
                                        .py_2()
                                        .rounded_md()
                                        .bg(rgb(BG_SECONDARY))
                                        .items_center()
                                        .justify_center()
                                        .border_1()
                                        .border_color(rgb(BORDER_COLOR))
                                        .child(
                                            Label::new("Edit")
                                                .text_sm()
                                                .font_weight(FontWeight::SEMIBOLD)
                                                .text_color(rgb(TEXT_PRIMARY)),
                                        ),
                                    div()
                                        .flex_1()
                                        .px_3()
                                        .py_2()
                                        .rounded_md()
                                        .bg(rgb(BG_SECONDARY))
                                        .items_center()
                                        .justify_center()
                                        .border_1()
                                        .border_color(rgb(BORDER_COLOR))
                                        .child(IconName::Close),
                                ])
                                .into_any_element(),
                        ])
                )
        } else {
            v_flex()
                .size_full()
                .items_center()
                .justify_center()
                .bg(rgb(BG_CARD))
                .rounded_lg()
                .child(
                    h_flex()
                        .flex_col()
                        .items_center()
                        .gap_3()
                        .child(IconName::SquareTerminal)
                        .child(
                            Label::new("Select an asset to view details")
                                .text_sm()
                                .text_color(rgb(TEXT_MUTED)),
                        ),
                )
        }
    }
}
