# Assets UI

**Purpose**: Network asset topology visualization with interactive canvas.

## OVERVIEW

Visualizes network assets (drones, controllers, ground stations) as nodes with connections, supporting zoom/pan and drag-drop node positioning.

## STRUCTURE

```
assets_ui/
├── src/
│   ├── topology_canvas.rs   # Interactive canvas (~492 lines)
│   ├── assets_panel.rs     # Asset list view
│   └── lib.rs            # Module exports
```

## WHERE TO LOOK

| Task | Location |
|------|----------|
| Canvas rendering | `topology_canvas.rs` |
| Node positioning | `topology_canvas.rs::NodePosition` |
| Drag-drop logic | `topology_canvas.rs` drag_state |
| Zoom/pan | `topology_canvas.rs` scale/offset |

## CONVENTIONS

### Canvas State

Maintain transform state (scale, offset) and node positions:
```rust
pub struct TopologyCanvas {
    nodes: Vec<AssetNode>,
    connections: Vec<ConnectionInfo>,
    node_positions: HashMap<String, NodePosition>,
    selected_node_id: Option<String>,
    scale: f32,
    offset_x: f32,
    offset_y: f32,
    drag_state: Option<(String, Point<Pixels>)>,
    canvas_bounds: Option<Bounds<Pixels>>,
}
```

### Position Tracking

Use HashMap for O(1) position lookup:
```rust
struct NodePosition {
    pub x: f32,
    pub y: f32,
}
```

### Event Emission

Emit selection events:
```rust
pub enum AssetSelectedEvent {
    NodeSelected(String),
}

impl EventEmitter<AssetSelectedEvent> for TopologyCanvas {}

// Emit:
cx.emit(AssetSelectedEvent::NodeSelected(node_id.clone()));
```

## ANTI-PATTERNS

- **Never use absolute pixel values for positioning** - use relative coordinates with transform
- **Don't recalculate positions on every render** - cache in HashMap
- **Avoid nested HashMap lookups** - use `get()` once and store reference
- **Never clear drag_state on mouse up without validation** - check bounds first
- **Don't hardcode node sizes** - use constants for radius, spacing

## CANVAS PATTERNS

### Pan/Zoom

Apply transform before rendering:
```rust
fn transform_point(&self, x: f32, y: f32) -> Point<Pixels> {
    px(x * self.scale + self.offset_x, y * self.scale + self.offset_y)
}
```

### Hit Testing

Check distance for node selection:
```rust
fn hit_test(&self, mouse_x: f32, mouse_y: f32) -> Option<&str> {
    for (id, pos) in &self.node_positions {
        let dx = pos.x - mouse_x;
        let dy = pos.y - mouse_y;
        if dx*dx + dy*dy < NODE_RADIUS*NODE_RADIUS {
            return Some(id);
        }
    }
    None
}
```

### Drag State

Track drag with delta calculation:
```rust
.on_mouse_move(|event, window, cx| {
    if let Some((node_id, start_pos)) = &self.drag_state {
        let delta = event.position - *start_pos;
        // Update node position
    }
})
```

### Connection Rendering

Draw lines between connected nodes:
```rust
for conn in &self.connections {
    let start = self.node_positions.get(&conn.from)?;
    let end = self.node_positions.get(&conn.to)?;
    // Draw line from start to end
}
```

## GPUI PATTERNS

### Event Handlers

Register handlers on element:
```rust
.on_click(cx.listener(|this, event, window, cx| {
    // Handle click
}))
.on_mouse_move(cx.listener(|this, event, window, cx| {
    // Handle drag
}))
```

### Render Loop

Custom rendering in canvas:
```rust
fn render_nodes(&self, window: &mut Window, cx: &mut Context<Self>) {
    for node in &self.nodes {
        let pos = self.node_positions.get(&node.id)?;
        // Draw node circle + label
    }
}
```

## NOTES

- Canvas is still WIP - basic structure exists, rendering needs implementation
- Node positions are auto-calculated in `create_sample_nodes()`
- Zoom/pan uses transform matrix (scale, offset_x, offset_y)
- Drag-drop updates node_positions HashMap, not AssetNode directly
- Connections are directional (from -> to) with optional metadata
