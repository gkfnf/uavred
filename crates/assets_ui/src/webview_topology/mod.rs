//! WebView-based Network Topology Canvas with Z1-Z5 Zone Boundaries
//! 
//! Displays assets in a unified force-directed graph while maintaining
//! Z1-Z5 zone visual boundaries like Obsidian's Graph View.

use gpui::*;
use gpui_wry::WebView as GpuiWebView;
use serde_json::json;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use data::models::Asset;
use data::{AssetStore, init_and_load_asset_store};

use wry;

/// Event emitted when an asset is selected in the webview topology
#[derive(Clone, Debug)]
pub enum WebViewTopologyEvent {
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

/// WebView-based topology canvas using D3.js for visualization
pub struct WebViewTopologyCanvas {
    focus_handle: FocusHandle,
    webview: Entity<GpuiWebView>,
    selected_node_id: Option<String>,
    ipc_state: IpcState,
    _subscriptions: Vec<Subscription>,
}

impl WebViewTopologyCanvas {
    /// Create a new WebView topology canvas
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();
        let ipc_state = IpcState::new();
        
        // Initialize asset store
        init_and_load_asset_store(cx);
        
        // Load assets
        let asset_store = AssetStore::global(cx);
        let assets = asset_store.read(cx).get_all_assets();
        
        tracing::info!("WebViewTopologyCanvas: loading {} assets", assets.len());
        
        // Generate HTML with embedded D3.js visualization
        let html = Self::generate_topology_html(&assets);
        
        // Create the webview with IPC handler
        let webview = Self::create_webview(window, cx, html, ipc_state.clone());
        
        // Subscribe to asset store updates
        let asset_store_clone = asset_store.clone();
        let subscription = cx.subscribe(&asset_store_clone, move |this, _store, event, cx| {
            use data::asset_store::AssetStoreEvent;
            if let AssetStoreEvent::AssetsUpdated = event {
                // Reload the webview with new data
                let assets = AssetStore::global(cx).read(cx).get_all_assets();
                let html = Self::generate_topology_html(&assets);
                this.reload_webview(html, cx);
            }
        });
        
        // Spawn a task to check for IPC messages and emit events
        let ipc_state_clone = ipc_state.clone();
        cx.spawn(async move |this, cx| {
            loop {
                // Check every 100ms for new messages
                cx.background_executor().timer(std::time::Duration::from_millis(100)).await;
                
                let messages = ipc_state_clone.drain_messages();
                for msg in messages {
                    if msg.starts_with("SELECT:") {
                        let node_id = msg.trim_start_matches("SELECT:").to_string();
                        tracing::info!("IPC received node selection: {}", node_id);
                        this.update(cx, |_, cx| {
                            cx.emit(WebViewTopologyEvent::NodeSelected(node_id));
                        }).ok();
                    }
                }
            }
        }).detach();
        
        Self {
            focus_handle,
            webview,
            selected_node_id: None,
            ipc_state,
            _subscriptions: vec![subscription],
        }
    }
    
