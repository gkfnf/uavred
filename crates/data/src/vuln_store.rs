//! VulnStore - Vulnerability Knowledge Base Management
//!
//! Architecture:
//! - Vulnerability = User-defined vulnerability definitions (knowledge base)
//! - Finding = AI Agent discoveries (instances linked to vulnerabilities)
//!
//! The VulnStore manages the vulnerability knowledge base as the primary entity,
//! with findings being associated discoveries made by AI agents.

use gpui::{App, AppContext, Context, Entity, EventEmitter, Global};
use std::sync::{Arc, Mutex};
use crate::models::{Finding, Vulnerability, FindingStatus, Severity};
use crate::repository::Database;

#[derive(Debug, Clone)]
pub enum VulnStoreEvent {
    /// Vulnerabilities list updated
    VulnerabilitiesUpdated,
    /// A vulnerability was selected
    VulnerabilitySelected(String),
    /// Findings for selected vulnerability updated
    FindingsUpdated,
    /// A finding was selected
    FindingSelected(i64),
}

struct GlobalVulnStore(Entity<VulnStore>);
impl Global for GlobalVulnStore {}

/// Grouping mode for the vulnerability list
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GroupBy {
    Severity,
    Asset,
    Mitre,
}

impl GroupBy {
    pub fn as_str(&self) -> &'static str {
        match self {
            GroupBy::Severity => "severity",
            GroupBy::Asset => "asset",
            GroupBy::Mitre => "mitre",
        }
    }
}

/// Vulnerability with its associated findings
#[derive(Debug, Clone)]
pub struct VulnerabilityWithFindings {
    pub vulnerability: Vulnerability,
    pub findings: Vec<Finding>,
    pub affected_assets: Vec<i64>,
}

impl VulnerabilityWithFindings {
    pub fn new(vulnerability: Vulnerability) -> Self {
        Self {
            vulnerability,
            findings: Vec::new(),
            affected_assets: Vec::new(),
        }
    }
    
    pub fn finding_count(&self) -> usize {
        self.findings.len()
    }
    
    pub fn confirmed_count(&self) -> usize {
        self.findings.iter()
            .filter(|f| f.status == FindingStatus::Confirmed)
            .count()
    }
    
    pub fn has_active_findings(&self) -> bool {
        self.findings.iter()
            .any(|f| matches!(f.status, FindingStatus::New | FindingStatus::Confirmed | FindingStatus::Validating))
    }
}

pub struct VulnStore {
    db: Arc<Mutex<Database>>,
    /// Vulnerability knowledge base (primary entity)
    vulnerabilities: Vec<VulnerabilityWithFindings>,
    /// Currently selected vulnerability ID
    selected_vuln_id: Option<String>,
    /// Currently selected finding ID (within the selected vulnerability)
    selected_finding_id: Option<i64>,
    /// Current grouping mode for the list view
    group_by: GroupBy,
    /// Search query for filtering vulnerabilities
    search_query: String,
    /// Set of collapsed group identifiers (e.g., "Critical", "T0806")
    collapsed_groups: std::collections::HashSet<String>,
    /// Loading state
    is_loading: bool,
    /// Last error message
    last_error: Option<String>,
}

impl EventEmitter<VulnStoreEvent> for VulnStore {}

impl VulnStore {
    pub fn new() -> anyhow::Result<Self> {
        let db = Database::open_local()?;
        Ok(Self {
            db: Arc::new(Mutex::new(db)),
            vulnerabilities: Vec::new(),
            search_query: String::new(),
            collapsed_groups: std::collections::HashSet::new(),
            selected_vuln_id: None,
            selected_finding_id: None,
            group_by: GroupBy::Severity,
            is_loading: false,
            last_error: None,
        })
    }

    // ============================================
    // Vulnerability Operations (Primary)
    // ============================================

