# 网络拓扑看板 - 资产分区布局实现计划

**目标**: 将TopologyCanvas改进为分区式拓扑看板，支持Z1-Z5分区布局，资产节点交互，及网络连接线绘制

**设计参考**: Assets_Expand.png 中的业务层级视图

---

## 一、架构概览

### 当前状态
- ✅ TopologyCanvas 基础框架已存在
- ✅ 分区数据结构 (ZoneType Z1-Z5)
- ✅ 资产数据模型 (AssetNode)
- ✅ 网络连接数据 (Connection)
- 🔄 布局算法需要改进 (当前简化)
- ❌ 网络连接线绘制未实现
- ❌ 分区卡片头部信息未完整

### 目标设计
```
TopologyCanvas (容器，支持滚动和缩放)
├── Header Section
│   ├── 标题: "网络拓扑 - 业务层级视图"
│   └── 图例 + 统计信息 (低危、中危、高危、严重 + 资产数 + 连接数)
│
└── Canvas Section (主画布)
    ├── 5个分区列 (Z1-Z5，宽度均分)
    │   ├── 分区卡片头
    │   │   ├── Z1-Z5 标签 + 图标
    │   │   ├── 描述文字
    │   │   ├── 资产数量
    │   │   └── "+" 按钮 (添加资产)
    │   │
    │   └── 资产节点区域 (中间部分)
    │       └── 多个 AssetNode (圆形 + 进度环)
    │
    └── 网络连接线层 (背景，连接各分区的节点)
        ├── 虚线连接 (灰色/绿色)
        ├── 实线连接 (紫色/橙色)
        └── 箭头方向标示
```

---

## 二、核心组件改进

### 2.1 TopologyCanvas 结构改进

**文件**: `crates/assets_ui/src/topology_canvas.rs`

#### 新增字段
```rust
pub struct TopologyCanvas {
    // 现有字段...
    nodes: Vec<AssetNode>,
    connections: Vec<Connection>,
    
    // 新增: 分区管理
    zones_data: HashMap<ZoneType, ZoneInfo>,  // Z1-Z5 分区数据
    node_positions: HashMap<String, NodePosition>,  // 节点位置映射
    
    // 交互状态
    selected_node_id: Option<String>,
    hovered_node_id: Option<String>,
    
    // 显示参数
    canvas_bounds: Bounds<Pixels>,
    zoom_level: f32,
    pan_x: f32,
    pan_y: f32,
    
    // 新增: 连接线样式
    connection_styles: HashMap<String, ConnectionStyle>,
}

// 分区信息
pub struct ZoneInfo {
    pub zone: ZoneType,
    pub name: String,           // "地面指挥中心", "通信网关层" 等
    pub color: u32,             // 分区背景色
    pub assets: Vec<String>,    // 该分区的资产ID列表
    pub position: ZonePosition, // 分区在画布中的位置
}

pub struct ZonePosition {
    pub x: f32,                 // 分区左边界
    pub y: f32,                 // 分区顶边界
    pub width: f32,             // 分区宽度
    pub height: f32,            // 分区高度
}

// 连接线样式
pub struct ConnectionStyle {
    pub color: Rgba,
    pub is_dashed: bool,
    pub width: f32,
}
```

### 2.2 布局计算算法改进

**核心逻辑**: `calculate_layout()`

```rust
fn calculate_layout(&mut self, canvas_width: f32, canvas_height: f32) {
    // 1. 初始化5个分区的边界
    let zone_width = canvas_width / 5.0;
    let zone_padding = 16.0;
    
    // 2. 为每个分区计算位置和资产节点位置
    for (zone_idx, zone_type) in [Z1, Z2, Z3, Z4, Z5].iter().enumerate() {
        let zone_x = zone_idx as f32 * zone_width;
        let zone_y = HEADER_HEIGHT;  // 分区标题下方
        
        // 2.1 计算该分区的资产列表
        let zone_assets = self.nodes.iter()
            .filter(|n| n.zone == *zone_type)
            .collect::<Vec<_>>();
        
        // 2.2 计算每个资产在该分区内的位置（网格或圆形排列）
        for (asset_idx, asset) in zone_assets.iter().enumerate() {
            let pos = self.calculate_node_position_in_zone(
                zone_x, zone_y, zone_width, zone_height,
                asset_idx, zone_assets.len()
            );
            self.node_positions.insert(asset.id.clone(), pos);
        }
    }
    
    // 3. 计算连接线的路径 (贝塞尔曲线或直线+弧线混合)
    self.recalculate_connections();
}

// 分区内节点位置计算
fn calculate_node_position_in_zone(
    &self,
    zone_x: f32,
    zone_y: f32,
    zone_width: f32,
    zone_height: f32,
    node_idx: usize,
    total_nodes: usize,
) -> NodePosition {
    let inner_padding = 20.0;
    let inner_width = zone_width - 2.0 * inner_padding;
    let inner_height = zone_height - 2.0 * inner_padding;
    
    // 网格排列: 2列或3列布局
    let cols = if total_nodes > 4 { 2 } else { 1 };
    let col = node_idx % cols;
    let row = node_idx / cols;
    
    let x = zone_x + inner_padding + col as f32 * (inner_width / cols as f32) + inner_width / (2.0 * cols as f32);
    let y = zone_y + inner_padding + row as f32 * (inner_height / 3.0) + inner_height / 6.0;
    
    NodePosition { x, y }
}
```

