use gpui::*;
use gpui_component::{h_flex, label::Label, v_flex};
use data::models::{AssetNode, ComplianceStatus};
use ui::theme::*;

pub struct NodeDetail {
    node: Option<AssetNode>,
}

impl NodeDetail {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self { node: None }
    }

    pub fn set_node(&mut self, node: Option<AssetNode>, cx: &mut Context<Self>) {
        self.node = node;
        cx.notify();
    }

    fn render_risk_score_bar(risk_score: u8) -> impl IntoElement {
        let percentage = risk_score as f32 / 100.0;
        let color = if risk_score >= 80 {
            SEVERITY_CRITICAL
        } else if risk_score >= 60 {
            SEVERITY_HIGH
        } else if risk_score >= 40 {
            SEVERITY_MEDIUM
        } else if risk_score >= 20 {
            SEVERITY_LOW
        } else {
            0x10b981
        };

        v_flex()
            .gap(px(4.0))
            .child(
                h_flex()
                    .items_center()
                    .justify_between()
                    .child(
                        Label::new("风险评分")
                            .text_sm()
                            .text_color(rgb(TEXT_SECONDARY))
                            .font_weight(FontWeight::MEDIUM),
                    )
                    .child(
                        Label::new(format!("{}", risk_score))
                            .text_sm()
                            .text_color(rgb(TEXT_PRIMARY))
                            .font_weight(FontWeight::SEMIBOLD),
                    ),
            )
            .child(
                div()
                    .w_full()
                    .h(px(8.0))
                    .bg(rgb(0xf3f4f6))
                    .rounded_full()
                    .overflow_hidden()
                    .child(
                        div()
                            .h_full()
                            .w_full()
                            .child(
                                div()
                                    .h_full()
                                    .w(px(400.0 * percentage))
                                    .bg(rgb(color))
                                    .rounded_full(),
                            ),
                    ),
            )
    }

    fn render_progress_bar(percentage: u8, label: &str) -> impl IntoElement {
        v_flex()
            .gap(px(4.0))
            .child(
                h_flex()
                    .items_center()
                    .justify_between()
                    .child(
                        Label::new(label)
                            .text_sm()
                            .text_color(rgb(TEXT_SECONDARY))
                            .font_weight(FontWeight::MEDIUM),
                    )
                    .child(
                        Label::new(format!("{}%", percentage))
                            .text_sm()
                            .text_color(rgb(TEXT_PRIMARY)),
                    ),
            )
            .child(
                div()
                    .w_full()
                    .h(px(6.0))
                    .bg(rgb(0xf3f4f6))
                    .rounded_full()
                    .overflow_hidden()
                    .child(
                        div()
                            .h_full()
                            .w_full()
                            .child(
                                div()
                                    .h_full()
                                    .w(px(400.0 * (percentage as f32 / 100.0)))
                                    .bg(rgb(ACCENT_PURPLE))
                                    .rounded_full(),
                            ),
                    ),
            )
    }

    fn render_compliance_tag(name: &str, status: ComplianceStatus) -> impl IntoElement {
        let (bg_color, text_color) = match status {
            ComplianceStatus::Compliant => (rgb(0xdcfce7), rgb(0x166534)),
            ComplianceStatus::NonCompliant => (rgb(0xfee2e2), rgb(0x991b1b)),
            ComplianceStatus::Pending => (rgb(0xfef3c7), rgb(0x92400e)),
            ComplianceStatus::NotApplicable => (rgb(0xf3f4f6), rgb(0x6b7280)),
        };

        div()
            .px(PADDING_SM)
            .py(PADDING_XS)
            .bg(bg_color)
            .rounded(BORDER_RADIUS_SM)
            .child(
                Label::new(name)
                    .text_xs()
                    .text_color(text_color)
                    .font_weight(FontWeight::MEDIUM),
            )
    }
}

