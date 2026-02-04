//! Resource management for topology visualization
//! 
//! This module handles loading and embedding static assets (HTML, CSS, JS)
//! for the D3.js topology visualization.

use serde_json::json;

/// Zone configuration: (id, name, bg_color, primary_color, min_pct, max_pct)
const ZONES: [(&str, &str, &str, &str, f64, f64); 5] = [
    ("Z1", "地面指挥中心", "#e8f4fd", "#2563eb", 0.0, 0.2),
    ("Z2", "通信网关层", "#d1fae5", "#10b981", 0.2, 0.4),
    ("Z3", "任务控制层", "#ede9fe", "#7c3aed", 0.4, 0.6),
    ("Z4", "飞控设备层", "#ffedd5", "#f97316", 0.6, 0.8),
    ("Z5", "安全应急系统", "#fee2e2", "#ef4444", 0.8, 1.0),
];

/// HTML template with placeholders
const HTML_TEMPLATE: &str = include_str!("../resources/topology.html");

/// CSS styles
const CSS_CONTENT: &str = include_str!("../resources/topology.css");

/// JavaScript for D3.js visualization
const JS_CONTENT: &str = include_str!("../resources/topology.js");

/// Maximum number of links for performance
const MAX_LINKS: usize = 500;

/// Topology resource builder - generates HTML with embedded data
pub struct TopologyResourceBuilder;

impl TopologyResourceBuilder {
    /// Build the complete HTML with embedded data
    pub fn build_html(assets: &[data::models::Asset]) -> String {
        let total_assets = assets.len();
        let node_scale_factor = Self::calculate_node_scale_factor(total_assets);
        
        let nodes = Self::build_nodes(assets);
        let links = Self::build_links(assets);
        let zones_json = serde_json::to_string(&ZONES).unwrap();
        
        HTML_TEMPLATE
            .replace("/* {{CSS_CONTENT}} */", CSS_CONTENT)
            .replace("{{NODES_DATA}}", &serde_json::to_string(&nodes).unwrap())
            .replace("{{LINKS_DATA}}", &serde_json::to_string(&links).unwrap())
            .replace("{{ZONES_DATA}}", &zones_json)
            .replace("{{NODE_SCALE_FACTOR}}", &node_scale_factor.to_string())
            .replace("{{TOTAL_ASSETS}}", &total_assets.to_string())
            .replace("// {{JS_CONTENT}}", JS_CONTENT)
    }
    
    /// Calculate scale factor based on asset count for adaptive node sizing
    fn calculate_node_scale_factor(total_assets: usize) -> f64 {
        match total_assets {
            n if n > 200 => 0.4,
            n if n > 100 => 0.6,
            n if n > 50 => 0.8,
            _ => 1.0,
        }
    }
    
    /// Build node data from assets
    fn build_nodes(assets: &[data::models::Asset]) -> Vec<serde_json::Value> {
        assets.iter().map(|asset| {
            let zone = asset.zone_id.as_deref().unwrap_or("Z1");
            let zone_index = ZONES.iter().position(|(z, _, _, _, _, _)| *z == zone).unwrap_or(0);
            
            json!({
                "id": asset.id.to_string(),
                "name": &asset.name,
                "group": zone,
                "zoneIndex": zone_index,
                "zoneColor": ZONES[zone_index].3,
                "color": Self::get_node_color(asset.risk_score, asset.vuln_count),
                "risk": asset.risk_score,
                "vulnCount": asset.vuln_count,
                "ip": asset.ip_address.as_deref().unwrap_or(""),
                "type": asset.asset_type.as_str(),
                "status": format!("{:?}", asset.status),
            })
        }).collect()
    }
    
    /// Build link data from asset network accessibility relationships
    fn build_links(assets: &[data::models::Asset]) -> Vec<serde_json::Value> {
        let mut links = Vec::new();
        let mut link_set = std::collections::HashSet::new();
        
        for source in assets {
            if source.accessible_networks.is_empty() {
                continue;
            }
            
            for target in assets {
                if source.id == target.id {
                    continue;
                }
                
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
                        
                        if links.len() >= MAX_LINKS {
                            tracing::info!("Link limit reached ({MAX_LINKS})");
                            return links;
                        }
                    }
                }
            }
        }
        
        links
    }
    
    /// Check if a network segment matches any of the accessible networks
    fn network_matches_accessible(network: &str, accessible_list: &[String]) -> bool {
        for accessible in accessible_list {
            // Direct match
            if network == accessible {
                return true;
            }
            
            // Check if network base IP is in accessible CIDR
            let network_base = network.split('/').next().unwrap_or(network);
            if Self::ip_in_cidr(network_base, accessible) {
                return true;
            }
        }
        false
    }
    
    /// Check if an IP is in a CIDR range
    fn ip_in_cidr(ip: &str, cidr: &str) -> bool {
        if cidr == "0.0.0.0/0" || cidr == "any" {
            return true;
        }
        
        let parts: Vec<&str> = cidr.split('/').collect();
        if parts.len() != 2 {
            return ip == cidr;
        }
        
        let network_ip = parts[0];
        let mask_bits: u32 = match parts[1].parse() {
            Ok(n) => n,
            Err(_) => return ip == cidr,
        };
        
        let ip_parts: Vec<u8> = ip.split('.').filter_map(|s| s.parse().ok()).collect();
        let net_parts: Vec<u8> = network_ip.split('.').filter_map(|s| s.parse().ok()).collect();
        
        if ip_parts.len() != 4 || net_parts.len() != 4 {
            return false;
        }
        
        let mask = if mask_bits == 0 { 0u32 } else { !((1u32 << (32 - mask_bits)) - 1) };
        
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
    
    /// Get color based on risk/vulnerability level
    /// Returns hex color code for node fill
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
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_calculate_node_scale_factor() {
        assert_eq!(TopologyResourceBuilder::calculate_node_scale_factor(10), 1.0);
        assert_eq!(TopologyResourceBuilder::calculate_node_scale_factor(75), 0.8);
        assert_eq!(TopologyResourceBuilder::calculate_node_scale_factor(150), 0.6);
        assert_eq!(TopologyResourceBuilder::calculate_node_scale_factor(250), 0.4);
    }
    
    #[test]
    fn test_get_node_color() {
        assert_eq!(TopologyResourceBuilder::get_node_color(10, 0), "#10b981");
        assert_eq!(TopologyResourceBuilder::get_node_color(25, 0), "#3b82f6");
        assert_eq!(TopologyResourceBuilder::get_node_color(50, 0), "#f59e0b");
        assert_eq!(TopologyResourceBuilder::get_node_color(75, 0), "#ef4444");
        assert_eq!(TopologyResourceBuilder::get_node_color(10, 6), "#ef4444"); // vuln_count > 5
    }
    
    #[test]
    fn test_ip_in_cidr() {
        assert!(TopologyResourceBuilder::ip_in_cidr("192.168.1.1", "192.168.1.0/24"));
        assert!(TopologyResourceBuilder::ip_in_cidr("10.0.0.5", "10.0.0.0/8"));
        assert!(!TopologyResourceBuilder::ip_in_cidr("192.168.2.1", "192.168.1.0/24"));
        assert!(TopologyResourceBuilder::ip_in_cidr("1.2.3.4", "0.0.0.0/0"));
    }
}
