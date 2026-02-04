//! Assets UI - Network asset topology visualization
//!
//! ## Module Structure
//!
//! ```
//! assets_ui/
//! ├── config/          # Static configuration (zone metadata, UI labels)
//! ├── components/      # Shared UI components
//! ├── asset_detail_panel/  # Asset detail view with card-based layout
//! ├── topology/        # D3.js-based topology visualization via WebView
//! └── events.rs        # Event definitions
//! ```

use gpui::*;
use gpui::prelude::FluentBuilder;
use gpui_component::{h_flex, label::Label, v_flex, IconName, input::{Input, InputState}};

mod asset_detail_panel;
mod components;
mod config;
mod events;
mod topology;

pub use asset_detail_panel::AssetDetailPanel;
pub use components::*;
pub use config::{theme_ext, ui_labels, zone_config, ZoneTypeExt};
pub use events::AssetActionEvent;
pub use topology::{TopologyCanvas, TopologyEvent};

use data::models::AssetNode;
use data::{AssetStore, init_and_load_asset_store};
use ui::theme::*;
use workspace::AppView;
use workspace_ui::AppState;

/// AssetsPanel - Top-level asset management container
///
/// Contains:
/// 1. TopologyCanvas - D3.js-based network asset topology via WebView
/// 2. AssetDetailPanel - displays selected asset details
/// 3. Search functionality for quick asset location
pub struct AssetsPanel {
    topology_expanded: bool,
    details_expanded: bool,
    topology_canvas: Option<Entity<TopologyCanvas>>,
    asset_detail_panel: Entity<AssetDetailPanel>,
    selected_asset: Option<AssetNode>,
    _subscriptions: Vec<Subscription>,
    // Search state
    search_input: Option<Entity<InputState>>,
    search_results: Vec<data::models::Asset>,
    show_search_dropdown: bool,
    _search_subscription: Option<Subscription>,
}

impl AssetsPanel {
    pub fn new(cx: &mut Context<Self>) -> Self {
        // Initialize asset store for database access
        init_and_load_asset_store(cx);
        
        let asset_detail_panel = cx.new(AssetDetailPanel::new);

        Self {
            topology_expanded: true,
            details_expanded: false,
            topology_canvas: None,
            asset_detail_panel,
            selected_asset: None,
            _subscriptions: Vec::new(),
            search_input: None,
            search_results: Vec::new(),
            show_search_dropdown: false,
            _search_subscription: None,
        }
    }
    
    /// Initialize the search input lazily
    fn ensure_search_input(&mut self, window: &mut Window, cx: &mut Context<Self>) -> Entity<InputState> {
        if self.search_input.is_none() {
            let search_input = cx.new(|cx| InputState::new(window, cx));
            
            // Observe search input changes
            let subscription = cx.observe(&search_input, |this, _input, cx| {
                let query = this.search_input.as_ref().map(|i| i.read(cx).value().to_string()).unwrap_or_default();
                this.on_search_change(query, cx);
            });
            
            self.search_input = Some(search_input);
            self._search_subscription = Some(subscription);
        }
        self.search_input.clone().unwrap()
    }
    
    /// Initialize the topology canvas lazily
    fn ensure_topology_canvas(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.topology_canvas.is_none() {
            tracing::info!("Creating topology canvas...");
            
            let topology_canvas = cx.new(|cx| TopologyCanvas::new(window, cx));
            
            // Subscribe to topology events
            let asset_detail_panel = self.asset_detail_panel.clone();
            let subscription = cx.subscribe(&topology_canvas, move |this, _topology, event, cx| {
                let node_id = match event {
                    TopologyEvent::NodeSelected(id) => id,
                };
                tracing::info!("Node selected: {}", node_id);
                
                if let Ok(asset_id) = node_id.parse::<i64>() {
                    let asset_store = AssetStore::global(cx);
                    if let Some(asset_node) = asset_store.read(cx).get_asset_node_by_id(asset_id) {
                        asset_detail_panel.update(cx, |panel, cx| {
                            panel.set_node(asset_node.clone(), cx);
                        });
                        this.selected_asset = Some(asset_node);
                        this.details_expanded = true;
                        cx.notify();
                    }
                }
            });
            
            self.topology_canvas = Some(topology_canvas);
            self._subscriptions.push(subscription);
            tracing::info!("Topology canvas created successfully");
        }
    }

