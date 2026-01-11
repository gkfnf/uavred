// T1-14: Flows 工作流视图 - DAG 画布组件
// 参考设计: WorkFlows.png 右侧

use data::models::{ConnectionType, FlowNode, NodeType};
use gpui::*;
use gpui_component::{
    h_flex,
    label::Label,
    v_flex, Sizable,
};
use std::collections::HashMap;
use ui::theme::*;

/// DAG 节点位置信息
#[derive(Debug, Clone)]
struct NodePosition {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

/// DAG 画布组件
pub struct DagCanvas {
    pub nodes: Vec<FlowNode>,
    pub selected_node_id: Option<String>,
    pub node_positions: HashMap<String, NodePosition>,
    pub critical_path_nodes: Vec<String>,
}

impl DagCanvas {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            nodes: Vec::new(),
            selected_node_id: None,
            node_positions: HashMap::new(),
            critical_path_nodes: Vec::new(),
        }
    }

    pub fn set_nodes(&mut self, nodes: Vec<FlowNode>, cx: &mut Context<Self>) {
        self.nodes = nodes;
        self.calculate_layout();
        cx.notify();
    }

    pub fn set_selected_node(&mut self, node_id: Option<String>, cx: &mut Context<Self>) {
        self.selected_node_id = node_id;
        cx.notify();
    }

    /// 计算节点布局（简单的层次布局算法）
    fn calculate_layout(&mut self) {
        self.node_positions.clear();
        self.critical_path_nodes.clear();

        if self.nodes.is_empty() {
            return;
        }

        // 简单的层次布局：根据依赖关系分层
        let mut layers: Vec<Vec<&FlowNode>> = Vec::new();
        let mut processed = std::collections::HashSet::new();
        let mut remaining: Vec<&FlowNode> = self.nodes.iter().collect();

        // 第一层：没有依赖的节点
        let mut current_layer: Vec<&FlowNode> = remaining
            .iter()
            .filter(|node| node.dependencies.is_empty())
            .cloned()
            .collect();

        while !current_layer.is_empty() {
            layers.push(current_layer.iter().cloned().cloned().collect());
            
            for node in &current_layer {
                processed.insert(node.id.clone());
            }

            // 下一层：依赖已处理的节点
            remaining.retain(|node| !processed.contains(&node.id));
            current_layer = remaining
                .iter()
                .filter(|node| {
                    node.dependencies
                        .iter()
                        .all(|dep_id| processed.contains(dep_id))
                })
                .cloned()
                .collect();
        }

        // 计算位置
        const NODE_WIDTH: f32 = 120.0;
        const NODE_HEIGHT: f32 = 80.0;
        const HORIZONTAL_SPACING: f32 = 150.0;
        const VERTICAL_SPACING: f32 = 120.0;
        const START_X: f32 = 50.0;
        const START_Y: f32 = 50.0;

        for (layer_idx, layer) in layers.iter().enumerate() {
            let y = START_Y + (layer_idx as f32 * VERTICAL_SPACING);
            let layer_width = layer.len() as f32 * HORIZONTAL_SPACING;
            let start_x = START_X + (400.0 - layer_width) / 2.0; // 居中

            for (node_idx, node) in layer.iter().enumerate() {
                let x = start_x + (node_idx as f32 * HORIZONTAL_SPACING);
                
                // 检查是否在关键路径上
                if node.critical_path.is_critical {
                    self.critical_path_nodes.push(node.id.clone());
                }

                self.node_positions.insert(
                    node.id.clone(),
                    NodePosition {
                        x,
                        y,
                        width: NODE_WIDTH,
                        height: NODE_HEIGHT,
                    },
                );
            }
        }
    }

    fn get_connection_color(&self, conn_type: &ConnectionType) -> u32 {
        match conn_type {
            ConnectionType::Dependency => 0x3b82f6,      // 蓝色
            ConnectionType::OnSuccess => 0x10b981,        // 绿色
            ConnectionType::OnFailure => 0xef4444,        // 红色
            ConnectionType::Conditional => 0xf59e0b,       // 黄色
        }
    }

    fn get_node_color(&self, node: &FlowNode) -> u32 {
        if self.critical_path_nodes.contains(&node.id) {
            return 0x7c3aed; // 紫色 - 关键路径
        }

        match node.status {
            data::models::ExecutionStatus::Running => 0x3b82f6,      // 蓝色
            data::models::ExecutionStatus::Completed => 0x10b981,   // 绿色
            data::models::ExecutionStatus::Failed => 0xef4444,       // 红色
            data::models::ExecutionStatus::Pending => 0x9ca3af,       // 灰色
            _ => 0x6b7280,
        }
    }
}

impl Render for DagCanvas {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .bg(rgb(BG_CARD))
            .child(self.render_canvas(cx))
            .child(self.render_stats(cx))
    }
}

impl DagCanvas {
    fn render_canvas(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        // 使用 SVG 或 Canvas 绘制 DAG
        // 这里使用 div 和绝对定位来模拟
        div()
            .flex_1()
            .relative()
            .overflow_hidden()
            .bg(rgb(BG_PRIMARY))
            .child(
                div()
                    .absolute()
                    .inset_0()
                    .child(self.render_connections(cx))
                    .child(self.render_nodes(cx)),
            )
    }