    /// Load all vulnerabilities with their associated findings
    pub fn load_vulnerabilities(&mut self, cx: &mut Context<Self>) -> anyhow::Result<()> {
        self.is_loading = true;
        self.last_error = None;
        cx.notify();

        let db = self.db.lock().unwrap();
        
        // Load all vulnerabilities
        match db.vulnerabilities().list_all() {
            Ok(vulns) => {
                let mut vulns_with_findings = Vec::new();
                
                // For each vulnerability, load its findings
                for vuln in vulns {
                    let mut vw = VulnerabilityWithFindings::new(vuln);
                    
                    // Load findings that reference this vulnerability
                    match db.findings().list_by_vulnerability(&vw.vulnerability.id) {
                        Ok(findings) => {
                            // Collect affected assets
                            let mut asset_ids: std::collections::HashSet<i64> = std::collections::HashSet::new();
                            for finding in &findings {
                                asset_ids.insert(finding.asset_id);
                            }
                            vw.affected_assets = asset_ids.into_iter().collect();
                            vw.findings = findings;
                        }
                        Err(e) => {
                            tracing::warn!("Failed to load findings for vuln {}: {}", vw.vulnerability.id, e);
                        }
                    }
                    
                    vulns_with_findings.push(vw);
                }
                
                self.vulnerabilities = vulns_with_findings;
                self.is_loading = false;
                cx.emit(VulnStoreEvent::VulnerabilitiesUpdated);
                cx.notify();
                Ok(())
            }
            Err(e) => {
                self.is_loading = false;
                self.last_error = Some(format!("Failed to load vulnerabilities: {}", e));
                cx.notify();
                Err(e)
            }
        }
    }

    /// Add a new vulnerability to the knowledge base
    pub fn add_vulnerability(&mut self, vuln: Vulnerability, cx: &mut Context<Self>) -> anyhow::Result<()> {
        let db = self.db.lock().unwrap();
        match db.vulnerabilities().create(&vuln) {
            Ok(()) => {
                drop(db);
                self.load_vulnerabilities(cx)?;
                Ok(())
            }
            Err(e) => {
                self.last_error = Some(format!("Failed to create vulnerability: {}", e));
                cx.notify();
                Err(e)
            }
        }
    }

    /// Update a vulnerability
    pub fn update_vulnerability(&mut self, vuln: Vulnerability, cx: &mut Context<Self>) -> anyhow::Result<()> {
        let db = self.db.lock().unwrap();
        match db.vulnerabilities().update(&vuln) {
            Ok(()) => {
                drop(db);
                // Update in-memory
                if let Some(pos) = self.vulnerabilities.iter().position(|v| v.vulnerability.id == vuln.id) {
                    self.vulnerabilities[pos].vulnerability = vuln;
                    cx.emit(VulnStoreEvent::VulnerabilitiesUpdated);
                    cx.notify();
                }
                Ok(())
            }
            Err(e) => {
                self.last_error = Some(format!("Failed to update vulnerability: {}", e));
                cx.notify();
                Err(e)
            }
        }
    }

    /// Delete a vulnerability
    pub fn delete_vulnerability(&mut self, vuln_id: &str, cx: &mut Context<Self>) -> anyhow::Result<()> {
        let db = self.db.lock().unwrap();
        match db.vulnerabilities().delete(vuln_id) {
            Ok(()) => {
                drop(db);
                self.vulnerabilities.retain(|v| v.vulnerability.id != vuln_id);
                if self.selected_vuln_id.as_deref() == Some(vuln_id) {
                    self.selected_vuln_id = None;
                    self.selected_finding_id = None;
                }
                cx.emit(VulnStoreEvent::VulnerabilitiesUpdated);
                cx.notify();
                Ok(())
            }
            Err(e) => {
                self.last_error = Some(format!("Failed to delete vulnerability: {}", e));
                cx.notify();
                Err(e)
            }
        }
    }

    // ============================================
    // Selection Operations
    // ============================================

    /// Select a vulnerability and emit event
    pub fn select_vulnerability(&mut self, vuln_id: String, cx: &mut Context<Self>) {
        self.selected_vuln_id = Some(vuln_id.clone());
        self.selected_finding_id = None; // Reset finding selection
        cx.emit(VulnStoreEvent::VulnerabilitySelected(vuln_id));
        cx.notify();
    }

    /// Select a specific finding within the current vulnerability
    pub fn select_finding(&mut self, finding_id: i64, cx: &mut Context<Self>) {
        self.selected_finding_id = Some(finding_id);
        cx.emit(VulnStoreEvent::FindingSelected(finding_id));
        cx.notify();
    }

    /// Set grouping mode
    pub fn set_group_by(&mut self, group_by: GroupBy, cx: &mut Context<Self>) {
        self.group_by = group_by;
        cx.notify();
    }

    /// Set search query and filter vulnerabilities
    pub fn set_search_query(&mut self, query: String, cx: &mut Context<Self>) {
        self.search_query = query;
        cx.notify();
    }

