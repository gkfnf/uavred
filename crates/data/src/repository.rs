// Minimal Repository - 只包含核心 Task 操作
// 其他 repository 可以按需添加

use anyhow::Result;
use sqlez::connection::Connection;
use std::path::Path;
use std::sync::{Arc, Mutex};

use crate::models::*;

/// Main database connection pool
pub struct Database {
    pub(crate) connection: Arc<Mutex<Connection>>,
}

impl Database {
    /// Open or create database at the given path
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let conn = Connection::open_file(path.as_ref().to_string_lossy().as_ref());
        
        conn.exec("PRAGMA foreign_keys = ON;")?()
            .map_err(|e| anyhow::anyhow!("Failed to enable foreign keys: {}", e))?;
        conn.exec("PRAGMA journal_mode = WAL;")?()
            .map_err(|e| anyhow::anyhow!("Failed to set WAL mode: {}", e))?;
        
        Ok(Self {
            connection: Arc::new(Mutex::new(conn)),
        })
    }
    
    pub fn default_path() -> Result<std::path::PathBuf> {
        if let Ok(db_path) = std::env::var("UAVRED_DB_PATH") {
            return Ok(db_path.into());
        }
        let local_db = std::path::PathBuf::from("database/uavred.db");
        if local_db.exists() {
            return Ok(local_db);
        }
        let data_dir = dirs::data_dir()
            .ok_or_else(|| anyhow::anyhow!("Cannot get data directory"))?
            .join("uavred");
        std::fs::create_dir_all(&data_dir)?;
        Ok(data_dir.join("uavred.db"))
    }
    
    pub fn local_path() -> std::path::PathBuf {
        std::path::PathBuf::from("database/uavred.db")
    }
    
    pub fn open_default() -> Result<Self> {
        let path = Self::default_path()?;
        Self::open(path)
    }
    
    pub fn open_local() -> Result<Self> {
        let path = Self::local_path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        Self::open(path)
    }
    
    pub fn initialize_schema(&self) -> Result<()> {
        let conn = self.connection.lock().unwrap();
        let schema = include_str!("../../../database/schema.sql");
        conn.exec(schema)?()
            .map_err(|e| anyhow::anyhow!("Failed to initialize schema: {}", e))?;
        Ok(())
    }
    
    pub fn tasks(&self) -> TaskRepository {
        TaskRepository::new(self.connection.clone())
    }

    pub fn findings(&self) -> FindingRepository {
        FindingRepository::new(self.connection.clone())
    }

    pub fn traffic(&self) -> TrafficRepository {
        TrafficRepository::new(self.connection.clone())
    }

    pub fn vulnerabilities(&self) -> VulnerabilityRepository {
        VulnerabilityRepository::new(self.connection.clone())
    }
}

pub struct TaskRepository {
    connection: Arc<Mutex<Connection>>,
}

impl TaskRepository {
    pub fn new(connection: Arc<Mutex<Connection>>) -> Self {
        Self { connection }
    }
    
    pub fn create(&self, task: &Task) -> Result<i64> {
        let conn = self.connection.lock().unwrap();
        let sql = r#"
            INSERT INTO tasks (title, description, mission_objective, task_type, priority, status, 
                              assignee, estimated_minutes, source, external_ref, metadata)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
        "#;
        
        conn.exec_bound::<(&str, &str, &str, &str, &str, &str, &str, Option<i64>, &str, &str, String)>(sql)?((
            &task.title,
            &task.description,
            &task.mission_objective,
            &task.task_type,
            task.priority.as_str(),
            task.status.as_str(),
            &task.assignee,
            task.estimated_minutes,
            &task.source,
            &task.external_ref,
            task.metadata.to_string(),
        ))?;
        
        let id: i64 = conn.select::<i64>("SELECT last_insert_rowid()")?()?
            .into_iter().next().unwrap_or(0);
        Ok(id)
    }
    
    pub fn get_by_id(&self, id: i64) -> Result<Option<Task>> {
        let conn = self.connection.lock().unwrap();
        let sql = r#"
            SELECT id, title, description, mission_objective, task_type, priority, status,
                   assignee, estimated_minutes, created_at, updated_at, started_at, 
                   completed_at, closed_at, close_reason, source, external_ref, metadata
            FROM tasks WHERE id = ?1
        "#;
        
        let rows = conn.select_bound::<i64, (i64, String, String, String, String, String, String, 
                                   String, Option<i64>, String, String, Option<String>, 
                                   Option<String>, Option<String>, String, String, String, String)>(sql)?(id)?;
        
        Ok(rows.into_iter().next().map(|row| self.row_to_task(row).ok()).flatten())
    }
    
