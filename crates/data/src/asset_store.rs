// AssetStore - 使用 UavredDatabase 提供全局资产管理 + 数据库持久化

use gpui::{App, AppContext, Context, Entity, EventEmitter, Global};
use std::sync::{Arc, Mutex};

use crate::models::{Asset, AssetStatus, ZoneType, AssetNode, Severity, Connection, ScanProgress, ComplianceStandard, ComplianceStatus};
use crate::repository::Database;

/// AssetStore 事件
#[derive(Debug, Clone)]
pub enum AssetStoreEvent {
    AssetsUpdated,
    AssetAdded(Asset),
    AssetUpdated(Asset),
    AssetDeleted(i64),
    ConnectionsUpdated,
}

/// 全局 AssetStore 包装
struct GlobalAssetStore(Entity<AssetStore>);

impl Global for GlobalAssetStore {}

/// AssetStore Entity - 管理资产状态
pub struct AssetStore {
    db: Arc<Mutex<Database>>,
    assets: Vec<Asset>,
}

impl EventEmitter<AssetStoreEvent> for AssetStore {}

impl AssetStore {
    pub fn new() -> anyhow::Result<Self> {
        let db = Database::open_local()?;
        
        Ok(Self {
            db: Arc::new(Mutex::new(db)),
            assets: Vec::new(),
        })
    }

    /// 从数据库加载所有资产
    pub fn load_all_assets(&mut self, cx: &mut Context<Self>) -> anyhow::Result<()> {
        let db = self.db.lock().unwrap();
        let assets = db.assets().list_all()?;
        drop(db);
        
        self.assets = assets;
        cx.emit(AssetStoreEvent::AssetsUpdated);
        cx.notify();
        
        Ok(())
    }

    /// 获取所有资产
    pub fn get_all_assets(&self) -> Vec<Asset> {
        self.assets.clone()
    }

    /// 获取指定区域的资产
    pub fn get_assets_by_zone(&self, zone_id: &str) -> Vec<Asset> {
        self.assets
            .iter()
            .filter(|a| a.zone_id.as_deref() == Some(zone_id))
            .cloned()
            .collect()
    }

    /// 获取单个资产
    pub fn get_asset(&self, id: i64) -> Option<Asset> {
        self.assets.iter().find(|a| a.id == id).cloned()
    }

    /// 获取完整资产节点（包含服务和连接）
    pub fn get_asset_node_by_id(&self, id: i64) -> Option<AssetNode> {
        // First check if we have it in cache
        if let Some(cached) = self.assets.iter().find(|a| a.id == id) {
            let mut asset = cached.clone();
            
            // Load services and connections from database
            if let Ok(services) = {
                let db = self.db.lock().unwrap();
                db.assets().get_services(id)
            } {
                asset.services = services;
            }
            
            if let Ok(connections) = {
                let db = self.db.lock().unwrap();
                db.assets().get_connections(id)
            } {
                asset.connections = connections;
            }
            
            return Some(Self::convert_asset_to_node(asset));
        }
        
        // Try to load from database
        let mut asset = {
            let db = self.db.lock().unwrap();
            db.assets().get_by_id(id).ok()??
        };
        
        // Load services and connections
        if let Ok(services) = {
            let db = self.db.lock().unwrap();
            db.assets().get_services(id)
        } {
            asset.services = services;
        }
        
        if let Ok(connections) = {
            let db = self.db.lock().unwrap();
            db.assets().get_connections(id)
        } {
            asset.connections = connections;
        }
        
        Some(Self::convert_asset_to_node(asset))
    }

    /// 添加资产
    pub fn add_asset(&mut self, mut asset: Asset, cx: &mut Context<Self>) -> anyhow::Result<i64> {
        let db = self.db.lock().unwrap();
        let id = db.assets().create(&asset)?;
        drop(db);
        
        asset.id = id;
        self.assets.push(asset.clone());
        
        cx.emit(AssetStoreEvent::AssetAdded(asset));
        cx.notify();
        
        Ok(id)
    }

    /// 更新资产
    pub fn update_asset(&mut self, asset: Asset, cx: &mut Context<Self>) -> anyhow::Result<()> {
        let db = self.db.lock().unwrap();
        db.assets().update(&asset)?;
        drop(db);
        
        // 更新内存中的资产
        if let Some(pos) = self.assets.iter().position(|a| a.id == asset.id) {
            self.assets[pos] = asset.clone();
        }
        
        cx.emit(AssetStoreEvent::AssetUpdated(asset));
        cx.notify();
        
        Ok(())
    }

    /// 删除资产
    pub fn delete_asset(&mut self, asset_id: i64, cx: &mut Context<Self>) -> anyhow::Result<()> {
        let db = self.db.lock().unwrap();
        db.assets().delete(asset_id)?;
        drop(db);
        
        self.assets.retain(|a| a.id != asset_id);
        
        cx.emit(AssetStoreEvent::AssetDeleted(asset_id));
        cx.notify();
        
        Ok(())
    }

