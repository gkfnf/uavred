# Assets UI Implementation Details

## 架构设计

### 整体布局
```
AssetsPanel (h_flex, size_full, bg: BG_PRIMARY)
├── TopologyCanvas (flex_1, bg: BG_CARD)
│   ├── Header: "Network Topology"
│   └── Canvas: Interactive node visualization
└── AssetDetailPanel (w: 400px, flex_none, bg: BG_CARD)
    ├── AssetHeader (asset name, IP, vuln badge)
    └── Detail Sections
        ├── Basic Information
        ├── Security & Risk
        ├── Network Information
        ├── Open Ports
        ├── Owner & Department
        ├── Scan Status
        └── Action Buttons
```

## 组件详解

### 1. AssetsPanel (lib.rs)

**职责**: 顶层容器，协调两个主要面板

**状态**:
```rust
pub struct AssetsPanel {
    topology_canvas: Entity<TopologyCanvas>,
    asset_detail_panel: Entity<AssetDetailPanel>,
}
```

**关键方法**:
- `new(cx)`: 创建实例并初始化子组件
- `render()`: 返回 h_flex 布局

**设计考虑**:
- 用 Entity 包装子组件以保持独立状态管理
- 右侧面板固定宽度 400px，确保舒适的阅读体验
- 左侧面板 flex_1，利用剩余空间

---

### 2. TopologyCanvas (topology_canvas.rs)

**职责**: 网络拓扑图可视化、节点交互

**核心数据结构**:
```rust
pub struct TopologyCanvas {
    nodes: Vec<AssetNode>,              // 资产节点列表
    connections: Vec<Connection>,        // 网络连接
    node_positions: HashMap<String, NodePosition>,  // 计算的节点位置
    selected_node_id: Option<String>,   // 选中的节点ID
    scale: f32,                          // 缩放因子
    offset_x: f32, offset_y: f32,       // 平移偏移
    drag_state: Option<(String, Point)>, // 拖拽状态
    canvas_bounds: Option<Bounds>,       // 画布边界
}

pub struct NodePosition {
    pub x: f32,
    pub y: f32,
}
```

**位置计算算法**:
1. 按 Zone 分组节点（Z1-Z5）
2. 为每个区域分配水平空间 (800px / 5)
3. 按分组数量均匀分布垂直位置

```
Zone Layout:
[Z1]    [Z2]    [Z3]    [Z4]    [Z5]
 .      .       .       .       .
 .      .       .       .       .
 .      .       .       .       .
```

**事件处理**:
- `handle_mouse_down()`: 点击检测（矩形碰撞）
- `handle_mouse_move()`: 拖拽处理（当前简化）
- `handle_mouse_up()`: 释放状态清理

**渲染**:
- 使用 GPUI 的 canvas 原语
- 连接线: PathBuilder.stroke() → Bezier 曲线
- 节点: PathBuilder.fill() → 圆形，选中时添加边框

**事件发射**:
```rust
cx.emit(AssetSelectedEvent::NodeSelected(node_id))
```

---

### 3. AssetDetailPanel (asset_detail_panel.rs)

**职责**: 显示选定资产的详细信息

**状态**:
```rust
pub struct AssetDetailPanel {
    selected_node: Option<AssetNode>,
}
```

**布局结构**:
```
┌─ Asset Header ─────────────────────┐
│ Icon  Name          IP      [Vulns]│
├────────────────────────────────────┤
│ Basic Information                  │
│ ┌─ ID ──┐ ┌─ Type ──┐             │
│ ├─ MAC ─┤ ├─ Vendor ┤             │
│ ├─ Firm ┤ ├─ Loc ───┤             │
│                                    │
│ Security & Risk                    │
│ [Badge] [Status]                   │
│                                    │
│ Network Information                │
│ [Zone] [Ports]                     │
│ [Services]                         │
│                                    │
│ Open Ports                         │
│ ├─ 22   TCP   SSH                  │
│ ├─ 443  TCP   HTTPS                │
│                                    │
│ Owner & Department                 │
│ [Owner] [Dept]                     │
│ [Business Purpose]                 │
│                                    │
│ Scan Status                        │
│ [Last]  [Next]                     │
│ [Type]  [Progress]                 │
│                                    │
│ [Scan] [Edit] [Delete]             │
└────────────────────────────────────┘
```

**关键方法**:
- `set_node(node, cx)`: 更新显示的资产
- `clear_node(cx)`: 清除选择
- `render_section_title()`: 章节标题
- `render()`: 完整 UI 树

**设计特性**:
- 所有值通过 `clone()` 确保生命周期合法
- `unwrap_or_else()` 处理 Option 值，默认为 "N/A"
- 条件渲染（有漏洞时显示徽章）
- 滚动容器用于长内容

---

### 4. 组件库 (components/)

#### InfoCard (info_card.rs)
**用途**: 统一的标签-值展示样式

```rust
pub fn render_info_card(label, value) -> AnyElement
```

**样式**:
- 背景: BG_SECONDARY (浅灰)
- 标签宽度: 80px (左对齐)
- 值: flex_1 (占据剩余空间)
- 间距: gap_3, padding_2, rounded_md