    /// Create the wry webview with the given HTML content
    fn create_webview(window: &mut Window, cx: &mut App, html: String, ipc_state: IpcState) -> Entity<GpuiWebView> {
        cx.new(|cx| {
            // Create IPC handler
            let ipc_state_clone = ipc_state.clone();
            
            // Create wry webview builder with IPC handler
            let builder = wry::WebViewBuilder::new()
                .with_html(&html)
                .with_devtools(true)
                .with_ipc_handler(move |req: wry::http::Request<String>| {
                    let body = req.body().clone();
                    tracing::info!("IPC message from WebView: {}", body);
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
                let window_handle = window.window_handle().map_err(|e| {
                    tracing::error!("Failed to get window handle: {:?}", e);
                    e
                }).unwrap();
                builder.build_as_child(&window_handle).unwrap()
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
                builder.build_gtk(&fixed).unwrap()
            };
            
            GpuiWebView::new(wry_webview, window, cx)
        })
    }
    
    /// Reload the webview with new HTML content
    fn reload_webview(&mut self, html: String, cx: &mut Context<Self>) {
        self.webview.update(cx, |webview, _| {
            // Use evaluate_script to update data or reload
            let _ = webview.evaluate_script(&format!(
                "if (window.updateTopologyData) {{ window.updateTopologyData({}); }}",
                html
            ));
        });
    }
    
    /// Check if an IP is in a CIDR range
    fn ip_in_cidr(ip: &str, cidr: &str) -> bool {
        // Handle special cases
        if cidr == "0.0.0.0/0" || cidr == "any" {
            return true;
        }
        
        // Parse CIDR
        let parts: Vec<&str> = cidr.split('/').collect();
        if parts.len() != 2 {
            return ip == cidr;
        }
        
        let network_ip = parts[0];
        let mask_bits: u32 = match parts[1].parse() {
            Ok(n) => n,
            Err(_) => return ip == cidr,
        };
        
        // Parse IPs
        let ip_parts: Vec<u8> = ip.split('.').filter_map(|s| s.parse().ok()).collect();
        let net_parts: Vec<u8> = network_ip.split('.').filter_map(|s| s.parse().ok()).collect();
        
        if ip_parts.len() != 4 || net_parts.len() != 4 {
            return false;
        }
        
        // Calculate mask
        let mask = if mask_bits == 0 {
            0u32
        } else {
            !((1u32 << (32 - mask_bits)) - 1)
        };
        
        // Convert to u32
        let ip_u32 = ((ip_parts[0] as u32) << 24) 
                   | ((ip_parts[1] as u32) << 16) 
                   | ((ip_parts[2] as u32) << 8) 
                   | (ip_parts[3] as u32);
        
        let net_u32 = ((net_parts[0] as u32) << 24) 
                    | ((net_parts[1] as u32) << 16) 
                    | ((net_parts[2] as u32) << 8) 
                    | (net_parts[3] as u32);
        
        (ip_u32 & mask) == (net_u32 & mask)
    }
    
    /// Check if a network segment matches any of the accessible networks
    fn network_matches_accessible(network: &str, accessible_list: &[String]) -> bool {
        for accessible in accessible_list {
            // Direct match
            if network == accessible {
                return true;
            }
            
            // CIDR match (check if any IP in network is in the accessible CIDR)
            // For simplicity, we check if network's base IP is in the accessible CIDR
            let network_base = network.split('/').next().unwrap_or(network);
            if Self::ip_in_cidr(network_base, accessible) {
                return true;
            }
        }
        false
    }
    
    /// Get color based on risk/vulnerability level
    fn get_node_color(risk: i32, vuln_count: i32) -> &'static str {
        if risk >= 70 || vuln_count > 5 {
            "#ef4444" // Red - high risk/vulnerable
        } else if risk >= 40 || vuln_count > 0 {
            "#f59e0b" // Orange - medium risk
        } else if risk >= 20 {
            "#3b82f6" // Blue - low risk
        } else {
            "#10b981" // Green - safe
        }
    }
    
    /// Generate the complete HTML with embedded D3.js visualization
    fn generate_topology_html(assets: &[Asset]) -> String {
        // Zone configuration - matching the native implementation
        let zones = vec![
            ("Z1", "地面指挥中心", "#e8f4fd", "#2563eb", 0.0, 0.2),
            ("Z2", "通信网关层", "#d1fae5", "#10b981", 0.2, 0.4),
            ("Z3", "任务控制层", "#ede9fe", "#7c3aed", 0.4, 0.6),
            ("Z4", "飞控设备层", "#ffedd5", "#f97316", 0.6, 0.8),
            ("Z5", "安全应急系统", "#fee2e2", "#ef4444", 0.8, 1.0),
        ];
        
        // Calculate node size based on total asset count
        let total_assets = assets.len();
        let node_scale_factor = if total_assets > 200 {
            0.4
        } else if total_assets > 100 {
            0.6
        } else if total_assets > 50 {
            0.8
        } else {
            1.0
        };
        
        // Build nodes from assets - color based on risk/vulnerability
        let nodes: Vec<serde_json::Value> = assets.iter().map(|asset| {
            let zone = asset.zone_id.as_deref().unwrap_or("Z1");
            let zone_index = zones.iter().position(|(z, _, _, _, _, _)| *z == zone).unwrap_or(0);
            
            // Color based on risk/vulnerability, not zone
            let color = Self::get_node_color(asset.risk_score, asset.vuln_count);
            
            json!({
                "id": asset.id.to_string(),
                "name": asset.name,
                "group": zone,
                "zoneIndex": zone_index,
                "zoneColor": zones[zone_index].3,
                "color": color,
                "risk": asset.risk_score,
                "vulnCount": asset.vuln_count,
                "ip": asset.ip_address.as_deref().unwrap_or(""),
                "type": asset.asset_type,
                "status": format!("{:?}", asset.status),
            })
        }).collect();
        
        // Build links from accessible_networks with CIDR matching
        let mut links = Vec::new();
        let mut link_set = std::collections::HashSet::new();
        
        // Create links based on network accessibility
        for source in assets {
            if source.accessible_networks.is_empty() {
                continue;
            }
            
            for target in assets {
                if source.id == target.id {
                    continue;
                }
                
                // Check if target's network is in source's accessible list
                if !target.network_segment.is_empty() 
                   && Self::network_matches_accessible(&target.network_segment, &source.accessible_networks) {
                    
                    let key = format!("{}-{}", source.id.min(target.id), source.id.max(target.id));
                    if !link_set.contains(&key) {
                        link_set.insert(key);
                        links.push(json!({
                            "source": source.id.to_string(),
                            "target": target.id.to_string(),
                            "type": "network"
                        }));
                    }
                }
            }
        }
        
        // Log connection statistics
        let mut connection_counts: HashMap<i64, usize> = HashMap::new();
        for link in &links {
            let source_id: i64 = link["source"].as_str().unwrap().parse().unwrap();
            let target_id: i64 = link["target"].as_str().unwrap().parse().unwrap();
            *connection_counts.entry(source_id).or_insert(0) += 1;
            *connection_counts.entry(target_id).or_insert(0) += 1;
        }
        
        // Find assets with most connections
        let mut sorted_connections: Vec<_> = connection_counts.iter().collect();
        sorted_connections.sort_by(|a, b| b.1.cmp(a.1));
        
        tracing::info!("Network topology: {} assets, {} connections", total_assets, links.len());
        for (id, count) in sorted_connections.iter().take(5) {
            if let Some(asset) = assets.iter().find(|a| a.id == **id) {
                tracing::info!("  Asset {} ({}): {} connections", asset.name, asset.ip_address.as_deref().unwrap_or("N/A"), count);
            }
        }
        
        // Limit links for performance if too many
        let max_links = 500;
        let links_to_show: Vec<_> = if links.len() > max_links {
            tracing::warn!("Limiting links from {} to {} for performance", links.len(), max_links);
            links.into_iter().take(max_links).collect()
        } else {
            links
        };
        
        let nodes_json = serde_json::to_string(&nodes).unwrap();
        let links_json = serde_json::to_string(&links_to_show).unwrap();
        let zones_json = serde_json::to_string(&zones).unwrap();
        
        format!(r##"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>UAVRed Network Topology</title>
    <script src="https://d3js.org/d3.v7.min.js"></script>
    <style>
        * {{ margin: 0; padding: 0; box-sizing: border-box; }}
        
        body {{
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
            overflow: hidden;
            color: #1f2937;
            background: #f9fafb;
        }}
        
        #graph {{
            width: 100vw;
            height: 100vh;
            position: relative;
        }}
        