    /// Toggle group collapse state
    pub fn toggle_group_collapsed(&mut self, group_id: String, cx: &mut Context<Self>) {
        if self.collapsed_groups.contains(&group_id) {
            self.collapsed_groups.remove(&group_id);
        } else {
            self.collapsed_groups.insert(group_id);
        }
        cx.notify();
    }

    /// Check if a group is collapsed
    pub fn is_group_collapsed(&self, group_id: &str) -> bool {
        self.collapsed_groups.contains(group_id)
    }

    /// Get filtered vulnerabilities based on search query
    pub fn filtered_vulnerabilities(&self) -> Vec<&VulnerabilityWithFindings> {
        if self.search_query.is_empty() {
            self.vulnerabilities.iter().collect()
        } else {
            let query = self.search_query.to_lowercase();
            self.vulnerabilities
                .iter()
                .filter(|v| {
                    v.vulnerability.name.to_lowercase().contains(&query)
                        || v.vulnerability.cve_id.to_lowercase().contains(&query)
                        || v.vulnerability.id.to_lowercase().contains(&query)
                        || v.vulnerability.description.to_lowercase().contains(&query)
                })
                .collect()
        }
    }

    // ============================================
    // Finding Operations (Linked to Vulnerabilities)
    // ============================================

    /// Add a finding linked to a vulnerability
    pub fn add_finding(&mut self, finding: Finding, cx: &mut Context<Self>) -> anyhow::Result<i64> {
        let db = self.db.lock().unwrap();
        match db.findings().create(&finding) {
            Ok(id) => {
                drop(db);
                // Reload to get updated associations
                self.load_vulnerabilities(cx)?;
                Ok(id)
            }
            Err(e) => {
                self.last_error = Some(format!("Failed to create finding: {}", e));
                cx.notify();
                Err(e)
            }
        }
    }

    /// Update finding status
    pub fn update_finding_status(&mut self, finding_id: i64, status: FindingStatus, cx: &mut Context<Self>) -> anyhow::Result<()> {
        let db = self.db.lock().unwrap();
        match db.findings().update_status(finding_id, status.clone()) {
            Ok(()) => {
                drop(db);
                // Update in-memory
                for vuln in &mut self.vulnerabilities {
                    if let Some(finding) = vuln.findings.iter_mut().find(|f| f.id == finding_id) {
                        finding.status = status;
                        cx.emit(VulnStoreEvent::FindingsUpdated);
                        cx.notify();
                        break;
                    }
                }
                Ok(())
            }
            Err(e) => {
                self.last_error = Some(format!("Failed to update finding status: {}", e));
                cx.notify();
                Err(e)
            }
        }
    }

    /// Delete a finding
    pub fn delete_finding(&mut self, finding_id: i64, cx: &mut Context<Self>) -> anyhow::Result<()> {
        let db = self.db.lock().unwrap();
        match db.findings().delete(finding_id) {
            Ok(()) => {
                drop(db);
                for vuln in &mut self.vulnerabilities {
                    vuln.findings.retain(|f| f.id != finding_id);
                }
                if self.selected_finding_id == Some(finding_id) {
                    self.selected_finding_id = None;
                }
                cx.emit(VulnStoreEvent::FindingsUpdated);
                cx.notify();
                Ok(())
            }
            Err(e) => {
                self.last_error = Some(format!("Failed to delete finding: {}", e));
                cx.notify();
                Err(e)
            }
        }
    }

    // ============================================
    // Grouped Views for UI
    // ============================================

    /// Get vulnerabilities grouped by severity
    pub fn grouped_by_severity(&self) -> Vec<(Severity, Vec<&VulnerabilityWithFindings>)> {
        use crate::models::Severity::*;
        let severities = vec![Critical, High, Medium, Low, Info];

        severities.into_iter()
            .map(|sev| {
                let vulns: Vec<&VulnerabilityWithFindings> = self.vulnerabilities.iter()
                    .filter(|v| v.vulnerability.severity == sev)
                    .collect();
                (sev, vulns)
            })
            .filter(|(_, vulns)| !vulns.is_empty())
            .collect()
    }

    /// Get vulnerabilities grouped by affected asset
    pub fn grouped_by_asset(&self) -> Vec<(i64, Vec<&VulnerabilityWithFindings>)> {
        use std::collections::BTreeMap;
        let mut groups: BTreeMap<i64, Vec<&VulnerabilityWithFindings>> = BTreeMap::new();
        
        for vuln in &self.vulnerabilities {
            for &asset_id in &vuln.affected_assets {
                groups.entry(asset_id)
                    .or_default()
                    .push(vuln);
            }
        }
        
        groups.into_iter().collect()
    }

