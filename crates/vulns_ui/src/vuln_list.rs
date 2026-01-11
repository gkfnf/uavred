// T1-4: Vulns 漏洞详情视图 - 漏洞列表组件
// 参考设计: Vulns.png 左侧

use data::{VulnData, VulnSeverity};
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

/// 过滤类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VulnFilterType {
    Severity,
    Asset,
    Mitre,
}

/// 漏洞列表组件
pub struct VulnList {
    pub filter_type: VulnFilterType,
    pub search_query: String,
    pub selected_vuln_id: Option<String>,
    pub vulnerabilities: Vec<VulnData>,
}

impl VulnList {
    pub fn new() -> Self {
        Self {
            filter_type: VulnFilterType::Severity,
            search_query: String::new(),
            selected_vuln_id: None,
            vulnerabilities: Vec::new(),
        }
    }

    pub fn set_filter_type(&mut self, filter_type: VulnFilterType) {
        self.filter_type = filter_type;
    }

    pub fn set_search_query(&mut self, query: String) {
        self.search_query = query;
    }

    pub fn set_selected_vuln(&mut self, vuln_id: Option<String>) {
        self.selected_vuln_id = vuln_id;
    }

    pub fn set_vulnerabilities(&mut self, vulns: Vec<VulnData>) {
        self.vulnerabilities = vulns;
    }

    /// 根据过滤类型和搜索查询过滤漏洞
    pub fn filtered_vulnerabilities(&self) -> Vec<&VulnData> {
        let mut filtered: Vec<&VulnData> = self
            .vulnerabilities
            .iter()
            .filter(|vuln| {
                // 搜索过滤
                if !self.search_query.is_empty() {
                    let query = self.search_query.to_lowercase();
                    vuln.title.to_lowercase().contains(&query)
                        || vuln.id.to_lowercase().contains(&query)
                        || vuln
                            .cve
                            .as_ref()
                            .map(|cve| cve.to_lowercase().contains(&query))
                            .unwrap_or(false)
                } else {
                    true
                }
            })
            .collect();

        // 按严重程度排序
        filtered.sort_by(|a, b| {
            let severity_order = |s: &VulnSeverity| match s {
                VulnSeverity::Critical => 0,
                VulnSeverity::High => 1,
                VulnSeverity::Medium => 2,
                VulnSeverity::Low => 3,
                VulnSeverity::Info => 4,
            };
            severity_order(&a.severity).cmp(&severity_order(&b.severity))
        });

        filtered
    }

    /// 按严重程度分组
    pub fn grouped_by_severity(&self) -> Vec<(VulnSeverity, Vec<&VulnData>)> {
        let filtered = self.filtered_vulnerabilities();
        let mut groups: std::collections::HashMap<VulnSeverity, Vec<&VulnData>> =
            std::collections::HashMap::new();

        for vuln in filtered {
            groups.entry(vuln.severity.clone()).or_default().push(vuln);
        }

        // 按严重程度顺序返回
        vec![
            (VulnSeverity::Critical, groups.remove(&VulnSeverity::Critical).unwrap_or_default()),
            (VulnSeverity::High, groups.remove(&VulnSeverity::High).unwrap_or_default()),
            (VulnSeverity::Medium, groups.remove(&VulnSeverity::Medium).unwrap_or_default()),
            (VulnSeverity::Low, groups.remove(&VulnSeverity::Low).unwrap_or_default()),
            (VulnSeverity::Info, groups.remove(&VulnSeverity::Info).unwrap_or_default()),
        ]
        .into_iter()
        .filter(|(_, vulns)| !vulns.is_empty())
        .collect()
    }
}

/// 渲染漏洞列表组件
pub fn render_vuln_list<T: 'static>(
    vuln_list: &VulnList,
    cx: &mut Context<T>,
    on_filter_change: impl Fn(&mut T, &mut Context<T>, VulnFilterType) + 'static,
    on_search_change: impl Fn(&mut T, &mut Context<T>, String) + 'static,
    on_vuln_select: impl Fn(&mut T, &mut Context<T>, String) + 'static,
) -> impl IntoElement {
    v_flex()
        .size_full()
        .bg(rgb(BG_CARD))
        .border_r(px(1.0))
        .border_color(rgb(BORDER_COLOR))
        .child(render_filter_tabs(vuln_list, cx, on_filter_change))
        .child(render_search_box(vuln_list, cx, on_search_change))
        .child(render_vuln_groups(vuln_list, cx, on_vuln_select))
}

