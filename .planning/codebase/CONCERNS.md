# Codebase Concerns

**Analysis Date:** 2026-01-29

## Tech Debt

### Unimplemented Scanner Modules

**Issue:** All scanner implementations are stubs returning empty results.

**Files:**
- `crates/scanner/src/network.rs` (lines 16-32)
- `crates/scanner/src/firmware.rs` (lines 17-37)
- `crates/scanner/src/protocol.rs` (lines 25-45)

**Impact:** Core security scanning functionality is non-functional. The application cannot perform actual UAV penetration testing.

**Fix approach:**
1. Implement MAVLink protocol parser for `protocol.rs`
2. Add network port scanning with async I/O for `network.rs`
3. Add firmware binary analysis (binwalk integration or similar) for `firmware.rs`

### Placeholder CVE Data

**Issue:** Hardcoded placeholder CVE identifier in vulnerability database.

**File:** `crates/core/src/vuln_db.rs` (line 45)

```rust
cve: Some("CVE-2023-XXXXX".to_string()),
```

**Impact:** Fake CVE data will confuse users and reduce credibility of security reports.

**Fix approach:** Replace with actual CVE identifiers from NVD or remove CVE field for unverified vulnerabilities.

### Unimplemented Workspace Panels

**Issue:** Three workspace panels are placeholders showing "Coming Soon".

**File:** `crates/uavred/src/workspace.rs` (lines 215-242)

```rust
AppView::Images => {
    // TODO: 实现 Images 面板
    div().child(Label::new("Images - Coming Soon"))
}
AppView::Devices => {
    // TODO: 实现 Devices 面板
    div().child(Label::new("Devices - Coming Soon"))
}
AppView::Settings => {
    // TODO: 实现 Settings 面板
    div().child(Label::new("Settings - Coming Soon"))
}
```

**Impact:** Incomplete user experience; users cannot access image analysis, hardware device management, or application settings.

### Agent System Not Connected

**Issue:** Agent scheduler has TODO comment but no actual task execution.

**File:** `crates/agent/src/scheduler.rs` (line 38)

```rust
// TODO: Actually execute the task
```

**Impact:** The autonomous agent system is non-operational despite UI showing "AI Active" status.

**Fix approach:** Connect scheduler to TaskExecutor and implement actual task dispatch.

### Task Executor Simulation Only

**Issue:** Task execution simulates work with sleep instead of performing actual operations.

**File:** `crates/agent/src/executor.rs` (lines 23-34)

```rust
pub async fn execute(&self, task: Task) -> Result<TaskResult> {
    tracing::info!("Executing task: {}", task.name);
    // Simulate task execution
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    Ok(TaskResult {
        task_name: task.name.clone(),
        success: true,
        output: format!("Task {} completed successfully", task.name),
    })
}
```

**Impact:** Tasks appear to complete but no actual security testing occurs.

## Known Bugs

### Unwrap on Optional CVSS Score

**Issue:** Potential panic when CVSS score is None.

**File:** `crates/vulns_ui/src/panels/cve_info_panel.rs` (line 107)

```rust
.when(vuln.cvss_score.is_some(), |s| {
    s.child(self.render_cvss_card(vuln.cvss_score.unwrap()))
})
```

**Impact:** Application will panic if a vulnerability without CVSS score is selected.

**Fix approach:** Use `if let Some(score) = vuln.cvss_score` pattern instead of `unwrap()`.

### Unwrap on Optional CWE ID

**Issue:** Potential panic when CWE ID is None.

**File:** `crates/vulns_ui/src/panels/cve_info_panel.rs` (line 154)

```rust
.child(vuln.cwe_id.clone().unwrap())
```

**Impact:** Application will panic if a vulnerability without CWE ID is displayed.

### String Slicing Panic Risk

**Issue:** Direct string slicing on detection time without bounds checking.

**File:** `crates/vulns_ui/src/panels/cve_info_panel.rs` (lines 121, 127)

```rust
.child(&finding.detected_at[..10])  // Date portion
.child(&finding.detected_at[11..])  // Time portion
```

**Impact:** Panic if datetime string is malformed or shorter than expected.

### Mutex Poisoning Risk

**Issue:** Database operations use `lock().unwrap()` which panics on poisoned mutex.

**Files:**
- `crates/data/src/task_store.rs` (multiple locations)
- `crates/data/src/vuln_store.rs` (line 37)
- `crates/data/src/repository.rs` (multiple locations)

