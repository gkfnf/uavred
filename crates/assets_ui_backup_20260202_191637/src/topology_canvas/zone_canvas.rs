//! Zone Canvas - Individual zone viewport with nodes and interactions

use gpui::*;
use gpui::prelude::FluentBuilder;

use data::models::{AssetNode, ZoneType, Asset, Severity, Connection, ScanProgress, ComplianceStandard, ComplianceStatus};
use ui::theme::*;

use crate::config::{theme_ext::*, ZoneTypeExt};
use crate::repository::AssetRepository;

use super::camera::{calculate_zoom_from_scroll, Camera, VirtualBounds};

/// Canvas state for a single security zone
pub struct ZoneCanvas {
    zone: ZoneType,
    nodes: Vec<AssetNode>,
    node_positions: Vec<NodeVirtualPos>,
    camera: Camera,
    is_dragging: bool,
    last_mouse_pos: Option<Point<Pixels>>,
    // 存储实际的viewport尺寸，用于hit_test
    actual_viewport: (f32, f32),
    // 网络ACL链接信息
    network_links: Vec<NetworkLinkInfo>,
}

/// Network link info for ACL-based topology
#[derive(Clone, Debug)]
pub struct NetworkLinkInfo {
    pub source_id: String,
    pub target_id: String,
    pub action: String, // "allow", "deny", "drop"
    pub direction: String, // "outbound", "inbound", "bidirectional"
    pub protocol: String,
    pub port_range: String,
}

/// Node position in virtual coordinates
#[derive(Clone, Debug)]
pub struct NodeVirtualPos {
    pub x: f32,
    pub y: f32,
}

impl ZoneCanvas {
    /// Create a new zone canvas with positioned nodes and auto-centered view
    pub fn new<R: AssetRepository>(
        zone: ZoneType,
        repository: &R,
        zone_virtual_width: f32,
        zone_virtual_height: f32,
    ) -> Self {
        let nodes = repository.get_assets_by_zone(zone);
        let node_positions =
            Self::calculate_node_positions(&nodes, zone_virtual_width, zone_virtual_height);

        let camera = Camera::new(zone_virtual_width, zone_virtual_height);

        let mut canvas = Self {
            zone,
            nodes,
            node_positions,
            camera,
            is_dragging: false,
            last_mouse_pos: None,
            actual_viewport: (zone_virtual_width, zone_virtual_height),
            network_links: Vec::new(),
        };

        // Auto-fit to show all nodes initially
        canvas.fit_to_view();

        canvas
    }

    /// Create a new zone canvas from raw Asset data (for database integration)
    pub fn new_with_assets(
        zone: ZoneType,
        assets: &[Asset],
        zone_virtual_width: f32,
        zone_virtual_height: f32,
    ) -> Self {
        // Filter assets by zone and convert to AssetNode
        let zone_str = zone.as_str();
        let nodes: Vec<AssetNode> = assets
            .iter()
            .filter(|a| a.zone_id.as_deref() == Some(zone_str))
            .map(|asset| Self::asset_to_node(asset, assets))
            .collect();
        
        let node_positions =
            Self::calculate_node_positions(&nodes, zone_virtual_width, zone_virtual_height);

        // Calculate network reachability links for this zone's assets
        let network_links = Self::calculate_zone_network_links(zone, assets);

        let camera = Camera::new(zone_virtual_width, zone_virtual_height);

        let mut canvas = Self {
            zone,
            nodes,
            node_positions,
            camera,
            is_dragging: false,
            last_mouse_pos: None,
            actual_viewport: (zone_virtual_width, zone_virtual_height),
            network_links,
        };

        // Auto-fit to show all nodes initially
        canvas.fit_to_view();

        canvas
    }

    /// Calculate network links for assets in this zone based on accessible_networks
    fn calculate_zone_network_links(zone: ZoneType, assets: &[Asset]) -> Vec<NetworkLinkInfo> {
        let zone_str = zone.as_str();
        let zone_assets: Vec<&Asset> = assets
            .iter()
            .filter(|a| a.zone_id.as_deref() == Some(zone_str))
            .collect();

        let mut links = Vec::new();

        for source in &zone_assets {
            if source.accessible_networks.is_empty() {
                continue;
            }

            for target in assets {
                if source.id == target.id || target.ip_address.is_none() {
                    continue;
                }

                // Check if target's network segment is in source's accessible networks
                let target_segment = &target.network_segment;
                if target_segment.is_empty() {
                    continue;
                }

                if Self::network_in_list(target_segment, &source.accessible_networks) {
                    // Check if reverse connection exists (bidirectional check)
                    let is_bidirectional = target.accessible_networks.iter().any(|net| {
                        source.network_segment == *net || 
                        (net == "0.0.0.0/0" && !source.network_segment.is_empty())
                    });

                    links.push(NetworkLinkInfo {
                        source_id: source.id.to_string(),
                        target_id: target.id.to_string(),
                        action: "allow".to_string(),
                        direction: if is_bidirectional { "bidirectional".to_string() } else { "outbound".to_string() },
                        protocol: if target.protocol.is_empty() { "TCP".to_string() } else { target.protocol.clone() },
                        port_range: Self::extract_ports_from_services(&target.services),
                    });
                }
            }
        }

        links
    }

    /// Check if a network segment is in the list of accessible networks
    fn network_in_list(network: &str, accessible_list: &[String]) -> bool {
        if accessible_list.iter().any(|n| n == "0.0.0.0/0" || n == "any") {
            return true;
        }
        
        // Normalize network (remove /32 suffix if present)
        let normalized = network.trim_end_matches("/32");
        
        accessible_list.iter().any(|accessible| {
            let acc_norm = accessible.trim_end_matches("/32");
            // Direct match
            acc_norm == normalized || 
            // Check CIDR overlap (simplified)
            Self::cidr_contains(accessible, normalized)
        })
    }

