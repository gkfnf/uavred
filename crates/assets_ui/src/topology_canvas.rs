use gpui::*;
use gpui_component::{h_flex, label::Label, v_flex};
use data::models::{AssetNode, ZoneType};
use ui::theme::*;

pub struct TopologyCanvas {
    nodes: Vec<AssetNode>,
    selected_node_id: Option<String>,
    hovered_node_id: Option<String>,
}

impl TopologyCanvas {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            nodes: Self::create_sample_nodes(),
            selected_node_id: None,
            hovered_node_id: None,
        }
    }

    pub fn set_nodes(&mut self, nodes: Vec<AssetNode>, cx: &mut Context<Self>) {
        self.nodes = nodes;
        cx.notify();
    }

    pub fn select_node(&mut self, node_id: String, cx: &mut Context<Self>) {
        self.selected_node_id = Some(node_id);
        cx.notify();
    }

    pub fn get_selected_node(&self) -> Option<&AssetNode> {
        self.selected_node_id
            .as_ref()
            .and_then(|id| self.nodes.iter().find(|n| n.id == *id))
    }

    fn create_sample_nodes() -> Vec<AssetNode> {
        vec![
            AssetNode::new(
                "node-1".to_string(),
                "External Gateway".to_string(),
                "192.168.1.1".to_string(),
                ZoneType::Z1,
            ),
            AssetNode::new(
                "node-2".to_string(),
                "DMZ Server".to_string(),
                "192.168.2.10".to_string(),
                ZoneType::Z2,
            ),
            AssetNode::new(
                "node-3".to_string(),
                "Business Server".to_string(),
                "192.168.3.20".to_string(),
                ZoneType::Z3,
            ),
            AssetNode::new(
                "node-4".to_string(),
                "Flight Controller".to_string(),
                "192.168.4.30".to_string(),
                ZoneType::Z4,
            ),
            AssetNode::new(
                "node-5".to_string(),
                "Device Comm".to_string(),
                "192.168.5.40".to_string(),
                ZoneType::Z5,
            ),
        ]
    }


    fn render_node_simple(
        node: &AssetNode,
        is_selected: bool,
        is_hovered: bool,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let node_id = node.id.clone();
        let severity_color = node.severity_color();
        let node_size = if is_selected { px(40.0) } else { px(32.0) };
        let border_width = if is_selected { px(3.0) } else { px(2.0) };

        v_flex()
            .gap(px(4.0))
            .items_center()
            .child(
                div()
                    .w(node_size)
                    .h(node_size)
                    .rounded_full()
                    .bg(rgb(severity_color))
                    .border(border_width)
                    .border_color(if is_selected {
                        rgb(ACCENT_PURPLE)
                    } else {
                        rgb(BG_CARD)
                    })
                    .cursor_pointer()
                    .on_mouse_enter(cx.listener(move |this: &mut Self, _, _, cx: &mut Context<Self>| {
                        this.hovered_node_id = Some(node_id.clone());
                        cx.notify();
                    }))
                    .on_mouse_leave(cx.listener(move |this: &mut Self, _, _, cx: &mut Context<Self>| {
                        this.hovered_node_id = None;
                        cx.notify();
                    }))
                    .on_click(cx.listener(move |this: &mut Self, _, _, cx: &mut Context<Self>| {
                        this.select_node(node_id.clone(), cx);
                    })),
            )
            .when(is_hovered, |this| {
                this.child(
                    div()
                        .px(PADDING_SM)
                        .py(PADDING_XS)
                        .bg(rgb(BG_DARK))
                        .rounded(BORDER_RADIUS_SM)
                        .child(
                            v_flex()
                                .gap(px(2.0))
                                .items_center()
                                .child(
                                    Label::new(&node.name)
                                        .text_xs()
                                        .text_color(rgb(0xffffff))
                                        .font_weight(FontWeight::SEMIBOLD),
                                )
                                .child(
                                    Label::new(&node.ip_address)
                                        .text_xs()
                                        .text_color(rgb(0x9ca3af)),
                                )
                                .child(
                                    Label::new(format!("Risk: {}", node.risk_score))
                                        .text_xs()
                                        .text_color(rgb(severity_color)),
                                ),
                        ),
                )
            })
    }
}

impl Render for TopologyCanvas {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let nodes_by_zone: std::collections::HashMap<ZoneType, Vec<&AssetNode>> =
            self.nodes.iter().fold(
                std::collections::HashMap::new(),
                |mut acc, node| {
                    acc.entry(node.zone).or_insert_with(Vec::new).push(node);
                    acc
                },
            );

