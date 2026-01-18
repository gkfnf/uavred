use gpui::*;
use std::collections::HashMap;

use gpui_component::{ElementExt, v_flex, h_flex};
use ui::theme::*;

use data::models::{AssetNode, Connection};
use crate::components::{TopologyZone, render_topology_zone};

impl EventEmitter<AssetSelectedEvent> for TopologyCanvas {}

#[derive(Clone, Debug)]
pub struct NodePosition {
    pub x: f32,
    pub y: f32,
}

pub enum AssetSelectedEvent {
    NodeSelected(String),
}

pub struct TopologyCanvas {
    nodes: Vec<AssetNode>,
    connections: Vec<Connection>,
    node_positions: HashMap<String, NodePosition>,
    selected_node_id: Option<String>,
    scale: f32,
    offset_x: f32,
    offset_y: f32,
    drag_state: Option<(String, Point<Pixels>)>,
    canvas_bounds: Option<Bounds<Pixels>>,
}

impl TopologyCanvas {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        let nodes = Self::create_sample_nodes();
        let connections = Self::create_sample_connections(&nodes);
        let node_positions = Self::calculate_node_positions(&nodes);

        Self {
            nodes,
            connections,
            node_positions,
            selected_node_id: None,
            scale: 1.0,
            offset_x: 0.0,
            offset_y: 0.0,
            drag_state: None,
            canvas_bounds: None,
        }
    }

    fn create_sample_nodes() -> Vec<AssetNode> {
        vec![
            AssetNode {
                id: "uav-1".to_string(),
                name: "DJI Mavic 3 Pro".to_string(),
                ip_address: "192.168.1.100".to_string(),
                mac_address: Some("00:11:22:33:44:55".to_string()),
                zone: data::models::ZoneType::Z4,
                severity: data::models::Severity::Medium,
                risk_score: 45,
                vulnerabilities_count: 2,
                services: vec![],
                open_ports: vec![],
                credentials: vec![],
                owner: "Team Alpha".to_string(),
                business_purpose: "Surveillance".to_string(),
                department: Some("Security".to_string()),
                scan_progress: data::models::ScanProgress {
                    percentage: 100,
                    last_scan: Some("2024-01-10T10:30:00Z".to_string()),
                    next_scan: Some("2024-01-11T10:30:00Z".to_string()),
                    scan_type: "Full".to_string(),
                    scanning: false,
                },
                compliance_standards: vec![],
                connections: vec![],
                status: data::models::AssetStatus::Online,
                last_seen: "2024-01-13T22:00:00Z".to_string(),
                asset_type: "UAV".to_string(),
                firmware_version: Some("v1.5.0".to_string()),
                manufacturer: Some("DJI".to_string()),
                location: Some("Sector 7".to_string()),
            },
            AssetNode {
                id: "gcs-1".to_string(),
                name: "Ground Control Station".to_string(),
                ip_address: "192.168.1.1".to_string(),
                mac_address: Some("AA:BB:CC:DD:EE:FF".to_string()),
                zone: data::models::ZoneType::Z3,
                severity: data::models::Severity::Info,
                risk_score: 10,
                vulnerabilities_count: 0,
                services: vec![],
                open_ports: vec![],
                credentials: vec![],
                owner: "Team Alpha".to_string(),
                business_purpose: "Flight Control".to_string(),
                department: Some("Operations".to_string()),
                scan_progress: data::models::ScanProgress {
                    percentage: 100,
                    last_scan: Some("2024-01-10T10:30:00Z".to_string()),
                    next_scan: Some("2024-01-11T10:30:00Z".to_string()),
                    scan_type: "Full".to_string(),
                    scanning: false,
                },
                compliance_standards: vec![],
                connections: vec![],
                status: data::models::AssetStatus::Online,
                last_seen: "2024-01-13T22:00:00Z".to_string(),
                asset_type: "GCS".to_string(),
                firmware_version: Some("v2.3.1".to_string()),
                manufacturer: Some("Custom".to_string()),
                location: Some("Control Room".to_string()),
            },
            AssetNode {
                id: "router-1".to_string(),
                name: "Network Router".to_string(),
                ip_address: "192.168.1.254".to_string(),
                mac_address: Some("11:22:33:44:55:66".to_string()),
                zone: data::models::ZoneType::Z2,
                severity: data::models::Severity::Low,
                risk_score: 5,
                vulnerabilities_count: 0,
                services: vec![],
                open_ports: vec![],
                credentials: vec![],
                owner: "IT Department".to_string(),
                business_purpose: "Network Gateway".to_string(),
                department: Some("IT".to_string()),
                scan_progress: data::models::ScanProgress {
                    percentage: 100,
                    last_scan: Some("2024-01-10T10:30:00Z".to_string()),
                    next_scan: Some("2024-01-11T10:30:00Z".to_string()),
                    scan_type: "Full".to_string(),
                    scanning: false,
                },
                compliance_standards: vec![],
                connections: vec![],
                status: data::models::AssetStatus::Online,
                last_seen: "2024-01-13T22:00:00Z".to_string(),
                asset_type: "Router".to_string(),
                firmware_version: Some("v3.1.2".to_string()),
                manufacturer: Some("Cisco".to_string()),
                location: Some("Server Room".to_string()),
            },
            AssetNode {
                id: "server-1".to_string(),
                name: "Data Server".to_string(),
                ip_address: "192.168.2.100".to_string(),
                mac_address: Some("CC:DD:EE:FF:00:11".to_string()),
                zone: data::models::ZoneType::Z1,
                severity: data::models::Severity::High,
                risk_score: 75,
                vulnerabilities_count: 5,
                services: vec![],
                open_ports: vec![],
                credentials: vec![],
                owner: "IT Department".to_string(),
                business_purpose: "Data Storage".to_string(),
                department: Some("IT".to_string()),
                scan_progress: data::models::ScanProgress {
                    percentage: 80,
                    last_scan: Some("2024-01-10T10:30:00Z".to_string()),
                    next_scan: Some("2024-01-11T10:30:00Z".to_string()),
                    scan_type: "Full".to_string(),
                    scanning: true,
                },
                compliance_standards: vec![],
                connections: vec![],
                status: data::models::AssetStatus::Online,
                last_seen: "2024-01-13T22:00:00Z".to_string(),
                asset_type: "Server".to_string(),
                firmware_version: Some("Ubuntu 22.04".to_string()),
                manufacturer: Some("Dell".to_string()),
                location: Some("Data Center".to_string()),
            },
        ]
    }

    fn create_sample_connections(nodes: &[AssetNode]) -> Vec<Connection> {
        vec![
            Connection {
                target_id: nodes[1].id.clone(),
                connection_type: "MAVLink".to_string(),
                protocol: "MAVLink".to_string(),
                port: 5760,
            },
            Connection {
                target_id: nodes[2].id.clone(),
                connection_type: "TCP".to_string(),
                protocol: "TCP".to_string(),
                port: 443,
            },
            Connection {
                target_id: nodes[3].id.clone(),
                connection_type: "HTTPS".to_string(),
                protocol: "HTTPS".to_string(),
                port: 443,
            },
        ]
    }

    fn calculate_node_positions(nodes: &[AssetNode]) -> HashMap<String, NodePosition> {
        let mut positions = HashMap::new();
        let canvas_width = 800.0;
        let canvas_height = 600.0;
        let padding = 100.0;

        let num_zones = 5;
        let zone_width = (canvas_width - 2.0 * padding) / num_zones as f32;

        let mut zone_nodes: HashMap<&str, Vec<&AssetNode>> = HashMap::new();

        for node in nodes {
            let zone_key = match node.zone {
                data::models::ZoneType::Z1 => "Z1",
                data::models::ZoneType::Z2 => "Z2",
                data::models::ZoneType::Z3 => "Z3",
                data::models::ZoneType::Z4 => "Z4",
                data::models::ZoneType::Z5 => "Z5",
            };
            zone_nodes
                .entry(zone_key)
                .or_insert_with(Vec::new)
                .push(node);
        }

        for (zone_idx, (_zone_key, zone_node_list)) in zone_nodes.iter().enumerate() {
            let zone_x = padding + zone_idx as f32 * zone_width;
            let num_nodes = zone_node_list.len();
            let vertical_spacing = (canvas_height - 2.0 * padding) / (num_nodes.max(1) + 1) as f32;

            for (node_idx, node) in zone_node_list.iter().enumerate() {
                positions.insert(
                    node.id.clone(),
                    NodePosition {
                        x: zone_x + zone_width / 2.0,
                        y: padding + (node_idx + 1) as f32 * vertical_spacing,
                    },
                );
            }
        }

        positions
    }

    fn get_node_color(asset_type: &str) -> Rgba {
        match asset_type {
            "UAV" => rgb(0x2563eb),
            "GCS" => rgb(0x7c3aed),
            "Router" => rgb(0x10b981),
            "Server" => rgb(0xf97316),
            _ => rgb(0x6b7280),
        }
    }

    fn get_node_size(asset_type: &str) -> f32 {
        match asset_type {
            "UAV" => 24.0,
            "GCS" => 28.0,
            "Router" => 26.0,
            "Server" => 30.0,
            _ => 22.0,
        }
    }

    fn handle_mouse_down(
        &mut self,
        event: &MouseDownEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if event.button == MouseButton::Left {
            if let Some(bounds) = self.canvas_bounds {
                let local_pos = Point::new(
                    event.position.x - bounds.origin.x,
                    event.position.y - bounds.origin.y,
                );

                for node in &self.nodes {
                    if let Some(pos) = self.node_positions.get(&node.id) {
                        let node_size = Self::get_node_size(&node.asset_type) * self.scale;
                        let hit_radius = px(node_size / 2.0);
                        let node_screen_x = px(pos.x * self.scale + self.offset_x);
                        let node_screen_y = px(pos.y * self.scale + self.offset_y);
                        
                        let hit_rect = Bounds::new(
                            Point::new(node_screen_x - hit_radius, node_screen_y - hit_radius),
                            Size::new(hit_radius * 2.0, hit_radius * 2.0),
                        );
                        
                        if hit_rect.contains(&local_pos) {
                            self.selected_node_id = Some(node.id.clone());
                            self.drag_state = Some((node.id.clone(), local_pos));
                            cx.emit(AssetSelectedEvent::NodeSelected(node.id.clone()));
                            cx.notify();
                            return;
                        }
                    }
                }

                self.selected_node_id = None;
                cx.notify();
            }
        }
    }

    fn handle_mouse_move(
        &mut self,
        _event: &MouseMoveEvent,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
        // Drag implementation will be added later
        // Currently just clearing drag state
        self.drag_state = None;
    }

    fn handle_mouse_up(
        &mut self,
        _event: &MouseUpEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.drag_state = None;
        cx.notify();
    }
}

