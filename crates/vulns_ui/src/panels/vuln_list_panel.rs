//! Vulnerability List Panel
//!
//! 左侧漏洞列表面板，包含：
//! - PanelHeader: 标题 + 徽章（固定高度，与中间/右侧栏对齐）
//! - 分界线（与中间/右侧栏对齐）
//! - FilterToolbar: 标签页 + 搜索框（在分界线下方）
//! - 可折叠的分组列表

use crate::components::*;
use crate::state::{ListViewType, VulnGroup, VulnState};
use gpui::*;
use gpui_component::{IconName, Sizable, input::{Input, InputState}, scroll::ScrollableElement as _, v_flex};
use ui::theme::*;



/// 漏洞列表面板
pub struct VulnListPanel {
    state: Entity<VulnState>,
    search_input: Option<Entity<InputState>>,
}

impl VulnListPanel {
    pub fn new(state: Entity<VulnState>, _cx: &mut Context<Self>) -> Self {
        Self {
            state,
            search_input: None,
        }
    }

    /// 确保搜索输入已初始化
    fn ensure_search_input(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Entity<InputState> {
        if let Some(ref input) = self.search_input {
            input.clone()
        } else {
            let input = cx.new(|cx| InputState::new(window, cx).placeholder("Search vulnerabilities..."));
            self.search_input = Some(input.clone());
            input
        }
    }

    /// 获取分组颜色（Severity 视图）
    fn group_title_color(&self, name: &str) -> u32 {
        match name {
            "CRITICAL" => SEVERITY_CRITICAL,
            "HIGH" => SEVERITY_HIGH,
            "MEDIUM" => SEVERITY_MEDIUM,
            "LOW" => SEVERITY_LOW,
            _ => TEXT_MUTED,
        }
    }
}

impl Render for VulnListPanel {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let search_input = self.ensure_search_input(window, cx);
        
        let state = self.state.read(cx);
        let total_count = state.count();
        let selected_id = state.selected_id().map(|s| s.to_string());
        let view_type = state.view_type();
        let groups = state.grouped_vulns();

        v_flex()
            .size_full()
            .w(px(320.0))
            .bg(rgb(BG_CARD))
            // ===== 头部区域 - 固定 42px，无下边框 =====
            .child(
                PanelHeader::new("Vulnerabilities")
                    .badge(format!("{}", total_count), 0xfca5a5)
                    .show_border(false),
            )
            // ===== 筛选工具栏（标签页 + 搜索框）=====
            .child(
                v_flex()
                    .w_full()
                    .px(PADDING_LG)
                    .pt(PADDING_SM)
                    .pb(PADDING_MD)
                    .gap(SPACING_SM)
                    // 标签页（小尺寸）
                    .child(
                        FilterTabs::new(view_type.as_str())
                            .tab("severity", "Severity")
                            .tab("asset", "Asset")
                            .tab("mitre", "MITRE")
                            .on_change(cx.listener(|this, tab_id: &SharedString, _, cx| {
                                let new_view = match tab_id.as_str() {
                                    "severity" => ListViewType::Severity,
                                    "asset" => ListViewType::Asset,
                                    _ => ListViewType::Mitre,
                                };
                                this.state.update(cx, |state, cx| {
                                    state.set_view_type(new_view, cx);
                                });
                            })),
                    )
                    // 搜索框（带搜索图标）
                    .child(
                        Input::new(&search_input)
                            .prefix(IconName::Search)
                            .small()
                    ),
            )
            // ===== 分界线 - 与中间/右侧栏对齐 =====
            .child(div().w_full().h(px(1.0)).bg(rgb(BORDER_COLOR)))
            // ===== 漏洞列表区域 =====
            .child(
                v_flex()
                    .flex_1()
                    .overflow_y_scrollbar()
                    .px(PADDING_LG)
                    .gap(SPACING_MD)
                    .children(groups.into_iter().map(|group: VulnGroup| {
                        let title_color = self.group_title_color(&group.name);
                        let is_expanded = state.is_group_expanded(&group.name);
                        let group_name = group.name.clone();

                        let mut group_el = CollapsibleGroup::new(&group.name, group.vulns.len())
                            .title_color(title_color)
                            .expanded(is_expanded)
                            .on_toggle(cx.listener(move |this, _, _, cx| {
                                this.state.update(cx, |state, cx| {
                                    state.toggle_group(&group_name, cx);
                                });
                            }));

                        for vuln in group.vulns {
                            let vuln_id = vuln.id.clone();
                            let is_selected = selected_id
                                .as_ref()
                                .map(|id| id == &vuln.id || vuln.cve.as_ref() == Some(id))
                                .unwrap_or(false);

                            group_el = group_el.child(
                                VulnCard::new(vuln)
                                    .selected(is_selected)
                                    .on_click(cx.listener(move |this, _, _, cx| {
                                        this.state.update(cx, |state, cx| {
                                            state.select(&vuln_id, cx);
                                        });
                                    })),
                            );
                        }
                        group_el
                    })),
            )
    }
}
