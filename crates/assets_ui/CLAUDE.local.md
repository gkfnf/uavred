# Assets UI Agent Instructions

## Scope
This agent is responsible ONLY for `crates/assets_ui/` - the asset topology panel.

## Module Structure
```
assets_ui/
├── lib.rs               # AssetsPanel - 主面板 (拓扑+详情)
├── topology_canvas.rs   # 资产拓扑画布 (网络图)
├── node_detail.rs       # 节点详情面板
└── node_detail_stub.rs  # 节点详情占位 (编译修复用)
```

## Key Data Models (from `data::models`)
- `AssetNode` - 资产节点 (ID, 类型, 位置, 连接)
- `AssetType` - UAV/GCS/Router/Server/Unknown
- `ConnectionInfo` - 节点间连接信息
- `DeviceInfo` - 设备详细信息

## Required Imports Pattern
```rust
use gpui::*;
use gpui_component::{h_flex, v_flex, canvas::Canvas};
use data::models::{AssetNode, AssetType, ConnectionInfo};
use ui::theme::*;
```

## Canvas Rendering Pattern
```rust
// 拓扑画布使用 Canvas 组件
fn render_topology(&self, cx: &mut Context<Self>) -> impl IntoElement {
    canvas(
        |_bounds, _window, _cx| {},  // prepaint
        |bounds, window, cx| {
            // paint - 绘制节点和连线
        },
    )
    .size_full()
}
```

## Theme Constants to Use
- Node colors by type: 定义在节点渲染中
- Connection lines: `BORDER_COLOR`, `ACCENT_BLUE`
- Selection: `BORDER_FOCUSED`
- Background: `BG_PRIMARY`

## Interaction Patterns
- 节点选择: `on_click` -> `cx.emit(AssetSelected(node_id))`
- 节点拖拽: 需要跟踪 `drag_state`
- 画布缩放/平移: viewport transform

## DO NOT
- 修改共享文件
- 实现网络扫描逻辑 (属于 `scanner/network.rs`)
- 创建新的资产类型 (修改 `data/models.rs`)

## Current TODOs
- [ ] 完善 node_detail.rs 实现 (当前使用 stub)
- [ ] 实现节点拖拽功能
- [ ] 添加画布缩放/平移
- [ ] 实现节点右键菜单
- [ ] 添加连接线动画效果
