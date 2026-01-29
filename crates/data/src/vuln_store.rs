//! VulnStore - Vulnerability findings state management

use gpui::{App, AppContext, Context, Entity, EventEmitter, Global};
use std::sync::{Arc, Mutex};
use crate::models::{Finding, Vulnerability, FindingStatus, Severity};
use crate::repository::Database;

#[derive(Debug, Clone)]
pub enum VulnStoreEvent {
    FindingsUpdated,
    FindingSelected(i64),
    VulnReferenceLoaded(String),
}

struct GlobalVulnStore(Entity<VulnStore>);
impl Global for GlobalVulnStore {}

pub struct VulnStore {
    db: Arc<Mutex<Database>>,
    /// Security findings (from findings table with AI analysis)
    findings: Vec<Finding>,
    /// Vulnerability reference data (from vulnerabilities table)
    vuln_references: Vec<Vulnerability>,
    /// Currently selected finding ID
    selected_finding_id: Option<i64>,
    /// Cached vulnerability reference for selected finding
    selected_vuln_reference: Option<Vulnerability>,
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
            findings: Vec::new(),
            vuln_references: Vec::new(),
            selected_finding_id: None,
            selected_vuln_reference: None,
            is_loading: false,
            last_error: None,
        })
    }

    // ============================================
    // Findings Operations
    // ============================================

    /// Load all findings from database with full AI analysis fields
    pub fn load_findings(&mut self, cx: &mut Context<Self>) -> anyhow::Result<()> {
        self.is_loading = true;
        self.last_error = None;
        cx.notify();

        let db = self.db.lock().unwrap();
        match db.findings().list_all() {
            Ok(findings) => {
                self.findings = findings;
                self.is_loading = false;
                cx.emit(VulnStoreEvent::FindingsUpdated);
                cx.notify();
                Ok(())
            }
            Err(e) => {
                self.is_loading = false;
                self.last_error = Some(format!("Failed to load findings: {}", e));
                cx.notify();
                Err(e)
            }
        }
    }

    /// Load findings filtered by severity
    pub fn load_findings_by_severity(&mut self, severity: Severity, cx: &mut Context<Self>) -> anyhow::Result<()> {
        self.is_loading = true;
        cx.notify();

        let db = self.db.lock().unwrap();
        match db.findings().list_by_severity(severity) {
            Ok(findings) => {
                self.findings = findings;
                self.is_loading = false;
                cx.emit(VulnStoreEvent::FindingsUpdated);
                cx.notify();
                Ok(())
            }
            Err(e) => {
                self.is_loading = false;
                self.last_error = Some(format!("Failed to load findings: {}", e));
                cx.notify();
                Err(e)
            }
        }
    }

    /// Load findings filtered by status
    pub fn load_findings_by_status(&mut self, status: FindingStatus, cx: &mut Context<Self>) -> anyhow::Result<()> {
        self.is_loading = true;
        cx.notify();

        let db = self.db.lock().unwrap();
        match db.findings().list_by_status(status) {
            Ok(findings) => {
                self.findings = findings;
                self.is_loading = false;
                cx.emit(VulnStoreEvent::FindingsUpdated);
                cx.notify();
                Ok(())
            }
            Err(e) => {
                self.is_loading = false;
                self.last_error = Some(format!("Failed to load findings: {}", e));
                cx.notify();
                Err(e)
            }
        }
    }

    /// Load findings for a specific asset
    pub fn load_findings_by_asset(&mut self, asset_id: i64, cx: &mut Context<Self>) -> anyhow::Result<()> {
        let db = self.db.lock().unwrap();
        match db.findings().list_by_asset(asset_id) {
            Ok(findings) => {
                self.findings = findings;
                cx.emit(VulnStoreEvent::FindingsUpdated);
                cx.notify();
                Ok(())
            }
            Err(e) => {
                self.last_error = Some(format!("Failed to load findings: {}", e));
                cx.notify();
                Err(e)
            }
        }
    }

    /// Add a new finding
    pub fn add_finding(&mut self, finding: Finding, cx: &mut Context<Self>) -> anyhow::Result<i64> {
        let db = self.db.lock().unwrap();
        match db.findings().create(&finding) {
            Ok(id) => {
                drop(db);
                // Reload to get the new finding with all fields
                self.load_findings(cx)?;
                Ok(id)
            }
            Err(e) => {
                self.last_error = Some(format!("Failed to create finding: {}", e));
                cx.notify();
                Err(e)
            }
        }
    }

    /// Update a finding
    pub fn update_finding(&mut self, finding: Finding, cx: &mut Context<Self>) -> anyhow::Result<()> {
        let db = self.db.lock().unwrap();
        match db.findings().update(&finding) {
            Ok(()) => {
                drop(db);
                // Update in-memory if present
                if let Some(pos) = self.findings.iter().position(|f| f.id == finding.id) {
                    self.findings[pos] = finding;
                    cx.emit(VulnStoreEvent::FindingsUpdated);
                    cx.notify();
                }
                Ok(())
            }
            Err(e) => {
                self.last_error = Some(format!("Failed to update finding: {}", e));
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
                if let Some(finding) = self.findings.iter_mut().find(|f| f.id == finding_id) {
                    finding.status = status;
                    cx.emit(VulnStoreEvent::FindingsUpdated);
                    cx.notify();
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
                self.findings.retain(|f| f.id != finding_id);
                if self.selected_finding_id == Some(finding_id) {
                    self.selected_finding_id = None;
                    self.selected_vuln_reference = None;
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
    // Selection Operations
    // ============================================

    /// Select a finding and load its vulnerability reference
    pub fn select_finding(&mut self, finding_id: i64, cx: &mut Context<Self>) {
        self.selected_finding_id = Some(finding_id);

        // Load vulnerability reference if finding has a vuln_id
        let vuln_id_to_load = self.findings.iter()
            .find(|f| f.id == finding_id)
            .and_then(|f| f.vuln_id.clone());

        if let Some(vuln_id) = vuln_id_to_load {
            self.load_vuln_reference(&vuln_id, cx);
        } else {
            self.selected_vuln_reference = None;
        }

        cx.emit(VulnStoreEvent::FindingSelected(finding_id));
        cx.notify();
    }

    /// Load vulnerability reference data by ID
    fn load_vuln_reference(&mut self, vuln_id: &str, cx: &mut Context<Self>) {
        let db = self.db.lock().unwrap();
        match db.vulnerabilities().get_by_id(vuln_id) {
            Ok(vuln) => {
                let has_vuln = vuln.is_some();
                self.selected_vuln_reference = vuln;
                if has_vuln {
                    cx.emit(VulnStoreEvent::VulnReferenceLoaded(vuln_id.to_string()));
                }
                cx.notify();
            }
            Err(e) => {
                tracing::error!("Failed to load vulnerability reference: {}", e);
            }
        }
    }

    // ============================================
    // Vulnerability Reference Operations
    // ============================================

    /// Load all vulnerability reference data
    pub fn load_vuln_references(&mut self, cx: &mut Context<Self>) -> anyhow::Result<()> {
        let db = self.db.lock().unwrap();
        match db.vulnerabilities().list_all() {
            Ok(vulns) => {
                self.vuln_references = vulns;
                cx.notify();
                Ok(())
            }
            Err(e) => {
                self.last_error = Some(format!("Failed to load vulnerabilities: {}", e));
                cx.notify();
                Err(e)
            }
        }
    }

    /// Get vulnerability reference by CVE ID
    pub fn get_vuln_by_cve(&self, cve_id: &str) -> Option<&Vulnerability> {
        self.vuln_references.iter().find(|v| v.cve_id == cve_id)
    }

    /// Get vulnerability reference by internal ID
    pub fn get_vuln_by_id(&self, id: &str) -> Option<&Vulnerability> {
        self.vuln_references.iter().find(|v| v.id == id)
    }

    // ============================================
    // Statistics
    // ============================================

    /// Get findings statistics
    pub fn get_stats(&self) -> anyhow::Result<crate::repository::FindingStats> {
        let db = self.db.lock().unwrap();
        db.findings().get_stats()
    }

    // ============================================
    // Getters
    // ============================================

    pub fn findings(&self) -> &[Finding] { &self.findings }

    pub fn findings_by_severity(&self, severity: Severity) -> Vec<&Finding> {
        self.findings.iter().filter(|f| f.severity == severity).collect()
    }

    pub fn selected_finding(&self) -> Option<&Finding> {
        self.selected_finding_id.and_then(|id| self.findings.iter().find(|f| f.id == id))
    }

    pub fn selected_finding_id(&self) -> Option<i64> {
        self.selected_finding_id
    }

    pub fn selected_vuln_reference(&self) -> Option<&Vulnerability> {
        self.selected_vuln_reference.as_ref()
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

    /// Get count of findings with AI analysis
    pub fn ai_analyzed_count(&self) -> usize {
        self.findings.iter().filter(|f| f.ai_confidence.is_some()).count()
    }

    /// Get count of findings with PoC code
    pub fn poc_available_count(&self) -> usize {
        self.findings.iter().filter(|f| !f.poc_code.is_empty()).count()
    }

    /// Get findings grouped by severity (for UI display)
    pub fn findings_grouped_by_severity(&self) -> Vec<(Severity, Vec<&Finding>)> {
        use crate::models::Severity::*;
        let severities = vec![Critical, High, Medium, Low];

        severities.into_iter()
            .map(|sev| {
                let findings: Vec<&Finding> = self.findings.iter()
                    .filter(|f| f.severity == sev)
                    .collect();
                (sev, findings)
            })
            .filter(|(_, findings)| !findings.is_empty())
            .collect()
    }
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
        if let Err(e) = store.load_findings(cx) {
            tracing::error!("Failed to load findings: {}", e);
        }
    });
}
