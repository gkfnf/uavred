use gpui::*;
use std::collections::HashMap;

use gpui_component::ElementExt;
use ui::theme::*;

use data::models::{AssetNode, ConnectionInfo};

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
    connections: Vec<ConnectionInfo>,
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

    fn create_sample_connections(nodes: &[AssetNode]) -> Vec<ConnectionInfo> {
        vec![
            ConnectionInfo {
                from_id: nodes[0].id.clone(),
                to_id: nodes[1].id.clone(),
                protocol: "MAVLink".to_string(),
                port: Some(5760),
                status: data::models::ConnectionStatus::Active,
            },
            ConnectionInfo {
                from_id: nodes[1].id.clone(),
                to_id: nodes[2].id.clone(),
                protocol: "TCP".to_string(),
                port: Some(443),
                status: data::models::ConnectionStatus::Active,
            },
            ConnectionInfo {
                from_id: nodes[2].id.clone(),
                to_id: nodes[3].id.clone(),
                protocol: "HTTPS".to_string(),
                port: Some(443),
                status: data::models::ConnectionStatus::Active,
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

        for (zone_idx, (zone_key, zone_node_list)) in zone_nodes.iter().enumerate() {
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
        window: &mut Window,
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
                        let dx_px = local_pos.x - px(pos.x * self.scale + self.offset_x);
                        let dy_px = local_pos.y - px(pos.y * self.scale + self.offset_y);

                        let dx = dx_px - start_pos.x;
                        let dy = dy_px - start_pos.y;

                        let distance = (dx * dx + dy * dy).sqrt();

                        let node_size = Self::get_node_size(&node.asset_type) * self.scale;

                        if distance <= (node_size / 2.0).0 {
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
        event: &MouseMoveEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some((ref node_id, ref start_pos)) = self.drag_state {
            if let Some(bounds) = self.canvas_bounds {
                let local_pos = Point::new(
                    event.position.x - bounds.origin.x,
                    event.position.y - bounds.origin.y,
                );

                let dx = (local_pos.x.0 - start_pos.x.0);
                let dy = (local_pos.y.0 - start_pos.y.0);

                if let Some(pos) = self.node_positions.get_mut(node_id) {
                    pos.x += dx / self.scale;
                    pos.y += dy / self.scale;
                }

                self.drag_state = Some((node_id.clone(), local_pos));
                cx.notify();
            }
        }
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

impl Render for TopologyCanvas {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let nodes_for_paint = self.nodes.clone();
        let connections_for_paint = self.connections.clone();
        let node_positions_for_paint = self.node_positions.clone();
        let selected_node_id_for_paint = self.selected_node_id.clone();
        let scale_for_paint = self.scale;
        let offset_x_for_paint = self.offset_x;
        let offset_y_for_paint = self.offset_y;

        let state_entity = cx.entity().clone();

        div()
            .size_full()
            .bg(rgb(BG_PRIMARY))
            .relative()
            .on_mouse_down(MouseButton::Left, cx.listener(Self::handle_mouse_down))
            .on_mouse_move(cx.listener(Self::handle_mouse_move))
            .on_mouse_up(MouseButton::Left, cx.listener(Self::handle_mouse_up))
            .on_prepaint(move |bounds, _window, cx| {
                state_entity.update(cx, |state, _| {
                    state.canvas_bounds = Some(bounds);
                })
            })
            .child(
                canvas(
                    move |bounds, _window, _cx| {
                        (
                            nodes_for_paint,
                            connections_for_paint,
                            node_positions_for_paint,
                            selected_node_id_for_paint,
                            scale_for_paint,
                            offset_x_for_paint,
                            offset_y_for_paint,
                            bounds,
                        )
                    },
                    move |_bounds,
                          (
                        nodes,
                        connections,
                        node_positions,
                        selected_node_id,
                        scale,
                        offset_x,
                        offset_y,
                        prepaint_bounds,
                    ),
                          window,
                          _cx| {
                        let origin = prepaint_bounds.origin;

                        for connection in &connections {
                            if let (Some(from_pos), Some(to_pos)) = (
                                node_positions.get(&connection.from_id),
                                node_positions.get(&connection.to_id),
                            ) {
                                let start = Point::new(
                                    origin.x + px(from_pos.x * scale + offset_x),
                                    origin.y + px(from_pos.y * scale + offset_y),
                                );
                                let end = Point::new(
                                    origin.x + px(to_pos.x * scale + offset_x),
                                    origin.y + px(to_pos.y * scale + offset_y),
                                );

                                let mut builder = PathBuilder::stroke(px(2.0));
                                builder.move_to(start);
                                builder.line_to(end);

                                let line_color = match connection.status {
                                    data::models::ConnectionStatus::Active => rgb(ACCENT_BLUE),
                                    data::models::ConnectionStatus::Inactive => rgb(TEXT_MUTED),
                                    data::models::ConnectionStatus::Warning => rgb(STATUS_WARNING),
                                    data::models::ConnectionStatus::Error => rgb(STATUS_ERROR),
                                };

                                if let Ok(path) = builder.build() {
                                    window.paint_path(path, line_color);
                                }
                            }
                        }

                        for node in &nodes {
                            if let Some(pos) = node_positions.get(&node.id) {
                                let center = Point::new(
                                    origin.x + px(pos.x * scale + offset_x),
                                    origin.y + px(pos.y * scale + offset_y),
                                );

                                let node_color = TopologyCanvas::get_node_color(&node.asset_type);
                                let node_size =
                                    TopologyCanvas::get_node_size(&node.asset_type) * scale;
                                let radius = px(node_size / 2.0);

                                let mut builder = PathBuilder::fill();
                                builder.move_to(center);
                                for i in 0..=60 {
                                    let angle = i as f32 * std::f32::consts::PI * 2.0 / 60.0;
                                    let point = Point::new(
                                        center.x + radius * angle.cos(),
                                        center.y + radius * angle.sin(),
                                    );
                                    builder.line_to(point);
                                }
                                builder.line_to(center);

                                if let Ok(path) = builder.build() {
                                    window.paint_path(path, node_color);
                                }

                                if selected_node_id.as_ref() == Some(&node.id) {
                                    let border_radius = radius + px(4.0);
                                    let mut border_builder = PathBuilder::stroke(px(3.0));
                                    for i in 0..=60 {
                                        let angle = i as f32 * std::f32::consts::PI * 2.0 / 60.0;
                                        let point = Point::new(
                                            center.x + border_radius * angle.cos(),
                                            center.y + border_radius * angle.sin(),
                                        );
                                        if i == 0 {
                                            border_builder.move_to(point);
                                        } else {
                                            border_builder.line_to(point);
                                        }
                                    }
                                    border_builder
                                        .line_to(Point::new(center.x + border_radius, center.y));

                                    if let Ok(border_path) = border_builder.build() {
                                        window.paint_path(border_path, rgb(BORDER_FOCUSED));
                                    }
                                }
                            }
                        }
                    },
                )
                .absolute()
                .size_full(),
            )
    }
}
