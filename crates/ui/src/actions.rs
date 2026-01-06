// Actions 定义 - 参考 zed 的 Actions 系统

use gpui::Action;
use serde::Deserialize;
use workspace::{AppView, DashboardView};

/// 激活指定视图
#[derive(Clone, PartialEq, Debug, Action, Deserialize)]
#[action(namespace = workspace, no_json)]
pub struct ActivateView(pub AppView);

/// 选择任务
#[derive(Clone, PartialEq, Debug, Action, Deserialize)]
#[action(namespace = workspace, no_json)]
pub struct SelectTask(pub Option<usize>);

/// 切换 Dashboard 子视图
#[derive(Clone, PartialEq, Debug, Action, Deserialize)]
#[action(namespace = workspace, no_json)]
pub struct SetDashboardView(pub DashboardView);

/// 设置漏洞过滤器
#[derive(Clone, PartialEq, Debug, Action, Deserialize)]
#[action(namespace = workspace, no_json)]
pub struct SetVulnFilter(pub workspace::VulnFilter);
