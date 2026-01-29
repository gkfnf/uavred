//! Vulns UI - 漏洞管理面板
//!
//! 三栏式布局：
//! - 左侧：漏洞列表（按资产/严重程度/MITRE 分组，支持筛选）
//! - 中间：漏洞详情和 PoC（AI 分析、代码块、MITRE 技术等）
//! - 右侧：CVE 数据库（CVSS 评分、检测时间等）
//!
//! # 架构设计
//!
//! ## 分层结构
//! - `components/`: 可复用的 UI 组件（无状态/纯展示）
//! - `panels/`: 三个主面板（有状态/业务逻辑）
//! - `state.rs`: UI 状态管理（视图类型、选中状态、分组状态）
//!
//! ## 数据流
//! - 数据模型来自外部 `data` crate（VulnData 等）
//! - UI 状态由 `VulnState` 管理
//! - 通过 GPUI 的 Entity 和事件进行通信

mod components;
mod panels;
mod state;

pub use components::*;
pub use panels::*;
pub use state::*;

use gpui::*;
use gpui_component::h_flex;

/// Vulns 面板 - 顶层容器
///
/// 组装三个子面板，形成完整的三栏布局
pub struct VulnsPanel {
    /// 状态（保留引用以便扩展）
    _state: Entity<VulnState>,
    /// 子面板
    vuln_list: Entity<VulnListPanel>,
    vuln_detail: Entity<VulnDetailPanel>,
    cve_info: Entity<CveInfoPanel>,
    /// 订阅
    _subscriptions: Vec<Subscription>,
}

impl VulnsPanel {
    pub fn new(cx: &mut Context<Self>) -> Self {
        // 创建状态并加载初始数据
        let state = cx.new(|_| {
            VulnState::new().with_vulns(state::mock::sample_vulns())
        });

        // 创建子面板
        let vuln_list = cx.new(|cx| VulnListPanel::new(state.clone(), cx));
        let vuln_detail = cx.new(|_| VulnDetailPanel::new(state.clone()));
        let cve_info = cx.new(|_| CveInfoPanel::new(state.clone()));

        // 订阅状态变化
        let subscription = cx.subscribe(&state, |_this, _state, _event: &VulnSelectedEvent, cx| {
            cx.notify();
        });

        Self {
            _state: state,
            vuln_list,
            vuln_detail,
            cve_info,
            _subscriptions: vec![subscription],
        }
    }
}

impl Render for VulnsPanel {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .size_full()
            .bg(rgb(ui::theme::BG_PRIMARY))
            // 左侧：漏洞列表
            .child(self.vuln_list.clone())
            // 中间：详情和 PoC
            .child(
                div()
                    .flex_1()
                    .h_full()
                    .overflow_hidden()
                    .child(self.vuln_detail.clone()),
            )
            // 右侧：CVE 数据库
            .child(self.cve_info.clone())
    }
}

/// 创建漏洞面板
pub fn vulns_panel(cx: &mut App) -> Entity<VulnsPanel> {
    cx.new(|cx| VulnsPanel::new(cx))
}
