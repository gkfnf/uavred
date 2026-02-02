//! Topology Canvas - Interactive network asset topology visualization
//!
//! ## Architecture
//!
//! The canvas is organized into 5 security zones (Z1-Z5), each with:
//! - Independent viewport (pan/zoom)
//! - Mouse wheel zoom
//! - Click+drag pan
//! - Node rendering via GPUI canvas API
//! - Connection lines between nodes (Obsidian-style bidirectional links)

mod camera;
mod zone_canvas;

use gpui::*;
use gpui::prelude::FluentBuilder;
use gpui_component::{h_flex, label::Label, v_flex, Icon, IconName};

use data::models::{Asset, AssetNode, ZoneType};
use data::{AssetStore, init_and_load_asset_store};
use ui::theme::*;

use crate::config::ZoneTypeExt;

pub use zone_canvas::{NetworkLinkInfo, NodeVirtualPos, ZoneCanvas};

impl EventEmitter<AssetSelectedEvent> for TopologyCanvas {}

/// Event emitted when an asset node is selected
#[derive(Clone, Debug)]
pub enum AssetSelectedEvent {
    NodeSelected(String),
}

/// Main topology canvas component
pub struct TopologyCanvas {
    zones: Vec<ZoneCanvas>,
    selected_node_id: Option<String>,
    hovered_node_id: Option<String>,
    /// Global network links across all zones (Obsidian-style graph)
    global_network_links: Vec<NetworkLinkInfo>,
}

impl TopologyCanvas {
    /// Create new TopologyCanvas using real database
    /// 
    /// Note: Call init_and_load_asset_store(cx) before creating this component
    pub fn new(cx: &mut Context<Self>) -> Self {
        tracing::info!("TopologyCanvas::new() called");
        
        // Initialize asset store if not already done
        init_and_load_asset_store(cx);
        
        // Load assets from the global store
        let asset_store = AssetStore::global(cx);
        let assets = asset_store.read(cx).get_all_assets();
        
        tracing::info!("TopologyCanvas loaded {} assets from database", assets.len());
        
        // Log assets by zone
        for zone in [ZoneType::Z1, ZoneType::Z2, ZoneType::Z3, ZoneType::Z4, ZoneType::Z5] {
            let zone_assets: Vec<_> = assets.iter().filter(|a| a.zone_id.as_deref() == Some(zone.as_str())).collect();
            tracing::info!("Zone {:?}: {} assets", zone, zone_assets.len());
            for asset in &zone_assets {
                tracing::info!("  - {} (id={}) at {:?}", asset.name, asset.id, asset.ip_address);
            }
        }
        
        let zone_virtual_width = 400.0;
        let zone_virtual_height = 600.0;

        let zones = vec![
            ZoneCanvas::new_with_assets(ZoneType::Z1, &assets, zone_virtual_width, zone_virtual_height),
            ZoneCanvas::new_with_assets(ZoneType::Z2, &assets, zone_virtual_width, zone_virtual_height),
            ZoneCanvas::new_with_assets(ZoneType::Z3, &assets, zone_virtual_width, zone_virtual_height),
            ZoneCanvas::new_with_assets(ZoneType::Z4, &assets, zone_virtual_width, zone_virtual_height),
            ZoneCanvas::new_with_assets(ZoneType::Z5, &assets, zone_virtual_width, zone_virtual_height),
        ];
        
        // Log zone canvas info with node positions
        for (idx, zone) in zones.iter().enumerate() {
            tracing::info!("ZoneCanvas[{}] ({:?}): {} nodes", idx, zone.zone(), zone.node_count());
            for (i, node) in zone.nodes().iter().enumerate() {
                if let Some(pos) = zone.node_positions().get(i) {
                    tracing::info!("  Node[{}]: id={} name={} pos=({:.1}, {:.1})", 
                        i, node.id, node.name, pos.x, pos.y);
                }
            }
        }

        // Calculate global network links for Obsidian-style graph
        let global_links = Self::calculate_global_network_links(&assets);
        tracing::info!("TopologyCanvas: calculated {} global network links", global_links.len());
        
        // Log first few links for debugging
        for (i, link) in global_links.iter().take(5).enumerate() {
            tracing::info!("  Link[{}]: {} -> {} ({})", i, link.source_id, link.target_id, link.direction);
        }
        
        Self {
            zones,
            selected_node_id: None,
            hovered_node_id: None,
            global_network_links: global_links,
        }
    }