impl Render for NodeDetail {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if let Some(node) = &self.node {
            v_flex()
                .size_full()
                .gap(px(16.0))
                .p(PADDING_LG)
                .overflow_y_auto()
                .child(
                    div()
                        .pb(PADDING_MD)
                        .border_b(px(1.0))
                        .border_color(rgb(BORDER_COLOR))
                        .child(
                            Label::new(node.zone.display_name())
                                .text_lg()
                                .text_color(rgb(TEXT_PRIMARY))
                                .font_weight(FontWeight::BOLD),
                        ),
                )
                .child(
                    v_flex()
                        .gap(px(12.0))
                        .child(
                            h_flex()
                                .items_center()
                                .justify_between()
                                .child(
                                    Label::new(&node.name)
                                        .text_xl()
                                        .text_color(rgb(TEXT_PRIMARY))
                                        .font_weight(FontWeight::BOLD),
                                )
                                .child(
                                    div()
                                        .px(PADDING_SM)
                                        .py(PADDING_XS)
                                        .bg(rgb(node.severity_color()))
                                        .rounded(BORDER_RADIUS_SM)
                                        .child(
                                            Label::new(node.severity.display_name())
                                                .text_xs()
                                                .text_color(rgb(0xffffff))
                                                .font_weight(FontWeight::SEMIBOLD),
                                        ),
                                ),
                        )
                        .child(
                            Label::new(format!("IP: {}", node.ip_address))
                                .text_sm()
                                .text_color(rgb(TEXT_SECONDARY)),
                        ),
                )
                .child(Self::render_risk_score_bar(node.risk_score))
                .child(
                    div()
                        .p(PADDING_MD)
                        .bg(rgb(BG_SECONDARY))
                        .rounded(BORDER_RADIUS)
                        .child(
                            v_flex()
                                .gap(px(8.0))
                                .child(
                                    Label::new("开放端口")
                                        .text_sm()
                                        .text_color(rgb(TEXT_PRIMARY))
                                        .font_weight(FontWeight::SEMIBOLD),
                                )
                                .child(
                                    if node.open_ports.is_empty() {
                                        Label::new("无开放端口")
                                            .text_sm()
                                            .text_color(rgb(TEXT_MUTED)),
                                    } else {
                                        v_flex()
                                            .gap(px(4.0))
                                            .children(
                                                node.open_ports
                                                    .iter()
                                                    .map(|port| {
                                                        div()
                                                            .px(PADDING_SM)
                                                            .py(PADDING_XS)
                                                            .bg(rgb(BG_CARD))
                                                            .rounded(BORDER_RADIUS_SM)
                                                            .border(px(1.0))
                                                            .border_color(rgb(BORDER_COLOR))
                                                            .child(
                                                                Label::new(format!("{}", port))
                                                                    .text_sm()
                                                                    .text_color(rgb(TEXT_PRIMARY)),
                                                            )
                                                    })
                                                    .collect::<Vec<_>>(),
                                            )
                                    },
                                ),
                        ),
                )
                .child(
                    div()
                        .p(PADDING_MD)
                        .bg(rgb(BG_SECONDARY))
                        .rounded(BORDER_RADIUS)
                        .child(
                            v_flex()
                                .gap(px(8.0))
                                .child(
                                    Label::new("检测到的服务")
                                        .text_sm()
                                        .text_color(rgb(TEXT_PRIMARY))
                                        .font_weight(FontWeight::SEMIBOLD),
                                )
                                .child(
                                    if node.services.is_empty() {
                                        Label::new("无检测到的服务")
                                            .text_sm()
                                            .text_color(rgb(TEXT_MUTED)),
                                    } else {
                                        v_flex()
                                            .gap(px(8.0))
                                            .children(
                                                node.services
                                                    .iter()
                                                    .map(|service| {
                                                        div()
                                                            .p(PADDING_SM)
                                                            .bg(rgb(BG_CARD))
                                                            .rounded(BORDER_RADIUS_SM)
                                                            .border(px(1.0))
                                                            .border_color(rgb(BORDER_COLOR))
                                                            .child(
                                                                v_flex()
                                                                    .gap(px(4.0))
                                                                    .child(
                                                                        Label::new(&service.service_name)
                                                                            .text_sm()
                                                                            .text_color(rgb(TEXT_PRIMARY))
                                                                            .font_weight(FontWeight::MEDIUM),
                                                                    )
                                                                    .child(
                                                                        Label::new(format!(
                                                                            "{}:{}",
                                                                            service.protocol, service.port
                                                                        ))
                                                                            .text_xs()
                                                                            .text_color(rgb(TEXT_SECONDARY)),
                                                                    )
                                                                    .when_some(
                                                                        &service.version,
                                                                        |this, version| {
                                                                            this.child(
                                                                                Label::new(format!("版本: {}", version))
                                                                                    .text_xs()
                                                                                    .text_color(rgb(TEXT_MUTED)),
                                                                            )
                                                                        },
                                                                    ),
                                                            )
                                                    })
                                                    .collect::<Vec<_>>(),
                                            )
                                    },
                                ),
                        ),
                )
                .child(
                    div()
                        .p(PADDING_MD)
                        .bg(rgb(BG_SECONDARY))
                        .rounded(BORDER_RADIUS)
                        .child(
                            v_flex()
                                .gap(px(8.0))
                                .child(
                                    Label::new("认证凭证")
                                        .text_sm()
                                        .text_color(rgb(TEXT_PRIMARY))
                                        .font_weight(FontWeight::SEMIBOLD),
                                )
                                .child(
                                    if node.credentials.is_empty() {
                                        Label::new("无认证凭证")
                                            .text_sm()
                                            .text_color(rgb(TEXT_MUTED)),
                                    } else {
                                        v_flex()
                                            .gap(px(8.0))
                                            .children(
                                                node.credentials
                                                    .iter()
                                                    .map(|cred| {
                                                        div()
                                                            .p(PADDING_SM)
                                                            .bg(rgb(BG_CARD))
                                                            .rounded(BORDER_RADIUS_SM)
                                                            .border(px(1.0))
                                                            .border_color(rgb(BORDER_COLOR))
                                                            .child(
                                                                v_flex()
                                                                    .gap(px(4.0))
                                                                    .child(
                                                                        Label::new(&cred.username)
                                                                            .text_sm()
                                                                            .text_color(rgb(TEXT_PRIMARY))
                                                                            .font_weight(FontWeight::MEDIUM),
                                                                    )
                                                                    .child(
                                                                        Label::new(format!(
                                                                            "类型: {}",
                                                                            cred.auth_type
                                                                        ))
                                                                            .text_xs()
                                                                            .text_color(rgb(TEXT_SECONDARY)),
                                                                    )
                                                                    .when_some(
                                                                        &cred.last_used,
                                                                        |this, last_used| {
                                                                            this.child(
                                                                                Label::new(format!(
                                                                                    "最后使用: {}",
                                                                                    last_used
                                                                                ))
                                                                                    .text_xs()
                                                                                    .text_color(rgb(TEXT_MUTED)),
                                                                            )
                                                                        },
                                                                    ),
                                                            )
                                                    })
                                                    .collect::<Vec<_>>(),
                                            )
                                    },
                                ),
                        ),
                )
                .child(
                    div()
                        .p(PADDING_MD)
                        .bg(rgb(BG_SECONDARY))
                        .rounded(BORDER_RADIUS)
                        .child(
                            v_flex()
                                .gap(px(8.0))
                                .child(
                                    Label::new("业务信息")
                                        .text_sm()
                                        .text_color(rgb(TEXT_PRIMARY))
                                        .font_weight(FontWeight::SEMIBOLD),
                                )
                                .child(
                                    v_flex()
                                        .gap(px(8.0))
                                        .child(
                                            h_flex()
                                                .gap(px(8.0))
                                                .items_center()
                                                .child(
                                                    Label::new("业务用途:")
                                                        .text_sm()
                                                        .text_color(rgb(TEXT_SECONDARY))
                                                        .w(px(80.0)),
                                                )
                                                .child(
                                                    Label::new(if node.business_purpose.is_empty() {
                                                        "未设置"
                                                    } else {
                                                        &node.business_purpose
                                                    })
                                                        .text_sm()
                                                        .text_color(rgb(TEXT_PRIMARY))
                                                        .flex_1(),
                                                ),
                                        )
                                        .child(
                                            h_flex()
                                                .gap(px(8.0))
                                                .items_center()
                                                .child(
                                                    Label::new("负责人:")
                                                        .text_sm()
                                                        .text_color(rgb(TEXT_SECONDARY))
                                                        .w(px(80.0)),
                                                )
                                                .child(
                                                    Label::new(if node.owner.is_empty() {
                                                        "未设置"
                                                    } else {
                                                        &node.owner
                                                    })
                                                        .text_sm()
                                                        .text_color(rgb(TEXT_PRIMARY))
                                                        .flex_1(),
                                                ),
                                        )
                                        .when_some(
                                            &node.department,
                                            |this, dept| {
                                                this.child(
                                                    h_flex()
                                                        .gap(px(8.0))
                                                        .items_center()
                                                        .child(
                                                            Label::new("部门:")
                                                                .text_sm()
                                                                .text_color(rgb(TEXT_SECONDARY))
                                                                .w(px(80.0)),
                                                        )
                                                        .child(
                                                            Label::new(dept)
                                                                .text_sm()
                                                                .text_color(rgb(TEXT_PRIMARY))
                                                                .flex_1(),
                                                        ),
                                                )
                                            },
                                        ),
                                ),
                        ),
                )
                .child(Self::render_progress_bar(
                    node.scan_progress.percentage,
                    "扫描进度",
                ))
                .child(
                    div()
                        .p(PADDING_MD)
                        .bg(rgb(BG_SECONDARY))
                        .rounded(BORDER_RADIUS)
                        .child(
                            v_flex()
                                .gap(px(8.0))
                                .child(
                                    Label::new("合规标准")
                                        .text_sm()
                                        .text_color(rgb(TEXT_PRIMARY))
                                        .font_weight(FontWeight::SEMIBOLD),
                                )
                                .child(
                                    if node.compliance_standards.is_empty() {
                                        Label::new("无合规标准")
                                            .text_sm()
                                            .text_color(rgb(TEXT_MUTED)),
                                    } else {
                                        h_flex()
                                            .gap(px(8.0))
                                            .flex_wrap()
                                            .children(
                                                node.compliance_standards
                                                    .iter()
                                                    .map(|standard| {
                                                        Self::render_compliance_tag(
                                                            &standard.name,
                                                            standard.status,
                                                        )
                                                    })
                                                    .collect::<Vec<_>>(),
                                            )
                                    },
                                ),
                        ),
                )
        } else {
            div()
                .size_full()
                .items_center()
                .justify_center()
                .p(PADDING_XL)
                .child(
                    Label::new("选择节点以查看详情")
                        .text_sm()
                        .text_color(rgb(TEXT_MUTED)),
                )
        }
    }
}
