// Flows 工作流视图 - 主 Panel 整合
// 参考设计: WorkFlows.png

pub mod action_bar;
pub mod dag_canvas;
pub mod flow_list;

use action_bar::{ActionBar, FlowStats};
use dag_canvas::DagCanvas;
use flow_list::FlowList;
use data::models::{FlowNode, NodeType};
use gpui::*;
use gpui_component::{h_flex, v_flex, Sizable};
use ui::theme::*;

/// Flows 面板 - 整合工作流列表、DAG 画布和操作栏
pub struct FlowsPanel {
    flow_list: Entity<FlowList>,
    dag_canvas: Entity<DagCanvas>,
    action_bar: Entity<ActionBar>,
    selected_flow_id: Option<String>,
    flows: Vec<FlowNode>,
}

impl FlowsPanel {
    pub fn new(cx: &mut Context<Self>) -> Self {
        // 初始化示例数据
        let mut flows = Vec::new();
        flows.push(FlowNode::new("网络扫描".to_string(), NodeType::Atomic, 1));
        flows.push(FlowNode::new("协议分析".to_string(), NodeType::Atomic, 2));
        flows.push(FlowNode::new("漏洞检测".to_string(), NodeType::Composite, 3));
        flows.push(FlowNode::new("完整扫描流程".to_string(), NodeType::Task, 4));

        // 延迟初始化 flow_list（在 render 时初始化 InputState）
        let flow_list = cx.new(|cx| FlowList {
            flows: flows.clone(),
            selected_flow_id: None,
            search_query: String::new(),
            filter_level: None,
            filter_category: None,
            search_input: None,
        });

        let dag_canvas = cx.new(|cx| {
            let mut canvas = DagCanvas::new(cx);
            canvas.set_nodes(flows.clone(), cx);
            canvas
        });

        let stats = Self::calculate_stats(&flows);
        let action_bar = cx.new(|cx| {
            let mut bar = ActionBar::new(cx);
            bar.set_stats(stats, cx);
            bar
        });

        Self {
            flow_list,
            dag_canvas,
            action_bar,
            selected_flow_id: None,
            flows,
        }
    }

    fn calculate_stats(flows: &[FlowNode]) -> FlowStats {
        let mut stats = FlowStats::default();
        for flow in flows {
            match flow.node_type {
                NodeType::Atomic => stats.atomic_count += 1,
                NodeType::Composite => stats.composite_count += 1,
                NodeType::Task => stats.task_count += 1,
            }
        }
        stats
    }

    fn handle_flow_selection(&mut self, flow_id: Option<String>, cx: &mut Context<Self>) {
        self.selected_flow_id = flow_id.clone();
        
        // 更新 DAG 画布的选中状态
        self.dag_canvas.update(cx, |canvas, cx| {
            canvas.set_selected_node(flow_id.clone(), cx);
        });

        // 更新操作栏的启用状态
        self.action_bar.update(cx, |bar, cx| {
            bar.set_has_selected_flow(flow_id.is_some(), cx);
        });

        cx.notify();
    }
}

impl Render for FlowsPanel {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // 初始化 InputState（如果还没有初始化）
        self.flow_list.update(cx, |list, cx| {
            list.init_input(window, cx);
        });

        // 同步选中状态（简化实现，实际应该通过事件系统）
        let current_selected = self.flow_list.read(cx).selected_flow_id.clone();
        if current_selected != self.selected_flow_id {
            self.handle_flow_selection(current_selected, cx);
        }

        v_flex()
            .size_full()
            .bg(rgb(BG_PRIMARY))
            .child(
                h_flex()
                    .flex_1()
                    .overflow_hidden()
                    .child(
                        div()
                            .w(px(320.0))
                            .border_r(px(1.0))
                            .border_color(rgb(BORDER_COLOR))
                            .child(self.flow_list.clone().into_any_element()),
                    )
                    .child(
                        div()
                            .flex_1()
                            .child(self.dag_canvas.clone().into_any_element()),
                    ),
            )
            .child(self.action_bar.clone().into_any_element())
    }
}
