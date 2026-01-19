# Assets UI Layout Restructure Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** 重新设计 Assets UI 为垂直列表布局，支持可展开/收起的两个主行：网络拓扑看板和资产详情面板

**Architecture:** 
- 改为 v_flex 垂直布局替代当前 h_flex
- 创建通用的可展开行（CollapsibleRow）组件
- 拓扑看板内部为 5 个竖直区域（Z1-Z5）并排显示
- 资产点作为圆形元素放在对应区域
- 资产间的拓扑关系用线和箭头连接
- 第二行为资产详情，仅在选中资产时展开

**Tech Stack:** 
- GPUI (v_flex, div, canvas for connections)
- gpui-component (label, icons)
- data::models (AssetNode, ZoneType, etc.)

---

## Task 1: 创建可展开行组件框架

**文件：**
- 创建: `crates/assets_ui/src/components/collapsible_row.rs`
- 修改: `crates/assets_ui/src/components/mod.rs`
- 修改: `crates/assets_ui/src/lib.rs` (导入并使用)

**Step 1: 定义 CollapsibleRow 组件结构**

```rust
// crates/assets_ui/src/components/collapsible_row.rs
use gpui::*;
use gpui_component::{h_flex, label::Label, v_flex, IconName};
use ui::theme::*;

#[derive(Clone)]
pub struct CollapsibleRow {
    pub title: SharedString,
    pub icon: IconName,
    pub is_expanded: bool,
}

impl CollapsibleRow {
    pub fn new(title: impl Into<SharedString>, icon: IconName) -> Self {
        Self {
            title: title.into(),
            icon,
            is_expanded: true, // 默认展开
        }
    }

    pub fn expanded(mut self, expanded: bool) -> Self {
        self.is_expanded = expanded;
        self
    }
}

pub fn render_collapsible_row_header(
    row: &CollapsibleRow,
    on_toggle: impl Fn(&mut CollapsibleRow, &mut App) + 'static,
) -> impl IntoElement {
    h_flex()
        .gap_2()
        .p_3()
        .border_b_1()
        .border_color(rgb(BORDER_COLOR))
        .bg(rgb(BG_PRIMARY))
        .items_center()
        .child(
            if row.is_expanded {
                IconName::ChevronDown
            } else {
                IconName::ChevronRight
            }
        )
        .child(row.icon)
        .child(
            Label::new(row.title.clone())
                .text_sm()
                .font_weight(FontWeight::SEMIBOLD)
        )
}
```

**Step 2: 更新 components/mod.rs**

```rust
// 在 crates/assets_ui/src/components/mod.rs 中添加
pub mod collapsible_row;
pub use collapsible_row::{CollapsibleRow, render_collapsible_row_header};
```

**Step 3: 编译验证**

运行: `cargo build -p assets_ui 2>&1 | grep error`
期望: 无错误

**Step 4: 提交**

```bash
cd /Users/fk/Devlopment/uavred
git add crates/assets_ui/src/components/collapsible_row.rs
git add crates/assets_ui/src/components/mod.rs
git commit -m "feat: add collapsible row component structure"
```

---

## Task 2: 创建拓扑看板区域组件

**文件：**
- 创建: `crates/assets_ui/src/components/topology_zone.rs`
- 修改: `crates/assets_ui/src/components/mod.rs`

**Step 1: 定义 TopologyZone 结构**

```rust
// crates/assets_ui/src/components/topology_zone.rs
use gpui::*;
use gpui_component::{label::Label, v_flex};
use ui::theme::*;
use data::models::{ZoneType, AssetNode};

pub struct TopologyZone {
    pub zone: ZoneType,
    pub assets: Vec<AssetNode>,
}

impl TopologyZone {
    pub fn new(zone: ZoneType, assets: Vec<AssetNode>) -> Self {
        Self { zone, assets }
    }

    fn zone_title(&self) -> &'static str {
        match self.zone {
            ZoneType::Z1 => "Z1",
            ZoneType::Z2 => "Z2",
            ZoneType::Z3 => "Z3",
            ZoneType::Z4 => "Z4",
            ZoneType::Z5 => "Z5",
        }
    }

    fn zone_color(&self) -> u32 {
        match self.zone {
            ZoneType::Z1 => 0xe3f2fd,  // 蓝色背景
            ZoneType::Z2 => 0xf1f8e9,  // 绿色背景
            ZoneType::Z3 => 0xfce4ec,  // 粉红背景
            ZoneType::Z4 => 0xfff3e0,  // 橙色背景
            ZoneType::Z5 => 0xf3e5f5,  // 紫色背景
        }
    }
}

pub fn render_topology_zone(zone: &TopologyZone) -> impl IntoElement {
    v_flex()
        .flex_1()
        .gap_2()
        .p_4()
        .rounded_lg()
        .bg(rgb(zone.zone_color()))
        .border_1()
        .border_color(rgb(BORDER_COLOR))
        .items_center()
        .child(
            Label::new(zone.zone_title())
                .text_sm()
                .font_weight(FontWeight::SEMIBOLD)
        )
        .children(
            zone.assets.iter().map(|asset| {
                // 资产点：圆形元素
                div()
                    .size(px(40.0), px(40.0))
                    .rounded_full()
                    .bg(rgb(0x2563eb))
                    .into_any_element()
            })
        )
}
```

