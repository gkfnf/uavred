# External Integrations

**Analysis Date:** 2026-01-29

## APIs & External Services

**No External APIs Configured:**
The codebase has no active external API integrations. The `reqwest` dependency is declared in workspace but not actively used in the main application code.

**Note on reqwest:**
- Declared in `src/gpui-component/Cargo.toml` with Zed's fork
- Features: `charset`, `http2`, `macos-system-configuration`, `multipart`, `rustls-tls-native-roots`, `socks`, `stream`
- **Current usage:** None in production code paths
- **Potential use:** Future CVE database lookups, threat intelligence feeds

## Data Storage

**Primary Database:**
- **SQLite** (via `libsqlite3-sys` v0.30 and custom `sqlez` wrapper)
- **Location:** Local file-based database
- **Default path:** Platform data directory
  - macOS: `~/Library/Application Support/uavred/uavred.db`
  - Linux: `~/.local/share/uavred/uavred.db`
  - Windows: `%APPDATA%/uavred/uavred.db`

**Database Access Pattern:**
```rust
// From crates/data/src/database.rs
let data_dir = dirs::data_dir()?.join("uavred");
let db_path = data_dir.join("tasks.db");
let connection = Connection::open_file(&db_path.to_string_lossy());
```

**Schema Management:**
- Manual table creation in `TasksDatabase::new()`
- No migration framework currently in use
- Tables: `tasks` (with id, title, task_type, priority, status, timestamps)

**File Storage:**
- **Local filesystem only** - No cloud storage integration
- Firmware analysis reads from local file paths (`std::path::PathBuf`)

**Caching:**
- **In-memory stores** (`crates/data/src/memory.rs`) for runtime state
- No external cache (Redis/Memcached) - not needed for desktop app

## Authentication & Identity

**Auth Provider:**
- **None** - Single-user desktop application
- No login system, no identity provider integration
- No OAuth, SSO, or API key management

**Security Model:**
- Application runs with user's OS permissions
- Database is local and unencrypted (SQLite default)
- No network authentication required

## Monitoring & Observability

**Error Tracking:**
- **None** - No Sentry, Rollbar, or similar external error tracking

**Logging:**
- **tracing** crate with `tracing-subscriber`
- Output: Console/terminal only
- Level: Configured to `INFO` in `main.rs`
- No structured log shipping to external systems

**Metrics:**
- **None** - No Prometheus, StatsD, or cloud monitoring

## CI/CD & Deployment

**Hosting:**
- **Desktop application** - No server deployment
- Distribution: Binary releases (not configured)

**CI Pipeline:**
- **None detected** - No `.github/workflows/`, `.gitlab-ci.yml`, etc.
- Build process: Local `cargo build` only

**Version Management:**
- Workspace version: `0.1.0` (all crates)
- No automated versioning or release management

## Environment Configuration

**Required Environment Variables:**
- **None** - Application does not use environment variables for configuration

**Configuration Approach:**
- Compile-time constants in `crates/ui/src/theme.rs`
- Hardcoded database paths using `dirs` crate
- No external configuration files (JSON/YAML/TOML configs)

**Secrets Management:**
- **Not applicable** - No API keys, database passwords, or external credentials

## Webhooks & Callbacks

**Incoming Webhooks:**
- **None** - No HTTP server or webhook endpoints

**Outgoing Webhooks:**
- **None** - No callbacks to external systems

## Network Communication

**Current State:**
- No active network clients in production code
- Scanner modules (`network.rs`, `protocol.rs`, `firmware.rs`) contain TODO stubs only

**Planned/Potential Network Operations:**
```rust
// From crates/scanner/src/network.rs - NOT IMPLEMENTED
pub async fn scan(&self) -> Result<ScanResult> {
    // TODO: Implement actual network scanning
    // - Port scanning
    // - Service detection
    // - UAV protocol detection (MAVLink, DJI, etc.)
}
```

**Protocols for Future Implementation:**
- MAVLink (UDP port 14550)
- DJI SDK
- ArduPilot/PX4 protocols
- WiFi scanning (platform-specific)

## Data Sources

**Vulnerability Database:**
- **Embedded, static data** in `crates/core/src/vuln_db.rs`
- Hardcoded UAV-specific vulnerabilities (UAV-001, UAV-002, UAV-003)
- No CVE API integration or NVD feeds

**CVE References:**
- Placeholder CVEs in vulnerability definitions
- No live CVE lookup capability

## Integration Summary

| Category | Integration | Status |
|----------|-------------|--------|
| External APIs | None | N/A |
| Database | SQLite (local) | Active |
| File Storage | Local filesystem | Active |
| Auth | None (single-user) | N/A |
| Error Tracking | None | N/A |
| Logging | Console only | Active |
| CI/CD | None | N/A |
| Webhooks | None | N/A |
| Network | Placeholder only | TODO |

## Future Integration Points

**Potential Additions:**
1. **CVE Database API** - NVD or VulDB for live vulnerability data
2. **Threat Intelligence** - MISP or similar feeds for UAV-specific threats
3. **Firmware Databases** - Binary analysis services
4. **Update Mechanism** - Auto-updater with signature verification
5. **Export/Reporting** - PDF generation, SIEM integration

---

*Integration audit: 2026-01-29*