    /// Check if a CIDR contains an IP or smaller network (simplified)
    fn cidr_contains(cidr: &str, ip_or_net: &str) -> bool {
        use std::net::Ipv4Addr;
        
        let Some((net_ip, mask_str)) = cidr.split_once('/') else {
            return cidr == ip_or_net;
        };
        
        let Ok(mask_bits): Result<u32, _> = mask_str.parse() else {
            return false;
        };

        let net_parts: Vec<u8> = net_ip.split('.').filter_map(|s| s.parse().ok()).collect();
        let ip_parts: Vec<u8> = ip_or_net.split('.').filter_map(|s| s.parse().ok()).collect();
        
        if net_parts.len() != 4 || ip_parts.len() != 4 {
            return false;
        }

        let net_u32 = ((net_parts[0] as u32) << 24) | ((net_parts[1] as u32) << 16) | 
                      ((net_parts[2] as u32) << 8) | (net_parts[3] as u32);
        let ip_u32 = ((ip_parts[0] as u32) << 24) | ((ip_parts[1] as u32) << 16) | 
                     ((ip_parts[2] as u32) << 8) | (ip_parts[3] as u32);
        
        let mask = if mask_bits == 0 { 0 } else { !((1u32 << (32 - mask_bits)) - 1) };
        
        (ip_u32 & mask) == (net_u32 & mask)
    }

    /// Extract ports from services (safe version that handles empty services)
    fn extract_ports_from_services(services: &[data::models::AssetService]) -> String {
        if services.is_empty() {
            return "-".to_string();
        }
        
        let ports: Vec<String> = services.iter()
            .filter(|s| s.port > 0)
            .map(|s| s.port.to_string())
            .collect();
        
        if ports.is_empty() {
            "-".to_string()
        } else {
            ports.join(", ")
        }
    }

    /// Convert Asset database model to AssetNode UI model
    fn asset_to_node(asset: &Asset, all_assets: &[Asset]) -> AssetNode {
        // Determine severity based on risk score
        let severity = if asset.risk_score >= 70 {
            Severity::High
        } else if asset.risk_score >= 40 {
            Severity::Medium
        } else {
            Severity::Low
        };

        // Parse zone from zone_id
        let zone = asset.zone_id.as_deref()
            .map(ZoneType::from)
            .unwrap_or(ZoneType::Z1);

        // Convert services to strings
        let services: Vec<String> = asset.services.iter()
            .map(|s| format!("{}:{}", s.service_name, s.port))
            .collect();

        // Extract open ports
        let open_ports: Vec<u16> = asset.services.iter()
            .filter(|s| s.port > 0 && s.port <= 65535)
            .map(|s| s.port as u16)
            .collect();

        // Convert connections - look up target asset IDs
        let connections: Vec<Connection> = asset.connections.iter()
            .filter_map(|c| {
                let is_source = c.source_asset_id == asset.id;
                let other_asset_id = if is_source { c.target_asset_id } else { c.source_asset_id };
                
                // Look up the other asset to get its info
                let target_id = all_assets.iter()
                    .find(|a| a.id == other_asset_id)
                    .map(|a| a.id.to_string())
                    .unwrap_or_else(|| other_asset_id.to_string());

                Some(Connection {
                    target_id,
                    connection_type: c.connection_type.clone(),
                    protocol: c.protocol.clone(),
                    port: 0,
                })
            })
            .collect();

        // Build compliance standards
        let compliance_standards: Vec<ComplianceStandard> = asset.compliance_standards.iter()
            .map(|name| ComplianceStandard {
                name: name.clone(),
                status: ComplianceStatus::Compliant,
                last_audit: None,
            })
            .collect();

        AssetNode {
            id: asset.id.to_string(),
            name: asset.name.clone(),
            ip_address: asset.ip_address.clone().unwrap_or_default(),
            mac_address: asset.mac_address.clone(),
            zone,
            severity,
            risk_score: asset.risk_score,
            vulnerabilities_count: asset.vuln_count,
            services,
            open_ports,
            credentials: Vec::new(),
            owner: asset.owner_team.clone(),
            business_purpose: asset.business_purpose.clone(),
            department: None,
            scan_progress: ScanProgress {
                percentage: 100,
                last_scan: asset.last_scan_at,
                next_scan: None,
                scan_type: "Full".to_string(),
                scanning: false,
            },
            compliance_standards,
            connections,
            status: asset.status.clone(),
            last_seen: asset.updated_at.format("%Y-%m-%d %H:%M").to_string(),
            asset_type: asset.asset_type.clone(),
            firmware_version: if asset.firmware_version.is_empty() { None } else { Some(asset.firmware_version.clone()) },
            manufacturer: if asset.model.is_empty() { None } else { Some(asset.model.clone()) },
            location: None,
        }
    }

    /// Get zone type
    pub fn zone(&self) -> ZoneType {
        self.zone
    }

    /// Get nodes reference
    pub fn nodes(&self) -> &[AssetNode] {
        &self.nodes
    }

    /// Get node positions reference (for debugging)
    pub fn node_positions(&self) -> &[NodeVirtualPos] {
        &self.node_positions
    }

    /// Get node count
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Get connection count
    pub fn connection_count(&self) -> usize {
        self.nodes.iter().map(|n| n.connections.len()).sum()
    }

    /// Calculate the bounding box of all nodes
    fn calculate_node_bounds(&self) -> VirtualBounds {
        let points: Vec<(f32, f32)> = self
            .node_positions
            .iter()
            .map(|pos| (pos.x, pos.y))
            .collect();
        VirtualBounds::from_points(&points, 80.0) // 80px padding
    }

    /// Fit the view to show all nodes centered
    pub fn fit_to_view(&mut self) {
        if self.nodes.is_empty() {
            self.camera.scale = 1.0;
            self.camera.offset_x = 0.0;
            self.camera.offset_y = 0.0;
            return;
        }

        // Calculate actual bounds of nodes with padding
        let bounds = self.calculate_node_bounds();
        // Use 1.15 padding (15% space around nodes) for comfortable viewing
        self.camera.fit_to_bounds(&bounds, 1.15);
    }

    /// Reset view to initial state
    pub fn reset_view(&mut self) {
        self.fit_to_view();
    }

    /// Handle scroll wheel - zoom centered on viewport center
    pub fn handle_scroll(&mut self, event: &ScrollWheelEvent) {
        let delta_y = match &event.delta {
            ScrollDelta::Pixels(p) => {
                let dy: f32 = p.y.into();
                dy
            }
            ScrollDelta::Lines(l) => l.y * 30.0,
        };

        // Calculate new scale
        let new_scale = calculate_zoom_from_scroll(delta_y, self.camera.scale);

        // Zoom at center of viewport
        let center_x = self.camera.viewport_width / 2.0;
        let center_y = self.camera.viewport_height / 2.0;
        self.camera.zoom_at(center_x, center_y, new_scale);
    }