    pub fn list_by_status(&self, status: TaskStatus) -> Result<Vec<Task>> {
        let conn = self.connection.lock().unwrap();
        let sql = r#"
            SELECT id, title, description, mission_objective, task_type, priority, status,
                   assignee, estimated_minutes, created_at, updated_at, started_at, 
                   completed_at, closed_at, close_reason, source, external_ref, metadata
            FROM tasks WHERE status = ?1
            ORDER BY priority DESC, created_at DESC
        "#;
        
        let rows = conn.select_bound::<&str, (i64, String, String, String, String, String, String, 
                                   String, Option<i64>, String, String, Option<String>, 
                                   Option<String>, Option<String>, String, String, String, String)>(sql)?(status.as_str())?;
        
        rows.into_iter().map(|row| self.row_to_task(row)).collect()
    }
    
    pub fn update(&self, task: &Task) -> Result<()> {
        let conn = self.connection.lock().unwrap();
        let sql = r#"
            UPDATE tasks SET
                title = ?1, description = ?2, mission_objective = ?3, task_type = ?4,
                priority = ?5, status = ?6, assignee = ?7, estimated_minutes = ?8,
                source = ?9, external_ref = ?10, metadata = ?11, updated_at = CURRENT_TIMESTAMP
            WHERE id = ?12
        "#;
        
        conn.exec_bound::<(&str, &str, &str, &str, &str, &str, &str, Option<i64>, &str, &str, String, i64)>(sql)?((
            &task.title, &task.description, &task.mission_objective, &task.task_type,
            task.priority.as_str(), task.status.as_str(), &task.assignee, task.estimated_minutes,
            &task.source, &task.external_ref, task.metadata.to_string(), task.id,
        ))?;
        Ok(())
    }
    
    pub fn delete(&self, id: i64) -> Result<()> {
        let conn = self.connection.lock().unwrap();
        conn.exec_bound::<i64>("DELETE FROM tasks WHERE id = ?1")?(id)?;
        Ok(())
    }
    
    pub fn update_status(&self, id: i64, status: &str) -> Result<()> {
        let conn = self.connection.lock().unwrap();
        let sql = "UPDATE tasks SET status = ?1, updated_at = CURRENT_TIMESTAMP WHERE id = ?2";
        conn.exec_bound::<(&str, i64)>(sql)?((status, id))?;
        Ok(())
    }
    
    fn row_to_task(&self, row: (i64, String, String, String, String, String, String, 
                               String, Option<i64>, String, String, Option<String>, 
                               Option<String>, Option<String>, String, String, String, String)) -> Result<Task> {
        let (id, title, description, mission_objective, task_type, priority_str, status_str,
             assignee, estimated_minutes, created_at_str, updated_at_str, started_at_str,
             completed_at_str, closed_at_str, close_reason, source, external_ref, metadata_str) = row;
        
        Ok(Task {
            id, title, description, mission_objective, task_type,
            priority: TaskPriority::from(priority_str.as_str()),
            status: TaskStatus::from(status_str.as_str()),
            assignee, estimated_minutes,
            created_at: parse_datetime(&created_at_str)?,
            updated_at: parse_datetime(&updated_at_str)?,
            started_at: started_at_str.and_then(|s| parse_datetime(&s).ok()),
            completed_at: completed_at_str.and_then(|s| parse_datetime(&s).ok()),
            closed_at: closed_at_str.and_then(|s| parse_datetime(&s).ok()),
            close_reason, source, external_ref,
            metadata: serde_json::from_str(&metadata_str).unwrap_or_default(),
            labels: Vec::new(), comments: Vec::new(), dependencies: Vec::new(),
        })
    }
}

pub struct FindingRepository {
    connection: Arc<Mutex<Connection>>,
}

impl FindingRepository {
    pub fn new(connection: Arc<Mutex<Connection>>) -> Self {
        Self { connection }
    }

