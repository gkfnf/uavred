use gpui::*;
use gpui_component::{h_flex, label::Label, v_flex, IconName};

use ui::theme::*;

use data::models::AssetNode;

pub struct NodeDetailPanel {
    selected_node: Option<AssetNode>,
}

impl NodeDetailPanel {
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

    fn get_asset_type_icon(asset_type: &str) -> IconName {
        match asset_type {
            "UAV" => IconName::Globe,
            "GCS" => IconName::LayoutDashboard,
            "Router" => IconName::Network,
            "Server" => IconName::HardDrive,
            _ => IconName::SquareTerminal,
        }
    }

    fn render_info_row(&self, label: &str, value: &str) -> Div {
        div()
            .flex_grow()
            .gap_3()
            .p_2()
            .rounded_md()
            .bg(rgb(BG_SECONDARY))
            .children(vec![
                Label::new(label.to_string())
                    .text_sm()
                    .text_color(rgb(TEXT_SECONDARY))
                    .font_weight(FontWeight::MEDIUM)
                    .w(px(100.)),
                Label::new(value.to_string())
                    .text_sm()
                    .text_color(rgb(TEXT_PRIMARY))
                    .flex_1(),
            ])
    }

    fn render_status_badge(&self, status: &str) -> Div {
        let (color, text) = match status {
            "Online" => (rgb(STATUS_SUCCESS), "ONLINE"),
            "Offline" => (rgb(TEXT_MUTED), "OFFLINE"),
            "Scanning" => (rgb(ACCENT_BLUE), "SCANNING"),
            "Maintenance" => (rgb(STATUS_WARNING), "MAINTENANCE"),
            _ => (rgb(TEXT_MUTED), "UNKNOWN"),
        };

        div()
            .px_3()
            .py_1()
            .rounded_md()
            .bg(rgb(BG_SECONDARY))
            .border_1()
            .border_color(color)
            .child(
                Label::new(text)
                    .text_xs()
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(color),
            )
    }
}

impl Render for NodeDetailPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if let Some(node) = &self.selected_node {
            let icon = Self::get_asset_type_icon(&node.asset_type);
            let status_str = match node.status {
                data::models::AssetStatus::Online => "Online",
                data::models::AssetStatus::Offline => "Offline",
                data::models::AssetStatus::Unknown => "Unknown",
                data::models::AssetStatus::Maintenance => "Maintenance",
            };
            let status_badge = self.render_status_badge(status_str);

            v_flex().size_full().gap_4().p_4().children(vec![
                div()
                    .flex_1()
                    .gap_2()
                    .pb_4()
                    .border_b_1()
                    .border_color(rgb(BORDER_COLOR))
                    .child(
                        h_flex().gap_2().items_center().child(icon).child(
                            h_flex()
                                .flex_1()
                                .gap_2()
                                .items_center()
                                .child(
                                    Label::new(node.name.as_str())
                                        .text_lg()
                                        .font_weight(FontWeight::SEMIBOLD),
                                )
                                .child(status_badge),
                        ),
                    ),
                div().flex_1().gap_3().children(vec![
                    Label::new("Basic Information")
                        .text_sm()
                        .text_color(rgb(TEXT_SECONDARY))
                        .font_weight(FontWeight::MEDIUM),
                    h_flex()
                        .gap_2()
                        .child(self.render_info_row("ID", &node.id))
                        .child(self.render_info_row("Type", &node.asset_type)),
                    h_flex()
                        .gap_2()
                        .child(self.render_info_row("IP Address", &node.ip_address))
                        .child(self.render_info_row(
                            "MAC Address",
                            node.mac_address.as_deref().unwrap_or("N/A"),
                        )),
                    h_flex().gap_2().children(vec![
                        self.render_info_row(
                            "Firmware",
                            node.firmware_version.as_deref().unwrap_or("N/A"),
                        ),
                        self.render_info_row(
                            "Manufacturer",
                            node.manufacturer.as_deref().unwrap_or("N/A"),
                        ),
                    ]),
                    self.render_info_row("Location", node.location.as_deref().unwrap_or("N/A")),
                    div().mt_4().child(
                        Label::new("Security Information")
                            .text_sm()
                            .text_color(rgb(TEXT_SECONDARY))
                            .font_weight(FontWeight::MEDIUM),
                    ),
                    h_flex().gap_2().children(vec![
                        self.render_info_row("Severity", node.severity.display_name()),
                        self.render_info_row("Risk Score", &format!("{}", node.risk_score)),
                    ]),
                    self.render_info_row(
                        "Vulnerabilities",
                        &format!("{}", node.vulnerabilities_count),
                    ),
                    div().mt_4().child(
                        Label::new("Owner Information")
                            .text_sm()
                            .text_color(rgb(TEXT_SECONDARY))
                            .font_weight(FontWeight::MEDIUM),
                    ),
                    h_flex().gap_2().children(vec![
                        self.render_info_row("Owner", &node.owner),
                        self.render_info_row(
                            "Department",
                            node.department.as_deref().unwrap_or("N/A"),
                        ),
                    ]),
                    self.render_info_row("Purpose", &node.business_purpose),
                    div().mt_4().child(
                        Label::new("Network Information")
                            .text_sm()
                            .text_color(rgb(TEXT_SECONDARY))
                            .font_weight(FontWeight::MEDIUM),
                    ),
                    self.render_info_row("Zone", node.zone.display_name()),
                    self.render_info_row("Open Ports", &format!("{}", node.open_ports_count())),
                    self.render_info_row("Services", &format!("{}", node.services_count())),
                    div().mt_4().child(
                        Label::new("Scan Status")
                            .text_sm()
                            .text_color(rgb(TEXT_SECONDARY))
                            .font_weight(FontWeight::MEDIUM),
                    ),
                    self.render_info_row(
                        "Last Scan",
                        node.scan_progress.last_scan.as_deref().unwrap_or("Never"),
                    ),
                    self.render_info_row(
                        "Next Scan",
                        node.scan_progress
                            .next_scan
                            .as_deref()
                            .unwrap_or("Not Scheduled"),
                    ),
                    h_flex().gap_2().children(vec![
                        self.render_info_row("Scan Type", &node.scan_progress.scan_type),
                        self.render_info_row(
                            "Progress",
                            &format!("{}%", node.scan_progress.percentage),
                        ),
                    ]),
                ]),
            ])
        } else {
            div().flex_1().items_center().justify_center().p_6().child(
                h_flex()
                    .flex_col()
                    .items_center()
                    .gap_2()
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
