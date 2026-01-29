//! Vulnerability Detail Panel
//!
//! 中间漏洞详情面板，包含：
//! - 统一高度的 PanelHeader
//! - 漏洞基本信息
//! - AI 安全分析
//! - PoC 代码块
//! - MITRE ATT&CK 技术
//! - AI 建议

use crate::components::*;
use crate::state::VulnState;
use data::{VulnData, VulnSeverity};
use gpui::prelude::FluentBuilder as _;
use gpui::*;
use gpui_component::{
    button::Button,
    scroll::ScrollableElement as _,
    v_flex,
    IconName,
    Sizable,
};
use ui::theme::*;

/// 漏洞详情面板
pub struct VulnDetailPanel {
    state: Entity<VulnState>,
}

impl VulnDetailPanel {
    pub fn new(state: Entity<VulnState>) -> Self {
        Self { state }
    }
}

impl Render for VulnDetailPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let vuln = self.state.read(cx).selected().cloned();

        if let Some(vuln) = vuln {
            render_vuln_detail_content(&vuln).into_any_element()
        } else {
            render_empty_state().into_any_element()
        }
    }
}

/// 渲染漏洞详情内容
fn render_vuln_detail_content(vuln: &VulnData) -> impl IntoElement {
    v_flex()
        .size_full()
        .bg(rgb(BG_CARD))
        .overflow_y_scrollbar()
        // 头部 - 统一 42px 高度，与左侧/右侧栏对齐
        .child(PanelHeader::new("Details & PoC").show_border(true))
        // 内容区域
        .child(
            v_flex()
                .p(PADDING_LG)
                .gap(SPACING_XL)
                // 漏洞头部信息
                .child(render_vuln_header(vuln))
                // 漏洞描述
                .child(render_vuln_description(vuln))
                // AI 安全分析
                .child(render_ai_analysis(vuln))
                // PoC 代码块
                .child(render_poc_section(vuln))
                // MITRE ATT&CK 技术
                .child(render_mitre_techniques(vuln))
                // AI 建议
                .child(render_ai_recommendation(vuln))
                // 底部按钮
                .child(render_action_buttons()),
        )
}

/// 渲染漏洞头部信息
fn render_vuln_header(vuln: &VulnData) -> impl IntoElement {
    let severity_label = format!("{:?}", vuln.severity).to_uppercase();
    let severity_color = match vuln.severity {
        VulnSeverity::Critical => SEVERITY_CRITICAL,
        VulnSeverity::High => SEVERITY_HIGH,
        VulnSeverity::Medium => SEVERITY_MEDIUM,
        VulnSeverity::Low => SEVERITY_LOW,
        VulnSeverity::Info => TEXT_MUTED,
    };

    v_flex()
        .gap(SPACING_MD)
        // 标签行
        .child(
            gpui_component::h_flex()
                .items_center()
                .gap(SPACING_SM)
                // 严重程度标签
                .child(
                    div()
                        .px(PADDING_SM)
                        .py(px(4.0))
                        .rounded(BORDER_RADIUS_SM)
                        .border_1()
                        .border_color(rgb(severity_color))
                        .text_xs()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(severity_color))
                        .child(severity_label),
                )
                // CVE ID
                .when(vuln.cve.is_some(), |this| {
                    this.child(
                        div()
                            .text_sm()
                            .text_color(rgb(TEXT_SECONDARY))
                            .child(vuln.cve.clone().unwrap()),
                    )
                })
                // 分隔点
                .child(div().text_sm().text_color(rgb(TEXT_MUTED)).child("·"))
                // CWE ID
                .when(vuln.cwe.is_some(), |this| {
                    this.child(
                        div()
                            .text_sm()
                            .text_color(rgb(TEXT_SECONDARY))
                            .child(vuln.cwe.clone().unwrap()),
                    )
                }),
        )
        // 标题
        .child(
            div()
                .text_xl()
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(rgb(TEXT_PRIMARY))
                .child(vuln.title.clone()),
        )
}

/// 渲染漏洞描述
fn render_vuln_description(vuln: &VulnData) -> impl IntoElement {
    v_flex()
        .gap(SPACING_SM)
        // 描述文本
        .child(
            div()
                .text_sm()
                .text_color(rgb(TEXT_SECONDARY))
                .line_height(px(22.0))
                .child(vuln.description.clone()),
        )
        // 检测位置
        .child(
            gpui_component::h_flex()
                .items_center()
                .gap(SPACING_SM)
                .child(
                    div()
                        .text_xs()
                        .text_color(rgb(TEXT_MUTED))
                        .child("Detected in:"),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(rgb(TEXT_SECONDARY))
                        .child(vuln.detection_location.component.clone()),
                )
                .when(vuln.detection_location.file_path.is_some(), |this| {
                    let path = vuln.detection_location.file_path.clone().unwrap();
                    this.child(div().text_xs().text_color(rgb(TEXT_MUTED)).child("·"))
                        .child(
                            div()
                                .text_xs()
                                .font_family("monospace")
                                .text_color(rgb(TEXT_SECONDARY))
                                .child(format!(
                                    "{}{}",
                                    path,
                                    vuln.detection_location
                                        .line_number
                                        .map(|n| format!(":{}", n))
                                        .unwrap_or_default()
                                )),
                        )
                }),
        )
}

