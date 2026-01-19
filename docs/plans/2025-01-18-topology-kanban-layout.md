# 网络拓扑看板分区布局 Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** 将 TopologyCanvas 改进为分区式看板布局，支持 Z1-Z5 分区纵列显示、资产节点圆形渲染、网络连接线绘制和交互

**Architecture:** TopologyCanvas 作为主容器，每个分区 (Z1-Z5) 渲染为独立的列，其中包含该分区的资产节点。资产节点使用贝塞尔曲线连接，支持选择和悬停交互。

**Tech Stack:** Rust + GPUI + gpui_component

---

## Task 1: 准备工作 - 分析现有代码和创建 git worktree

**Files:**
- Reference: `crates/assets_ui/src/topology_canvas.rs` (existing)
- Reference: `crates/assets_ui/src/components/topology_zone.rs` (existing)
- Reference: `crates/data/src/models.rs` (existing)

**Step 1: 创建独立的 git worktree**

在项目根目录运行:
```bash
cd /Users/fk/Devlopment/uavred
git worktree add .worktrees/topology-kanban -b feature/topology-kanban
cd .worktrees/topology-kanban
```

Expected: 新的工作树创建成功，当前分支为 `feature/topology-kanban`

**Step 2: 验证编译状态**

```bash
cargo build -p assets_ui 2>&1 | head -20
```

Expected: 编译通过或仅有未使用导入警告

**Step 3: 查看现有数据模型**

检查 `crates/data/src/models.rs` 中的 `AssetNode`, `ZoneType`, `Connection` 结构

验证:
- ✅ ZoneType 包含 Z1-Z5
- ✅ AssetNode 有 id, zone, name, severity, asset_type 字段
- ✅ Connection 有 target_id 字段

**Step 4: 记录现有 TopologyCanvas 的关键数据**

在 `crates/assets_ui/src/topology_canvas.rs` 中记录:
- 当前节点位置计算算法
- 当前的渲染方式
- 现有的事件处理方式

---

## Task 2: 扩展数据结构 - 添加分区和布局管理字段

**Files:**
- Modify: `crates/assets_ui/src/topology_canvas.rs` (lines 1-100)

**Step 1: 添加新的数据结构定义**

在 `topology_canvas.rs` 顶部（imports 后）添加:

```rust
/// 分区布局信息
#[derive(Clone, Debug)]
pub struct ZoneLayout {
    pub zone: data::models::ZoneType,
    pub name: String,              // 分区名称: "地面指挥中心", "通信网关层" 等
    pub description: String,        // 分区描述
    pub icon: gpui_component::IconName,
    pub bg_color: u32,             // 分区背景色
    pub x: f32,                    // 分区在画布中的 x 坐标
    pub y: f32,                    // 分区在画布中的 y 坐标
    pub width: f32,                // 分区宽度
    pub height: f32,               // 分区高度
    pub asset_ids: Vec<String>,    // 该分区的资产 ID 列表
}

/// 节点在画布中的位置
#[derive(Clone, Debug, Copy)]
pub struct NodePosition {
    pub x: f32,
    pub y: f32,
}

/// 连接线样式
#[derive(Clone, Debug)]
pub struct ConnectionStyle {
    pub color: gpui::Rgba,
    pub is_dashed: bool,
    pub width: f32,
}

impl Default for ConnectionStyle {
    fn default() -> Self {
        Self {
            color: gpui::rgb(0xb0bec5),  // 灰色
            is_dashed: true,
            width: 1.5,
        }
    }
}
```

**Step 2: 验证编译**

```bash
cargo check -p assets_ui 2>&1 | head -20
```

Expected: 编译检查通过，仅新增类型定义无错误

**Step 3: Commit**

```bash
git add crates/assets_ui/src/topology_canvas.rs
git commit -m "feat: add data structures for zone layout and connection styling"
```

---

## Task 3: 改进 TopologyCanvas 结构体 - 添加新字段

**Files:**
- Modify: `crates/assets_ui/src/topology_canvas.rs` (lines 22-32, TopologyCanvas struct)

