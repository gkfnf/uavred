# Coding Conventions

**Analysis Date:** 2026-01-29

## Naming Patterns

**Files:**
- Use `snake_case.rs` for all Rust source files
- No `mod.rs` files - prefer `src/some_module.rs` over `src/some_module/mod.rs`
- For crate library roots, specify path in `Cargo.toml` with `[lib] path = "...rs"` for descriptive naming

**Functions:**
- Use `snake_case` for all function names
- Constructor functions use `new()` pattern
- Render functions use `render_*` prefix (e.g., `render_vuln_list()`, `render_header()`)
- Private helpers use leading underscore only when intentionally unused

**Variables:**
- Use full words, no abbreviations (e.g., `task_store` not `ts`, `connection` not `conn`)
- Use variable shadowing to scope clones in async contexts for clarity
- Boolean flags use positive phrasing (e.g., `is_active` not `is_not_active`)

**Types:**
- Structs/enums use `PascalCase`
- Generic parameters use single uppercase letters (e.g., `T`, `M`, `R`)
- Type aliases use `PascalCase`
- Traits use `PascalCase` with descriptive names (e.g., `ZoneTypeExt`)

**Constants:**
- `SCREAMING_SNAKE_CASE` for true constants
- Theme constants follow pattern: `CATEGORY_PROPERTY` (e.g., `BG_PRIMARY`, `TEXT_SECONDARY`, `SEVERITY_CRITICAL`)

## Code Style

**Formatting:**
- Use `cargo fmt` for all formatting
- Rust 2024 edition
- Max width: 100 characters (implied from codebase)
- Import organization: stdlib first, then external crates, then internal modules

**Linting:**
- Use `cargo clippy` with warnings as errors: `cargo clippy -- -D warnings`
- Clippy config at `/Users/fk/Devlopment/uavred/src/zed/clippy.toml`
- Forbidden: `unsafe_code` (workspace-level lint)
- Disallowed methods enforce async patterns (e.g., use `smol::process::Command` not `std::process::Command`)

**Import Organization:**
```rust
// 1. Standard library
use std::collections::HashMap;
use std::sync::Arc;

// 2. External crates
use gpui::*;
use gpui_component::{h_flex, v_flex, button::Button};
use anyhow::Result;

// 3. Internal modules
use crate::theme::*;
use data::models::{Task, VulnData};
use ui::events::WorkspaceEvent;
```

## Error Handling

**Patterns:**
- Use `anyhow::Result<T>` for most functions
- Propagate errors with `?` operator
- Never silently discard errors with `let _ =` on fallible operations
- Use `.log_err()` when ignoring errors but want visibility
- Use explicit `match` or `if let Err(...)` for custom error handling

**Example:**
```rust
// Good - propagates error
pub async fn execute(&self, task: Task) -> Result<TaskResult> {
    let result = self.run_scan(task).await?;
    Ok(result)
}

// Bad - silently discards error
let _ = client.request(...).await?;

// Good - explicit handling
client.request(...).await?;
```

**Avoid panics:**
- Avoid `unwrap()`, use `?` or explicit error handling
- Be careful with indexing - may panic if out of bounds
- Use `anyhow::Context` for adding context to errors

## Logging

**Framework:** `tracing` crate

**Patterns:**
- Use appropriate log levels:
  - `tracing::info!()` - Important operations (task execution, view changes)
  - `tracing::debug!()` - Detailed state information
  - `tracing::warn!()` - Recoverable issues
  - `tracing::error!()` - Failures that need attention
- Include relevant identifiers in log messages

**Example:**
```rust
tracing::info!("Executing task: {}", task.name);
tracing::error!("Failed to connect: {:?}", error);
```

## Comments

**When to Comment:**
- Explain "why" not "what" - code should be self-documenting
- Document non-obvious design decisions
- Chinese comments are used for UI-related documentation
- English for technical implementation details

**Documentation:**
- Use `///` for public API documentation
- Use `//!` for module-level documentation
- Document panics, errors, and safety invariants

**Example:**
```rust
/// Workspace - 顶层协调者，类似 zed 的 Workspace
/// Manages active view switching and panel lifecycle
pub struct Workspace {
    active_view: AppView,
    // ...
}
```

## Function Design

**Size:**
- Functions should fit on screen (~50 lines max ideally)
- Break large render functions into smaller `render_*` helpers

**Parameters:**
- `window: &mut Window` comes before `cx: &mut Context<T>` when present
- Callbacks come after context parameters
- Use impl IntoElement for flexible return types in UI code

**Return Values:**
- Use `impl IntoElement` for UI component functions
- Use `anyhow::Result<T>` for fallible operations
- Return `Option<T>` for lookups that may fail

## Module Design

**Exports:**
- Re-export commonly used items at crate root (`lib.rs`)
- Use `pub use` to flatten module hierarchies where appropriate

**Barrel Files:**
- `mod.rs` files are NOT used - prefer explicit file naming
- Each module file explicitly declares its submodules

**Example structure:**
```rust
// crates/data/src/lib.rs
pub mod models;
pub mod repository;
pub mod task_store;

pub use models::*;
pub use repository::*;
pub use task_store::{TaskStore, TaskStoreEvent, init_task_store};
```

## GPUI-Specific Patterns

**Entity Pattern:**
- Use `Entity<T>` for stateful components
- Store subscriptions in `_subscriptions: Vec<Subscription>` field
- Clone entities when sharing between components

**Example:**
```rust
pub struct MyPanel {
    task_store: Entity<TaskStore>,
    _subscriptions: Vec<Subscription>,
}
```

**Render Trait:**
- All view components implement `Render`
- Use `Root` wrapper for all windows
- Call `cx.notify()` after state changes to trigger re-render

**Event Handling:**
- Use `cx.listener()` for event handlers that need entity access
- Use `cx.emit()` to emit events
- Subscribe with `cx.subscribe()` and store subscription

**Async Pattern:**
- Use `cx.spawn()` for foreground async tasks
- Use `cx.background_spawn()` for background work
- Use `.detach_and_log_err(cx)` to fire-and-forget tasks

**Example:**
```rust
cx.spawn(async move |this, cx| {
    let result = async_operation().await?;
    this.update(cx, |this, cx| {
        this.state = result;
        cx.notify();
    })?;
    Ok::<_, anyhow::Error>(())
}).detach_and_log_err(cx);
```

## Theme Constants

**Location:** `crates/ui/src/theme.rs`

**Usage:**
- Never hardcode colors or spacing
- Use semantic naming (e.g., `BG_PRIMARY` not `WHITE`)
- Severity colors: `SEVERITY_CRITICAL`, `SEVERITY_HIGH`, etc.
- Spacing: `SPACING_SM`, `SPACING_MD`, `SPACING_LG`
- Padding: `PADDING_SM`, `PADDING_MD`, `PADDING_LG`
- Text sizes: `TEXT_SIZE_SM`, `TEXT_SIZE_BASE`, `TEXT_SIZE_LG`

**Example:**
```rust
use ui::theme::*;

v_flex()
    .p(PADDING_MD)
    .gap(SPACING_SM)
    .bg(rgb(BG_CARD))
    .rounded(BORDER_RADIUS)
```

## Database Patterns

**sqlez Usage:**
- Use `query!` macro for type-safe SQL
- Use `ThreadSafeConnection` for shared database access
- Migrations defined as static arrays of SQL strings
- Use `Domain` trait for migration management

**Example:**
```rust
query! {
    pub fn read_kvp(key: &str) -> Result<Option<String>> {
        SELECT value FROM kv_store WHERE key = (?)
    }
}
```

---

*Convention analysis: 2026-01-29*
