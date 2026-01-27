use data::models::AssetNode;
use gpui::*;
use gpui_component::{h_flex, label::Label, v_flex, Icon, IconName, Sizable};
use ui::theme::*;

use crate::events::AssetActionEvent;

impl EventEmitter<AssetActionEvent> for AssetDetailPanel {}

/// AssetDetailPanel - Displays detailed information about a selected asset
///
/// Shows:
/// - Asset header with name, IP, and actions (Delete)
/// - Zone and risk score information
/// - Open ports and detected services
/// - Credentials and business purpose
/// - Action buttons (AI Analysis, Scan, Edit)
///
/// Emits AssetActionEvent when user clicks action buttons.
pub struct AssetDetailPanel {
    selected_node: Option<AssetNode>,
}

impl AssetDetailPanel {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self {
            selected_node: None,
        }
    }

    pub fn set_node(&mut self, node: AssetNode, cx: &mut Context<Self>) {
        self.selected_node = Some(node);
        cx.notify();
    }

    pub fn clear_node(&mut self, cx: &mut Context<Self>) {
        self.selected_node = None;
        cx.notify();
    }
}

impl Render for AssetDetailPanel {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if let Some(node) = self.selected_node.clone() {
            v_flex()
                .size_full()
                .gap_0()
                .bg(rgb(0xffffff)) // White background for the panel
                .child(
                    // Panel Header
                    h_flex()
                        .w_full()
                        .p_4()
                        .items_center()
                        .gap_3()
                        .child(
                            Label::new("资产详情")
                                .text_sm()
                                .text_color(rgb(TEXT_SECONDARY)),
                        )
                        .child(div().size(px(8.0)).rounded_full().bg(rgb(0xfbbf24))) // Risk dot
                        .child(
                            Label::new(node.name.clone())
                                .text_base()
                                .font_weight(FontWeight::BOLD),
                        )
                        .child(
                            Label::new(node.ip_address.clone())
                                .text_sm()
                                .text_color(rgb(TEXT_MUTED)),
                        )
                        .child(div().flex_1())
                        .child(
                            h_flex()
                                .gap_4()
                                .child(
                                    Icon::new(IconName::Settings)
                                        .with_size(px(18.0))
                                        .text_color(rgb(TEXT_MUTED)),
                                )
                                .child(
                                    div()
                                        .cursor_pointer()
                                        .on_click({
                                            let node_id = node.id.clone();
                                            move |_, _, cx: &mut Context<AssetDetailPanel>| {
                                                cx.emit(AssetActionEvent::DeleteRequested(
                                                    node_id.clone(),
                                                ));
                                            }
                                        })
                                        .child(
                                            Icon::new(IconName::Delete)
                                                .with_size(px(18.0))
                                                .text_color(rgb(TEXT_MUTED)),
                                        ),
                                )
                                .child(
                                    Icon::new(IconName::ChevronDown)
                                        .with_size(px(18.0))
                                        .text_color(rgb(TEXT_MUTED)),
                                ),
                        ),
                )
                .child(
                    // Content Grid
                    h_flex()
                        .w_full()
                        .p_4()
                        .gap_4()
                        .items_start()
                        // Column 1: Zone & Risk
                        .child(
                            v_flex()
                                .w(px(180.0))
                                .gap_4()
                                .child(
                                    // Zone Card
                                    v_flex()
                                        .p_3()
                                        .rounded_lg()
                                        .bg(rgb(0xfdf4ff)) // Very light purple
                                        .border_1()
                                        .border_color(rgb(0xf5d0fe))
                                        .gap_2()
                                        .child(
                                            h_flex()
                                                .gap_2()
                                                .items_center()
                                                .child(
                                                    Icon::new(IconName::CircleCheck)
                                                        .with_size(px(20.0))
                                                        .text_color(rgb(0x7c3aed)),
                                                )
                                                .child(
                                                    Label::new(format!(
                                                        "Z{}",
                                                        (node.zone as u8 + 1)
                                                    ))
                                                    .font_weight(FontWeight::BOLD)
                                                    .text_color(rgb(0x7c3aed)),
                                                ),
                                        )
                                        .child(
                                            Label::new(node.zone.display_name())
                                                .text_sm()
                                                .font_weight(FontWeight::MEDIUM),
                                        ),
                                )
                                .child(
                                    // Risk Score Card
                                    v_flex()
                                        .p_3()
                                        .rounded_lg()
                                        .bg(rgb(0xfdf4ff))
                                        .border_1()
                                        .border_color(rgb(0xf5d0fe))
                                        .child(
                                            h_flex()
                                                .justify_between()
                                                .child(
                                                    Label::new("风险评分")
                                                        .text_xs()
                                                        .text_color(rgb(0x7c3aed))
                                                        .font_weight(FontWeight::BOLD),
                                                )
                                                .child(
                                                    Label::new(node.risk_score.to_string())
                                                        .text_xl()
                                                        .font_weight(FontWeight::BOLD)
                                                        .text_color(rgb(0x7c3aed)),
                                                ),
                                        )
                                        .child(
                                            div()
                                                .mt_2()
                                                .w_full()
                                                .h(px(6.0))
                                                .bg(rgb(0xe5e7eb))
                                                .rounded_full()
                                                .child(
                                                    div()
                                                        .w(relative(node.risk_score as f32 / 100.0))
                                                        .h_full()
                                                        .bg(rgb(0xfbbf24))
                                                        .rounded_full(),
                                                ),
                                        ),
                                ),
                        )
                        // Column 2: Ports & Services
                        .child(
                            v_flex()
                                .flex_1()
                                .gap_4()
                                .child(
                                    // Open Ports
                                    v_flex()
                                        .gap_2()
                                        .child(
                                            Label::new("开放端口")
                                                .text_xs()
                                                .text_color(rgb(TEXT_SECONDARY))
                                                .font_weight(FontWeight::BOLD),
                                        )
                                        .child(h_flex().gap_2().children(
                                            node.open_ports.iter().map(|port| {
                                                div()
                                                    .px_3()
                                                    .py_1()
                                                    .bg(rgb(0xf3f4f6))
                                                    .rounded_md()
                                                    .child(Label::new(port.to_string()).text_xs())
                                            }),
                                        ))
                                        .child(
                                            Label::new("协议: HTTPS")
                                                .text_xs()
                                                .text_color(rgb(TEXT_MUTED)),
                                        )
                                        .child(
                                            Label::new("最后扫描: 3m ago")
                                                .text_xs()
                                                .text_color(rgb(TEXT_MUTED)),
                                        ),
                                )
                                .child(
                                    // Services
                                    v_flex()
                                        .gap_2()
                                        .child(
                                            Label::new("检测到的服务")
                                                .text_xs()
                                                .text_color(rgb(TEXT_SECONDARY))
                                                .font_weight(FontWeight::BOLD),
                                        )
                                        .child(
                                            v_flex()
                                                .gap_1()
                                                .child(
                                                    div()
                                                        .px_3()
                                                        .py_1()
                                                        .bg(rgb(0xf3f4f6))
                                                        .rounded_md()
                                                        .child(Label::new("HTTPS").text_xs()),
                                                )
                                                .child(
                                                    div()
                                                        .px_3()
                                                        .py_1()
                                                        .bg(rgb(0xf3f4f6))
                                                        .rounded_md()
                                                        .child(Label::new("REST API").text_xs()),
                                                )
                                                .child(
                                                    div()
                                                        .px_3()
                                                        .py_1()
                                                        .bg(rgb(0xf3f4f6))
                                                        .rounded_md()
                                                        .child(Label::new("WebSocket").text_xs()),
                                                ),
                                        ),
                                ),
                        )
                        // Column 3: Credentials & Purpose
                        .child(
                            v_flex()
                                .flex_1()
                                .gap_4()
                                .child(
                                    // Credentials
                                    v_flex()
                                        .p_3()
                                        .rounded_lg()
                                        .bg(rgb(0xf8fafc))
                                        .border_1()
                                        .border_color(rgb(0xe2e8f0))
                                        .gap_2()
                                        .child(
                                            Label::new("认证凭证")
                                                .text_xs()
                                                .text_color(rgb(TEXT_SECONDARY))
                                                .font_weight(FontWeight::BOLD),
                                        )
                                        .child(
                                            Label::new("类型: OAuth2 + MFA")
                                                .text_xs()
                                                .text_color(rgb(TEXT_MUTED)),
                                        )
                                        .child(
                                            div()
                                                .px_3()
                                                .py_1()
                                                .bg(rgb(0xffffff))
                                                .border_1()
                                                .border_color(rgb(0xe2e8f0))
                                                .rounded_md()
                                                .child(Label::new("mission_ctrl").text_xs()),
                                        )
                                        .child(
                                            h_flex()
                                                .gap_1()
                                                .items_center()
                                                .child(
                                                    div()
                                                        .size(px(6.0))
                                                        .rounded_full()
                                                        .bg(rgb(0x10b981)),
                                                )
                                                .child(
                                                    Label::new("有效")
                                                        .text_xs()
                                                        .text_color(rgb(0x10b981)),
                                                ),
                                        ),
                                )
                                .child(
                                    // Purpose
                                    v_flex()
                                        .p_3()
                                        .rounded_lg()
                                        .bg(rgb(0xf0f9ff))
                                        .border_1()
                                        .border_color(rgb(0xbae6fd))
                                        .gap_1()
                                        .child(
                                            Label::new("业务用途")
                                                .text_xs()
                                                .text_color(rgb(0x0284c7))
                                                .font_weight(FontWeight::BOLD),
                                        )
                                        .child(
                                            Label::new("任务规划与执行控制")
                                                .text_xs()
                                                .text_color(rgb(TEXT_PRIMARY)),
                                        ),
                                ),
                        )
                        // Column 4: Actions & Stats
                        .child(
                            v_flex()
                                .w(px(180.0))
                                .gap_2()
                                .child(
                                    div()
                                        .w_full()
                                        .py_2()
                                        .rounded_lg()
                                        .bg(rgb(0xfdf4ff))
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .gap_2()
                                        .cursor_pointer()
                                        .on_click({
                                            let node = node.clone();
                                            move |_, _, cx: &mut Context<AssetDetailPanel>| {
                                                cx.emit(AssetActionEvent::ScanRequested(
                                                    node.clone(),
                                                ));
                                            }
                                        })
                                        .child(
                                            Icon::new(IconName::Star)
                                                .with_size(px(16.0))
                                                .text_color(rgb(0x7c3aed)),
                                        )
                                        .child(
                                            Label::new("AI 分析")
                                                .text_sm()
                                                .font_weight(FontWeight::BOLD)
                                                .text_color(rgb(0x7c3aed)),
                                        ),
                                )
                                .child(
                                    div()
                                        .w_full()
                                        .py_2()
                                        .rounded_lg()
                                        .bg(rgb(0x7c3aed))
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .gap_2()
                                        .cursor_pointer()
                                        .on_click({
                                            let node = node.clone();
                                            move |_, _, cx: &mut Context<AssetDetailPanel>| {
                                                cx.emit(AssetActionEvent::ScanRequested(
                                                    node.clone(),
                                                ));
                                            }
                                        })
                                        .child(
                                            Icon::new(IconName::BatteryCharging)
                                                .with_size(px(16.0))
                                                .text_color(rgb(0xffffff)),
                                        )
                                        .child(
                                            Label::new("扫描资产")
                                                .text_sm()
                                                .font_weight(FontWeight::BOLD)
                                                .text_color(rgb(0xffffff)),
                                        ),
                                )
                                .child(
                                    div()
                                        .w_full()
                                        .py_2()
                                        .rounded_lg()
                                        .bg(rgb(0xf1f5f9))
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .gap_2()
                                        .cursor_pointer()
                                        .on_click({
                                            let node = node.clone();
                                            move |_, _, cx: &mut Context<AssetDetailPanel>| {
                                                cx.emit(AssetActionEvent::EditRequested(
                                                    node.clone(),
                                                ));
                                            }
                                        })
                                        .child(
                                            Icon::new(IconName::Settings)
                                                .with_size(px(16.0))
                                                .text_color(rgb(TEXT_SECONDARY)),
                                        )
                                        .child(
                                            Label::new("配置")
                                                .text_sm()
                                                .font_weight(FontWeight::BOLD)
                                                .text_color(rgb(TEXT_SECONDARY)),
                                        ),
                                )
                                .child(
                                    v_flex()
                                        .mt_4()
                                        .gap_1()
                                        .child(
                                            Label::new("漏洞统计")
                                                .text_xs()
                                                .text_color(rgb(TEXT_MUTED)),
                                        )
                                        .child(
                                            Label::new(node.vulnerabilities_count.to_string())
                                                .text_xl()
                                                .font_weight(FontWeight::BOLD),
                                        ),
                                ),
                        ),
                )
        } else {
            v_flex()
                .size_full()
                .items_center()
                .justify_center()
                .bg(rgb(BG_CARD))
                .rounded_lg()
                .child(
                    v_flex()
                        .items_center()
                        .gap_3()
                        .child(
                            Icon::new(IconName::SquareTerminal)
                                .with_size(px(48.0))
                                .text_color(rgb(TEXT_MUTED)),
                        )
                        .child(
                            Label::new("选择一个资产来查看详情")
                                .text_sm()
                                .text_color(rgb(TEXT_MUTED)),
                        ),
                )
        }
    }
}
