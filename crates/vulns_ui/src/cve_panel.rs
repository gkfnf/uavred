//! CVE Panel 组件 - CVE/CVSS 信息面板
//!
//! 显示 CVE 详情、CVSS 评分和 AI 分析

use data::VulnData;
use gpui::prelude::FluentBuilder as _;
use gpui::*;
use gpui_component::{
    button::{Button, ButtonVariants as _},
    h_flex,
    scroll::ScrollableElement as _,
    v_flex, IconName, Sizable, StyledExt as _,
};
use ui::theme::*;

/// 获取 CVSS 分数颜色
fn cvss_color(score: f64) -> u32 {
    if score >= 9.0 {
        SEVERITY_CRITICAL
    } else if score >= 7.0 {
        SEVERITY_HIGH
    } else if score >= 4.0 {
        SEVERITY_MEDIUM
    } else {
        SEVERITY_LOW
    }
}

/// 渲染 CVSS 分数条
fn render_cvss_bar(score: f64) -> impl IntoElement {
    let percentage = (score / 10.0 * 100.0).min(100.0) as f32;
    let color = cvss_color(score);

    h_flex()
        .w_full()
        .h(px(8.0))
        .rounded_full()
        .bg(rgb(BG_SECONDARY))
        .overflow_hidden()
        .child(
            div()
                .h_full()
                .w(px(percentage * 3.0)) // Approximate width based on percentage
                .bg(rgb(color))
                .rounded_full(),
        )
}

/// 渲染 CVE 面板
pub fn render_cve_panel(vuln: Option<&VulnData>) -> impl IntoElement {
    v_flex()
        .size_full()
        .bg(rgb(BG_CARD))
        .border_l_1()
        .border_color(rgb(BORDER_COLOR))
        .when_some(vuln, |this, vuln| {
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
                            div()
                                .text_base()
                                .font_semibold()
                                .text_color(rgb(TEXT_PRIMARY))
                                .child("CVE Details"),
                        )
                        .child(
                            Button::new("close-cve")
                                .ghost()
                                .small()
                                .icon(IconName::Close),
                        ),
                )
                // 内容区
                .child(
                    v_flex()
                        .flex_1()
                        .overflow_y_scrollbar()
                        .p(PADDING_LG)
                        .gap(SPACING_LG)
                        // CVE 标识
                        .when_some(vuln.cve.clone(), |this, cve| {
                            this.child(
                                v_flex()
                                    .gap(SPACING_SM)
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_semibold()
                                            .text_color(rgb(TEXT_SECONDARY))
                                            .child("CVE IDENTIFIER"),
                                    )
                                    .child(
                                        h_flex()
                                            .items_center()
                                            .gap(SPACING_SM)
                                            .child(
                                                div()
                                                    .text_lg()
                                                    .font_semibold()
                                                    .text_color(rgb(ACCENT_BLUE))
                                                    .child(cve),
                                            )
                                            .child(
                                                Button::new("copy-cve")
                                                    .ghost()
                                                    .xsmall()
                                                    .icon(IconName::Copy),
                                            ),
                                    ),
                            )
                        })
                        // CWE
                        .when_some(vuln.cwe.clone(), |this, cwe| {
                            this.child(
                                v_flex()
                                    .gap(SPACING_SM)
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_semibold()
                                            .text_color(rgb(TEXT_SECONDARY))
                                            .child("CWE"),
                                    )
                                    .child(
                                        div().text_sm().text_color(rgb(TEXT_PRIMARY)).child(cwe),
                                    ),
                            )
                        })
                        // CVSS 评分
                        .when_some(vuln.cvss.clone(), |this, cvss| {
                            let score = cvss.base_score;
                            let color = cvss_color(score);

                            this.child(
                                v_flex()
                                    .gap(SPACING_MD)
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_semibold()
                                            .text_color(rgb(TEXT_SECONDARY))
                                            .child("CVSS SCORE"),
                                    )
                                    // 分数显示
                                    .child(
                                        h_flex()
                                            .items_center()
                                            .gap(SPACING_MD)
                                            .child(
                                                div()
                                                    .text_3xl()
                                                    .font_bold()
                                                    .text_color(rgb(color))
                                                    .child(format!("{:.1}", score)),
                                            )
                                            .child(
                                                div()
                                                    .px(PADDING_SM)
                                                    .py(px(2.0))
                                                    .rounded(BORDER_RADIUS_SM)
                                                    .bg(rgb(color))
                                                    .text_xs()
                                                    .text_color(rgb(0xffffff))
                                                    .child(format!("{:?}", cvss.base_severity)),
                                            ),
                                    )
                                    // 分数条
                                    .child(render_cvss_bar(score))
                                    // 向量字符串
                                    .child(
                                        div()
                                            .w_full()
                                            .p(PADDING_SM)
                                            .bg(rgb(BG_SECONDARY))
                                            .rounded(BORDER_RADIUS_SM)
                                            .text_xs()
                                            .text_color(rgb(TEXT_SECONDARY))
                                            .child(cvss.vector_string.clone()),
                                    ),
                            )
                        })
                        // 利用信息
                        .child(
                            v_flex()
                                .gap(SPACING_SM)
                                .child(
                                    div()
                                        .text_sm()
                                        .font_semibold()
                                        .text_color(rgb(TEXT_SECONDARY))
                                        .child("EXPLOIT STATUS"),
                                )
                                .child(
                                    h_flex()
                                        .gap(SPACING_SM)
                                        .child(
                                            div()
                                                .px(PADDING_SM)
                                                .py(px(2.0))
                                                .rounded(BORDER_RADIUS_SM)
                                                .bg(rgb(if vuln.exploit_available {
                                                    SEVERITY_CRITICAL
                                                } else {
                                                    BG_SECONDARY
                                                }))
                                                .text_xs()
                                                .text_color(rgb(if vuln.exploit_available {
                                                    0xffffff
                                                } else {
                                                    TEXT_SECONDARY
                                                }))
                                                .child(if vuln.exploit_available {
                                                    "Exploit Available"
                                                } else {
                                                    "No Exploit"
                                                }),
                                        )
                                        .child(
                                            div()
                                                .px(PADDING_SM)
                                                .py(px(2.0))
                                                .rounded(BORDER_RADIUS_SM)
                                                .bg(rgb(if vuln.poc_available {
                                                    SEVERITY_HIGH
                                                } else {
                                                    BG_SECONDARY
                                                }))
                                                .text_xs()
                                                .text_color(rgb(if vuln.poc_available {
                                                    0xffffff
                                                } else {
                                                    TEXT_SECONDARY
                                                }))
                                                .child(if vuln.poc_available {
                                                    "PoC Available"
                                                } else {
                                                    "No PoC"
                                                }),
                                        ),
                                ),
                        )
                        // 参考链接
                        .when(!vuln.references.is_empty(), |this| {
                            this.child(
                                v_flex()
                                    .gap(SPACING_SM)
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_semibold()
                                            .text_color(rgb(TEXT_SECONDARY))
                                            .child("REFERENCES"),
                                    )
                                    .children(vuln.references.iter().take(5).map(|url| {
                                        div()
                                            .text_xs()
                                            .text_color(rgb(ACCENT_BLUE))
                                            .cursor_pointer()
                                            .overflow_hidden()
                                            .child(url.clone())
                                    })),
                            )
                        }),
                )
        })
        .when(vuln.is_none(), |this| {
            this.flex_1().items_center().justify_center().child(
                div()
                    .text_color(rgb(TEXT_MUTED))
                    .child("Select a vulnerability to view CVE details"),
            )
        })
}
