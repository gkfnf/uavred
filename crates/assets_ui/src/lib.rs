//! Assets UI - Network asset topology visualization
//!
//! ## Module Structure
//!
//! ```text
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
use gpui_component::{h_flex, label::Label, v_flex, IconName, button::Button, Sizable};

mod asset_detail_panel;
mod components;
mod config;
mod repository;
mod events;
mod topology_canvas;
mod topology_webview;
mod webview_topology;

pub use asset_detail_panel::AssetDetailPanel;
pub use components::*;
pub use config::{theme_ext, ui_labels, zone_config, ZoneTypeExt};
pub use repository::{AssetRepository, MockAssetRepository};
pub use events::AssetActionEvent;
pub use topology_canvas::{AssetSelectedEvent, NodeVirtualPos, TopologyCanvas};
pub use topology_webview::TopologyWebView;
pub use webview_topology::{WebViewTopologyCanvas, WebViewTopologyEvent};

use data::models::AssetNode;
use data::{AssetStore, init_and_load_asset_store};
use ui::theme::*;
use workspace::AppView;
use workspace_ui::AppState;

/// Topology view mode
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TopologyViewMode {
    /// Native Z1-Z5 zone-based canvas
    Native,
    /// WebView-based D3.js force-directed graph
    WebView,
}

/// AssetsPanel - Top-level asset management container
///
/// Coordinates:
/// 1. TopologyCanvas - renders network asset topology (native or webview)
/// 2. AssetDetailPanel - displays selected asset details
pub struct AssetsPanel {
    topology_expanded: bool,
    details_expanded: bool,
    view_mode: TopologyViewMode,
    topology_canvas: Entity<TopologyCanvas>,
    webview_topology: Option<Entity<WebViewTopologyCanvas>>,
    asset_detail_panel: Entity<AssetDetailPanel>,
    selected_asset: Option<AssetNode>,
    _subscriptions: Vec<Subscription>,
}

