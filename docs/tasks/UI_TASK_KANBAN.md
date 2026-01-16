# Kanban UI 开发任务 - 细粒度分解

## 概述

本文档包含 Kanban 模块和 ThreePanelLayout 组件的细粒度开发任务，
每个任务都是独立的、可验证的单元，适合 UI 子代理并行执行。

---

## Wave 1: 基础设施 (已完成)

### Task 1.1: 创建 kanban_ui crate 骨架 ✅

**状态**: 已完成

**文件**:
- `crates/kanban_ui/Cargo.toml`
- `crates/kanban_ui/src/lib.rs`
- `Cargo.toml` (workspace members)

**验证**: `cargo check --package kanban_ui`

---

### Task 1.2: 补充 theme 常量 ✅

**状态**: 已完成

**文件**: `crates/ui/src/theme.rs`

**新增常量**:
- `KANBAN_COLUMN_WIDTH`, `KANBAN_COLUMN_GAP`, `KANBAN_CARD_MIN_HEIGHT`
- `DETAIL_PANEL_WIDTH`, `DETAIL_PANEL_MIN_WIDTH`, `DETAIL_PANEL_MAX_WIDTH`
- `ANIMATION_FAST`, `ANIMATION_NORMAL`, `ANIMATION_SLOW`
- `STATUS_TODO_BG`, `STATUS_IN_PROGRESS_BG`, `STATUS_IN_REVIEW_BG`, `STATUS_DONE_BG`, `STATUS_CANCELED_BG`

**验证**: `cargo check --package ui`

---

### Task 1.3: 创建 layouts 模块骨架 ✅

**状态**: 已完成

**文件**:
- `crates/ui/src/layouts/mod.rs`
- `crates/ui/src/layouts/three_panel.rs`
- `crates/ui/src/lib.rs` (添加 layouts 导出)

**验证**: `cargo check --package ui`

---

## Wave 2: 核心组件实现

### Task 2.1: 完善 TaskCard 组件

**状态**: 基础实现完成，需要完善

**文件**: `crates/kanban_ui/src/task_card.rs`

**待办**:
1. [ ] 添加拖拽支持 (`on_drag`, `DragState`)
2. [ ] 添加右键菜单支持
3. [ ] 优化悬停动画效果

**代码修改**:

```rust
// 在 TaskCard 中添加拖拽支持
use gpui::DragState;

pub struct TaskCard {
    // ... 现有字段
    dragging: bool,
    on_drag_start: Option<Box<dyn Fn(usize, &mut Window, &mut App) + 'static>>,
}

impl TaskCard {
    pub fn on_drag_start(
        mut self,
        handler: impl Fn(usize, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_drag_start = Some(Box::new(handler));
        self
    }
}

// 在 render 中添加
.on_drag(DraggedTask { id: task_id }, |_, _, _, cx| {
    // 创建拖拽预览
    cx.new(|_| DraggedTaskPreview { id: task_id })
})
```

**验证**:
- `cargo check --package kanban_ui`
- 视觉测试: 拖拽任务卡片时显示半透明预览

---

### Task 2.2: 完善 KanbanColumn 组件

**状态**: 基础实现完成，需要完善

**文件**: `crates/kanban_ui/src/kanban_column.rs`

**待办**:
1. [ ] 修复 on_task_click 回调传递问题
2. [ ] 添加任务拖放目标区域
3. [ ] 添加空列占位符
4. [ ] 添加折叠/展开功能

**代码修改**:

```rust
// 修复回调传递 - 使用 Rc 包装
use std::rc::Rc;

pub struct KanbanColumn {
    // ...
    on_task_click: Option<Rc<dyn Fn(usize, &mut Window, &mut App) + 'static>>,
}

// 在 render 中正确传递回调
.children(self.tasks.into_iter().map(|task| {
    let task_id = task.id;
    let is_selected = selected_id == Some(task_id);
    let on_click = self.on_task_click.clone();

    TaskCard::new(task)
        .selected(is_selected)
        .on_click(move |_, window, cx| {
            if let Some(ref handler) = on_click {
                handler(task_id, window, cx);
            }
        })
}))
```