    /// Create a new finding
    pub fn create(&self, finding: &Finding) -> Result<i64> {
        let conn = self.connection.lock().unwrap();
        let sql = r#"
            INSERT INTO findings (
                vuln_id, asset_id, service_id, task_id, title, description, evidence,
                severity, cvss_score, status, ai_confidence, ai_analysis, ai_recommendation,
                poc_code, poc_language, mitre_techniques, remediation_steps, remediation_eta,
                remediated_at, remediated_by, detected_at, detected_by
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22)
        "#;

        let mitre_json = serde_json::to_string(&finding.mitre_techniques).unwrap_or_default();
        let remediation_eta_str = finding.remediation_eta.map(|d| d.to_string());
        let remediated_at_str = finding.remediated_at.map(|d| d.to_rfc3339());
        let detected_at_str = finding.detected_at.to_rfc3339();

        conn.exec_bound::<(
            Option<&str>, i64, Option<i64>, Option<i64>, &str, &str, &str, &str, Option<f64>, &str,
            Option<i32>, &str, &str, &str, &str, &str, &str, Option<&str>, Option<&str>, &str, &str, &str
        )>(sql)?((
            finding.vuln_id.as_deref(),
            finding.asset_id,
            finding.service_id,
            finding.task_id,
            &finding.title,
            &finding.description,
            &finding.evidence,
            finding.severity.as_str(),
            finding.cvss_score,
            finding.status.as_str(),
            finding.ai_confidence,
            &finding.ai_analysis,
            &finding.ai_recommendation,
            &finding.poc_code,
            &finding.poc_language,
            &mitre_json,
            &finding.remediation_steps,
            remediation_eta_str.as_deref(),
            remediated_at_str.as_deref(),
            &finding.remediated_by,
            &detected_at_str,
            &finding.detected_by,
        ))?;

        let id: i64 = conn.select::<i64>("SELECT last_insert_rowid()")?()?
            .into_iter().next().unwrap_or(0);
        Ok(id)
    }

    /// Get finding by ID
    pub fn get_by_id(&self, id: i64) -> Result<Option<Finding>> {
        let conn = self.connection.lock().unwrap();
        let sql = r#"
            SELECT id, vuln_id, asset_id, service_id, task_id, title, description, evidence,
                   severity, cvss_score, status, ai_confidence, ai_analysis, ai_recommendation,
                   poc_code, poc_language, mitre_techniques, remediation_steps, remediation_eta,
                   remediated_at, remediated_by, detected_at, detected_by
            FROM findings WHERE id = ?1
        "#;

        let rows = conn.select_bound::<i64, (
            i64, Option<String>, i64, Option<i64>, Option<i64>, String, String, String,
            String, Option<f64>, String, Option<i32>, String, String,
            String, String, String, String, Option<String>,
            Option<String>, String, String, String
        )>(sql)?(id)?;

        Ok(rows.into_iter().next().map(|row| self.row_to_finding(row).ok()).flatten())
    }

    /// List all findings ordered by severity and detection time
    pub fn list_all(&self) -> Result<Vec<Finding>> {
        let conn = self.connection.lock().unwrap();
        let sql = r#"
            SELECT id, vuln_id, asset_id, service_id, task_id, title, description, evidence,
                   severity, cvss_score, status, ai_confidence, ai_analysis, ai_recommendation,
                   poc_code, poc_language, mitre_techniques, remediation_steps, remediation_eta,
                   remediated_at, remediated_by, detected_at, detected_by
            FROM findings
            ORDER BY CASE severity
                WHEN 'critical' THEN 1
                WHEN 'high' THEN 2
                WHEN 'medium' THEN 3
                WHEN 'low' THEN 4
                ELSE 5
            END, detected_at DESC
        "#;

        let rows: Vec<(
            i64, Option<String>, i64, Option<i64>, Option<i64>, String, String, String,
            String, Option<f64>, String, Option<i32>, String, String,
            String, String, String, String, Option<String>,
            Option<String>, String, String, String
        )> = conn.select(sql)?()?;

        rows.into_iter().map(|row| self.row_to_finding(row)).collect()
    }

    /// List findings by severity
    pub fn list_by_severity(&self, severity: Severity) -> Result<Vec<Finding>> {
        let conn = self.connection.lock().unwrap();
        let sql = r#"
            SELECT id, vuln_id, asset_id, service_id, task_id, title, description, evidence,
                   severity, cvss_score, status, ai_confidence, ai_analysis, ai_recommendation,
                   poc_code, poc_language, mitre_techniques, remediation_steps, remediation_eta,
                   remediated_at, remediated_by, detected_at, detected_by
            FROM findings WHERE severity = ?1
            ORDER BY detected_at DESC
        "#;

        let rows = conn.select_bound::<&str, (
            i64, Option<String>, i64, Option<i64>, Option<i64>, String, String, String,
            String, Option<f64>, String, Option<i32>, String, String,
            String, String, String, String, Option<String>,
            Option<String>, String, String, String
        )>(sql)?(severity.as_str())?;

        rows.into_iter().map(|row| self.row_to_finding(row)).collect()
    }

