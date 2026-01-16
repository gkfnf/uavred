# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

UAVRed is a desktop penetration testing tool for UAV (drone) ecosystems, built with **Rust + GPUI + gpui-component**. 
It provides autonomous agent-driven security testing with a modern, high-performance desktop UI.

**Tech Stack**: Rust (2024 edition), GPUI (from Zed), gpui-component (60+ UI components), SQLite (via sqlez), Tokio async runtime

**Language**: The codebase contains Chinese comments and documentation, but code itself (variable names, function names) uses English.

## Build & Development Commands

```bash
# Build
cargo build                    # Debug build
cargo build --release          # Release build
make build                     # Same as cargo build

# Run
cargo run                      # Start application
make run

# Development with auto-reload
cargo watch -x run            # Requires cargo-watch
make dev                      # Same as above

# Testing
cargo test                    # Run all tests
cargo test --package agent    # Test specific crate
make test

# Code Quality
cargo clippy -- -D warnings   # Lint (treat warnings as errors)
cargo fmt                     # Format code
cargo check                   # Fast compile check without binary
make clippy                   # Lint via Makefile
make fmt                      # Format via Makefile

# All checks at once
make all                      # Runs fmt + clippy + test + build

# Clean
cargo clean                   # Remove build artifacts
make clean
```

## Architecture Overview

UAVRed follows **Zed editor's workspace architecture patterns**. Understanding this is critical:

### Workspace Pattern (Zed-inspired)

```
Workspace (top-level coordinator)
  └─> Panels (feature-specific views)
       └─> Components (reusable UI elements from gpui-component)
```

**Key file**: `crates/uavred/src/workspace.rs`

The Workspace manages:
- Active view switching (Dashboard, Assets, Vulns, Traffic, Flows, Devices, Monitor, Settings)
- Lazy panel initialization (panels created only when first accessed)
- Event emission for view changes
- Global state coordination

### Multi-Crate Structure

```
crates/
├── uavred/          # Application entry point (main.rs, workspace.rs)
├── workspace/       # Shared workspace types (AppView enum, etc.)
├── core/            # Business logic (Task, VulnDatabase)
├── agent/           # Agent system (Scheduler, Executor)
├── scanner/         # Security scanners (network, protocol, firmware)
├── data/            # Data layer (models, database, repositories, stores)
├── ui/              # Shared UI (theme constants, events, actions)
├── *_ui/            # Feature panels (dashboard_ui, assets_ui, vulns_ui, etc.)
└── workspace_ui/    # Workspace-level UI components (sidebar)
```

**Dependencies flow**: UI → Agent → Core → Data → Scanner

### Three-Layer Architecture

1. **Presentation Layer** (UI Components)
   - Render data from stores
   - Emit user actions via GPUI's action system
   - Auto-update on state changes via `cx.observe()`

2. **Application State Layer** (Stores)
   - `TaskStore`: Task management (SQLite-backed via sqlez)
   - Global state using GPUI's `Global` trait
   - Event emission via `EventEmitter` trait

3. **Data Layer** (`crates/data`)
   - `models.rs`: Rich data models (TaskData, VulnData, TrafficEntry, FlowNode, etc.)
   - `database.rs`: SQLite operations using sqlez
   - `repository.rs`: Abstract data access interfaces
   - `memory.rs`: In-memory storage for non-persistent data

## Key GPUI Patterns to Follow

### Entity Pattern
Use `Entity<T>` for stateful components:

```rust
pub struct MyPanel {
    task_store: Entity<TaskStore>,  // Reference to shared store
    _subscriptions: Vec<Subscription>,  // Auto-cleanup on drop
}

// Access in methods:
let tasks = self.task_store.read(cx).tasks();
self.task_store.update(cx, |store, cx| {
    store.add_task(task, cx);
});
```

### Root Component Requirement
Every window **must** wrap content in `Root`:

```rust
use gpui_component::Root;

cx.new(|cx| Root::new(view, window, cx))
```

`Root` provides dialog layer, notification layer, and theme context.

### Render Trait Implementation
All view components implement `Render`:

```rust
impl Render for MyPanel {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .child(self.render_header(cx))
            .child(self.render_content(cx))
    }
}
```

### Event-Driven Communication
Define events in `crates/ui/src/events.rs`:

```rust
pub enum WorkspaceEvent {
    ViewChanged(AppView),
    TaskSelected(Option<usize>),
}

impl EventEmitter<WorkspaceEvent> for Workspace {}

// Emit:
cx.emit(WorkspaceEvent::ViewChanged(AppView::Dashboard));

// Subscribe:
cx.subscribe(&workspace, |this, _workspace, event, cx| {
    match event {
        WorkspaceEvent::ViewChanged(view) => { /* ... */ }
    }
})
```

### Action System
Define actions in `crates/ui/src/actions.rs`:

```rust
#[derive(Clone, PartialEq, Debug, Action, Deserialize)]
#[action(namespace = workspace, no_json)]
pub struct ActivateView(pub AppView);

// Dispatch:
cx.dispatch_action(&ActivateView(AppView::Dashboard));

// Handle:
.on_action(|this: &mut Self, action: &ActivateView, window, cx| {
    this.active_view = action.0;
});
```

### Async Task Pattern
Background operations use `cx.spawn()`:

```rust
cx.spawn(async move |this, cx| {
    let result = async_operation().await?;
    this.update(cx, |this, cx| {
        this.state = result;
        cx.notify();  // Trigger re-render
    })?;
    Ok::<_, anyhow::Error>(())
}).detach_and_log_err(cx);
```

## Agent System Architecture

The agent system provides autonomous security testing:

```
TaskStore (UI creates tasks)
    ↓
Scheduler (agent/scheduler.rs) - Assigns tasks to agents
    ↓
Executor (agent/executor.rs) - Runs tasks asynchronously
    ↓
Scanner Modules (scanner/network.rs, protocol.rs, firmware.rs)
    ↓
Results → TaskStore → UI updates reactively
```

**Key integration**: `TaskStore` acts as the bridge between UI and agent system.

## Important Conventions

### Code Safety
```toml
[workspace.lints]
rust = { unsafe_code = "forbid" }  # No unsafe code allowed
```

### Naming Conventions
- **Panels**: `*Panel` suffix (e.g., `DashboardPanel`)
- **Events**: `*Event` enums
- **Actions**: Struct types implementing `Action` trait
- **Entities**: Use `Entity<T>` for stateful components

### Theme Constants
All UI styling uses constants from `crates/ui/src/theme.rs`:

```rust
pub const BG_PRIMARY: u32 = 0xf5f5f5;
pub const TEXT_PRIMARY: u32 = 0x1f2937;
pub const SEVERITY_CRITICAL: u32 = 0xef4444;
pub const BORDER_RADIUS: Pixels = px(6.0);
pub const PADDING_MD: Pixels = px(12.0);
```

**Do not hardcode colors or spacing** - use theme constants.

### Error Handling
Use `anyhow::Result` consistently:

```rust
pub async fn execute(&self, task: Task) -> Result<TaskResult> {
    // ... implementation
}
```

### Logging
Use `tracing` crate:

```rust
tracing::info!("Executing task: {}", task.name);
tracing::error!("Failed to connect: {:?}", error);
```

## Data Models (`crates/data/src/models.rs`)

Rich, comprehensive models (~988 lines):
- **TaskData**: Task management with status tracking
- **VulnData**: Vulnerability info (CVE, CVSS, MITRE ATT&CK)
- **TrafficEntry**: Network traffic with anomaly detection
- **FlowNode**: Workflow DAG nodes with execution metrics
- **DeviceInfo**: Hardware device information
- **AssetNode**: Network asset topology data
- **ContainerStatus**: Docker container monitoring

These models are shared across all crates.

## Panel Creation Pattern

When adding a new panel:

1. Create crate: `crates/new_feature_ui/`
2. Define panel struct with `Entity` references
3. Implement `Render` trait
4. Add to `workspace/src/lib.rs` AppView enum
5. Add lazy initialization to `workspace.rs`
6. Register in workspace's render method
7. Add to sidebar in `workspace_ui/src/sidebar.rs`

## Database Operations

Uses **sqlez** (Zed's SQLite wrapper):

```rust
// Async-friendly
impl TaskStore {
    pub fn add_task(&mut self, task: TaskData, cx: &mut Context<Self>) {
        cx.spawn(async move |this, cx| {
            db.save_task(&task)?;
            this.update(cx, |this, cx| {
                this.tasks.push(task);
                cx.emit(TaskStoreEvent::TaskAdded(task));
            })
        }).detach_and_log_err(cx);
    }
}
```

## Common Imports Pattern

Most UI files follow this structure:

```rust
use gpui::*;
use gpui_component::{
    h_flex, v_flex,
    button::{Button, ButtonVariants},
    label::Label,
    Root,
};
use crate::theme::*;  // Theme constants
```

## Important Notes

- **Lazy Panel Initialization**: Panels are created only when first accessed to improve startup performance
- **Virtual Lists**: Use virtualized lists/tables for large datasets (mentioned in docs)
- **Async Runtime**: Tokio is used throughout for async operations
- **No Unsafe Code**: The workspace forbids unsafe code
- **Documentation**: Extensive docs in `docs/` directory cover architecture patterns from Zed

## Reference Documentation

- `docs/ARCHITECTURE.md` - System architecture overview
- `docs/UI_ARCHITECTURE.md` - UI interaction patterns
- `docs/ZED_WORKSPACE_ARCHITECTURE.md` - Deep dive on Zed patterns
- `UAVRed_UI_Tasks.md` - Parallel development task breakdown