**验证**:
- `cargo check --package kanban_ui`
- 视觉测试: 点击任务卡片触发回调

---

### Task 2.3: 完善 ThreePanelLayout 组件

**状态**: 基础实现完成

**文件**: `crates/ui/src/layouts/three_panel.rs`

**待办**:
1. [ ] 添加面板最小/最大尺寸限制
2. [ ] 添加面板隐藏动画
3. [ ] 添加折叠按钮

**代码修改**:

```rust
pub struct ThreePanelLayout {
    // ... 现有字段
    first_min_size: Option<Pixels>,
    first_max_size: Option<Pixels>,
    last_min_size: Option<Pixels>,
    last_max_size: Option<Pixels>,
}

impl ThreePanelLayout {
    pub fn first_size_range(mut self, min: Pixels, max: Pixels) -> Self {
        self.first_min_size = Some(min);
        self.first_max_size = Some(max);
        self
    }

    pub fn last_size_range(mut self, min: Pixels, max: Pixels) -> Self {
        self.last_min_size = Some(min);
        self.last_max_size = Some(max);
        self
    }
}
```

**验证**:
- `cargo check --package ui`
- 视觉测试: 拖拽面板边界，确认尺寸限制生效

---

## Wave 3: 复合组件和动画

### Task 3.1: 完善 TaskDetailPanel 组件

**状态**: 基础实现完成，需要完善

**文件**: `crates/kanban_ui/src/task_detail.rs`

**待办**:
1. [ ] 添加编辑功能（修改任务名称、描述、优先级）
2. [ ] 添加状态切换按钮
3. [ ] 添加子任务列表
4. [ ] 添加评论区域

**验证**:
- `cargo check --package kanban_ui`
- 视觉测试: 详情面板显示完整任务信息

---

### Task 3.2: 完善 KanbanBoard Squeeze 动画

**状态**: 基础实现完成，需要调优

**文件**: `crates/kanban_ui/src/kanban_board.rs`

**待办**:
1. [ ] 优化动画流畅度
2. [ ] 添加关闭动画（收回面板）
3. [ ] 处理窗口大小变化时的重新布局

**关键代码**:

```rust
// 改进动画实现 - 使用 window.use_keyed_state 管理动画状态
fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
    let animation_state = window.use_keyed_state(
        ElementId::NamedInteger("detail-animation".into(), self.selected_task_id.unwrap_or(0)),
        cx,
        |_, _| AnimationState::default(),
    );

    // ... 动画逻辑
}
```

**验证**:
- `cargo run`
- 视觉测试: 点击任务卡片，详情面板流畅滑出，左侧内容平滑压缩

---

### Task 3.3: 实现键盘导航

**状态**: 待实现

**文件**: `crates/kanban_ui/src/kanban_board.rs`

**待办**:
1. [ ] ESC 键关闭详情面板
2. [ ] 方向键选择任务
3. [ ] Enter 键打开详情
4. [ ] Tab 键切换列

**代码**:

```rust
// 在 KanbanBoard 中添加键盘事件处理
impl KanbanBoard {
    fn handle_key_down(&mut self, event: &KeyDownEvent, cx: &mut Context<Self>) {
        match event.keystroke.key.as_str() {
            "escape" => self.close_detail_panel(cx),
            "up" => self.select_previous_task(cx),
            "down" => self.select_next_task(cx),
            "left" => self.select_previous_column(cx),
            "right" => self.select_next_column(cx),
            "enter" => {
                if self.selected_task_id.is_some() {
                    self.detail_panel_visible = true;
                    cx.notify();
                }
            }
            _ => {}
        }
    }
}

// 在 render 中绑定
.on_key_down(cx.listener(|this, event, _, cx| {
    this.handle_key_down(event, cx);
}))
```

**验证**:
- `cargo run`
- 测试所有键盘快捷键

---

## Wave 4: 集成任务

### Task 4.1: 集成到 dashboard_ui

**状态**: 待实现

**文件**: `crates/dashboard_ui/src/mission_control.rs`

