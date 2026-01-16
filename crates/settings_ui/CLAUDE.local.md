# Settings UI Agent Instructions

## Scope
This agent is responsible ONLY for `crates/settings_ui/` - the application settings panel.

## Module Structure
```
settings_ui/
├── lib.rs      # SettingsPanel - 主面板 (左侧边栏 + 右内容)
├── sidebar.rs  # 设置分类导航侧边栏
└── content.rs  # 设置内容区域 (表单项)
```

## Settings Categories
```
General     - 通用设置 (语言、启动视图、自动更新)
Appearance  - 外观设置 (主题、字体、布局)
AI          - AI 配置 (模型选择、API Key、提示词)
Security    - 安全设置 (认证、加密、审计)
Network     - 网络设置 (代理、超时、重试)
Workflow    - 工作流设置 (默认流程、调度策略)
Scanner     - 扫描器设置 (并发数、超时、范围限制)
Storage     - 存储设置 (数据库路径、缓存、日志)
Advanced    - 高级设置 (调试、实验性功能)
```

## Required Imports Pattern
```rust
use gpui::*;
use gpui_component::{
    h_flex, v_flex,
    button::Button,
    label::Label,
    input::{Input, InputState},
    checkbox::Checkbox,
    switch::Switch,
    dropdown::Dropdown,
};
use ui::theme::*;
```

## Component Patterns

### Toggle/Switch Setting
```rust
fn render_toggle_setting(
    label: &str,
    description: &str,
    enabled: bool,
    on_change: impl Fn(bool) + 'static,
) -> impl IntoElement {
    h_flex()
        .w_full()
        .justify_between()
        .items_center()
        .p(PADDING_MD)
        .bg(rgb(BG_CARD))
        .rounded(BORDER_RADIUS)
        .child(
            v_flex()
                .gap(SPACING_XS)
                .child(Label::new(label).text_color(rgb(TEXT_PRIMARY)))
                .child(Label::new(description).text_sm().text_color(rgb(TEXT_SECONDARY)))
        )
        .child(Switch::new(enabled).on_change(on_change))
}
```

### Dropdown Setting
```rust
fn render_dropdown_setting(
    label: &str,
    options: &[&str],
    selected: usize,
) -> impl IntoElement {
    h_flex()
        .w_full()
        .justify_between()
        .items_center()
        .p(PADDING_MD)
        .bg(rgb(BG_CARD))
        .rounded(BORDER_RADIUS)
        .child(Label::new(label))
        .child(
            Dropdown::new(options)
                .selected(selected)
                .w(px(200.0))
        )
}
```

### Category Sidebar Item
```rust
fn render_category(name: &str, selected: bool) -> impl IntoElement {
    let bg = if selected {
        rgb(ACCENT_PURPLE).opacity(0.1)
    } else {
        rgb(BG_CARD)
    };

    h_flex()
        .w_full()
        .p(PADDING_SM)
        .rounded(BORDER_RADIUS_SM)
        .bg(bg)
        .when(selected, |el| el.border_l_2().border_color(rgb(ACCENT_PURPLE)))
        .child(Label::new(name))
}
```

## Theme Constants to Use
- Sidebar: `BG_PRIMARY`, `BORDER_COLOR`, width `px(280.0)`
- Category selected: `ACCENT_PURPLE` with opacity
- Content area: `BG_PRIMARY`, `PADDING_LG` (px(16.0))
- Setting items: `BG_CARD`, `BORDER_RADIUS`, `PADDING_MD`
- Text: `TEXT_PRIMARY`, `TEXT_SECONDARY`

## Current Code Issues
```rust
// sidebar.rs:50 - 使用 with_opacity 方法可能不存在
this.bg(ui::theme::ACCENT_PURPLE.with_opacity(0.1))  // ⚠️ 检查 GPUI API

// content.rs:43-49 - when 回调返回类型不匹配
.when(is_toggle, |_| {  // ❌ 应该返回 Self，不是新 v_flex
    v_flex()...
})
```

## State Management
```rust
pub struct SettingsPanel {
    selected_category: String,           // 当前选中分类
    settings: HashMap<String, Value>,    // 设置值存储
    search_query: String,                // 搜索过滤
}

// 分类切换
fn select_category(&mut self, category: &str, cx: &mut Context<Self>) {
    self.selected_category = category.to_string();
    cx.notify();
}

// 设置变更
fn update_setting(&mut self, key: &str, value: Value, cx: &mut Context<Self>) {
    self.settings.insert(key.to_string(), value);
    cx.emit(SettingsChanged { key, value });
    cx.notify();
}
```

## DO NOT
- 修改共享文件
- 实现实际的配置文件读写 (属于 `data` crate)
- 添加敏感信息明文存储

## Current TODOs
- [ ] 实现分类切换交互 (sidebar -> content)
- [ ] 添加真实的 Switch/Toggle 组件
- [ ] 实现 Dropdown 选择器
- [ ] 添加设置搜索过滤功能
- [ ] 实现设置值双向绑定
- [ ] 添加"恢复默认"按钮
- [ ] 实现设置变更保存
- [ ] 添加设置验证提示
- [ ] 实现 settings.json 编辑器打开
