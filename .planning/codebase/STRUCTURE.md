# Codebase Structure

**Analysis Date:** 2026-01-29

## Directory Layout

```
/Users/fk/Devlopment/uavred/
├── Cargo.toml                    # Workspace manifest
├── Cargo.lock                    # Dependency lockfile
├── CLAUDE.md                     # Project instructions for Claude Code
├── Makefile                      # Build automation
├── config.toml                   # Application configuration
├── database/                     # SQLite database files
│   ├── uavred.db                # Main database
│   └── schema.sql               # Database schema
├── crates/                       # Main application crates
│   ├── uavred/                  # Application entry point
│   ├── workspace/               # Shared workspace types
│   ├── workspace_ui/            # Workspace-level UI components
│   ├── dashboard_ui/            # Dashboard panel (kanban + findings)
│   ├── assets_ui/               # Asset topology panel
│   ├── vulns_ui/                # Vulnerability management panel
│   ├── traffic_ui/              # Traffic analysis panel
│   ├── flows_ui/                # Workflow panel
│   ├── kanban_ui/               # Reusable kanban components
│   ├── monitor_ui/              # Container monitoring panel
│   ├── settings_ui/             # Settings panel
│   ├── devices_ui/              # Hardware devices panel
│   ├── scan_ui/                 # Scan configuration panel
│   ├── data/                    # Data layer (models, DB, stores)
│   ├── db/                      # Database utilities (kvp, query)
│   ├── sqlez/                   # SQLite wrapper (Zed's sqlez)
│   ├── sqlez_macros/            # Macros for sqlez
│   ├── ui/                      # Shared UI (theme, events, actions)
│   ├── core/                    # Business logic (task, vuln_db)
│   ├── agent/                   # Agent system (scheduler, executor)
│   └── scanner/                 # Security scanners
├── docs/                        # Documentation
│   ├── ARCHITECTURE.md          # System architecture
│   ├── UI_ARCHITECTURE.md       # UI patterns
│   └── ZED_WORKSPACE_ARCHITECTURE.md  # Zed patterns reference
├── src/                         # External dependencies (subtrees)
│   ├── gpui-component/          # UI component library (60+ components)
│   └── zed/                     # Zed editor source (GPUI)
└── .planning/codebase/          # This directory
```

## Directory Purposes

### Application Crates (`crates/uavred/`)

- **Purpose**: Application entry point and workspace coordinator
- **Contains**: `main.rs`, `workspace.rs`
- **Key files**:
  - `src/main.rs`: Application initialization, window creation
  - `src/workspace.rs`: Top-level workspace managing all panels

### Workspace Types (`crates/workspace/`)

- **Purpose**: Shared types across all crates
- **Contains**: `AppView` enum, `TaskData`, `VulnFilter`, `DashboardView`
- **Key file**: `src/lib.rs`

### Feature UI Crates (`crates/*_ui/`)

Each feature panel follows this structure:
```
crates/{feature}_ui/
├── Cargo.toml
└── src/
    ├── lib.rs           # Panel struct + Render impl
    ├── components.rs    # (optional) Shared components
    └── {submodules}/    # (optional) Feature-specific modules
```

**Implemented Panels:**
- `dashboard_ui/`: Mission control kanban + findings view
- `assets_ui/`: Network topology + asset detail cards
- `vulns_ui/`: Vulnerability list + detail + CVE info
- `traffic_ui/`: Network traffic analysis
- `flows_ui/`: Workflow execution

**Stub Panels (TODO):**
- `devices_ui/`: Hardware device management
- `settings_ui/`: Application settings
- `scan_ui/`: Scan configuration
- `monitor_ui/`: Container monitoring

### Data Layer (`crates/data/`)

- **Purpose**: Models, database, repositories, stores
- **Contains**:
  - `src/models.rs`: All data models (~1069 lines)
  - `src/repository.rs`: Database access layer
  - `src/task_store.rs`: Task state management
  - `src/vuln_store.rs`: Vulnerability state management
  - `src/uavred_db.rs`: Database initialization

### Shared UI (`crates/ui/`)

- **Purpose**: Cross-cutting UI concerns
- **Contains**:
  - `src/theme.rs`: Color constants, spacing, sizing
  - `src/events.rs`: Event definitions
  - `src/actions.rs`: Action definitions

### Infrastructure (`crates/sqlez/`, `crates/sqlez_macros/`, `crates/db/`)