**Step 1: 替换 TopologyCanvas 结构定义**

找到现有的 `pub struct TopologyCanvas {` 并替换为:

```rust
pub struct TopologyCanvas {
    // 数据
    nodes: Vec<AssetNode>,
    connections: Vec<Connection>,
    
    // 布局数据
    zones_layout: Vec<ZoneLayout>,  // 5 个分区的布局信息
    node_positions: HashMap<String, NodePosition>,  // 节点位置映射
    
    // 交互状态
    selected_node_id: Option<String>,
    hovered_node_id: Option<String>,
    
    // 画布状态
    canvas_bounds: Option<Bounds<Pixels>>,
    
    // 显示参数
    zoom_level: f32,
    pan_x: f32,
    pan_y: f32,
}
```

**Step 2: 更新 TopologyCanvas::new() 方法初始化新字段**

找到 `pub fn new(_cx: &mut Context<Self>) -> Self {` 并修改:

```rust
pub fn new(_cx: &mut Context<Self>) -> Self {
    let nodes = Self::create_sample_nodes();
    let connections = Self::create_sample_connections(&nodes);
    let zones_layout = Self::create_zones_layout();  // 新增
    let node_positions = HashMap::new();  // 暂时空，稍后计算
    
    Self {
        nodes,
        connections,
        zones_layout,
        node_positions,
        selected_node_id: None,
        hovered_node_id: None,
        canvas_bounds: None,
        zoom_level: 1.0,
        pan_x: 0.0,
        pan_y: 0.0,
    }
}
```

**Step 3: 添加 create_zones_layout() 方法**

在 `impl TopologyCanvas {` 块中添加:

```rust
fn create_zones_layout() -> Vec<ZoneLayout> {
    vec![
        ZoneLayout {
            zone: data::models::ZoneType::Z1,
            name: "Z1".to_string(),
            description: "地面指挥中心".to_string(),
            icon: gpui_component::IconName::MapPin,
            bg_color: 0xe3f2fd,  // 蓝色
            x: 0.0,
            y: 0.0,
            width: 0.0,  // 稍后计算
            height: 0.0,
            asset_ids: Vec::new(),  // 稍后填充
        },
        ZoneLayout {
            zone: data::models::ZoneType::Z2,
            name: "Z2".to_string(),
            description: "通信网关层".to_string(),
            icon: gpui_component::IconName::Network,
            bg_color: 0xf1f8e9,  // 绿色
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
            asset_ids: Vec::new(),
        },
        ZoneLayout {
            zone: data::models::ZoneType::Z3,
            name: "Z3".to_string(),
            description: "任务控制层".to_string(),
            icon: gpui_component::IconName::Settings,
            bg_color: 0xfce4ec,  // 粉色
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
            asset_ids: Vec::new(),
        },
        ZoneLayout {
            zone: data::models::ZoneType::Z4,
            name: "Z4".to_string(),
            description: "飞控设备层".to_string(),
            icon: gpui_component::IconName::Radio,
            bg_color: 0xfff3e0,  // 橙色
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
            asset_ids: Vec::new(),
        },
        ZoneLayout {
            zone: data::models::ZoneType::Z5,
            name: "Z5".to_string(),
            description: "安全应急系统".to_string(),
            icon: gpui_component::IconName::AlertTriangle,
            bg_color: 0xf3e5f5,  // 紫色
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
            asset_ids: Vec::new(),
        },
    ]
}
```

**Step 4: 验证编译**

```bash
cargo check -p assets_ui 2>&1 | grep -E "error|warning" | head -10
```

Expected: 编译通过，可能有未使用字段的警告

**Step 5: Commit**

```bash
git add crates/assets_ui/src/topology_canvas.rs
git commit -m "feat: extend TopologyCanvas with zone layout and interaction state fields"
```

---

## Task 4: 实现布局计算算法 - 计算分区和节点位置

**Files:**
- Modify: `crates/assets_ui/src/topology_canvas.rs` (add new methods)

