# Traffic UI Agent Instructions

## Scope
This agent is responsible ONLY for `crates/traffic_ui/` - the network traffic analysis panel.

## Module Structure
```
traffic_ui/
├── lib.rs              # TrafficPanel - 主面板布局
├── query_bar.rs        # 顶部查询栏 (过滤/搜索/时间范围)
├── traffic_table.rs    # 中部流量表格 (虚拟滚动)
├── request_response.rs # 右侧请求/响应详情
└── actions_panel.rs    # 底部操作面板 (重放/导出/标记)
```

## Key Data Models (from `data::models`)
- `TrafficEntry` - 单条流量记录
- `TrafficProtocol` - HTTP/MAVLink/RTSP/WebSocket 等
- `TrafficDirection` - Inbound/Outbound
- `AnomalyInfo` - 异常检测信息

## Required Imports Pattern
```rust
use gpui::*;
use gpui_component::{h_flex, v_flex, table::*, input::TextInput};
use data::models::{TrafficEntry, TrafficProtocol};
use ui::theme::*;
```

## Theme Constants to Use
- Protocol colors: 根据协议类型使用 `ACCENT_BLUE`, `ACCENT_PURPLE`
- Anomaly highlight: `STATUS_WARNING`, `STATUS_ERROR`
- Table: `BG_CARD`, `BORDER_COLOR`, `TEXT_PRIMARY`, `TEXT_SECONDARY`

## Performance Requirements
- 流量表格必须使用虚拟滚动 (large dataset)
- 避免在 render 中进行大量计算
- 使用 `cx.spawn()` 处理异步过滤操作

## Component Patterns
```rust
// 虚拟列表示例
fn render_traffic_table(&self, cx: &mut Context<Self>) -> impl IntoElement {
    // Use List with item_count for virtualization
}
```

## DO NOT
- 修改共享文件 (`ui/theme.rs`, `data/models.rs`)
- 实现实际的网络抓包逻辑 (属于 `scanner` crate)
- 添加新的协议解析 (属于 `scanner/protocol.rs`)

## Current TODOs
- [ ] 实现流量过滤查询语法
- [ ] 添加协议解码展示
- [ ] 实现请求重放功能 UI
- [ ] 添加异常流量高亮