**步骤**:
1. 导入 `kanban_ui::KanbanBoard`
2. 替换现有的任务列表布局
3. 连接 TaskStore 数据
4. 处理 KanbanEvent 事件

**代码**:

```rust
use kanban_ui::KanbanBoard;

pub fn render_mission_control(
    panel: &mut DashboardPanel,
    window: &mut Window,
    cx: &mut Context<DashboardPanel>,
) -> impl IntoElement {
    // 使用 KanbanBoard 替代现有布局
    let kanban = cx.new(|cx| KanbanBoard::new(cx));

    v_flex()
        .size_full()
        .child(kanban)
}
```

**验证**:
- `cargo run`
- 导航到 Dashboard -> Mission Control
- 确认 Kanban 看板正常显示

---

### Task 4.2: 重构 vulns_ui 使用 ThreePanelLayout

**状态**: 待实现

**文件**: `crates/vulns_ui/src/lib.rs`

**步骤**:
1. 导入 `ui::layouts::three_panel_horizontal`
2. 替换现有的三栏布局
3. 测试面板调整功能

**验证**:
- `cargo run`
- 导航到 Vulns 视图
- 确认三面板布局正常，可拖拽调整

---

## Wave 5: 高级功能

### Task 5.1: 实现拖拽改变任务状态

**状态**: 待实现

**文件**:
- `crates/kanban_ui/src/task_card.rs`
- `crates/kanban_ui/src/kanban_column.rs`
- `crates/kanban_ui/src/kanban_board.rs`

**步骤**:
1. 定义 DraggedTask 类型
2. 在 TaskCard 添加 on_drag
3. 在 KanbanColumn 添加 drop 处理
4. 在 KanbanBoard 处理状态更新

**代码**:

```rust
// task_card.rs
#[derive(Clone)]
pub struct DraggedTask {
    pub id: usize,
    pub from_status: TaskStatus,
}

// kanban_column.rs - 添加 drop 区域
.on_drop(DraggedTask, cx.listener(|this, dragged: &DraggedTask, _, cx| {
    // 触发任务移动事件
    cx.emit(KanbanEvent::TaskMoved {
        task_id: dragged.id,
        from: dragged.from_status,
        to: this.status,
    });
}))
```

**验证**:
- `cargo run`
- 拖拽任务到其他列
- 确认任务状态已更新

---

### Task 5.2: 添加任务搜索/过滤

**状态**: 待实现

**文件**: `crates/kanban_ui/src/kanban_board.rs`

**步骤**:
1. 添加搜索输入框
2. 实现过滤逻辑
3. 高亮匹配文本

---

## 任务依赖关系

```
Wave 1 (已完成)
├── 1.1 kanban_ui 骨架 ✅
├── 1.2 theme 常量 ✅
└── 1.3 layouts 骨架 ✅
        │
        ▼
Wave 2 (进行中)
├── 2.1 TaskCard 完善
├── 2.2 KanbanColumn 完善
└── 2.3 ThreePanelLayout 完善
        │
        ▼
Wave 3
├── 3.1 TaskDetailPanel 完善
├── 3.2 Squeeze 动画调优
└── 3.3 键盘导航
        │
        ▼
Wave 4
├── 4.1 集成 dashboard_ui
└── 4.2 重构 vulns_ui
        │
        ▼
Wave 5
├── 5.1 拖拽状态变更
└── 5.2 搜索/过滤
```

---

## 验收标准

1. [ ] `cargo check` 无错误
2. [ ] `cargo clippy -- -D warnings` 无警告
3. [ ] Kanban 5 列正常显示
4. [ ] 点击任务卡片，详情面板 squeeze 滑出
5. [ ] 动画流畅 (~200ms)
6. [ ] 左侧内容平滑压缩，无截断
7. [ ] ESC 关闭详情面板
8. [ ] 三面板布局可拖拽调整
9. [ ] 拖拽任务可改变状态 (Wave 5)

---

## 注意事项

- 所有颜色/尺寸使用 `ui::theme::*` 常量
- 使用 gpui-component 的现有组件
- 动画使用 `with_animation` + `cubic_bezier`
- 保持代码风格一致
- 每个任务完成后运行 `cargo check` 验证
