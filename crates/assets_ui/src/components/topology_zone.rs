use gpui::*;
use gpui_component::{label::Label, v_flex};
use ui::theme::*;
use data::models::{ZoneType, AssetNode};

#[derive(Clone)]
pub struct TopologyZone {
    pub zone: ZoneType,
    pub assets: Vec<AssetNode>,
}

impl TopologyZone {
    pub fn new(zone: ZoneType, assets: Vec<AssetNode>) -> Self {
        Self { zone, assets }
    }

    fn zone_title(&self) -> &'static str {
        match self.zone {
            ZoneType::Z1 => "Z1",
            ZoneType::Z2 => "Z2",
            ZoneType::Z3 => "Z3",
            ZoneType::Z4 => "Z4",
            ZoneType::Z5 => "Z5",
        }
    }

    fn zone_color(&self) -> u32 {
        match self.zone {
            ZoneType::Z1 => 0xe3f2fd,
            ZoneType::Z2 => 0xf1f8e9,
            ZoneType::Z3 => 0xfce4ec,
            ZoneType::Z4 => 0xfff3e0,
            ZoneType::Z5 => 0xf3e5f5,
        }
    }
}

pub fn render_topology_zone(zone: &TopologyZone) -> impl IntoElement {
    v_flex()
        .flex_1()
        .gap_2()
        .p_4()
        .rounded_lg()
        .bg(rgb(zone.zone_color()))
        .border_1()
        .border_color(rgb(BORDER_COLOR))
        .items_center()
        .child(
            Label::new(zone.zone_title())
                .text_sm()
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(rgb(TEXT_PRIMARY))
        )
        .children(
            zone.assets.iter().take(3).map(|_asset| {
                div()
                    .w(px(32.0))
                    .h(px(32.0))
                    .rounded_full()
                    .bg(rgb(0x2563eb))
                    .border_2()
                    .border_color(rgb(0xffffff))
                    .m_2()
                    .into_any_element()
            })
        )
}
