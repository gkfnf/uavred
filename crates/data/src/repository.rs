// 数据访问接口（trait）- 为数据库集成做准备

use crate::models::{TaskData, TaskStatus, VulnData, AssetData};
use workspace::VulnFilter;

/// 任务数据仓库接口
pub trait TaskRepository: Send + Sync {
    /// 获取指定状态的任务列表
    fn get_tasks(&self, status: TaskStatus) -> Vec<TaskData>;
    /// 添加任务
    fn add_task(&mut self, task: TaskData);
    /// 移除任务
    fn remove_task(&mut self, id: usize);
    /// 更新任务
    fn update_task(&mut self, task: TaskData);
    /// 获取下一个可用的任务 ID
    fn get_next_task_id(&self) -> usize;
}

/// 漏洞数据仓库接口
pub trait VulnRepository: Send + Sync {
    /// 获取过滤后的漏洞列表
    fn get_vulns(&self, filter: VulnFilter) -> Vec<VulnData>;
    /// 添加漏洞
    fn add_vuln(&mut self, vuln: VulnData);
    /// 移除漏洞
    fn remove_vuln(&mut self, id: &str);
    /// 更新漏洞
    fn update_vuln(&mut self, vuln: VulnData);
}

/// 资产数据仓库接口
pub trait AssetRepository: Send + Sync {
    /// 获取所有资产
    fn get_assets(&self) -> Vec<AssetData>;
    /// 添加资产
    fn add_asset(&mut self, asset: AssetData);
    /// 移除资产
    fn remove_asset(&mut self, id: &str);
    /// 更新资产
    fn update_asset(&mut self, asset: AssetData);
}
