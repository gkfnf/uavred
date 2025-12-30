use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vulnerability {
    pub id: String,
    pub name: String,
    pub description: String,
    pub severity: VulnSeverity,
    pub cve: Option<String>,
    pub affected_systems: Vec<String>,
    pub exploit_available: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum VulnSeverity {
    Low,
    Medium,
    High,
    Critical,
}

pub struct VulnerabilityDatabase {
    vulnerabilities: HashMap<String, Vulnerability>,
}

impl VulnerabilityDatabase {
    pub fn new() -> Self {
        let mut db = Self {
            vulnerabilities: HashMap::new(),
        };
        db.load_default_vulnerabilities();
        db
    }

    fn load_default_vulnerabilities(&mut self) {
        // MAVLink vulnerabilities
        self.add_vulnerability(Vulnerability {
            id: "UAV-001".to_string(),
            name: "MAVLink Unauthenticated Command Injection".to_string(),
            description: "MAVLink protocol lacks authentication, allowing arbitrary command injection".to_string(),
            severity: VulnSeverity::Critical,
            cve: Some("CVE-2023-XXXXX".to_string()),
            affected_systems: vec!["ArduPilot".to_string(), "PX4".to_string()],
            exploit_available: true,
        });

        self.add_vulnerability(Vulnerability {
            id: "UAV-002".to_string(),
            name: "DJI WiFi Default Credentials".to_string(),
            description: "DJI drones often ship with default WiFi credentials".to_string(),
            severity: VulnSeverity::High,
            cve: None,
            affected_systems: vec!["DJI Phantom".to_string(), "DJI Mavic".to_string()],
            exploit_available: true,
        });

        self.add_vulnerability(Vulnerability {
            id: "UAV-003".to_string(),
            name: "GPS Spoofing Vulnerability".to_string(),
            description: "UAVs vulnerable to GPS spoofing attacks due to lack of GPS authentication".to_string(),
            severity: VulnSeverity::High,
            cve: None,
            affected_systems: vec!["Most consumer drones".to_string()],
            exploit_available: true,
        });
    }

    pub fn add_vulnerability(&mut self, vuln: Vulnerability) {
        self.vulnerabilities.insert(vuln.id.clone(), vuln);
    }

    pub fn get_vulnerability(&self, id: &str) -> Option<&Vulnerability> {
        self.vulnerabilities.get(id)
    }

    pub fn search(&self, query: &str) -> Vec<&Vulnerability> {
        self.vulnerabilities
            .values()
            .filter(|v| {
                v.name.to_lowercase().contains(&query.to_lowercase())
                    || v.description.to_lowercase().contains(&query.to_lowercase())
            })
            .collect()
    }

    pub fn get_by_severity(&self, severity: VulnSeverity) -> Vec<&Vulnerability> {
        self.vulnerabilities
            .values()
            .filter(|v| v.severity == severity)
            .collect()
    }
}

impl Default for VulnerabilityDatabase {
    fn default() -> Self {
        Self::new()
    }
}
