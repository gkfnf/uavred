// 事件定义 - 参考 zed 的 Event 系统

use workspace::{AppView, DashboardView, TaskData};

/// Workspace 级别的事件
#[derive(Debug, Clone)]
pub enum WorkspaceEvent {
    /// 视图切换
    ViewChanged(AppView),
    /// 任务被选中
    TaskSelected(Option<usize>),
    /// 任务被添加
    TaskAdded(TaskData),
    /// 任务被移除
    TaskRemoved(usize),
    /// 任务被更新
    TaskUpdated(TaskData),
}

/// Dashboard 面板的事件
#[derive(Debug, Clone)]
pub enum DashboardEvent {
    /// Dashboard 子视图切换
    ViewChanged(DashboardView),
    /// 任务被选中
    TaskSelected(Option<usize>),
    /// 任务被添加
    TaskAdded(TaskData),
    /// 任务被移除
    TaskRemoved(usize),
}

/// Assets 面板的事件
#[derive(Debug, Clone)]
pub enum AssetsEvent {
    /// 资产被选中
    AssetSelected(String),
    /// 资产列表更新
    AssetsUpdated,
}

/// Vulns 面板的事件
#[derive(Debug, Clone)]
pub enum VulnsEvent {
    /// 过滤条件改变
    FilterChanged(workspace::VulnFilter),
    /// 漏洞被选中
    VulnSelected(usize),
}
