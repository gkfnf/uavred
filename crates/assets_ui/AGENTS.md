# Assets UI

**Purpose**: Network asset topology visualization with interactive canvas.

## Architecture Overview

```
assets_ui/
├── src/
│   ├── lib.rs                    # Top-level container (AssetsPanel)
│   ├── config/                   # Configuration layer
│   │   ├── zone_config.rs        # Zone metadata (Z1-Z5 names, colors)
│   │   ├── ui_labels.rs          # UI text constants (i18n ready)
│   │   └── theme_ext.rs          # Theme color constants
│   ├── repository/               # Data access layer
│   │   ├── repository.rs         # AssetRepository trait
│   │   └── mock_repository.rs    # Sample data for development
│   ├── asset_detail_panel/       # Asset detail view
│   │   ├── mod.rs                # Main panel component
│   │   └── cards/                # Individual info cards
│   │       ├── zone_card.rs      # Zone info
│   │       ├── risk_card.rs      # Risk score
│   │       ├── status_card.rs    # Online/offline status
│   │       ├── ports_card.rs     # Open ports
│   │       ├── services_card.rs  # Detected services
│   │       ├── credentials_card.rs
│   │       ├── business_card.rs
│   │       ├── owner_card.rs
│   │       ├── compliance_card.rs
│   │       ├── actions_card.rs   # AI/Scan/Config buttons
│   │       └── vuln_stats_card.rs
│   ├── topology_canvas/          # Network topology visualization
│   │   ├── mod.rs                # Main canvas component
│   │   ├── camera.rs             # Camera/viewport management
│   │   └── zone_canvas.rs        # Per-zone viewport with auto-centering
│   ├── components/               # Shared UI components
│   │   ├── asset_header.rs
│   │   ├── port_list.rs
│   │   ├── risk_badge.rs
│   │   ├── status_indicator.rs
│   │   └── topology_zone.rs
│   └── events.rs                 # Event definitions
```

## Data Flow

```
Database (future)
    │
    ▼
Repository (MockAssetRepository now)
    │
    ▼
TopologyCanvas / AssetDetailPanel
    │
    ▼
Card Components / ZoneCanvas
```

## Key Design Decisions

### 1. Configuration Layer (`config/`)

All static configuration centralized:

```rust
// Zone metadata
let config = zone.config();  // ZoneConfig { short_name, layer_name, primary_color, ... }

// UI labels
ui_labels::panel::TOPOLOGY_TITLE  // "网络拓扑 - 业务层级视图"
ui_labels::severity::HIGH          // "高危"

// Theme colors
theme_ext::CARD_RISK_BG  // 0xfdf4ff
```

### 2. Repository Pattern (`repository/`)

```rust
pub trait AssetRepository {
    fn get_all_assets(&self) -> Vec<AssetNode>;
    fn get_assets_by_zone(&self, zone: ZoneType) -> Vec<AssetNode>;
    fn get_asset_by_id(&self, id: &str) -> Option<AssetNode>;
}
```

- `MockAssetRepository` - Development/test data
- Future: `DbAssetRepository` - Real database connection

### 3. Card-Based Architecture

AssetDetailPanel uses 11 focused card components instead of one 590-line render function:

| Card | Responsibility |
|------|---------------|
| ZoneCard | Security zone info |
| RiskCard | Risk score + progress bar |
| StatusCard | Online/offline status |
| PortsCard | Open ports list |
| ServicesCard | Detected services |
| CredentialsCard | Auth info |
| BusinessCard | Business purpose |
| OwnerCard | Owner/team |
| ComplianceCard | Compliance badges |
| ActionsCard | AI/Scan/Config buttons |
| VulnStatsCard | Vulnerability count |

### 4. Camera System (`camera.rs`)

Centralized viewport management with smooth interactions:

```rust
pub struct Camera {
    pub scale: f32,           // Zoom level (0.1 - 5.0)
    pub offset_x: f32,        // Pan offset X
    pub offset_y: f32,        // Pan offset Y
    pub viewport_width: f32,  // Screen width
    pub viewport_height: f32, // Screen height
}
```

**Features:**
- **Auto-centering on init**: Each zone automatically fits all its nodes on creation
- **Mouse-wheel zoom**: 15% step factor, min 10%, max 500%
- **Pan by dragging**: Click and drag to pan the viewport
- **Zoom-to-point**: Zoom keeps the point under cursor stable
- **Fit-to-view**: `fit_to_view()` adjusts scale and offset to show all nodes

**Usage:**
```rust
// Auto-center on initialization (happens automatically)
let mut canvas = ZoneCanvas::new(...);
// canvas.fit_to_view(); // Already called in new()

// Manual reset
canvas.reset_view();

// Zoom at specific point
canvas.zoom(delta_y, mouse_x, mouse_y);

// Center on specific node
canvas.center_on_node("node-id");
```

### 5. Zone Canvas

TopologyCanvas delegates to 5 ZoneCanvas instances:
- Each zone has independent viewport (pan/zoom) via `Camera`
- **Auto-centered on load**: All nodes visible in viewport
- **Mouse wheel** = Zoom (centered on viewport center)
- **Click + drag** = Pan
- Rendering via GPUI canvas API with coordinate transformation

**Interaction Summary:**
- 🖱️ **Mouse wheel** = Zoom in/out (centered)
- 🖱️ **Click + drag** = Pan canvas
- 👆 **Click** = Select node

**Visual Design:**
- Each zone's canvas fills its allocated area completely
- Canvas content is **clipped by zone boundaries** (`overflow_hidden`)
- Nodes can be panned/zoomed infinitely, but only visible portion is shown
- Background color matches zone theme color

## Database Integration (Future)

To connect to real database:

1. Implement `DbAssetRepository`:
```rust
pub struct DbAssetRepository {
    db: DatabaseConnection,
}

impl AssetRepository for DbAssetRepository {
    fn get_all_assets(&self) -> Vec<AssetNode> {
        // Query database
    }
    // ...
}
```

2. Update `TopologyCanvas::new()` to use `DbAssetRepository` instead of `MockAssetRepository`

## Where Data Comes From

| UI Element | Source |
|-----------|--------|
| Asset count | Repository (DB in future) |
| Connection count | Repository (calculated from asset.connections) |
| Zone names | `config::zone_config` (Z1-Z5 Chinese names) |
| Severity labels | `config::ui_labels::severity` (低危/中危/高危/严重) |
| Risk score | AssetNode.risk_score (from DB) |
| Open ports | AssetNode.open_ports (from DB) |
| Services | AssetNode.services (from DB, with defaults) |
| Compliance | AssetNode.compliance_standards (with defaults) |
| Status | AssetNode.status (from DB) |
| Action buttons | Static UI (AI分析/扫描资产/配置) |

## Coding Conventions

### Import Order
```rust
// 1. Standard library
use std::...;

// 2. External crates
use gpui::*;
use gpui_component::*;

// 3. Internal modules
use crate::config::*;
use crate::repository::*;
use data::models::*;
```

### Text Constants
All user-facing text must use `ui_labels` constants:
```rust
// ✅ Good
Label::new(ui_labels::panel::TOPOLOGY_TITLE)

// ❌ Bad
Label::new("网络拓扑 - 业务层级视图")
```

### Colors
Use theme constants, not magic numbers:
```rust
// ✅ Good
.bg(rgb(theme_ext::CARD_RISK_BG))

// ❌ Bad
.bg(rgb(0xfdf4ff))
```