    fn render_connections(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let mut connection_elements = Vec::new();

        for node in &self.nodes {
            let from_pos = match self.node_positions.get(&node.id) {
                Some(pos) => pos,
                None => continue,
            };

            for (target_id, conn_type) in &node.connections {
                let to_pos = match self.node_positions.get(target_id) {
                    Some(pos) => pos,
                    None => continue,
                };

                let color = self.get_connection_color(conn_type);
                let from_x = from_pos.x + from_pos.width / 2.0;
                let from_y = from_pos.y + from_pos.height / 2.0;
                let to_x = to_pos.x + to_pos.width / 2.0;
                let to_y = to_pos.y + to_pos.height / 2.0;

                // 计算箭头方向
                let dx = to_x - from_x;
                let dy = to_y - from_y;
                let length = (dx * dx + dy * dy).sqrt();
                let angle = dy.atan2(dx);

                // 绘制连接线（使用简单的 div 和绝对定位）
                // 注意：这是一个简化的实现，实际应该使用 SVG 或 Canvas
                connection_elements.push(
                    div()
                        .absolute()
                        .left(px(from_x))
                        .top(px(from_y))
                        .w(px(length))
                        .h(px(2.0))
                        .bg(rgb(color))
                        .into_any_element(),
                );
            }
        }

        div().children(connection_elements).into_any_element()
    }

    fn render_nodes(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let mut node_elements = Vec::new();

        for node in &self.nodes {
            let pos = match self.node_positions.get(&node.id) {
                Some(p) => p.clone(),
                None => continue,
            };

            let is_selected = self
                .selected_node_id
                .as_ref()
                .map(|id| id == &node.id)
                .unwrap_or(false);
            let node_color = self.get_node_color(node);
            let is_critical = self.critical_path_nodes.contains(&node.id);

            let node_id = node.id.clone();

            // 节点类型标签
            let type_label = match node.node_type {
                NodeType::Atomic => "原子",
                NodeType::Composite => "组合",
                NodeType::Task => "任务",
            };

            node_elements.push(
                div()
                    .absolute()
                    .left(px(pos.x))
                    .top(px(pos.y))
                    .w(px(pos.width))
                    .h(px(pos.height))
                    .bg(rgb(node_color))
                    .rounded(BORDER_RADIUS)
                    .border(if is_selected { px(3.0) } else { px(1.0) })
                    .border_color(if is_selected {
                        rgb(ACCENT_PURPLE)
                    } else if is_critical {
                        rgb(0x7c3aed)
                    } else {
                        rgb(BORDER_COLOR)
                    })
                    .p(PADDING_SM)
                    .cursor_pointer()
                    .child(
                        v_flex()
                            .size_full()
                            .gap(PADDING_XS)
                            .items_center()
                            .justify_center()
                            .child(
                                Label::new(&node.name)
                                    .text_sm()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(rgb(0xffffff))
                                    .text_center(),
                            )
                            .child(
                                Label::new(format!("ID: {}", &node.id[..8]))
                                    .text_xs()
                                    .text_color(rgb(0xffffff))
                                    .opacity(0.8),
                            )
                            .child(
                                Label::new(type_label)
                                    .text_xs()
                                    .text_color(rgb(0xffffff))
                                    .opacity(0.7),
                            ),
                    )
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |this: &mut Self, _, _, cx| {
                            this.set_selected_node(Some(node_id.clone()), cx);
                        }),
                    )
                    .into_any_element(),
            );
        }

        div().children(node_elements).into_any_element()
    }

    fn render_stats(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        if self.nodes.is_empty() {
            return div().into_any_element();
        }

        let total_nodes = self.nodes.len();
        let max_parallel = self
            .nodes
            .iter()
            .map(|n| n.parallel.max_parallel)
            .max()
            .unwrap_or(0);
        
        let total_duration: u64 = self.nodes.iter().map(|n| n.metrics.estimated_duration_ms).sum();
        let avg_success_rate: f64 = if self.nodes.is_empty() {
            0.0
        } else {
            self.nodes
                .iter()
                .map(|n| n.metrics.success_rate)
                .sum::<f64>()
                / self.nodes.len() as f64
        };
        
        let total_executions: u32 = self.nodes.iter().map(|n| n.metrics.total_executions).sum();

        h_flex()
            .w_full()
            .h(px(60.0))
            .px(PADDING_MD)
            .py(PADDING_SM)
            .border_t(px(1.0))
            .border_color(rgb(BORDER_COLOR))
            .bg(rgb(BG_SECONDARY))
            .gap(PADDING_LG)
            .items_center()
            .child(self.render_stat_item("节点数", &format!("{}", total_nodes)))
            .child(self.render_stat_item("最大并行", &format!("{}", max_parallel)))
            .child(self.render_stat_item("耗时", &format!("{}ms", total_duration)))
            .child(self.render_stat_item("成功率", &format!("{:.1}%", avg_success_rate)))
            .child(self.render_stat_item("运行次数", &format!("{}", total_executions)))
    }

    fn render_stat_item(&self, label: &str, value: &str) -> impl IntoElement {
        v_flex()
            .gap(PADDING_XS)
            .items_center()
            .child(
                Label::new(label)
                    .text_xs()
                    .text_color(rgb(TEXT_SECONDARY)),
            )
            .child(
                Label::new(value)
                    .text_base()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgb(TEXT_PRIMARY)),
            )
    }
}
