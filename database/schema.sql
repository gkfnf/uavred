-- UAVRed Database Schema
-- Version: 1.0.0
-- Description: Complete database schema for UAV Red Team security testing platform

-- Enable foreign keys and WAL mode for better performance
PRAGMA foreign_keys = ON;
PRAGMA journal_mode = WAL;
PRAGMA synchronous = NORMAL;

-- ============================================
-- 1. CORE TABLES
-- ============================================

-- Tasks - Mission Control kanban tasks
CREATE TABLE IF NOT EXISTS tasks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    description TEXT DEFAULT '',
    mission_objective TEXT DEFAULT '',
    task_type TEXT NOT NULL DEFAULT 'task', -- task, mission, audit
    priority TEXT NOT NULL DEFAULT 'medium', -- low, medium, high, critical
    status TEXT NOT NULL DEFAULT 'todo', -- todo, in_progress, in_review, done, canceled
    assignee TEXT DEFAULT '',
    estimated_minutes INTEGER,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    started_at DATETIME,
    completed_at DATETIME,
    closed_at DATETIME,
    close_reason TEXT DEFAULT '',
    source TEXT DEFAULT 'manual', -- manual, agent, workflow
    external_ref TEXT DEFAULT '',
    metadata TEXT DEFAULT '{}' -- JSON for extensibility
);

CREATE INDEX idx_tasks_status ON tasks(status);
CREATE INDEX idx_tasks_priority ON tasks(priority);
CREATE INDEX idx_tasks_assignee ON tasks(assignee);
CREATE INDEX idx_tasks_status_priority ON tasks(status, priority);
CREATE INDEX idx_tasks_created_at ON tasks(created_at);

-- Task Dependencies (blocks/blocked_by relationships)
CREATE TABLE IF NOT EXISTS task_dependencies (
    task_id INTEGER NOT NULL,
    depends_on_id INTEGER NOT NULL,
    dependency_type TEXT NOT NULL DEFAULT 'blocks', -- blocks, parent-child, related
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (task_id, depends_on_id),
    FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE CASCADE,
    FOREIGN KEY (depends_on_id) REFERENCES tasks(id) ON DELETE CASCADE
);

CREATE INDEX idx_task_deps_task ON task_dependencies(task_id);
CREATE INDEX idx_task_deps_depends ON task_dependencies(depends_on_id);

-- Task Labels
CREATE TABLE IF NOT EXISTS task_labels (
    task_id INTEGER NOT NULL,
    label TEXT NOT NULL,
    PRIMARY KEY (task_id, label),
    FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE CASCADE
);

CREATE INDEX idx_task_labels_label ON task_labels(label);

-- Task Comments
CREATE TABLE IF NOT EXISTS task_comments (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    task_id INTEGER NOT NULL,
    author TEXT NOT NULL,
    content TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE CASCADE
);

CREATE INDEX idx_task_comments_task ON task_comments(task_id);

-- ============================================
-- 2. ASSETS MODULE
-- ============================================

