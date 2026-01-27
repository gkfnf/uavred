use gpui::*;
use gpui_component::{h_flex, label::Label, v_flex, IconName};

mod asset_detail_panel;
mod components;
mod events;
mod topology_canvas;

pub use asset_detail_panel::AssetDetailPanel;
pub use components::*;
pub use events::AssetActionEvent;
pub use topology_canvas::{AssetSelectedEvent, TopologyCanvas};

use data::models::AssetNode;
use ui::theme::*;

/// AssetsPanel - Top-level asset management container
///
/// Coordinates:
/// 1. TopologyCanvas - renders network asset topology
/// 2. AssetDetailPanel - displays selected asset details
///
/// Selection flow:
/// User clicks asset node → TopologyCanvas emits AssetSelectedEvent
/// → AssetsPanel subscribes and updates AssetDetailPanel
/// → Details panel shows asset information and action buttons
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
        let subscription = cx.subscribe(&topology_canvas, move |this, topology, event, cx| {
            let AssetSelectedEvent::NodeSelected(node_id) = event;
            // Find the node in the topology canvas and clone it
            let node = topology
                .read(cx)
                .nodes
                .iter()
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
        });

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
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _event, _window, cx| {
                                    this.toggle_topology(cx);
                                }),
                            )
                            .cursor_pointer()
                            .child(if self.topology_expanded {
                                IconName::ChevronDown
                            } else {
                                IconName::ChevronRight
                            })
                            .child(IconName::Network)
                            .child(
                                Label::new("网络拓扑 - 业务层级视图")
                                    .text_sm()
                                    .font_weight(FontWeight::SEMIBOLD),
                            )
                            .child(div().flex_1())
                            .child(
                                // Status legend from design
                                h_flex()
                                    .gap_3()
                                    .items_center()
                                    .child(
                                        h_flex()
                                            .gap_1()
                                            .items_center()
                                            .child(
                                                div()
                                                    .size(px(8.0))
                                                    .rounded_full()
                                                    .bg(rgb(0x10b981)),
                                            )
                                            .child(
                                                Label::new("低危")
                                                    .text_xs()
                                                    .text_color(rgb(TEXT_SECONDARY)),
                                            ),
                                    )
                                    .child(
                                        h_flex()
                                            .gap_1()
                                            .items_center()
                                            .child(
                                                div()
                                                    .size(px(8.0))
                                                    .rounded_full()
                                                    .bg(rgb(0xfbbf24)),
                                            )
                                            .child(
                                                Label::new("中危")
                                                    .text_xs()
                                                    .text_color(rgb(TEXT_SECONDARY)),
                                            ),
                                    )
                                    .child(
                                        h_flex()
                                            .gap_1()
                                            .items_center()
                                            .child(
                                                div()
                                                    .size(px(8.0))
                                                    .rounded_full()
                                                    .bg(rgb(0xf97316)),
                                            )
                                            .child(
                                                Label::new("高危")
                                                    .text_xs()
                                                    .text_color(rgb(TEXT_SECONDARY)),
                                            ),
                                    )
                                    .child(
                                        h_flex()
                                            .gap_1()
                                            .items_center()
                                            .child(
                                                div()
                                                    .size(px(8.0))
                                                    .rounded_full()
                                                    .bg(rgb(0xef4444)),
                                            )
                                            .child(
                                                Label::new("严重")
                                                    .text_xs()
                                                    .text_color(rgb(TEXT_SECONDARY)),
                                            ),
                                    ),
                            )
                            .child(
                                h_flex()
                                    .gap_2()
                                    .items_center()
                                    .ml_4()
                                    .child(
                                        Label::new("8 资产")
                                            .text_xs()
                                            .text_color(rgb(TEXT_SECONDARY)),
                                    )
                                    .child(div().w(px(1.0)).h(px(12.0)).bg(rgb(BORDER_COLOR)))
                                    .child(
                                        Label::new("19 连接")
                                            .text_xs()
                                            .text_color(rgb(TEXT_SECONDARY)),
                                    ),
                            )
                            .child(IconName::ChevronDown),
                    )
                    .child(if self.topology_expanded {
                        self.topology_canvas.clone().into_any_element()
                    } else {
                        div().into_any_element()
                    }),
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
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _event, _window, cx| {
                                    this.toggle_details(cx);
                                }),
                            )
                            .cursor_pointer()
                            .child(if self.details_expanded {
                                IconName::ChevronDown
                            } else {
                                IconName::ChevronRight
                            })
                            .child(IconName::File)
                            .child(
                                Label::new("资产详情")
                                    .text_sm()
                                    .font_weight(FontWeight::SEMIBOLD),
                            ),
                    )
                    .child(if self.details_expanded && self.selected_asset.is_some() {
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
                                    .text_color(rgb(TEXT_MUTED)),
                            )
                            .into_any_element()
                    } else {
                        div().into_any_element()
                    }),
            )
    }
}