    /// Get vulnerabilities grouped by MITRE technique
    pub fn grouped_by_mitre(&self) -> Vec<(String, Vec<&VulnerabilityWithFindings>)> {
        use std::collections::BTreeMap;
        let mut groups: BTreeMap<String, Vec<&VulnerabilityWithFindings>> = BTreeMap::new();
        
        for vuln in &self.vulnerabilities {
            // Get MITRE techniques from vulnerability or its findings
            let mut techniques = vuln.vulnerability.mitre_techniques.clone();
            
            // Also collect from findings
            for finding in &vuln.findings {
                for tech in &finding.mitre_techniques {
                    if !techniques.contains(tech) {
                        techniques.push(tech.clone());
                    }
                }
            }
            
            for tech in techniques {
                groups.entry(tech)
                    .or_default()
                    .push(vuln);
            }
        }
        
        groups.into_iter().collect()
    }

    // ============================================
    // Statistics
    // ============================================

    /// Get vulnerability statistics
    pub fn get_stats(&self) -> VulnStats {
        let total = self.vulnerabilities.len();
        let with_findings = self.vulnerabilities.iter().filter(|v| !v.findings.is_empty()).count();
        let total_findings: usize = self.vulnerabilities.iter().map(|v| v.findings.len()).sum();
        
        let mut by_severity = std::collections::HashMap::new();
        for vuln in &self.vulnerabilities {
            *by_severity.entry(vuln.vulnerability.severity.clone()).or_insert(0) += 1;
        }
        
        VulnStats {
            total_vulnerabilities: total,
            vulnerabilities_with_findings: with_findings,
            total_findings,
            by_severity,
        }
    }

    // ============================================
    // Getters
    // ============================================

    pub fn vulnerabilities(&self) -> &[VulnerabilityWithFindings] {
        &self.vulnerabilities
    }

    pub fn selected_vulnerability(&self) -> Option<&VulnerabilityWithFindings> {
        self.selected_vuln_id.as_ref()
            .and_then(|id| self.vulnerabilities.iter().find(|v| v.vulnerability.id == *id))
    }

    pub fn selected_vulnerability_id(&self) -> Option<&str> {
        self.selected_vuln_id.as_deref()
    }

    pub fn selected_finding(&self) -> Option<&Finding> {
        self.selected_finding_id.and_then(|id| {
            self.vulnerabilities.iter()
                .flat_map(|v| &v.findings)
                .find(|f| f.id == id)
        })
    }

    pub fn selected_finding_id(&self) -> Option<i64> {
        self.selected_finding_id
    }

    pub fn group_by(&self) -> GroupBy {
        self.group_by
    }

    pub fn search_query(&self) -> &str {
        &self.search_query
    }

    pub fn is_loading(&self) -> bool {
        self.is_loading
    }

    pub fn last_error(&self) -> Option<&str> {
        self.last_error.as_deref()
    }

    pub fn clear_error(&mut self, cx: &mut Context<Self>) {
        self.last_error = None;
        cx.notify();
    }
}

/// Statistics about vulnerabilities and findings
#[derive(Debug, Clone)]
pub struct VulnStats {
    pub total_vulnerabilities: usize,
    pub vulnerabilities_with_findings: usize,
    pub total_findings: usize,
    pub by_severity: std::collections::HashMap<Severity, usize>,
}

impl VulnStore {
    pub fn global(cx: &mut App) -> Entity<Self> {
        if cx.has_global::<GlobalVulnStore>() {
            return cx.global::<GlobalVulnStore>().0.clone();
        }
        panic!("VulnStore::global() called but no global VulnStore exists. Call init_vuln_store() first.")
    }
}

pub fn init_vuln_store(cx: &mut App) {
    if !cx.has_global::<GlobalVulnStore>() {
        let vuln_store = cx.new(|_cx| VulnStore::new().expect("Failed to initialize VulnStore"));
        cx.set_global(GlobalVulnStore(vuln_store));
    }
}

pub fn init_and_load_vuln_store(cx: &mut App) {
    init_vuln_store(cx);
    let store = VulnStore::global(cx);
    store.update(cx, |store, cx| {
        if let Err(e) = store.load_vulnerabilities(cx) {
            tracing::error!("Failed to load vulnerabilities: {}", e);
        }
    });
}