---

## 三、渲染改进

### 3.1 TopologyCanvas 渲染结构

```rust
impl Render for TopologyCanvas {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // 1. 更新布局 (响应式)
        self.update_layout_if_needed(cx);
        
        // 2. 构建UI树
        v_flex()
            .flex_1()
            .bg(rgb(BG_CARD))
            .size_full()
            
            // 2.1 头部 (标题 + 图例 + 统计)
            .child(self.render_header())
            
            // 2.2 主画布 (5个分区 + 连接线)
            .child(self.render_canvas())
    }
}
```

### 3.2 分区卡片渲染 (TopologyZone改进)

**文件**: `crates/assets_ui/src/components/topology_zone.rs`

```rust
pub fn render_topology_zone_kanban(zone: &TopologyZone) -> impl IntoElement {
    v_flex()
        .flex_1()
        .gap_0()
        .rounded_lg()
        .bg(rgb(zone.zone_color()))
        .border_1()
        .border_color(rgb(BORDER_COLOR))
        .overflow_hidden()
        
        // 分区卡片头
        .child(
            h_flex()
                .gap_2()
                .p_3()
                .bg(rgb(ZONE_HEADER_BG))
                .border_b_1()
                .border_color(rgb(BORDER_COLOR))
                .items_center()
                
                // 区域图标
                .child(self.render_zone_icon())
                
                // 区域标签和描述
                .child(
                    v_flex()
                        .gap_1()
                        .flex_1()
                        .child(Label::new(zone.zone_title()))
                        .child(Label::new(zone.zone_description()))
                )
                
                // 资产数量
                .child(
                    div()
                        .text_center()
                        .child(Label::new(format!("{}", zone.assets.len())))
                        .child(Label::new("资产"))
                )
                
                // 添加按钮
                .child(render_add_button())
        )
        
        // 分区内容区域 (资产节点)
        .child(
            v_flex()
                .flex_1()
                .p_4()
                .gap_2()
                .children(
                    zone.assets.iter().map(|asset| {
                        render_topology_asset_node(asset)
                    })
                )
        )
}
```

### 3.3 资产节点渲染 (新增)

**文件**: `crates/assets_ui/src/components/topology_node.rs` (新建)

```rust
pub fn render_topology_asset_node(node: &AssetNode) -> impl IntoElement {
    let severity_color = get_severity_color(&node.severity);
    let progress = node.scan_progress.percentage as f32 / 100.0;
    
    div()
        .relative()
        .w(px(64.0))
        .h(px(64.0))
        .m_auto()
        .cursor_pointer()
        .on_mouse_enter(cx.listener(move |this, _, _, _cx| {
            this.hovered_node_id = Some(node.id.clone());
        }))
        
        // 外圈: 进度环
        .child(
            div()
                .absolute()
                .inset_0()
                .rounded_full()
                .border(progress_stroke)
        )
        
        // 内圈: 资产颜色圆
        .child(
            div()
                .absolute()
                .inset_2()
                .rounded_full()
                .bg(rgb(severity_color))
        )
        
        // 悬停时显示提示框
        .when_some(self.hovered_node_id == Some(node.id.clone()), |this| {
            this.child(render_node_tooltip(node))
        })
}
```

### 3.4 网络连接线绘制 (新增)

**文件**: `crates/assets_ui/src/components/topology_connections.rs` (新建)

```rust
pub fn render_topology_connections(
    canvas: &TopologyCanvas,
    canvas_bounds: Bounds<Pixels>,
) -> impl IntoElement {
    canvas()
        .on_paint(|_bounds, cx| {
            // 为每条连接绘制连接线
            for connection in &canvas.connections {
                if let (Some(from_pos), Some(to_pos)) = (
                    canvas.node_positions.get(&from_id),
                    canvas.node_positions.get(&connection.target_id)
                ) {
                    // 选择连接线样式
                    let style = canvas.connection_styles
                        .get(&connection.target_id)
                        .cloned()
                        .unwrap_or_default();
                    
                    // 绘制贝塞尔曲线
                    let path = PathBuilder::new()
                        .move_to(point(from_pos.x, from_pos.y))
                        .cubic_curve_to(
                            // 控制点1
                            point(from_pos.x + 50.0, from_pos.y),
                            // 控制点2
                            point(to_pos.x - 50.0, to_pos.y),
                            // 终点
                            point(to_pos.x, to_pos.y),
                        )
                        .build();
                    
                    if style.is_dashed {
                        cx.paint_path(path, paint::stroke(style.color, style.width, StrokeStyle::dashed(...)));
                    } else {
                        cx.paint_path(path, paint::stroke(style.color, style.width));
                    }
                }
            }
        })
}
```

---

## 四、交互实现

### 4.1 节点选择事件

