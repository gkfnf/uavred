use data::models::AssetNode;
use gpui::*;
use gpui_component::{h_flex, label::Label, IconName};
use ui::theme::*;

pub fn render_asset_header(node: &AssetNode) -> impl IntoElement {
    let icon = match node.asset_type.as_str() {
        "UAV" => IconName::Globe,
        "GCS" => IconName::LayoutDashboard,
        "Router" => IconName::Network,
        "Server" => IconName::HardDrive,
        _ => IconName::SquareTerminal,
    };

    let has_vulns = node.vulnerabilities_count > 0;

    h_flex()
        .gap_3()
        .items_center()
        .py_3()
        .px_4()
        .border_b_1()
        .border_color(rgb(BORDER_COLOR))
        .bg(rgb(BG_PRIMARY))
        .child(icon)
        .child(
            h_flex()
                .flex_1()
                .flex_col()
                .gap_1()
                .child(
                    Label::new(node.name.clone())
                        .text_lg()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(TEXT_PRIMARY)),
                )
                .child(if has_vulns {
                    h_flex()
                        .gap_2()
                        .items_center()
                        .child(
                            Label::new(node.ip_address.clone())
                                .text_sm()
                                .text_color(rgb(TEXT_SECONDARY)),
                        )
                        .child(
                            h_flex()
                                .gap_1()
                                .items_center()
                                .px_2()
                                .rounded_sm()
                                .bg(rgb(SEVERITY_HIGH))
                                .child(
                                    Label::new(format!(
                                        "{} vuln{}",
                                        node.vulnerabilities_count,
                                        if node.vulnerabilities_count == 1 {
                                            ""
                                        } else {
                                            "s"
                                        }
                                    ))
                                    .text_xs()
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(rgb(0xffffff)),
                                ),
                        )
                        .into_any_element()
                } else {
                    h_flex()
                        .gap_2()
                        .items_center()
                        .child(
                            Label::new(node.ip_address.clone())
                                .text_sm()
                                .text_color(rgb(TEXT_SECONDARY)),
                        )
                        .into_any_element()
                }),
        )
}
