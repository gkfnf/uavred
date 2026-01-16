# UI 开发任务 - 第一轮：TopNavBar 完善

## 任务目标

将现有的 TopNavBar (TitleBar) 完善为与设计图一致的顶部导航栏。

## 参考设计图

- `interface_pic/DashBoard_A.png`
- `interface_pic/Assets.png`
- `interface_pic/Vulns.png`

## 当前状态分析

**已有实现** (`crates/uavred/src/workspace.rs`):
- 使用 gpui_component 的 `TitleBar` 组件
- 基本的导航项 (`nav_item` 方法)
- Badge 数字徽章 (Vulns: 2, Traffic: 8)
- AI Active 状态指示器

**缺失/需改进**:
1. 导航项缺少图标
2. 导航项样式不完全匹配设计图
3. 缺少时间显示
4. Settings 应该在右侧区域
5. AI Active 状态需要调整样式

---

## 子任务拆分

### 任务 1.1: 补充主题常量

**文件**: `crates/ui/src/theme.rs`

**操作**: 在文件末尾添加以下常量

```rust
// === 新增常量 ===

// 导航栏相关
pub const NAV_BG: u32 = 0xffffff;
pub const NAV_ITEM_HOVER: u32 = 0xf3f4f6;
pub const NAV_ITEM_ACTIVE_BG: u32 = 0xf3e8ff;  // 浅紫色背景
pub const NAV_ITEM_ACTIVE_BORDER: u32 = 0x7c3aed;  // 紫色边框

// Badge 颜色
pub const BADGE_BG_PURPLE: u32 = 0x7c3aed;
pub const BADGE_BG_RED: u32 = 0xef4444;
pub const BADGE_TEXT: u32 = 0xffffff;

// 导航栏尺寸
pub const NAV_HEIGHT: Pixels = px(48.0);
pub const NAV_ITEM_HEIGHT: Pixels = px(32.0);
pub const NAV_ICON_SIZE: Pixels = px(16.0);
pub const BADGE_SIZE: Pixels = px(18.0);
```

**验证**: `cargo check --package ui`

---

### 任务 1.2: 创建 NavIcon 枚举

**文件**: `crates/ui/src/icons.rs` (新建)

**操作**: 创建新文件，定义导航图标

```rust
//! 导航图标定义

/// 导航项图标类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavIcon {
    Dashboard,
    Assets,
    Scan,
    Vulns,
    Traffic,
    Flows,
    Devices,
    Monitor,
    Settings,
}

impl NavIcon {
    /// 返回图标的 Unicode 字符 (临时方案，后续可替换为 SVG)
    pub fn as_char(&self) -> char {
        match self {
            NavIcon::Dashboard => '\u{1F4CA}',  // 📊
            NavIcon::Assets => '\u{1F5A5}',     // 🖥
            NavIcon::Scan => '\u{1F50D}',       // 🔍
            NavIcon::Vulns => '\u{26A0}',       // ⚠
            NavIcon::Traffic => '\u{1F4E1}',    // 📡
            NavIcon::Flows => '\u{1F504}',      // 🔄
            NavIcon::Devices => '\u{1F4F1}',    // 📱
            NavIcon::Monitor => '\u{1F5B5}',    // 🖵
            NavIcon::Settings => '\u{2699}',    // ⚙
        }
    }

    /// 返回图标名称 (用于 gpui_component Icon)
    pub fn icon_name(&self) -> &'static str {
        match self {
            NavIcon::Dashboard => "layout-dashboard",
            NavIcon::Assets => "server",
            NavIcon::Scan => "scan",
            NavIcon::Vulns => "bug",
            NavIcon::Traffic => "network",
            NavIcon::Flows => "workflow",
            NavIcon::Devices => "smartphone",
            NavIcon::Monitor => "activity",
            NavIcon::Settings => "settings",
        }
    }
}
```

**操作**: 更新 `crates/ui/src/lib.rs`，添加模块导出

```rust
// 在 lib.rs 中添加
pub mod icons;
pub use icons::NavIcon;
```

**验证**: `cargo check --package ui`

---

### 任务 1.3: 改进 nav_item 方法

**文件**: `crates/uavred/src/workspace.rs`

**操作**: 修改 `nav_item` 方法，使用 gpui_component 的 Icon

**修改前位置**: 第 162-185 行

**修改后代码**:

```rust
fn nav_item(
    &mut self,
    cx: &mut Context<Self>,
    view: AppView,
    label: &str,
    icon_name: &'static str,
) -> impl IntoElement {
    use gpui_component::icon::Icon;
    use ui::theme::*;

    let is_active = self.active_view == view;
    let badge_count = match view {
        AppView::Vulns => Some(2),
        AppView::Traffic => Some(4),
        _ => None,
    };

    let label_text = label.to_string();

    // 构建按钮内容
    let content = h_flex()
        .items_center()
        .gap(px(6.0))
        .child(Icon::new(icon_name).size(NAV_ICON_SIZE))
        .child(Label::new(label_text.clone()).text_sm());

    // 添加 badge
    let content = if let Some(count) = badge_count {
        content.child(
            Badge::new()
                .count(count)
                .color(rgb(BADGE_BG_PURPLE))
                .small()
        )
    } else {
        content
    };

    // 基础样式
    let mut item = div()
        .id(format!("nav-{:?}", view))
        .px(PADDING_MD)
        .py(PADDING_SM)
        .rounded(BORDER_RADIUS)
        .cursor_pointer()
        .child(content);

    // 激活/悬停状态
    if is_active {
        item = item
            .bg(rgb(NAV_ITEM_ACTIVE_BG))
            .border_1()
            .border_color(rgb(NAV_ITEM_ACTIVE_BORDER));
    } else {
        item = item
            .hover(|style| style.bg(rgb(NAV_ITEM_HOVER)));
    }

    // 点击事件
    item.on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _, cx| {
        this.set_active_view(view, cx);
    }))
}
```

**验证**: `cargo check --package uavred`

---

### 任务 1.4: 更新 render_title_bar 方法

**文件**: `crates/uavred/src/workspace.rs`

**操作**: 修改 `render_title_bar` 方法以匹配设计图布局

**修改后代码**:

```rust
fn render_title_bar(
    &mut self,
    _window: &mut Window,
    cx: &mut Context<Self>,
) -> impl IntoElement {
    use ui::theme::*;

    // 左侧：主导航项
    let main_nav = h_flex()
        .items_center()
        .gap(px(4.0))
        .child(self.nav_item(cx, AppView::Dashboard, "Dashboard", "layout-dashboard"))
        .child(self.nav_item(cx, AppView::Assets, "Assets", "server"))
        .child(self.nav_item(cx, AppView::Vulns, "Vulns", "bug"))
        .child(self.nav_item(cx, AppView::Traffic, "Traffic", "network"))
        .child(self.nav_item(cx, AppView::Flows, "Flows", "workflow"));

    // 右侧：Settings + AI 状态 + 时间
    let right_section = h_flex()
        .items_center()
        .gap(px(16.0))
        .child(self.nav_item(cx, AppView::Settings, "Settings", "settings"))
        .child(self.render_ai_status())
        .child(self.render_clock());

    TitleBar::new()
        .child(
            h_flex()
                .flex_1()
                .items_center()
                .justify_between()
                .child(main_nav)
                .child(right_section)
        )
}

fn render_ai_status(&self) -> impl IntoElement {
    use ui::theme::*;

    h_flex()
        .items_center()
        .gap(px(6.0))
        .child(
            div()
                .w(px(8.0))
                .h(px(8.0))
                .rounded_full()
                .bg(rgb(STATUS_SUCCESS))
        )
        .child(
            Label::new("AI Active")
                .text_sm()
                .text_color(rgb(STATUS_SUCCESS))
        )
}

fn render_clock(&self) -> impl IntoElement {
    use ui::theme::*;

    // 静态时间显示 (后续可改为动态)
    Label::new("3:07:57 AM")
        .text_sm()
        .text_color(rgb(TEXT_SECONDARY))
}
```

**验证**: `cargo check --package uavred`

---

### 任务 1.5: 编译测试与修复

**操作顺序**:

1. 运行 `cargo check` 检查编译
2. 修复任何编译错误
3. 运行 `cargo clippy -- -D warnings` 检查代码质量
4. 运行 `cargo run` 验证视觉效果

**预期结果**:
- 编译无错误
- TopNavBar 显示带图标的导航项
- 激活项有紫色边框和浅紫色背景
- Vulns 和 Traffic 显示数字徽章
- 右侧显示 Settings、AI Active 状态和时间

---

## 验收标准

1. [ ] `cargo check` 通过
2. [ ] `cargo clippy -- -D warnings` 无警告
3. [ ] 导航项显示图标 + 文字
4. [ ] 激活状态有视觉区分
5. [ ] Badge 显示在 Vulns (2) 和 Traffic (4)
6. [ ] Settings 在右侧
7. [ ] AI Active 状态指示器可见
8. [ ] 时间显示在最右侧

---

## 注意事项

- 使用 gpui_component 已有组件，不要重复造轮子
- 所有颜色/尺寸使用 theme.rs 常量
- 保持代码风格与现有代码一致
- 如遇到 Icon 不显示问题，可暂时回退到纯文字

## 后续任务预告

- 第二轮: Dashboard 看板布局 (KanbanLayout + TaskCard)
- 第三轮: Vulns 三面板布局 (ThreePanelLayout)