        div()
            .size_full()
            .bg(rgb(BG_PRIMARY))
            .overflow_hidden()
            .child(
                h_flex()
                    .size_full()
                    .gap(px(40.0))
                    .px(PADDING_LG)
                    .py(PADDING_LG)
                    .items_center()
                    .justify_center()
                    .children(
                        vec![
                            ZoneType::Z1,
                            ZoneType::Z2,
                            ZoneType::Z3,
                            ZoneType::Z4,
                            ZoneType::Z5,
                        ]
                        .into_iter()
                        .map(|zone| {
                            let zone_nodes: Vec<&AssetNode> = nodes_by_zone
                                .get(&zone)
                                .map(|v| v.clone())
                                .unwrap_or_default();
                            
                            v_flex()
                                .flex_1()
                                .gap(px(20.0))
                                .items_center()
                                .children(
                                    zone_nodes
                                        .iter()
                                        .map(|node| {
                                            let is_selected = self
                                                .selected_node_id
                                                .as_ref()
                                                .map(|id| id == &node.id)
                                                .unwrap_or(false);
                                            let is_hovered = self
                                                .hovered_node_id
                                                .as_ref()
                                                .map(|id| id == &node.id)
                                                .unwrap_or(false);
                                            
                                            self.render_node_simple(
                                                node,
                                                is_selected,
                                                is_hovered,
                                                cx,
                                            )
                                        })
                                        .collect::<Vec<_>>(),
                                )
                        })
                        .collect::<Vec<_>>(),
                    )
                    .child(
                        div()
                            .absolute()
                            .top(px(10.0))
                            .left(px(10.0))
                            .px(PADDING_MD)
                            .py(PADDING_SM)
                            .bg(rgb(BG_CARD))
                            .rounded(BORDER_RADIUS)
                            .border(px(1.0))
                            .border_color(rgb(BORDER_COLOR))
                            .child(
                                h_flex()
                                    .gap(px(16.0))
                                    .items_center()
                                    .children(vec![
                                        div()
                                            .flex()
                                            .gap(px(8.0))
                                            .items_center()
                                            .child(
                                                div()
                                                    .w(px(12.0))
                                                    .h(px(12.0))
                                                    .rounded_full()
                                                    .bg(rgb(SEVERITY_CRITICAL)),
                                            )
                                            .child(Label::new("CRITICAL").text_xs()),
                                        div()
                                            .flex()
                                            .gap(px(8.0))
                                            .items_center()
                                            .child(
                                                div()
                                                    .w(px(12.0))
                                                    .h(px(12.0))
                                                    .rounded_full()
                                                    .bg(rgb(SEVERITY_HIGH)),
                                            )
                                            .child(Label::new("HIGH").text_xs()),
                                        div()
                                            .flex()
                                            .gap(px(8.0))
                                            .items_center()
                                            .child(
                                                div()
                                                    .w(px(12.0))
                                                    .h(px(12.0))
                                                    .rounded_full()
                                                    .bg(rgb(SEVERITY_MEDIUM)),
                                            )
                                            .child(Label::new("MEDIUM").text_xs()),
                                        div()
                                            .flex()
                                            .gap(px(8.0))
                                            .items_center()
                                            .child(
                                                div()
                                                    .w(px(12.0))
                                                    .h(px(12.0))
                                                    .rounded_full()
                                                    .bg(rgb(SEVERITY_LOW)),
                                            )
                                            .child(Label::new("LOW").text_xs()),
                                    ]),
                            ),
                    )
                    .child(
                        div()
                            .absolute()
                            .top(px(10.0))
                            .right(px(10.0))
                            .px(PADDING_MD)
                            .py(PADDING_SM)
                            .bg(rgb(BG_CARD))
                            .rounded(BORDER_RADIUS)
                            .border(px(1.0))
                            .border_color(rgb(BORDER_COLOR))
                            .child(
                                Label::new(format!(
                                    "{} 资产 · {} 连接",
                                    self.nodes.len(),
                                    self.nodes
                                        .iter()
                                        .map(|n| n.connections.len())
                                        .sum::<usize>()
                                ))
                                .text_sm()
                                .text_color(rgb(TEXT_PRIMARY)),
                            ),
                    )
                    .child(
                        div()
                            .absolute()
                            .bottom(px(10.0))
                            .left(px(10.0))
                            .right(px(10.0))
                            .px(PADDING_MD)
                            .py(PADDING_SM)
                            .bg(rgb(BG_CARD))
                            .rounded(BORDER_RADIUS)
                            .border(px(1.0))
                            .border_color(rgb(BORDER_COLOR))
                            .child(
                                h_flex()
                                    .gap(px(24.0))
                                    .items_center()
                                    .justify_center()
                                    .children(
                                        vec![
                                            ZoneType::Z1,
                                            ZoneType::Z2,
                                            ZoneType::Z3,
                                            ZoneType::Z4,
                                            ZoneType::Z5,
                                        ]
                                        .into_iter()
                                        .map(|zone| {
                                            div()
                                                .flex()
                                                .gap(px(8.0))
                                                .items_center()
                                                .px(PADDING_SM)
                                                .py(PADDING_XS)
                                                .bg(rgb(BG_SECONDARY))
                                                .rounded(BORDER_RADIUS_SM)
                                                .child(
                                                    Label::new(zone.display_name())
                                                        .text_sm()
                                                        .text_color(rgb(TEXT_PRIMARY)),
                                                )
                                        })
                                        .collect::<Vec<_>>(),
                                    ),
                            ),
                    )
            )
    }
}