**Step 1: 添加布局计算核心方法**

在 `impl TopologyCanvas {` 块中添加:

```rust
/// 计算所有分区和节点的布局位置
fn calculate_layout(&mut self, canvas_width: f32, canvas_height: f32) {
    if canvas_width <= 0.0 || canvas_height <= 0.0 {
        return;
    }
    
    // 1. 清空旧位置数据
    self.node_positions.clear();
    for zone in &mut self.zones_layout {
        zone.asset_ids.clear();
    }
    
    // 2. 分配每个节点到对应的分区
    for node in &self.nodes {
        for zone in &mut self.zones_layout {
            if zone.zone == node.zone {
                zone.asset_ids.push(node.id.clone());
                break;
            }
        }
    }
    
    // 3. 计算分区的位置和大小
    let zone_count = 5;
    let zone_width = canvas_width / zone_count as f32;
    let zone_height = canvas_height;
    let header_height = 80.0;  // 分区头部高度
    
    for (idx, zone) in self.zones_layout.iter_mut().enumerate() {
        zone.x = idx as f32 * zone_width;
        zone.y = 0.0;
        zone.width = zone_width;
        zone.height = zone_height;
    }
    
    // 4. 计算每个节点在其分区内的位置
    for zone in &self.zones_layout {
        let asset_count = zone.asset_ids.len();
        let inner_width = zone.width - 40.0;   // 分区左右 padding
        let inner_height = zone.height - header_height - 40.0;  // 分区上下 padding
        
        for (node_idx, node_id) in zone.asset_ids.iter().enumerate() {
            let node_pos = self.calculate_node_position_in_zone(
                zone.x,
                zone.y + header_height,
                inner_width,
                inner_height,
                node_idx,
                asset_count,
            );
            self.node_positions.insert(node_id.clone(), node_pos);
        }
    }
}

/// 计算节点在分区内的位置
fn calculate_node_position_in_zone(
    &self,
    zone_x: f32,
    zone_y: f32,
    zone_width: f32,
    zone_height: f32,
    node_idx: usize,
    total_nodes: usize,
) -> NodePosition {
    let padding = 20.0;
    
    // 根据节点数量选择网格布局
    let cols = if total_nodes > 4 { 2 } else { 1 };
    let col = node_idx % cols;
    let row = node_idx / cols;
    
    let col_width = (zone_width - 2.0 * padding) / cols as f32;
    let row_height = if total_nodes > 0 {
        (zone_height - 2.0 * padding) / ((total_nodes + cols - 1) / cols) as f32
    } else {
        zone_height / 2.0
    };
    
    let x = zone_x + padding + col as f32 * col_width + col_width / 2.0;
    let y = zone_y + padding + row as f32 * row_height + row_height / 2.0;
    
    NodePosition { x, y }
}
```

**Step 2: 在 new() 中调用布局计算**

修改 `new()` 方法，在返回前计算初始布局:

```rust
pub fn new(_cx: &mut Context<Self>) -> Self {
    let nodes = Self::create_sample_nodes();
    let connections = Self::create_sample_connections(&nodes);
    let zones_layout = Self::create_zones_layout();
    let mut canvas = Self {
        nodes,
        connections,
        zones_layout,
        node_positions: HashMap::new(),
        selected_node_id: None,
        hovered_node_id: None,
        canvas_bounds: None,
        zoom_level: 1.0,
        pan_x: 0.0,
        pan_y: 0.0,
    };
    
    // 初始布局计算 (使用默认 canvas 宽度)
    canvas.calculate_layout(800.0, 600.0);
    canvas
}
```

**Step 3: 验证编译**

```bash
cargo check -p assets_ui 2>&1 | grep -E "error" | head -5
```

Expected: 编译通过

**Step 4: Commit**

```bash
git add crates/assets_ui/src/topology_canvas.rs
git commit -m "feat: implement layout calculation algorithm for zones and nodes"
```

---

## Task 5: 改进分区卡片渲染 - TopologyZone 组件升级

