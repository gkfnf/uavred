//! VulnDetail 组件 - 漏洞详情面板
//!
//! 显示选中漏洞的完整详情

use data::{VulnData, VulnSeverity, VulnStatus};
use gpui::prelude::FluentBuilder as _;
use gpui::*;
use gpui_component::{
    button::{Button, ButtonVariants as _},
    h_flex,
    scroll::ScrollableElement as _,
    v_flex, IconName, Sizable, StyledExt as _,
};
use ui::theme::*;

/// 获取严重性颜色
fn severity_color(severity: &VulnSeverity) -> u32 {
    match severity {
        VulnSeverity::Critical => SEVERITY_CRITICAL,
        VulnSeverity::High => SEVERITY_HIGH,
        VulnSeverity::Medium => SEVERITY_MEDIUM,
        VulnSeverity::Low => SEVERITY_LOW,
        VulnSeverity::Info => TEXT_MUTED,
    }
}

/// 获取状态颜色
fn status_color(status: &VulnStatus) -> u32 {
    match status {
        VulnStatus::New => SEVERITY_HIGH,
        VulnStatus::Validating => ACCENT_BLUE,
        VulnStatus::Confirmed => SEVERITY_CRITICAL,
        VulnStatus::FalsePositive => TEXT_MUTED,
        VulnStatus::Mitigated => SEVERITY_MEDIUM,
        VulnStatus::Resolved => STATUS_SUCCESS,
    }
}

/// 渲染信息行
fn render_info_row(label: &str, value: impl IntoElement) -> impl IntoElement {
    h_flex()
        .items_start()
        .gap(SPACING_MD)
        .child(
            div()
                .w(px(100.0))
                .text_sm()
                .text_color(rgb(TEXT_SECONDARY))
                .child(label.to_string()),
        )
        .child(
            div()
                .flex_1()
                .text_sm()
                .text_color(rgb(TEXT_PRIMARY))
                .child(value),
        )
}

/// 渲染漏洞详情面板
pub fn render_vuln_detail(vuln: Option<&VulnData>) -> impl IntoElement {
    v_flex()
        .size_full()
        .bg(rgb(BG_CARD))
        .when_some(vuln, |this, vuln| {
            let severity_clr = severity_color(&vuln.severity);
            let status_clr = status_color(&vuln.status);

            this
                // 头部
                .child(
                    h_flex()
                        .items_center()
                        .justify_between()
                        .px(PADDING_LG)
                        .py(PADDING_MD)
                        .border_b_1()
                        .border_color(rgb(BORDER_COLOR))
                        .child(
                            h_flex()
                                .items_center()
                                .gap(SPACING_SM)
                                .child(
                                    div()
                                        .text_base()
                                        .font_semibold()
                                        .text_color(rgb(TEXT_PRIMARY))
                                        .child(vuln.title.clone()),
                                )
                                .child(
                                    div()
                                        .px(PADDING_XS)
                                        .py(px(2.0))
                                        .rounded(BORDER_RADIUS_SM)
                                        .bg(rgb(severity_clr))
                                        .text_xs()
                                        .text_color(rgb(0xffffff))
                                        .child(format!("{:?}", vuln.severity)),
                                ),
                        )
                        .child(
                            h_flex()
                                .gap(SPACING_SM)
                                .child(
                                    Button::new("export-vuln")
                                        .ghost()
                                        .small()
                                        .icon(IconName::ArrowDown),
                                )
                                .child(
                                    Button::new("share-vuln")
                                        .ghost()
                                        .small()
                                        .icon(IconName::ExternalLink),
                                ),
                        ),
                )
                // 内容区
                .child(
                    v_flex()
                        .flex_1()
                        .overflow_y_scrollbar()
                        .p(PADDING_LG)
                        .gap(SPACING_LG)
                        // 基本信息区
                        .child(
                            v_flex()
                                .gap(SPACING_MD)
                                .child(
                                    div()
                                        .text_sm()
                                        .font_semibold()
                                        .text_color(rgb(TEXT_SECONDARY))
                                        .child("BASIC INFORMATION"),
                                )
                                .child(render_info_row("ID", vuln.id.clone()))
                                .child(render_info_row(
                                    "Status",
                                    div()
                                        .px(PADDING_SM)
                                        .py(px(2.0))
                                        .rounded(BORDER_RADIUS_SM)
                                        .bg(rgb(status_clr))
                                        .text_color(rgb(0xffffff))
                                        .child(format!("{:?}", vuln.status)),
                                ))
                                .child(render_info_row("Affected", vuln.affected.clone()))
                                .child(render_info_row("Detected", vuln.detection_time.clone())),
                        )
                        // 描述区
                        .child(
                            v_flex()
                                .gap(SPACING_SM)
                                .child(
                                    div()
                                        .text_sm()
                                        .font_semibold()
                                        .text_color(rgb(TEXT_SECONDARY))
                                        .child("DESCRIPTION"),
                                )
                                .child(
                                    div()
                                        .text_sm()
                                        .text_color(rgb(TEXT_PRIMARY))
                                        .child(vuln.description.clone()),
                                ),
                        )
                        // 受影响系统
                        .when(!vuln.affected_systems.is_empty(), |this| {
                            this.child(
                                v_flex()
                                    .gap(SPACING_SM)
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_semibold()
                                            .text_color(rgb(TEXT_SECONDARY))
                                            .child("AFFECTED SYSTEMS"),
                                    )
                                    .child(h_flex().flex_wrap().gap(SPACING_XS).children(
                                        vuln.affected_systems.iter().map(|sys| {
                                            div()
                                                .px(PADDING_SM)
                                                .py(px(2.0))
                                                .rounded(BORDER_RADIUS_SM)
                                                .bg(rgb(BG_SECONDARY))
                                                .text_xs()
                                                .text_color(rgb(TEXT_SECONDARY))
                                                .child(sys.clone())
                                        }),
                                    )),
                            )
                        })
                        // 攻击技术
                        .when(!vuln.attack_techniques.is_empty(), |this| {
                            this.child(
                                v_flex()
                                    .gap(SPACING_SM)
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_semibold()
                                            .text_color(rgb(TEXT_SECONDARY))
                                            .child("MITRE ATT&CK"),
                                    )
                                    .child(h_flex().flex_wrap().gap(SPACING_XS).children(
                                        vuln.attack_techniques.iter().map(|tech| {
                                            div()
                                                .px(PADDING_SM)
                                                .py(px(2.0))
                                                .rounded(BORDER_RADIUS_SM)
                                                .bg(rgb(ACCENT_PURPLE))
                                                .text_xs()
                                                .text_color(rgb(0xffffff))
                                                .child(tech.clone())
                                        }),
                                    )),
                            )
                        }),
                )
        })
        .when(vuln.is_none(), |this| {
            this.flex_1().items_center().justify_center().child(
                div()
                    .text_color(rgb(TEXT_MUTED))
                    .child("Select a vulnerability to view details"),
            )
        })
}
