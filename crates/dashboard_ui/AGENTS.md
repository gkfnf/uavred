# Dashboard UI

**Purpose**: Main dashboard with mission control, AI activity, and findings display.

## OVERVIEW

Central navigation hub with mission control kanban, AI agent activity monitor, scan findings display, and component library for reusable dashboard elements.

## STRUCTURE

```
dashboard_ui/
├── src/
│   ├── dashboard_panel.rs   # Main panel (~262 lines)
│   ├── mission_control.rs  # Kanban + detail panel (~331 lines)
│   ├── findings.rs         # Vulnerability findings (~298 lines)
│   └── components.rs       # Shared UI components (~281 lines)
```

## WHERE TO LOOK

| Task | Location |
|------|----------|
| Panel structure | `dashboard_panel.rs` |
| Mission control | `mission_control.rs` |
| Findings display | `findings.rs` |
| Render helpers | `components.rs` |

## CONVENTIONS

### Panel Structure

Main panel struct with optional stores:
```rust
pub struct DashboardPanel {
    task_store: Option<Entity<TaskStore>>,
    selected_task_id: Option<usize>,
    // ... state fields
}
```

### Component Rendering

Extract large render blocks into helper functions:
```rust
use crate::components::{
    render_ai_activity,
    render_kanban_column,
    render_task_card,
};
```

### State Cloning

Clone task lists before render to avoid borrowing:
```rust
let todo_tasks = panel.todo_tasks.clone();
let in_progress_tasks = panel.in_progress_tasks.clone();
```

### Conditional Sections

`.when()` for conditional rendering:
```rust
.when_some(self.selected_task_id, |this, task_id| {
    // render detail panel
})
```

## ANTI-PATTERNS

- **Never mix rendering logic with business logic** - keep components pure
- **Don't duplicate component code** - use `components.rs` for shared elements
- **Avoid hardcoded theme values** - use constants from `ui::theme::*`
- **Never create Entity<TaskStore> in panel** - receive from parent (workspace)
- **Don't use inline styles** - prefer utility methods like `.p_4()`, `.bg_gray_100()`

## GPUI PATTERNS

### Layout

Flexbox layouts with `h_flex()` and `v_flex()`:
```rust
h_flex()
    .flex_1()
    .pt(px(0.0))
    .px(px(24.0))
    .pb(px(24.0))
    .gap(px(16.0))
```

### Spacing

Use `px()` utility for pixel values:
```rust
.padding(px(16.0))
.margin_left(px(8.0))
.gap(px(12.0))
```

### Children

Add components with `.child()`:
```rust
.child(render_header())
.child(render_mission_control())
```

## COMPONENT LIBRARY

### Functions in `components.rs`

| Function | Purpose |
|----------|---------|
| `render_ai_activity` | AI agent activity display |
| `render_ai_tool` | AI tool usage stats |
| `render_kanban_column_header` | Column header with count |
| `render_task_card` | Draggable task card |

### Card Pattern

Reusable task card with status badge:
```rust
pub fn render_task_card(
    task: &TaskData,
    window: &mut Window,
    cx: &mut Context<DashboardPanel>,
) -> impl IntoElement {
    div()
        .p_4()
        .bg_white()
        .rounded_lg()
        .shadow_sm()
        .child(task.title.clone())
}
```

## INTEGRATION

- **TaskStore**: Shared across dashboard, kanban, and other panels
- **Theme Constants**: `use ui::theme::*` for colors, spacing
- **GPUI Components**: Buttons, labels, icons from `gpui_component`

## NOTES

- Mission Control is the primary view, other sections are secondary
- Task cards appear in both dashboard and kanban
- AI activity shows real-time agent operations
- Findings panel displays scan results (CVSS scores, CVEs)
