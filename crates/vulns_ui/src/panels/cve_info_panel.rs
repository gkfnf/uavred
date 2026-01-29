//! CVE Info Panel
//!
//! 右侧 CVE 信息面板，包含：
//! - 统一高度的 PanelHeader
//! - CVSS 评分卡片
//! - 检测时间卡片
//! - 资产卡片
//! - 快速操作

use crate::components::{InfoCard, PanelHeader};
use crate::state::VulnState;
use gpui::*;
use gpui_component::{
    button::Button,
    v_flex,
    Sizable,
};
use ui::theme::*;

/// CVE 信息面板
pub struct CveInfoPanel {
    state: Entity<VulnState>,
}

impl CveInfoPanel {
    pub fn new(state: Entity<VulnState>) -> Self {
        Self { state }
    }
}

impl Render for CveInfoPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let vuln = self.state.read(cx).selected().cloned();

        if let Some(vuln) = vuln {
            render_cve_info_content(&vuln).into_any_element()
        } else {
            render_empty_state().into_any_element()
        }
    }
}

/// 渲染 CVE 信息内容
fn render_cve_info_content(vuln: &data::VulnData) -> impl IntoElement {
    v_flex()
        .size_full()
        .w(px(280.0))
        .bg(rgb(BG_CARD))
        .border_l_1()
        .border_color(rgb(BORDER_COLOR))
        // 头部 - 统一 42px 高度，与左侧/中间栏对齐
        .child(PanelHeader::new("CVE Database").show_border(true))
        // 内容区域
        .child(
            v_flex()
                .p(PADDING_LG)
                .gap(SPACING_LG)
                // CVSS Score 卡片
                .child(render_cvss_score_card(vuln))
                // Detection Time 卡片
                .child(render_detection_time_card(vuln))
                // Asset 卡片
                .child(render_asset_card(vuln))
                // 分隔线
                .child(div().w_full().h(px(1.0)).bg(rgb(BORDER_COLOR)))
                // Quick Actions
                .child(render_quick_actions()),
        )
}

/// 渲染 CVSS 评分卡片
fn render_cvss_score_card(vuln: &data::VulnData) -> impl IntoElement {
    let (score, color) = match &vuln.cvss {
        Some(cvss) => {
            let score = cvss.base_score;
            let color = if score >= 9.0 {
                SEVERITY_CRITICAL
            } else if score >= 7.0 {
                SEVERITY_HIGH
            } else if score >= 4.0 {
                SEVERITY_MEDIUM
            } else {
                SEVERITY_LOW
            };
            (format!("{:.1}", score), color)
        }
        None => ("N/A".to_string(), TEXT_MUTED),
    };

    InfoCard::new("CVSS Score", score)
        .value_color(color)
        .subtitle("v3.1 Base Score")
}

/// 渲染检测时间卡片
fn render_detection_time_card(vuln: &data::VulnData) -> impl IntoElement {
    InfoCard::new("Detection Time", &vuln.detection_time)
}

/// 渲染资产卡片
fn render_asset_card(vuln: &data::VulnData) -> impl IntoElement {
    InfoCard::new("Asset", &vuln.affected)
}

/// 渲染快速操作
fn render_quick_actions() -> impl IntoElement {
    v_flex()
        .gap(SPACING_MD)
        // 标题
        .child(
            div()
                .text_sm()
                .text_color(rgb(TEXT_SECONDARY))
                .child("Quick Actions"),
        )
        // 查看资产详情按钮
        .child(
            Button::new("view-asset-details")
                .outline()
                .small()
                .w_full()
                .label("View Asset Details"),
        )
}

/// 渲染空状态
fn render_empty_state() -> impl IntoElement {
    v_flex()
        .size_full()
        .w(px(280.0))
        .bg(rgb(BG_CARD))
        .border_l_1()
        .border_color(rgb(BORDER_COLOR))
        .items_center()
        .justify_center()
        .child(
            div()
                .text_color(rgb(TEXT_MUTED))
                .child("Select a vulnerability to view CVE details"),
        )
}
