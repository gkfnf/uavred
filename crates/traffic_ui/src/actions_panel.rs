// Traffic Actions 面板组件
// T1-11: Traffic 流量分析视图 - Actions 面板

use gpui::*;
use gpui_component::{
    button::{Button, ButtonVariants as _},
    group_box::{GroupBox, GroupBoxVariants as _},
    h_flex,
    label::Label,
    tag::Tag,
    v_flex, Sizable,
};

use data::{TrafficEntry, AnomalyType};
use ui::theme;

/// Actions 面板组件
pub struct ActionsPanel {
    selected_traffic: Option<TrafficEntry>,
    statistics: TrafficStatistics,
    protocol_distribution: Vec<ProtocolStat>,
}

/// 流量统计信息
#[derive(Debug, Clone)]
pub struct TrafficStatistics {
    pub total: usize,
    pub anomalies: usize,
    pub success_rate: f64,
    pub avg_time_ms: u64,
}

impl Default for TrafficStatistics {
    fn default() -> Self {
        Self {
            total: 0,
            anomalies: 0,
            success_rate: 0.0,
            avg_time_ms: 0,
        }
    }
}

/// 协议统计
#[derive(Debug, Clone)]
pub struct ProtocolStat {
    pub protocol: String,
    pub count: usize,
}

impl ActionsPanel {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self {
            selected_traffic: None,
            statistics: TrafficStatistics::default(),
            protocol_distribution: Vec::new(),
        }
    }

    pub fn set_selected_traffic(&mut self, traffic: Option<TrafficEntry>, cx: &mut Context<Self>) {
        self.selected_traffic = traffic;
        cx.notify();
    }

    pub fn set_statistics(&mut self, stats: TrafficStatistics, cx: &mut Context<Self>) {
        self.statistics = stats;
        cx.notify();
    }

    pub fn set_protocol_distribution(&mut self, distribution: Vec<ProtocolStat>, cx: &mut Context<Self>) {
        self.protocol_distribution = distribution;
        cx.notify();
    }
}

impl Render for ActionsPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .w(px(300.0))
            .gap(theme::SPACING_MD)
            .child(render_packet_info(cx, &self.selected_traffic))
            .child(render_action_buttons(cx))
            .child(render_statistics(&self.statistics))
            .child(render_protocols_distribution(&self.protocol_distribution))
    }
}

/// Packet Info 卡片
fn render_packet_info(
    _cx: &mut Context<ActionsPanel>,
    selected_traffic: &Option<TrafficEntry>,
) -> impl IntoElement {
    if let Some(ref traffic) = selected_traffic {
        let has_anomaly = !traffic.anomalies.is_empty();

        GroupBox::new()
            .outline()
            .child(
                v_flex()
                    .gap(theme::SPACING_SM)
                    .p(theme::PADDING_MD)
                    .child(
                        Label::new("Packet Info")
                            .text_base()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(theme::TEXT_PRIMARY)),
                    )
                    .child(
                        v_flex()
                            .gap(theme::SPACING_XS)
                            .child(
                                h_flex()
                                    .justify_between()
                                    .child(Label::new("ID:").text_sm().text_color(rgb(theme::TEXT_SECONDARY)))
                                    .child(Label::new(traffic.id.to_string()).text_sm().text_color(rgb(theme::TEXT_PRIMARY))),
                            )
                            .child(
                                h_flex()
                                    .justify_between()
                                    .child(Label::new("Size:").text_sm().text_color(rgb(theme::TEXT_SECONDARY)))
                                    .child(Label::new(format!("{}B", traffic.response_size)).text_sm().text_color(rgb(theme::TEXT_PRIMARY))),
                            )
                            .child(
                                h_flex()
                                    .justify_between()
                                    .child(Label::new("Time:").text_sm().text_color(rgb(theme::TEXT_SECONDARY)))
                                    .child(Label::new(format!("{}ms", traffic.duration_ms)).text_sm().text_color(rgb(theme::TEXT_PRIMARY))),
                            ),
                    )
                    .when(has_anomaly, |this| {
                        this.child(
                            h_flex()
                                .gap(theme::SPACING_XS)
                                .items_center()
                                .pt(theme::SPACING_XS)
                                .child(
                                    div()
                                        .w(px(6.0))
                                        .h(px(6.0))
                                        .rounded_full()
                                        .bg(rgb(theme::SEVERITY_CRITICAL)),
                                )
                                .child(
                                    Label::new("Anomaly Detected")
                                        .text_sm()
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .text_color(rgb(theme::SEVERITY_CRITICAL)),
                                ),
                        )
                    }),
            )
    } else {
        GroupBox::new()
            .outline()
            .child(
                v_flex()
                    .p(theme::PADDING_MD)
                    .items_center()
                    .justify_center()
                    .h(px(120.0))
                    .child(
                        Label::new("No packet selected")
                            .text_sm()
                            .text_color(rgb(theme::TEXT_MUTED)),
                    )
            )
    }
}