    /// Start drag operation
    pub fn start_drag(&mut self, position: Point<Pixels>) {
        self.is_dragging = true;
        self.last_mouse_pos = Some(position);
    }

    /// End drag operation
    pub fn end_drag(&mut self) {
        self.is_dragging = false;
        self.last_mouse_pos = None;
    }

    /// Update drag - pan the camera
    pub fn update_drag(&mut self, position: Point<Pixels>) {
        if let Some(last) = self.last_mouse_pos {
            let dx: f32 = (position.x - last.x).into();
            let dy: f32 = (position.y - last.y).into();

            // Pan: when we drag right (positive dx), content moves right, so offset decreases
            // Actually: to pan the view right, we need to move the content left
            // But in our coordinate system:
            // screen_x = (virtual_x - offset_x) * scale
            // To make content appear to move right on screen, we decrease offset_x
            // But when we drag mouse right, we want to pan right, so:
            // offset_x = offset_x - dx / scale
            self.camera.offset_x -= dx / self.camera.scale;
            self.camera.offset_y -= dy / self.camera.scale;

            self.last_mouse_pos = Some(position);
        }
    }

    /// Check if currently dragging
    pub fn is_dragging(&self) -> bool {
        self.is_dragging
    }

    /// Hit test for node selection - use screen coordinates directly
    pub fn hit_test(&self, canvas_x: f32, canvas_y: f32) -> Option<String> {
        if self.is_dragging {
            return None;
        }

        // Convert canvas (screen) coordinates to virtual coordinates
        let (virtual_x, virtual_y) = self.camera.screen_to_virtual(canvas_x, canvas_y);
        
        // Larger hit radius for easier selection
        let hit_radius_virtual = 30.0 / self.camera.scale;

        let mut closest_node: Option<(String, f32)> = None;

        for (idx, node) in self.nodes.iter().enumerate() {
            if let Some(pos) = self.node_positions.get(idx) {
                let dx = pos.x - virtual_x;
                let dy = pos.y - virtual_y;
                let dist_sq = dx * dx + dy * dy;
                let dist = dist_sq.sqrt();

                if dist <= hit_radius_virtual {
                    if closest_node.is_none() || dist < closest_node.as_ref().unwrap().1 {
                        closest_node = Some((node.id.clone(), dist));
                    }
                }
            }
        }

        closest_node.map(|(id, _)| id)
    }

    /// Calculate node positions - Force-directed layout (Obsidian-style)
    /// Nodes naturally distribute in center area, connected nodes cluster together
    fn calculate_node_positions(
        nodes: &[AssetNode],
        zone_width: f32,
        zone_height: f32,
    ) -> Vec<NodeVirtualPos> {
        let count = nodes.len();
        if count == 0 {
            return vec![];
        }
        if count == 1 {
            return vec![NodeVirtualPos { x: zone_width / 2.0, y: zone_height / 2.0 }];
        }

        // Initialize with random positions in center area (not spread out)
        let mut positions: Vec<NodeVirtualPos> = nodes
            .iter()
            .enumerate()
            .map(|(i, node)| {
                let hash = Self::hash(&node.id);
                // Start in a small circle near center
                let angle = (hash % 360) as f32 * std::f32::consts::PI / 180.0;
                let radius = (hash % 30) as f32 + 10.0;
                NodeVirtualPos {
                    x: zone_width / 2.0 + radius * angle.cos(),
                    y: zone_height / 2.0 + radius * angle.sin(),
                }
            })
            .collect();

        // Build connection map
        let mut connections: std::collections::HashMap<usize, Vec<usize>> = std::collections::HashMap::new();
        for (idx, node) in nodes.iter().enumerate() {
            for conn in &node.connections {
                if let Some(target_idx) = nodes.iter().position(|n| n.id == conn.target_id) {
                    connections.entry(idx).or_default().push(target_idx);
                    connections.entry(target_idx).or_default().push(idx);
                }
            }
        }

        // Force-directed iterations - gentler forces for organic clustering
        let iterations = 150;
        // Smaller ideal distance keeps nodes closer together
        let k = 40.0 + (zone_width * zone_height / count as f32).sqrt() * 0.3;

        for iter in 0..iterations {
            let mut forces: Vec<(f32, f32)> = vec![(0.0, 0.0); count];

            // Repulsion: all nodes repel each other (weaker for closer distribution)
            for i in 0..count {
                for j in (i + 1)..count {
                    let dx = positions[i].x - positions[j].x;
                    let dy = positions[i].y - positions[j].y;
                    let dist_sq = dx * dx + dy * dy;
                    let dist = dist_sq.sqrt().max(0.1);

                    // Gentler repulsion
                    let force = (k * k) / (dist_sq + 100.0);
                    let fx = (dx / dist) * force;
                    let fy = (dy / dist) * force;

                    forces[i].0 += fx;
                    forces[i].1 += fy;
                    forces[j].0 -= fx;
                    forces[j].1 -= fy;
                }
            }

            // Attraction: connected nodes attract (stronger for clustering)
            for (&i, targets) in &connections {
                for &j in targets {
                    if i >= j { continue; }
                    let dx = positions[j].x - positions[i].x;
                    let dy = positions[j].y - positions[i].y;
                    let dist = (dx * dx + dy * dy).sqrt().max(0.1);

                    // Stronger attraction for connected nodes
                    let force = (dist * dist) / (k * 0.8);
                    let fx = (dx / dist) * force * 0.8;
                    let fy = (dy / dist) * force * 0.8;

                    forces[i].0 += fx;
                    forces[i].1 += fy;
                    forces[j].0 -= fx;
                    forces[j].1 -= fy;
                }
            }

            // Gentle center gravity to keep nodes in view
            let center_x = zone_width / 2.0;
            let center_y = zone_height / 2.0;
            for i in 0..count {
                let dx = center_x - positions[i].x;
                let dy = center_y - positions[i].y;
                // Very gentle pull to center
                forces[i].0 += dx * 0.003;
                forces[i].1 += dy * 0.003;
            }

            // Apply forces with decreasing cooling
            let cooling = 0.5 * (1.0 - iter as f32 / iterations as f32);
            for i in 0..count {
                positions[i].x += forces[i].0 * cooling;
                positions[i].y += forces[i].1 * cooling;
            }
        }

        // Final pass: gently push nodes away from edges back toward center
        let margin = 50.0;
        for i in 0..count {
            let mut fx = 0.0;
            let mut fy = 0.0;
            
            if positions[i].x < margin {
                fx = (margin - positions[i].x) * 0.1;
            } else if positions[i].x > zone_width - margin {
                fx = -(positions[i].x - (zone_width - margin)) * 0.1;
            }
            
            if positions[i].y < margin {
                fy = (margin - positions[i].y) * 0.1;
            } else if positions[i].y > zone_height - margin {
                fy = -(positions[i].y - (zone_height - margin)) * 0.1;
            }
            
            positions[i].x += fx;
            positions[i].y += fy;
        }

        positions
    }
    