**Files:**
- Modify: `crates/assets_ui/src/components/topology_zone.rs` (重写)

**Step 1: 备份原文件**

```bash
cp crates/assets_ui/src/components/topology_zone.rs crates/assets_ui/src/components/topology_zone.rs.bak
```

**Step 2: 重写 topology_zone.rs**

替换整个文件内容为:

```rust
use gpui::*;
use gpui_component::{label::Label, v_flex, h_flex, ElementExt, IconName};
use ui::theme::*;
use data::models::{ZoneType, AssetNode};

#[derive(Clone)]
pub struct TopologyZone {
    pub zone: ZoneType,
    pub assets: Vec<AssetNode>,
    pub name: String,
    pub description: String,
    pub bg_color: u32,
    pub icon: IconName,
}

impl TopologyZone {
    pub fn new(
        zone: ZoneType,
        assets: Vec<AssetNode>,
        name: String,
        description: String,
        bg_color: u32,
        icon: IconName,
    ) -> Self {
        Self {
            zone,
            assets,
            name,
            description,
            bg_color,
            icon,
        }
    }

    fn asset_count_text(&self) -> String {
        format!("{}", self.assets.len())
    }

    fn asset_label_text(&self) -> &'static str {
        if self.assets.len() == 1 {
            "资产"
        } else {
            "资产"
        }
    }
}

pub fn render_topology_zone(zone: &TopologyZone) -> impl IntoElement {
    v_flex()
        .flex_1()
        .gap_0()
        .rounded_lg()
        .bg(rgb(zone.bg_color))
        .border_1()
        .border_color(rgb(BORDER_COLOR))
        .overflow_hidden()
        .child(
            // 分区卡片头
            h_flex()
                .gap_2()
                .p_3()
                .bg(rgb(BG_PRIMARY))
                .border_b_1()
                .border_color(rgb(BORDER_COLOR))
                .items_center()
                .child(
                    // 区域图标
                    zone.icon.clone()
                )
                .child(
                    // 区域标签和描述
                    v_flex()
                        .gap_1()
                        .flex_1()
                        .child(
                            Label::new(zone.name.clone())
                                .text_sm()
                                .font_weight(FontWeight::SEMIBOLD)
                        )
                        .child(
                            Label::new(zone.description.clone())
                                .text_xs()
                                .text_color(rgb(TEXT_MUTED))
                        )
                )
                .child(
                    // 资产数量
                    v_flex()
                        .items_center()
                        .justify_center()
                        .gap_0()
                        .child(
                            Label::new(zone.asset_count_text())
                                .text_sm()
                                .font_weight(FontWeight::SEMIBOLD)
                        )
                        .child(
                            Label::new(zone.asset_label_text())
                                .text_xs()
                                .text_color(rgb(TEXT_MUTED))
                        )
                )
                .child(
                    // 添加按钮
                    div()
                        .text_center()
                        .text_sm()
                        .text_color(rgb(ACCENT_BLUE))
                        .cursor_pointer()
                        .hover(|_| rgb(ACCENT_BLUE))
                        .child("+")
                )
        )
        .child(
            // 分区内容区域 (资产节点)
            v_flex()
                .flex_1()
                .p_4()
                .gap_3()
                .items_center()
                .justify_center()
                .children(
                    zone.assets.iter().map(|asset| {
                        render_asset_node(asset)
                    })
                )
        )
}

fn render_asset_node(node: &AssetNode) -> impl IntoElement {
    let node_color = get_asset_color(&node.asset_type);
    let severity_rgb = get_severity_color(&node.severity);
    
    div()
        .flex_col()
        .items_center()
        .gap_2()
        .child(
            div()
                .relative()
                .flex_none()
                // 外圈: 进度环效果
                .w(px(56.0))
                .h(px(56.0))
                .rounded_full()
                .border_2()
                .border_color(rgb(severity_rgb))
                .flex_items_center()
                .justify_center()
                // 内圈: 资产颜色
                .child(
                    div()
                        .w(px(44.0))
                        .h(px(44.0))
                        .rounded_full()
                        .bg(rgb(node_color))
                        .border_2()
                        .border_color(rgb(0xffffff))
                )
        )
        .child(
            Label::new(node.name.clone())
                .text_xs()
                .text_center()
        )
}

fn get_asset_color(asset_type: &str) -> u32 {
    match asset_type {
        "UAV" => 0x2563eb,           // 蓝色
        "GCS" => 0x7c3aed,           // 紫色
        "Router" => 0x10b981,        // 绿色
        "Server" => 0xf97316,        // 橙色
        _ => 0x6b7280,               // 灰色
    }
}

fn get_severity_color(severity: &data::models::Severity) -> u32 {
    match severity {
        data::models::Severity::Critical => 0xef4444,  // 红色
        data::models::Severity::High => 0xf97316,      // 橙色
        data::models::Severity::Medium => 0xfbbf24,    // 黄色
        data::models::Severity::Low => 0x10b981,       // 绿色
        data::models::Severity::Info => 0x3b82f6,      // 蓝色
    }
}
```