```rust
let db = self.db.lock().unwrap();
```

**Impact:** If a thread panics while holding the mutex, subsequent lock attempts will panic.

**Fix approach:** Use `lock().unwrap_or_else(|e| e.into_inner())` to recover from poisoned mutexes.

## Security Considerations

### Unsafe Code in sqlez Crate

**Issue:** The `sqlez` crate contains extensive unsafe code for SQLite FFI bindings.

**Files:**
- `crates/sqlez/src/connection.rs` (lines 18, 30, 72, 122, 136, 145, 159, 169, 175, 197, 204, 273)
- `crates/sqlez/src/statement.rs` (multiple locations)
- `crates/sqlez/src/thread_safe_connection.rs` (lines 38-39)
- `crates/sqlez/src/migrations.rs` (line 18)

**Impact:** Memory safety violations could lead to crashes or security vulnerabilities. The workspace lints forbid unsafe_code, but sqlez is exempt as a low-level database wrapper.

**Current mitigation:** Uses Zed's battle-tested sqlez implementation.

**Recommendations:**
- Audit unsafe blocks for correct lifetime management
- Consider rusqlite as a safer alternative
- Add extensive tests for edge cases

### SQL Injection Risk in Migration Queries

**Issue:** Dynamic SQL construction in migration cleanup.

**File:** `crates/sqlez/src/migrations.rs` (lines 134-140)

```rust
self.exec(&format!(
    "DELETE FROM {child_table} WHERE {child_key} IS NOT NULL..."
))
```

**Impact:** While currently internal use only, this pattern could be exploited if table names are ever user-controlled.

**Current mitigation:** Table names come from SQLite's own schema introspection, not user input.

### No Input Validation on Task Data

**Issue:** Task creation accepts arbitrary strings without sanitization.

**File:** `crates/data/src/repository.rs` (lines 93-105)

**Impact:** Potential for XSS if task data is rendered in web contexts, or SQL injection if not properly parameterized.

**Current mitigation:** Uses parameterized queries which prevents SQL injection.

## Performance Bottlenecks

### Synchronous Database Access

**Issue:** All database operations block the async runtime.

**Files:**
- `crates/data/src/task_store.rs` (lines 46, 96, 111, 128, etc.)
- `crates/data/src/vuln_store.rs` (line 36)

```rust
let db = self.db.lock().unwrap();
let tasks = db.tasks().list_by_status(status)?;
```

**Impact:** Database queries block the GPUI main thread, causing UI freezes during large operations.

**Improvement path:**
1. Use `cx.spawn()` for database operations
2. Implement pagination for large result sets
3. Add async database connection pool

### Loading All Tasks Into Memory

**Issue:** TaskStore loads all tasks regardless of need.

**File:** `crates/data/src/task_store.rs` (lines 45-62)

```rust
pub fn load_all_tasks(&mut self, cx: &mut Context<Self>) -> anyhow::Result<()> {
    let mut all_tasks = Vec::new();
    for status in [TaskStatus::Todo, TaskStatus::InProgress, ...] {
        let tasks = db.tasks().list_by_status(status)?;
        all_tasks.extend(tasks);
    }
    self.tasks = all_tasks;
    // ...
}
```

**Impact:** Memory usage grows linearly with task count; no upper bound.

**Scaling path:** Implement virtualized lists and lazy loading with pagination.

### Mock Repository Generates 200+ Test Nodes

**Issue:** Mock asset repository creates 200 synthetic nodes for testing.

**File:** `crates/assets_ui/src/repository/mock_repository.rs` (lines 148-183)

**Impact:** Development builds carry unnecessary test data generation overhead.

**Fix approach:** Move test data generation behind a feature flag or dev-only configuration.

## Fragile Areas

### Global Store Initialization Pattern

**Issue:** Panic-prone global access pattern for stores.

**Files:**
- `crates/data/src/task_store.rs` (lines 212-218)
- `crates/data/src/vuln_store.rs` (lines 83-88)

```rust
pub fn global(cx: &mut App) -> Entity<Self> {
    if cx.has_global::<GlobalTaskStore>() {
        return cx.global::<GlobalTaskStore>().0.clone();
    }
    panic!("TaskStore::global() called but no global TaskStore exists...")
}
```

**Why fragile:** Any code calling `global()` before initialization crashes the application.

