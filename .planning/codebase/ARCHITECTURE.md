# Architecture

**Analysis Date:** 2026-01-29

## Pattern Overview

**Overall:** Zed Editor-inspired Workspace Architecture with Three-Layer UI Pattern

UAVRed follows the **Zed editor's workspace architecture patterns** closely. The architecture is built around a central Workspace coordinator that manages feature-specific panels, using GPUI's reactive Entity system for state management.

**Key Characteristics:**
- **Workspace Pattern**: Single top-level coordinator managing all views
- **Lazy Panel Initialization**: Panels created only when first accessed
- **Event-Driven Communication**: GPUI's EventEmitter trait for cross-component communication
- **Entity-Based State**: All stateful components use GPUI's `Entity<T>` pattern
- **Three-Layer Architecture**: Presentation → Application State → Data Layer

## Layers

### Presentation Layer (UI Components)

- **Purpose**: Render data from stores and handle user interactions
- **Location**: `crates/*_ui/src/`
- **Contains**: Panel implementations, reusable components, render logic
- **Depends on**: Application State Layer (Stores), Data Layer (Models)
- **Used by**: Workspace (`crates/uavred/src/workspace.rs`)

**Key Panels:**
- `DashboardPanel` (`crates/dashboard_ui/src/dashboard_panel.rs`): Mission control kanban + findings
- `AssetsPanel` (`crates/assets_ui/src/lib.rs`): Network topology + asset details
- `VulnsPanel` (`crates/vulns_ui/src/lib.rs`): Vulnerability list + detail + CVE info
- `TrafficPanel` (`crates/traffic_ui/src/lib.rs`): Network traffic analysis
- `FlowsPanel` (`crates/flows_ui/src/lib.rs`): Workflow execution

**Component Pattern:**
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

### Application State Layer (Stores)

- **Purpose**: Global state management with database persistence
- **Location**: `crates/data/src/task_store.rs`, `crates/data/src/vuln_store.rs`
- **Contains**: Store entities, event emission, business logic
- **Depends on**: Data Layer (Repository, Models)
- **Used by**: Presentation Layer (Panels)

**Key Stores:**
- `TaskStore` (`crates/data/src/task_store.rs`): Task management with kanban board state
- `VulnStore` (`crates/data/src/vuln_store.rs`): Vulnerability data management

**Store Pattern:**
```rust
pub struct TaskStore {
    db: Arc<Mutex<Database>>,
    tasks: Vec<Task>,
}

impl EventEmitter<TaskStoreEvent> for TaskStore {}

impl TaskStore {
    pub fn global(cx: &mut App) -> Entity<Self> {
        // Access global singleton
    }
}
```

### Data Layer

- **Purpose**: Data models, database operations, repository pattern
- **Location**: `crates/data/src/`
- **Contains**: Models, repositories, database connections
- **Depends on**: SQLite (via sqlez)
- **Used by**: Application State Layer

**Key Components:**
- `models.rs` (`crates/data/src/models.rs`): Rich data models (Task, Asset, Vuln, Traffic, etc.)
- `repository.rs` (`crates/data/src/repository.rs`): Database access layer
- `uavred_db.rs` (`crates/data/src/uavred_db.rs`): Database initialization

### Core Business Logic Layer

- **Purpose**: Business logic independent of UI
- **Location**: `crates/core/src/`, `crates/agent/src/`, `crates/scanner/src/`
- **Contains**: Task definitions, vulnerability DB, agent system, scanners
- **Depends on**: Data Layer
- **Used by**: Application State Layer

**Key Modules:**
- `core`: Task definitions (`crates/core/src/task.rs`), Vuln DB (`crates/core/src/vuln_db.rs`)
- `agent`: Scheduler (`crates/agent/src/scheduler.rs`), Executor (`crates/agent/src/executor.rs`)
- `scanner`: Network (`crates/scanner/src/network.rs`), Protocol (`crates/scanner/src/protocol.rs`), Firmware (`crates/scanner/src/firmware.rs`)

### Shared Infrastructure Layer

- **Purpose**: Cross-cutting concerns (theme, events, actions)
- **Location**: `crates/ui/src/`, `crates/workspace/src/`
- **Contains**: Theme constants, events, actions, shared types
- **Depends on**: GPUI
- **Used by**: All UI crates

**Key Files:**
- `theme.rs` (`crates/ui/src/theme.rs`): Color constants, spacing, sizing
- `events.rs` (`crates/ui/src/events.rs`): Event definitions (WorkspaceEvent, DashboardEvent, etc.)
- `actions.rs` (`crates/ui/src/actions.rs`): Action definitions (ActivateView, SelectTask, etc.)
- `lib.rs` (`crates/workspace/src/lib.rs`): Shared types (AppView, TaskData, VulnFilter)

## Data Flow

### Task Creation Flow

1. **User Action**: Click "Add Task" in DashboardPanel (`crates/dashboard_ui/src/add_task_modal.rs`)
2. **Event Emission**: Modal emits task data via callback
3. **Store Update**: DashboardPanel calls `TaskStore::add_task_raw()`
4. **Database Persist**: TaskStore writes to SQLite via `TaskRepository::create()`
5. **State Update**: TaskStore updates in-memory Vec<Task>
6. **Event Broadcast**: TaskStore emits `TaskStoreEvent::TaskAdded`
7. **UI Update**: Subscribed panels receive event, call `cx.notify()` to re-render