impl TopologyCanvas {
    fn group_nodes_by_zone(&self) -> Vec<(data::models::ZoneType, Vec<AssetNode>)> {
        use std::collections::BTreeMap;
        use data::models::ZoneType;
        
        let mut zones: BTreeMap<String, Vec<AssetNode>> = BTreeMap::new();
        for node in &self.nodes {
            let zone_key = format!("{:?}", node.zone);
            zones.entry(zone_key)
                .or_insert_with(Vec::new)
                .push(node.clone());
        }
        
        zones.into_iter().map(|(_, assets)| {
            let zone = if !assets.is_empty() {
                assets[0].zone.clone()
            } else {
                data::models::ZoneType::Z1
            };
            (zone, assets)
        }).collect()
    }
}

impl Render for TopologyCanvas {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let zones = self.group_nodes_by_zone();
        
        v_flex()
            .flex_none()
            .h(px(400.0))
            .gap_2()
            .p_4()
            .bg(rgb(BG_CARD))
            .child(
                h_flex()
                    .flex_1()
                    .gap_2()
                    .children(
                        zones.iter().map(|(zone, assets)| {
                            let topology_zone = TopologyZone::new(zone.clone(), assets.clone());
                            render_topology_zone(&topology_zone).into_any_element()
                        })
                    )
            )
    }
}