/// 渲染过滤 Tab
fn render_filter_tabs<T: 'static>(
    vuln_list: &VulnList,
    cx: &mut Context<T>,
    on_filter_change: impl Fn(&mut T, &mut Context<T>, VulnFilterType) + 'static,
) -> impl IntoElement {
    h_flex()
        .w_full()
        .px(PADDING_MD)
        .pt(PADDING_MD)
        .gap(px(4.0))
        .border_b(px(1.0))
        .border_color(rgb(BORDER_COLOR))
        .pb(PADDING_SM)
        .children(vec![
            Button::new("filter-severity")
                .ghost()
                .small()
                .label("Severity")
                .selected(vuln_list.filter_type == VulnFilterType::Severity)
                .on_click(cx.listener(move |this, _, _, cx| {
                    on_filter_change(this, cx, VulnFilterType::Severity);
                })),
            Button::new("filter-asset")
                .ghost()
                .small()
                .label("Asset")
                .selected(vuln_list.filter_type == VulnFilterType::Asset)
                .on_click(cx.listener(move |this, _, _, cx| {
                    on_filter_change(this, cx, VulnFilterType::Asset);
                })),
            Button::new("filter-mitre")
                .ghost()
                .small()
                .label("MITRE")
                .selected(vuln_list.filter_type == VulnFilterType::Mitre)
                .on_click(cx.listener(move |this, _, _, cx| {
                    on_filter_change(this, cx, VulnFilterType::Mitre);
                })),
        ])
}

/// 渲染搜索框
fn render_search_box<T: 'static>(
    vuln_list: &VulnList,
    cx: &mut Context<T>,
    on_search_change: impl Fn(&mut T, &mut Context<T>, String) + 'static,
) -> impl IntoElement {
    h_flex()
        .w_full()
        .px(PADDING_MD)
        .py(PADDING_SM)
        .border_b(px(1.0))
        .border_color(rgb(BORDER_COLOR))
        .child(
            div()
                .w_full()
                .h(px(32.0))
                .px(PADDING_SM)
                .bg(rgb(BG_SECONDARY))
                .rounded(BORDER_RADIUS)
                .border(px(1.0))
                .border_color(rgb(BORDER_COLOR))
                .items_center()
                .child(
                    Label::new(
                        if vuln_list.search_query.is_empty() {
                            "Search vulnerabilities...".to_string()
                        } else {
                            vuln_list.search_query.clone()
                        }
                    )
                    .text_sm()
                    .text_color(if vuln_list.search_query.is_empty() {
                        rgb(TEXT_MUTED)
                    } else {
                        rgb(TEXT_PRIMARY)
                    }),
                )
                .cursor_text()
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(move |this: &mut T, _, _, cx: &mut Context<T>| {
                        // 这里可以触发输入框焦点，实际实现中需要处理文本输入
                        // 暂时使用占位符实现
                    }),
                ),
        )
}

/// 渲染漏洞分组列表
fn render_vuln_groups<T: 'static>(
    vuln_list: &VulnList,
    cx: &mut Context<T>,
    on_vuln_select: impl Fn(&mut T, &mut Context<T>, String) + 'static,
) -> impl IntoElement {
    let groups = vuln_list.grouped_by_severity();

    v_flex()
        .flex_1()
        .overflow_y_auto()
        .children(groups.into_iter().map(|(severity, vulns)| {
            render_severity_group(severity, vulns, vuln_list.selected_vuln_id.as_deref(), cx, &on_vuln_select)
        }))
}