    /// List findings by status
    pub fn list_by_status(&self, status: FindingStatus) -> Result<Vec<Finding>> {
        let conn = self.connection.lock().unwrap();
        let sql = r#"
            SELECT id, vuln_id, asset_id, service_id, task_id, title, description, evidence,
                   severity, cvss_score, status, ai_confidence, ai_analysis, ai_recommendation,
                   poc_code, poc_language, mitre_techniques, remediation_steps, remediation_eta,
                   remediated_at, remediated_by, detected_at, detected_by
            FROM findings WHERE status = ?1
            ORDER BY CASE severity
                WHEN 'critical' THEN 1
                WHEN 'high' THEN 2
                WHEN 'medium' THEN 3
                WHEN 'low' THEN 4
                ELSE 5
            END, detected_at DESC
        "#;

        let rows = conn.select_bound::<&str, (
            i64, Option<String>, i64, Option<i64>, Option<i64>, String, String, String,
            String, Option<f64>, String, Option<i32>, String, String,
            String, String, String, String, Option<String>,
            Option<String>, String, String, String
        )>(sql)?(status.as_str())?;

        rows.into_iter().map(|row| self.row_to_finding(row)).collect()
    }

    /// List findings by asset
    pub fn list_by_asset(&self, asset_id: i64) -> Result<Vec<Finding>> {
        let conn = self.connection.lock().unwrap();
        let sql = r#"
            SELECT id, vuln_id, asset_id, service_id, task_id, title, description, evidence,
                   severity, cvss_score, status, ai_confidence, ai_analysis, ai_recommendation,
                   poc_code, poc_language, mitre_techniques, remediation_steps, remediation_eta,
                   remediated_at, remediated_by, detected_at, detected_by
            FROM findings WHERE asset_id = ?1
            ORDER BY CASE severity
                WHEN 'critical' THEN 1
                WHEN 'high' THEN 2
                WHEN 'medium' THEN 3
                WHEN 'low' THEN 4
                ELSE 5
            END
        "#;

        let rows = conn.select_bound::<i64, (
            i64, Option<String>, i64, Option<i64>, Option<i64>, String, String, String,
            String, Option<f64>, String, Option<i32>, String, String,
            String, String, String, String, Option<String>,
            Option<String>, String, String, String
        )>(sql)?(asset_id)?;

        rows.into_iter().map(|row| self.row_to_finding(row)).collect()
    }

    /// Update finding
    pub fn update(&self, finding: &Finding) -> Result<()> {
        let conn = self.connection.lock().unwrap();
        let sql = r#"
            UPDATE findings SET
                vuln_id = ?1, asset_id = ?2, service_id = ?3, task_id = ?4,
                title = ?5, description = ?6, evidence = ?7,
                severity = ?8, cvss_score = ?9, status = ?10,
                ai_confidence = ?11, ai_analysis = ?12, ai_recommendation = ?13,
                poc_code = ?14, poc_language = ?15, mitre_techniques = ?16,
                remediation_steps = ?17, remediation_eta = ?18,
                remediated_at = ?19, remediated_by = ?20
            WHERE id = ?21
        "#;

        let mitre_json = serde_json::to_string(&finding.mitre_techniques).unwrap_or_default();
        let remediation_eta_str = finding.remediation_eta.map(|d| d.to_string());
        let remediated_at_str = finding.remediated_at.map(|d| d.to_rfc3339());

        conn.exec_bound::<(
            Option<&str>, i64, Option<i64>, Option<i64>, &str, &str, &str, &str, Option<f64>, &str,
            Option<i32>, &str, &str, &str, &str, &str, &str, Option<&str>, Option<&str>, &str, i64
        )>(sql)?((
            finding.vuln_id.as_deref(),
            finding.asset_id,
            finding.service_id,
            finding.task_id,
            &finding.title,
            &finding.description,
            &finding.evidence,
            finding.severity.as_str(),
            finding.cvss_score,
            finding.status.as_str(),
            finding.ai_confidence,
            &finding.ai_analysis,
            &finding.ai_recommendation,
            &finding.poc_code,
            &finding.poc_language,
            &mitre_json,
            &finding.remediation_steps,
            remediation_eta_str.as_deref(),
            remediated_at_str.as_deref(),
            &finding.remediated_by,
            finding.id,
        ))?;
        Ok(())
    }

    /// Update finding status
    pub fn update_status(&self, id: i64, status: FindingStatus) -> Result<()> {
        let conn = self.connection.lock().unwrap();
        let sql = "UPDATE findings SET status = ?1 WHERE id = ?2";
        conn.exec_bound::<(&str, i64)>(sql)?((status.as_str(), id))?;
        Ok(())
    }

    /// Delete finding
    pub fn delete(&self, id: i64) -> Result<()> {
        let conn = self.connection.lock().unwrap();
        conn.exec_bound::<i64>("DELETE FROM findings WHERE id = ?1")?(id)?;
        Ok(())
    }