**Step 2: 更新 mod.rs**

```rust
pub mod topology_zone;
pub use topology_zone::{TopologyZone, render_topology_zone};
```

**Step 3: 编译验证**

运行: `cargo build -p assets_ui 2>&1 | grep error`
期望: 无错误

**Step 4: 提交**

```bash
git add crates/assets_ui/src/components/topology_zone.rs
git commit -m "feat: add topology zone area component"
```

---

## Task 3: 重构主 AssetsPanel 为垂直布局

**文件：**
- 修改: `crates/assets_ui/src/lib.rs` (完全重写)
- 修改: `crates/assets_ui/src/asset_detail_panel.rs` (简化)
- 修改: `crates/assets_ui/src/topology_canvas.rs` (改为区域管理)

**Step 1: 重写 AssetsPanel 为垂直布局**

```rust
// crates/assets_ui/src/lib.rs
use gpui::*;
use gpui_component::{h_flex, label::Label, v_flex, IconName};
use ui::theme::*;
use data::models::AssetNode;

mod components;
mod asset_detail_panel;
mod topology_canvas;

pub use asset_detail_panel::AssetDetailPanel;
pub use topology_canvas::TopologyCanvas;
pub use components::*;

pub struct AssetsPanel {
    topology_expanded: bool,
    details_expanded: bool,
    topology_canvas: Entity<TopologyCanvas>,
    asset_detail_panel: Entity<AssetDetailPanel>,
    selected_asset: Option<AssetNode>,
}

impl AssetsPanel {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let topology_canvas = cx.new(TopologyCanvas::new);
        let asset_detail_panel = cx.new(AssetDetailPanel::new);

        Self {
            topology_expanded: true,
            details_expanded: false,
            topology_canvas,
            asset_detail_panel,
            selected_asset: None,
        }
    }
}

impl Render for AssetsPanel {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .gap_0()
            .bg(rgb(BG_PRIMARY))
            // Row 1: Network Topology
            .child(
                v_flex()
                    .flex_none()
                    .gap_0()
                    .bg(rgb(BG_CARD))
                    .border_b_1()
                    .border_color(rgb(BORDER_COLOR))
                    .child(
                        h_flex()
                            .gap_2()
                            .p_3()
                            .items_center()
                            .child(
                                if self.topology_expanded {
                                    IconName::ChevronDown
                                } else {
                                    IconName::ChevronRight
                                }
                            )
                            .child(IconName::Network)
                            .child(
                                Label::new("网络拓扑 - 业务层级视图")
                                    .text_sm()
                                    .font_weight(FontWeight::SEMIBOLD)
                            )
                    )
                    .when(self.topology_expanded, |this| {
                        this.child(
                            self.topology_canvas.clone()
                                .into_any_element()
                        )
                    })
            )
            // Row 2: Asset Details
            .child(
                v_flex()
                    .flex_1()
                    .gap_0()
                    .bg(rgb(BG_CARD))
                    .child(
                        h_flex()
                            .gap_2()
                            .p_3()
                            .items_center()
                            .child(
                                if self.details_expanded {
                                    IconName::ChevronDown
                                } else {
                                    IconName::ChevronRight
                                }
                            )
                            .child(IconName::FileText)
                            .child(
                                Label::new("资产详情")
                                    .text_sm()
                                    .font_weight(FontWeight::SEMIBOLD)
                            )
                    )
                    .when(self.details_expanded && self.selected_asset.is_some(), |this| {
                        this.child(
                            self.asset_detail_panel.clone()
                                .into_any_element()
                        )
                    })
                    .when(self.details_expanded && self.selected_asset.is_none(), |this| {
                        this.child(
                            div()
                                .flex_1()
                                .items_center()
                                .justify_center()
                                .p_6()
                                .child(
                                    Label::new("选择一个资产来查看详情")
                                        .text_sm()
                                        .text_color(rgb(TEXT_MUTED))
                                )
                        )
                    })
            )
    }
}
```