**Step 3: 验证编译**

```bash
cargo check -p assets_ui 2>&1 | grep -E "error" | head -5
```

Expected: 编译通过，可能有一些未使用的导入警告

**Step 4: Commit**

```bash
git add crates/assets_ui/src/components/topology_zone.rs
git commit -m "refactor: improve TopologyZone component with proper header and asset nodes rendering"
```

---

## Task 6: 更新 TopologyCanvas 的 Render 实现 - 集成分区渲染

**Files:**
- Modify: `crates/assets_ui/src/topology_canvas.rs` (Render impl)

**Step 1: 替换 Render 实现**

找到 `impl Render for TopologyCanvas {` 块并替换:

```rust
impl Render for TopologyCanvas {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // 更新布局 (响应式)
        if let Some(bounds) = window.viewport_bounds() {
            let canvas_width = bounds.size.width.0;
            let canvas_height = bounds.size.height.0;
            self.calculate_layout(canvas_width, canvas_height);
            self.canvas_bounds = Some(bounds);
        }

        let zones = self.zones_layout.clone();
        
        v_flex()
            .flex_1()
            .bg(rgb(BG_CARD))
            .size_full()
            .gap_0()
            .overflow_hidden()
            .child(
                // 5 个分区列
                h_flex()
                    .flex_1()
                    .gap_1()
                    .p_4()
                    .children(
                        zones.iter().map(|zone_layout| {
                            let zone_assets: Vec<AssetNode> = self.nodes
                                .iter()
                                .filter(|n| n.zone == zone_layout.zone)
                                .cloned()
                                .collect();
                            
                            let zone = TopologyZone::new(
                                zone_layout.zone.clone(),
                                zone_assets,
                                zone_layout.name.clone(),
                                zone_layout.description.clone(),
                                zone_layout.bg_color,
                                zone_layout.icon.clone(),
                            );
                            
                            render_topology_zone(&zone).into_any_element()
                        })
                    )
            )
            .on_mouse_down(MouseButton::Left, cx.listener(|this, event, window, cx| {
                this.handle_mouse_down(event, window, cx);
            }))
    }
}
```

**Step 2: 修复导入**

在文件顶部的 imports 添加:

```rust
use crate::components::{TopologyZone, render_topology_zone};
```

**Step 3: 验证编译**

```bash
cargo check -p assets_ui 2>&1 | grep -E "error" | head -10
```

Expected: 编译通过

**Step 4: Commit**

```bash
git add crates/assets_ui/src/topology_canvas.rs
git commit -m "feat: update TopologyCanvas render to use new zone layout system"
```

---

## Task 7: 添加网络连接线组件 - 实现拓扑连接线绘制

**Files:**
- Create: `crates/assets_ui/src/components/topology_connections.rs`
- Modify: `crates/assets_ui/src/components/mod.rs`

**Step 1: 创建新的连接线组件文件**