    /// Get findings statistics
    pub fn get_stats(&self) -> Result<FindingStats> {
        let conn = self.connection.lock().unwrap();

        let total: i64 = conn.select::<i64>("SELECT COUNT(*) FROM findings")?()?
            .into_iter().next().unwrap_or(0);

        let by_severity_sql = "SELECT severity, COUNT(*) FROM findings GROUP BY severity";
        let by_severity_rows: Vec<(String, i64)> = conn.select(by_severity_sql)?()?;

        let by_status_sql = "SELECT status, COUNT(*) FROM findings GROUP BY status";
        let by_status_rows: Vec<(String, i64)> = conn.select(by_status_sql)?()?;

        let ai_analyzed: i64 = conn.select::<i64>("SELECT COUNT(*) FROM findings WHERE ai_confidence IS NOT NULL")?()?
            .into_iter().next().unwrap_or(0);

        let with_poc: i64 = conn.select::<i64>("SELECT COUNT(*) FROM findings WHERE poc_code != ''")?()?
            .into_iter().next().unwrap_or(0);

        let mut critical_count = 0;
        let mut high_count = 0;
        let mut medium_count = 0;
        let mut low_count = 0;

        for (sev, count) in by_severity_rows {
            match sev.as_str() {
                "critical" => critical_count = count,
                "high" => high_count = count,
                "medium" => medium_count = count,
                "low" => low_count = count,
                _ => {}
            }
        }

        let mut new_count = 0;
        let mut confirmed_count = 0;
        let mut remediated_count = 0;

        for (status, count) in by_status_rows {
            match status.as_str() {
                "new" => new_count = count,
                "confirmed" => confirmed_count = count,
                "remediated" => remediated_count = count,
                _ => {}
            }
        }

        Ok(FindingStats {
            total,
            critical_count,
            high_count,
            medium_count,
            low_count,
            new_count,
            confirmed_count,
            remediated_count,
            ai_analyzed,
            with_poc,
        })
    }

    fn row_to_finding(&self, row: (
        i64, Option<String>, i64, Option<i64>, Option<i64>, String, String, String,
        String, Option<f64>, String, Option<i32>, String, String,
        String, String, String, String, Option<String>,
        Option<String>, String, String, String
    )) -> Result<Finding> {
        let (id, vuln_id, asset_id, service_id, task_id, title, description, evidence,
             severity_str, cvss_score, status_str, ai_confidence, ai_analysis, ai_recommendation,
             poc_code, poc_language, mitre_json, remediation_steps, remediation_eta_str,
             remediated_at_str, remediated_by, detected_at_str, detected_by) = row;

        let mitre_techniques: Vec<String> = serde_json::from_str(&mitre_json).unwrap_or_default();

        // Parse remediation_eta as NaiveDate
        let remediation_eta = remediation_eta_str.and_then(|s| {
            chrono::NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok()
        });

        // Parse remediated_at as DateTime
        let remediated_at = remediated_at_str.and_then(|s: String| {
            chrono::DateTime::parse_from_rfc3339(&s)
                .map(|dt| dt.with_timezone(&chrono::Utc))
                .ok()
        });

        // Parse detected_at as DateTime
        let detected_at = chrono::DateTime::parse_from_rfc3339(&detected_at_str)
            .map(|dt| dt.with_timezone(&chrono::Utc))
            .or_else(|_| {
                chrono::NaiveDateTime::parse_from_str(&detected_at_str, "%Y-%m-%d %H:%M:%S")
                    .map(|naive| chrono::DateTime::from_naive_utc_and_offset(naive, chrono::Utc))
            })
            .map_err(|e| anyhow::anyhow!("Failed to parse detected_at: {}", e))?;

        Ok(Finding {
            id,
            vuln_id,
            asset_id,
            service_id,
            task_id,
            title,
            description,
            evidence,
            severity: Severity::from(severity_str.as_str()),
            cvss_score,
            status: FindingStatus::from(status_str.as_str()),
            ai_confidence,
            ai_analysis,
            ai_recommendation,
            poc_code,
            poc_language,
            mitre_techniques,
            remediation_steps,
            remediation_eta,
            remediated_at,
            remediated_by,
            detected_at,
            detected_by,
        })
    }
}

/// Statistics for findings
#[derive(Debug, Clone)]
pub struct FindingStats {
    pub total: i64,
    pub critical_count: i64,
    pub high_count: i64,
    pub medium_count: i64,
    pub low_count: i64,
    pub new_count: i64,
    pub confirmed_count: i64,
    pub remediated_count: i64,
    pub ai_analyzed: i64,
    pub with_poc: i64,
}

