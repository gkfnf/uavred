// T1-13: Flows 工作流视图 - 工作流列表组件
// 参考设计: WorkFlows.png 左侧

use data::models::{FlowNode, NodeType};
use gpui::*;
use gpui_component::{
    button::{Button, ButtonVariants as _},
    group_box::GroupBox,
    h_flex,
    input::InputState,
    label::Label,
    tag::Tag,
    v_flex, IconName, Sizable,
};
use ui::theme::*;

/// 工作流列表组件
pub struct FlowList {
    pub flows: Vec<FlowNode>,
    pub selected_flow_id: Option<String>,
    pub search_query: String,
    pub filter_level: Option<String>,
    pub filter_category: Option<String>,
    search_input: Option<Entity<InputState>>,
}

impl FlowList {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let search_input = cx.new(|cx| InputState::new(window, cx));

        // 初始化示例数据
        let mut flows = Vec::new();
        flows.push(FlowNode::new("网络扫描".to_string(), NodeType::Atomic, 1));
        flows.push(FlowNode::new("协议分析".to_string(), NodeType::Atomic, 2));
        flows.push(FlowNode::new("漏洞检测".to_string(), NodeType::Composite, 3));
        flows.push(FlowNode::new("完整扫描流程".to_string(), NodeType::Task, 4));

        Self {
            flows,
            selected_flow_id: None,
            search_query: String::new(),
            filter_level: None,
            filter_category: None,
            search_input: Some(search_input),
        }
    }

    /// 延迟初始化 InputState（当 window 可用时）
    pub fn init_input(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.search_input.is_none() {
            self.search_input = Some(cx.new(|cx| InputState::new(window, cx)));
        }
    }

    pub fn set_selected_flow(&mut self, flow_id: Option<String>, cx: &mut Context<Self>) {
        self.selected_flow_id = flow_id;
        cx.notify();
    }

    pub fn set_flows(&mut self, flows: Vec<FlowNode>, cx: &mut Context<Self>) {
        self.flows = flows;
        cx.notify();
    }

    fn get_filtered_flows(&self) -> Vec<&FlowNode> {
        self.flows
            .iter()
            .filter(|flow| {
                // 搜索过滤
                if !self.search_query.is_empty() {
                    if !flow.name.to_lowercase().contains(&self.search_query.to_lowercase()) {
                        return false;
                    }
                }

                // 层级过滤
                if let Some(ref level) = self.filter_level {
                    let matches = match level.as_str() {
                        "atomic" => flow.node_type == NodeType::Atomic,
                        "composite" => flow.node_type == NodeType::Composite,
                        "task" => flow.node_type == NodeType::Task,
                        _ => true,
                    };
                    if !matches {
                        return false;
                    }
                }

                // 类别过滤（这里可以根据需要扩展）
                if let Some(ref category) = self.filter_category {
                    // 可以根据 flow 的 metadata 或其他字段进行过滤
                    if !category.is_empty() {
                        // 示例：可以根据需要实现
                    }
                }

                true
            })
            .collect()
    }

    fn group_flows_by_type(&self, flows: Vec<&FlowNode>) -> (Vec<&FlowNode>, Vec<&FlowNode>, Vec<&FlowNode>) {
        let mut atomic = Vec::new();
        let mut composite = Vec::new();
        let mut task = Vec::new();

        for flow in flows {
            match flow.node_type {
                NodeType::Atomic => atomic.push(flow),
                NodeType::Composite => composite.push(flow),
                NodeType::Task => task.push(flow),
            }
        }

        (atomic, composite, task)
    }
}

impl Render for FlowList {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let filtered_flows = self.get_filtered_flows();
        let (atomic_flows, composite_flows, task_flows) = self.group_flows_by_type(filtered_flows);

        v_flex()
            .size_full()
            .bg(rgb(BG_CARD))
            .border_r(px(1.0))
            .border_color(rgb(BORDER_COLOR))
            .child(self.render_header(cx))
            .child(self.render_filters(cx))
            .child(
                div()
                    .flex_1()
                    .overflow_y_auto()
                    .px(PADDING_MD)
                    .py(PADDING_SM)
                    .child(
                        v_flex()
                            .gap(PADDING_MD)
                            .child(self.render_flow_group("原子工作流", &atomic_flows, cx))
                            .child(self.render_flow_group("组合工作流", &composite_flows, cx))
                            .child(self.render_flow_group("任务工作流", &task_flows, cx)),
                    ),
            )
    }
}

