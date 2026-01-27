use data::models::{AssetNode, ZoneType};
use gpui::*;
use gpui_component::{h_flex, label::Label, v_flex, Icon, IconName, Sizable};
use ui::theme::*;

#[derive(Clone)]
pub struct TopologyZone {
    pub zone: ZoneType,
    pub assets: Vec<AssetNode>,
    pub name: String,
    pub description: String,
    pub bg_color: u32,
    pub icon: IconName,
}

impl TopologyZone {
    pub fn new(
        zone: ZoneType,
        assets: Vec<AssetNode>,
        name: String,
        description: String,
        bg_color: u32,
        icon: IconName,
    ) -> Self {
        Self {
            zone,
            assets,
            name,
            description,
            bg_color,
            icon,
        }
    }

    fn asset_count_text(&self) -> String {
        format!("{} 资产", self.assets.len())
    }
}

pub fn render_topology_zone_bg(zone: &TopologyZone) -> impl IntoElement {
    let zone_color = get_zone_base_color(&zone.zone);

    v_flex()
        .flex_1()
        .size_full()
        .gap_0()
        .bg(rgb(zone.bg_color))
        .border_r_1()
        .border_color(rgb(BORDER_COLOR))
        .child(
            // 分区卡片头
            h_flex()
                .flex_none()
                .w_full()
                .gap_2()
                .p_3()
                .items_center()
                .child(
                    // 区域图标 (带 Zx 文字的六边形)
                    div()
                        .relative()
                        .child(
                            Icon::new(IconName::CircleCheck)
                                .with_size(px(28.0))
                                .text_color(rgb(zone_color)),
                        )
                        .child(
                            div()
                                .absolute()
                                .size_full()
                                .flex()
                                .items_center()
                                .justify_center()
                                .child(
                                    Label::new(zone.name.clone())
                                        .text_xs()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(rgb(zone_color)),
                                ),
                        ),
                )
                .child(
                    // 区域描述
                    Label::new(zone.description.clone())
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(TEXT_PRIMARY)),
                )
                .child(div().flex_1())
                .child(
                    // 添加按钮
                    div()
                        .text_center()
                        .text_lg()
                        .text_color(rgb(zone_color))
                        .cursor_pointer()
                        .child("+"),
                ),
        )
        .child(
            // 资产数量统计
            div().px_3().pb_2().child(
                Label::new(zone.asset_count_text())
                    .text_xs()
                    .text_color(rgb(TEXT_SECONDARY)),
            ),
        )
}

pub fn render_asset_node_at(
    node: &AssetNode,
    pos: &crate::topology_canvas::NodePosition,
) -> impl IntoElement {
    let node_color = get_asset_color(&node.asset_type);
    let severity_color = node.severity.color_hex();

    div().absolute().top(px(pos.y)).left(px(pos.x)).child(
        v_flex()
            .items_center()
            .gap_2()
            .child(
                div()
                    .relative()
                    .size(px(48.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    // 外圈: 严重程度环
                    .child(
                        div()
                            .absolute()
                            .size(px(48.0))
                            .rounded_full()
                            .border_2()
                            .border_color(rgb(severity_color)),
                    )
                    // 内圈: 资产节点
                    .child(
                        div()
                            .size(px(36.0))
                            .rounded_full()
                            .bg(rgb(node_color))
                            .border_2()
                            .border_color(rgb(0xffffff))
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(
                                // 节点中心点
                                div().size(px(6.0)).rounded_full().bg(rgb(0xffffff)),
                            ),
                    )
                    // 风险状态指示点
                    .child(
                        div()
                            .absolute()
                            .top_0()
                            .right_0()
                            .size(px(14.0))
                            .rounded_full()
                            .bg(rgb(severity_color))
                            .border_2()
                            .border_color(rgb(0xffffff)),
                    ),
            )
            .child(
                v_flex().items_center().child(
                    Label::new(node.name.clone())
                        .text_xs()
                        .text_center()
                        .font_weight(FontWeight::MEDIUM),
                ),
            )
            .on_mouse_down(MouseButton::Left, move |_, _, _| {
                // Selection logic handled by TopologyCanvas
            }),
    )
}

fn get_zone_base_color(zone_type: &ZoneType) -> u32 {
    match zone_type {
        ZoneType::Z1 => 0x2563eb, // 蓝色
        ZoneType::Z2 => 0x10b981, // 绿色
        ZoneType::Z3 => 0x7c3aed, // 紫色
        ZoneType::Z4 => 0xf97316, // 橙色
        ZoneType::Z5 => 0xef4444, // 红色
    }
}

fn get_asset_color(asset_type: &str) -> u32 {
    match asset_type {
        "UAV" => 0x10b981,    // 绿色
        "GCS" => 0x2563eb,    // 蓝色
        "Router" => 0x10b981, // 绿色
        "Server" => 0x7c3aed, // 紫色
        _ => 0x6b7280,        // 灰色
    }
}
