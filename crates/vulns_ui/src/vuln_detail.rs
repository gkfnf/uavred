// T1-5: Vulns 漏洞详情视图 - 详情面板组件
// 参考设计: Vulns.png 中间

use data::{VulnData, VulnSeverity};
use gpui::*;
use gpui_component::{
    div,
    h_flex,
    label::Label,
    tag::Tag,
    v_flex, Sizable,
};
use ui::theme::*;

/// 渲染漏洞详情面板
pub fn render_vuln_detail(vuln: Option<&VulnData>) -> impl IntoElement {
    match vuln {
        Some(v) => render_detail_content(v),
        None => render_empty_state(),
    }
}

/// 渲染详情内容
fn render_detail_content(vuln: &VulnData) -> impl IntoElement {
    v_flex()
        .size_full()
        .bg(rgb(BG_CARD))
        .border_r(px(1.0))
        .border_color(rgb(BORDER_COLOR))
        .overflow_y_auto()
        .child(render_header(vuln))
        .child(render_description(vuln))
        .child(render_detection_location(vuln))
        .child(render_ai_analysis(vuln))
        .child(render_poc_code(vuln))
        .child(render_mitre_tags(vuln))
}

/// 渲染空状态
fn render_empty_state() -> impl IntoElement {
    v_flex()
        .size_full()
        .bg(rgb(BG_CARD))
        .border_r(px(1.0))
        .border_color(rgb(BORDER_COLOR))
        .items_center()
        .justify_center()
        .child(
            Label::new("Select a vulnerability to view details")
                .text_sm()
                .text_color(rgb(TEXT_SECONDARY)),
        )
}

/// 渲染头部（CVE/CWE 标签）
fn render_header(vuln: &VulnData) -> impl IntoElement {
    v_flex()
        .w_full()
        .px(PADDING_LG)
        .pt(PADDING_LG)
        .pb(PADDING_MD)
        .gap(px(12.0))
        .border_b(px(1.0))
        .border_color(rgb(BORDER_COLOR))
        .child(
            Label::new(&vuln.title)
                .text_lg()
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(rgb(TEXT_PRIMARY)),
        )
        .child(
            h_flex()
                .gap(px(8.0))
                .items_center()
                .children(
                    vec![
                        vuln.cve.as_ref().map(|cve| {
                            Tag::new()
                                .bg(rgb(0x3b82f6))
                                .text_color(rgb(0xffffff))
                                .child(Label::new(format!("CVE-{}", cve)).text_sm())
                        }),
                        vuln.cwe.as_ref().map(|cwe| {
                            Tag::new()
                                .bg(rgb(0x6366f1))
                                .text_color(rgb(0xffffff))
                                .child(Label::new(format!("CWE-{}", cwe)).text_sm())
                        }),
                    ]
                    .into_iter()
                    .flatten()
                    .collect(),
                ),
        )
}

/// 渲染漏洞描述
fn render_description(vuln: &VulnData) -> impl IntoElement {
    v_flex()
        .w_full()
        .px(PADDING_LG)
        .py(PADDING_MD)
        .gap(px(8.0))
        .border_b(px(1.0))
        .border_color(rgb(BORDER_COLOR))
        .child(
            Label::new("Description")
                .text_sm()
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(rgb(TEXT_PRIMARY)),
        )
        .child(
            Label::new(&vuln.description)
                .text_sm()
                .text_color(rgb(TEXT_SECONDARY))
                .line_height(px(20.0)),
        )
}

/// 渲染检测位置信息
fn render_detection_location(vuln: &VulnData) -> impl IntoElement {
    let location = &vuln.detection_location;
    let location_text = format!(
        "Component: {}{}{}",
        location.component,
        location
            .file_path
            .as_ref()
            .map(|path| format!("\nFile: {}", path))
            .unwrap_or_default(),
        location
            .line_number
            .map(|line| format!("\nLine: {}", line))
            .unwrap_or_default(),
    );

    v_flex()
        .w_full()
        .px(PADDING_LG)
        .py(PADDING_MD)
        .gap(px(8.0))
        .border_b(px(1.0))
        .border_color(rgb(BORDER_COLOR))
        .child(
            Label::new("Detection Location")
                .text_sm()
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(rgb(TEXT_PRIMARY)),
        )
        .child(
            div()
                .px(PADDING_SM)
                .py(PADDING_SM)
                .bg(rgb(BG_SECONDARY))
                .rounded(BORDER_RADIUS)
                .child(
                    Label::new(location_text)
                        .text_sm()
                        .font_family("monospace")
                        .text_color(rgb(TEXT_PRIMARY)),
                ),
        )
        .child(
            h_flex()
                .gap(px(8.0))
                .items_center()
                .child(
                    Label::new(format!("Affected: {}", vuln.affected))
                        .text_xs()
                        .text_color(rgb(TEXT_SECONDARY)),
                )
                .child(
                    Label::new(format!("Detection Time: {}", vuln.detection_time))
                        .text_xs()
                        .text_color(rgb(TEXT_SECONDARY)),
                ),
        )
}

