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
- [x] 完善统计卡片数据绑定 (Enhance stat card data binding)
  - Created `stat_card.rs` with render_stat_card() function
  - Displays counts for: To Do, In Progress, In Review, Done, Critical vulns
  - Shows dynamic data from DashboardPanel task lists
  - Color-coded with severity themes
- [x] 实现任务进度环形图 (Implement task progress ring chart)
  - Created `progress_ring.rs` with visual progress indicator
  - Shows percentage completion and remaining task count
  - Uses green progress bar visualization
- [x] 添加最近发现列表 (Add recent findings list)
  - Created `recent_findings.rs` with RecentFinding struct
  - Displays up to 5 recent vulnerability findings
  - Shows severity badges, asset, and timestamp
- [x] 实现快速操作按钮 (Implement quick action buttons)
  - Created `quick_actions.rs` with render_quick_actions() function
  - Buttons: New Task, Run Scan, Export Report
  - Connected to DashboardPanel action handlers (on_new_task, on_run_scan, on_export)
- [x] 清理 dashboard_ui/theme.rs (迁移到 ui::theme)
  - Removed duplicate theme constants from dashboard_ui
  - All new components use `use ui::theme::*`
  - Consolidated to single source of truth in ui crate
