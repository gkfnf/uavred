# Technology Stack

**Analysis Date:** 2026-01-29

## Languages

**Primary:**
- **Rust** (Edition 2024 for most crates, 2021 for some legacy crates) - 100% of application code
- **Chinese** - Comments and documentation (code uses English identifiers)

**Configuration:**
- **TOML** - Cargo manifests and configuration
- **SQL** - Database migrations and queries

## Runtime

**Environment:**
- **Native desktop application** - No web runtime
- **GPUI** - GPU-accelerated UI framework from Zed editor
- **Tokio** - Async runtime for background operations

**Package Manager:**
- **Cargo** - Rust's built-in package manager
- **Lockfile:** `Cargo.lock` present and committed

## Frameworks

**Core UI Framework:**
- **GPUI** (from Zed Industries) - GPU-accelerated immediate-mode UI framework
  - Source: `git = "https://github.com/zed-industries/zed"`
  - Pattern: Entity-based reactive UI with `Render` trait
  - Window management: Native OS windows with custom titlebars

**UI Component Library:**
- **gpui-component** (v0.5.0) - 60+ pre-built UI components
  - Location: `src/gpui-component/` (git submodule)
  - Provides: `Root`, `Button`, `Label`, `h_flex`, `v_flex`, tables, dialogs, etc.

**Async Runtime:**
- **Tokio** (v1.35) - Full feature set enabled
  - Used in: `crates/agent/` for task execution
  - Pattern: `cx.spawn()` for background tasks with GPUI integration

**Alternative Async:**
- **smol** (v2) - Used in `sqlez` for database operations

## Key Dependencies

**Critical Infrastructure:**
- `libsqlite3-sys` (v0.30) - SQLite C bindings
- `sqlez` (internal) - Zed's SQLite wrapper with async support
- `anyhow` (v1) - Error handling throughout codebase
- `serde` + `serde_json` - Serialization for all data models
- `uuid` (v1.6) - Unique identifiers for tasks/agents
- `chrono` (v0.4) - Date/time handling
- `parking_lot` (v0.12) - Synchronization primitives

**UI/Graphics:**
- `gpui-macros` (v0.2.2) - Proc macros for GPUI
- `raw-window-handle` (v0.6.2) - Window handle interop
- `taffy` - Layout engine (via GPUI)

**Development:**
- `tracing` + `tracing-subscriber` - Structured logging
- `notify` (v7.0.0) - File system watching
- `dirs` (v5.0) - Platform-appropriate data directories

**Internationalization:**
- `rust-i18n` (v3) - i18n framework (configured but minimal usage)

**LSP Support:**
- `lsp-types` (v0.97.0) - Language server protocol types

## Configuration

**Environment:**
- No `.env` file detected
- Configuration via code constants in `crates/ui/src/theme.rs`
- Database path: Platform data directory (`~/.local/share/uavred/` on Linux, `~/Library/Application Support/uavred/` on macOS)

**Build Configuration:**
- Workspace root: `/Users/fk/Devlopment/uavred/Cargo.toml`
- 18 crates in workspace (see STRUCTURE.md)
- Edition 2024 for new crates, 2021 for legacy
- Lint: `unsafe_code = "forbid"` at workspace level

**Development Tools:**
- `cargo fmt` - Code formatting
- `cargo clippy` - Linting (warnings as errors in CI)
- `cargo watch` - Auto-reload development
- Makefile targets: `build`, `run`, `test`, `fmt`, `clippy`, `dev`, `release`

## Platform Requirements

**Development:**
- Rust toolchain (2024 edition support required)
- macOS or Linux (GPUI platform support)
- SQLite development libraries (for `libsqlite3-sys`)

**Production:**
- Desktop deployment only
- GPU support required (GPUI uses GPU acceleration)
- Local SQLite database (no external DB server needed)

## Crate Dependencies Flow

```
uavred (binary entry)
  ├─> dashboard_ui, assets_ui, vulns_ui, etc. (UI panels)
  ├─> workspace_ui (sidebar)
  ├─> data (TaskStore, VulnStore)
  └─> workspace (shared types)

data
  ├─> sqlez (SQLite wrapper)
  ├─> workspace (TaskData, etc.)
  └─> db (legacy database layer)

agent
  ├─> core (Task, Vulnerability)
  ├─> tokio (async runtime)
  └─> scanner (network, protocol, firmware)

All UI crates
  ├─> gpui (UI framework)
  ├─> gpui-component (components)
  └─> ui (theme, events, actions)
```

## Notable Patterns

**No Unsafe Code:** Workspace-level lint forbids unsafe code (sqlez crate explicitly allows it for SQLite FFI).

**Git Submodules:**
- `src/gpui-component/` - UI component library

**Internal Dependencies:**
- Heavy use of path dependencies for internal crates
- Workspace-level dependency declarations for external crates

---

*Stack analysis: 2026-01-29*