    /// 更新资产状态
    pub fn update_asset_status(&mut self, asset_id: i64, status: AssetStatus, cx: &mut Context<Self>) -> anyhow::Result<()> {
        let db = self.db.lock().unwrap();
        db.assets().update_status(asset_id, status.clone())?;
        drop(db);
        
        if let Some(pos) = self.assets.iter().position(|a| a.id == asset_id) {
            self.assets[pos].status = status;
            let asset = self.assets[pos].clone();
            cx.emit(AssetStoreEvent::AssetUpdated(asset));
            cx.notify();
        }
        
        Ok(())
    }

    /// 更新资产区域
    pub fn update_asset_zone(&mut self, asset_id: i64, zone_id: Option<String>, cx: &mut Context<Self>) -> anyhow::Result<()> {
        let db = self.db.lock().unwrap();
        db.assets().update_zone(asset_id, zone_id.as_deref())?;
        drop(db);
        
        if let Some(pos) = self.assets.iter().position(|a| a.id == asset_id) {
            self.assets[pos].zone_id = zone_id;
            let asset = self.assets[pos].clone();
            cx.emit(AssetStoreEvent::AssetUpdated(asset));
            cx.notify();
        }
        
        Ok(())
    }

    /// 搜索资产
    pub fn search_assets(&self, query: &str) -> Vec<Asset> {
        if query.is_empty() {
            return self.get_all_assets();
        }
        
        let query_lower = query.to_lowercase();
        self.assets
            .iter()
            .filter(|a| {
                a.name.to_lowercase().contains(&query_lower) ||
                a.ip_address.as_ref().map(|ip| ip.to_lowercase().contains(&query_lower)).unwrap_or(false) ||
                a.asset_type.to_lowercase().contains(&query_lower)
            })
            .cloned()
            .collect()
    }

    /// 获取资产统计
    pub fn get_stats(&self) -> anyhow::Result<AssetStats> {
        let total = self.assets.len() as i64;
        
        let mut by_zone: std::collections::HashMap<String, i64> = std::collections::HashMap::new();
        let mut by_status: std::collections::HashMap<String, i64> = std::collections::HashMap::new();
        let mut by_type: std::collections::HashMap<String, i64> = std::collections::HashMap::new();
        let mut total_vulns: i64 = 0;
        let mut total_risk: i32 = 0;
        let mut high_risk_count: i64 = 0;

        for asset in &self.assets {
            if let Some(ref zone_id) = asset.zone_id {
                *by_zone.entry(zone_id.clone()).or_insert(0) += 1;
            }
            *by_status.entry(asset.status.as_str().to_string()).or_insert(0) += 1;
            *by_type.entry(asset.asset_type.clone()).or_insert(0) += 1;
            total_vulns += asset.vuln_count as i64;
            total_risk += asset.risk_score;
            if asset.risk_score >= 70 {
                high_risk_count += 1;
            }
        }

        let avg_risk = if total > 0 { (total_risk as i64 / total) as i32 } else { 0 };

        Ok(AssetStats {
            total,
            by_zone,
            by_status,
            by_type,
            total_vulns,
            avg_risk,
            high_risk_count,
        })
    }

    // ============================================
    // 连接管理 (Topology)
    // ============================================

    /// 获取所有连接 (用于拓扑图)
    pub fn get_all_connections(&self) -> anyhow::Result<Vec<crate::models::AssetConnection>> {
        let db = self.db.lock().unwrap();
        db.assets().get_all_connections()
    }

    /// 添加连接
    pub fn add_connection(&mut self, connection: crate::models::AssetConnection, cx: &mut Context<Self>) -> anyhow::Result<i64> {
        let db = self.db.lock().unwrap();
        let id = db.assets().add_connection(&connection)?;
        drop(db);
        
        cx.emit(AssetStoreEvent::ConnectionsUpdated);
        cx.notify();
        
        Ok(id)
    }

    /// 获取资产的连接
    pub fn get_asset_connections(&self, asset_id: i64) -> anyhow::Result<Vec<crate::models::AssetConnection>> {
        let db = self.db.lock().unwrap();
        db.assets().get_connections(asset_id)
    }

    /// 计算网络可达性连接（基于accessible_networks）
    pub fn calculate_network_reachability(&self) -> anyhow::Result<Vec<(i64, i64, String)>> {
        let db = self.db.lock().unwrap();
        db.assets().calculate_network_reachability()
    }

    // ============================================
    // 转换为 AssetNode (用于 Topology UI)
    // ============================================

