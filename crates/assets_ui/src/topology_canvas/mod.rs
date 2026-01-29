//! Topology Canvas - Interactive network asset topology visualization
//!
//! ## Architecture
//!
//! The canvas is organized into 5 security zones (Z1-Z5), each with:
//! - Independent viewport (pan/zoom)
//! - Mouse wheel zoom
//! - Click+drag pan
//! - Node rendering via GPUI canvas API
//! - Connection lines between nodes

mod camera;
mod zone_canvas;

use gpui::*;
use gpui::prelude::FluentBuilder;
use gpui_component::{h_flex, label::Label, v_flex, Icon, IconName};

use data::models::{AssetNode, ZoneType};
use ui::theme::*;

use crate::config::ZoneTypeExt;
use crate::repository::MockAssetRepository;

pub use zone_canvas::{NodeVirtualPos, ZoneCanvas};

impl EventEmitter<AssetSelectedEvent> for TopologyCanvas {}

/// Event emitted when an asset node is selected
#[derive(Clone, Debug)]
pub enum AssetSelectedEvent {
    NodeSelected(String),
}

/// Main topology canvas component
pub struct TopologyCanvas {
    zones: Vec<ZoneCanvas>,
    selected_node_id: Option<String>,
    hovered_node_id: Option<String>,
}

impl TopologyCanvas {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        let repository = MockAssetRepository::new();
        let zone_virtual_width = 400.0;
        let zone_virtual_height = 600.0;

        let zones = vec![
            ZoneCanvas::new(ZoneType::Z1, &repository, zone_virtual_width, zone_virtual_height),
            ZoneCanvas::new(ZoneType::Z2, &repository, zone_virtual_width, zone_virtual_height),
            ZoneCanvas::new(ZoneType::Z3, &repository, zone_virtual_width, zone_virtual_height),
            ZoneCanvas::new(ZoneType::Z4, &repository, zone_virtual_width, zone_virtual_height),
            ZoneCanvas::new(ZoneType::Z5, &repository, zone_virtual_width, zone_virtual_height),
        ];

        Self {
            zones,
            selected_node_id: None,
            hovered_node_id: None,
        }
    }

    /// Get all nodes across all zones
    pub fn get_nodes(&self) -> Vec<&AssetNode> {
        self.zones.iter().flat_map(|z| z.nodes()).collect()
    }

    /// Get total node count
    pub fn node_count(&self) -> usize {
        self.zones.iter().map(|z| z.node_count()).sum()
    }

    /// Get total connection count
    pub fn connection_count(&self) -> usize {
        self.zones.iter().map(|z| z.connection_count()).sum()
    }

    /// Reset all zones to fit their content
    pub fn reset_all_views(&mut self) {
        for zone in &mut self.zones {
            zone.reset_view();
        }
    }

    /// Render zone header bar
    fn render_zone_headers(&self) -> impl IntoElement {
        h_flex()
            .w_full()
            .h(px(55.0))
            .border_b_1()
            .border_color(rgb(BORDER_COLOR))
            .bg(rgb(0xffffff))
            .children(self.zones.iter().enumerate().map(|(idx, zone)| {
                self.render_zone_header(zone, idx)
            }).collect::<Vec<_>>())
    }

    /// Render individual zone header
    fn render_zone_header(&self, zone: &ZoneCanvas, idx: usize) -> AnyElement {
        let config = zone.zone().config();
        let count = zone.node_count();
        let is_last = idx == 4;

        h_flex()
            .flex_1()
            .h_full()
            .when(!is_last, |this: gpui::Div| this.border_r_1().border_color(rgb(BORDER_COLOR)))
            .px_3()
            .py_2()
            .gap_2()
            .child(
                Icon::new(IconName::CircleCheck)
                    .size(px(16.0))
                    .text_color(rgb(config.primary_color))
            )
            .child(
                v_flex()
                    .gap_1()
                    .child(
                        h_flex()
                            .gap_2()
                            .child(
                                Label::new(config.short_name)
                                    .text_sm()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(config.primary_color))
                            )
                            .child(
                                Label::new(config.layer_name)
                                    .text_xs()
                                    .text_color(rgb(TEXT_PRIMARY))
                            )
                    )
                    .child(
                        Label::new(format!("{} 资产", count))
                            .text_xs()
                            .text_color(rgb(TEXT_SECONDARY))
                    )
            )
            .child(div().flex_1())
            .child(
                div()
                    .text_lg()
                    .text_color(rgb(config.primary_color))
                    .cursor_pointer()
                    .child("+")
            )
            .into_any_element()
    }

    /// Render zone canvases
    fn render_zone_canvases(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let selected_id = self.selected_node_id.clone();
        let hovered_id = self.hovered_node_id.clone();

        h_flex()
            .flex_1()
            .w_full()
            .children(self.zones.iter_mut().enumerate().map(move |(zone_idx, zone)| {
                zone.render_canvas(zone_idx, &selected_id, &hovered_id, cx)
            }).collect::<Vec<_>>())
    }
}

impl Render for TopologyCanvas {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .child(self.render_zone_headers())
            .child(self.render_zone_canvases(cx))
    }
}
