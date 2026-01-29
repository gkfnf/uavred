//! Vulnerability Card Component
//!
//! 漏洞列表中的卡片组件

use data::{VulnData, VulnSeverity};
use gpui::prelude::FluentBuilder as _;
use gpui::*;
use gpui_component::{h_flex, v_flex};
use ui::theme::*;

/// 漏洞卡片组件
#[derive(IntoElement)]
pub struct VulnCard {
    vuln: VulnData,
    is_selected: bool,
    on_click: Option<Box<dyn Fn(&MouseDownEvent, &mut Window, &mut App) + 'static>>,
}

impl VulnCard {
    /// 创建新的漏洞卡片
    pub fn new(vuln: VulnData) -> Self {
        Self {
            vuln,
            is_selected: false,
            on_click: None,
        }
    }

    /// 设置选中状态
    pub fn selected(mut self, selected: bool) -> Self {
        self.is_selected = selected;
        self
    }

    /// 设置点击回调
    pub fn on_click(
        mut self,
        handler: impl Fn(&MouseDownEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }

    /// 获取严重程度颜色
    fn severity_color(severity: &VulnSeverity) -> u32 {
        match severity {
            VulnSeverity::Critical => SEVERITY_CRITICAL,
            VulnSeverity::High => SEVERITY_HIGH,
            VulnSeverity::Medium => SEVERITY_MEDIUM,
            VulnSeverity::Low => SEVERITY_LOW,
            VulnSeverity::Info => TEXT_MUTED,
        }
    }
}

impl RenderOnce for VulnCard {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let severity_color = Self::severity_color(&self.vuln.severity);
        let bg_color = if self.is_selected { 0xf3e8ff } else { BG_CARD };
        let border_color = if self.is_selected { ACCENT_PURPLE } else { BORDER_COLOR };

        // 获取 AI 置信度
        let ai_confidence = self
            .vuln
            .ai_analysis
            .as_ref()
            .map(|a| format!("{:.0}%", a.confidence_score * 100.0))
            .unwrap_or_else(|| "N/A".to_string());

        let element = h_flex()
            .id(SharedString::from(format!("vuln-card-{}", self.vuln.id)))
            .w_full()
            .bg(rgb(bg_color))
            .rounded(BORDER_RADIUS)
            .border_1()
            .border_color(rgb(border_color))
            .cursor_pointer()
            // 左侧严重程度条
            .child(
                div()
                    .w(px(4.0))
                    .h_full()
                    .flex_none()
                    .rounded_l(BORDER_RADIUS)
                    .bg(rgb(severity_color)),
            )
            // 内容区域
            .child(
                v_flex()
                    .flex_1()
                    .p(PADDING_MD)
                    .gap(SPACING_XS)
                    // 标题
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(TEXT_PRIMARY))
                            .line_height(px(20.0))
                            .child(self.vuln.title.clone()),
                    )
                    // CVE ID
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(TEXT_SECONDARY))
                            .font_family("monospace")
                            .child(self.vuln.cve.clone().unwrap_or_else(|| self.vuln.id.clone())),
                    )
                    // AI 置信度和 PoC 标签
                    .child(
                        h_flex()
                            .items_center()
                            .gap(SPACING_SM)
                            .child(
                                h_flex()
                                    .items_center()
                                    .gap(px(2.0))
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(rgb(ACCENT_PURPLE))
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .child("AI"),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(rgb(ACCENT_PURPLE))
                                            .child(ai_confidence),
                                    ),
                            )
                            .child(div().text_xs().text_color(rgb(TEXT_MUTED)).child("·"))
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(if self.vuln.poc_available {
                                        rgb(ACCENT_BLUE)
                                    } else {
                                        rgb(TEXT_MUTED)
                                    })
                                    .child(if self.vuln.poc_available { "PoC" } else { "No PoC" }),
                            ),
                    ),
            );

        if let Some(on_click) = self.on_click {
            element.on_mouse_down(MouseButton::Left, on_click)
        } else {
            element
        }
    }
}

/// 可折叠的分组组件
#[derive(IntoElement)]
pub struct CollapsibleGroup {
    name: SharedString,
    count: usize,
    is_expanded: bool,
    title_color: u32,
    children: Vec<AnyElement>,
    on_toggle: Option<Box<dyn Fn(&MouseDownEvent, &mut Window, &mut App) + 'static>>,
}

impl CollapsibleGroup {
    pub fn new(name: impl Into<SharedString>, count: usize) -> Self {
        Self {
            name: name.into(),
            count,
            is_expanded: true,
            title_color: TEXT_SECONDARY,
            children: Vec::new(),
            on_toggle: None,
        }
    }

    pub fn expanded(mut self, expanded: bool) -> Self {
        self.is_expanded = expanded;
        self
    }

    pub fn title_color(mut self, color: u32) -> Self {
        self.title_color = color;
        self
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }

    pub fn on_toggle(
        mut self,
        handler: impl Fn(&MouseDownEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_toggle = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for CollapsibleGroup {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        v_flex()
            .w_full()
            .gap(SPACING_SM)
            // 分组标题
            .child(
                h_flex()
                    .items_center()
                    .justify_between()
                    .cursor_pointer()
                    .py(px(8.0))
                    .when_some(self.on_toggle, |this, on_toggle| {
                        this.on_mouse_down(MouseButton::Left, on_toggle)
                    })
                    // 左侧：展开图标 + 标题
                    .child(
                        h_flex()
                            .items_center()
                            .gap(SPACING_SM)
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(TEXT_SECONDARY))
                                    .child(if self.is_expanded { "▼" } else { "▶" }),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(rgb(self.title_color))
                                    .child(self.name.to_string().to_uppercase()),
                            ),
                    )
                    // 右侧：数量
                    .child(
                        div()
                            .px(PADDING_SM)
                            .py(px(2.0))
                            .rounded_full()
                            .bg(rgb(BG_SECONDARY))
                            .text_xs()
                            .text_color(rgb(TEXT_SECONDARY))
                            .child(format!("{}", self.count)),
                    ),
            )
            // 子元素（仅在展开时显示）
            .when(self.is_expanded, |this| {
                this.child(
                    v_flex()
                        .w_full()
                        .gap(SPACING_SM)
                        .children(self.children),
                )
            })
    }
}