    /// Calculate global network links based on:
    /// 1. Network reachability (accessible_networks)
    /// 2. Service-based connections (same service type = likely communication)
    fn calculate_global_network_links(assets: &[Asset]) -> Vec<NetworkLinkInfo> {
        let mut links = Vec::new();
        let mut link_set = std::collections::HashSet::new();
        
        // Build network to assets map
        let mut network_to_assets: std::collections::HashMap<String, Vec<i64>> = std::collections::HashMap::new();
        for asset in assets {
            if !asset.network_segment.is_empty() {
                network_to_assets.entry(asset.network_segment.clone())
                    .or_default()
                    .push(asset.id);
            }
        }
        
        // Build service-based connections
        // Group assets by service type
        let mut service_to_assets: std::collections::HashMap<String, Vec<i64>> = std::collections::HashMap::new();
        for asset in assets {
            for service in &asset.services {
                if !service.service_name.is_empty() {
                    service_to_assets.entry(service.service_name.clone())
                        .or_default()
                        .push(asset.id);
                }
            }
        }
        
        // 1. Create links from network reachability
        for source in assets {
            for accessible_net in &source.accessible_networks {
                if accessible_net == &source.network_segment {
                    continue;
                }
                
                if let Some(target_assets) = network_to_assets.get(accessible_net) {
                    for &target_id in target_assets {
                        if source.id == target_id {
                            continue;
                        }
                        
                        let link_key = format!("net-{}-{}", source.id.min(target_id), source.id.max(target_id));
                        
                        if !link_set.contains(&link_key) {
                            link_set.insert(link_key);
                            
                            links.push(NetworkLinkInfo {
                                source_id: source.id.to_string(),
                                target_id: target_id.to_string(),
                                action: "network".to_string(),
                                direction: "bidirectional".to_string(),
                                protocol: "IP".to_string(),
                                port_range: "-".to_string(),
                            });
                            
                            if links.len() >= 300 {
                                tracing::info!("Reached link limit from network reachability");
                                return links;
                            }
                        }
                    }
                }
            }
        }
        
        // 2. Create links from service relationships
        // Assets with same service type likely communicate with each other
        for (service_name, asset_ids) in &service_to_assets {
            if asset_ids.len() < 2 || asset_ids.len() > 20 {
                continue; // Skip if too few or too many assets with this service
            }
            
            // Create connections between all assets with the same service
            for i in 0..asset_ids.len() {
                for j in (i + 1)..asset_ids.len() {
                    let id1 = asset_ids[i];
                    let id2 = asset_ids[j];
                    
                    let link_key = format!("svc-{}-{}-{}", service_name, id1.min(id2), id1.max(id2));
                    
                    if !link_set.contains(&link_key) {
                        link_set.insert(link_key);
                        
                        links.push(NetworkLinkInfo {
                            source_id: id1.to_string(),
                            target_id: id2.to_string(),
                            action: "service".to_string(),
                            direction: "bidirectional".to_string(),
                            protocol: service_name.clone(),
                            port_range: "-".to_string(),
                        });
                        
                        if links.len() >= 500 {
                            tracing::info!("Reached total link limit (500)");
                            return links;
                        }
                    }
                }
            }
        }

        tracing::info!("Calculated {} network links ({} from services)", 
            links.len(), link_set.iter().filter(|k| k.starts_with("svc-")).count());
        links
    }

    /// Check if a network segment is in the accessible list
    fn network_in_list(network: &str, accessible_list: &[String]) -> bool {
        if accessible_list.iter().any(|n| n == "0.0.0.0/0" || n == "any") {
            return true;
        }
        
        let normalized = network.trim_end_matches("/32");
        
        accessible_list.iter().any(|accessible| {
            let acc_norm = accessible.trim_end_matches("/32");
            acc_norm == normalized || Self::cidr_contains(accessible, normalized)
        })
    }