/// 渲染 AI 安全分析
fn render_ai_analysis(vuln: &VulnData) -> impl IntoElement {
    let analysis = match &vuln.ai_analysis {
        Some(a) => a,
        None => return div().into_any_element(),
    };

    v_flex()
        .p(PADDING_LG)
        .gap(SPACING_MD)
        .rounded(BORDER_RADIUS)
        .bg(rgb(0xfdf4ff))
        .border_1()
        .border_color(rgb(0xf5d0fe))
        // 标题
        .child(
            gpui_component::h_flex()
                .items_center()
                .gap(SPACING_SM)
                .child(
                    div()
                        .w(px(8.0))
                        .h(px(8.0))
                        .rounded_full()
                        .bg(rgb(ACCENT_PURPLE)),
                )
                .child(
                    div()
                        .text_sm()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(ACCENT_PURPLE))
                        .child("AI Security Analysis"),
                ),
        )
        // 分数条
        .child(ScoreBar::new("Confidence Score", analysis.confidence_score * 100.0).color(ACCENT_PURPLE))
        .child(ScoreBar::new("Exploitability", 95.0).color(SEVERITY_CRITICAL))
        .child(ScoreBar::new("Potential Impact", 98.0).color(SEVERITY_HIGH))
        .into_any_element()
}

/// 渲染 PoC 区域
fn render_poc_section(vuln: &VulnData) -> impl IntoElement {
    if !vuln.poc_available {
        return div().into_any_element();
    }

    let poc_code = r#"{"data":"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA..."}"#;

    CodeBlock::new(poc_code)
        .title("AI-Generated PoC")
        .http_request(HttpMethod::Post, "/api/v1/telemetry")
        .header("Content-Type", "application/json")
        .into_any_element()
}

/// 渲染 MITRE ATT&CK 技术
fn render_mitre_techniques(vuln: &VulnData) -> impl IntoElement {
    if vuln.attack_techniques.is_empty() {
        return div().into_any_element();
    }

    v_flex()
        .gap(SPACING_SM)
        // 标题
        .child(
            div()
                .text_sm()
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(rgb(TEXT_SECONDARY))
                .child("MITRE ATT&CK Techniques"),
        )
        // 技术标签
        .child(
            TechniqueTagGroup::new()
                .with_style(TechniqueTagStyle::Default)
                .tags(vuln.attack_techniques.iter().cloned())
        )
        .into_any_element()
}

/// 渲染 AI 建议
fn render_ai_recommendation(vuln: &VulnData) -> impl IntoElement {
    let recommendation = vuln
        .ai_analysis
        .as_ref()
        .and_then(|a| a.recommendations.first().cloned())
        .unwrap_or_else(|| "No recommendations available.".to_string());

    v_flex()
        .p(PADDING_LG)
        .gap(SPACING_SM)
        .rounded(BORDER_RADIUS)
        .bg(rgb(0xeff6ff))
        .border_1()
        .border_color(rgb(0xbfdbfe))
        // 标题
        .child(
            gpui_component::h_flex()
                .items_center()
                .gap(SPACING_SM)
                .child(div().text_color(rgb(0xf59e0b)).child("💡"))
                .child(
                    div()
                        .text_sm()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(ACCENT_BLUE))
                        .child("AI Recommendation"),
                ),
        )
        // 建议内容
        .child(
            div()
                .text_sm()
                .text_color(rgb(0x1e40af))
                .line_height(px(22.0))
                .child(recommendation),
        )
}

/// 渲染操作按钮
fn render_action_buttons() -> impl IntoElement {
    gpui_component::h_flex()
        .w_full()
        .gap(SPACING_MD)
        // Test in Traffic 按钮
        .child(
            Button::new("test-in-traffic")
                .outline()
                .large()
                .w(px(200.0))
                .label("Test in Traffic")
                .icon(IconName::ArrowRight),
        )
        // FUZZ Test 按钮
        .child(
            Button::new("fuzz-test")
                .outline()
                .large()
                .w(px(200.0))
                .label("FUZZ Test")
                .icon(IconName::TriangleAlert),
        )
}

/// 渲染空状态
fn render_empty_state() -> impl IntoElement {
    v_flex()
        .size_full()
        .bg(rgb(BG_CARD))
        .items_center()
        .justify_center()
        .child(
            div()
                .text_color(rgb(TEXT_MUTED))
                .child("Select a vulnerability to view details"),
        )
}
