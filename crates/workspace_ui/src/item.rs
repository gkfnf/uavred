use std::any::Any;
use std::any::TypeId;

pub trait ItemHandle: Any + Send + Sync {
    fn title(&self) -> String;
    fn icon(&self) -> String;
    fn type_id(&self) -> TypeId {
        TypeId::of::<Self>()
    }
    fn clone_handle(&self) -> Box<dyn ItemHandle>;
}

#[derive(Clone)]
pub struct TaskItem {
    pub task_id: usize,
    pub title: String,
    pub task_type: String,
    pub priority: String,
    pub status: String,
}

impl ItemHandle for TaskItem {
    fn title(&self) -> String {
        self.title.clone()
    }

    fn icon(&self) -> String {
        match self.task_type.as_str() {
            "SCAN" => "search".to_string(),
            "ANALYSIS" => "file-text".to_string(),
            "RECON" => "radar".to_string(),
            "EXPLOIT" => "target".to_string(),
            "PENTEST" => "shield".to_string(),
            _ => "check-square".to_string(),
        }
    }

    fn clone_handle(&self) -> Box<dyn ItemHandle> {
        Box::new(self.clone())
    }
}

#[derive(Clone)]
pub struct AssetItem {
    pub asset_id: String,
    pub name: String,
    pub asset_type: String,
}

impl ItemHandle for AssetItem {
    fn title(&self) -> String {
        self.name.clone()
    }

    fn icon(&self) -> String {
        match self.asset_type.as_str() {
            "drone" => "plane".to_string(),
            "controller" => "gamepad".to_string(),
            "firmware" => "file".to_string(),
            _ => "folder".to_string(),
        }
    }

    fn clone_handle(&self) -> Box<dyn ItemHandle> {
        Box::new(self.clone())
    }
}

#[derive(Clone)]
pub struct VulnItem {
    pub vuln_id: usize,
    pub title: String,
    pub severity: String,
    pub status: String,
}

impl ItemHandle for VulnItem {
    fn title(&self) -> String {
        self.title.clone()
    }

    fn icon(&self) -> String {
        match self.severity.as_str() {
            "critical" => "alert-circle".to_string(),
            "high" => "alert-triangle".to_string(),
            "medium" => "info".to_string(),
            _ => "check-circle".to_string(),
        }
    }

    fn clone_handle(&self) -> Box<dyn ItemHandle> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub enum AssetsEvent {
    AssetSelected(String),
    AssetsUpdated,
}

#[derive(Debug, Clone)]
pub enum VulnsEvent {
    FilterChanged(String),
    VulnSelected(usize),
}