        /* Zone backgrounds */
        .zone-bg {{
            stroke: rgba(0,0,0,0.08);
            stroke-width: 1px;
        }}
        
        .zone-label {{
            font-size: 13px;
            font-weight: 600;
        }}
        
        .zone-sublabel {{
            font-size: 11px;
            fill: #6b7280;
        }}
        
        .zone-count {{
            font-size: 10px;
            fill: #9ca3af;
        }}
        
        /* Node styles */
        .node {{
            cursor: pointer;
            stroke: #fff;
            stroke-width: 1.5px;
            transition: all 0.2s ease;
        }}
        
        .node:hover {{
            stroke: #1f2937;
            stroke-width: 2px;
            filter: brightness(1.1);
        }}
        
        .node.selected {{
            stroke: #1f2937;
            stroke-width: 3px;
            filter: drop-shadow(0 0 8px rgba(0,0,0,0.3));
        }}
        
        .node.vulnerable {{
            stroke-dasharray: 3,2;
        }}
        
        /* Link styles - lighter and thinner */
        .link {{
            stroke: #d1d5db;
            stroke-width: 0.8px;
            stroke-opacity: 0.5;
            transition: all 0.2s;
        }}
        
        .link.cross-zone {{
            stroke: #9ca3af;
            stroke-dasharray: 3,3;
            stroke-opacity: 0.4;
        }}
        
        .link:hover {{
            stroke-width: 1.5px;
            stroke: #6b7280;
            stroke-opacity: 0.8;
        }}
        
        /* Label styles - only show on hover or for important nodes */
        .label {{
            font-size: 9px;
            fill: #4b5563;
            font-weight: 400;
            pointer-events: none;
            text-shadow: 0 1px 2px rgba(255,255,255,0.9);
            opacity: 0;
            transition: opacity 0.2s;
        }}
        
        .label.visible {{
            opacity: 1;
        }}
        
        .label-bg {{
            fill: rgba(255,255,255,0.85);
            rx: 3;
            pointer-events: none; /* Prevent blocking clicks on other nodes */
        }}
        
        /* Tooltip */
        .tooltip {{
            position: absolute;
            padding: 10px 14px;
            background: rgba(0,0,0,0.9);
            border-radius: 6px;
            font-size: 12px;
            color: #fff;
            pointer-events: none;
            opacity: 0;
            transition: opacity 0.2s;
            box-shadow: 0 4px 12px rgba(0,0,0,0.3);
            z-index: 1000;
            max-width: 250px;
        }}
        
        .tooltip.show {{
            opacity: 1;
        }}
        
        .tooltip strong {{
            display: block;
            margin-bottom: 4px;
            font-size: 13px;
            color: #60a5fa;
        }}
        