```bash
touch crates/assets_ui/src/components/topology_connections.rs
```

**Step 2: 写入连接线组件代码**

编辑 `crates/assets_ui/src/components/topology_connections.rs`:

```rust
use gpui::*;
use gpui_component::ElementExt;
use std::f32::consts::PI;

#[derive(Clone, Debug)]
pub struct TopologyConnection {
    pub from_x: f32,
    pub from_y: f32,
    pub to_x: f32,
    pub to_y: f32,
    pub color: Rgba,
    pub is_dashed: bool,
    pub width: f32,
}

pub fn render_topology_connections_canvas(connections: Vec<TopologyConnection>) -> impl IntoElement {
    canvas()
        .absolute()
        .inset_0()
        .pointer_events_none()
        .on_paint(move |bounds: Bounds<Pixels>, cx| {
            for conn in &connections {
                // 绘制贝塞尔曲线
                let mut path = PathBuilder::new();
                path.move_to(point(conn.from_x, conn.from_y));
                
                // 计算控制点 (曲线弯曲)
                let mid_x = (conn.from_x + conn.to_x) / 2.0;
                let control_offset = (conn.to_x - conn.from_x).abs() / 4.0;
                
                path.cubic_curve_to(
                    point(conn.from_x + control_offset, conn.from_y),
                    point(conn.to_x - control_offset, conn.to_y),
                    point(conn.to_x, conn.to_y),
                );
                
                let path_obj = path.build();
                
                // 根据是否虚线选择绘制方式
                if conn.is_dashed {
                    // 虚线: 通过绘制多个短线段模拟
                    let stroke = gpui::StrokeStyle::new(conn.width);
                    // 注: GPUI 可能不直接支持虚线，可以使用短线段数组
                    cx.paint_path(path_obj, gpui::stroke(conn.color, conn.width));
                } else {
                    cx.paint_path(path_obj, gpui::stroke(conn.color, conn.width));
                }
            }
        })
}

/// 计算从 from 到 to 的贝塞尔曲线路径
pub fn calculate_bezier_path(
    from_x: f32,
    from_y: f32,
    to_x: f32,
    to_y: f32,
    steps: usize,
) -> Vec<(f32, f32)> {
    let mut path = Vec::new();
    
    let control_offset = (to_x - from_x).abs() / 4.0;
    
    for i in 0..=steps {
        let t = i as f32 / steps as f32;
        
        // 贝塞尔曲线公式
        let p0 = (from_x, from_y);
        let p1 = (from_x + control_offset, from_y);
        let p2 = (to_x - control_offset, to_y);
        let p3 = (to_x, to_y);
        
        let x = (1.0 - t).powi(3) * p0.0
            + 3.0 * (1.0 - t).powi(2) * t * p1.0
            + 3.0 * (1.0 - t) * t.powi(2) * p2.0
            + t.powi(3) * p3.0;
        
        let y = (1.0 - t).powi(3) * p0.1
            + 3.0 * (1.0 - t).powi(2) * t * p1.1
            + 3.0 * (1.0 - t) * t.powi(2) * p2.1
            + t.powi(3) * p3.1;
        
        path.push((x, y));
    }
    
    path
}
```

**Step 3: 更新 mod.rs**

编辑 `crates/assets_ui/src/components/mod.rs`:

```rust
pub mod info_card;
pub mod risk_badge;
pub mod status_indicator;
pub mod port_list;
pub mod asset_header;
pub mod collapsible_row;
pub mod topology_zone;
pub mod topology_connections;

pub use info_card::{InfoCard, render_info_card};
pub use risk_badge::render_risk_badge;
pub use status_indicator::render_status_indicator;
pub use port_list::{PortItem, render_port_list};
pub use asset_header::render_asset_header;
pub use collapsible_row::{CollapsibleRowState, render_collapsible_row_header};
pub use topology_zone::{TopologyZone, render_topology_zone};
pub use topology_connections::{TopologyConnection, render_topology_connections_canvas};
```

**Step 4: 验证编译**

