// T1-6: Vulns 漏洞详情视图 - CVE 数据库面板
// 参考设计: Vulns.png 右侧

use data::VulnData;
use gpui::*;
use gpui_component::{
    button::{Button, ButtonVariants as _},
    div,
    h_flex,
    label::Label,
    tag::Tag,
    v_flex, IconName, Sizable,
};
use ui::theme::*;

/// 渲染 CVE 数据库面板
pub fn render_cve_panel<T: 'static>(
    vuln: Option<&VulnData>,
    cx: &mut Context<T>,
    on_asset_click: impl Fn(&mut T, &mut Context<T>, String) + 'static,
    on_quick_action: impl Fn(&mut T, &mut Context<T>, &str) + 'static,
) -> impl IntoElement {
    match vuln {
        Some(v) => render_cve_content(v, cx, on_asset_click, on_quick_action),
        None => render_empty_state(),
    }
}

/// 渲染 CVE 内容
fn render_cve_content<T: 'static>(
    vuln: &VulnData,
    cx: &mut Context<T>,
    on_asset_click: impl Fn(&mut T, &mut Context<T>, String) + 'static,
    on_quick_action: impl Fn(&mut T, &mut Context<T>, &str) + 'static,
) -> impl IntoElement {
    v_flex()
        .size_full()
        .bg(rgb(BG_CARD))
        .overflow_y_auto()
        .child(render_cvss_score(vuln))
        .child(render_detection_info(vuln))
        .child(render_asset_links(vuln, cx, on_asset_click))
        .child(render_quick_actions(cx, on_quick_action))
}

/// 渲染空状态
fn render_empty_state() -> impl IntoElement {
    v_flex()
        .size_full()
        .bg(rgb(BG_CARD))
        .items_center()
        .justify_center()
        .child(
            Label::new("CVE Database information will appear here")
                .text_sm()
                .text_color(rgb(TEXT_SECONDARY)),
        )
}

/// 渲染 CVSS Score
fn render_cvss_score(vuln: &VulnData) -> impl IntoElement {
    let cvss = vuln.cvss.as_ref();
    let base_score = cvss.map(|c| c.base_score).unwrap_or(0.0);
    let severity_color = match vuln.severity {
        data::VulnSeverity::Critical => rgb(SEVERITY_CRITICAL),
        data::VulnSeverity::High => rgb(SEVERITY_HIGH),
        data::VulnSeverity::Medium => rgb(SEVERITY_MEDIUM),
        data::VulnSeverity::Low => rgb(SEVERITY_LOW),
        data::VulnSeverity::Info => rgb(TEXT_SECONDARY),
    };

    v_flex()
        .w_full()
        .px(PADDING_LG)
        .pt(PADDING_LG)
        .pb(PADDING_MD)
        .gap(px(12.0))
        .border_b(px(1.0))
        .border_color(rgb(BORDER_COLOR))
        .child(
            Label::new("CVSS Score")
                .text_sm()
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(rgb(TEXT_PRIMARY)),
        )
        .child(
            h_flex()
                .gap(px(16.0))
                .items_center()
                .child(
                    div()
                        .w(px(80.0))
                        .h(px(80.0))
                        .rounded_full()
                        .bg(severity_color)
                        .items_center()
                        .justify_center()
                        .child(
                            Label::new(format!("{:.1}", base_score))
                                .text_xl()
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(0xffffff)),
                        ),
                )
                .child(
                    v_flex()
                        .gap(px(4.0))
                        .child(
                            Label::new(format!("Base Score: {:.1}", base_score))
                                .text_sm()
                                .text_color(rgb(TEXT_PRIMARY)),
                        )
                        .child(
                            cvss
                                .and_then(|c| c.temporal_score)
                                .map(|score| {
                                    Label::new(format!("Temporal: {:.1}", score))
                                        .text_xs()
                                        .text_color(rgb(TEXT_SECONDARY))
                                })
                                .unwrap_or_else(|| div()),
                        )
                        .child(
                            cvss
                                .and_then(|c| c.environmental_score)
                                .map(|score| {
                                    Label::new(format!("Environmental: {:.1}", score))
                                        .text_xs()
                                        .text_color(rgb(TEXT_SECONDARY))
                                })
                                .unwrap_or_else(|| div()),
                        ),
                ),
        )
        .child(
            cvss
                .map(|c| {
                    div()
                        .px(PADDING_SM)
                        .py(PADDING_SM)
                        .bg(rgb(BG_SECONDARY))
                        .rounded(BORDER_RADIUS)
                        .child(
                            Label::new(format!("Vector: {}", c.vector_string))
                                .text_xs()
                                .font_family("monospace")
                                .text_color(rgb(TEXT_PRIMARY)),
                        )
                })
                .unwrap_or_else(|| div().into_any_element()),
        )
}

