use gpui::*;
use std::collections::HashMap;

use gpui_component::h_flex;
use ui::theme::*;

use crate::components::{render_asset_node_at, render_topology_zone_bg, TopologyZone};
use data::models::{AssetNode, Connection};

impl EventEmitter<AssetSelectedEvent> for TopologyCanvas {}

/// Events emitted by TopologyCanvas when user interacts with nodes
#[derive(Clone, Debug)]
pub enum AssetSelectedEvent {
    /// User clicked on an asset node
    NodeSelected(String),
}

/// Position of a node on the canvas
#[derive(Clone, Debug)]
pub struct NodePosition {
    pub x: f32,
    pub y: f32,
}

/// 分区布局信息
#[derive(Clone)]
pub struct ZoneLayout {
    pub zone: data::models::ZoneType,
    pub name: String,        // 分区名称: "地面指挥中心", "通信网关层" 等
    pub description: String, // 分区描述
    pub icon: gpui_component::IconName,
    pub bg_color: u32,          // 分区背景色
    pub x: f32,                 // 分区在画布中的 x 坐标
    pub y: f32,                 // 分区在画布中的 y 坐标
    pub width: f32,             // 分区宽度
    pub height: f32,            // 分区高度
    pub asset_ids: Vec<String>, // 该分区的资产 ID 列表
}

// /// 连接线样式 (Reserved for future use when implementing status-based colors)
// #[derive(Clone, Debug)]
// pub struct ConnectionStyle {
//     pub color: Rgba,
//     pub is_dashed: bool,
//     pub width: f32,
// }
//
// impl Default for ConnectionStyle {
//     fn default() -> Self {
//         Self {
//             color: rgb(0xb0bec5),  // 灰色
//             is_dashed: true,
//             width: 1.5,
//         }
//     }
// }

/// TopologyCanvas - Network topology visualization canvas
///
/// Renders 5 network zones (Z1-Z5) with asset nodes and connections.
/// Handles user interaction for asset selection.
///
/// Layout:
/// - Zone backgrounds and headers (Layer 1)
/// - Connection lines between nodes (Layer 2)
/// - Asset nodes at absolute positions (Layer 3)
pub struct TopologyCanvas {
    // 数据
    pub nodes: Vec<AssetNode>,

    // 布局数据
    zones_layout: Vec<ZoneLayout>,                 // 5 个分区的布局信息
    node_positions: HashMap<String, NodePosition>, // 节点位置映射

    // 交互状态
    selected_node_id: Option<String>,

    // 画布状态
    canvas_bounds: Option<Bounds<Pixels>>,

    // 显示参数 (kept for future pan/zoom implementation)
    scale: f32,
    offset_x: f32,
    offset_y: f32,
    drag_state: Option<(String, Point<Pixels>)>,
}

impl TopologyCanvas {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        let nodes = Self::create_sample_nodes();
        let zones_layout = Self::create_zones_layout();
        let mut canvas = Self {
            nodes,
            zones_layout,
            node_positions: HashMap::new(),
            selected_node_id: None,
            canvas_bounds: None,
            scale: 1.0,
            offset_x: 0.0,
            offset_y: 0.0,
            drag_state: None,
        };