    /// Extract network segment from IP address
    fn extract_network_segment(ip: &str) -> String {
        if ip.is_empty() {
            return String::new();
        }
        
        // For IPs like 192.168.1.100, return 192.168.1.0/24
        let parts: Vec<&str> = ip.split('.').collect();
        if parts.len() == 4 {
            format!("{}.{}.{}.0/24", parts[0], parts[1], parts[2])
        } else {
            // For domains, return as-is
            ip.to_string()
        }
    }
    
    /// Extract C-segment (third octet) from IP address
    /// e.g., "192.168.1.100" -> 1, "10.0.5.20" -> 5
    fn extract_c_segment(ip: &str) -> u8 {
        ip.split('.')
            .nth(2)
            .and_then(|s| s.parse().ok())
            .unwrap_or(0)
    }

    /// Simple hash function for deterministic jitter
    fn hash(s: &str) -> u64 {
        let mut h: u64 = 5381;
        for b in s.bytes() {
            h = h.wrapping_mul(33).wrapping_add(b as u64);
        }
        h
    }

    /// Render the zone canvas with global network links
    pub fn render_canvas(
        &mut self,
        zone_idx: usize,
        selected_id: &Option<String>,
        hovered_id: &Option<String>,
        global_links: &[NetworkLinkInfo],
        cx: &mut Context<super::TopologyCanvas>,
    ) -> AnyElement {
        let zone_config = self.zone.config();
        let bg_color = zone_config.bg_color;

        // Clone necessary data for the paint closure
        let camera = self.camera.clone();
        let nodes_data: Vec<_> = self
            .nodes
            .iter()
            .map(|n| {
                (
                    n.id.clone(),
                    n.name.clone(),  // Name for labels
                    n.asset_type.clone(),
                    n.severity.clone(),
                    n.connections.clone(),
                    n.ip_address.clone(),  // IP for network segment extraction
                )
            })
            .collect();
        let positions = self.node_positions.clone();
        let selected_id_clone = selected_id.clone();
        let hovered_id_clone = hovered_id.clone();
        // Use global links for Obsidian-style graph
        let network_links = global_links.to_vec();

        let is_last = zone_idx == 4;

        // The div fills the zone area, overflow_hidden clips the canvas content
        div()
            .flex_1()
            .h_full()
            .when(!is_last, |this: gpui::Div| {
                this.border_r_1().border_color(rgb(BORDER_COLOR))
            })
            .overflow_hidden()
            .child(
                // canvas element fills the div completely
                canvas(
                    // Layout pass - update camera viewport
                    move |bounds, _window, _cx| {
                        // 更新camera的viewport尺寸以匹配实际渲染区域
                        let width: f32 = bounds.size.width.into();
                        let height: f32 = bounds.size.height.into();
                        // 使用Arc<Mutex<>>来共享和更新camera
                        // 但这里我们无法直接修改camera，所以需要在其他地方处理
                        // 暂时记录日志
                        tracing::debug!("[CANVAS] Zone {:?} bounds: {:.1}x{:.1}", bg_color, width, height);
                    },
                    // Paint pass - draw everything
                    move |bounds, _, window, _cx| {
                        let origin = bounds.origin;
                        let origin_x: f32 = origin.x.into();
                        let origin_y: f32 = origin.y.into();

                        // Draw background
                        window.paint_quad(PaintQuad {
                            bounds: Bounds::new(origin, bounds.size),
                            background: rgb(bg_color).into(),
                            border_widths: Default::default(),
                            border_color: Default::default(),
                            border_style: Default::default(),
                            corner_radii: Default::default(),
                        });

                        // Draw all content - it will be clipped by the bounds automatically
                        Self::paint_content(
                            window,
                            origin_x,
                            origin_y,
                            &camera,
                            &nodes_data,
                            &positions,
                            &selected_id_clone,
                            &hovered_id_clone,
                            &network_links,
                        );
                    },
                )
                .size_full(),
            )
            // Mouse event handlers
            // IMPORTANT: event.position is relative to this element (the canvas div)
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(move |this, event: &MouseDownEvent, _window, cx| {
                    let local_x: f32 = event.position.x.into();
                    let local_y: f32 = event.position.y.into();
                    
                    if let Some(zone) = this.zones.get_mut(zone_idx) {
                        zone.start_drag(event.position);
                        
                        // Test hit directly with canvas-local coordinates
                        if let Some(node_id) = zone.hit_test(local_x, local_y) {
                            cx.emit(super::AssetSelectedEvent::NodeSelected(node_id));
                            cx.notify();
                        }
                    }
                }),
            )
            .on_mouse_up(
                MouseButton::Left,
                cx.listener(move |this, _, _, _| {
                    if let Some(zone) = this.zones.get_mut(zone_idx) {
                        zone.end_drag();
                    }
                }),
            )
            .on_mouse_move(cx.listener(move |this, event: &MouseMoveEvent, _window, cx| {
                if let Some(zone) = this.zones.get_mut(zone_idx) {
                    // Handle dragging
                    if zone.is_dragging() {
                        zone.update_drag(event.position);
                        cx.notify();
                        return;
                    }

                    // Handle hover with canvas-local coordinates
                    let local_x: f32 = event.position.x.into();
                    let local_y: f32 = event.position.y.into();
                    
                    let new_hovered = zone.hit_test(local_x, local_y);
                    if this.hovered_node_id != new_hovered {
                        this.hovered_node_id = new_hovered;
                        cx.notify();
                    }
                }
            }))
            .on_scroll_wheel(cx.listener(move |this, event: &ScrollWheelEvent, _, cx| {
                if let Some(zone) = this.zones.get_mut(zone_idx) {
                    zone.handle_scroll(event);
                    cx.notify();
                }
            }))
            .into_any_element()
    }

    /// Paint all canvas content - Obsidian-style knowledge graph
    /// 1. C-segment hubs (large translucent circles)
    /// 2. Asset-to-hub connections (thin lines)
    /// 3. Cross-segment connections (curved lines)
    /// 4. Asset nodes (small dots)
    fn paint_content(
        window: &mut Window,
        origin_x: f32,
        origin_y: f32,
        camera: &Camera,
        nodes_data: &[(String, String, String, data::models::Severity, Vec<data::models::Connection>, String)],
        positions: &[NodeVirtualPos],
        selected_id: &Option<String>,
        hovered_id: &Option<String>,
        network_links: &[NetworkLinkInfo],
    ) {
        // Safety check
        if nodes_data.len() != positions.len() {
            tracing::error!("Mismatch: nodes_data({}) != positions({})", nodes_data.len(), positions.len());
            return;
        }

        // Limit render count to prevent buffer overflow
        const MAX_NODES: usize = 500;
        let render_count = nodes_data.len().min(MAX_NODES);
        let scale = camera.scale;
        
        // Build C-segment hub info
        let mut c_segment_hubs: std::collections::HashMap<u8, (f32, f32, usize)> = std::collections::HashMap::new();
        for idx in 0..render_count {
            if let Some(pos) = positions.get(idx) {
                let ip = &nodes_data[idx].5;
                let c_seg = Self::extract_c_segment(ip);
                if c_seg > 0 {
                    let entry = c_segment_hubs.entry(c_seg).or_insert((0.0, 0.0, 0));
                    entry.0 += pos.x;
                    entry.1 += pos.y;
                    entry.2 += 1;
                }
            }
        }
        
        // Calculate hub centers
        let hub_centers: std::collections::HashMap<u8, (f32, f32)> = c_segment_hubs
            .iter()
            .map(|(&c_seg, (sum_x, sum_y, count))| {
                (c_seg, (sum_x / *count as f32, sum_y / *count as f32))
            })
            .collect();

        // 1. Draw network topology links
        if !network_links.is_empty() {
            tracing::info!("Painting {} links for zone", network_links.len());
            Self::paint_network_topology_links(
                window, origin_x, origin_y, camera, nodes_data, positions, render_count, selected_id, network_links, scale,
            );
        }
        
        // 4. Draw asset nodes
        Self::paint_nodes(
            window, origin_x, origin_y, camera, nodes_data, positions, selected_id, hovered_id, render_count,
        );
    }
    
    /// Draw actual network topology links between assets - VISIBLE lines
    fn paint_network_topology_links(
        window: &mut Window,
        origin_x: f32,
        origin_y: f32,
        camera: &Camera,
        nodes_data: &[(String, String, String, data::models::Severity, Vec<data::models::Connection>, String)],
        positions: &[NodeVirtualPos],
        render_count: usize,
        selected_id: &Option<String>,
        network_links: &[NetworkLinkInfo],
        scale: f32,
    ) {
        // Build node ID to position map
        let mut node_positions: std::collections::HashMap<&str, (f32, f32)> = std::collections::HashMap::new();
        for idx in 0..render_count {
            if let Some(pos) = positions.get(idx) {
                node_positions.insert(&nodes_data[idx].0, (pos.x, pos.y));
            }
        }
        
        // Limit total lines
        const MAX_LINKS: usize = 500;
        let mut link_count = 0;
        let mut skipped_count = 0;
        
        // Draw asset-to-asset links
        for link in network_links {
            if link_count >= MAX_LINKS {
                break;
            }
            
            // Look up source and target positions
            let source_pos = node_positions.get(link.source_id.as_str());
            let target_pos = node_positions.get(link.target_id.as_str());
            
            if let (Some(&(sx, sy)), Some(&(tx, ty))) = (source_pos, target_pos) {
                let (ssx, ssy) = camera.virtual_to_screen(sx, sy);
                let (tsx, tsy) = camera.virtual_to_screen(tx, ty);
                
                let start_x = origin_x + ssx;
                let start_y = origin_y + ssy;
                let end_x = origin_x + tsx;
                let end_y = origin_y + tsy;
                
                // Skip very short links (same node)
                let dist = ((end_x - start_x).powi(2) + (end_y - start_y).powi(2)).sqrt();
                if dist < 5.0 {
                    skipped_count += 1;
                    continue;
                }
                
                // Check if involves selected node
                let involves_selected = selected_id.as_ref().map(|s| {
                    s == &link.source_id || s == &link.target_id
                }).unwrap_or(false);
                
                // HIGHLY VISIBLE lines
                let (color, width) = if involves_selected {
                    // Selected: bright orange, thick
                    (gpui::Rgba { r: 1.0, g: 0.3, b: 0.0, a: 1.0 }, px(4.0 * scale))
                } else {
                    // Default: strong color
                    let line_color = if link.action == "service" {
                        // Service connections: blue
                        gpui::Rgba { r: 0.3, g: 0.5, b: 0.9, a: 0.85 }
                    } else {
                        // Network connections: dark gray
                        gpui::Rgba { r: 0.3, g: 0.3, b: 0.3, a: 0.8 }
                    };
                    (line_color, px(2.5 * scale))
                };
                
                // Draw line
                let mut pb = PathBuilder::stroke(width);
                pb.move_to(Point::new(px(start_x), px(start_y)));
                pb.line_to(Point::new(px(end_x), px(end_y)));
                if let Ok(path) = pb.build() {
                    window.paint_path(path, color);
                    link_count += 1;
                }
            } else {
                skipped_count += 1;
            }
        }
        
        if link_count > 0 {
            tracing::debug!("Painted {} links, skipped {} (scale={:.2})", link_count, skipped_count, scale);
        }
    }

    fn paint_nodes(
        window: &mut Window,
        origin_x: f32,
        origin_y: f32,
        camera: &Camera,
        nodes_data: &[(String, String, String, data::models::Severity, Vec<data::models::Connection>, String)],
        positions: &[NodeVirtualPos],
        selected_id: &Option<String>,
        hovered_id: &Option<String>,
        render_count: usize,
    ) {
        let scale = camera.scale;
        let node_count = render_count;
        
        // Smaller nodes for cleaner look
        let (base_r, select_r, hover_r) = if node_count > 100 {
            (2.5 * scale, 5.0 * scale, 4.0 * scale)  // Compact for large datasets
        } else if node_count > 30 {
            (3.5 * scale, 6.0 * scale, 5.0 * scale) // Medium
        } else {
            (5.0 * scale, 8.0 * scale, 6.5 * scale) // Large for small datasets
        };

        for idx in 0..render_count {
            let (node_id, node_name, asset_type, severity, _, _) = &nodes_data[idx];
            
            if let Some(pos) = positions.get(idx) {
                // Convert virtual position to screen position
                let (screen_x, screen_y) = camera.virtual_to_screen(pos.x, pos.y);
                let final_x = origin_x + screen_x;
                let final_y = origin_y + screen_y;

                let is_selected = selected_id.as_ref() == Some(node_id);
                let is_hovered = hovered_id.as_ref() == Some(node_id);

                let node_color = rgb(node_color(asset_type));
                let severity_color = rgb(severity_color(severity));

                let screen_pos = Point::new(px(final_x), px(final_y));

                // Hover ring (drawn behind)
                if is_hovered {
                    let mut pb = PathBuilder::fill();
                    Self::add_circle(&mut pb, screen_pos, hover_r);
                    if let Ok(path) = pb.build() {
                        window.paint_path(path, severity_color);
                    }
                }

                // Selection ring
                if is_selected {
                    let mut pb = PathBuilder::stroke(px(2.0 * scale));
                    Self::add_circle(&mut pb, screen_pos, select_r);
                    if let Ok(path) = pb.build() {
                        window.paint_path(path, rgb(SELECTION_RING));
                    }
                }

                // Severity indicator ring
                let mut pb = PathBuilder::fill();
                Self::add_circle(&mut pb, screen_pos, base_r + 1.0);
                if let Ok(path) = pb.build() {
                    window.paint_path(path, severity_color);
                }

                // Node body - simple dot
                let mut pb = PathBuilder::fill();
                Self::add_circle(&mut pb, screen_pos, base_r);
                if let Ok(path) = pb.build() {
                    window.paint_path(path, node_color);
                }

                // Draw label for selected or hovered nodes
                if is_selected || is_hovered {
                    Self::paint_node_label(window, final_x, final_y + base_r + 10.0, node_name, scale);
                }
            }
        }
    }

    /// Draw network segment labels at cluster centers - minimal clean style
    fn paint_segment_labels(
        window: &mut Window,
        origin_x: f32,
        origin_y: f32,
        camera: &Camera,
        nodes_data: &[(String, String, String, data::models::Severity, Vec<data::models::Connection>, String)],
        positions: &[NodeVirtualPos],
        selected_id: &Option<String>,
    ) {
        let scale = camera.scale;
        
        // Only show labels when zoomed in enough
        if scale < 0.8 {
            return;
        }

        // Group nodes by network segment
        let mut segment_info: std::collections::HashMap<String, (f32, f32, usize)> = std::collections::HashMap::new();
        
        for (idx, (_, _, _, _, _, ip_address)) in nodes_data.iter().enumerate() {
            if let Some(pos) = positions.get(idx) {
                let network = Self::extract_network_segment(ip_address);
                if !network.is_empty() {
                    let entry = segment_info.entry(network).or_insert((0.0, 0.0, 0));
                    entry.0 += pos.x;
                    entry.1 += pos.y;
                    entry.2 += 1;
                }
            }
        }

        // Draw label for each segment
        for (network, (sum_x, sum_y, count)) in segment_info {
            let avg_x = sum_x / count as f32;
            let avg_y = sum_y / count as f32;
            
            let (screen_x, screen_y) = camera.virtual_to_screen(avg_x, avg_y);
            let final_x = origin_x + screen_x;
            let final_y = origin_y + screen_y - 35.0 * scale;

            // Format: "1.x" - showing only C-segment
            let label = if let Some(c_seg) = network.split('.').nth(2) {
                format!("{}.x", c_seg)
            } else {
                network.chars().take(8).collect::<String>()
            };
            
            let font_size = if count > 30 { 7.0 } else { 8.0 };
            let alpha = if scale > 1.2 { 0.9 } else { 0.6 };
            
            // Draw small dot as label indicator
            let mut pb = PathBuilder::fill();
            pb.move_to(Point::new(px(final_x), px(final_y - 3.0)));
            Self::add_circle(&mut pb, Point::new(px(final_x), px(final_y)), 3.0 * scale);
            if let Ok(path) = pb.build() {
                window.paint_path(path, gpui::Rgba { r: 0.7, g: 0.7, b: 0.7, a: alpha });
            }
            
            // Draw text indicator
            let text_width = label.len() as f32 * font_size * 0.5 * scale;
            let line_y = final_y + 5.0 * scale;
            let mut pb = PathBuilder::stroke(px(scale * 0.5));
            pb.move_to(Point::new(px(final_x - text_width / 2.0), px(line_y)));
            pb.line_to(Point::new(px(final_x + text_width / 2.0), px(line_y)));
            if let Ok(path) = pb.build() {
                window.paint_path(path, gpui::Rgba { r: 0.8, g: 0.8, b: 0.8, a: alpha });
            }
        }
    }

    /// Draw a simple text label below a node
    fn paint_node_label(
        window: &mut Window,
        x: f32,
        y: f32,
        text: &str,
        scale: f32,
    ) {
        // Truncate long names (use char count for UTF-8 safety)
        let label = if text.chars().count() > 12 {
            format!("{}...", text.chars().take(10).collect::<String>())
        } else {
            text.to_string()
        };
        
        // Draw text background for readability
        let text_width = label.len() as f32 * 6.0 * scale;
        let bg_bounds = gpui::Bounds::new(
            Point::new(px(x - text_width / 2.0 - 4.0), px(y - 8.0 * scale)),
            gpui::Size::new(px(text_width + 8.0), px(16.0 * scale)),
        );
        
        window.paint_quad(PaintQuad {
            bounds: bg_bounds,
            background: gpui::Rgba { r: 1.0, g: 1.0, b: 1.0, a: 0.9 }.into(),
            border_widths: Default::default(),
            border_color: Default::default(),
            border_style: Default::default(),
            corner_radii: Default::default(),
        });
        
        // Note: Text rendering in canvas requires more setup
        // For now, we draw a placeholder line indicating label presence
        let mut pb = PathBuilder::stroke(px(1.0));
        pb.move_to(Point::new(px(x - text_width / 2.0), px(y)));
        pb.line_to(Point::new(px(x + text_width / 2.0), px(y)));
        if let Ok(path) = pb.build() {
            window.paint_path(path, gpui::Rgba { r: 0.3, g: 0.3, b: 0.3, a: 0.8 });
        }
    }

    /// Paint network topology links - clean curved lines between related segments
    /// Only shows actual network connectivity, no decorative elements
    fn paint_obsidian_links(
        window: &mut Window,
        origin_x: f32,
        origin_y: f32,
        camera: &Camera,
        nodes_data: &[(String, String, String, data::models::Severity, Vec<data::models::Connection>, String)],
        positions: &[NodeVirtualPos],
        selected_id: &Option<String>,
        network_links: &[NetworkLinkInfo],
    ) {
        // Calculate center position for each network segment
        let mut segment_centers: std::collections::HashMap<String, (f32, f32)> = std::collections::HashMap::new();
        let mut segment_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
        
        for (idx, (_, _, _, _, _, ip_address)) in nodes_data.iter().enumerate() {
            if let Some(pos) = positions.get(idx) {
                let network = Self::extract_network_segment(ip_address);
                if !network.is_empty() {
                    let entry = segment_centers.entry(network.clone()).or_insert((0.0, 0.0));
                    entry.0 += pos.x;
                    entry.1 += pos.y;
                    *segment_counts.entry(network).or_insert(0) += 1;
                }
            }
        }
        
        // Average to get center
        for (net, (sum_x, sum_y)) in &mut segment_centers {
            if let Some(&count) = segment_counts.get(net) {
                if count > 0 {
                    *sum_x /= count as f32;
                    *sum_y /= count as f32;
                }
            }
        }

        // Draw links between segments
        for link in network_links {
            let source_center = segment_centers.get(&link.source_id);
            let target_center = segment_centers.get(&link.target_id);
            
            // Only draw if both segments are visible in this zone
            let (sx, sy) = match source_center {
                Some(c) => *c,
                None => continue,
            };
            let (tx, ty) = match target_center {
                Some(c) => *c,
                None => continue,
            };
            
            // Convert to screen coordinates
            let (start_sx, start_sy) = camera.virtual_to_screen(sx, sy);
            let (end_sx, end_sy) = camera.virtual_to_screen(tx, ty);
            
            let start_x = origin_x + start_sx;
            let start_y = origin_y + start_sy;
            let end_x = origin_x + end_sx;
            let end_y = origin_y + end_sy;
            
            // Skip very short links (same cluster)
            let dist = ((end_x - start_x).powi(2) + (end_y - start_y).powi(2)).sqrt();
            if dist < 30.0 {
                continue;
            }

            // Determine visual style
            let involves_selected = selected_id.as_ref().map(|s| {
                nodes_data.iter().any(|(id, _, _, _, _, ip)| {
                    id == s && (
                        Self::extract_network_segment(ip) == link.source_id ||
                        Self::extract_network_segment(ip) == link.target_id
                    )
                })
            }).unwrap_or(false);

            let (color, width, alpha) = if involves_selected {
                (gpui::Rgba { r: 1.0, g: 0.5, b: 0.0, a: 1.0 }, px(3.0), 1.0)
            } else if selected_id.is_some() {
                (gpui::Rgba { r: 0.5, g: 0.5, b: 0.5, a: 0.15 }, px(1.0), 0.15)
            } else {
                // Subtle gray lines for topology
                (gpui::Rgba { r: 0.6, g: 0.6, b: 0.6, a: 0.4 }, px(1.5), 0.4)
            };
            
            // Skip very faint lines
            if alpha < 0.2 {
                continue;
            }

            // Draw clean curved line
            Self::paint_topology_link(window, start_x, start_y, end_x, end_y, width, color);
        }
    }
    
    /// Draw a clean curved topology link using line segments
    fn paint_topology_link(
        window: &mut Window,
        start_x: f32,
        start_y: f32,
        end_x: f32,
        end_y: f32,
        width: Pixels,
        color: gpui::Rgba,
    ) {
        // Calculate control point for curve
        let mid_x = (start_x + end_x) / 2.0;
        let mid_y = (start_y + end_y) / 2.0;
        
        // Add slight curve based on distance
        let dist = ((end_x - start_x).powi(2) + (end_y - start_y).powi(2)).sqrt();
        let curve_offset = (dist * 0.15).min(40.0);
        
        // Perpendicular direction
        let dx = end_x - start_x;
        let dy = end_y - start_y;
        let len = (dx * dx + dy * dy).sqrt();
        if len < 0.001 {
            return;
        }
        let perp_x = -dy / len;
        let perp_y = dx / len;
        
        let cp_x = mid_x + perp_x * curve_offset;
        let cp_y = mid_y + perp_y * curve_offset;
        
        // Draw curve using multiple line segments (quadratic bezier approximation)
        let mut pb = PathBuilder::stroke(width);
        pb.move_to(Point::new(px(start_x), px(start_y)));
        
        // Draw 8 segments for smooth curve
        for i in 1..=8 {
            let t = i as f32 / 8.0;
            // Quadratic bezier formula: (1-t)^2 * P0 + 2(1-t)t * P1 + t^2 * P2
            let mt = 1.0 - t;
            let x = mt * mt * start_x + 2.0 * mt * t * cp_x + t * t * end_x;
            let y = mt * mt * start_y + 2.0 * mt * t * cp_y + t * t * end_y;
            pb.line_to(Point::new(px(x), px(y)));
        }
        
        if let Ok(path) = pb.build() {
            window.paint_path(path, color);
        }
    }

    /// Draw a small arrow at zone edge to indicate cross-zone connection
    fn paint_edge_arrow(
        window: &mut Window,
        from_x: f32,
        from_y: f32,
        to_x: f32,
        to_y: f32,
        color: gpui::Rgba,
    ) {
        let angle = (to_y - from_y).atan2(to_x - from_x);
        let arrow_size = 5.0;
        
        let angle1 = angle + std::f32::consts::PI * 0.8;
        let angle2 = angle - std::f32::consts::PI * 0.8;
        
        let x1 = to_x + arrow_size * angle1.cos();
        let y1 = to_y + arrow_size * angle1.sin();
        let x2 = to_x + arrow_size * angle2.cos();
        let y2 = to_y + arrow_size * angle2.sin();
        
        let mut pb = PathBuilder::fill();
        pb.move_to(Point::new(px(to_x), px(to_y)));
        pb.line_to(Point::new(px(x1), px(y1)));
        pb.line_to(Point::new(px(x2), px(y2)));
        pb.close();
        
        if let Ok(path) = pb.build() {
            window.paint_path(path, color);
        }
    }

    /// Draw a straight line link
    fn paint_bezier_link(
        window: &mut Window,
        start_x: f32,
        start_y: f32,
        end_x: f32,
        end_y: f32,
        width: gpui::Pixels,
        color: gpui::Rgba,
        is_bidirectional: bool,
    ) {
        // Draw straight line
        let mut pb = PathBuilder::stroke(width);
        pb.move_to(Point::new(px(start_x), px(start_y)));
        pb.line_to(Point::new(px(end_x), px(end_y)));
        
        if let Ok(path) = pb.build() {
            window.paint_path(path, color);
        }

        // Draw small indicator at midpoint for bidirectional links only
        if is_bidirectional {
            let mid_x = (start_x + end_x) / 2.0;
            let mid_y = (start_y + end_y) / 2.0;
            
            // Small dot at midpoint for bidirectional indicator
            let dot_size = 2.0;
            let mut pb = PathBuilder::fill();
            Self::add_circle_to_path(&mut pb, px(mid_x), px(mid_y), dot_size);
            if let Ok(path) = pb.build() {
                window.paint_path(path, color);
            }
        }
    }

    /// Draw a dashed line to indicate external/zone-crossing connection
    fn paint_dashed_link(
        window: &mut Window,
        start_x: f32,
        start_y: f32,
        end_x: f32,
        end_y: f32,
        color: gpui::Rgba,
    ) {
        let dx = end_x - start_x;
        let dy = end_y - start_y;
        let dist = (dx * dx + dy * dy).sqrt();
        
        if dist < 1.0 {
            return;
        }
        
        let dash_len = 5.0;
        let gap_len = 3.0;
        let segment_len = dash_len + gap_len;
        let num_segments = (dist / segment_len) as i32;
        
        let dir_x = dx / dist;
        let dir_y = dy / dist;
        
        for i in 0..num_segments {
            let seg_start = i as f32 * segment_len;
            let seg_end = seg_start + dash_len;
            
            if seg_end > dist {
                break;
            }
            
            let x1 = start_x + dir_x * seg_start;
            let y1 = start_y + dir_y * seg_start;
            let x2 = start_x + dir_x * seg_end;
            let y2 = start_y + dir_y * seg_end;
            
            let mut pb = PathBuilder::stroke(px(1.5));
            pb.move_to(Point::new(px(x1), px(y1)));
            pb.line_to(Point::new(px(x2), px(y2)));
            
            if let Ok(path) = pb.build() {
                window.paint_path(path, color);
            }
        }
    }

    /// Helper to add circle to path (static version)
    fn add_circle_to_path(builder: &mut PathBuilder, cx: gpui::Pixels, cy: gpui::Pixels, radius: f32) {
        let segments = 12;
        let cx_f: f32 = cx.into();
        let cy_f: f32 = cy.into();
        
        for i in 0..=segments {
            let angle = (i as f32 / segments as f32) * std::f32::consts::TAU;
            let x = cx_f + radius * angle.cos();
            let y = cy_f + radius * angle.sin();
            
            if i == 0 {
                builder.move_to(Point::new(px(x), px(y)));
            } else {
                builder.line_to(Point::new(px(x), px(y)));
            }
        }
        builder.close();
    }

    /// 绘制方向箭头
    fn paint_arrow(
        window: &mut Window,
        start_x: f32,
        start_y: f32,
        end_x: f32,
        end_y: f32,
        color: gpui::Rgba, // 修复：使用Rgba而不是Hsla
    ) {
        // 计算箭头位置（在线段的中点）
        let mid_x = (start_x + end_x) / 2.0;
        let mid_y = (start_y + end_y) / 2.0;
        
        // 计算角度
        let angle = (end_y - start_y).atan2(end_x - start_x);
        let arrow_size = 6.0;
        
        // 箭头的两个点
        let arrow_angle1 = angle + std::f32::consts::PI * 0.75; // 135度
        let arrow_angle2 = angle - std::f32::consts::PI * 0.75; // -135度
        
        let x1 = mid_x + arrow_size * arrow_angle1.cos();
        let y1 = mid_y + arrow_size * arrow_angle1.sin();
        let x2 = mid_x + arrow_size * arrow_angle2.cos();
        let y2 = mid_y + arrow_size * arrow_angle2.sin();
        
        // 绘制箭头（三角形）
        let mut pb = PathBuilder::fill();
        pb.move_to(Point::new(px(mid_x), px(mid_y)));
        pb.line_to(Point::new(px(x1), px(y1)));
        pb.line_to(Point::new(px(x2), px(y2)));
        pb.close();
        
        if let Ok(path) = pb.build() {
            window.paint_path(path, color);
        }
    }

    fn add_circle(builder: &mut PathBuilder, center: Point<Pixels>, radius: f32) {
        let segments = if radius > 5.0 { 16 } else { 12 };
        let cx: f32 = center.x.into();
        let cy: f32 = center.y.into();

        for i in 0..=segments {
            let angle = (i as f32 / segments as f32) * std::f32::consts::TAU;
            let x = cx + radius * angle.cos();
            let y = cy + radius * angle.sin();

            if i == 0 {
                builder.move_to(Point::new(px(x), px(y)));
            } else {
                builder.line_to(Point::new(px(x), px(y)));
            }
        }
        builder.close();
    }
}
