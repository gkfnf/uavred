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
        .h_full()
        .gap_0()
        .bg(rgb(zone.bg_color))
        .border_r_1()
        .border_color(rgb(BORDER_COLOR))
        .child(
            // 分区卡片头 - 固定高度，不换行
            h_flex()
                .flex_none()
                .w_full()
                .h(px(48.0))
                .gap_2()
                .p_3()
                .items_center()
                .child(
                    // 区域图标 (带 Zx 文字)
                    div()
                        .relative()
                        .child(
                            Icon::new(IconName::CircleCheck)
                                .with_size(px(24.0))
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
                    // 区域描述 - 不换行
                    Label::new(zone.description.clone())
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(TEXT_PRIMARY))
                        .whitespace_nowrap(),
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
        // 分割线 - 分隔标题和节点区域
        .child(
            div()
                .w_full()
                .h(px(1.0))
                .bg(rgb(BORDER_COLOR)),
        )
        .child(
            // 资产数量统计
            div()
                .flex_none()
                .px_3()
                .py_2()
                .child(
                    Label::new(zone.asset_count_text())
                        .text_xs()
                        .text_color(rgb(TEXT_SECONDARY)),
                ),
        )
        // 节点区域分割线
        .child(
            div()
                .w_full()
                .h(px(1.0))
                .bg(rgb(BORDER_COLOR)),
        )
        // 节点渲染区域（占位，实际节点通过绝对定位渲染）
        .child(div().flex_1().w_full())
}

// render_asset_node_at 函数已移除 - 节点现在通过 canvas API 绘制

fn get_zone_base_color(zone_type: &ZoneType) -> u32 {
    match zone_type {
        ZoneType::Z1 => 0x2563eb, // 蓝色
        ZoneType::Z2 => 0x10b981, // 绿色
        ZoneType::Z3 => 0x7c3aed, // 紫色
        ZoneType::Z4 => 0xf97316, // 橙色
        ZoneType::Z5 => 0xef4444, // 红色
    }
}

#[allow(dead_code)]
fn get_asset_color(asset_type: &str) -> u32 {
    match asset_type {
        "UAV" => 0x10b981,    // 绿色
        "GCS" => 0x2563eb,    // 蓝色
        "Router" => 0x10b981, // 绿色
        "Server" => 0x7c3aed, // 紫色
        _ => 0x6b7280,        // 灰色
    }
}