impl AssetsPanel {
    pub fn new(cx: &mut Context<Self>) -> Self {
        // Initialize asset store for database access
        init_and_load_asset_store(cx);
        
        let topology_canvas = cx.new(TopologyCanvas::new);
        let asset_detail_panel = cx.new(AssetDetailPanel::new);

        // Subscribe to asset selection events from topology canvas
        let asset_detail_panel_clone = asset_detail_panel.clone();
        let subscription = cx.subscribe(&topology_canvas, move |this, _topology: Entity<TopologyCanvas>, event, cx| {
            tracing::info!("AssetsPanel received event: {:?}", event);
            
            // Use if let to properly handle the event reference
            if let AssetSelectedEvent::NodeSelected(node_id) = event {
                tracing::info!("NodeSelected event with node_id: {}", node_id);
                
                // Parse the node ID (it's a string representation of the database ID)
                if let Ok(asset_id) = node_id.parse::<i64>() {
                    tracing::info!("Parsed asset_id: {}", asset_id);
                    
                    // Load full asset data from database
                    let asset_store = AssetStore::global(cx);
                    if let Some(asset_node) = asset_store.read(cx).get_asset_node_by_id(asset_id) {
                        tracing::info!("Found asset node: {} ({})", asset_node.name, asset_node.ip_address);
                        
                        // Update detail panel with full node data
                        asset_detail_panel_clone.update(cx, |panel, cx| {
                            panel.set_node(asset_node.clone(), cx);
                        });
                        // Update local state
                        this.selected_asset = Some(asset_node.clone());
                        this.details_expanded = true;
                        cx.notify();
                        tracing::info!("AssetsPanel updated successfully");
                    } else {
                        tracing::warn!("Could not find asset with id {} in AssetStore", asset_id);
                    }
                } else {
                    tracing::error!("Failed to parse node_id '{}' as i64", node_id);
                }
            }
        });

        Self {
            topology_expanded: true,
            details_expanded: false,
            view_mode: TopologyViewMode::Native,
            topology_canvas,
            webview_topology: None,
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

    /// Toggle between native and webview topology views
    fn toggle_view_mode(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        tracing::info!("Toggling view mode from {:?}", self.view_mode);
        
        self.view_mode = match self.view_mode {
            TopologyViewMode::Native => TopologyViewMode::WebView,
            TopologyViewMode::WebView => TopologyViewMode::Native,
        };
        
        tracing::info!("View mode switched to {:?}", self.view_mode);
        cx.notify();
    }

    /// Set WebView visibility - called by Workspace when switching tabs
    pub fn set_webview_visible(&self, visible: bool, cx: &mut Context<Self>) {
        if let Some(ref webview_topo) = self.webview_topology {
            webview_topo.update(cx, |webview, cx| {
                webview.set_visible(visible, cx);
            });
        }
    }

    /// Render topology panel header
    fn render_topology_header(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let repository = MockAssetRepository::new();
        let asset_count = repository.get_asset_count();
        let connection_count = repository.get_connection_count();
        let view_mode = self.view_mode;
        let is_expanded = self.topology_expanded;

        h_flex()
            .gap_2()
            .p_3()
            .items_center()
            .bg(rgb(BG_PRIMARY))
            .child(
                // Expand/collapse button - separate from header click
                div()
                    .cursor_pointer()
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, _, _, cx| this.toggle_topology(cx)),
                    )
                    .child(if is_expanded {
                        IconName::ChevronDown
                    } else {
                        IconName::ChevronRight
                    })
            )
            .child(IconName::Network)
            .child(
                Label::new(config::ui_labels::panel::TOPOLOGY_TITLE)
                    .text_sm()
                    .font_weight(FontWeight::SEMIBOLD),
            )
            .child(div().flex_1())
            // View mode toggle button - always visible when expanded
            .child(
                Button::new("toggle-view-mode")
                    .label(if view_mode == TopologyViewMode::Native {
                        "🌐 切换到 D3.js 拓扑"
                    } else {
                        "📊 切换到 Z1-Z5 分区"
                    })
                    .small()
                    .on_click(cx.listener(move |this, _, window, cx| {
                        tracing::info!("Toggle view mode button clicked");
                        this.toggle_view_mode(window, cx);
                    }))
            )
            .child(div().w(px(12.0)))
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
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Check if we should show the webview based on active view
        let is_assets_active = cx.try_global::<AppState>()
            .map(|state| state.get_active_view() == AppView::Assets)
            .unwrap_or(true); // Default to visible if no global state
        
        // Initialize webview topology if needed and in webview mode
        if self.view_mode == TopologyViewMode::WebView && self.webview_topology.is_none() {
            tracing::info!("Creating WebView topology in render...");
            let webview_topo = cx.new(|cx| WebViewTopologyCanvas::new(window, cx));
            
            // Subscribe to webview topology events
            let asset_detail_panel_clone = self.asset_detail_panel.clone();
            let subscription = cx.subscribe(&webview_topo, move |this, _webview, event, cx| {
                if let WebViewTopologyEvent::NodeSelected(node_id) = event {
                    tracing::info!("WebView topology node selected: {}", node_id);
                    if let Ok(asset_id) = node_id.parse::<i64>() {
                        let asset_store = AssetStore::global(cx);
                        if let Some(asset_node) = asset_store.read(cx).get_asset_node_by_id(asset_id) {
                            asset_detail_panel_clone.update(cx, |panel, cx| {
                                panel.set_node(asset_node.clone(), cx);
                            });
                            this.selected_asset = Some(asset_node.clone());
                            this.details_expanded = true;
                            cx.notify();
                        }
                    }
                }
            });
            
            // Set initial visibility based on active view
            webview_topo.update(cx, |webview, cx| {
                webview.set_visible(is_assets_active, cx);
            });
            
            self.webview_topology = Some(webview_topo);
            self._subscriptions.push(subscription);
            tracing::info!("WebView topology created successfully");
        }
        
        // Ensure WebView visibility is in sync with active view
        if let Some(ref webview_topo) = self.webview_topology {
            webview_topo.update(cx, |webview, cx| {
                webview.set_visible(is_assets_active, cx);
            });
        }

        let topology_content: gpui::AnyElement = if !self.topology_expanded {
            div().into_any_element()
        } else {
            match self.view_mode {
                TopologyViewMode::Native => self.topology_canvas.clone().into_any_element(),
                TopologyViewMode::WebView => {
                    if let Some(ref webview_topo) = self.webview_topology {
                        webview_topo.clone().into_any_element()
                    } else {
                        div()
                            .size_full()
                            .items_center()
                            .justify_center()
                            .child(Label::new("正在初始化 WebView..."))
                            .into_any_element()
                    }
                }
            }
        };
        
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
                    .child(self.render_topology_header(window, cx))
                    .child(topology_content),
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