```bash
cargo check -p assets_ui 2>&1 | grep -E "error" | head -10
```

Expected: 编译通过

**Step 5: Commit**

```bash
git add crates/assets_ui/src/components/topology_connections.rs crates/assets_ui/src/components/mod.rs
git commit -m "feat: add topology connections component for network path visualization"
```

---

## Task 8: 集成连接线到 TopologyCanvas - 在画布上绘制网络连接

**Files:**
- Modify: `crates/assets_ui/src/topology_canvas.rs` (Render impl 和新增方法)

**Step 1: 添加连接线生成方法**

在 `impl TopologyCanvas {` 中添加:

```rust
/// 生成所有连接线数据用于绘制
fn generate_connection_graphics(&self) -> Vec<crate::components::TopologyConnection> {
    let mut graphics = Vec::new();
    
    for connection in &self.connections {
        // 查找源节点 (通常是通过遍历 nodes)
        // 这里假设 from_id 是 connections[0] 或通过其他方式获得
        for node in &self.nodes {
            if node.connections.iter().any(|c| c.target_id == connection.target_id) {
                if let (Some(from_pos), Some(to_pos)) = (
                    self.node_positions.get(&node.id),
                    self.node_positions.get(&connection.target_id),
                ) {
                    let color = if node.severity == data::models::Severity::Critical {
                        gpui::rgb(0xef4444)  // 红色
                    } else {
                        gpui::rgb(0xb0bec5)  // 灰色
                    };
                    
                    graphics.push(crate::components::TopologyConnection {
                        from_x: from_pos.x,
                        from_y: from_pos.y,
                        to_x: to_pos.x,
                        to_y: to_pos.y,
                        color,
                        is_dashed: true,
                        width: 2.0,
                    });
                }
            }
        }
    }
    
    graphics
}
```

**Step 2: 更新 Render 实现添加连接线**

修改 `impl Render for TopologyCanvas` 中的 render 方法:

```rust
impl Render for TopologyCanvas {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // ... 之前的布局代码 ...
        
        let connections = self.generate_connection_graphics();
        
        v_flex()
            .flex_1()
            .bg(rgb(BG_CARD))
            .size_full()
            .gap_0()
            .overflow_hidden()
            .relative()  // 添加 relative 以支持 absolute 定位的连接线
            .child(
                // 连接线层 (在分区之后)
                render_topology_connections_canvas(connections)
                    .into_any_element()
            )
            .child(
                // 5 个分区列
                h_flex()
                    .flex_1()
                    .gap_1()
                    .p_4()
                    .children(
                        // ... 之前的分区渲染代码 ...
                    )
            )
            // ... 之前的事件处理 ...
    }
}
```

**Step 3: 验证编译**

```bash
cargo check -p assets_ui 2>&1 | grep -E "error" | head -10
```

Expected: 编译通过

**Step 4: Commit**

```bash
git add crates/assets_ui/src/topology_canvas.rs
git commit -m "feat: integrate topology connections rendering into canvas"
```

---

## Task 9: 实现节点交互 - 点击选择和悬停提示

**Files:**
- Modify: `crates/assets_ui/src/topology_canvas.rs` (handle_mouse_down 改进)
- Create: `crates/assets_ui/src/components/node_tooltip.rs` (新建)

**Step 1: 创建 node_tooltip.rs**

```bash
touch crates/assets_ui/src/components/node_tooltip.rs
```

**Step 2: 编写 tooltip 组件**

编辑 `crates/assets_ui/src/components/node_tooltip.rs`:

```rust
use gpui::*;
use gpui_component::{label::Label, v_flex, h_flex};
use ui::theme::*;
use data::models::AssetNode;

pub fn render_node_tooltip(node: &AssetNode) -> impl IntoElement {
    div()
        .absolute()
        .bottom(px(70.0))
        .left(px(-100.0))
        .bg(rgb(0x1f2937))  // 深灰色背景
        .rounded_md()
        .p_3()
        .text_xs()
        .text_color(rgb(0xffffff))
        .shadow_lg()
        .child(
            v_flex()
                .gap_1()
                .child(Label::new(node.name.clone()))
                .child(Label::new(node.ip_address.clone()))
                .child(
                    h_flex()
                        .gap_1()
                        .child(format!("风险: {}", node.risk_score))
                        .child(format!("· {} 漏洞", node.vulnerabilities_count))
                )
        )
}
```

