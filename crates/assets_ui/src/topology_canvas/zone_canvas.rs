//! Zone Canvas - Individual zone viewport with nodes and interactions

use gpui::*;
use gpui::prelude::FluentBuilder;

use data::models::{AssetNode, ZoneType};
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
        };

        // Auto-fit to show all nodes initially
        canvas.fit_to_view();

        canvas
    }

    /// Get zone type
    pub fn zone(&self) -> ZoneType {
        self.zone
    }

    /// Get nodes reference
    pub fn nodes(&self) -> &[AssetNode] {
        &self.nodes
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

        let bounds = self.calculate_node_bounds();
        self.camera.fit_to_bounds(&bounds, 1.1); // 10% padding
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

    /// Hit test for node selection
    pub fn hit_test(&self, relative_x: f32, relative_y: f32) -> Option<String> {
        // Don't select while dragging
        if self.is_dragging {
            return None;
        }

        let hit_radius = 15.0;
        let (virtual_x, virtual_y) = self.camera.screen_to_virtual(relative_x, relative_y);

        let mut closest_node: Option<(String, f32)> = None;

        for (idx, node) in self.nodes.iter().enumerate() {
            if let Some(pos) = self.node_positions.get(idx) {
                let dx = pos.x - virtual_x;
                let dy = pos.y - virtual_y;
                let dist_sq = dx * dx + dy * dy;
                let screen_dist_sq = dist_sq * self.camera.scale * self.camera.scale;

                if screen_dist_sq <= hit_radius * hit_radius {
                    if closest_node.is_none() || screen_dist_sq < closest_node.as_ref().unwrap().1 {
                        closest_node = Some((node.id.clone(), screen_dist_sq));
                    }
                }
            }
        }

        closest_node.map(|(id, _)| id)
    }

    /// Calculate node positions using grid layout with jitter
    fn calculate_node_positions(
        nodes: &[AssetNode],
        zone_width: f32,
        zone_height: f32,
    ) -> Vec<NodeVirtualPos> {
        let count = nodes.len();
        if count == 0 {
            return vec![];
        }

        let cols = ((count as f32).sqrt().ceil() as usize).max(3).min(8);
        let rows = (count + cols - 1) / cols;

        let margin = 50.0;
        let usable_width = zone_width - 2.0 * margin;
        let usable_height = zone_height - 2.0 * margin;

        nodes
            .iter()
            .enumerate()
            .map(|(idx, node)| {
                let row = idx / cols;
                let col = idx % cols;

                let grid_x = if cols > 1 {
                    col as f32 / (cols - 1) as f32
                } else {
                    0.5
                };
                let grid_y = if rows > 1 {
                    row as f32 / (rows - 1) as f32
                } else {
                    0.5
                };

                // Add pseudo-random jitter based on node ID
                let hash = Self::hash(&node.id);
                let jitter_x = (hash % 50) as f32 - 25.0;
                let jitter_y = ((hash / 100) % 50) as f32 - 25.0;

                let x = margin + grid_x * usable_width + jitter_x;
                let y = margin + grid_y * usable_height + jitter_y;

                NodeVirtualPos {
                    x: x.clamp(margin, zone_width - margin),
                    y: y.clamp(margin, zone_height - margin),
                }
            })
            .collect()
    }

    /// Simple hash function for deterministic jitter
    fn hash(s: &str) -> u64 {
        let mut h: u64 = 5381;
        for b in s.bytes() {
            h = h.wrapping_mul(33).wrapping_add(b as u64);
        }
        h
    }

    /// Render the zone canvas
    pub fn render_canvas(
        &mut self,
        zone_idx: usize,
        selected_id: &Option<String>,
        hovered_id: &Option<String>,
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
                    n.asset_type.clone(),
                    n.severity.clone(),
                    n.connections.clone(),
                )
            })
            .collect();
        let positions = self.node_positions.clone();
        let selected_id_clone = selected_id.clone();
        let hovered_id_clone = hovered_id.clone();

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
                    // Layout pass - nothing special needed
                    move |_bounds, _window, _cx| {},
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
                        );
                    },
                )
                .size_full(),
            )
            // Mouse event handlers
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(move |this, event: &MouseDownEvent, _, cx| {
                    if let Some(zone) = this.zones.get_mut(zone_idx) {
                        let mouse_x: f32 = event.position.x.into();
                        let mouse_y: f32 = event.position.y.into();
                        
                        zone.start_drag(event.position);

                        // Try to select node (account for header offset)
                        let relative_y = mouse_y - 56.0;
                        if relative_y >= 0.0 {
                            if let Some(node_id) = zone.hit_test(mouse_x, relative_y) {
                                cx.emit(super::AssetSelectedEvent::NodeSelected(node_id));
                                cx.notify();
                            }
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
            .on_mouse_move(cx.listener(move |this, event: &MouseMoveEvent, _, cx| {
                if let Some(zone) = this.zones.get_mut(zone_idx) {
                    // Handle dragging
                    if zone.is_dragging() {
                        zone.update_drag(event.position);
                        cx.notify();
                        return;
                    }

                    // Handle hover
                    let mouse_x: f32 = event.position.x.into();
                    let mouse_y: f32 = event.position.y.into();
                    let relative_y = mouse_y - 56.0;

                    if relative_y >= 0.0 {
                        let new_hovered = zone.hit_test(mouse_x, relative_y);
                        if this.hovered_node_id != new_hovered {
                            this.hovered_node_id = new_hovered;
                            cx.notify();
                        }
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

    /// Paint all canvas content
    fn paint_content(
        window: &mut Window,
        origin_x: f32,
        origin_y: f32,
        camera: &Camera,
        nodes_data: &[(String, String, data::models::Severity, Vec<data::models::Connection>)],
        positions: &[NodeVirtualPos],
        selected_id: &Option<String>,
        hovered_id: &Option<String>,
    ) {
        // Draw connections first (behind nodes)
        Self::paint_connections(
            window, origin_x, origin_y, camera, nodes_data, positions, selected_id,
        );

        // Draw nodes
        Self::paint_nodes(
            window, origin_x, origin_y, camera, nodes_data, positions, selected_id, hovered_id,
        );
    }

    fn paint_nodes(
        window: &mut Window,
        origin_x: f32,
        origin_y: f32,
        camera: &Camera,
        nodes_data: &[(String, String, data::models::Severity, Vec<data::models::Connection>)],
        positions: &[NodeVirtualPos],
        selected_id: &Option<String>,
        hovered_id: &Option<String>,
    ) {
        let scale = camera.scale;
        let base_r = 5.0 * scale;
        let select_r = 12.0 * scale;
        let hover_r = 8.0 * scale;

        for (idx, (node_id, asset_type, severity, _)) in nodes_data.iter().enumerate() {
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
                Self::add_circle(&mut pb, screen_pos, base_r + 1.5);
                if let Ok(path) = pb.build() {
                    window.paint_path(path, severity_color);
                }

                // Node body
                let mut pb = PathBuilder::fill();
                Self::add_circle(&mut pb, screen_pos, base_r);
                if let Ok(path) = pb.build() {
                    window.paint_path(path, node_color);
                }
            }
        }
    }

    fn paint_connections(
        window: &mut Window,
        origin_x: f32,
        origin_y: f32,
        camera: &Camera,
        nodes_data: &[(String, String, data::models::Severity, Vec<data::models::Connection>)],
        positions: &[NodeVirtualPos],
        selected_id: &Option<String>,
    ) {
        for (idx, (node_id, _, _, connections)) in nodes_data.iter().enumerate() {
            if let Some(start_pos) = positions.get(idx) {
                let (start_sx, start_sy) = camera.virtual_to_screen(start_pos.x, start_pos.y);
                let start_x = origin_x + start_sx;
                let start_y = origin_y + start_sy;

                for conn in connections {
                    if let Some(target_idx) =
                        nodes_data.iter().position(|(id, _, _, _)| id == &conn.target_id)
                    {
                        if let Some(end_pos) = positions.get(target_idx) {
                            let (end_sx, end_sy) = camera.virtual_to_screen(end_pos.x, end_pos.y);
                            let end_x = origin_x + end_sx;
                            let end_y = origin_y + end_sy;

                            let is_highlighted = selected_id
                                .as_ref()
                                .map(|s| s == node_id || s == conn.target_id.as_str())
                                .unwrap_or(false);

                            let color = if is_highlighted {
                                rgb(CONNECTION_HIGHLIGHT)
                            } else {
                                rgb(CONNECTION_DEFAULT)
                            };
                            let width = if is_highlighted { px(2.0) } else { px(1.0) };

                            let start = Point::new(px(start_x), px(start_y));
                            let end = Point::new(px(end_x), px(end_y));

                            let mut pb = PathBuilder::stroke(width);
                            pb.move_to(start);
                            pb.line_to(end);
                            if let Ok(path) = pb.build() {
                                window.paint_path(path, color);
                            }
                        }
                    }
                }
            }
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