**Step 2: 修改 topology_canvas.rs 支持区域视图**

在 TopologyCanvas 中添加方法：

```rust
impl TopologyCanvas {
    pub fn group_by_zone(&self) -> Vec<(ZoneType, Vec<&AssetNode>)> {
        let mut zones: std::collections::HashMap<ZoneType, Vec<&AssetNode>> = std::collections::HashMap::new();
        for node in &self.nodes {
            zones.entry(node.zone.clone())
                .or_insert_with(Vec::new)
                .push(node);
        }
        let mut result: Vec<_> = zones.into_iter().collect();
        result.sort_by_key(|(zone, _)| format!("{:?}", zone));
        result
    }
}

impl Render for TopologyCanvas {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let zones = self.group_by_zone();
        
        v_flex()
            .flex_none()
            .h(px(400.0))  // 固定高度
            .gap_2()
            .p_4()
            .bg(rgb(BG_CARD))
            .child(
                // 5个区域并排
                h_flex()
                    .flex_1()
                    .gap_2()
                    .children(
                        zones.iter().map(|(zone, assets)| {
                            render_topology_zone(&TopologyZone::new(
                                zone.clone(),
                                assets.iter().map(|a| (*a).clone()).collect()
                            )).into_any_element()
                        })
                    )
            )
    }
}
```

**Step 3: 编译验证**

运行: `cargo build -p assets_ui 2>&1 | head -50`
期望: 成功编译

**Step 4: 提交**

```bash
git add crates/assets_ui/src/lib.rs
git add crates/assets_ui/src/topology_canvas.rs
git commit -m "refactor: change layout from horizontal to vertical collapsible rows"
```

---

## Task 4: 连接关系渲染和交互

**文件：**
- 修改: `crates/assets_ui/src/topology_canvas.rs` (添加 canvas 连接线)
- 修改: `crates/assets_ui/src/components/topology_zone.rs` (可点击的资产点)

**Step 1: 在 topology_canvas 中添加 canvas 渲染连接线**

```rust
fn render_connections(&self, window: &mut Window) {
    for connection in &self.connections {
        // 从 from 节点到 to 节点绘制线
        // 这里需要实现线和箭头的绘制
        // 使用 window.paint_path() 绘制
    }
}
```

**Step 2: 编译验证**

运行: `cargo build -p assets_ui 2>&1 | grep error`
期望: 无错误

**Step 3: 提交**

```bash
git commit -m "feat: add connection rendering in topology canvas"
```

---

## Task 5: 集成资产选择和展开逻辑

**文件：**
- 修改: `crates/assets_ui/src/lib.rs` (添加事件处理)
- 修改: `crates/assets_ui/src/asset_detail_panel.rs` (与选择同步)

**关键实现：**
- 点击资产点时，更新 `selected_asset`
- 自动展开 `details_expanded`
- 调用 `asset_detail_panel.set_node()`

---

## Task 6: 测试与完成

**验证清单：**
- [ ] 拓扑看板在Row 1展开时显示 Z1-Z5 五个区域
- [ ] 点击资产点能选中并展开Row 2详情
- [ ] 资产详情正确显示选中资产信息
- [ ] 行头的展开/收起图标响应正确
- [ ] 布局在不同窗口大小下合理

---

## 总结变化

| 当前 | 新设计 |
|------|--------|
| 水平布局 h_flex | 垂直布局 v_flex |
| 左侧拓扑，右侧详情 | 上面拓扑，下面详情 |
| 两个固定面板 | 两个可展开行 |
| 节点在单个画布 | Z1-Z5 五个区域 |
| 无行标题 | 带展开/收起的行标题 |

---

**执行方式:** 
我建议使用 **Subagent-Driven** 方式逐任务执行，每完成一个任务我进行代码审查。