### Asset Selection Flow

1. **User Action**: Click node in TopologyCanvas (`crates/assets_ui/src/topology_canvas/`)
2. **Event Emission**: Canvas emits `AssetSelectedEvent::NodeSelected(node_id)`
3. **Parent Handle**: AssetsPanel receives event via `cx.subscribe()`
4. **Detail Update**: AssetsPanel updates AssetDetailPanel with selected node
5. **UI Update**: Detail panel renders asset information using card-based layout

### Vulnerability View Flow

1. **Store Access**: VulnsPanel reads from global VulnStore (`crates/data/src/vuln_store.rs`)
2. **List Render**: VulnListPanel displays filtered vulnerabilities
3. **Selection**: User clicks vulnerability in list
4. **Detail Render**: VulnDetailPanel shows full vulnerability info
5. **CVE Panel**: CveInfoPanel displays CVE/CVSS data

## Key Abstractions

### Entity Pattern

All stateful components use GPUI's `Entity<T>` for lifecycle management:

```rust
pub struct MyPanel {
    task_store: Entity<TaskStore>,
    _subscriptions: Vec<Subscription>,
}
```

**Pattern:** Store references to other entities, use `cx.subscribe()` for event handling, store subscriptions to auto-cleanup on drop.

### Global State Pattern

Stores are registered as GPUI globals for app-wide access:

```rust
struct GlobalTaskStore(Entity<TaskStore>);
impl Global for GlobalTaskStore {}

// Access anywhere
let store = TaskStore::global(cx);
```

### Event-Driven Communication

Events defined in `crates/ui/src/events.rs`:

```rust
pub enum WorkspaceEvent {
    ViewChanged(AppView),
    TaskSelected(Option<usize>),
    TaskAdded(TaskData),
}
```

Subscribe pattern:
```rust
let subscription = cx.subscribe(&source, |this, source, event, cx| {
    // Handle event
});
```

### Action System

Actions defined in `crates/ui/src/actions.rs`:

```rust
#[derive(Clone, PartialEq, Debug, Action, Deserialize)]
#[action(namespace = workspace, no_json)]
pub struct ActivateView(pub AppView);
```

Dispatch: `cx.dispatch_action(&ActivateView(AppView::Dashboard))`

### Repository Pattern

Data access abstracted through repositories:

```rust
pub struct TaskRepository {
    connection: Arc<Mutex<Connection>>,
}

impl TaskRepository {
    pub fn create(&self, task: &Task) -> Result<i64> { ... }
    pub fn list_by_status(&self, status: TaskStatus) -> Result<Vec<Task>> { ... }
}
```

## Entry Points

### Application Entry

- **Location**: `crates/uavred/src/main.rs`
- **Responsibilities**:
  1. Initialize tracing subscriber for logging
  2. Create GPUI Application with assets
  3. Initialize gpui-component theme
  4. Initialize and load TaskStore
  5. Open main window with Workspace
  6. Wrap in Root component (required for dialogs/notifications)

### Workspace Entry

- **Location**: `crates/uavred/src/workspace.rs`
- **Responsibilities**:
  1. Manage active view state (`AppView` enum)
  2. Lazy-initialize panels on first access
  3. Render title bar with navigation
  4. Render active panel based on view state
  5. Handle view switching via events/actions

### Panel Entry Points

Each panel is initialized lazily via `get_or_create_*_panel()` methods:

```rust
fn get_or_create_dashboard_panel(&mut self, cx: &mut Context<Self>) -> Entity<DashboardPanel> {
    if let Some(ref panel) = self.dashboard_panel {
        panel.clone()
    } else {
        let panel = cx.new(|cx| DashboardPanel::new(cx));
        self.dashboard_panel = Some(panel.clone());
        panel
    }
}
```

## Error Handling

**Strategy**: Use `anyhow::Result` consistently throughout the codebase

**Patterns:**
- Database operations return `anyhow::Result<T>`
- Async tasks use `cx.spawn().detach_and_log_err(cx)` for error logging
- Store methods propagate errors to callers

**Example:**
```rust
pub async fn execute(&self, task: Task) -> Result<TaskResult> {
    // ... implementation
}

// Spawn async task
cx.spawn(async move |this, cx| {
    let result = async_operation().await?;
    this.update(cx, |this, cx| {
        this.state = result;
        cx.notify();
    })?;
    Ok::<_, anyhow::Error>(())
}).detach_and_log_err(cx);
```

## Cross-Cutting Concerns

**Logging**: Use `tracing` crate
```rust
tracing::info!("Executing task: {}", task.name);
tracing::error!("Failed to connect: {:?}", error);
```

**Validation**: Model-level validation in data layer

**Authentication**: Auth fields in models (`auth_type`, `auth_status`, `auth_credential`)

**Theming**: All UI uses constants from `crates/ui/src/theme.rs`
- Colors: `BG_PRIMARY`, `TEXT_PRIMARY`, `SEVERITY_CRITICAL`
- Spacing: `PADDING_MD`, `SPACING_SM`
- Sizing: `BORDER_RADIUS`, `HEADER_HEIGHT`

---

*Architecture analysis: 2026-01-29*