**Safe modification:** Always call `init_task_store()` before `TaskStore::global()`, or return `Option<Entity<Self>>`.

### Complex Type Conversions in Repository

**Issue:** Repository methods use complex tuple types prone to breakage.

**File:** `crates/data/src/repository.rs` (lines 121-123, 138-140)

```rust
let rows = conn.select_bound::<i64, (i64, String, String, String, String, String, String,
                           String, Option<i64>, String, String, Option<String>,
                           Option<String>, Option<String>, String, String, String, String)>(sql)?(id)?;
```

**Why fragile:** Adding a column requires updating all tuple type signatures.

**Safe modification:** Use struct-based row mapping with derive macros.

### Manual SQL Schema Management

**Issue:** Schema defined in raw SQL file without migration versioning.

**File:** `database/schema.sql`

**Why fragile:** Schema changes require manual SQL editing with no rollback capability.

**Test coverage:** Limited integration tests for database operations.

## Scaling Limits

### SQLite Concurrent Write Limit

**Current capacity:** Single writer due to SQLite's WAL mode with single write queue.

**Limit:** `crates/sqlez/src/thread_safe_connection.rs` serializes all writes through a single background thread.

**Scaling path:**
1. Shard data across multiple database files
2. Implement write batching
3. Consider migration to PostgreSQL for high-write scenarios

### In-Memory Vulnerability Storage

**Current capacity:** `crates/data/src/vuln_store.rs` loads all vulnerabilities into `Vec<Vulnerability>`.

**Limit:** No pagination; all vulns loaded at startup.

**Scaling path:** Implement lazy loading with database-backed pagination.

## Dependencies at Risk

### GPUI from Git Without Version Pin

**Issue:** `Cargo.toml` references GPUI from git without a specific commit or tag.

**File:** `/Users/fk/Devlopment/uavred/Cargo.toml` (line 32)

```toml
gpui = { git = "https://github.com/zed-industries/zed" }
```

**Risk:** API breaking changes in GPUI can break the build without warning.

**Impact:** Build reproducibility issues; unexpected breakages.

**Migration plan:** Pin to a specific commit hash or tagged release.

### Local gpui-component Path Dependency

**Issue:** `gpui-component` is referenced via local path.

**File:** `/Users/fk/Devlopment/uavred/Cargo.toml` (line 33)

```toml
gpui-component = { path = "src/gpui-component/crates/ui", package = "gpui-component" }
```

**Risk:** Changes in local component library can break multiple UI crates simultaneously.

## Missing Critical Features

### No Test Suite

**Problem:** No test files exist in the main crates.

**Blocks:** Safe refactoring, CI/CD integration, regression prevention.

**Files to create:**
- `crates/data/src/tests/` - Database operation tests
- `crates/scanner/src/tests/` - Scanner logic tests
- `crates/agent/src/tests/` - Agent scheduling tests

### No Error Recovery for Database Operations

**Problem:** All database errors are propagated with `?` but no retry or recovery logic.

**Files:** All repository methods in `crates/data/src/repository.rs`

**Impact:** Transient SQLite errors (busy, locked) cause permanent operation failures.

### No Audit Logging

**Problem:** Security testing tool lacks audit trail for actions taken.

**Impact:** Cannot track who performed what security tests when.

**Files to modify:**
- `crates/data/src/repository.rs` - Add audit logging hooks
- `crates/agent/src/executor.rs` - Log task execution

## Test Coverage Gaps

### No Unit Tests for Core Business Logic

**What's not tested:**
- Task state machine transitions
- Vulnerability severity calculations
- Asset topology graph operations
- Scanner result parsing

**Files:**
- `crates/core/src/task.rs`
- `crates/core/src/vuln_db.rs`
- `crates/scanner/src/*.rs`

**Risk:** Logic errors in security testing workflows go undetected.

**Priority:** High - Security tools require high confidence in correctness.

### No Integration Tests for Database Layer

**What's not tested:**
- Concurrent access patterns
- Migration rollback scenarios
- Connection failure recovery

**Files:**
- `crates/data/src/repository.rs`
- `crates/sqlez/src/connection.rs`

**Risk:** Data corruption or loss in production scenarios.

### UI Components Untested

**What's not tested:**
- GPUI component rendering
- Event handling
- State synchronization

**Files:** All `*_ui` crates

**Risk:** UI regressions only discovered through manual testing.

---

*Concerns audit: 2026-01-29*
