# UAVRed Database Architecture

## Overview

UAVRed uses SQLite as its embedded database for local data persistence. The database is designed to support all functional modules of the UAV Red Team security testing platform.

- **Schema Version**: 1.0.0
- **Engine**: SQLite 3.45+ with WAL mode enabled

## Database Locations

### Development Mode (Current Worktree)

During active development in this worktree, the database is stored locally:

- **Local Database File**: `./database/uavred.db` (relative to project root)
- **Purpose**: Isolated testing and schema iteration
- **Note**: This location is used when running from the development worktree

### Production/Shared Mode

Once database schema is stabilized, the shared database location is:

- **Shared Database File**: `~/Library/Application Support/uavred/uavred.db`
- **Purpose**: Shared across all worktrees and production builds
- **Migration**: When schema is finalized, copy `./database/uavred.db` to the shared location

### Location Selection Logic

The application automatically selects the appropriate database location:

1. **Development**: Uses `./database/uavred.db` if the file exists or if running in dev mode
2. **Production**: Uses `~/Library/Application Support/uavred/uavred.db`
3. **Override**: Set `UAVRED_DB_PATH` environment variable to use a custom location

## Quick Stats

## Quick Stats

| Metric | Count |
|--------|-------|
| Tables | 24 |
| Indexes | 56 |
| Views | 4 |
| Settings | 29 |

## Table Structure

### 1. Core Tables

#### `tasks`
Mission Control kanban board tasks.

| Field | Type | Description |
|-------|------|-------------|
| id | INTEGER PK | Auto-increment ID |
| title | TEXT | Task title |
| description | TEXT | Task description |
| mission_objective | TEXT | Mission objective details |
| task_type | TEXT | task, mission, audit |
| priority | TEXT | low, medium, high, critical |
| status | TEXT | todo, in_progress, in_review, done, canceled |
| assignee | TEXT | Assigned user/agent |
| estimated_minutes | INTEGER | Time estimate |
| created_at | DATETIME | Creation timestamp |
| updated_at | DATETIME | Last update timestamp |
| started_at | DATETIME | When work started |
| completed_at | DATETIME | When completed |
| closed_at | DATETIME | When closed |
| source | TEXT | manual, agent, workflow |
| metadata | TEXT | JSON extensibility |

**Related Tables**: `task_dependencies`, `task_labels`, `task_comments`

---

### 2. Assets Module

#### `asset_zones`
Business hierarchy zones (Z1, Z2, Z3...).

| Field | Type | Description |
|-------|------|-------------|
| id | TEXT PK | Zone ID (Z1, Z2...) |
| name | TEXT | Zone name |
| level | INTEGER | Hierarchy level |
| color | TEXT | UI color code |

#### `assets`
Network assets (drones, GCS, servers, etc.).

| Field | Type | Description |
|-------|------|-------------|
| id | INTEGER PK | Asset ID |
| name | TEXT | Asset name |
| asset_type | TEXT | drone, gcs, server, gateway, controller, sensor |
| zone_id | TEXT FK | Parent zone |
| ip_address | TEXT | IP address |
| mac_address | TEXT | MAC address |
| status | TEXT | online, offline, busy, error, maintenance |
| risk_score | INTEGER | 0-100 risk score |
| vuln_count | INTEGER | Number of vulnerabilities |
| model | TEXT | Hardware model |
| firmware_version | TEXT | Firmware version |
| protocol | TEXT | MAVLink, DJI, ArduPilot, PX4 |
| auth_type | TEXT | API Key, Certificate, Password |
| auth_status | TEXT | valid, invalid, unknown |
| compliance_standards | TEXT | JSON ["ISO 27001", "NIST 800-53"] |

**Related Tables**: `asset_services`, `asset_connections`

---

### 3. Vulnerabilities Module

#### `vulnerabilities`
Vulnerability database (CVEs, UAV-specific).

| Field | Type | Description |
|-------|------|-------------|
| id | TEXT PK | UAV-001, CVE-2024-1234 |
| name | TEXT | Vulnerability name |
| description | TEXT | Full description |
| vuln_type | TEXT | buffer_overflow, sql_injection, etc. |
| severity | TEXT | info, low, medium, high, critical |
| cvss_score | REAL | CVSS score |
| cve_id | TEXT | CVE reference |
| cwe_id | TEXT | CWE reference |
| affected_systems | TEXT | JSON array |
| exploit_available | INTEGER | Boolean |
| ref_urls | TEXT | JSON array of references |

#### `findings`
Actual discovered vulnerabilities.

| Field | Type | Description |
|-------|------|-------------|
| id | INTEGER PK | Finding ID |
| vuln_id | TEXT FK | Reference to vulnerability |
| asset_id | INTEGER FK | Affected asset |
| title | TEXT | Finding title |
| severity | TEXT | Severity level |
| status | TEXT | new, validating, confirmed, false_positive, remediated |
| ai_confidence | INTEGER | AI detection confidence (0-100) |
| ai_analysis | TEXT | AI-generated analysis |
| poc_code | TEXT | Proof of concept code |
| mitre_techniques | TEXT | JSON ["T0806", "T0868"] |