/// 渲染检测信息
fn render_detection_info(vuln: &VulnData) -> impl IntoElement {
    v_flex()
        .w_full()
        .px(PADDING_LG)
        .py(PADDING_MD)
        .gap(px(8.0))
        .border_b(px(1.0))
        .border_color(rgb(BORDER_COLOR))
        .child(
            Label::new("Detection Information")
                .text_sm()
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(rgb(TEXT_PRIMARY)),
        )
        .child(
            v_flex()
                .gap(px(6.0))
                .child(
                    h_flex()
                        .gap(px(8.0))
                        .items_center()
                        .child(
                            Label::new("Detection Time:")
                                .text_xs()
                                .text_color(rgb(TEXT_SECONDARY)),
                        )
                        .child(
                            Label::new(&vuln.detection_time)
                                .text_xs()
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(rgb(TEXT_PRIMARY)),
                        ),
                )
                .child(
                    h_flex()
                        .gap(px(8.0))
                        .items_center()
                        .child(
                            Label::new("Scan Type:")
                                .text_xs()
                                .text_color(rgb(TEXT_SECONDARY)),
                        )
                        .child(
                            Tag::new()
                                .small()
                                .bg(rgb(0x3b82f6))
                                .text_color(rgb(0xffffff))
                                .child(
                                    Label::new(format!("{:?}", vuln.scan_type))
                                        .text_xs(),
                                ),
                        ),
                )
                .child(
                    h_flex()
                        .gap(px(8.0))
                        .items_center()
                        .child(
                            Label::new("Status:")
                                .text_xs()
                                .text_color(rgb(TEXT_SECONDARY)),
                        )
                        .child(
                            Tag::new()
                                .small()
                                .bg(match vuln.status {
                                    data::VulnStatus::New => rgb(0x3b82f6),
                                    data::VulnStatus::Confirmed => rgb(0x10b981),
                                    data::VulnStatus::FalsePositive => rgb(0x6b7280),
                                    data::VulnStatus::Mitigated => rgb(0xfbbf24),
                                    data::VulnStatus::Resolved => rgb(0x10b981),
                                    data::VulnStatus::Validating => rgb(0xf97316),
                                })
                                .text_color(rgb(0xffffff))
                                .child(
                                    Label::new(format!("{:?}", vuln.status))
                                        .text_xs(),
                                ),
                        ),
                ),
        )
}

/// 渲染资产链接
fn render_asset_links<T: 'static>(
    vuln: &VulnData,
    cx: &mut Context<T>,
    on_asset_click: impl Fn(&mut T, &mut Context<T>, String) + 'static,
) -> impl IntoElement {
    v_flex()
        .w_full()
        .px(PADDING_LG)
        .py(PADDING_MD)
        .gap(px(8.0))
        .border_b(px(1.0))
        .border_color(rgb(BORDER_COLOR))
        .child(
            Label::new("Affected Assets")
                .text_sm()
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(rgb(TEXT_PRIMARY)),
        )
        .child(
            v_flex()
                .gap(px(6.0))
                .child(
                    Button::new("asset-link-main")
                        .ghost()
                        .small()
                        .label(&vuln.affected)
                        .icon(IconName::Link)
                        .on_click(cx.listener(move |this, _, _, cx| {
                            on_asset_click(this, cx, vuln.affected.clone());
                        })),
                )
                .children(
                    vuln.affected_systems
                        .iter()
                        .map(|asset| {
                            let asset_clone = asset.clone();
                            Button::new(format!("asset-link-{}", asset))
                                .ghost()
                                .small()
                                .label(asset)
                                .icon(IconName::Link)
                                .on_click(cx.listener(move |this, _, _, cx| {
                                    on_asset_click(this, cx, asset_clone.clone());
                                }))
                        })
                        .collect::<Vec<_>>(),
                ),
        )
}

/// 渲染快速操作按钮
fn render_quick_actions<T: 'static>(
    cx: &mut Context<T>,
    on_quick_action: impl Fn(&mut T, &mut Context<T>, &str) + 'static,
) -> impl IntoElement {
    v_flex()
        .w_full()
        .px(PADDING_LG)
        .py(PADDING_MD)
        .gap(px(8.0))
        .child(
            Label::new("Quick Actions")
                .text_sm()
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(rgb(TEXT_PRIMARY)),
        )
        .child(
            v_flex()
                .gap(px(6.0))
                .child(
                    Button::new("action-validate")
                        .default()
                        .small()
                        .label("Validate")
                        .icon(IconName::Check)
                        .on_click(cx.listener(move |this, _, _, cx| {
                            on_quick_action(this, cx, "validate");
                        })),
                )
                .child(
                    Button::new("action-exploit")
                        .default()
                        .small()
                        .label("Generate Exploit")
                        .icon(IconName::Code)
                        .on_click(cx.listener(move |this, _, _, cx| {
                            on_quick_action(this, cx, "exploit");
                        })),
                )
                .child(
                    Button::new("action-mitigate")
                        .default()
                        .small()
                        .label("Suggest Mitigation")
                        .icon(IconName::Shield)
                        .on_click(cx.listener(move |this, _, _, cx| {
                            on_quick_action(this, cx, "mitigate");
                        })),
                )
                .child(
                    Button::new("action-export")
                        .ghost()
                        .small()
                        .label("Export Report")
                        .icon(IconName::Download)
                        .on_click(cx.listener(move |this, _, _, cx| {
                            on_quick_action(this, cx, "export");
                        })),
                )
                .child(
                    Button::new("action-share")
                        .ghost()
                        .small()
                        .label("Share")
                        .icon(IconName::Share)
                        .on_click(cx.listener(move |this, _, _, cx| {
                            on_quick_action(this, cx, "share");
                        })),
                ),
        )
}