```rust
fn handle_node_click(&mut self, node_id: String, cx: &mut Context<Self>) {
    self.selected_node_id = Some(node_id.clone());
    cx.emit(AssetSelectedEvent::NodeSelected(node_id));
    cx.notify();
}
```

### 4.2 节点悬停提示

```rust
fn render_node_tooltip(&self, node: &AssetNode) -> impl IntoElement {
    div()
        .absolute()
        .bg(rgb(0x1f2937))  // 深灰背景
        .rounded_md()
        .p_2()
        .text_xs()
        .text_color(rgb(0xffffff))
        .shadow_lg()
        .child(format!("{}\n{}\n风险: {} · {} 漏洞", 
            node.name,
            node.ip_address,
            node.risk_score,
            node.vulnerabilities_count
        ))
}
```

---

## 五、实现任务分解

### Phase 1: 数据结构和布局算法 (2小时)
- [ ] 添加ZoneInfo, ZonePosition, ConnectionStyle结构体
- [ ] 实现calculate_layout()布局计算
- [ ] 实现calculate_node_position_in_zone()分区内位置计算
- [ ] 单元测试布局算法

### Phase 2: 分区卡片渲染 (1.5小时)
- [ ] 改进TopologyZone组件结构
- [ ] 实现分区卡片头部 (标签+描述+资产数+按钮)
- [ ] 实现分区背景色渲染
- [ ] 测试分区布局显示

### Phase 3: 资产节点渲染 (1.5小时)
- [ ] 新建topology_node.rs组件
- [ ] 实现节点圆形渲染 (带进度环)
- [ ] 实现节点颜色映射 (根据severity)
- [ ] 实现节点悬停提示

### Phase 4: 网络连接线 (2小时)
- [ ] 新建topology_connections.rs组件
- [ ] 实现贝塞尔曲线路径计算
- [ ] 实现虚线/实线样式区分
- [ ] 实现箭头方向标示

### Phase 5: 交互和事件处理 (1小时)
- [ ] 实现节点点击选择
- [ ] 实现节点悬停高亮
- [ ] 实现与AssetDetailPanel的事件联动
- [ ] 测试完整交互流程

### Phase 6: 样式微调和优化 (1小时)
- [ ] 响应式布局测试
- [ ] 颜色和间距调整
- [ ] 边界情况处理
- [ ] 性能优化

---

## 六、数据流图

```
User clicks node
    ↓
TopologyCanvas.handle_mouse_down()
    ↓
检测节点碰撞 (使用节点位置 + 大小)
    ↓
YES: cx.emit(AssetSelectedEvent::NodeSelected(id))
    ↓
AssetsPanel 接收事件
    ↓
更新AssetDetailPanel.selected_asset
    ↓
cx.notify() → 重新渲染
```

---

## 七、文件清单

### 修改文件
- [ ] `crates/assets_ui/src/topology_canvas.rs` - 核心改进
- [ ] `crates/assets_ui/src/lib.rs` - 调整主容器
- [ ] `crates/assets_ui/src/components/topology_zone.rs` - 分区卡片改进

### 新建文件
- [ ] `crates/assets_ui/src/components/topology_node.rs` - 资产节点
- [ ] `crates/assets_ui/src/components/topology_connections.rs` - 连接线

---

## 八、样式常量

```rust
// 分区颜色 (来自Assets_Expand.png)
const ZONE_Z1_COLOR: u32 = 0xe3f2fd;  // 蓝色
const ZONE_Z2_COLOR: u32 = 0xf1f8e9;  // 绿色
const ZONE_Z3_COLOR: u32 = 0xfce4ec;  // 粉色
const ZONE_Z4_COLOR: u32 = 0xfff3e0;  // 橙色
const ZONE_Z5_COLOR: u32 = 0xf3e5f5;  // 紫色

// 连接线颜色
const CONNECTION_NORMAL: u32 = 0xb0bec5;    // 灰色虚线
const CONNECTION_ACTIVE: u32 = 0x7c3aed;    // 紫色实线
const CONNECTION_ERROR: u32 = 0xef4444;     // 红色实线

// 尺寸
const NODE_SIZE: f32 = 48.0;
const NODE_HOVER_SIZE: f32 = 56.0;
const PROGRESS_RING_WIDTH: f32 = 3.0;

// 分区布局
const HEADER_HEIGHT: f32 = 60.0;
const ZONE_PADDING: f32 = 16.0;
const ZONE_INNER_PADDING: f32 = 20.0;
```

---

## 九、测试检查清单

- [ ] 5个分区正确显示
- [ ] 资产节点在正确的分区列中
- [ ] 网络连接线正确连接节点
- [ ] 节点点击事件触发
- [ ] 节点悬停显示提示信息
- [ ] 详情面板与拓扑图交互
- [ ] 响应式布局测试 (不同窗口宽度)
- [ ] 性能测试 (100+ 资产节点)

---

## 十、参考资源

- **设计稿**: Assets_Expand.png (位置: /Users/fk/Devlopment/uavred/interface_pic/)
- **GPUI Canvas**: https://github.com/zed-industries/zed/tree/main/crates/gpui
- **当前实现**: `crates/assets_ui/src/topology_canvas.rs`
- **数据模型**: `crates/data/src/models.rs`