---

### 4. Traffic Module

#### `traffic`
Captured network traffic.

| Field | Type | Description |
|-------|------|-------------|
| id | INTEGER PK | Traffic ID |
| protocol | TEXT | HTTP, HTTPS, MAVLink, RTSP |
| method | TEXT | GET, POST, etc. |
| src_ip | TEXT | Source IP |
| dst_ip | TEXT | Destination IP |
| request_headers | TEXT | HTTP headers |
| request_body | BLOB | Request body |
| response_body | BLOB | Response body |
| is_anomaly | INTEGER | Boolean flag |
| anomaly_score | REAL | Anomaly confidence |
| captured_at | DATETIME | Capture timestamp |

#### `traffic_anomalies`
Detected anomalies in traffic.

| Field | Type | Description |
|-------|------|-------------|
| id | INTEGER PK | Anomaly ID |
| traffic_id | INTEGER FK | Reference to traffic |
| anomaly_type | TEXT | sql_injection, xss, buffer_overflow |
| confidence | INTEGER | Detection confidence |

---

### 5. Workflows Module

#### `workflows`
Workflow definitions (DAG).

| Field | Type | Description |
|-------|------|-------------|
| id | INTEGER PK | Workflow ID |
| name | TEXT | Workflow name |
| workflow_type | TEXT | atomic, composite, mission |
| category | TEXT | port_scan, web_test, mavlink_scan |
| node_count | INTEGER | Number of nodes |
| max_parallel | INTEGER | Parallel execution limit |
| success_rate | INTEGER | Historical success rate |

#### `workflow_nodes`
DAG nodes.

| Field | Type | Description |
|-------|------|-------------|
| id | INTEGER PK | Node ID |
| workflow_id | INTEGER FK | Parent workflow |
| node_id | TEXT | Unique node ID in workflow |
| node_type | TEXT | scan, validate, fuzz, exploit |
| action | TEXT | Specific action |
| config | TEXT | JSON configuration |
| position_x | REAL | UI X position |
| position_y | REAL | UI Y position |

#### `workflow_node_edges`
DAG edges (dependencies).

| Field | Type | Description |
|-------|------|-------------|
| workflow_id | INTEGER FK | Parent workflow |
| source_node_id | TEXT | Source node |
| target_node_id | TEXT | Target node |
| edge_type | TEXT | success, failure, always, condition |

#### `workflow_executions`
Workflow execution instances.

| Field | Type | Description |
|-------|------|-------------|
| id | INTEGER PK | Execution ID |
| workflow_id | INTEGER FK | Workflow definition |
| status | TEXT | pending, running, completed, failed |
| progress_percent | INTEGER | 0-100 progress |
| findings_count | INTEGER | Findings discovered |

---

### 6. Agents Module

#### `agent_images`
Container image definitions.

| Field | Type | Description |
|-------|------|-------------|
| id | INTEGER PK | Image ID |
| name | TEXT | Image name (ai-pentest-agent) |
| version | TEXT | Version |
| image_type | TEXT | agent, scanner, fuzzer |
| capabilities | TEXT | JSON ["web_scan", "mavlink_fuzz"] |

#### `agents`
Running agent instances.

| Field | Type | Description |
|-------|------|-------------|
| id | INTEGER PK | Agent ID |
| name | TEXT | Agent name (Agent-Alpha) |
| image_id | INTEGER FK | Image reference |
| status | TEXT | running, stopped, building |
| current_task_id | INTEGER FK | Current task |
| cpu_percent | REAL | CPU usage |
| memory_percent | REAL | Memory usage |
| live_trace | TEXT | Recent output |

---

### 7. Devices Module

#### `devices`
Hardware SDR devices.

| Field | Type | Description |
|-------|------|-------------|
| id | INTEGER PK | Device ID |
| name | TEXT | Device name (HackRF One) |
| device_type | TEXT | HackRF, USRP, BladeRF |
| serial_number | TEXT | Hardware SN |
| status | TEXT | connected, busy, ready, error |
| frequency_hz | INTEGER | Current frequency |
| sample_rate | INTEGER | Sample rate |
| temperature_celsius | REAL | Device temperature |
| total_runtime_seconds | INTEGER | Total usage time |

---

### 8. Settings Module

#### `settings`
Application configuration.

| Field | Type | Description |
|-------|------|-------------|
| key | TEXT PK | Setting key |
| value | TEXT | Setting value |
| value_type | TEXT | string, int, float, bool, json |
| category | TEXT | general, appearance, ai, security |
| is_editable | INTEGER | Boolean |

