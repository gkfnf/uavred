#[derive(Debug, Clone)]
pub enum WorkspaceEvent {
    ViewChanged(String),
    TaskSelected(Option<usize>),
    TaskAdded(TaskData),
    TaskRemoved(usize),
    AssetSelected(String),
    VulnSelected(usize),
}

#[derive(Debug, Clone)]
pub enum DashboardEvent {
    ViewChanged(String),
    TaskSelected(Option<usize>),
    TaskAdded(TaskData),
    TaskRemoved(usize),
}

#[derive(Debug, Clone)]
pub enum SidebarEvent {
    Toggle,
    Opened,
    Closed,
}

#[derive(Debug, Clone)]
pub struct TaskData {
    pub id: usize,
    pub title: String,
    pub task_type: String,
    pub priority: String,
    pub status: String,
}