pub struct TrafficRepository {
    connection: Arc<Mutex<Connection>>,
}

impl TrafficRepository {
    pub fn new(connection: Arc<Mutex<Connection>>) -> Self {
        Self { connection }
    }

    /// Create a new traffic entry
    pub fn create(&self, traffic: &Traffic) -> Result<i64> {
        let conn = self.connection.lock().unwrap();
        let sql = r#"
            INSERT INTO traffic (
                protocol, method, path, src_ip, src_port, dst_ip, dst_port,
                request_headers, request_body, response_headers, response_body, response_status,
                size_bytes, duration_ms, asset_id, is_anomaly, anomaly_type, anomaly_score, tags, captured_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20)
        "#;

        let tags_json = serde_json::to_string(&traffic.tags).unwrap_or_default();
        let captured_at_str = traffic.captured_at.to_rfc3339();

        conn.exec_bound::<(
            &str, Option<&str>, &str, &str, Option<i32>, &str, Option<i32>,
            &str, Option<&[u8]>, &str, Option<&[u8]>, Option<i32>,
            i64, i32, Option<i64>, i64, &str, f64, &str, &str
        )>(sql)?((
            &traffic.protocol,
            traffic.method.as_deref(),
            &traffic.path,
            &traffic.src_ip,
            traffic.src_port,
            &traffic.dst_ip,
            traffic.dst_port,
            &traffic.request_headers,
            traffic.request_body.as_deref(),
            &traffic.response_headers,
            traffic.response_body.as_deref(),
            traffic.response_status,
            traffic.size_bytes,
            traffic.duration_ms,
            traffic.asset_id,
            if traffic.is_anomaly { 1 } else { 0 },
            &traffic.anomaly_type,
            traffic.anomaly_score,
            &tags_json,
            &captured_at_str,
        ))?;

        let id: i64 = conn.select::<i64>("SELECT last_insert_rowid()")?()?
            .into_iter().next().unwrap_or(0);
        Ok(id)
    }

    /// Get traffic by ID
    pub fn get_by_id(&self, id: i64) -> Result<Option<Traffic>> {
        let conn = self.connection.lock().unwrap();
        let sql = r#"
            SELECT id, protocol, method, path, src_ip, src_port, dst_ip, dst_port,
                   request_headers, request_body, response_headers, response_body, response_status,
                   size_bytes, duration_ms, asset_id, is_anomaly, anomaly_type, anomaly_score, tags, captured_at
            FROM traffic WHERE id = ?1
        "#;

        let rows = conn.select_bound::<i64, (
            i64, String, Option<String>, String, String, Option<i32>, String, Option<i32>,
            String, Option<Vec<u8>>, String, Option<Vec<u8>>, Option<i32>,
            i64, i32, Option<i64>, i64, String, f64, String, String
        )>(sql)?(id)?;

        Ok(rows.into_iter().next().map(|row| self.row_to_traffic(row).ok()).flatten())
    }

    /// List all traffic entries ordered by capture time (newest first)
    pub fn list_all(&self, limit: Option<i64>) -> Result<Vec<Traffic>> {
        let conn = self.connection.lock().unwrap();
        let sql = format!(
            r#"SELECT id, protocol, method, path, src_ip, src_port, dst_ip, dst_port,
                   request_headers, request_body, response_headers, response_body, response_status,
                   size_bytes, duration_ms, asset_id, is_anomaly, anomaly_type, anomaly_score, tags, captured_at
            FROM traffic
            ORDER BY captured_at DESC
            LIMIT {}"#,
            limit.unwrap_or(1000)
        );

        let rows: Vec<(
            i64, String, Option<String>, String, String, Option<i32>, String, Option<i32>,
            String, Option<Vec<u8>>, String, Option<Vec<u8>>, Option<i32>,
            i64, i32, Option<i64>, i64, String, f64, String, String
        )> = conn.select(&sql)?()?;

        rows.into_iter().map(|row| self.row_to_traffic(row)).collect()
    }

    /// List traffic by asset
    pub fn list_by_asset(&self, asset_id: i64, limit: Option<i64>) -> Result<Vec<Traffic>> {
        let conn = self.connection.lock().unwrap();
        let sql = format!(
            r#"SELECT id, protocol, method, path, src_ip, src_port, dst_ip, dst_port,
                   request_headers, request_body, response_headers, response_body, response_status,
                   size_bytes, duration_ms, asset_id, is_anomaly, anomaly_type, anomaly_score, tags, captured_at
            FROM traffic WHERE asset_id = ?1
            ORDER BY captured_at DESC
            LIMIT {}"#,
            limit.unwrap_or(100)
        );

        let rows = conn.select_bound::<i64, (
            i64, String, Option<String>, String, String, Option<i32>, String, Option<i32>,
            String, Option<Vec<u8>>, String, Option<Vec<u8>>, Option<i32>,
            i64, i32, Option<i64>, i64, String, f64, String, String
        )>(&sql)?(asset_id)?;

        rows.into_iter().map(|row| self.row_to_traffic(row)).collect()
    }

