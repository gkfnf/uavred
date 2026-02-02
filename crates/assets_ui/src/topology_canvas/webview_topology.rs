//! WebView-based topology - MVP version
//! Replaces complex GPUI canvas rendering with D3.js

use gpui::*;
use std::sync::Arc;
use serde_json::json;

use crate::repository::AssetRepository;
use crate::topology_canvas::{AssetSelectedEvent, NetworkLinkInfo};

pub struct WebViewTopology {
    focus_handle: FocusHandle,
}

impl WebViewTopology {
    pub fn new<R: AssetRepository>(
        window: &mut Window,
        cx: &mut Context<Self>,
        repository: &R,
    ) -> Self {
        let focus_handle = cx.focus_handle();
        
        // Load assets and create D3 data
        let assets = repository.get_all_assets();
        let links = Self::calculate_links(&assets);
        
        // Convert to JSON for D3
        let nodes_json: Vec<serde_json::Value> = assets.iter().map(|a| {
            json!({
                "id": a.id.to_string(),
                "name": a.name,
                "group": a.zone_id.clone().unwrap_or_else(|| "Z1".to_string()),
                "risk_score": a.risk_score,
                "ip": a.ip_address.clone().unwrap_or_default(),
                "network": a.network_segment,
            })
        }).collect();
        
        let links_json: Vec<serde_json::Value> = links.iter().map(|l| {
            json!({
                "source": l.source_id,
                "target": l.target_id,
                "type": l.action,
                "protocol": l.protocol,
            })
        }).collect();
        
        tracing::info!("WebViewTopology: {} nodes, {} links", nodes_json.len(), links_json.len());
        
        Self {
            focus_handle,
        }
    }
    
    /// Simple link calculation for MVP
    fn calculate_links(assets: &[data::models::Asset]) -> Vec<NetworkLinkInfo> {
        let mut links = Vec::new();
        let mut link_set = std::collections::HashSet::new();
        
        // Group by network
        let mut network_assets: std::collections::HashMap<String, Vec<i64>> = std::collections::HashMap::new();
        for asset in assets {
            if !asset.network_segment.is_empty() {
                network_assets.entry(asset.network_segment.clone())
                    .or_default()
                    .push(asset.id);
            }
        }
        
        // Create links from accessible_networks
        for source in assets {
            for accessible in &source.accessible_networks {
                if let Some(targets) = network_assets.get(accessible) {
                    for &target_id in targets {
                        if source.id == target_id {
                            continue;
                        }
                        
                        let key = format!("{}-{}", source.id.min(target_id), source.id.max(target_id));
                        if !link_set.contains(&key) {
                            link_set.insert(key);
                            links.push(NetworkLinkInfo {
                                source_id: source.id.to_string(),
                                target_id: target_id.to_string(),
                                action: "network".to_string(),
                                direction: "bidirectional".to_string(),
                                protocol: "IP".to_string(),
                                port_range: "-".to_string(),
                            });
                            
                            if links.len() >= 300 {
                                return links;
                            }
                        }
                    }
                }
            }
        }
        
        links
    }
    
    /// Generate HTML with embedded D3.js
    pub fn generate_html(nodes: &[serde_json::Value], links: &[serde_json::Value]) -> String {
        let nodes_str = serde_json::to_string(nodes).unwrap_or_default();
        let links_str = serde_json::to_string(links).unwrap_or_default();
        
        format!(r#"<!DOCTYPE html>
<html>
<head>
<meta charset="UTF-8">
<script src="https://d3js.org/d3.v7.min.js"></script>
<style>
body {{ margin: 0; padding: 0; overflow: hidden; background: #f0f2f5; font-family: system-ui, sans-serif; }}
svg {{ width: 100vw; height: 100vh; }}
.node {{ cursor: pointer; stroke: #fff; stroke-width: 2px; }}
.node:hover {{ stroke: #ff4757; stroke-width: 3px; }}
.link {{ stroke: #747d8c; stroke-opacity: 0.6; stroke-width: 1.5px; }}
.label {{ font-size: 10px; fill: #2f3542; pointer-events: none; }}
</style>
</head>
<body>
<svg id="graph"></svg>
<script>
const nodes = {nodes};
const links = {links};

const color = d3.scaleOrdinal()
    .domain(['Z1','Z2','Z3','Z4','Z5'])
    .range(['#3498db','#2ecc71','#9b59b6','#f39c12','#e74c3c']);

const width = window.innerWidth, height = window.innerHeight;
const svg = d3.select("#graph");
const g = svg.append("g");

svg.call(d3.zoom().on("zoom", e => g.attr("transform", e.transform)));

const sim = d3.forceSimulation(nodes)
    .force("link", d3.forceLink(links).id(d => d.id).distance(100))
    .force("charge", d3.forceManyBody().strength(-400))
    .force("center", d3.forceCenter(width/2, height/2))
    .force("collide", d3.forceCollide().radius(20));

const link = g.append("g").selectAll("line").data(links).join("line").attr("class", "link");
const node = g.append("g").selectAll("circle").data(nodes).join("circle")
    .attr("class", "node").attr("r", 8).attr("fill", d => color(d.group))
    .call(d3.drag().on("start", (e,d) => {{ if(!e.active) sim.alphaTarget(0.3).restart(); d.fx=d.x; d.fy=d.y; }})
                  .on("drag", (e,d) => {{ d.fx=e.x; d.fy=e.y; }})
                  .on("end", (e,d) => {{ if(!e.active) sim.alphaTarget(0); d.fx=null; d.fy=null; }}));

const label = g.append("g").selectAll("text").data(nodes).join("text")
    .attr("class", "label").text(d => d.name?.substring(0,10))
    .attr("dx", 10).attr("dy", 3);

node.on("click", (e,d) => {{
    console.log("CLICK:"+d.id);
    if(window.webkit) window.webkit.messageHandlers.nodeClicked.postMessage(d.id);
}});

sim.on("tick", () => {{
    link.attr("x1",d=>d.source.x).attr("y1",d=>d.source.y).attr("x2",d=>d.target.x).attr("y2",d=>d.target.y);
    node.attr("cx",d=>d.x).attr("cy",d=>d.y);
    label.attr("x",d=>d.x).attr("y",d=>d.y);
}});
</script>
</body>
</html>"#, nodes=nodes_str, links=links_str)
    }
}

impl Focusable for WebViewTopology {
    fn focus_handle(&self, _cx: &gpui::App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for WebViewTopology {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        // For MVP, return a placeholder - actual WebView integration needs gpui_wry
        div()
            .size_full()
            .child("WebView Topology - Placeholder")
    }
}