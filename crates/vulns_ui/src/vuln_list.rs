//! VulnList 组件 - 漏洞列表面板
//!
//! 显示漏洞列表，支持筛选和选择

use data::{VulnData, VulnSeverity};
use gpui::prelude::FluentBuilder as _;
use gpui::*;
use gpui_component::{h_flex, scroll::ScrollableElement as _, v_flex, StyledExt as _};
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

/// 渲染漏洞列表项
fn render_vuln_item(vuln: &VulnData, is_selected: bool) -> impl IntoElement {
    let severity_clr = severity_color(&vuln.severity);
    let bg_color = if is_selected { BG_SECONDARY } else { BG_CARD };

    v_flex()
        .id(SharedString::from(format!("vuln-{}", vuln.id)))
        .w_full()
        .p(PADDING_MD)
        .bg(rgb(bg_color))
        .rounded(BORDER_RADIUS)
        .border_1()
        .border_color(rgb(if is_selected {
            ACCENT_PURPLE
        } else {
            BORDER_COLOR
        }))
        .cursor_pointer()
        .gap(SPACING_SM)
        // 标题行
        .child(
            h_flex()
                .items_center()
                .justify_between()
                .child(
                    div()
                        .text_sm()
                        .font_semibold()
                        .text_color(rgb(TEXT_PRIMARY))
                        .max_w(px(180.0))
                        .overflow_hidden()
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
        // 信息行
        .child(
            h_flex()
                .items_center()
                .gap(SPACING_SM)
                .child(
                    div()
                        .text_xs()
                        .text_color(rgb(TEXT_SECONDARY))
                        .child(vuln.id.clone()),
                )
                .when_some(vuln.cve.clone(), |this, cve| {
                    this.child(div().text_xs().text_color(rgb(ACCENT_BLUE)).child(cve))
                }),
        )
}

/// 渲染漏洞列表面板
pub fn render_vuln_list(vulns: &[VulnData], selected_id: Option<&str>) -> impl IntoElement {
    let count = vulns.len();

    v_flex()
        .size_full()
        .bg(rgb(BG_SECONDARY))
        .border_r_1()
        .border_color(rgb(BORDER_COLOR))
        // 头部
        .child(
            v_flex()
                .w_full()
                .p(PADDING_MD)
                .gap(SPACING_SM)
                .border_b_1()
                .border_color(rgb(BORDER_COLOR))
                // 标题
                .child(
                    h_flex()
                        .items_center()
                        .justify_between()
                        .child(
                            div()
                                .text_base()
                                .font_semibold()
                                .text_color(rgb(TEXT_PRIMARY))
                                .child("Vulnerabilities"),
                        )
                        .child(
                            div()
                                .px(PADDING_SM)
                                .py(px(2.0))
                                .rounded_full()
                                .bg(rgb(TEXT_MUTED))
                                .text_xs()
                                .text_color(rgb(0xffffff))
                                .child(format!("{}", count)),
                        ),
                )
                // 搜索框占位符
                .child(
                    div()
                        .w_full()
                        .h(px(32.0))
                        .bg(rgb(BG_CARD))
                        .rounded(BORDER_RADIUS)
                        .border_1()
                        .border_color(rgb(BORDER_COLOR))
                        .px(PADDING_SM)
                        .flex()
                        .items_center()
                        .child(
                            div()
                                .text_sm()
                                .text_color(rgb(TEXT_MUTED))
                                .child("Search vulnerabilities..."),
                        ),
                ),
        )
        // 列表
        .child(
            v_flex()
                .flex_1()
                .overflow_y_scrollbar()
                .p(PADDING_SM)
                .gap(SPACING_SM)
                .children(vulns.iter().map(|vuln| {
                    let is_selected = selected_id == Some(vuln.id.as_str());
                    render_vuln_item(vuln, is_selected)
                })),
        )
}