**为什么用函数而不是组件?**
- 避免生命周期复杂性（&str → SharedString）
- 直接返回 AnyElement，易于集成
- 减少编译复杂性

#### RiskBadge (risk_badge.rs)
**用途**: 显示威胁等级和风险评分

**颜色映射**:
```
Critical → #ef4444 (红)
High     → #f97316 (橙)
Medium   → #fbbf24 (黄)
Low      → #10b981 (绿)
Info     → #10b981 (绿)
```

**格式**: `[Severity] [Score]/100`

#### StatusIndicator (status_indicator.rs)
**用途**: 资产在线状态显示

**元素**:
- 彩色圆点 (2px × 2px)
- 状态文本
- 颜色反映状态

#### PortList (port_list.rs)
**用途**: 开放端口列表

**特性**:
- 最大高度 200px（可滚动）
- 每行: 端口号 | 协议 | 服务
- 空状态处理

#### AssetHeader (asset_header.rs)
**用途**: 资产卡头部

**布局**:
```
[Icon] Name
        IP Address [Vuln Badge]
```

**图标选择**:
- UAV → Globe
- GCS → LayoutDashboard
- Router → Network
- Server → HardDrive
- Default → SquareTerminal

---

## 数据流

### 选择资产流程
```
用户点击拓扑节点
    ↓
TopologyCanvas.handle_mouse_down()
    ↓
点击检测（矩形碰撞）
    ↓
cx.emit(AssetSelectedEvent::NodeSelected(id))
    ↓
AssetDetailPanel.set_node(node, cx)
    ↓
self.selected_node = Some(node)
    ↓
cx.notify() → re-render
```

### 渲染流程
```
AssetsPanel.render()
    ├── TopologyCanvas.render()
    │   └── canvas() → paint connections & nodes
    └── AssetDetailPanel.render()
        ├── if selected_node.is_some()
        │   ├── AssetHeader
        │   ├── render_section_title()
        │   ├── render_info_card()
        │   ├── RiskBadge, StatusIndicator
        │   ├── PortList
        │   └── Action buttons
        └── else
            └── Empty state message
```

---

## 样式决策

### 为什么使用函数组件?
1. **生命周期**: SharedString 自动处理 &str 转换
2. **简洁**: 无需定义 RenderOnce trait
3. **灵活**: 易于组合和重用
4. **性能**: 减少中间类型转换

### 为什么右侧面板固定宽度?
1. **信息密度**: 列表/表格内容需要固定宽度
2. **可读性**: 防止行过长，影响阅读体验
3. **一致性**: 与 Dashboard 等其他模块保持一致

### 为什么用 HashMap 存储位置?
1. **查询**:  O(1) 时间复杂度
2. **无序**: 不需要维护顺序
3. **易用**: 与节点 ID 关联自然

---

## 已知技术债务

### 1. Pixels API 限制
```rust
// 不能这样做（私有字段）:
let dx = local_pos.x.0 - start_pos.x.0;

// 解决: 用 Bounds.contains() 进行碰撞检测
let hit_rect = Bounds::new(...);
if hit_rect.contains(&local_pos) { ... }
```

### 2. 拖拽未完成
- drag_state 框架已就位
- 需要 Pixels 减法操作
- 当前仅记录开始位置

### 3. 连接线样式固定
- 所有线条都是蓝色 (ACCENT_BLUE)
- 应根据 ConnectionStatus 改变颜色
- 需要修改 create_sample_connections()

---

## 测试覆盖

当前没有单元测试，因为：
1. UI 组件测试需要 GPUI test harness
2. 数据层已有充分测试
3. 集成测试更重要

**未来可测试的**:
- `calculate_node_positions()` → 单元测试
- `hit_test()` → 单元测试
- 事件流 → 集成测试

---

## 性能考虑

### 当前
- 4 个示例节点 → 可接受
- 10+ 连接线 → 性能良好
- 详情面板：30+ 卡片 → 可接受

### 瓶颈
- 节点数 > 100 时，拓扑图滚动可能卡顿
- 连接线数 > 200 时，SVG 渲染下降
- 无虚拟滚动支持

### 优化方案
1. **虚拟滚动**: 仅渲染可见节点
2. **WebGL 画布**: 用 wgpu 替代 SVG（未来）
3. **增量更新**: 只重绘变化的元素
4. **连接线聚合**: 多条线合并为带束

---

## 版本历史

### v0.1.0 (Phase 1) ✅
- [x] 基础容器架构
- [x] 拓扑图可视化
- [x] 详情面板框架
- [x] 信息卡片组件库
- [x] 编译通过，无警告

### v0.2.0 (Phase 2) - 计划
- [ ] 完整的拖拽支持
- [ ] 缩放和平移
- [ ] 连接线状态样式
- [ ] 动画过渡

### v0.3.0 (Phase 3) - 计划
- [ ] 扫描、编辑、删除按钮
- [ ] 操作确认对话框
- [ ] 资产编辑表单

### v0.4.0 (Phase 4) - 计划
- [ ] 搜索功能
- [ ] 分区过滤
- [ ] 资产类型过滤
- [ ] 高级查询

### v1.0.0 - 最终版本
- [ ] 完整功能
- [ ] 性能优化
- [ ] 完整测试覆盖
- [ ] 文档完成