-- Asset Zones (业务层级)
CREATE TABLE IF NOT EXISTS asset_zones (
    id TEXT PRIMARY KEY, -- Z1, Z2, Z3...
    name TEXT NOT NULL,
    description TEXT DEFAULT '',
    level INTEGER NOT NULL DEFAULT 0,
    color TEXT DEFAULT '#4CAF50',
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Assets (无人机、GCS、服务器等)
CREATE TABLE IF NOT EXISTS assets (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    asset_type TEXT NOT NULL, -- drone, gcs, server, gateway, controller, sensor, emergency_system
    zone_id TEXT,
    ip_address TEXT,
    mac_address TEXT,
    
    -- Status
    status TEXT NOT NULL DEFAULT 'online', -- online, offline, busy, error, maintenance
    
    -- Risk assessment
    risk_score INTEGER DEFAULT 0, -- 0-100
    vuln_count INTEGER DEFAULT 0,
    
    -- Details
    model TEXT DEFAULT '',
    firmware_version TEXT DEFAULT '',
    protocol TEXT DEFAULT '', -- MAVLink, DJI, ArduPilot, PX4
    
    -- Authentication
    auth_type TEXT DEFAULT '', -- API Key, Certificate, Password, None
    auth_status TEXT DEFAULT 'unknown', -- valid, invalid, unknown
    auth_credential TEXT DEFAULT '', -- encrypted
    
    -- Business info
    business_purpose TEXT DEFAULT '',
    owner_team TEXT DEFAULT '',
    
    -- Compliance
    compliance_standards TEXT DEFAULT '[]', -- JSON array ["ISO 27001", "NIST 800-53"]
    
    -- Scan info
    last_scan_at DATETIME,
    scan_interval_minutes INTEGER DEFAULT 60,
    
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    FOREIGN KEY (zone_id) REFERENCES asset_zones(id)
);

CREATE INDEX idx_assets_type ON assets(asset_type);
CREATE INDEX idx_assets_zone ON assets(zone_id);
CREATE INDEX idx_assets_status ON assets(status);
CREATE INDEX idx_assets_ip ON assets(ip_address);

-- Asset Services (开放的端口和服务)
CREATE TABLE IF NOT EXISTS asset_services (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    asset_id INTEGER NOT NULL,
    port INTEGER NOT NULL,
    protocol TEXT NOT NULL, -- TCP, UDP, HTTP, HTTPS, WebSocket, MAVLink
    service_name TEXT DEFAULT '',
    service_version TEXT DEFAULT '',
    banner TEXT DEFAULT '',
    is_vulnerable INTEGER DEFAULT 0,
    detected_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (asset_id) REFERENCES assets(id) ON DELETE CASCADE
);

CREATE INDEX idx_asset_services_asset ON asset_services(asset_id);
CREATE INDEX idx_asset_services_port ON asset_services(port);

-- Asset Network Connections (拓扑图中的连接)
CREATE TABLE IF NOT EXISTS asset_connections (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    source_asset_id INTEGER NOT NULL,
    target_asset_id INTEGER NOT NULL,
    connection_type TEXT NOT NULL DEFAULT 'data', -- data, control, telemetry
    protocol TEXT DEFAULT '',
    is_active INTEGER DEFAULT 1,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (source_asset_id) REFERENCES assets(id) ON DELETE CASCADE,
    FOREIGN KEY (target_asset_id) REFERENCES assets(id) ON DELETE CASCADE
);

CREATE INDEX idx_asset_conn_source ON asset_connections(source_asset_id);
CREATE INDEX idx_asset_conn_target ON asset_connections(target_asset_id);

-- ============================================
-- 3. VULNERABILITIES MODULE
-- ============================================

-- Vulnerability Database
CREATE TABLE IF NOT EXISTS vulnerabilities (
    id TEXT PRIMARY KEY, -- UAV-001, CVE-2024-1234, etc.
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    
    -- Classification
    vuln_type TEXT NOT NULL, -- buffer_overflow, sql_injection, auth_bypass, default_creds, etc.
    severity TEXT NOT NULL, -- info, low, medium, high, critical
    
    -- Scoring
    cvss_score REAL,
    cvss_vector TEXT DEFAULT '',
    
    -- References
    cve_id TEXT DEFAULT '',
    cwe_id TEXT DEFAULT '',
    
    -- Affected systems
    affected_systems TEXT DEFAULT '[]', -- JSON array
    affected_versions TEXT DEFAULT '',
    
    -- Exploitation
    exploit_available INTEGER DEFAULT 0,
    exploit_complexity TEXT DEFAULT '', -- easy, medium, hard
    
    -- Metadata
    disclosure_date DATE,
    solution TEXT DEFAULT '',
    ref_urls TEXT DEFAULT '[]', -- JSON array of URLs
    
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_vulns_severity ON vulnerabilities(severity);
CREATE INDEX idx_vulns_type ON vulnerabilities(vuln_type);
CREATE INDEX idx_vulns_cve ON vulnerabilities(cve_id);

-- Security Findings (实际发现的漏洞实例)
CREATE TABLE IF NOT EXISTS findings (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    
    -- Reference
    vuln_id TEXT,
    asset_id INTEGER NOT NULL,
    service_id INTEGER,
    task_id INTEGER,
    
    -- Finding details
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    evidence TEXT DEFAULT '',
    
    -- Severity (can override vuln severity for specific context)
    severity TEXT NOT NULL,
    cvss_score REAL,
    
    -- Status workflow
    status TEXT NOT NULL DEFAULT 'new', -- new, validating, confirmed, false_positive, remediated, accepted
    
    -- AI Analysis
    ai_confidence INTEGER, -- 0-100
    ai_analysis TEXT DEFAULT '',
    ai_recommendation TEXT DEFAULT '',
    
    -- Proof of Concept
    poc_code TEXT DEFAULT '',
    poc_language TEXT DEFAULT '', -- python, rust, etc.
    
    -- MITRE ATT&CK
    mitre_techniques TEXT DEFAULT '[]', -- JSON array ["T0806", "T0868"]
    
    -- Remediation
    remediation_steps TEXT DEFAULT '',
    remediation_eta DATE,
    remediated_at DATETIME,
    remediated_by TEXT DEFAULT '',
    
    -- Detection info
    detected_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    detected_by TEXT DEFAULT 'manual', -- manual, scanner, agent, ai
    
    FOREIGN KEY (vuln_id) REFERENCES vulnerabilities(id),
    FOREIGN KEY (asset_id) REFERENCES assets(id) ON DELETE CASCADE,
    FOREIGN KEY (service_id) REFERENCES asset_services(id) ON DELETE SET NULL,
    FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE SET NULL
);

CREATE INDEX idx_findings_asset ON findings(asset_id);
CREATE INDEX idx_findings_vuln ON findings(vuln_id);
CREATE INDEX idx_findings_status ON findings(status);
CREATE INDEX idx_findings_severity ON findings(severity);
CREATE INDEX idx_findings_task ON findings(task_id);
CREATE INDEX idx_findings_detected ON findings(detected_at);

-- ============================================
-- 4. TRAFFIC MODULE
-- ============================================

-- Captured Traffic
CREATE TABLE IF NOT EXISTS traffic (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    
    -- Packet info
    protocol TEXT NOT NULL, -- HTTP, HTTPS, MAVLink, RTSP, TCP, UDP
    method TEXT, -- GET, POST, etc.
    path TEXT DEFAULT '',
    
    -- Source/Destination
    src_ip TEXT NOT NULL,
    src_port INTEGER,
    dst_ip TEXT NOT NULL,
    dst_port INTEGER,
    
    -- Content
    request_headers TEXT DEFAULT '',
    request_body BLOB,
    response_headers TEXT DEFAULT '',
    response_body BLOB,
    response_status INTEGER,
    
    -- Size & Timing
    size_bytes INTEGER NOT NULL DEFAULT 0,
    duration_ms INTEGER DEFAULT 0,
    
    -- Asset reference
    asset_id INTEGER,
    
    -- Analysis
    is_anomaly INTEGER DEFAULT 0,
    anomaly_type TEXT DEFAULT '',
    anomaly_score REAL DEFAULT 0,
    
    -- Tags for filtering
    tags TEXT DEFAULT '[]', -- JSON array
    
    captured_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    FOREIGN KEY (asset_id) REFERENCES assets(id) ON DELETE SET NULL
);

CREATE INDEX idx_traffic_asset ON traffic(asset_id);
CREATE INDEX idx_traffic_protocol ON traffic(protocol);
CREATE INDEX idx_traffic_anomaly ON traffic(is_anomaly);
CREATE INDEX idx_traffic_captured ON traffic(captured_at);

-- Traffic Anomalies
CREATE TABLE IF NOT EXISTS traffic_anomalies (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    traffic_id INTEGER NOT NULL,
    anomaly_type TEXT NOT NULL, -- buffer_overflow, sql_injection, xss, etc.
    confidence INTEGER NOT NULL, -- 0-100
    description TEXT NOT NULL,
    payload_sample TEXT DEFAULT '',
    detected_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (traffic_id) REFERENCES traffic(id) ON DELETE CASCADE
);

CREATE INDEX idx_traffic_anomalies_traffic ON traffic_anomalies(traffic_id);

-- ============================================
-- 5. WORKFLOWS MODULE
-- ============================================

-- Workflow Definitions
CREATE TABLE IF NOT EXISTS workflows (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    description TEXT DEFAULT '',
    
    -- Classification
    workflow_type TEXT NOT NULL, -- atomic, composite, mission
    category TEXT DEFAULT '', -- port_scan, web_test, sql_test, mavlink_scan
    
    -- DAG structure stored as JSON
    node_count INTEGER DEFAULT 0,
    max_parallel INTEGER DEFAULT 1,
    estimated_duration_seconds INTEGER,
    
    -- Stats
    success_rate INTEGER DEFAULT 0, -- percentage
    total_executions INTEGER DEFAULT 0,
    
    -- Status
    is_active INTEGER DEFAULT 1,
    is_template INTEGER DEFAULT 0,
    
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_workflows_type ON workflows(workflow_type);
CREATE INDEX idx_workflows_active ON workflows(is_active);

-- Workflow Nodes (DAG nodes)
CREATE TABLE IF NOT EXISTS workflow_nodes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    workflow_id INTEGER NOT NULL,
    node_id TEXT NOT NULL, -- unique within workflow, e.g., "node-1"
    
    -- Node definition
    name TEXT NOT NULL,
    node_type TEXT NOT NULL, -- scan, validate, fuzz, exploit, report
    action TEXT NOT NULL, -- port_scan, web_validate, sql_test, etc.
    
    -- Execution
    estimated_duration_seconds INTEGER,
    max_retries INTEGER DEFAULT 0,
    
    -- Configuration
    config TEXT DEFAULT '{}', -- JSON with parameters
    
    -- Position for UI
    position_x REAL DEFAULT 0,
    position_y REAL DEFAULT 0,
    
    FOREIGN KEY (workflow_id) REFERENCES workflows(id) ON DELETE CASCADE
);

CREATE INDEX idx_workflow_nodes_workflow ON workflow_nodes(workflow_id);

-- Workflow Node Dependencies (DAG edges)
CREATE TABLE IF NOT EXISTS workflow_node_edges (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    workflow_id INTEGER NOT NULL,
    source_node_id TEXT NOT NULL,
    target_node_id TEXT NOT NULL,
    edge_type TEXT DEFAULT 'success', -- success, failure, always, condition
    condition TEXT DEFAULT '', -- condition expression if edge_type is condition
    FOREIGN KEY (workflow_id) REFERENCES workflows(id) ON DELETE CASCADE
);

CREATE INDEX idx_workflow_edges ON workflow_node_edges(workflow_id);

-- Workflow Executions
CREATE TABLE IF NOT EXISTS workflow_executions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    workflow_id INTEGER NOT NULL,
    
    -- Execution context
    name TEXT DEFAULT '',
    target_assets TEXT DEFAULT '[]', -- JSON array of asset IDs
    
    -- Status
    status TEXT NOT NULL DEFAULT 'pending', -- pending, running, paused, completed, failed, canceled
    
    -- Timing
    started_at DATETIME,
    completed_at DATETIME,
    duration_seconds INTEGER,
    
    -- Results
    progress_percent INTEGER DEFAULT 0,
    nodes_completed INTEGER DEFAULT 0,
    nodes_total INTEGER DEFAULT 0,
    
    -- Output
    findings_count INTEGER DEFAULT 0,
    report_path TEXT DEFAULT '',
    
    -- Error info
    error_message TEXT DEFAULT '',
    
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    FOREIGN KEY (workflow_id) REFERENCES workflows(id) ON DELETE CASCADE
);

CREATE INDEX idx_workflow_exec_workflow ON workflow_executions(workflow_id);
CREATE INDEX idx_workflow_exec_status ON workflow_executions(status);

-- ============================================
-- 6. AGENTS & IMAGES MODULE
-- ============================================

-- Container Images / Agent Templates
CREATE TABLE IF NOT EXISTS agent_images (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL, -- ai-pentest-agent, network-scanner
    version TEXT NOT NULL,
    description TEXT DEFAULT '',
    
    -- Image details
    image_type TEXT NOT NULL, -- agent, scanner, fuzzer, exploit
    docker_image TEXT DEFAULT '',
    
    -- Capabilities
    capabilities TEXT DEFAULT '[]', -- JSON array ["web_scan", "mavlink_fuzz"]
    
    -- Status
    status TEXT DEFAULT 'available', -- available, building, error, deprecated
    
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_agent_images_type ON agent_images(image_type);

-- Running Agents / Containers
CREATE TABLE IF NOT EXISTS agents (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL, -- Agent-Alpha, Agent-Beta
    image_id INTEGER,
    
    -- Container info
    container_id TEXT DEFAULT '',
    docker_exec_command TEXT DEFAULT '',
    
    -- Status
    status TEXT NOT NULL DEFAULT 'stopped', -- running, stopped, building, error
    
    -- Current task
    current_task_id INTEGER,
    current_task_name TEXT DEFAULT '',
    
    -- Resources
    cpu_percent REAL DEFAULT 0,
    memory_percent REAL DEFAULT 0,
    memory_mb INTEGER DEFAULT 0,
    
    -- Exposed ports
    exposed_ports TEXT DEFAULT '[]', -- JSON array
    
    -- Runtime
    started_at DATETIME,
    running_duration_seconds INTEGER DEFAULT 0,
    tasks_completed INTEGER DEFAULT 0,
    
    -- Live trace
    live_trace TEXT DEFAULT '', -- recent output
    
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    FOREIGN KEY (image_id) REFERENCES agent_images(id) ON DELETE SET NULL,
    FOREIGN KEY (current_task_id) REFERENCES tasks(id) ON DELETE SET NULL
);

CREATE INDEX idx_agents_status ON agents(status);
CREATE INDEX idx_agents_image ON agents(image_id);
CREATE INDEX idx_agents_task ON agents(current_task_id);

-- Agent Execution Logs
CREATE TABLE IF NOT EXISTS agent_logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    agent_id INTEGER NOT NULL,
    log_level TEXT NOT NULL DEFAULT 'info', -- debug, info, warning, error
    message TEXT NOT NULL,
    metadata TEXT DEFAULT '{}',
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (agent_id) REFERENCES agents(id) ON DELETE CASCADE
);

CREATE INDEX idx_agent_logs_agent ON agent_logs(agent_id);

-- ============================================
-- 7. DEVICES MODULE (Hardware SDR)
-- ============================================

-- Hardware Devices (SDR)
CREATE TABLE IF NOT EXISTS devices (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL, -- HackRF One, USRP B210
    device_type TEXT NOT NULL, -- HackRF, USRP, BladeRF, RTL-SDR, PlutoSDR
    
    -- Hardware info
    serial_number TEXT DEFAULT '',
    firmware_version TEXT DEFAULT '',
    device_path TEXT DEFAULT '', -- /dev/ttyUSB1
    
    -- Status
    status TEXT NOT NULL DEFAULT 'disconnected', -- connected, busy, ready, error, disconnected
    
    -- Radio parameters
    frequency_hz INTEGER DEFAULT 0,
    sample_rate INTEGER DEFAULT 0, -- samples per second
    bandwidth_hz INTEGER DEFAULT 0,
    gain_db INTEGER DEFAULT 0,
    
    -- Temperature monitoring
    temperature_celsius REAL,
    
    -- Usage stats
    total_runtime_seconds INTEGER DEFAULT 0,
    tasks_completed INTEGER DEFAULT 0,
    last_used_at DATETIME,
    
    -- Current operation
    current_operation TEXT DEFAULT '',
    current_task_id INTEGER,
    
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    FOREIGN KEY (current_task_id) REFERENCES tasks(id) ON DELETE SET NULL
);

CREATE INDEX idx_devices_type ON devices(device_type);
CREATE INDEX idx_devices_status ON devices(status);

-- ============================================
-- 8. SETTINGS & CONFIGURATION
-- ============================================

-- Application Settings
CREATE TABLE IF NOT EXISTS settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    value_type TEXT DEFAULT 'string', -- string, int, float, bool, json
    description TEXT DEFAULT '',
    category TEXT DEFAULT 'general', -- general, appearance, ai, security, network, workflow, scanner, storage
    is_editable INTEGER DEFAULT 1,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_settings_category ON settings(category);

-- Default settings
INSERT OR IGNORE INTO settings (key, value, value_type, description, category) VALUES
('app.name', 'UAV Red Team', 'string', 'Application name', 'general'),
('app.version', '0.1.0', 'string', 'Application version', 'general'),
('app.auto_update', 'true', 'bool', 'Automatically check and install updates', 'general'),
('app.language', 'zh-CN', 'string', 'UI display language', 'general'),
('app.startup_view', 'dashboard', 'string', 'Default view when application starts', 'general'),
('app.log_level', 'info', 'string', 'Logging level', 'general'),

('appearance.theme', 'dark', 'string', 'UI theme', 'appearance'),
('appearance.font_size', '14', 'int', 'Default font size', 'appearance'),
('appearance.show_tooltips', 'true', 'bool', 'Show tooltips', 'appearance'),

('ai.enabled', 'true', 'bool', 'Enable AI features', 'ai'),
('ai.model', 'claude', 'string', 'Default AI model', 'ai'),
('ai.confidence_threshold', '80', 'int', 'Minimum AI confidence for alerts', 'ai'),

('security.require_auth', 'true', 'bool', 'Require authorization for sensitive operations', 'security'),
('security.log_all_actions', 'true', 'bool', 'Log all security-related actions', 'security'),
('security.encrypt_sensitive', 'true', 'bool', 'Encrypt sensitive data in database', 'security'),
('security.max_failed_attempts', '3', 'int', 'Maximum failed login attempts', 'security'),

('network.timeout_seconds', '30', 'int', 'Default network timeout', 'network'),
('network.max_concurrent_scans', '10', 'int', 'Maximum concurrent network scans', 'network'),
('network.common_uav_ports', '[14550,14551,5760,5761,8554,8080]', 'json', 'Common UAV ports to scan', 'network'),

('scanner.firmware_max_size_mb', '512', 'int', 'Maximum firmware file size to analyze', 'scanner'),
('scanner.string_min_length', '4', 'int', 'Minimum string length for firmware analysis', 'scanner'),
('scanner.extract_timeout_seconds', '300', 'int', 'Firmware extraction timeout', 'scanner'),
('scanner.auto_update_vulns', 'true', 'bool', 'Auto update vulnerability database', 'scanner'),
('scanner.vuln_update_interval_hours', '24', 'int', 'Vulnerability DB update interval', 'scanner'),

('workflow.default_timeout_seconds', '3600', 'int', 'Default workflow execution timeout', 'workflow'),
('workflow.max_parallel_nodes', '10', 'int', 'Maximum parallel workflow nodes', 'workflow'),

('export.default_format', 'json', 'string', 'Default export format', 'general'),
('export.include_metadata', 'true', 'bool', 'Include metadata in exports', 'general'),
('export.compress_results', 'true', 'bool', 'Compress exported results', 'general');

-- ============================================
-- 9. AUDIT & LOGS
-- ============================================

-- Audit Log
CREATE TABLE IF NOT EXISTS audit_logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    action TEXT NOT NULL, -- create_task, delete_asset, run_workflow, etc.
    entity_type TEXT NOT NULL, -- task, asset, vulnerability, workflow, etc.
    entity_id TEXT, -- can be string ID
    
    -- User/Agent info
    actor_type TEXT NOT NULL DEFAULT 'user', -- user, agent, system
    actor_id TEXT DEFAULT '',
    actor_name TEXT DEFAULT '',
    
    -- Details
    description TEXT NOT NULL,
    old_value TEXT DEFAULT '',
    new_value TEXT DEFAULT '',
    
    -- Context
    ip_address TEXT DEFAULT '',
    user_agent TEXT DEFAULT '',
    
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_audit_logs_action ON audit_logs(action);
CREATE INDEX idx_audit_logs_entity ON audit_logs(entity_type, entity_id);
CREATE INDEX idx_audit_logs_actor ON audit_logs(actor_type, actor_id);
CREATE INDEX idx_audit_logs_created ON audit_logs(created_at);

-- System Events
CREATE TABLE IF NOT EXISTS system_events (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    event_type TEXT NOT NULL, -- error, warning, info, success
    source TEXT NOT NULL, -- module/component name
    message TEXT NOT NULL,
    details TEXT DEFAULT '',
    stack_trace TEXT DEFAULT '',
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_system_events_type ON system_events(event_type);
CREATE INDEX idx_system_events_source ON system_events(source);

-- ============================================
-- 10. VIEWS
-- ============================================

-- View: Ready Tasks (not blocked by dependencies)
CREATE VIEW IF NOT EXISTS ready_tasks AS
SELECT t.*
FROM tasks t
LEFT JOIN task_dependencies td ON t.id = td.task_id
WHERE t.status = 'todo'
  AND (td.task_id IS NULL OR NOT EXISTS (
      SELECT 1 FROM task_dependencies td2
      JOIN tasks t2 ON td2.depends_on_id = t2.id
      WHERE td2.task_id = t.id AND t2.status NOT IN ('done', 'canceled')
  ));

-- View: Blocked Tasks
CREATE VIEW IF NOT EXISTS blocked_tasks AS
SELECT 
    t.*,
    COUNT(td.depends_on_id) as blocked_by_count,
    GROUP_CONCAT(t2.title) as blocked_by_titles
FROM tasks t
JOIN task_dependencies td ON t.id = td.task_id
JOIN tasks t2 ON td.depends_on_id = t2.id
WHERE t.status IN ('todo', 'in_progress', 'blocked')
  AND t2.status NOT IN ('done', 'canceled')
GROUP BY t.id;

-- View: Asset Risk Summary
CREATE VIEW IF NOT EXISTS asset_risk_summary AS
SELECT 
    a.id,
    a.name,
    a.asset_type,
    a.zone_id,
    a.risk_score,
    COUNT(DISTINCT f.id) as total_findings,
    COUNT(DISTINCT CASE WHEN f.severity = 'critical' THEN f.id END) as critical_count,
    COUNT(DISTINCT CASE WHEN f.severity = 'high' THEN f.id END) as high_count,
    COUNT(DISTINCT CASE WHEN f.severity = 'medium' THEN f.id END) as medium_count,
    COUNT(DISTINCT s.id) as service_count
FROM assets a
LEFT JOIN findings f ON a.id = f.asset_id AND f.status IN ('new', 'validating', 'confirmed')
LEFT JOIN asset_services s ON a.id = s.asset_id
GROUP BY a.id;

-- View: Recent Activity
CREATE VIEW IF NOT EXISTS recent_activity AS
SELECT 
    'task' as entity_type,
    id as entity_id,
    title as description,
    status,
    updated_at as activity_time
FROM tasks
WHERE updated_at > datetime('now', '-7 days')

UNION ALL

SELECT 
    'finding' as entity_type,
    id as entity_id,
    title as description,
    status,
    detected_at as activity_time
FROM findings
WHERE detected_at > datetime('now', '-7 days')

UNION ALL

SELECT 
    'workflow_execution' as entity_type,
    id as entity_id,
    name as description,
    status,
    updated_at as activity_time
FROM workflow_executions
WHERE updated_at > datetime('now', '-7 days')

ORDER BY activity_time DESC;
