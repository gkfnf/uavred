use gpui::*;
use gpui_component::{h_flex, label::Label, v_flex, IconName};

mod components;
mod asset_detail_panel;
mod topology_canvas;
mod events;

pub use asset_detail_panel::AssetDetailPanel;
pub use topology_canvas::{TopologyCanvas, AssetSelectedEvent};
pub use components::*;
pub use events::AssetActionEvent;

use ui::theme::*;
use data::models::AssetNode;

pub struct AssetsPanel {
    topology_expanded: bool,
    details_expanded: bool,
    topology_canvas: Entity<TopologyCanvas>,
    asset_detail_panel: Entity<AssetDetailPanel>,
    selected_asset: Option<AssetNode>,
    _subscriptions: Vec<Subscription>,
}

impl AssetsPanel {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let topology_canvas = cx.new(TopologyCanvas::new);
        let asset_detail_panel = cx.new(AssetDetailPanel::new);

        // Subscribe to asset selection events from topology canvas
        let asset_detail_panel_clone = asset_detail_panel.clone();
        let subscription = cx.subscribe(
            &topology_canvas,
            move |this, topology, event, cx| {
                let AssetSelectedEvent::NodeSelected(node_id) = event;
                // Find the node in the topology canvas and clone it
                let node = topology.read(cx).nodes.iter()
                    .find(|n| n.id == *node_id)
                    .cloned();
                
                if let Some(node) = node {
                    // Update detail panel with selected node
                    asset_detail_panel_clone.update(cx, |panel, cx| {
                        panel.set_node(node.clone(), cx);
                    });
                    // Update local state
                    this.selected_asset = Some(node);
                    this.details_expanded = true;
                    cx.notify();
                }
            },
        );

        Self {
            topology_expanded: true,
            details_expanded: false,
            topology_canvas,
            asset_detail_panel,
            selected_asset: None,
            _subscriptions: vec![subscription],
        }
    }

    fn toggle_topology(&mut self, cx: &mut Context<Self>) {
        self.topology_expanded = !self.topology_expanded;
        cx.notify();
    }

    fn toggle_details(&mut self, cx: &mut Context<Self>) {
        self.details_expanded = !self.details_expanded;
        cx.notify();
    }
}

impl Render for AssetsPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .gap_0()
            .bg(rgb(BG_PRIMARY))
            .child(
                // Row 1: Network Topology
                v_flex()
                    .flex_none()
                    .gap_0()
                    .bg(rgb(BG_CARD))
                    .border_b_1()
                    .border_color(rgb(BORDER_COLOR))
                    .child(
                        h_flex()
                            .gap_2()
                            .p_3()
                            .items_center()
                            .bg(rgb(BG_PRIMARY))
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                this.toggle_topology(cx);
                            }))
                            .cursor_pointer()
                            .child(
                                if self.topology_expanded {
                                    IconName::ChevronDown
                                } else {
                                    IconName::ChevronRight
                                }
                            )
                            .child(IconName::Network)
                            .child(
                                Label::new("网络拓扑 - 业务层级视图")
                                    .text_sm()
                                    .font_weight(FontWeight::SEMIBOLD)
                            )
                    )
                    .child(
                        if self.topology_expanded {
                            self.topology_canvas.clone().into_any_element()
                        } else {
                            div().into_any_element()
                        }
                    )
            )
            .child(
                // Row 2: Asset Details
                v_flex()
                    .flex_1()
                    .gap_0()
                    .bg(rgb(BG_CARD))
                    .child(
                        h_flex()
                            .gap_2()
                            .p_3()
                            .items_center()
                            .bg(rgb(BG_PRIMARY))
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                this.toggle_details(cx);
                            }))
                            .cursor_pointer()
                            .child(
                                if self.details_expanded {
                                    IconName::ChevronDown
                                } else {
                                    IconName::ChevronRight
                                }
                            )
                            .child(IconName::File)
                            .child(
                                Label::new("资产详情")
                                    .text_sm()
                                    .font_weight(FontWeight::SEMIBOLD)
                            )
                    )
                    .child(
                        if self.details_expanded && self.selected_asset.is_some() {
                            self.asset_detail_panel.clone().into_any_element()
                        } else if self.details_expanded && self.selected_asset.is_none() {
                            div()
                                .flex_1()
                                .items_center()
                                .justify_center()
                                .p_6()
                                .child(
                                    Label::new("选择一个资产来查看详情")
                                        .text_sm()
                                        .text_color(rgb(TEXT_MUTED))
                                )
                                .into_any_element()
                        } else {
                            div().into_any_element()
                        }
                    )
            )
    }
}