        // 初始布局计算 (使用默认 canvas 宽度)
        canvas.calculate_layout(800.0, 600.0);
        canvas
    }

    fn create_sample_nodes() -> Vec<AssetNode> {
        vec![
            // Z1 - 地面指挥中心
            AssetNode {
                id: "gcs-primary".to_string(),
                name: "GCS Primary".to_string(),
                ip_address: "10.0.1.10".to_string(),
                mac_address: Some("00:11:22:33:44:55".to_string()),
                zone: data::models::ZoneType::Z1,
                severity: data::models::Severity::Low,
                risk_score: 15,
                vulnerabilities_count: 1,
                services: vec![],
                open_ports: vec![80, 443],
                credentials: vec![],
                owner: "Command Center".to_string(),
                business_purpose: "Ground Control".to_string(),
                department: Some("Operations".to_string()),
                scan_progress: data::models::ScanProgress {
                    percentage: 100,
                    last_scan: Some("2024-01-13T10:30:00Z".to_string()),
                    next_scan: Some("2024-01-14T10:30:00Z".to_string()),
                    scan_type: "Full".to_string(),
                    scanning: false,
                },
                compliance_standards: vec![],
                connections: vec![data::models::Connection {
                    target_id: "telemetry-service".to_string(),
                    connection_type: "Data".to_string(),
                    protocol: "MAVLink".to_string(),
                    port: 5760,
                }],
                status: data::models::AssetStatus::Online,
                last_seen: "2024-01-13T22:00:00Z".to_string(),
                asset_type: "GCS".to_string(),
                firmware_version: Some("v2.5.0".to_string()),
                manufacturer: Some("DefenseTech".to_string()),
                location: Some("Main HQ".to_string()),
            },
            // Z2 - 通信网关层
            AssetNode {
                id: "telemetry-service".to_string(),
                name: "Telemetry Se...".to_string(),
                ip_address: "10.0.1.20".to_string(),
                mac_address: Some("AA:BB:CC:DD:EE:01".to_string()),
                zone: data::models::ZoneType::Z2,
                severity: data::models::Severity::Low,
                risk_score: 10,
                vulnerabilities_count: 0,
                services: vec![],
                open_ports: vec![5760],
                credentials: vec![],
                owner: "Network Team".to_string(),
                business_purpose: "Data Relaying".to_string(),
                department: Some("IT".to_string()),
                scan_progress: data::models::ScanProgress {
                    percentage: 100,
                    last_scan: Some("2024-01-13T10:30:00Z".to_string()),
                    next_scan: Some("2024-01-14T10:30:00Z".to_string()),
                    scan_type: "Full".to_string(),
                    scanning: false,
                },
                compliance_standards: vec![],
                connections: vec![data::models::Connection {
                    target_id: "mission-control-server".to_string(),
                    connection_type: "Data".to_string(),
                    protocol: "TCP".to_string(),
                    port: 8080,
                }],
                status: data::models::AssetStatus::Online,
                last_seen: "2024-01-13T22:00:00Z".to_string(),
                asset_type: "Router".to_string(),
                firmware_version: Some("v1.2.0".to_string()),
                manufacturer: Some("Cisco".to_string()),
                location: Some("Comms Tower".to_string()),
            },
            AssetNode {
                id: "data-gateway".to_string(),
                name: "Data Gateway".to_string(),
                ip_address: "10.0.1.21".to_string(),
                mac_address: Some("AA:BB:CC:DD:EE:02".to_string()),
                zone: data::models::ZoneType::Z2,
                severity: data::models::Severity::Low,
                risk_score: 12,
                vulnerabilities_count: 0,
                services: vec![],
                open_ports: vec![8080],
                credentials: vec![],
                owner: "Network Team".to_string(),
                business_purpose: "Data Ingestion".to_string(),
                department: Some("IT".to_string()),
                scan_progress: data::models::ScanProgress {
                    percentage: 100,
                    last_scan: Some("2024-01-13T10:30:00Z".to_string()),
                    next_scan: Some("2024-01-14T10:30:00Z".to_string()),
                    scan_type: "Full".to_string(),
                    scanning: false,
                },
                compliance_standards: vec![],
                connections: vec![data::models::Connection {
                    target_id: "mission-control-server".to_string(),
                    connection_type: "Data".to_string(),
                    protocol: "TCP".to_string(),
                    port: 8080,
                }],
                status: data::models::AssetStatus::Online,
                last_seen: "2024-01-13T22:00:00Z".to_string(),
                asset_type: "Router".to_string(),
                firmware_version: Some("v1.2.1".to_string()),
                manufacturer: Some("Cisco".to_string()),
                location: Some("Comms Tower".to_string()),
            },
            // Z3 - 任务控制层
            AssetNode {
                id: "mission-control-server".to_string(),
                name: "Mission Control Server".to_string(),
                ip_address: "10.0.1.52".to_string(),
                mac_address: Some("CC:DD:EE:FF:00:11".to_string()),
                zone: data::models::ZoneType::Z3,
                severity: data::models::Severity::Medium,
                risk_score: 55,
                vulnerabilities_count: 2,
                services: vec![],
                open_ports: vec![443, 8080, 9090],
                credentials: vec![],
                owner: "IT Department".to_string(),
                business_purpose: "Mission Planning".to_string(),
                department: Some("IT".to_string()),
                scan_progress: data::models::ScanProgress {
                    percentage: 85,
                    last_scan: Some("2024-01-13T11:00:00Z".to_string()),
                    next_scan: Some("2024-01-14T11:00:00Z".to_string()),
                    scan_type: "Full".to_string(),
                    scanning: false,
                },
                compliance_standards: vec![],
                connections: vec![
                    data::models::Connection {
                        target_id: "dji-mavic-3".to_string(),
                        connection_type: "Control".to_string(),
                        protocol: "DJI".to_string(),
                        port: 0,
                    },
                    data::models::Connection {
                        target_id: "flight-controller".to_string(),
                        connection_type: "Control".to_string(),
                        protocol: "MAVLink".to_string(),
                        port: 0,
                    },
                    data::models::Connection {
                        target_id: "sensor-array".to_string(),
                        connection_type: "Data".to_string(),
                        protocol: "UDP".to_string(),
                        port: 0,
                    },
                ],
                status: data::models::AssetStatus::Online,
                last_seen: "2024-01-13T22:00:00Z".to_string(),
                asset_type: "Server".to_string(),
                firmware_version: Some("Ubuntu 22.04 LTS".to_string()),
                manufacturer: Some("Dell".to_string()),
                location: Some("Data Center".to_string()),
            },
            // Z4 - 飞控设备层
            AssetNode {
                id: "dji-mavic-3".to_string(),
                name: "DJI Mavic 3 ...".to_string(),
                ip_address: "192.168.1.100".to_string(),
                mac_address: Some("00:11:22:33:44:55".to_string()),
                zone: data::models::ZoneType::Z4,
                severity: data::models::Severity::Medium,
                risk_score: 45,
                vulnerabilities_count: 2,
                services: vec![],
                open_ports: vec![],
                credentials: vec![],
                owner: "Flight Team".to_string(),
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
                connections: vec![data::models::Connection {
                    target_id: "emergency-system".to_string(),
                    connection_type: "Alert".to_string(),
                    protocol: "Custom".to_string(),
                    port: 0,
                }],
                status: data::models::AssetStatus::Online,
                last_seen: "2024-01-13T22:00:00Z".to_string(),
                asset_type: "UAV".to_string(),
                firmware_version: Some("v1.5.0".to_string()),
                manufacturer: Some("DJI".to_string()),
                location: Some("Sector 7".to_string()),
            },
            AssetNode {
                id: "flight-controller".to_string(),
                name: "Flight Contr...".to_string(),
                ip_address: "192.168.1.101".to_string(),
                mac_address: Some("00:11:22:33:44:56".to_string()),
                zone: data::models::ZoneType::Z4,
                severity: data::models::Severity::Low,
                risk_score: 20,
                vulnerabilities_count: 1,
                services: vec![],
                open_ports: vec![],
                credentials: vec![],
                owner: "Flight Team".to_string(),
                business_purpose: "Autopilot".to_string(),
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
                manufacturer: Some("Pixhawk".to_string()),
                location: Some("Sector 7".to_string()),
            },
            AssetNode {
                id: "sensor-array".to_string(),
                name: "Sensor Array".to_string(),
                ip_address: "192.168.1.102".to_string(),
                mac_address: Some("00:11:22:33:44:57".to_string()),
                zone: data::models::ZoneType::Z4,
                severity: data::models::Severity::Low,
                risk_score: 15,
                vulnerabilities_count: 0,
                services: vec![],
                open_ports: vec![],
                credentials: vec![],
                owner: "Flight Team".to_string(),
                business_purpose: "Telemetry".to_string(),
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
                manufacturer: Some("Generic".to_string()),
                location: Some("Sector 7".to_string()),
            },
            // Z5 - 安全应急系统
            AssetNode {
                id: "emergency-system".to_string(),
                name: "Emergency Sy...".to_string(),
                ip_address: "10.0.5.1".to_string(),
                mac_address: Some("EE:EE:EE:EE:EE:EE".to_string()),
                zone: data::models::ZoneType::Z5,
                severity: data::models::Severity::Low,
                risk_score: 5,
                vulnerabilities_count: 0,
                services: vec![],
                open_ports: vec![],
                credentials: vec![],
                owner: "Safety Team".to_string(),
                business_purpose: "Safety".to_string(),
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
                asset_type: "Server".to_string(),
                firmware_version: Some("v4.0.0".to_string()),
                manufacturer: Some("SafeGuard".to_string()),
                location: Some("Safety Bunker".to_string()),
            },
        ]
    }

    #[allow(dead_code)]
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

    fn create_zones_layout() -> Vec<ZoneLayout> {
        vec![
            ZoneLayout {
                zone: data::models::ZoneType::Z1,
                name: "Z1".to_string(),
                description: "地面指挥中心".to_string(),
                icon: gpui_component::IconName::CircleCheck,
                bg_color: 0xf0f7ff, // 浅蓝色
                x: 0.0,
                y: 0.0,
                width: 0.0,
                height: 0.0,
                asset_ids: Vec::new(),
            },
            ZoneLayout {
                zone: data::models::ZoneType::Z2,
                name: "Z2".to_string(),
                description: "通信网关层".to_string(),
                icon: gpui_component::IconName::CircleCheck,
                bg_color: 0xf0fff4, // 浅绿色
                x: 0.0,
                y: 0.0,
                width: 0.0,
                height: 0.0,
                asset_ids: Vec::new(),
            },
            ZoneLayout {
                zone: data::models::ZoneType::Z3,
                name: "Z3".to_string(),
                description: "任务控制层".to_string(),
                icon: gpui_component::IconName::CircleCheck,
                bg_color: 0xf5f3ff, // 浅紫色
                x: 0.0,
                y: 0.0,
                width: 0.0,
                height: 0.0,
                asset_ids: Vec::new(),
            },
            ZoneLayout {
                zone: data::models::ZoneType::Z4,
                name: "Z4".to_string(),
                description: "飞控设备层".to_string(),
                icon: gpui_component::IconName::CircleCheck,
                bg_color: 0xfffaf0, // 浅橙色
                x: 0.0,
                y: 0.0,
                width: 0.0,
                height: 0.0,
                asset_ids: Vec::new(),
            },
            ZoneLayout {
                zone: data::models::ZoneType::Z5,
                name: "Z5".to_string(),
                description: "安全紧急系统".to_string(),
                icon: gpui_component::IconName::CircleCheck,
                bg_color: 0xfff5f5, // 浅红色
                x: 0.0,
                y: 0.0,
                width: 0.0,
                height: 0.0,
                asset_ids: Vec::new(),
            },
        ]
    }

    /// 计算所有分区和节点的布局位置
    fn calculate_layout(&mut self, canvas_width: f32, canvas_height: f32) {
        if canvas_width <= 0.0 || canvas_height <= 0.0 {
            return;
        }

        // 1. 清空旧位置数据
        self.node_positions.clear();
        for zone in &mut self.zones_layout {
            zone.asset_ids.clear();
        }

        // 2. 分配每个节点到对应的分区
        for node in &self.nodes {
            for zone in &mut self.zones_layout {
                if zone.zone == node.zone {
                    zone.asset_ids.push(node.id.clone());
                    break;
                }
            }
        }

        // 3. 计算分区的位置和大小
        let zone_count = 5;
        let zone_width = canvas_width / zone_count as f32;
        let zone_height = canvas_height;
        let header_height = 80.0; // 分区头部高度

        for (idx, zone) in self.zones_layout.iter_mut().enumerate() {
            zone.x = idx as f32 * zone_width;
            zone.y = 0.0;
            zone.width = zone_width;
            zone.height = zone_height;
        }

        // 4. 计算每个节点在其分区内的位置
        for zone in &self.zones_layout {
            let asset_count = zone.asset_ids.len();
            let inner_width = zone.width - 40.0; // 分区左右 padding
            let inner_height = zone.height - header_height - 40.0; // 分区上下 padding

            for (node_idx, node_id) in zone.asset_ids.iter().enumerate() {
                let node_pos = self.calculate_node_position_in_zone(
                    zone.x,
                    zone.y + header_height,
                    inner_width,
                    inner_height,
                    node_idx,
                    asset_count,
                );
                self.node_positions.insert(node_id.clone(), node_pos);
            }
        }
    }

    /// 计算节点在分区内的位置
    fn calculate_node_position_in_zone(
        &self,
        zone_x: f32,
        zone_y: f32,
        zone_width: f32,
        zone_height: f32,
        node_idx: usize,
        total_nodes: usize,
    ) -> NodePosition {
        let x = zone_x + zone_width / 2.0;
        let content_height = zone_height - 100.0;
        let start_y = zone_y + 50.0;

        match total_nodes {
            1 => {
                // 居中
                NodePosition {
                    x,
                    y: start_y + content_height / 2.0,
                }
            }
            2 => {
                // 一个偏上，一个偏下
                let y = if node_idx == 0 {
                    start_y + content_height * 0.3
                } else {
                    start_y + content_height * 0.7
                };
                NodePosition { x, y }
            }
            3 => {
                // V 字形布局
                let y = match node_idx {
                    0 => start_y + content_height * 0.2, // 最上
                    1 => start_y + content_height * 0.5, // 中间
                    2 => start_y + content_height * 0.8, // 最下
                    _ => start_y + content_height / 2.0,
                };
                NodePosition { x, y }
            }
            _ => {
                // 网格布局
                let rows = (total_nodes as f32).sqrt().ceil() as usize;
                let row = node_idx / rows;
                let row_height = content_height / rows as f32;
                NodePosition {
                    x,
                    y: start_y + row as f32 * row_height + row_height / 2.0,
                }
            }
        }
    }

    #[allow(dead_code)]
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

    #[allow(dead_code)]
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

    #[allow(dead_code)]
    fn handle_mouse_move(
        &mut self,
        _event: &MouseMoveEvent,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
        // TODO: Phase 2 - Implement drag-drop
        // Required: Calculate delta between start_pos and current pos
        // Update node_positions HashMap
        // Notify for re-render
        self.drag_state = None;
    }

    #[allow(dead_code)]
    fn handle_mouse_up(
        &mut self,
        _event: &MouseUpEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // TODO: Phase 2 - Finalize drop position
        self.drag_state = None;
        cx.notify();
    }
}

