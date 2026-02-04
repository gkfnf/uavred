//! WebView-based Network Topology Visualization
//! 
//! This module provides a D3.js-based topology visualization using a WebView.
//! All visualization logic is in separate HTML/CSS/JS files in the resources folder.

mod resources;

use gpui::*;
use gpui::prelude::FluentBuilder;
use gpui_wry::WebView as GpuiWebView;
use std::sync::{Arc, Mutex};
use wry;

pub use resources::TopologyResourceBuilder;

/// Event emitted when an asset is selected in the topology
#[derive(Clone, Debug)]
pub enum TopologyEvent {
    NodeSelected(String),
}

/// Shared state for IPC communication between WebView and GPUI
#[derive(Clone)]
struct IpcState {
    pending_messages: Arc<Mutex<Vec<String>>>,
}

impl IpcState {
    fn new() -> Self {
        Self {
            pending_messages: Arc::new(Mutex::new(Vec::new())),
        }
    }
    
    fn push_message(&self, msg: String) {
        if let Ok(mut messages) = self.pending_messages.lock() {
            messages.push(msg);
        }
    }
    
    fn drain_messages(&self) -> Vec<String> {
        if let Ok(mut messages) = self.pending_messages.lock() {
            std::mem::take(&mut *messages)
        } else {
            Vec::new()
        }
    }
}

/// WebView-based topology canvas
pub struct TopologyCanvas {
    focus_handle: FocusHandle,
    webview: Entity<GpuiWebView>,
    selected_node_id: Option<String>,
    /// Kept alive for IPC message handling, referenced by cloned Arc in closure
    _ipc_state: IpcState,
    _subscriptions: Vec<Subscription>,
    /// Whether the canvas is visible (controlled by parent)
    visible: bool,
}

impl TopologyCanvas {
    /// Create a new topology canvas
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();
        let ipc_state = IpcState::new();
        
        // Initialize asset store
        data::init_and_load_asset_store(cx);
        
        // Load assets and generate HTML
        let asset_store = data::AssetStore::global(cx);
        let assets = asset_store.read(cx).get_all_assets();
        let html = TopologyResourceBuilder::build_html(&assets);
        
        tracing::info!("TopologyCanvas: loading {} assets", assets.len());
        
        // Create the webview
        let webview = Self::create_webview(window, cx, html, ipc_state.clone());
        
        // Subscribe to asset store updates
        let subscription = cx.subscribe(&asset_store, move |this, _store, event, cx| {
            use data::asset_store::AssetStoreEvent;
            if let AssetStoreEvent::AssetsUpdated = event {
                let assets = data::AssetStore::global(cx).read(cx).get_all_assets();
                let html = TopologyResourceBuilder::build_html(&assets);
                this.reload(html, cx);
            }
        });
        
        // Spawn IPC message handler
        let ipc_state_clone = ipc_state.clone();
        cx.spawn(async move |this, cx| {
            loop {
                cx.background_executor().timer(std::time::Duration::from_millis(100)).await;
                
                let messages = ipc_state_clone.drain_messages();
                for msg in messages {
                    if msg.starts_with("SELECT:") {
                        let node_id = msg.trim_start_matches("SELECT:").to_string();
                        tracing::info!("IPC received node selection: {}", node_id);
                        this.update(cx, |_, cx| {
                            cx.emit(TopologyEvent::NodeSelected(node_id));
                        }).ok();
                    }
                }
            }
        }).detach();
        
        Self {
            focus_handle,
            webview,
            selected_node_id: None,
            _ipc_state: ipc_state,
            _subscriptions: vec![subscription],
            visible: true,
        }
    }
    
    /// Create the wry webview with the given HTML content
    fn create_webview(
        window: &mut Window, 
        cx: &mut App, 
        html: String, 
        ipc_state: IpcState
    ) -> Entity<GpuiWebView> {
        cx.new(|cx| {
            let ipc_state_clone = ipc_state.clone();
            
            let builder = wry::WebViewBuilder::new()
                .with_html(&html)
                .with_devtools(true)
                .with_ipc_handler(move |req: wry::http::Request<String>| {
                    let body = req.body().clone();
                    tracing::debug!("IPC message from WebView: {}", body);
                    ipc_state_clone.push_message(body);
                });
            
            #[cfg(any(
                target_os = "windows",
                target_os = "macos",
                target_os = "ios",
                target_os = "android"
            ))]
            let wry_webview = {
                use raw_window_handle::HasWindowHandle;
                let window_handle = window.window_handle().expect("Failed to get window handle");
                builder.build_as_child(&window_handle).expect("Failed to build webview")
            };
            
            #[cfg(not(any(
                target_os = "windows",
                target_os = "macos",
                target_os = "ios",
                target_os = "android"
            )))]
            let wry_webview = {
                use gtk::prelude::*;
                use wry::WebViewBuilderExtUnix;
                let fixed = gtk::Fixed::builder().build();
                fixed.show_all();
                builder.build_gtk(&fixed).expect("Failed to build webview")
            };
            
            GpuiWebView::new(wry_webview, window, cx)
        })
    }
    
    /// Reload the webview with new HTML content
    fn reload(&mut self, html: String, cx: &mut Context<Self>) {
        self.webview.update(cx, |webview, _| {
            // Note: wry doesn't have a direct reload with new HTML method
            // We use evaluate_script to call the update function if available
            let _ = webview.evaluate_script(&format!(
                "if (window.updateTopologyData) {{ window.updateTopologyData({}); }}",
                html
            ));
        });
    }
    
    /// Select a node programmatically
    pub fn select_node(&mut self, node_id: String, cx: &mut Context<Self>) {
        self.selected_node_id = Some(node_id.clone());
        self.webview.update(cx, |webview, _| {
            let _ = webview.evaluate_script(&format!(
                "if (window.highlightNode) {{ window.highlightNode('{}'); }}",
                node_id
            ));
        });
    }
    
    /// Focus and zoom to a specific node in the topology
    /// This will animate the view to center on the node and highlight it
    pub fn focus_node(&mut self, node_id: String, cx: &mut Context<Self>) {
        self.selected_node_id = Some(node_id.clone());
        self.webview.update(cx, |webview, _| {
            // First select the node, then focus it with animation
            let script = format!(
                "if (window.focusNode) {{ window.focusNode('{}'); }}",
                node_id
            );
            let _ = webview.evaluate_script(&script);
        });
    }
    
    /// Set the visibility of the topology canvas
    /// This controls both the webview visibility and whether we render anything
    pub fn set_visible(&mut self, visible: bool, cx: &mut Context<Self>) {
        self.visible = visible;
        self.webview.update(cx, |webview, _| {
            if visible {
                webview.show();
            } else {
                webview.hide();
            }
        });
        cx.notify();
    }
    
    /// Check if the canvas is visible
    pub fn is_visible(&self) -> bool {
        self.visible
    }
}

impl Focusable for TopologyCanvas {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<TopologyEvent> for TopologyCanvas {}

impl Render for TopologyCanvas {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .track_focus(&self.focus_handle)
            .size_full()
            .when(self.visible, |this| {
                this.child(self.webview.clone())
            })
    }
}

impl Drop for TopologyCanvas {
    fn drop(&mut self) {
        // Cleanup is handled automatically by GPUI entity system
        tracing::debug!("TopologyCanvas dropped");
    }
}