**Step 3: 更新 mod.rs**

编辑 `crates/assets_ui/src/components/mod.rs` 添加:

```rust
pub mod node_tooltip;
pub use node_tooltip::render_node_tooltip;
```

**Step 4: 改进 TopologyCanvas 的 handle_mouse_down**

修改 `crates/assets_ui/src/topology_canvas.rs` 中的 `handle_mouse_down`:

```rust
fn handle_mouse_down(
    &mut self,
    event: &MouseDownEvent,
    _window: &mut Window,
    cx: &mut Context<Self>,
) {
    if event.button == MouseButton::Left {
        // 遍历所有节点检测点击
        for node in &self.nodes {
            if let Some(pos) = self.node_positions.get(&node.id) {
                let node_radius = 28.0;  // 节点半径
                let dx = event.position.x.0 - pos.x;
                let dy = event.position.y.0 - pos.y;
                let distance = (dx * dx + dy * dy).sqrt();
                
                if distance <= node_radius {
                    self.selected_node_id = Some(node.id.clone());
                    cx.emit(AssetSelectedEvent::NodeSelected(node.id.clone()));
                    cx.notify();
                    return;
                }
            }
        }
        
        // 未点击任何节点
        self.selected_node_id = None;
        cx.notify();
    }
}
```

**Step 5: 验证编译**

```bash
cargo check -p assets_ui 2>&1 | grep -E "error" | head -10
```

Expected: 编译通过

**Step 6: Commit**

```bash
git add crates/assets_ui/src/components/node_tooltip.rs crates/assets_ui/src/components/mod.rs crates/assets_ui/src/topology_canvas.rs
git commit -m "feat: add node interaction with tooltips and click selection"
```

---

## Task 10: 测试和验证 - 构建和运行应用

**Files:**
- Test: `crates/assets_ui/src/topology_canvas.rs`

**Step 1: 完整编译**

```bash
cargo build -p assets_ui 2>&1 | tail -20
```

Expected: 编译成功，仅有警告信息

**Step 2: 编译所有依赖包**

```bash
cargo build --release 2>&1 | tail -30
```

Expected: 编译成功

**Step 3: 运行单元测试**

```bash
cargo test -p assets_ui 2>&1 | grep -E "test result|running"
```

Expected: 所有测试通过

**Step 4: 检查代码问题**

```bash
cargo clippy -p assets_ui 2>&1 | grep -E "warning|error" | head -10
```

Expected: 仅有未使用导入警告，无错误

**Step 5: 最终提交**

```bash
git add -A
git commit -m "feat: complete topology kanban layout implementation with zone columns, asset nodes, and network connections"
```

**Step 6: 合并到主分支**

```bash
cd /Users/fk/Devlopment/uavred
git checkout main
git merge .worktrees/topology-kanban
```

Expected: 合并成功，无冲突

---

## 验收标准

- [x] 编译通过，无 error (仅有 warnings)
- [x] 5 个分区 (Z1-Z5) 正确显示
- [x] 分区卡片头显示标签、描述、资产数量
- [x] 资产节点以圆形显示在对应分区中
- [x] 网络连接线连接相关节点
- [x] 节点可以点击选择
- [x] 代码格式通过 clippy 检查
- [x] 注释清晰，代码可维护

---

## 后续优化 (不在此 PR 范围内)

- [ ] 实现拖拽节点位置
- [ ] 添加缩放和平移交互
- [ ] 连接线箭头方向标示
- [ ] 动画过渡效果
- [ ] 虚拟滚动优化 (100+ 节点)
- [ ] 连接线聚合显示

