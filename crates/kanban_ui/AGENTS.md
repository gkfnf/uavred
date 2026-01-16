# Kanban UI

**Purpose**: Drag-and-drop task board with squeeze-style detail panel.

## OVERVIEW

Implements 5-column kanban board (Todo, In Progress, In Review, Done, Canceled) with Entity<TaskStore> integration, drag-and-drop task movement, and collapsible detail panel.

## STRUCTURE

```
kanban_ui/
├── src/
│   ├── kanban_board.rs      # Main board container (~295 lines)
│   ├── kanban_column.rs     # Column component
│   ├── task_card.rs         # Draggable task card (~365 lines)
│   └── task_detail.rs       # Detail panel (squeeze-style)
```

## WHERE TO LOOK

| Task | Location |
|------|----------|
| Board layout | `kanban_board.rs` |
| Column rendering | `kanban_column.rs` |
| Task card UI | `task_card.rs` |
| Detail panel | `task_detail.rs` |
| Drag-drop logic | `task_card.rs` |

## CONVENTIONS

### Entity Pattern

Board holds Entity<TaskStore> for reactive updates:
```rust
pub struct KanbanBoard {
    task_store: Entity<TaskStore>,
    selected_task_id: Option<usize>,
    _subscriptions: Vec<Subscription>,
}
```

Access via `read()` or `update()`:
```rust
let tasks = self.task_store.read(cx).tasks_by_status(status);
```

### Event Emission

Use EventEmitter for board events:
```rust
pub enum KanbanEvent {
    TaskSelected(Option<usize>),
    TaskMoved { task_id: usize, from: TaskStatus, to: TaskStatus },
    DetailPanelToggled(bool),
}

impl EventEmitter<KanbanEvent> for KanbanBoard {}

// Emit:
cx.emit(KanbanEvent::TaskMoved { task_id, from, to });
```

### Subscription Pattern

Subscribe to store changes:
```rust
cx.subscribe(&self.task_store, |this, store, event, cx| {
    match event {
        TaskStoreEvent::TaskAdded => { /* update UI */ }
    }
    cx.notify();
})
```

### Column Layout

Use `h_flex()` for horizontal columns with `flex_1()` equal widths:
```rust
h_flex()
    .flex_1()
    .gap(px(16.0))
    .child(column_todo)
    .child(column_in_progress)
```

### Task Card State

Clone data to avoid borrowing issues:
```rust
let todo_tasks = self.task_store.read(cx).tasks_by_status(TaskStatus::Todo);
```

## ANTI-PATTERNS

- **Never hold Entity reference across async boundaries** - clone data or use `WeakEntity`
- **Don't mutate state in event handlers without update()** - use `cx.update()`
- **Avoid manual drag-drop state** - use GPUI's built-in drag handlers (planned)
- **Never create multiple TaskStore instances** - use shared Entity<TaskStore>
- **Don't bypass cx.notify()** - must call after state changes to trigger re-render

## GPUI PATTERNS

### Flexbox Layout

Tailwind-like utility methods:
- `.flex_1()` - flex: 1
- `.gap(px(16.0))` - spacing
- `.items_start()` - align-items: flex-start
- `.h(px(0.0))` - height: 0 (for flex child expansion)

### Conditional Rendering

`.when_some()` and `.when()` for conditional UI:
```rust
.when_some(self.selected_task_id, |this, task_id| {
    // render selected task
})
.when(self.detail_panel_visible, |this| {
    // render detail panel
})
```

### Scroll Containers

Wrap columns in scrollable containers:
```rust
use gpui_component::scroll::ScrollableElement as _;

v_flex()
    .overflow_y_scroll()
    .size_full()
```

## NOTES

- KanbanBoard is a 5-column board, not traditional 4-column
- Detail panel is squeeze-style (side panel, not modal)
- Drag-drop is WIP - currently button-based movement
- TaskStore provides reactive updates to all panels