/// 渲染 AI Security Analysis 进度条
fn render_ai_analysis(vuln: &VulnData) -> impl IntoElement {
    let ai_analysis = vuln.ai_analysis.as_ref();

    v_flex()
        .w_full()
        .px(PADDING_LG)
        .py(PADDING_MD)
        .gap(px(12.0))
        .border_b(px(1.0))
        .border_color(rgb(BORDER_COLOR))
        .child(
            Label::new("AI Security Analysis")
                .text_sm()
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(rgb(TEXT_PRIMARY)),
        )
        .child(
            v_flex()
                .gap(px(8.0))
                .children(
                    vec![
                        render_progress_bar(
                            "Confidence",
                            ai_analysis.map(|ai| ai.confidence_score).unwrap_or(0.0),
                            rgb(ACCENT_PURPLE),
                        ),
                        render_progress_bar(
                            "Exploitability",
                            ai_analysis
                                .and_then(|ai| vuln.cvss.as_ref().map(|cvss| cvss.exploitability.unwrap_or(0.0)))
                                .unwrap_or(0.0),
                            rgb(SEVERITY_HIGH),
                        ),
                        render_progress_bar(
                            "Impact",
                            ai_analysis
                                .and_then(|ai| vuln.cvss.as_ref().map(|cvss| cvss.impact.unwrap_or(0.0)))
                                .unwrap_or(0.0),
                            rgb(SEVERITY_CRITICAL),
                        ),
                    ],
                ),
        )
        .children(
            ai_analysis
                .map(|ai| {
                    vec![v_flex()
                        .gap(px(4.0))
                        .child(
                            Label::new("Analysis Reasoning:")
                                .text_xs()
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(rgb(TEXT_SECONDARY)),
                        )
                        .child(
                            Label::new(&ai.reasoning)
                                .text_xs()
                                .text_color(rgb(TEXT_SECONDARY))
                                .line_height(px(16.0)),
                        )
                        ]
                })
                .unwrap_or_default(),
        )
}

/// 渲染进度条
fn render_progress_bar(label: &str, value: f64, color: Hsla) -> impl IntoElement {
    let label_str = label.to_string();
    let percentage = (value * 100.0).min(100.0).max(0.0);

    v_flex()
        .gap(px(4.0))
        .child(
            h_flex()
                .w_full()
                .items_center()
                .justify_between()
                .child(
                    Label::new(label_str)
                        .text_xs()
                        .text_color(rgb(TEXT_SECONDARY)),
                )
                .child(
                    Label::new(format!("{:.0}%", percentage))
                        .text_xs()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(TEXT_PRIMARY)),
                ),
        )
        .child(
            div()
                .w_full()
                .h(px(8.0))
                .bg(rgb(BG_SECONDARY))
                .rounded(BORDER_RADIUS_SM)
                .overflow_hidden()
                .child(
                    h_flex()
                        .h_full()
                        .w_full()
                        .child(
                            div()
                                .h_full()
                                .w(DefiniteLength::Fraction((percentage / 100.0).max(0.0).min(1.0)))
                                .bg(color)
                                .rounded(BORDER_RADIUS_SM),
                        )
                        .child(div().flex_1()),
                ),
        )
}

/// 渲染 AI-Generated PoC 代码块
fn render_poc_code(vuln: &VulnData) -> impl IntoElement {
    if !vuln.poc_available {
        return div();
    }

    let poc_code = format!(
        r#"// AI-Generated Proof of Concept
// CVE: {}
// Severity: {:?}

fn exploit() {{
    // PoC code would be generated here
    println!("Exploiting vulnerability...");
}}"#,
        vuln.cve.as_ref().unwrap_or(&vuln.id),
        vuln.severity
    );

    v_flex()
        .w_full()
        .px(PADDING_LG)
        .py(PADDING_MD)
        .gap(px(8.0))
        .border_b(px(1.0))
        .border_color(rgb(BORDER_COLOR))
        .child(
            h_flex()
                .w_full()
                .items_center()
                .justify_between()
                .child(
                    Label::new("AI-Generated PoC")
                        .text_sm()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(TEXT_PRIMARY)),
                )
                .child(
                    Tag::new()
                        .small()
                        .bg(rgb(0xfef3c7))
                        .text_color(rgb(0x92400e))
                        .child(Label::new("PoC Available").text_xs()),
                ),
        )
        .child(
            div()
                .w_full()
                .px(PADDING_MD)
                .py(PADDING_MD)
                .bg(rgb(0x1f2937))
                .rounded(BORDER_RADIUS)
                .overflow_x_auto()
                .child(
                    Label::new(poc_code)
                        .text_xs()
                        .font_family("monospace")
                        .text_color(rgb(0xffffff))
                        .whitespace_pre(),
                ),
        )
}

/// 渲染 MITRE ATT&CK 标签
fn render_mitre_tags(vuln: &VulnData) -> impl IntoElement {
    if vuln.attack_tactics.is_empty()
        && vuln.attack_techniques.is_empty()
        && vuln.attack_subtechniques.is_empty()
    {
        return div();
    }

    let mut tags = Vec::new();

    // Tactics
    for tactic in &vuln.attack_tactics {
        tags.push(
            Tag::new()
                .bg(rgb(0x7c3aed))
                .text_color(rgb(0xffffff))
                .child(Label::new(format!("TA: {}", tactic)).text_xs()),
        );
    }

    // Techniques
    for technique in &vuln.attack_techniques {
        tags.push(
            Tag::new()
                .bg(rgb(0x6366f1))
                .text_color(rgb(0xffffff))
                .child(Label::new(format!("T{}", technique)).text_xs()),
        );
    }

    // Sub-techniques
    for subtech in &vuln.attack_subtechniques {
        tags.push(
            Tag::new()
                .bg(rgb(0x8b5cf6))
                .text_color(rgb(0xffffff))
                .child(Label::new(format!("T{}", subtech)).text_xs()),
        );
    }

    v_flex()
        .w_full()
        .px(PADDING_LG)
        .py(PADDING_MD)
        .gap(px(8.0))
        .child(
            Label::new("MITRE ATT&CK")
                .text_sm()
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(rgb(TEXT_PRIMARY)),
        )
        .child(
            h_flex()
                .w_full()
                .flex_wrap()
                .gap(px(6.0))
                .children(tags),
        )
}