- **Purpose**: Database infrastructure
- **sqlez/**: Zed's SQLite wrapper
- **sqlez_macros/**: Procedural macros for sqlez
- **db/**: Key-value store and query utilities

### External Dependencies (`src/`)

- **gpui-component/**: UI component library (local fork)
- **zed/**: Zed editor source (for GPUI framework)

## Key File Locations

### Entry Points

- `crates/uavred/src/main.rs`: Application entry
- `crates/uavred/src/workspace.rs`: Workspace coordinator

### Configuration

- `Cargo.toml`: Workspace members, dependencies
- `config.toml`: Runtime configuration
- `database/schema.sql`: Database schema

### Core Models

- `crates/data/src/models.rs`: Task, Asset, Vuln, Traffic, Flow, Device models
- `crates/workspace/src/lib.rs`: Shared enums (AppView, TaskData, VulnFilter)

### State Management

- `crates/data/src/task_store.rs`: TaskStore with global access
- `crates/data/src/vuln_store.rs`: VulnStore with global access

### Theme/Style

- `crates/ui/src/theme.rs`: All color/spacing constants

### Events/Actions

- `crates/ui/src/events.rs`: WorkspaceEvent, DashboardEvent, AssetsEvent, VulnsEvent
- `crates/ui/src/actions.rs`: ActivateView, SelectTask, SetVulnFilter

## Naming Conventions

### Files

- **Panel files**: `{feature}_panel.rs` or `lib.rs` in `{feature}_ui/src/`
- **Component files**: `{component}.rs` (e.g., `task_card.rs`, `topology_canvas.rs`)
- **Model files**: `models.rs` (contains all models for a domain)
- **Store files**: `{domain}_store.rs` (e.g., `task_store.rs`, `vuln_store.rs`)

### Directories

- **Crate names**: `snake_case` (e.g., `dashboard_ui`, `assets_ui`)
- **Feature folders**: Mirror crate structure (e.g., `asset_detail_panel/`, `topology_canvas/`)
- **Component folders**: Plural (e.g., `components/`, `cards/`, `panels/`)

### Struct/Type Names

- **Panels**: `*Panel` suffix (e.g., `DashboardPanel`, `AssetsPanel`)
- **Events**: `*Event` suffix (e.g., `WorkspaceEvent`, `AssetSelectedEvent`)
- **Stores**: `*Store` suffix (e.g., `TaskStore`, `VulnStore`)
- **Models**: Domain names (e.g., `Task`, `Asset`, `Vulnerability`, `Finding`)

## Where to Add New Code

### New Feature Panel

1. **Create crate**: `crates/new_feature_ui/`
   ```
   crates/new_feature_ui/
   ├── Cargo.toml
   └── src/
       └── lib.rs
   ```

2. **Add to workspace**: Edit root `Cargo.toml` members list

3. **Implement panel**: Follow pattern in `crates/vulns_ui/src/lib.rs`
   ```rust
   pub struct NewFeaturePanel {
       // Entity references to stores
   }

   impl Render for NewFeaturePanel {
       fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
           // Implementation
       }
   }
   ```

4. **Add to AppView**: Edit `crates/workspace/src/lib.rs`

5. **Add to Workspace**: Edit `crates/uavred/src/workspace.rs`
   - Add field: `new_feature_panel: Option<Entity<NewFeaturePanel>>`
   - Add getter: `get_or_create_new_feature_panel()`
   - Add to render: match arm in `render_main_content()`

### New Data Model

1. **Add to**: `crates/data/src/models.rs`
2. **Follow pattern**: Include Serialize/Deserialize, derive Debug/Clone
3. **Add conversions**: From/Into traits for related types
4. **Add repository methods**: In `crates/data/src/repository.rs` if needed

### New Store

1. **Create file**: `crates/data/src/{domain}_store.rs`
2. **Follow pattern**: See `crates/data/src/vuln_store.rs`
3. **Include**: Global registration, EventEmitter, database integration
4. **Export in**: `crates/data/src/lib.rs`

### New Shared Component

1. **If used by multiple panels**: Add to appropriate `*_ui` crate or create new shared crate
2. **If panel-specific**: Add to panel's `components/` subdirectory
3. **Follow pattern**: Return `impl IntoElement`, accept callbacks as parameters

### New Event

1. **Add to**: `crates/ui/src/events.rs`
2. **Follow pattern**: Derive Debug + Clone, include relevant data
3. **Emit**: `cx.emit(MyEvent::Variant)`
4. **Subscribe**: `cx.subscribe(&source, handler)`

### New Action

1. **Add to**: `crates/ui/src/actions.rs`
2. **Follow pattern**: Derive Action, use `#[action(namespace = workspace, no_json)]`
3. **Dispatch**: `cx.dispatch_action(&MyAction)`
4. **Handle**: `.on_action(|this, action, window, cx| { ... })`

## Special Directories

### `src/gpui-component/`

- **Purpose**: UI component library (forked/modified)
- **Contains**: 60+ UI components (Button, Input, Table, etc.)
- **Key**: `crates/ui/` re-exports from here
- **Generated**: No (source code)
- **Committed**: Yes

### `src/zed/`

- **Purpose**: Zed editor source (for GPUI framework)
- **Contains**: GPUI framework, sqlez, other Zed crates
- **Used by**: Application for GPUI types
- **Generated**: No (source code)
- **Committed**: Yes (subtree)

### `database/`

- **Purpose**: SQLite database storage
- **Contains**: `uavred.db`, `schema.sql`
- **Generated**: Database file is generated, schema.sql is source
- **Committed**: No (database files in .gitignore)

### `.planning/`

- **Purpose**: Planning documents for Claude Code
- **Contains**: This codebase analysis
- **Generated**: No
- **Committed**: No (in .gitignore)

---

*Structure analysis: 2026-01-29*