    /// List anomalies
    pub fn list_anomalies(&self, limit: Option<i64>) -> Result<Vec<Traffic>> {
        let conn = self.connection.lock().unwrap();
        let sql = format!(
            r#"SELECT id, protocol, method, path, src_ip, src_port, dst_ip, dst_port,
                   request_headers, request_body, response_headers, response_body, response_status,
                   size_bytes, duration_ms, asset_id, is_anomaly, anomaly_type, anomaly_score, tags, captured_at
            FROM traffic WHERE is_anomaly = 1
            ORDER BY anomaly_score DESC, captured_at DESC
            LIMIT {}"#,
            limit.unwrap_or(100)
        );

        let rows: Vec<(
            i64, String, Option<String>, String, String, Option<i32>, String, Option<i32>,
            String, Option<Vec<u8>>, String, Option<Vec<u8>>, Option<i32>,
            i64, i32, Option<i64>, i64, String, f64, String, String
        )> = conn.select(&sql)?()?;

        rows.into_iter().map(|row| self.row_to_traffic(row)).collect()
    }

    /// Get traffic statistics
    pub fn get_stats(&self) -> Result<TrafficStats> {
        let conn = self.connection.lock().unwrap();

        let total: i64 = conn.select::<i64>("SELECT COUNT(*) FROM traffic")?()?
            .into_iter().next().unwrap_or(0);

        let anomalies: i64 = conn.select::<i64>("SELECT COUNT(*) FROM traffic WHERE is_anomaly = 1")?()?
            .into_iter().next().unwrap_or(0);

        let total_size: i64 = conn.select::<i64>("SELECT COALESCE(SUM(size_bytes), 0) FROM traffic")?()?
            .into_iter().next().unwrap_or(0);

        let avg_duration_row: Vec<Option<f64>> = conn.select::<Option<f64>>(
            "SELECT AVG(duration_ms) FROM traffic WHERE duration_ms > 0"
        )?()?;
        let avg_duration_ms = avg_duration_row.into_iter().next().flatten().unwrap_or(0.0) as i32;

        let protocol_rows: Vec<(String, i64)> = conn.select(
            "SELECT protocol, COUNT(*) FROM traffic GROUP BY protocol"
        )?()?;

        Ok(TrafficStats {
            total,
            anomalies,
            total_size_bytes: total_size,
            avg_duration_ms,
            by_protocol: protocol_rows,
        })
    }

    /// Delete old traffic entries
    pub fn delete_old(&self, older_than_days: i32) -> Result<i64> {
        let conn = self.connection.lock().unwrap();
        let sql = "DELETE FROM traffic WHERE captured_at < datetime('now', ?1)";
        let param = format!("-{} days", older_than_days);
        conn.exec_bound::<String>(sql)?(param)?;

        let deleted: i64 = conn.select::<i64>("SELECT changes()")?()?
            .into_iter().next().unwrap_or(0);
        Ok(deleted)
    }

    fn row_to_traffic(&self, row: (
        i64, String, Option<String>, String, String, Option<i32>, String, Option<i32>,
        String, Option<Vec<u8>>, String, Option<Vec<u8>>, Option<i32>,
        i64, i32, Option<i64>, i64, String, f64, String, String
    )) -> Result<Traffic> {
        let (id, protocol, method, path, src_ip, src_port, dst_ip, dst_port,
             request_headers, request_body, response_headers, response_body, response_status,
             size_bytes, duration_ms, asset_id, is_anomaly, anomaly_type, anomaly_score, tags_json, captured_at_str) = row;

        let tags = serde_json::from_str(&tags_json).unwrap_or_default();

        Ok(Traffic {
            id,
            protocol,
            method,
            path,
            src_ip,
            src_port,
            dst_ip,
            dst_port,
            request_headers,
            request_body,
            response_headers,
            response_body,
            response_status,
            size_bytes,
            duration_ms,
            asset_id,
            is_anomaly: is_anomaly != 0,
            anomaly_type,
            anomaly_score,
            tags,
            captured_at: parse_datetime(&captured_at_str)?,
        })
    }
}