**Categories**:
- `general`: App name, version, language
- `appearance`: Theme, font size
- `ai`: AI model, confidence threshold
- `security`: Auth requirements, encryption
- `network`: Timeouts, concurrent limits
- `scanner`: Scan settings, update intervals
- `workflow`: Execution limits

---

### 9. Audit & Logs

#### `audit_logs`
Security audit trail.

| Field | Type | Description |
|-------|------|-------------|
| id | INTEGER PK | Log ID |
| action | TEXT | create_task, delete_asset, etc. |
| entity_type | TEXT | task, asset, vulnerability |
| actor_type | TEXT | user, agent, system |
| actor_name | TEXT | Who performed action |
| old_value | TEXT | Previous state |
| new_value | TEXT | New state |
| ip_address | TEXT | Client IP |

#### `system_events`
System events and errors.

| Field | Type | Description |
|-------|------|-------------|
| id | INTEGER PK | Event ID |
| event_type | TEXT | error, warning, info, success |
| source | TEXT | Component name |
| message | TEXT | Event message |
| stack_trace | TEXT | Error stack trace |

---

## Views

### `ready_tasks`
Tasks that are not blocked by dependencies.

### `blocked_tasks`
Tasks blocked by incomplete dependencies with blocker information.

### `asset_risk_summary`
Aggregated asset risk metrics including finding counts by severity.

### `recent_activity`
Union of recent tasks, findings, and workflow executions.

---

## Rust API Usage

### Using the db crate (Zed-style)

```rust
use db::{sqlez, sqlez_macros, static_connection};
use db::sqlez::domain::Domain;
use db::sqlez_macros::sql;

// Define your database domain
enum UavredDB {}

impl Domain for UavredDB {
    const NAME: &str = "uavred";
    const MIGRATIONS: &[&str] = &[
        sql!(CREATE TABLE tasks (id INTEGER PRIMARY KEY, title TEXT);),
    ];
}

// Create a static connection wrapper
pub struct Database(db::sqlez::thread_safe_connection::ThreadSafeConnection);
db::static_connection!(DB, Database, [UavredDB]);

// Use the database
let tasks = DB.select::<(i64, String)>("SELECT id, title FROM tasks")?()?;
```

### Using the data crate (Repository pattern)

```rust
use data::UavredDatabase;
use data::models::*;

// Open database (auto-detects dev/prod location)
let db = UavredDatabase::open_default()?;

// Or open at specific path
let db = UavredDatabase::open("./database/uavred.db")?;

// Create a task
let task = Task {
    title: "Analyze Flight Logs".to_string(),
    priority: TaskPriority::High,
    status: TaskStatus::Todo,
    ..Default::default()
};
let task_id = db.tasks.create(&task)?;

// Query tasks
let todo_tasks = db.tasks.list_by_status(TaskStatus::Todo)?;

// Get dashboard stats
let stats = db.get_dashboard_stats()?;
println!("Total tasks: {}", stats.total_tasks);
```

---

## Migration from tasks.db

The migration script transfers existing tasks from the old `tasks.db` to the new `uavred.db`:

```bash
# Migration is automatic when using the new Database API
# Old data is preserved in tasks.db as backup
```

Migrated data:
- ✅ 40 tasks with status, priority, and timestamps
- ✅ All task relationships preserved

---

## Security Considerations

1. **Parameterized Queries**: All SQL uses parameterized queries to prevent injection
2. **Foreign Keys**: Enabled for referential integrity
3. **WAL Mode**: Enabled for better concurrency and crash recovery
4. **Data Directory**: Database stored in OS-specific user data directory (production) or project directory (development)
5. **Access Control**: Shared database location (`~/Library/Application Support/uavred/`) follows OS user permissions

## Development Workflow

### Initial Setup

```bash
# Create local database directory
mkdir -p database

# The database will be automatically created at ./database/uavred.db
# when running the application in development mode
```

### Schema Changes

1. Modify `database/schema.sql` with your changes
2. Test locally with `./database/uavred.db`
3. Verify all features work correctly
4. Commit changes

### Promoting to Shared Database

When the schema is stable and ready for all worktrees:

```bash
# 1. Backup existing shared database (if any)
cp ~/Library/Application\ Support/uavred/uavred.db \
   ~/Library/Application\ Support/uavred/uavred.db.backup.$(date +%Y%m%d)

# 2. Copy development database to shared location
cp ./database/uavred.db ~/Library/Application\ Support/uavred/uavred.db

# 3. Update documentation to reflect the shared schema version
```

### Environment Variables

- `UAVRED_DB_PATH`: Override default database location
- `UAVRED_STATELESS`: Run without persistent database (in-memory only)

---

## Future Enhancements

- [ ] Full-text search (FTS5) for task descriptions and findings
- [ ] Database encryption for sensitive fields
- [ ] Backup and restore utilities
- [ ] Migration versioning system