impl FlowList {
    fn render_header(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .w_full()
            .h(px(48.0))
            .px(PADDING_MD)
            .items_center()
            .justify_between()
            .border_b(px(1.0))
            .border_color(rgb(BORDER_COLOR))
            .bg(rgb(BG_SECONDARY))
            .child(
                Label::new("工作流")
                    .text_lg()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgb(TEXT_PRIMARY)),
            )
            .child(
                Button::new("new-flow-btn")
                    .primary()
                    .small()
                    .icon(IconName::Plus)
                    .label("新建")
                    .on_click(cx.listener(|this: &mut Self, _, _, cx| {
                        // TODO: 实现新建工作流逻辑
                        cx.notify();
                    })),
            )
    }

    fn render_filters(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .w_full()
            .gap(PADDING_SM)
            .px(PADDING_MD)
            .py(PADDING_SM)
            .border_b(px(1.0))
            .border_color(rgb(BORDER_COLOR))
            .bg(rgb(BG_CARD))
            .child(
                h_flex()
                    .w_full()
                    .gap(PADDING_SM)
                    .items_center()
                    .child(
                        div()
                            .flex_1()
                            .h(px(32.0))
                            .px(PADDING_SM)
                            .bg(rgb(BG_SECONDARY))
                            .rounded(BORDER_RADIUS)
                            .border(px(1.0))
                            .border_color(rgb(BORDER_COLOR))
                            .child(
                                Label::new(if self.search_query.is_empty() {
                                    "搜索工作流...".to_string()
                                } else {
                                    self.search_query.clone()
                                })
                                .text_sm()
                                .text_color(if self.search_query.is_empty() {
                                    rgb(TEXT_MUTED)
                                } else {
                                    rgb(TEXT_PRIMARY)
                                }),
                            )
                            .cursor_text()
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this: &mut Self, _, _, cx| {
                                    // TODO: 实现实际的输入框交互
                                    // 这里使用简化版本，实际应该使用 InputState
                                    cx.notify();
                                }),
                            ),
                    )
                    .child(
                        Button::new("filter-level-btn")
                            .outline()
                            .small()
                            .label(if let Some(ref level) = self.filter_level {
                                match level.as_str() {
                                    "atomic" => "原子",
                                    "composite" => "组合",
                                    "task" => "任务",
                                    _ => "层级",
                                }
                            } else {
                                "层级"
                            })
                            .on_click(cx.listener(|this: &mut Self, _, _, cx| {
                                // TODO: 实现下拉选择
                                this.filter_level = if this.filter_level.is_some() {
                                    None
                                } else {
                                    Some("atomic".to_string())
                                };
                                cx.notify();
                            })),
                    )
                    .child(
                        Button::new("filter-category-btn")
                            .outline()
                            .small()
                            .label(if let Some(ref cat) = self.filter_category {
                                match cat.as_str() {
                                    "scan" => "扫描",
                                    "analysis" => "分析",
                                    "test" => "测试",
                                    _ => "类别",
                                }
                            } else {
                                "类别"
                            })
                            .on_click(cx.listener(|this: &mut Self, _, _, cx| {
                                // TODO: 实现下拉选择
                                this.filter_category = if this.filter_category.is_some() {
                                    None
                                } else {
                                    Some("scan".to_string())
                                };
                                cx.notify();
                            })),
                    ),
            )
    }

    fn render_flow_group(
        &mut self,
        title: &str,
        flows: &[&FlowNode],
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        if flows.is_empty() {
            return div().into_any_element();
        }

        v_flex()
            .gap(PADDING_SM)
            .child(
                Label::new(format!("{} ({})", title, flows.len()))
                    .text_sm()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgb(TEXT_SECONDARY)),
            )
            .child(
                v_flex()
                    .gap(PADDING_SM)
                    .children(flows.iter().map(|flow| {
                        self.render_flow_card(flow, cx)
                    })),
            )
            .into_any_element()
    }

    fn render_flow_card(&mut self, flow: &FlowNode, cx: &mut Context<Self>) -> impl IntoElement {
        let flow_id = flow.id.clone();
        let is_selected = self.selected_flow_id.as_ref().map(|id| id == &flow_id).unwrap_or(false);

        // 节点类型标签颜色
        let (type_bg, type_text, type_label) = match flow.node_type {
            NodeType::Atomic => (rgb(0x3b82f6), rgb(0xffffff), "原子"),
            NodeType::Composite => (rgb(0x10b981), rgb(0xffffff), "组合"),
            NodeType::Task => (rgb(0x7c3aed), rgb(0xffffff), "任务"),
        };

        // 状态标签颜色
        let (status_bg, status_text, status_label) = match flow.status {
            data::models::ExecutionStatus::Pending => (rgb(0xf3f4f6), rgb(0x6b7280), "待执行"),
            data::models::ExecutionStatus::Running => (rgb(0x3b82f6), rgb(0xffffff), "运行中"),
            data::models::ExecutionStatus::Completed => (rgb(0x10b981), rgb(0xffffff), "已完成"),
            data::models::ExecutionStatus::Failed => (rgb(0xef4444), rgb(0xffffff), "失败"),
            data::models::ExecutionStatus::Skipped => (rgb(0x9ca3af), rgb(0xffffff), "跳过"),
            data::models::ExecutionStatus::Cancelled => (rgb(0x6b7280), rgb(0xffffff), "已取消"),
        };

        let mut card = GroupBox::new()
            .outline()
            .child(
                v_flex()
                    .gap(PADDING_SM)
                    .p(PADDING_MD)
                    .child(
                        h_flex()
                            .items_center()
                            .justify_between()
                            .child(
                                Label::new(&flow.name)
                                    .text_base()
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(rgb(TEXT_PRIMARY)),
                            )
                            .child(
                                h_flex()
                                    .gap(PADDING_XS)
                                    .items_center()
                                    .child(
                                        Tag::new()
                                            .small()
                                            .bg(type_bg)
                                            .text_color(type_text)
                                            .child(Label::new(type_label).text_xs()),
                                    )
                                    .child(
                                        Tag::new()
                                            .small()
                                            .bg(status_bg)
                                            .text_color(status_text)
                                            .child(Label::new(status_label).text_xs()),
                                    ),
                            ),
                    )
                    .child(
                        h_flex()
                            .gap(PADDING_MD)
                            .items_center()
                            .child(
                                Label::new(format!("步骤: {}", flow.step_info.step_number))
                                    .text_sm()
                                    .text_color(rgb(TEXT_SECONDARY)),
                            )
                            .child(
                                Label::new(format!(
                                    "耗时: {}ms",
                                    flow.metrics.estimated_duration_ms
                                ))
                                .text_sm()
                                .text_color(rgb(TEXT_SECONDARY)),
                            ),
                    )
                    .child(
                        if flow.targets.is_empty() {
                            div().into_any_element()
                        } else {
                            v_flex()
                                .gap(PADDING_XS)
                                .child(
                                    Label::new("目标:")
                                        .text_xs()
                                        .text_color(rgb(TEXT_MUTED)),
                                )
                                .child(
                                    h_flex()
                                        .gap(PADDING_XS)
                                        .flex_wrap()
                                        .children(flow.targets.iter().take(3).map(|target| {
                                            Tag::new()
                                                .small()
                                                .bg(rgb(0xf3f4f6))
                                                .text_color(rgb(TEXT_SECONDARY))
                                                .child(Label::new(target).text_xs())
                                        })),
                                )
                                .into_any_element()
                        },
                    ),
            );

        // 选中状态：紫色边框
        if is_selected {
            card = card.border(px(2.0)).border_color(rgb(ACCENT_PURPLE));
        }

        div()
            .w_full()
            .cursor_pointer()
            .child(card)
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(move |this: &mut Self, _, _, cx| {
                    this.set_selected_flow(Some(flow_id.clone()), cx);
                }),
            )
    }
}
