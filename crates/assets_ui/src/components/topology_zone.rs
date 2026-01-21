use gpui::*;
use gpui_component::{label::Label, v_flex, h_flex, IconName};
use ui::theme::*;
use data::models::{ZoneType, AssetNode};

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
        format!("{}", self.assets.len())
    }

    fn asset_label_text(&self) -> &'static str {
        if self.assets.len() == 1 {
            "资产"
        } else {
            "资产"
        }
    }
}

pub fn render_topology_zone(zone: &TopologyZone) -> impl IntoElement {
    v_flex()
        .flex_1()
        .size_full()
        .gap_0()
        .rounded_lg()
        .bg(rgb(zone.bg_color))
        .border_1()
        .border_color(rgb(BORDER_COLOR))
        .overflow_hidden()
        .child(
            // 分区卡片头
            h_flex()
                .flex_none()
                .w_full()
                .gap_2()
                .p_3()
                .bg(rgb(BG_PRIMARY))
                .border_b_1()
                .border_color(rgb(BORDER_COLOR))
                .items_center()
                .child(
                    // 区域图标
                    zone.icon.clone()
                )
                .child(
                    // 区域标签和描述
                    v_flex()
                        .gap_1()
                        .flex_1()
                        .child(
                            Label::new(zone.name.clone())
                                .text_sm()
                                .font_weight(FontWeight::SEMIBOLD)
                        )
                        .child(
                            Label::new(zone.description.clone())
                                .text_xs()
                                .text_color(rgb(TEXT_MUTED))
                        )
                )
                .child(
                    // 资产数量
                    v_flex()
                        .items_center()
                        .justify_center()
                        .gap_0()
                        .child(
                            Label::new(zone.asset_count_text())
                                .text_sm()
                                .font_weight(FontWeight::SEMIBOLD)
                        )
                        .child(
                            Label::new(zone.asset_label_text())
                                .text_xs()
                                .text_color(rgb(TEXT_MUTED))
                        )
                )
                .child(
                    // 添加按钮
                    div()
                        .text_center()
                        .text_sm()
                        .text_color(rgb(ACCENT_BLUE))
                        .cursor_pointer()
                        .child("+")
                )
        )
        .child(
            // 分区内容区域 (资产节点) - 占据剩余所有空间
            v_flex()
                .flex_1()
                .size_full()
                .p_4()
                .gap_3()
                .items_center()
                .justify_center()
                .children(
                    zone.assets.iter().map(|asset| {
                        render_asset_node(asset)
                    })
                )
        )
}

fn render_asset_node(node: &AssetNode) -> impl IntoElement {
    let node_color = get_asset_color(&node.asset_type);
    let severity_rgb = get_severity_color(&node.severity);
    
    v_flex()
        .items_center()
        .gap_2()
        .child(
            h_flex()
                .items_center()
                .justify_center()
                // 外圈: 进度环效果
                .w(px(56.0))
                .h(px(56.0))
                .rounded_full()
                .border_2()
                .border_color(rgb(severity_rgb))
                // 内圈: 资产颜色
                .child(
                    div()
                        .w(px(44.0))
                        .h(px(44.0))
                        .rounded_full()
                        .bg(rgb(node_color))
                        .border_2()
                        .border_color(rgb(0xffffff))
                )
        )
        .child(
            Label::new(node.name.clone())
                .text_xs()
                .text_center()
        )
}

fn get_asset_color(asset_type: &str) -> u32 {
    match asset_type {
        "UAV" => 0x2563eb,           // 蓝色
        "GCS" => 0x7c3aed,           // 紫色
        "Router" => 0x10b981,        // 绿色
        "Server" => 0xf97316,        // 橙色
        _ => 0x6b7280,               // 灰色
    }
}

fn get_severity_color(severity: &data::models::Severity) -> u32 {
    match severity {
        data::models::Severity::Critical => 0xef4444,  // 红色
        data::models::Severity::High => 0xf97316,      // 橙色
        data::models::Severity::Medium => 0xfbbf24,    // 黄色
        data::models::Severity::Low => 0x10b981,       // 绿色
        data::models::Severity::Info => 0x3b82f6,      // 蓝色
    }
}