    fn toggle_topology(&mut self, cx: &mut Context<Self>) {
        self.topology_expanded = !self.topology_expanded;
        
        // Update webview visibility based on collapsed state
        if let Some(ref canvas) = self.topology_canvas {
            canvas.update(cx, |canvas, cx| {
                canvas.set_visible(self.topology_expanded, cx);
            });
        }
        
        cx.notify();
    }

    fn toggle_details(&mut self, cx: &mut Context<Self>) {
        self.details_expanded = !self.details_expanded;
        cx.notify();
    }
    
    /// Handle search input change
    fn on_search_change(&mut self, query: String, cx: &mut Context<Self>) {
        if query.is_empty() {
            self.search_results.clear();
            self.show_search_dropdown = false;
        } else {
            // Search assets by name or IP
            let asset_store = AssetStore::global(cx);
            let all_assets = asset_store.read(cx).get_all_assets();
            let query_lower = query.to_lowercase();
            
            self.search_results = all_assets
                .into_iter()
                .filter(|asset| {
                    let name_match = asset.name.to_lowercase().contains(&query_lower);
                    let ip_match = asset.ip_address
                        .as_ref()
                        .map(|ip| ip.to_lowercase().contains(&query_lower))
                        .unwrap_or(false);
                    name_match || ip_match
                })
                .take(10) // Limit to 10 results
                .collect();
            
            self.show_search_dropdown = !self.search_results.is_empty();
        }
        cx.notify();
    }
    
    /// Handle search result selection
    fn on_search_select(&mut self, asset: data::models::Asset, cx: &mut Context<Self>) {
        // Clear search state
        self.search_results.clear();
        self.show_search_dropdown = false;
        
        // Focus node in topology
        if let Some(ref canvas) = self.topology_canvas {
            canvas.update(cx, |canvas, cx| {
                canvas.focus_node(asset.id.to_string(), cx);
            });
        }
        
        // Note: We don't auto-expand the detail panel anymore
        // User can click on the focused node to see details
        cx.notify();
    }
    
    /// Set WebView visibility - called by Workspace when switching tabs
    /// This is in addition to the automatic visibility sync in render()
    pub fn set_webview_visible(&mut self, visible: bool, cx: &mut Context<Self>) {
        // Update topology canvas visibility if it exists
        if let Some(ref canvas) = self.topology_canvas {
            canvas.update(cx, |canvas, _cx| {
                canvas.set_visible(visible && self.topology_expanded, _cx);
            });
        }
    }

    /// Render topology panel header
    fn render_topology_header(&mut self, window: &mut Window, cx: &mut Context<Self>, asset_count: usize, connection_count: usize) -> impl IntoElement {
        let is_expanded = self.topology_expanded;

        h_flex()
            .gap_2()
            .p_3()
            .items_center()
            .bg(rgb(BG_PRIMARY))
            .child(
                // Expand/collapse button
                div()
                    .cursor_pointer()
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, _, window, cx| {
                            // Ensure canvas is created before toggling
                            this.ensure_topology_canvas(window, cx);
                            this.toggle_topology(cx);
                        }),
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
            // Search box - moved to left side with more space
            .child(div().w(px(16.0)))
            .child(self.render_search_box(window, cx))
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
    }
    
    /// Get total asset count from store
    fn get_asset_count(&self, cx: &mut App) -> usize {
        let asset_store = AssetStore::global(cx);
        asset_store.read(cx).get_all_assets().len()
    }
    