    /// Check if a CIDR contains an IP or smaller network
    fn cidr_contains(cidr: &str, ip_or_net: &str) -> bool {
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

    /// Extract ports from services
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

    /// Create new TopologyCanvas with explicit asset list (for testing)
    pub fn with_assets(assets: Vec<data::models::Asset>) -> Self {
        let zone_virtual_width = 400.0;
        let zone_virtual_height = 600.0;

        let zones = vec![
            ZoneCanvas::new_with_assets(ZoneType::Z1, &assets, zone_virtual_width, zone_virtual_height),
            ZoneCanvas::new_with_assets(ZoneType::Z2, &assets, zone_virtual_width, zone_virtual_height),
            ZoneCanvas::new_with_assets(ZoneType::Z3, &assets, zone_virtual_width, zone_virtual_height),
            ZoneCanvas::new_with_assets(ZoneType::Z4, &assets, zone_virtual_width, zone_virtual_height),
            ZoneCanvas::new_with_assets(ZoneType::Z5, &assets, zone_virtual_width, zone_virtual_height),
        ];

        let global_links = Self::calculate_global_network_links(&assets);

        Self {
            zones,
            selected_node_id: None,
            hovered_node_id: None,
            global_network_links: global_links,
        }
    }

    /// Get all nodes across all zones
    pub fn get_nodes(&self) -> Vec<&AssetNode> {
        self.zones.iter().flat_map(|z| z.nodes()).collect()
    }

    /// Get total node count
    pub fn node_count(&self) -> usize {
        self.zones.iter().map(|z| z.node_count()).sum()
    }

    /// Get total connection count
    pub fn connection_count(&self) -> usize {
        self.zones.iter().map(|z| z.connection_count()).sum()
    }

    /// Reset all zones to fit their content
    pub fn reset_all_views(&mut self) {
        for zone in &mut self.zones {
            zone.reset_view();
        }
    }

    /// Render zone header bar
    fn render_zone_headers(&self) -> impl IntoElement {
        h_flex()
            .w_full()
            .h(px(55.0))
            .border_b_1()
            .border_color(rgb(BORDER_COLOR))
            .bg(rgb(0xffffff))
            .children(self.zones.iter().enumerate().map(|(idx, zone)| {
                self.render_zone_header(zone, idx)
            }).collect::<Vec<_>>())
    }

    /// Render individual zone header
    fn render_zone_header(&self, zone: &ZoneCanvas, idx: usize) -> AnyElement {
        let config = zone.zone().config();
        let count = zone.node_count();
        let is_last = idx == 4;

        h_flex()
            .flex_1()
            .h_full()
            .when(!is_last, |this: gpui::Div| this.border_r_1().border_color(rgb(BORDER_COLOR)))
            .px_3()
            .py_2()
            .gap_2()
            .child(
                Icon::new(IconName::CircleCheck)
                    .size(px(16.0))
                    .text_color(rgb(config.primary_color))
            )
            .child(
                v_flex()
                    .gap_1()
                    .child(
                        h_flex()
                            .gap_2()
                            .child(
                                Label::new(config.short_name)
                                    .text_sm()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(config.primary_color))
                            )
                            .child(
                                Label::new(config.layer_name)
                                    .text_xs()
                                    .text_color(rgb(TEXT_PRIMARY))
                            )
                    )
                    .child(
                        Label::new(format!("{} 资产", count))
                            .text_xs()
                            .text_color(rgb(TEXT_SECONDARY))
                    )
            )
            .child(div().flex_1())
            .child(
                div()
                    .text_lg()
                    .text_color(rgb(config.primary_color))
                    .cursor_pointer()
                    .child("+")
            )
            .into_any_element()
    }

    /// Render zone canvases
    fn render_zone_canvases(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let selected_id = self.selected_node_id.clone();
        let hovered_id = self.hovered_node_id.clone();
        let global_links = self.global_network_links.clone();

        h_flex()
            .flex_1()
            .w_full()
            .children(self.zones.iter_mut().enumerate().map(move |(zone_idx, zone)| {
                zone.render_canvas(zone_idx, &selected_id, &hovered_id, &global_links, cx)
            }).collect::<Vec<_>>())
    }
}

impl Render for TopologyCanvas {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .child(self.render_zone_headers())
            .child(self.render_zone_canvases(cx))
    }
}
