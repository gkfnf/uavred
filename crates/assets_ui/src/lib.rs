//! Assets UI - Network asset topology visualization
//!
//! ## Module Structure
//!
//! ```
//! assets_ui/
//! ├── config/          # Static configuration (zone metadata, UI labels)
//! ├── data/            # Data access layer (repository pattern)
//! ├── components/      # Shared UI components
//! ├── asset_detail_panel/  # Asset detail view with card-based layout
//! ├── topology_canvas/     # Interactive network topology canvas
//! └── events.rs        # Event definitions
//! ```

use gpui::*;
use gpui::prelude::FluentBuilder;
use gpui_component::{h_flex, label::Label, v_flex, IconName};

mod asset_detail_panel;
mod components;
mod config;
mod repository;
mod events;
mod topology_canvas;

pub use asset_detail_panel::AssetDetailPanel;
pub use components::*;
pub use config::{theme_ext, ui_labels, zone_config, ZoneTypeExt};
pub use repository::{AssetRepository, MockAssetRepository};
pub use events::AssetActionEvent;
pub use topology_canvas::{AssetSelectedEvent, NodeVirtualPos, TopologyCanvas};

use data::models::AssetNode;
use ui::theme::*;

/// AssetsPanel - Top-level asset management container
///
/// Coordinates:
/// 1. TopologyCanvas - renders network asset topology
/// 2. AssetDetailPanel - displays selected asset details
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
        let subscription = cx.subscribe(&topology_canvas, move |this, topology: Entity<TopologyCanvas>, event, cx| {
            let AssetSelectedEvent::NodeSelected(node_id) = event;
            // Find the node in the topology canvas
            let node: Option<AssetNode> = topology
                .read(cx)
                .get_nodes()
                .into_iter()
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

    /// Render topology panel header
    fn render_topology_header(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let repository = MockAssetRepository::new();
        let asset_count = repository.get_asset_count();
        let connection_count = repository.get_connection_count();

        h_flex()
            .gap_2()
            .p_3()
            .items_center()
            .bg(rgb(BG_PRIMARY))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _, _, cx| this.toggle_topology(cx)),
            )
            .cursor_pointer()
            .child(if self.topology_expanded {
                IconName::ChevronDown
            } else {
                IconName::ChevronRight
            })
            .child(IconName::Network)
            .child(
                Label::new(config::ui_labels::panel::TOPOLOGY_TITLE)
                    .text_sm()
                    .font_weight(FontWeight::SEMIBOLD),
            )
            .child(div().flex_1())
            .child(self.render_severity_legend())
            .child(
                h_flex()
                    .gap_2()
                    .items_center()
                    .ml_4()
                    .child(
                        Label::new(format!("{} {}", asset_count, config::ui_labels::panel::ASSETS_COUNT_LABEL))
                            .text_xs()
                            .text_color(rgb(TEXT_SECONDARY)),
                    )
                    .child(div().w(px(1.0)).h(px(12.0)).bg(rgb(BORDER_COLOR)))
                    .child(
                        Label::new(format!("{} {}", connection_count, config::ui_labels::panel::CONNECTIONS_COUNT_LABEL))
                            .text_xs()
                            .text_color(rgb(TEXT_SECONDARY)),
                    ),
            )
            .child(IconName::ChevronDown)
    }

    /// Render severity level legend
    fn render_severity_legend(&self) -> impl IntoElement {
        use config::ui_labels::severity;

        h_flex()
            .gap_3()
            .items_center()
            .child(Self::render_legend_item(config::theme_ext::SEVERITY_LOW, severity::LOW))
            .child(Self::render_legend_item(config::theme_ext::SEVERITY_MEDIUM, severity::MEDIUM))
            .child(Self::render_legend_item(config::theme_ext::SEVERITY_HIGH, severity::HIGH))
            .child(Self::render_legend_item(config::theme_ext::SEVERITY_CRITICAL, severity::CRITICAL))
    }

    fn render_legend_item(color: u32, label: impl Into<SharedString>) -> impl IntoElement {
        let label: SharedString = label.into();
        h_flex()
            .gap_1()
            .items_center()
            .child(
                div()
                    .size(px(8.0))
                    .rounded_full()
                    .bg(rgb(color)),
            )
            .child(
                Label::new(label)
                    .text_xs()
                    .text_color(rgb(TEXT_SECONDARY)),
            )
    }

    /// Render detail panel header
    fn render_detail_header(&self, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .gap_2()
            .p_3()
            .items_center()
            .bg(rgb(BG_PRIMARY))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _, _, cx| this.toggle_details(cx)),
            )
            .cursor_pointer()
            .child(if self.details_expanded {
                IconName::ChevronDown
            } else {
                IconName::ChevronRight
            })
            .child(IconName::File)
            .child(
                Label::new(config::ui_labels::panel::DETAIL_TITLE)
                    .text_sm()
                    .font_weight(FontWeight::SEMIBOLD),
            )
    }

    /// Render detail panel content area
    fn render_detail_content(&self) -> impl IntoElement {
        if self.details_expanded && self.selected_asset.is_some() {
            self.asset_detail_panel.clone().into_any_element()
        } else if self.details_expanded && self.selected_asset.is_none() {
            div()
                .flex_1()
                .items_center()
                .justify_center()
                .p_6()
                .child(
                    Label::new(config::ui_labels::panel::NO_SELECTION_MESSAGE)
                        .text_sm()
                        .text_color(rgb(TEXT_MUTED)),
                )
                .into_any_element()
        } else {
            div().into_any_element()
        }
    }
}

impl Render for AssetsPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .gap_0()
            .bg(rgb(BG_PRIMARY))
            // Network topology panel
            .child(
                v_flex()
                    .when(self.topology_expanded, |this| this.h(px(520.0)).flex_none())
                    .when(!self.topology_expanded, |this| this.flex_none())
                    .gap_0()
                    .bg(rgb(BG_CARD))
                    .border_b_1()
                    .border_color(rgb(BORDER_COLOR))
                    .child(self.render_topology_header(cx))
                    .child(if self.topology_expanded {
                        self.topology_canvas.clone().into_any_element()
                    } else {
                        div().into_any_element()
                    }),
            )
            // Asset detail panel
            .child(
                v_flex()
                    .when(self.details_expanded, |this| this.flex_1().min_h(px(200.0)))
                    .when(!self.details_expanded, |this| this.flex_none())
                    .gap_0()
                    .bg(rgb(BG_CARD))
                    .child(self.render_detail_header(cx))
                    .child(self.render_detail_content()),
            )
    }
}