    /// Get estimated connection count (for display purposes)
    fn get_connection_count(&self, cx: &mut App) -> usize {
        // In a real implementation, this might come from a cache or pre-computed value
        // For now, return a reasonable estimate based on asset count
        let count = self.get_asset_count(cx);
        count.saturating_mul(2).min(500)
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

    /// Render search box with horizontal results list
    fn render_search_box(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Get or create search input (ensures observe subscription is set up)
        let search_input = self.ensure_search_input(window, cx);
        let search_results = self.search_results.clone();
        let show_results = self.show_search_dropdown;
        
        // Horizontal layout: [Search Input] [Result 1] [Result 2] ...
        h_flex()
            .items_center()
            .gap_2()
            // Search input box (compact)
            .child(
                h_flex()
                    .w(px(200.0))
                    .items_center()
                    .gap_2()
                    .px_3()
                    .py_2()
                    .bg(rgb(BG_SECONDARY))
                    .rounded_md()
                    .border_1()
                    .border_color(rgb(BORDER_COLOR))
                    .child(IconName::Search)
                    .child(
                        div()
                            .flex_1()
                            .child(Input::new(&search_input))
                    )
            )
            // Horizontal search results - show up to 3 matches
            .when(show_results, |this| {
                this.children(search_results.into_iter().take(3).map(|asset| {
                    let asset_clone = asset.clone();
                    let name = if asset.name.len() > 12 {
                        format!("{}...", &asset.name[..12])
                    } else {
                        asset.name.clone()
                    };
                    let ip = asset.ip_address.clone().unwrap_or_else(|| "N/A".to_string());
                    
                    div()
                        .px_2()
                        .py_1()
                        .bg(rgb(BG_CARD))
                        .rounded_md()
                        .border_1()
                        .border_color(rgb(BORDER_COLOR))
                        .cursor_pointer()
                        .hover(|style| style.bg(rgb(BG_CARD_HOVER)))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _, _window, cx| {
                                this.on_search_select(asset_clone.clone(), cx);
                            })
                        )
                        .child(
                            v_flex()
                                .gap_0()
                                .child(
                                    Label::new(name)
                                        .text_xs()
                                        .text_color(rgb(TEXT_PRIMARY))
                                )
                                .child(
                                    Label::new(ip)
                                        .text_xs()
                                        .text_color(rgb(TEXT_SECONDARY))
                                )
                        )
                        .into_any_element()
                }).collect::<Vec<_>>())
            })
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
        // Check if assets view is active
        let is_assets_active = cx.try_global::<AppState>()
            .map(|state| state.get_active_view() == AppView::Assets)
            .unwrap_or(true);
        
        // Initialize topology canvas on first render if expanded
        if self.topology_expanded {
            self.ensure_topology_canvas(window, cx);
        }
        
        // Sync topology canvas visibility with active view and expanded state
        if let Some(ref canvas) = self.topology_canvas {
            let should_be_visible = is_assets_active && self.topology_expanded;
            canvas.update(cx, |canvas, cx| {
                if canvas.is_visible() != should_be_visible {
                    canvas.set_visible(should_be_visible, cx);
                }
            });
        }
        
        // Calculate asset counts for display
        let asset_count = self.get_asset_count(cx);
        let connection_count = self.get_connection_count(cx);

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
                    // Header with search box (includes dropdown)
                    .child(self.render_topology_header(window, cx, asset_count, connection_count))
                    // WebView topology canvas
                    .child(if self.topology_expanded {
                        if let Some(ref canvas) = self.topology_canvas {
                            canvas.clone().into_any_element()
                        } else {
                            div()
                                .size_full()
                                .items_center()
                                .justify_center()
                                .child(Label::new("正在初始化拓扑..."))
                                .into_any_element()
                        }
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

/// Helper function to create the assets panel
pub fn create_assets_panel(cx: &mut App) -> Entity<AssetsPanel> {
    cx.new(|cx| AssetsPanel::new(cx))
}