        .tooltip .risk-high {{ color: #ef4444; }}
        .tooltip .risk-medium {{ color: #f59e0b; }}
        .tooltip .risk-low {{ color: #10b981; }}
        
        /* Controls */
        .controls {{
            position: absolute;
            bottom: 16px;
            right: 16px;
            display: flex;
            gap: 6px;
            background: rgba(255,255,255,0.95);
            padding: 6px;
            border-radius: 6px;
            box-shadow: 0 1px 4px rgba(0,0,0,0.1);
            border: 1px solid #e5e7eb;
        }}
        
        .control-btn {{
            width: 28px;
            height: 28px;
            border: 1px solid #e5e7eb;
            border-radius: 4px;
            background: #fff;
            color: #374151;
            cursor: pointer;
            font-size: 14px;
            display: flex;
            align-items: center;
            justify-content: center;
            transition: all 0.15s;
        }}
        
        .control-btn:hover {{
            background: #f3f4f6;
            border-color: #d1d5db;
        }}
        
        /* Legend - positioned in Z5 area (right side) */
        .legend {{
            position: absolute;
            top: 60px;
            right: 16px;
            background: rgba(255,255,255,0.95);
            padding: 12px;
            border-radius: 6px;
            box-shadow: 0 1px 4px rgba(0,0,0,0.1);
            border: 1px solid #e5e7eb;
            font-size: 11px;
            max-width: 180px;
        }}
        
        .legend-item {{
            display: flex;
            align-items: center;
            gap: 6px;
            margin: 4px 0;
        }}
        
        .legend-dot {{
            width: 8px;
            height: 8px;
            border-radius: 50%;
        }}
    </style>
</head>
<body>
    <div id="graph"></div>
    <div class="tooltip" id="tooltip"></div>
    
    <!-- Legend -->
    <div class="legend">
        <div style="font-weight: 600; margin-bottom: 6px;">风险等级</div>
        <div class="legend-item">
            <div class="legend-dot" style="background: #ef4444;"></div>
            <span>高危 (风险≥70或有漏洞)</span>
        </div>
        <div class="legend-item">
            <div class="legend-dot" style="background: #f59e0b;"></div>
            <span>中危 (风险≥40)</span>
        </div>
        <div class="legend-item">
            <div class="legend-dot" style="background: #3b82f6;"></div>
            <span>低危 (风险≥20)</span>
        </div>
        <div class="legend-item">
            <div class="legend-dot" style="background: #10b981;"></div>
            <span>安全 (风险<20)</span>
        </div>
        <div style="margin-top: 8px; font-size: 10px; color: #6b7280;">
            共 {total_assets} 个资产
        </div>
    </div>
    
    <div class="controls">
        <button class="control-btn" onclick="resetZoom()" title="重置视图">⟲</button>
        <button class="control-btn" onclick="zoomIn()" title="放大">+</button>
        <button class="control-btn" onclick="zoomOut()" title="缩小">−</button>
    </div>

    <script>
        // Data from Rust
        const nodes = {nodes_json};
        const links = {links_json};
        const zones = {zones_json};
        const nodeScaleFactor = {node_scale_factor};
        const totalAssets = {total_assets};
        
        const width = window.innerWidth;
        const height = window.innerHeight;
        const headerHeight = 55;
        
        // Zone boundaries
        const zoneWidth = width / zones.length;
        
        // Count nodes per zone
        const zoneNodeCounts = {{}};
        zones.forEach((z, i) => zoneNodeCounts[i] = nodes.filter(n => n.zoneIndex === i).length);
        
        // Create SVG
        const svg = d3.select('#graph')
            .append('svg')
            .attr('width', width)
            .attr('height', height);
        
        // Main container for zoom
        const g = svg.append('g');
        
        // Define arrow markers for directed links
        const defs = svg.append('defs');
        
        // Arrow marker for normal links - smaller size
        defs.append('marker')
            .attr('id', 'arrow-normal')
            .attr('viewBox', '0 -5 10 10')
            .attr('refX', 15) // Closer to node
            .attr('refY', 0)
            .attr('markerWidth', 4)
            .attr('markerHeight', 4)
            .attr('orient', 'auto')
            .append('path')
            .attr('d', 'M0,-4L8,0L0,4')
            .attr('fill', '#9ca3af');
        
        // Arrow marker for highlighted links
        defs.append('marker')
            .attr('id', 'arrow-highlighted')
            .attr('viewBox', '0 -5 10 10')
            .attr('refX', 15)
            .attr('refY', 0)
            .attr('markerWidth', 5)
            .attr('markerHeight', 5)
            .attr('orient', 'auto')
            .append('path')
            .attr('d', 'M0,-4L8,0L0,4')
            .attr('fill', '#4b5563');
        
        // Arrow marker for cross-zone links
        defs.append('marker')
            .attr('id', 'arrow-crosszone')
            .attr('viewBox', '0 -5 10 10')
            .attr('refX', 15)
            .attr('refY', 0)
            .attr('markerWidth', 4)
            .attr('markerHeight', 4)
            .attr('orient', 'auto')
            .append('path')
            .attr('d', 'M0,-4L8,0L0,4')
            .attr('fill', '#6b7280');
        
        // Calculate initial scale to fit all content
        const contentWidth = width;
        const contentHeight = height;
        const initialScale = Math.min(
            width / contentWidth * 0.95,
            height / contentHeight * 0.95,
            1
        );
        
        // Zoom behavior - prevent zooming out beyond initial fit
        const zoom = d3.zoom()
            .scaleExtent([initialScale * 0.8, 3]) // Minimum zoom is slightly smaller than initial
            .on('zoom', (event) => {{
                g.attr('transform', event.transform);
            }});
        
        svg.call(zoom);
        
        // Draw zone backgrounds
        const zoneGroups = g.selectAll('.zone-group')
            .data(zones)
            .join('g')
            .attr('class', 'zone-group');
        
        // Zone background rectangles
        zoneGroups.append('rect')
            .attr('class', 'zone-bg')
            .attr('x', (d, i) => i * zoneWidth)
            .attr('y', 0)
            .attr('width', zoneWidth)
            .attr('height', height)
            .attr('fill', d => d[2])
            .attr('fill-opacity', 0.5);
        
        // Zone separators
        zoneGroups.filter((d, i) => i > 0)
            .append('line')
            .attr('x1', (d, i) => (i + 1) * zoneWidth)
            .attr('y1', 0)
            .attr('x2', (d, i) => (i + 1) * zoneWidth)
            .attr('y2', height)
            .attr('stroke', '#d1d5db')
            .attr('stroke-width', 1);
        
        // Zone headers
        const headerGroups = zoneGroups.append('g').attr('class', 'zone-header');
        
        // Zone icon circle
        headerGroups.append('circle')
            .attr('cx', (d, i) => i * zoneWidth + 16)
            .attr('cy', 20)
            .attr('r', 8)
            .attr('fill', d => d[3]);
        
        // Zone ID text
        headerGroups.append('text')
            .attr('class', 'zone-label')
            .attr('x', (d, i) => i * zoneWidth + 30)
            .attr('y', 16)
            .attr('fill', d => d[3])
            .text(d => d[0]);
        
        // Zone name
        headerGroups.append('text')
            .attr('class', 'zone-sublabel')
            .attr('x', (d, i) => i * zoneWidth + 30)
            .attr('y', 30)
            .text(d => d[1]);
        
        // Asset count
        headerGroups.append('text')
            .attr('class', 'zone-count')
            .attr('x', (d, i) => i * zoneWidth + 30)
            .attr('y', 42)
            .text((d, i) => zoneNodeCounts[i] + ' 资产');
        
        // Calculate zone centers for force layout
        const zoneCenters = zones.map((_, i) => ({{
            x: i * zoneWidth + zoneWidth / 2,
            y: (height + headerHeight) / 2
        }}));
        
        // Adaptive radius based on node count - larger minimum to prevent overlap
        const baseRadius = Math.max(4, 6 * nodeScaleFactor);
        const radiusScale = d3.scaleLinear()
            .domain([0, 100])
            .range([baseRadius, baseRadius * 1.5]);
        
        // Calculate connection count per node for visual weight
        const connectionCounts = {{}};
        links.forEach(l => {{
            const s = l.source.id || l.source;
            const t = l.target.id || l.target;
            connectionCounts[s] = (connectionCounts[s] || 0) + 1;
            connectionCounts[t] = (connectionCounts[t] || 0) + 1;
        }});
        
        // Calculate density per zone for adaptive forces
        const maxZoneDensity = Math.max(...Object.values(zoneNodeCounts)) / (zoneWidth * height);
        const densityFactor = Math.min(1, 200 / totalAssets); // More spread for dense zones
        
        // Force simulation with enhanced collision detection
        const simulation = d3.forceSimulation(nodes)
            .alphaDecay(0.02) // Slower decay for better convergence
            .velocityDecay(0.3) // More damping for stability
            .force('link', d3.forceLink(links)
                .id(d => d.id)
                .distance(d => 40 * densityFactor + 20))
            .force('charge', d3.forceManyBody()
                .strength(d => -50 * densityFactor * nodeScaleFactor)
                .distanceMin(10)
                .distanceMax(300))
            .force('collision', d3.forceCollide()
                .radius(d => radiusScale(d.risk) + 6) // Larger collision radius
                .strength(1.0) // Maximum collision strength
                .iterations(3)) // Multiple iterations for better separation
            .force('x', d3.forceX(d => zoneCenters[d.zoneIndex].x).strength(0.5 * densityFactor))
            .force('y', d3.forceY(d => zoneCenters[d.zoneIndex].y).strength(0.2))
            .force('boundary', forceBoundary());
        
        // Enhanced boundary force to keep nodes in zones with padding
        function forceBoundary() {{
            const minPadding = radiusScale(100) + 10;
            return () => {{
                nodes.forEach(d => {{
                    const r = radiusScale(d.risk) + 5;
                    const zoneLeft = d.zoneIndex * zoneWidth + minPadding;
                    const zoneRight = (d.zoneIndex + 1) * zoneWidth - minPadding;
                    const zoneTop = headerHeight + minPadding;
                    const zoneBottom = height - minPadding;
                    
                    // Soft boundary with force instead of hard clamp
                    const k = 0.1; // Spring constant
                    if (d.x < zoneLeft + r) d.vx += (zoneLeft + r - d.x) * k;
                    if (d.x > zoneRight - r) d.vx -= (d.x - (zoneRight - r)) * k;
                    if (d.y < zoneTop + r) d.vy += (zoneTop + r - d.y) * k;
                    if (d.y > zoneBottom - r) d.vy -= (d.y - (zoneBottom - r)) * k;
                    
                    // Hard clamp as last resort
                    d.x = Math.max(zoneLeft + r, Math.min(zoneRight - r, d.x));
                    d.y = Math.max(zoneTop + r, Math.min(zoneBottom - r, d.y));
                }});
            }};
        }}
        
        // Draw links with arrow markers
        const link = g.append('g')
            .attr('class', 'links')
            .selectAll('line')
            .data(links)
            .join('line')
            .attr('class', d => {{
                const sourceNode = nodes.find(n => n.id === (d.source.id || d.source));
                const targetNode = nodes.find(n => n.id === (d.target.id || d.target));
                const isCrossZone = sourceNode && targetNode && sourceNode.zoneIndex !== targetNode.zoneIndex;
                return isCrossZone ? 'link cross-zone' : 'link';
            }})
            .attr('marker-end', d => {{
                const sourceNode = nodes.find(n => n.id === (d.source.id || d.source));
                const targetNode = nodes.find(n => n.id === (d.target.id || d.target));
                const isCrossZone = sourceNode && targetNode && sourceNode.zoneIndex !== targetNode.zoneIndex;
                return isCrossZone ? 'url(#arrow-crosszone)' : 'url(#arrow-normal)';
            }});
        
        // Draw nodes
        const node = g.append('g')
            .attr('class', 'nodes')
            .selectAll('circle')
            .data(nodes)
            .join('circle')
            .attr('class', d => 'node' + (d.vulnCount > 0 ? ' vulnerable' : ''))
            .attr('r', d => {{
                // Slightly larger radius for highly connected nodes (hubs)
                const connections = connectionCounts[d.id] || 0;
                const hubBonus = connections > 5 ? 2 : connections > 2 ? 1 : 0;
                return radiusScale(d.risk) + hubBonus;
            }})
            .attr('fill', d => d.color)
            .call(d3.drag()
                .on('start', dragstarted)
                .on('drag', dragged)
                .on('end', dragended));
        
        // Draw labels - only show on hover or for important nodes initially
        const labelGroup = g.append('g').attr('class', 'labels');
        
        const labelBg = labelGroup.selectAll('rect')
            .data(nodes)
            .join('rect')
            .attr('class', 'label-bg')
            .attr('x', d => -40)
            .attr('y', d => radiusScale(d.risk) + 2)
            .attr('width', 80)
            .attr('height', 12)
            .attr('opacity', 0);
        
        const label = labelGroup.selectAll('text')
            .data(nodes)
            .join('text')
            .attr('class', 'label')
            .text(d => d.name.length > 15 ? d.name.substring(0, 12) + '...' : d.name)
            .attr('text-anchor', 'middle')
            .attr('y', d => radiusScale(d.risk) + 11);
        
        // Tooltip
        const tooltip = d3.select('#tooltip');
        
        // Selection handling
        let selectedNode = null;
        
        function updateSelection(d) {{
            // Show label for selected node
            label.classed('visible', n => n.id === d.id || n.id === selectedNode);
            labelBg.attr('opacity', n => n.id === d.id || n.id === selectedNode ? 1 : 0);
        }}
        
        // Node interactions
        node.on('click', function(event, d) {{
            event.stopPropagation();
            
            // Update selection
            d3.selectAll('.node').classed('selected', false);
            d3.select(this).classed('selected', true);
            selectedNode = d.id;
            
            updateSelection(d);
            
            // Notify Rust
            if (window.ipc) {{
                window.ipc.postMessage('SELECT:' + d.id);
            }}
        }});
        
        node.on('dblclick', function(event, d) {{
            event.stopPropagation();
            const transform = d3.zoomIdentity
                .translate(width / 2, height / 2)
                .scale(1.8)
                .translate(-d.x, -d.y);
            
            svg.transition()
                .duration(500)
                .call(zoom.transform, transform);
        }});
        
        // Maximum links to show on hover to prevent visual pollution
        const MAX_HOVER_LINKS = 20;
        
        node.on('mouseover', function(event, d) {{
            // Show label on hover
            d3.select(this).select(function() {{ return this.parentNode; }})
                .selectAll('.label')
                .filter(n => n.id === d.id)
                .classed('visible', true);
            
            const outgoingCount = links.filter(l => (l.source.id || l.source) === d.id).length;
            const incomingCount = links.filter(l => (l.target.id || l.target) === d.id).length;
            const totalConnections = outgoingCount + incomingCount;
            
            // For super-connected nodes, only show representative links
            const isSuperConnector = totalConnections > MAX_HOVER_LINKS;
            
            // Get connected links sorted by importance (cross-zone first, then by zone)
            const connectedLinks = links.map((l, idx) => {{
                const sourceId = l.source.id || l.source;
                const targetId = l.target.id || l.target;
                const isConnected = (sourceId === d.id || targetId === d.id);
                const isOutgoing = (sourceId === d.id);
                const isCrossZone = l.source.zoneIndex !== l.target.zoneIndex;
                return {{ link: l, idx, isConnected, isOutgoing, isCrossZone }};
            }}).filter(x => x.isConnected);
            
            // Sort: cross-zone first, then by target zone
            connectedLinks.sort((a, b) => {{
                if (a.isCrossZone !== b.isCrossZone) return b.isCrossZone - a.isCrossZone;
                return 0;
            }});
            
            // Select links to highlight (limit for super connectors)
            const linksToHighlight = isSuperConnector 
                ? connectedLinks.slice(0, MAX_HOVER_LINKS) 
                : connectedLinks;
            
            const highlightIndices = new Set(linksToHighlight.map(x => x.idx));
            
            // Highlight selected links
            link.each(function(l, idx) {{
                const connInfo = connectedLinks.find(c => c.idx === idx);
                const isHighlighted = highlightIndices.has(idx);
                const isConnected = connInfo !== undefined;
                const isOutgoing = connInfo ? connInfo.isOutgoing : false;
                
                d3.select(this)
                    .style('stroke-opacity', isHighlighted ? 0.9 : (isConnected ? 0.1 : 0.03))
                    .style('stroke-width', isHighlighted ? 2.5 : (isConnected ? 1 : 0.3))
                    .style('stroke', isHighlighted ? (isOutgoing ? '#2563eb' : '#059669') : null)
                    .attr('marker-end', isHighlighted ? 
                        (isOutgoing ? 'url(#arrow-highlighted)' : 'url(#arrow-crosszone)') : 
                        (l.source.zoneIndex !== l.target.zoneIndex ? 'url(#arrow-crosszone)' : 'url(#arrow-normal)'));
            }});
            
            // Highlight connected nodes (only those with visible links)
            const connectedNodeIds = new Set();
            linksToHighlight.forEach(l => {{
                connectedNodeIds.add(l.link.source.id || l.link.source);
                connectedNodeIds.add(l.link.target.id || l.link.target);
            }});
            
            node.style('opacity', n => {{
                if (n.id === d.id) return 1;
                if (connectedNodeIds.has(n.id)) return 0.9;
                return isSuperConnector ? 0.2 : 0.3;
            }});
            
            const riskClass = d.risk >= 70 || d.vulnCount > 0 ? 'risk-high' : 
                              d.risk >= 40 ? 'risk-medium' : 'risk-low';
            
            // Build tooltip with connection summary
            let tooltipHtml = `
                <strong>${{d.name}}</strong>
                <span class="${{riskClass}}">风险: ${{d.risk}}/100 ${{d.vulnCount > 0 ? '(' + d.vulnCount + '漏洞)' : ''}}</span><br>
                IP: ${{d.ip || 'N/A'}} | 区域: ${{d.group}}<br>
            `;
            
            if (outgoingCount > 0) {{
                tooltipHtml += `<span style="color:#2563eb">→ 可访问 ${{outgoingCount}} 个资产</span>`;
                if (isSuperConnector) {{
                    tooltipHtml += ` <span style="color:#6b7280;font-size:10px">(显示前${{MAX_HOVER_LINKS}}个)</span>`;
                }}
                tooltipHtml += '<br>';
            }}
            
            if (incomingCount > 0) {{
                tooltipHtml += `<span style="color:#059669">← 被 ${{incomingCount}} 个资产访问</span><br>`;
            }}
            
            if (d.vulnCount > 0) {{
                tooltipHtml += '<span style="color:#ef4444">⚠ 存在漏洞</span>';
            }}
            
            tooltip.html(tooltipHtml)
            .style('left', (event.pageX + 12) + 'px')
            .style('top', (event.pageY - 12) + 'px')
            .classed('show', true);
        }});
        
        node.on('mouseout', function(event, d) {{
            // Hide label if not selected
            if (selectedNode !== d.id) {{
                label.filter(n => n.id === d.id).classed('visible', false);
            }}
            
            // Reset links to default style
            link.each(function(l) {{
                const isCrossZone = l.source.zoneIndex !== l.target.zoneIndex;
                d3.select(this)
                    .style('stroke-opacity', null)
                    .style('stroke-width', null)
                    .style('stroke', null)
                    .attr('marker-end', isCrossZone ? 'url(#arrow-crosszone)' : 'url(#arrow-normal)');
            }});
            
            // Reset node opacity
            node.style('opacity', null);
            
            tooltip.classed('show', false);
        }});
        
        // Background click to deselect
        svg.on('click', () => {{
            d3.selectAll('.node').classed('selected', false);
            selectedNode = null;
            label.classed('visible', false);
            labelBg.attr('opacity', 0);
            link.style('stroke-opacity', null).style('stroke-width', null);
        }});
        
        // Update positions on tick
        simulation.on('tick', () => {{
            link
                .attr('x1', d => d.source.x)
                .attr('y1', d => d.source.y)
                .attr('x2', d => d.target.x)
                .attr('y2', d => d.target.y);
            
            node
                .attr('cx', d => d.x)
                .attr('cy', d => d.y);
            
            labelBg
                .attr('x', d => d.x - 40)
                .attr('y', d => d.y + radiusScale(d.risk) + 1);
            
            label
                .attr('x', d => d.x)
                .attr('y', d => d.y + radiusScale(d.risk) + 11);
        }});
        
        // Drag functions
        function dragstarted(event, d) {{
            if (!event.active) simulation.alphaTarget(0.3).restart();
            d.fx = d.x;
            d.fy = d.y;
        }}
        
        function dragged(event, d) {{
            d.fx = event.x;
            d.fy = event.y;
        }}
        
        function dragended(event, d) {{
            if (!event.active) simulation.alphaTarget(0);
            d.fx = null;
            d.fy = null;
        }}
        
        // Control functions
        window.zoomIn = function() {{
            svg.transition().call(zoom.scaleBy, 1.3);
        }};
        
        window.zoomOut = function() {{
            svg.transition().call(zoom.scaleBy, 0.7);
        }};
        
        // Calculate optimal initial transform - fill the viewport
        function calculateOptimalTransform() {{
            const bounds = g.node().getBBox();
            // Minimal padding to maximize content area
            const paddingX = 10;
            const paddingY = 10;
            
            const availableWidth = width - paddingX * 2;
            const availableHeight = height - paddingY * 2;
            
            // Calculate scale to fit content exactly in viewport
            const scaleX = availableWidth / bounds.width;
            const scaleY = availableHeight / bounds.height;
            const scale = Math.min(scaleX, scaleY);
            
            return {{
                x: paddingX - bounds.x * scale,
                y: paddingY - bounds.y * scale,
                k: scale
            }};
        }}
        
        window.resetZoom = function() {{
            const t = calculateOptimalTransform();
            svg.transition().duration(500).call(
                zoom.transform, 
                d3.zoomIdentity.translate(t.x, t.y).scale(t.k)
            );
        }};
        
        // Apply initial transform after simulation stabilizes
        setTimeout(() => {{
            const t = calculateOptimalTransform();
            
            // Update zoom extent to prevent zooming out too far
            zoom.scaleExtent([t.k * 0.9, 3]);
            
            // Apply initial transform
            svg.call(
                zoom.transform, 
                d3.zoomIdentity.translate(t.x, t.y).scale(t.k)
            );
        }}, 600);
        
        // Expose functions for Rust
        window.updateTopologyData = function(newNodes, newLinks) {{
            console.log('Updating topology data:', newNodes.length, 'nodes');
        }};
        
        window.highlightNode = function(nodeId) {{
            d3.selectAll('.node').classed('selected', d => d.id === nodeId);
            label.classed('visible', d => d.id === nodeId);
        }};
    </script>
</body>
</html>"##, 
            nodes_json = nodes_json, 
            links_json = links_json, 
            zones_json = zones_json,
            node_scale_factor = node_scale_factor,
            total_assets = total_assets
        )
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
    
    /// Get the underlying webview entity
    pub fn webview(&self) -> Entity<GpuiWebView> {
        self.webview.clone()
    }
    
    /// Set the visibility of the WebView
    pub fn set_visible(&self, visible: bool, cx: &mut Context<Self>) {
        self.webview.update(cx, |webview, _| {
            if visible {
                webview.show();
            } else {
                webview.hide();
            }
        });
    }
}

impl Focusable for WebViewTopologyCanvas {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<WebViewTopologyEvent> for WebViewTopologyCanvas {}

impl Render for WebViewTopologyCanvas {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .track_focus(&self.focus_handle)
            .size_full()
            .child(self.webview.clone())
    }
}