/// 操作按钮组
fn render_action_buttons(cx: &mut Context<ActionsPanel>) -> impl IntoElement {
    v_flex()
        .gap(theme::SPACING_SM)
        .child(
            Button::new("replay-request")
                .label("Replay")
                .w_full()
                .on_click(cx.listener(|_this, _, window, cx| {
                    // TODO: 实现重放功能
                    window.push_notification("Replaying request...", cx);
                })),
        )
        .child(
            Button::new("fuzz-request")
                .label("FUZZ")
                .primary()
                .w_full()
                .on_click(cx.listener(|_this, _, window, cx| {
                    // TODO: 实现模糊测试功能
                    window.push_notification("Starting fuzzing...", cx);
                })),
        )
        .child(
            Button::new("export-curl")
                .label("Export as cURL")
                .ghost()
                .w_full()
                .on_click(cx.listener(|_this, _, window, cx| {
                    // TODO: 实现导出 cURL 功能
                    window.push_notification("Exported as cURL", cx);
                })),
        )
}

/// Statistics 卡片
fn render_statistics(stats: &TrafficStatistics) -> impl IntoElement {
    GroupBox::new()
        .outline()
        .child(
            v_flex()
                .gap(theme::SPACING_SM)
                .p(theme::PADDING_MD)
                .child(
                    Label::new("Statistics")
                        .text_base()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(theme::TEXT_PRIMARY)),
                )
                .child(
                    v_flex()
                        .gap(theme::SPACING_XS)
                        .child(
                            h_flex()
                                .justify_between()
                                .child(Label::new("Total:").text_sm().text_color(rgb(theme::TEXT_SECONDARY)))
                                .child(Label::new(stats.total.to_string()).text_sm().text_color(rgb(theme::TEXT_PRIMARY))),
                        )
                        .child(
                            h_flex()
                                .justify_between()
                                .child(Label::new("Anomalies:").text_sm().text_color(rgb(theme::TEXT_SECONDARY)))
                                .child(
                                    Label::new(stats.anomalies.to_string())
                                        .text_sm()
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .text_color(rgb(theme::SEVERITY_CRITICAL)),
                                ),
                        )
                        .child(
                            h_flex()
                                .justify_between()
                                .child(Label::new("Success:").text_sm().text_color(rgb(theme::TEXT_SECONDARY)))
                                .child(
                                    Label::new(format!("{:.1}%", stats.success_rate))
                                        .text_sm()
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .text_color(rgb(theme::STATUS_SUCCESS)),
                                ),
                        )
                        .child(
                            h_flex()
                                .justify_between()
                                .child(Label::new("Avg Time:").text_sm().text_color(rgb(theme::TEXT_SECONDARY)))
                                .child(
                                    Label::new(format!("{}ms", stats.avg_time_ms))
                                        .text_sm()
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .text_color(rgb(theme::ACCENT_PURPLE)),
                                ),
                        ),
                ),
        )
}

/// Protocols 分布条
fn render_protocols_distribution(protocols: &[ProtocolStat]) -> impl IntoElement {
    let total: usize = protocols.iter().map(|p| p.count).sum();
    let max_count = protocols.iter().map(|p| p.count).max().unwrap_or(1);

    GroupBox::new()
        .outline()
        .child(
            v_flex()
                .gap(theme::SPACING_SM)
                .p(theme::PADDING_MD)
                .child(
                    Label::new("Protocols")
                        .text_base()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(theme::TEXT_PRIMARY)),
                )
                .children(protocols.iter().map(|protocol_stat| {
                    let percentage = if total > 0 {
                        (protocol_stat.count as f64 / total as f64) * 100.0
                    } else {
                        0.0
                    };
                    let bar_width = if max_count > 0 {
                        (protocol_stat.count as f64 / max_count as f64) * 100.0
                    } else {
                        0.0
                    };

                    v_flex()
                        .gap(theme::SPACING_XS)
                        .child(
                            h_flex()
                                .justify_between()
                                .items_center()
                                .child(
                                    Label::new(&protocol_stat.protocol)
                                        .text_sm()
                                        .text_color(rgb(theme::TEXT_PRIMARY)),
                                )
                                .child(
                                    Label::new(protocol_stat.count.to_string())
                                        .text_sm()
                                        .text_color(rgb(theme::TEXT_SECONDARY)),
                                ),
                        )
                        .child(
                            div()
                                .w_full()
                                .h(px(6.0))
                                .bg(rgb(theme::BG_SECONDARY))
                                .rounded(theme::BORDER_RADIUS_SM)
                                .overflow_hidden()
                                .child(
                                    div()
                                        .h_full()
                                        .w(Percentage::from(bar_width))
                                        .bg(rgb(theme::ACCENT_BLUE)),
                                ),
                        )
                }))
                .when(protocols.is_empty(), |this| {
                    this.child(
                        Label::new("No protocol data")
                            .text_sm()
                            .text_color(rgb(theme::TEXT_MUTED)),
                    )
                }),
        )
}
