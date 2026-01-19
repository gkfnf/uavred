# Assets UI Quick Start Guide

## 快速概览

Assets 模块提供了网络资产的拓扑可视化和详细信息展示。

## 主要组件

### AssetsPanel
主容器，分为两部分：
- **左侧 (flex_1)**: 网络拓扑图（TopologyCanvas）
- **右侧 (400px fixed)**: 资产详情面板（AssetDetailPanel）

```rust
pub fn view(cx: &mut ViewContext<Self>) -> impl IntoElement {
    AssetsPanel::new(cx)
}
```

## 使用示例

### 创建 AssetsPanel
```rust
use assets_ui::AssetsPanel;
use gpui::*;

let assets_panel = cx.new(AssetsPanel::new);
```

### 处理资产选择事件
```rust
// TopologyCanvas 会发出 AssetSelectedEvent
// 在 AssetDetailPanel 中处理：
detail_panel.set_node(selected_node, cx);
```

## 组件职责

| 组件 | 职责 | 状态 |
|------|------|------|
| **TopologyCanvas** | 网络拓扑可视化、节点点击检测 | ✅ 实现 |
| **AssetDetailPanel** | 资产信息展示、操作按钮 | ✅ 实现 |
| **InfoCard** | 标签-值对展示 | ✅ 实现 |
| **RiskBadge** | 风险等级徽章 | ✅ 实现 |
| **StatusIndicator** | 状态指示器 | ✅ 实现 |
| **PortList** | 端口列表 | ✅ 实现 |
| **AssetHeader** | 资产头部信息 | ✅ 实现 |

## 数据模型

所有组件使用 `data::models` 中的模型：
- `AssetNode` - 资产对象（包含所有元数据）
- `Severity` - 风险等级枚举
- `AssetStatus` - 资产状态枚举
- `Connection` - 网络连接信息
- `ZoneType` - 网络区域枚举

## 样式系统

使用 `ui::theme` 中的常量：
```rust
BG_PRIMARY, BG_CARD, BG_SECONDARY
TEXT_PRIMARY, TEXT_SECONDARY, TEXT_MUTED
ACCENT_BLUE, ACCENT_PURPLE
SEVERITY_CRITICAL, SEVERITY_HIGH, SEVERITY_MEDIUM, SEVERITY_LOW
```

## 开发流程

### 添加新的信息卡片
```rust
// 在 AssetDetailPanel 中使用 render_info_card
render_info_card("Label", "Value")
    .into_any_element()
```

### 自定义组件样式
```rust
h_flex()
    .gap_2()
    .p_3()
    .rounded_md()
    .bg(rgb(BG_SECONDARY))
    .child(Label::new("Text"))
```

### 处理事件
```rust
// TopologyCanvas 中的事件
cx.emit(AssetSelectedEvent::NodeSelected(node_id));

// AssetDetailPanel 中接收
detail_panel.set_node(node, cx);
```

## 常见任务

### 更新资产信息
```rust
let node = AssetNode {
    id: "uav-1".to_string(),
    name: "DJI Mavic 3".to_string(),
    ip_address: "192.168.1.100".to_string(),
    // ... 其他字段
};
detail_panel.set_node(node, cx);
```

### 改变资产选择
```rust
// 拓扑图中点击节点会触发：
// cx.emit(AssetSelectedEvent::NodeSelected(id))

// 或手动调用：
detail_panel.set_node(node, cx);
```

### 清除选择
```rust
detail_panel.clear_node(cx);
```

## 已知问题

1. **拖拽功能**: drag_state 已初始化但拖拽逻辑未完成（Pixels API 访问限制）
2. **连接线样式**: 目前所有连接线都是蓝色，实际应根据状态改变颜色
3. **缩放平移**: 当前没有实现滚轮缩放和拖拽平移

## 下一步

- [ ] 完成拖拽节点功能
- [ ] 添加按钮事件处理（扫描、编辑、删除）
- [ ] 实现搜索和过滤功能
- [ ] 添加动画过渡效果
- [ ] 性能优化（虚拟滚动等）

## 文件位置

```
crates/assets_ui/
├── src/
│   ├── lib.rs                    # 主入口
│   ├── asset_detail_panel.rs     # 详情面板
│   ├── topology_canvas.rs        # 拓扑图
│   └── components/               # 组件库
└── Cargo.toml
```

## 构建与测试

```bash
# 构建
cargo build -p assets_ui

# 运行所有测试
cargo test -p assets_ui

# 格式检查
cargo clippy -p assets_ui
```

## 参考资源

- GPUI 文档: https://github.com/zed-industries/zed/tree/main/crates/gpui
- 数据模型: `crates/data/src/models.rs`
- UI 主题: `crates/ui/src/theme.rs`
- 组件库: `src/gpui-component/`
