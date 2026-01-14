# Dashboard UI Agent Instructions

## Scope
This agent is responsible ONLY for `crates/dashboard_ui/` - the main dashboard panel.

## Module Structure
```
dashboard_ui/
├── lib.rs              # DashboardPanel - 主面板
├── dashboard_panel.rs  # 面板布局实现
├── components.rs       # 统计卡片、图表组件
├── findings.rs         # 发现列表组件
├── mission_control.rs  # 任务控制面板
└── theme.rs            # (已废弃，使用 ui::theme)
```

## Key Data Models
- `TaskData` - 任务状态 (from `data::models`)
- `VulnData` - 漏洞概览 (from `data::models`)
- `DashboardStats` - 统计数据 (可能需要定义)

## Required Imports Pattern
```rust
use gpui::*;
use gpui_component::{h_flex, v_flex, button::Button, label::Label};
use data::models::{TaskData, VulnData, TaskStatus};
use ui::theme::*;
```

## Component Patterns

### Stat Card
```rust
fn render_stat_card(title: &str, value: &str, icon: impl IntoElement) -> impl IntoElement {
    v_flex()
        .p(PADDING_MD)
        .bg(rgb(BG_CARD))
        .rounded(BORDER_RADIUS)
        .child(Label::new(title).text_color(rgb(TEXT_SECONDARY)))
        .child(Label::new(value).text_size(TEXT_SIZE_XL))
}
```

### Recent Findings List
```rust
fn render_findings(&self, findings: &[VulnData]) -> impl IntoElement {
    v_flex()
        .gap(SPACING_SM)
        .children(findings.iter().take(5).map(|f| self.render_finding_row(f)))
}
```

## Theme Constants to Use
- Card: `BG_CARD`, `BORDER_RADIUS`, `PADDING_MD`
- Stats: `TEXT_SIZE_XL`, `ACCENT_PURPLE`, `ACCENT_BLUE`
- Severity badges: `SEVERITY_*` colors
- Status: `STATUS_SUCCESS`, `STATUS_WARNING`

## DO NOT
- 修改共享文件
- 实现复杂的数据聚合 (应在 data layer)
- 重复定义 theme 常量 (使用 `ui::theme::*`)

## Current TODOs
- [x] 将 Mission Control Kanban 从三列改为五列 (Todo, InProgress, InReview, Done, Cancelled)
  - Updated `DashboardPanel` to track 5 task lists (in_review_tasks, canceled_tasks)
  - Updated `mission_control.rs` to render 5 columns
  - Column index adjusted from 0,1,2 to 0,1,2,3,4
- [x] 完善 Kanban 列标题 - 根据 KANBAN_UI_TASKS.md
  - Enhanced `render_kanban_column_header` with status-specific color indicators
  - To Do: Dark Grey, In Progress: Blue, In Review: Orange, Done: Green, Cancelled: Red
  - Added border bottom for visual separation
- [x] 优化任务卡片 - 根据 KANBAN_UI_TASKS.md
  - Improved `render_task_card` with better card styling
  - Added action button (Ellipsis menu) to card header
  - Updated to use proper white background with shadow and border
  - Proper padding and spacing for content
  - Support for tags and action buttons
- [x] 修复 MissionControl 布局
  - Removed incorrectly placed stat_card, progress_ring, recent_findings, quick_actions
  - MissionControl now displays pure Kanban board as per design
  - Restored proper separation: MissionControl = Kanban view, Findings = Vulnerabilities view
  - Cleaned up dashboard_panel.rs render method