/// Statistics for traffic
#[derive(Debug, Clone)]
pub struct TrafficStats {
    pub total: i64,
    pub anomalies: i64,
    pub total_size_bytes: i64,
    pub avg_duration_ms: i32,
    pub by_protocol: Vec<(String, i64)>,
}

pub struct VulnerabilityRepository {
    connection: Arc<Mutex<Connection>>,
}

impl VulnerabilityRepository {
    pub fn new(connection: Arc<Mutex<Connection>>) -> Self {
        Self { connection }
    }

    /// Get vulnerability by ID
    pub fn get_by_id(&self, id: &str) -> Result<Option<Vulnerability>> {
        let conn = self.connection.lock().unwrap();
        let sql = r#"
            SELECT id, name, description, vuln_type, severity, cvss_score, cvss_vector,
                   cve_id, cwe_id, affected_systems, affected_versions, exploit_available,
                   exploit_complexity, solution, ref_urls, created_at, updated_at
            FROM vulnerabilities WHERE id = ?1
        "#;

        let rows = conn.select_bound::<&str, (
            String, String, String, String, String, Option<f64>, String,
            String, String, String, String, i64, String, String, String, String, String
        )>(sql)?(id)?;

        Ok(rows.into_iter().next().map(|row| self.row_to_vulnerability(row).ok()).flatten())
    }

    /// Get vulnerability by CVE ID
    pub fn get_by_cve(&self, cve_id: &str) -> Result<Option<Vulnerability>> {
        let conn = self.connection.lock().unwrap();
        let sql = r#"
            SELECT id, name, description, vuln_type, severity, cvss_score, cvss_vector,
                   cve_id, cwe_id, affected_systems, affected_versions, exploit_available,
                   exploit_complexity, solution, ref_urls, created_at, updated_at
            FROM vulnerabilities WHERE cve_id = ?1
        "#;

        let rows = conn.select_bound::<&str, (
            String, String, String, String, String, Option<f64>, String,
            String, String, String, String, i64, String, String, String, String, String
        )>(sql)?(cve_id)?;

        Ok(rows.into_iter().next().map(|row| self.row_to_vulnerability(row).ok()).flatten())
    }

    /// List all vulnerabilities
    pub fn list_all(&self) -> Result<Vec<Vulnerability>> {
        let conn = self.connection.lock().unwrap();
        let sql = r#"
            SELECT id, name, description, vuln_type, severity, cvss_score, cvss_vector,
                   cve_id, cwe_id, affected_systems, affected_versions, exploit_available,
                   exploit_complexity, solution, ref_urls, created_at, updated_at
            FROM vulnerabilities
            ORDER BY CASE severity
                WHEN 'critical' THEN 1
                WHEN 'high' THEN 2
                WHEN 'medium' THEN 3
                WHEN 'low' THEN 4
                ELSE 5
            END, cvss_score DESC
        "#;

        let rows: Vec<(
            String, String, String, String, String, Option<f64>, String,
            String, String, String, String, i64, String, String, String, String, String
        )> = conn.select(sql)?()?;

        rows.into_iter().map(|row| self.row_to_vulnerability(row)).collect()
    }

    fn row_to_vulnerability(&self, row: (
        String, String, String, String, String, Option<f64>, String,
        String, String, String, String, i64, String, String, String, String, String
    )) -> Result<Vulnerability> {
        let (id, name, description, vuln_type, severity_str, cvss_score, cvss_vector,
             cve_id, cwe_id, affected_systems_json, affected_versions, exploit_available,
             exploit_complexity, solution, ref_urls_json, created_at_str, updated_at_str) = row;

        let affected_systems = serde_json::from_str(&affected_systems_json).unwrap_or_default();
        let ref_urls = serde_json::from_str(&ref_urls_json).unwrap_or_default();

        Ok(Vulnerability {
            id,
            name,
            description,
            vuln_type,
            severity: Severity::from(severity_str.as_str()),
            cvss_score,
            cvss_vector,
            cve_id,
            cwe_id,
            affected_systems,
            affected_versions,
            exploit_available: exploit_available != 0,
            exploit_complexity,
            solution,
            disclosure_date: None,
            ref_urls,
            created_at: parse_datetime(&created_at_str)?,
            updated_at: parse_datetime(&updated_at_str)?,
        })
    }
}

fn parse_datetime(s: &str) -> Result<chrono::DateTime<chrono::Utc>> {
    chrono::DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&chrono::Utc))
        .or_else(|_| {
            chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S")
                .map(|naive| chrono::DateTime::from_naive_utc_and_offset(naive, chrono::Utc))
        })
        .map_err(|e| anyhow::anyhow!("Failed to parse datetime: {}", e))
}
