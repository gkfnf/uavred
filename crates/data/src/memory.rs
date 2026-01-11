// 内存数据实现（当前使用）

use crate::models::{TaskData, TaskStatus, VulnData, VulnSeverity, AssetData};
use crate::repository::{TaskRepository, VulnRepository, AssetRepository};
use workspace::VulnFilter;

/// 内存任务仓库实现
pub struct MemoryTaskRepository {
    tasks: Vec<TaskData>,
}

impl MemoryTaskRepository {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
        }
    }

    pub fn with_initial_tasks(tasks: Vec<TaskData>) -> Self {
        Self { tasks }
    }
}

impl TaskRepository for MemoryTaskRepository {
    fn get_tasks(&self, status: TaskStatus) -> Vec<TaskData> {
        self.tasks
            .iter()
            .filter(|task| task.status == status)
            .cloned()
            .collect()
    }

    fn add_task(&mut self, task: TaskData) {
        self.tasks.push(task);
    }

    fn remove_task(&mut self, id: usize) {
        self.tasks.retain(|task| task.id != id);
    }

    fn update_task(&mut self, updated_task: TaskData) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == updated_task.id) {
            *task = updated_task;
        }
    }

    fn get_next_task_id(&self) -> usize {
        self.tasks
            .iter()
            .map(|t| t.id)
            .max()
            .unwrap_or(0) + 1
    }
}

/// 内存漏洞仓库实现
pub struct MemoryVulnRepository {
    vulns: Vec<VulnData>,
}

impl MemoryVulnRepository {
    pub fn new() -> Self {
        Self {
            vulns: Vec::new(),
        }
    }

    pub fn with_initial_vulns(vulns: Vec<VulnData>) -> Self {
        Self { vulns }
    }
}

impl VulnRepository for MemoryVulnRepository {
    fn get_vulns(&self, filter: VulnFilter) -> Vec<VulnData> {
        self.vulns
            .iter()
            .filter(|vuln| match filter {
                VulnFilter::All => true,
                VulnFilter::Critical => vuln.severity == VulnSeverity::Critical,
                VulnFilter::High => vuln.severity == VulnSeverity::High,
                VulnFilter::Medium => vuln.severity == VulnSeverity::Medium,
                VulnFilter::Low => vuln.severity == VulnSeverity::Low,
            })
            .cloned()
            .collect()
    }

    fn add_vuln(&mut self, vuln: VulnData) {
        self.vulns.push(vuln);
    }

    fn remove_vuln(&mut self, id: &str) {
        self.vulns.retain(|v| v.id != id);
    }

    fn update_vuln(&mut self, updated_vuln: VulnData) {
        if let Some(vuln) = self.vulns.iter_mut().find(|v| v.id == updated_vuln.id) {
            *vuln = updated_vuln;
        }
    }
}

/// 内存资产仓库实现
pub struct MemoryAssetRepository {
    assets: Vec<AssetData>,
}

impl MemoryAssetRepository {
    pub fn new() -> Self {
        Self {
            assets: Vec::new(),
        }
    }

    pub fn with_initial_assets(assets: Vec<AssetData>) -> Self {
        Self { assets }
    }
}

impl AssetRepository for MemoryAssetRepository {
    fn get_assets(&self) -> Vec<AssetData> {
        self.assets.clone()
    }

    fn add_asset(&mut self, asset: AssetData) {
        self.assets.push(asset);
    }

    fn remove_asset(&mut self, id: &str) {
        self.assets.retain(|a| a.id != id);
    }

    fn update_asset(&mut self, updated_asset: AssetData) {
        if let Some(asset) = self.assets.iter_mut().find(|a| a.id == updated_asset.id) {
            *asset = updated_asset;
        }
    }
}