/// 渲染严重程度分组
fn render_severity_group<T: 'static>(
    severity: VulnSeverity,
    vulns: Vec<&VulnData>,
    selected_id: Option<&str>,
    cx: &mut Context<T>,
    on_vuln_select: &impl Fn(&mut T, &mut Context<T>, String) + 'static,
) -> impl IntoElement {
    let severity_name = match severity {
        VulnSeverity::Critical => "CRITICAL",
        VulnSeverity::High => "HIGH",
        VulnSeverity::Medium => "MEDIUM",
        VulnSeverity::Low => "LOW",
        VulnSeverity::Info => "INFO",
    };

    let severity_color = match severity {
        VulnSeverity::Critical => rgb(SEVERITY_CRITICAL),
        VulnSeverity::High => rgb(SEVERITY_HIGH),
        VulnSeverity::Medium => rgb(SEVERITY_MEDIUM),
        VulnSeverity::Low => rgb(SEVERITY_LOW),
        VulnSeverity::Info => rgb(TEXT_SECONDARY),
    };

    v_flex()
        .w_full()
        .gap(px(4.0))
        .child(
            h_flex()
                .w_full()
                .px(PADDING_MD)
                .py(PADDING_SM)
                .items_center()
                .gap(px(8.0))
                .child(
                    div()
                        .w(px(4.0))
                        .h(px(16.0))
                        .rounded(px(2.0))
                        .bg(severity_color),
                )
                .child(
                    Label::new(severity_name)
                        .text_sm()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(TEXT_PRIMARY)),
                )
                .child(
                    Label::new(format!("({})", vulns.len()))
                        .text_sm()
                        .text_color(rgb(TEXT_SECONDARY)),
                ),
        )
        .children(vulns.into_iter().map(|vuln| {
            render_vuln_card(vuln, selected_id == Some(vuln.id.as_str()), cx, on_vuln_select)
        }))
}

/// 渲染漏洞卡片
fn render_vuln_card<T: 'static>(
    vuln: &VulnData,
    is_selected: bool,
    cx: &mut Context<T>,
    on_vuln_select: &impl Fn(&mut T, &mut Context<T>, String) + 'static,
) -> impl IntoElement {
    let vuln_id = vuln.id.clone();
    let severity_color = match vuln.severity {
        VulnSeverity::Critical => rgb(SEVERITY_CRITICAL),
        VulnSeverity::High => rgb(SEVERITY_HIGH),
        VulnSeverity::Medium => rgb(SEVERITY_MEDIUM),
        VulnSeverity::Low => rgb(SEVERITY_LOW),
        VulnSeverity::Info => rgb(TEXT_SECONDARY),
    };

    let ai_confidence = vuln
        .ai_analysis
        .as_ref()
        .map(|ai| ai.confidence_score)
        .unwrap_or(0.0);

    let mut card = h_flex()
        .w_full()
        .px(PADDING_MD)
        .py(PADDING_SM)
        .gap(px(12.0))
        .items_start()
        .cursor_pointer()
        .bg(if is_selected {
            rgb(0xf3e8ff)
        } else {
            rgb(BG_CARD)
        })
        .hover(|style| style.bg(rgb(0xf9fafb)))
        .child(
            div()
                .w(px(4.0))
                .h_full()
                .rounded(px(2.0))
                .bg(severity_color),
        )
        .child(
            v_flex()
                .flex_1()
                .gap(px(4.0))
                .child(
                    h_flex()
                        .gap(px(8.0))
                        .items_center()
                        .child(
                            Label::new(
                                vuln.cve
                                    .as_ref()
                                    .map(|cve| cve.as_str())
                                    .unwrap_or(&vuln.id),
                            )
                            .text_sm()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(TEXT_PRIMARY)),
                        )
                        .child(
                            Label::new(format!("AI: {:.0}%", ai_confidence * 100.0))
                                .text_xs()
                                .text_color(rgb(ACCENT_PURPLE)),
                        ),
                )
                .child(
                    Label::new(&vuln.title)
                        .text_xs()
                        .text_color(rgb(TEXT_SECONDARY))
                        .line_clamp(2),
                )
                .child(
                    h_flex()
                        .gap(px(4.0))
                        .items_center()
                        .children(
                            if vuln.poc_available {
                                vec![Tag::new()
                                    .small()
                                    .bg(rgb(0xfef3c7))
                                    .text_color(rgb(0x92400e))
                                    .child(Label::new("PoC").text_xs())]
                            } else {
                                vec![]
                            },
                        ),
                ),
        );

    if is_selected {
        card = card.border_l(px(3.0)).border_color(rgb(ACCENT_PURPLE));
    }

    div()
        .id(("vuln-card", vuln.id.clone()))
        .w_full()
        .child(card)
        .on_mouse_down(
            MouseButton::Left,
            cx.listener(move |this: &mut T, _, _, cx: &mut Context<T>| {
                on_vuln_select(this, cx, vuln_id.clone());
            }),
        )
}
