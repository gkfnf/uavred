//! WebView-based topology visualization using D3.js
//! MVP version - basic force-directed graph with click support

use gpui::*;
use std::sync::Arc;

/// WebView-based topology canvas
pub struct TopologyWebView {
    webview: Entity<WebView>,
    _assets: Arc<Vec<serde_json::Value>>,
    _links: Arc<Vec<serde_json::Value>>,
}

impl TopologyWebView {
    pub fn new(
        window: &mut Window,
        cx: &mut App,
        assets: Vec<serde_json::Value>,
        links: Vec<serde_json::Value>,
        on_node_click: Arc<dyn Fn(String) + Send + Sync>,
    ) -> Entity<Self> {
        let assets_clone = Arc::new(assets.clone());
        let links_clone = Arc::new(links.clone());
        
        let html = generate_d3_html(&assets, &links);
        
        let webview = cx.new(|cx| {
            let builder = wry::WebViewBuilder::new()
                .with_html(&html);
            
            #[cfg(any(
                target_os = "windows",
                target_os = "macos",
                target_os = "ios",
                target_os = "android"
            ))]
            let wry_webview = {
                use raw_window_handle::HasWindowHandle;
                let window_handle = window.window_handle().expect("No window handle");
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
            
            // Create GPUI WebView wrapper
            WebView::new(wry_webview, window, cx)
        });
        
        cx.new(|_cx| Self {
            webview,
            _assets: assets_clone,
            _links: links_clone,
        })
    }
    
    pub fn webview(&self) -> Entity<WebView> {
        self.webview.clone()
    }
}

/// Generate D3.js HTML with embedded data
fn generate_d3_html(assets: &[serde_json::Value], links: &[serde_json::Value]) -> String {
    let nodes_json = serde_json::to_string(assets).unwrap();
    let links_json = serde_json::to_string(links).unwrap();
    
    format!(r##"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <script src="https://d3js.org/d3.v7.min.js"></script>
    <style>
        body {{
            margin: 0;
            padding: 0;
            overflow: hidden;
            background: #f8f9fa;
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
        }}
        #graph {{
            width: 100vw;
            height: 100vh;
        }}
        .node {{
            cursor: pointer;
            stroke: #fff;
            stroke-width: 2px;
            transition: all 0.2s;
        }}
        .node:hover {{
            stroke: #ff6b6b;
            stroke-width: 3px;
        }}
        .link {{
            stroke: #999;
            stroke-opacity: 0.6;
            stroke-width: 1.5px;
        }}
        .link.service {{
            stroke: #4dabf7;
        }}
        .link.network {{
            stroke: #adb5bd;
        }}
        .label {{
            font-size: 11px;
            fill: #495057;
            pointer-events: none;
            text-shadow: 0 1px 3px rgba(255,255,255,0.8);
        }}
        .tooltip {{
            position: absolute;
            padding: 8px 12px;
            background: rgba(0,0,0,0.8);
            color: white;
            border-radius: 4px;
            font-size: 12px;
            pointer-events: none;
            opacity: 0;
            transition: opacity 0.2s;
        }}
    </style>
</head>
<body>
    <div id="graph"></div>
    <div class="tooltip" id="tooltip"></div>
    
    <script>
        // Data from Rust
        const nodes = {nodes_json};
        const links = {links_json};
        
        // Color scale by zone
        const colorScale = d3.scaleOrdinal()
            .domain(['Z1', 'Z2', 'Z3', 'Z4', 'Z5'])
            .range(['#339af0', '#51cf66', '#845ef7', '#ff922b', '#ff6b6b']);
        
        // Setup SVG
        const width = window.innerWidth;
        const height = window.innerHeight;
        
        const svg = d3.select('#graph')
            .append('svg')
            .attr('width', width)
            .attr('height', height)
            .attr('viewBox', [0, 0, width, height]);
        
        // Add zoom behavior
        const g = svg.append('g');
        
        svg.call(d3.zoom()
            .extent([[0, 0], [width, height]])
            .scaleExtent([0.1, 4])
            .on('zoom', (event) => {{
                g.attr('transform', event.transform);
            }}));
        
        // Force simulation
        const simulation = d3.forceSimulation(nodes)
            .force('link', d3.forceLink(links).id(d => d.id).distance(80))
            .force('charge', d3.forceManyBody().strength(-300))
            .force('center', d3.forceCenter(width / 2, height / 2))
            .force('collision', d3.forceCollide().radius(25));
        
        // Draw links
        const link = g.append('g')
            .attr('class', 'links')
            .selectAll('line')
            .data(links)
            .join('line')
            .attr('class', d => `link ${{d.type || 'network'}}`);
        
        // Draw nodes
        const node = g.append('g')
            .attr('class', 'nodes')
            .selectAll('circle')
            .data(nodes)
            .join('circle')
            .attr('class', 'node')
            .attr('r', d => d.risk_score > 70 ? 10 : d.risk_score > 40 ? 8 : 6)
            .attr('fill', d => colorScale(d.group || 'Z1'))
            .call(d3.drag()
                .on('start', dragstarted)
                .on('drag', dragged)
                .on('end', dragended));
        
        // Draw labels
        const label = g.append('g')
            .attr('class', 'labels')
            .selectAll('text')
            .data(nodes)
            .join('text')
            .attr('class', 'label')
            .text(d => d.name?.length > 15 ? d.name.substring(0, 12) + '...' : d.name)
            .attr('dx', 12)
            .attr('dy', 4);
        
        // Click handler - notify Rust
        node.on('click', function(event, d) {{
            event.stopPropagation();
            console.log('Node clicked:', d.id, d.name);
            
            // Send to Rust via console.log bridge or custom protocol
            if (window.gpui) {{
                window.gpui.nodeClicked(d.id, d.name);
            }}
            
            // Visual feedback
            d3.selectAll('.node').attr('stroke', '#fff');
            d3.select(this).attr('stroke', '#ff6b6b').attr('stroke-width', 4);
        }});
        
        // Hover tooltip
        node.on('mouseover', function(event, d) {{
            const tooltip = d3.select('#tooltip');
            tooltip.html(`<strong>${{d.name}}</strong><br/>IP: ${{d.ip || 'N/A'}}<br/>Risk: ${{d.risk_score || 0}}`)
                .style('left', (event.pageX + 10) + 'px')
                .style('top', (event.pageY - 10) + 'px')
                .style('opacity', 1);
        }}).on('mouseout', function() {{
            d3.select('#tooltip').style('opacity', 0);
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
            
            label
                .attr('x', d => d.x)
                .attr('y', d => d.y);
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
        
        // Expose to window for Rust to call
        window.updateData = function(newNodes, newLinks) {{
            // Update logic here
            console.log('Data update requested');
        }};
    </script>
</body>
</html>"##, nodes_json = nodes_json, links_json = links_json)
}

// Placeholder WebView struct - actual implementation depends on gpui_wry
pub struct WebView {
    // Implementation details
}

impl WebView {
    pub fn new(_webview: wry::WebView, _window: &mut Window, _cx: &mut App) -> Self {
        Self {}
    }
}