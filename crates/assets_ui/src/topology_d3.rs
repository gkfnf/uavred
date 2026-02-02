//! D3.js-based topology view using WebView
//! MVP - One day implementation

use gpui::*;
use std::sync::Arc;

/// Simple WebView topology placeholder
/// Full implementation requires gpui_wry integration
pub struct D3TopologyView {
    focus_handle: FocusHandle,
    html_content: String,
}

impl D3TopologyView {
    pub fn new(assets: &[crate::repository::AssetData], links: &[crate::topology_canvas::NetworkLinkInfo]) -> Self {
        let focus_handle = FocusHandle::new(&gpui::App::new(|_|{}).unwrap());
        
        // Generate D3 HTML
        let html = Self::generate_d3_html(assets, links);
        
        Self {
            focus_handle,
            html_content: html,
        }
    }
    
    fn generate_d3_html(assets: &[crate::repository::AssetData], links: &[crate::topology_canvas::NetworkLinkInfo]) -> String {
        // Convert to JSON strings
        let nodes_json = assets.iter().map(|a| {
            format!(r#"{{"id":"{}","name":"{}","group":"{}","risk":{}}}"#,
                a.id, 
                a.name.replace("\"", "\\\""),
                a.zone_id.as_deref().unwrap_or("Z1"),
                a.risk_score
            )
        }).collect::<Vec<_>>().join(",");
        
        let links_json = links.iter().map(|l| {
            format!(r#"{{"source":"{}","target":"{}"}}"#,
                l.source_id, l.target_id
            )
        }).collect::<Vec<_>>().join(",");
        
        format!(r##"<!DOCTYPE html>
<html><head><meta charset="UTF-8">
<script src="https://d3js.org/d3.v7.min.js"></script>
<style>
body{{margin:0;padding:0;overflow:hidden;background:#f5f5f5;font-family:system-ui}}
svg{{width:100vw;height:100vh}}
.node{{cursor:pointer;stroke:#fff;stroke-width:2px}}
.node:hover{{stroke:#e74c3c;stroke-width:3px}}
.link{{stroke:#95a5a6;stroke-opacity:.6;stroke-width:2px}}
.label{{font-size:11px;fill:#2c3e50;pointer-events:none}}
</style></head><body>
<svg id="g"></svg>
<script>
const nodes=[{}];
const links=[{}];
const w=window.innerWidth,h=window.innerHeight;
const svg=d3.select("#g"),g=svg.append("g");
svg.call(d3.zoom().on("zoom",e=>g.attr("transform",e.transform)));
const c=d3.scaleOrdinal().domain(['Z1','Z2','Z3','Z4','Z5']).range(['#3498db','#2ecc71','#9b59b6','#f39c12','#e74c3c']);
const s=d3.forceSimulation(nodes).force("l",d3.forceLink(links).id(d=>d.id).distance(100)).force("c",d3.forceManyBody().strength(-400)).force("x",d3.forceCenter(w/2,h/2)).force("o",d3.forceCollide().radius(25));
const L=g.append("g").selectAll("line").data(links).join("line").attr("class","link");
const N=g.append("g").selectAll("circle").data(nodes).join("circle").attr("class","node").attr("r",d=>d.risk>70?10:8).attr("fill",d=>c(d.group)).call(d3.drag().on("start",(e,d)=>{{if(!e.active)s.alphaTarget(0.3).restart();d.fx=d.x;d.fy=d.y}}).on("drag",(e,d)=>{{d.fx=e.x;d.fy=e.y}}).on("end",(e,d)=>{{if(!e.active)s.alphaTarget(0);d.fx=null;d.fy=null}}));
const T=g.append("g").selectAll("text").data(nodes).join("text").attr("class","label").text(d=>d.name.slice(0,12)).attr("dx",12).attr("dy",4);
N.on("click",(e,d)=>{{console.log("NODE_CLICK:"+d.id);e.stopPropagation();}});
s.on("tick",()=>{{L.attr("x1",d=>d.source.x).attr("y1",d=>d.source.y).attr("x2",d=>d.target.x).attr("y2",d=>d.target.y);N.attr("cx",d=>d.x).attr("cy",d=>d.y);T.attr("x",d=>d.x).attr("y",d=>d.y)}});
</script></body></html>"##, nodes_json, links_json)
    }
}

impl Focusable for D3TopologyView {
    fn focus_handle(&self, _cx: &gpui::App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for D3TopologyView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        // For MVP, show the HTML content size as placeholder
        div()
            .size_full()
            .bg(rgb(0xf5f5f5))
            .flex()
            .items_center()
            .justify_center()
            .child(format!("D3 Topology ({} bytes HTML)", self.html_content.len()))
    }
}