impl Render for TopologyCanvas {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // 初次渲染时计算布局
        if self.node_positions.is_empty() && !self.nodes.is_empty() {
            self.calculate_layout(1200.0, 600.0);
        }

        let zones = self.zones_layout.clone();
        let nodes = self.nodes.clone();

        div()
            .relative()
            .flex_1()
            .bg(rgb(BG_CARD))
            .size_full()
            .overflow_hidden()
            .child(
                // Layer 1: Zone backgrounds and headers
                h_flex()
                    .size_full()
                    .gap_0()
                    .children(zones.iter().map(|zone_layout| {
                        let zone_assets: Vec<AssetNode> = self
                            .nodes
                            .iter()
                            .filter(|n| n.zone == zone_layout.zone)
                            .cloned()
                            .collect();

                        let zone = TopologyZone::new(
                            zone_layout.zone.clone(),
                            zone_assets,
                            zone_layout.name.clone(),
                            zone_layout.description.clone(),
                            zone_layout.bg_color,
                            zone_layout.icon.clone(),
                        );

                        render_topology_zone_bg(&zone).into_any_element()
                    })),
            )
            .child(
                // Layer 2: Connections
                {
                    let node_positions = self.node_positions.clone();
                    let nodes = self.nodes.clone();

                    canvas(
                        move |_bounds, _window, _cx| {
                            // Prepaint: collect connection paths
                            let mut paths = Vec::new();
                            for node in &nodes {
                                if let Some(start_pos) = node_positions.get(&node.id) {
                                    for conn in &node.connections {
                                        if let Some(end_pos) = node_positions.get(&conn.target_id) {
                                            let start = Point::new(
                                                px(start_pos.x + 24.0),
                                                px(start_pos.y + 24.0),
                                            );
                                            let end = Point::new(
                                                px(end_pos.x + 24.0),
                                                px(end_pos.y + 24.0),
                                            );
                                            paths.push((start, end));
                                        }
                                    }
                                }
                            }
                            paths
                        },
                        move |bounds, paths, window, _cx| {
                            // Paint: draw connection lines
                            for (start, end) in paths {
                                let start_abs = Point::new(
                                    bounds.origin.x + start.x,
                                    bounds.origin.y + start.y,
                                );
                                let end_abs =
                                    Point::new(bounds.origin.x + end.x, bounds.origin.y + end.y);

                                let mut path_builder = PathBuilder::stroke(px(1.5));
                                path_builder = path_builder.dash_array(&[px(4.0), px(2.0)]);
                                path_builder.move_to(start_abs);
                                path_builder.line_to(end_abs);

                                if let Ok(path) = path_builder.build() {
                                    window.paint_path(path, rgb(0xcbd5e1));
                                }
                            }
                        },
                    )
                    .absolute()
                    .size_full()
                },
            )
            .child(
                // Layer 3: Asset nodes at absolute positions
                div()
                    .absolute()
                    .size_full()
                    .children(nodes.iter().filter_map(|node| {
                        let pos = self.node_positions.get(&node.id)?;
                        Some(render_asset_node_at(node, pos))
                    })),
            )
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, event, window, cx| {
                    this.handle_mouse_down(event, window, cx);
                }),
            )
    }
}