    /// 获取指定区域的 AssetNode (用于 Topology Canvas)
    pub fn get_asset_nodes_by_zone(&self, zone: ZoneType) -> Vec<AssetNode> {
        let zone_id = zone.as_str();
        self.get_assets_by_zone(zone_id)
            .into_iter()
            .map(|asset| Self::convert_asset_to_node(asset))
            .collect()
    }

    /// 将 Asset 转换为 AssetNode (用于 Topology UI)
    fn asset_to_node(&self, asset: Asset) -> AssetNode {
        Self::convert_asset_to_node(asset)
    }
    
    /// 静态方法：将 Asset 转换为 AssetNode
    fn convert_asset_to_node(asset: Asset) -> AssetNode {
        // 确定严重性 (基于风险评分)
        let severity = if asset.risk_score >= 70 {
            Severity::High
        } else if asset.risk_score >= 40 {
            Severity::Medium
        } else {
            Severity::Low
        };

        // 从 zone_id 解析 ZoneType
        let zone = asset.zone_id.as_deref()
            .map(ZoneType::from)
            .unwrap_or(ZoneType::Z1);

        // 转换服务为字符串列表
        let services: Vec<String> = asset.services.iter()
            .map(|s| format!("{}:{}", s.protocol, s.port))
            .collect();

        // 提取开放端口
        let open_ports: Vec<u16> = asset.services.iter()
            .filter(|s| s.port > 0 && s.port <= 65535)
            .map(|s| s.port as u16)
            .collect();

        // 转换连接 - 使用目标资产的ID作为字符串
        let connections: Vec<Connection> = asset.connections.iter()
            .map(|c| {
                // 确定此资产是源还是目标
                let is_source = c.source_asset_id == asset.id;
                let other_id = if is_source { c.target_asset_id } else { c.source_asset_id };
                
                Connection {
                    target_id: other_id.to_string(),
                    connection_type: c.connection_type.clone(),
                    protocol: c.protocol.clone(),
                    port: 0,
                }
            })
            .collect();

        // 构建合规标准
        let compliance_standards: Vec<ComplianceStandard> = asset.compliance_standards.iter()
            .map(|name| ComplianceStandard {
                name: name.clone(),
                status: ComplianceStatus::Compliant, // 默认状态
                last_audit: None,
            })
            .collect();

        AssetNode {
            id: asset.id.to_string(),
            name: asset.name,
            ip_address: asset.ip_address.unwrap_or_default(),
            mac_address: asset.mac_address,
            zone,
            severity,
            risk_score: asset.risk_score,
            vulnerabilities_count: asset.vuln_count,
            services,
            open_ports,
            credentials: Vec::new(), // 暂不暴露凭证
            owner: asset.owner_team,
            business_purpose: asset.business_purpose,
            department: None,
            scan_progress: ScanProgress {
                percentage: 100,
                last_scan: asset.last_scan_at,
                next_scan: None,
                scan_type: "Full".to_string(),
                scanning: false,
            },
            compliance_standards,
            connections,
            status: asset.status,
            last_seen: asset.updated_at.format("%Y-%m-%d %H:%M").to_string(),
            asset_type: asset.asset_type,
            firmware_version: if asset.firmware_version.is_empty() { None } else { Some(asset.firmware_version) },
            manufacturer: if asset.model.is_empty() { None } else { Some(asset.model) },
            location: None,
        }
    }
}

impl AssetStore {
    /// 获取全局 AssetStore Entity
    pub fn global(cx: &mut App) -> Entity<Self> {
        if cx.has_global::<GlobalAssetStore>() {
            return cx.global::<GlobalAssetStore>().0.clone();
        }

        panic!("AssetStore::global() called but no global AssetStore exists. Call init_asset_store() first.")
    }
}

/// 资产统计数据
#[derive(Debug, Clone)]
pub struct AssetStats {
    pub total: i64,
    pub by_zone: std::collections::HashMap<String, i64>,
    pub by_status: std::collections::HashMap<String, i64>,
    pub by_type: std::collections::HashMap<String, i64>,
    pub total_vulns: i64,
    pub avg_risk: i32,
    pub high_risk_count: i64,
}

/// 初始化全局 AssetStore
pub fn init_asset_store(cx: &mut App) {
    if !cx.has_global::<GlobalAssetStore>() {
        let asset_store = cx.new(|_cx| AssetStore::new().expect("Failed to initialize AssetStore"));
        cx.set_global(GlobalAssetStore(asset_store));
    }
}

/// 初始化并加载数据
pub fn init_and_load_asset_store(cx: &mut App) {
    init_asset_store(cx);
    
    // 触发数据加载
    let store = AssetStore::global(cx);
    store.update(cx, |store, cx| {
        if let Err(e) = store.load_all_assets(cx) {
            eprintln!("Failed to load assets: {}", e);
        }
    });
}
