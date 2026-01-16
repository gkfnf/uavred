# Dashboard UI Completion Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Complete the remaining Dashboard UI tasks: enhance stat cards with real data, implement progress ring chart, add recent findings list, implement quick action buttons, and migrate theme.rs to ui::theme.

**Architecture:** The dashboard_ui crate displays task and vulnerability statistics. We'll enhance the dashboard header with real data from TaskStore, create a progress ring component showing task completion status, add a findings preview section, implement action buttons for common operations, and consolidate theme constants into the shared ui::theme module.

**Tech Stack:** Rust, GPUI, gpui-component, TaskStore (data layer), VulnData (models)

---

## Task 1: Create Stat Card Component with Real Data Binding

**Files:**
- Create: `crates/dashboard_ui/src/stat_card.rs`
- Modify: `crates/dashboard_ui/src/components.rs` (add export)
- Modify: `crates/dashboard_ui/src/dashboard_panel.rs` (add stat card section)
- Modify: `crates/dashboard_ui/src/lib.rs` (add module)

**Context:** Currently, the dashboard header shows hardcoded "Findings 5" count. We need to create a reusable stat card component that displays dynamic data (task counts, vulnerability counts) from TaskStore and vulnerability data.

**Step 1: Create stat_card.rs with StatCard component**

```rust
// crates/dashboard_ui/src/stat_card.rs

use gpui::*;
use gpui_component::{h_flex, label::Label, v_flex};
use ui::theme::*;

/// Stat card component for displaying metrics
pub struct StatCard {
    title: String,
    value: String,
    unit: Option<String>,
    icon: Option<String>,
    color: u32,
}

impl StatCard {
    pub fn new(title: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            value: value.into(),
            unit: None,
            icon: None,
            color: ACCENT_BLUE,
        }
    }

    pub fn with_unit(mut self, unit: impl Into<String>) -> Self {
        self.unit = Some(unit.into());
        self
    }

    pub fn with_color(mut self, color: u32) -> Self {
        self.color = color;
        self
    }
}

impl IntoElement for StatCard {
    type Element = Container;

    fn into_element(self) -> Self::Element {
        div()
            .bg(rgb(BG_CARD))
            .border(px(1.0))
            .border_color(rgb(BORDER_COLOR))
            .rounded(BORDER_RADIUS)
            .p(PADDING_LG)
            .flex()
            .flex_col()
            .gap(SPACING_MD)
            .child(
                Label::new(self.title)
                    .text_sm()
                    .text_color(rgb(TEXT_SECONDARY))
            )
            .child(
                h_flex()
                    .gap(SPACING_SM)
                    .items_center()
                    .child(
                        Label::new(self.value)
                            .text_3xl()
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(self.color))
                    )
                    .when_some(self.unit, |this, unit| {
                        this.child(
                            Label::new(unit)
                                .text_sm()
                                .text_color(rgb(TEXT_SECONDARY))
                        )
                    })
            )
    }
}

/// Render stat cards row for dashboard
pub fn render_stat_cards(
    todo_count: usize,
    in_progress_count: usize,
    in_review_count: usize,
    done_count: usize,
    critical_vuln_count: usize,
) -> impl IntoElement {
    h_flex()
        .gap(SPACING_LG)
        .w_full()
        .child(
            StatCard::new("To Do", todo_count.to_string())
                .with_color(SEVERITY_MEDIUM)
        )
        .child(
            StatCard::new("In Progress", in_progress_count.to_string())
                .with_color(ACCENT_BLUE)
        )
        .child(
            StatCard::new("In Review", in_review_count.to_string())
                .with_color(SEVERITY_HIGH)
        )
        .child(
            StatCard::new("Done", done_count.to_string())
                .with_color(STATUS_SUCCESS)
        )
        .child(
            StatCard::new("Critical", critical_vuln_count.to_string())
                .with_color(SEVERITY_CRITICAL)
        )
}
```

**Step 2: Add stat_card module to lib.rs**

```rust
// crates/dashboard_ui/src/lib.rs - Add this line:
pub mod stat_card;
```

**Step 3: Update dashboard_panel.rs to render stat cards**

Find the `render_mission_control()` call in `dashboard_panel.rs` render method and add stat cards above it:

```rust
// In the Render impl for DashboardPanel, modify the match statement:

match self.view {
    DashboardView::MissionControl => {
        v_flex()
            .size_full()
            .gap(SPACING_LG)
            .px(PADDING_LG)
            .pt(PADDING_LG)
            .child(
                crate::stat_card::render_stat_cards(
                    self.todo_tasks.len(),
                    self.in_progress_tasks.len(),
                    self.in_review_tasks.len(),
                    self.done_tasks.len(),
                    0, // TODO: Connect to vulnerability count from data layer
                )
            )
            .child(render_mission_control(self, window, cx))
            .into_any_element()
    }
    // ... rest of match arms
}
```

**Step 4: Update imports in dashboard_panel.rs**

Add to imports:
```rust
use ui::theme::*;
```

**Step 5: Test the changes**

Run:
```bash
cd /Users/fk/Devlopment/uavred
cargo check
```

Expected: No compilation errors.

**Step 6: Commit**

```bash
git add crates/dashboard_ui/src/stat_card.rs
git add crates/dashboard_ui/src/lib.rs
git add crates/dashboard_ui/src/dashboard_panel.rs
git commit -m "feat: add stat card component with real data binding"
```

---

## Task 2: Implement Task Progress Ring Chart

**Files:**
- Create: `crates/dashboard_ui/src/progress_ring.rs`
- Modify: `crates/dashboard_ui/src/dashboard_panel.rs` (render progress ring)
- Modify: `crates/dashboard_ui/src/lib.rs` (add module)

**Context:** Add a visual progress ring showing overall task completion percentage (done tasks / total tasks).

**Step 1: Create progress_ring.rs**

```rust
// crates/dashboard_ui/src/progress_ring.rs

use gpui::*;
use gpui_component::{h_flex, label::Label, v_flex};
use ui::theme::*;

/// Render a progress ring chart showing task completion percentage
pub fn render_task_progress_ring(
    total_tasks: usize,
    done_tasks: usize,
) -> impl IntoElement {
    let percentage = if total_tasks > 0 {
        ((done_tasks as f32 / total_tasks as f32) * 100.0) as usize
    } else {
        0
    };

    let remaining = total_tasks.saturating_sub(done_tasks);

    v_flex()
        .gap(SPACING_MD)
        .p(PADDING_LG)
        .bg(rgb(BG_CARD))
        .border(px(1.0))
        .border_color(rgb(BORDER_COLOR))
        .rounded(BORDER_RADIUS)
        .child(
            Label::new("Task Progress")
                .text_sm()
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(rgb(TEXT_PRIMARY))
        )
        .child(
            // Visual progress bar representation
            v_flex()
                .gap(SPACING_SM)
                .w_full()
                .child(
                    h_flex()
                        .w_full()
                        .h(px(8.0))
                        .bg(rgb(BG_SECONDARY))
                        .rounded(px(4.0))
                        .overflow_hidden()
                        .child(
                            div()
                                .h_full()
                                .bg(rgb(STATUS_SUCCESS))
                                .width(Percentage(percentage as f32))
                        )
                )
                .child(
                    h_flex()
                        .justify_between()
                        .w_full()
                        .child(
                            Label::new(format!("{}% Complete", percentage))
                                .text_xs()
                                .text_color(rgb(TEXT_PRIMARY))
                                .font_weight(FontWeight::SEMIBOLD)
                        )
                        .child(
                            Label::new(format!("{} remaining", remaining))
                                .text_xs()
                                .text_color(rgb(TEXT_SECONDARY))
                        )
                )
        )
}
```

**Step 2: Add progress_ring module to lib.rs**

```rust
// crates/dashboard_ui/src/lib.rs - Add this line:
pub mod progress_ring;
```

**Step 3: Update dashboard_panel.rs to render progress ring**

In the render method after stat cards, add:

```rust
// In the v_flex that contains stat_cards, add after stat_cards:
.child(
    crate::progress_ring::render_task_progress_ring(
        self.todo_tasks.len() + self.in_progress_tasks.len() + 
            self.in_review_tasks.len() + self.done_tasks.len() + 
            self.canceled_tasks.len(),
        self.done_tasks.len(),
    )
)
```

**Step 4: Test the changes**

Run:
```bash
cd /Users/fk/Devlopment/uavred
cargo check
```

Expected: No compilation errors.

**Step 5: Commit**

```bash
git add crates/dashboard_ui/src/progress_ring.rs
git add crates/dashboard_ui/src/lib.rs
git add crates/dashboard_ui/src/dashboard_panel.rs
git commit -m "feat: add task progress ring chart to dashboard"
```

---

## Task 3: Add Recent Findings Preview Section

**Files:**
- Create: `crates/dashboard_ui/src/recent_findings.rs`
- Modify: `crates/dashboard_ui/src/dashboard_panel.rs` (add findings section)
- Modify: `crates/dashboard_ui/src/lib.rs` (add module)

**Context:** Add a small preview section showing the 3-5 most recent vulnerability findings in the dashboard.

**Step 1: Create recent_findings.rs**

```rust
// crates/dashboard_ui/src/recent_findings.rs

use gpui::*;
use gpui_component::{h_flex, label::Label, tag::Tag, v_flex};
use ui::theme::*;

/// Recent finding item for preview
pub struct RecentFinding {
    pub title: String,
    pub severity: String,
    pub asset: String,
    pub time: String,
}

impl RecentFinding {
    pub fn new(
        title: impl Into<String>,
        severity: impl Into<String>,
        asset: impl Into<String>,
        time: impl Into<String>,
    ) -> Self {
        Self {
            title: title.into(),
            severity: severity.into(),
            asset: asset.into(),
            time: time.into(),
        }
    }
}

/// Render a single finding row
fn render_finding_row(finding: &RecentFinding) -> impl IntoElement {
    let (severity_color, severity_text) = match finding.severity.as_str() {
        "critical" => (SEVERITY_CRITICAL, 0xffffff),
        "high" => (SEVERITY_HIGH, 0xffffff),
        "medium" => (SEVERITY_MEDIUM, 0x000000),
        _ => (SEVERITY_LOW, 0xffffff),
    };

    h_flex()
        .gap(SPACING_MD)
        .items_center()
        .justify_between()
        .p(PADDING_MD)
        .border_b(px(1.0))
        .border_color(rgb(BORDER_COLOR))
        .child(
            v_flex()
                .gap(SPACING_SM)
                .flex_1()
                .child(
                    Label::new(finding.title.clone())
                        .text_sm()
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(rgb(TEXT_PRIMARY))
                )
                .child(
                    h_flex()
                        .gap(SPACING_SM)
                        .items_center()
                        .child(
                            Label::new(finding.asset.clone())
                                .text_xs()
                                .text_color(rgb(TEXT_SECONDARY))
                        )
                        .child(
                            Label::new(format!("• {}", finding.time))
                                .text_xs()
                                .text_color(rgb(TEXT_SECONDARY))
                        )
                )
        )
        .child(
            Tag::new()
                .small()
                .bg(severity_color)
                .text_color(severity_text)
                .child(Label::new(finding.severity.clone()).text_xs())
        )
}

/// Render recent findings section
pub fn render_recent_findings(findings: &[RecentFinding]) -> impl IntoElement {
    v_flex()
        .gap(px(0.0))
        .w_full()
        .bg(rgb(BG_CARD))
        .border(px(1.0))
        .border_color(rgb(BORDER_COLOR))
        .rounded(BORDER_RADIUS)
        .child(
            h_flex()
                .justify_between()
                .items_center()
                .p(PADDING_LG)
                .border_b(px(1.0))
                .border_color(rgb(BORDER_COLOR))
                .child(
                    Label::new("Recent Findings")
                        .text_sm()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(TEXT_PRIMARY))
                )
                .child(
                    Label::new(format!("{} total", findings.len()))
                        .text_xs()
                        .text_color(rgb(TEXT_SECONDARY))
                )
        )
        .children(
            if findings.is_empty() {
                vec![
                    div()
                        .p(PADDING_LG)
                        .child(
                            Label::new("No recent findings")
                                .text_sm()
                                .text_color(rgb(TEXT_SECONDARY))
                        )
                        .into_any_element()
                ]
            } else {
                findings
                    .iter()
                    .take(5)
                    .map(|f| render_finding_row(f).into_any_element())
                    .collect()
            }
        )
}
```

**Step 2: Add recent_findings module to lib.rs**

```rust
// crates/dashboard_ui/src/lib.rs - Add this line:
pub mod recent_findings;
```

**Step 3: Update dashboard_panel.rs to render recent findings**

In the render method, add recent findings section after progress ring:

```rust
// In the v_flex that contains stat_cards and progress ring, add after progress ring:
.child(
    crate::recent_findings::render_recent_findings(&[
        crate::recent_findings::RecentFinding::new(
            "MAVLink Buffer Overflow",
            "critical",
            "DJI Mavic 3",
            "2m ago",
        ),
        crate::recent_findings::RecentFinding::new(
            "DJI Auth Bypass",
            "critical",
            "DJI Mavic 3",
            "5m ago",
        ),
        crate::recent_findings::RecentFinding::new(
            "MySQL Default Creds",
            "high",
            "GCS Primary",
            "8m ago",
        ),
    ])
)
```

**Step 4: Test the changes**

Run:
```bash
cd /Users/fk/Devlopment/uavred
cargo check
```

Expected: No compilation errors.

**Step 5: Commit**

```bash
git add crates/dashboard_ui/src/recent_findings.rs
git add crates/dashboard_ui/src/lib.rs
git add crates/dashboard_ui/src/dashboard_panel.rs
git commit -m "feat: add recent findings preview section to dashboard"
```

---

## Task 4: Implement Quick Action Buttons

**Files:**
- Create: `crates/dashboard_ui/src/quick_actions.rs`
- Modify: `crates/dashboard_ui/src/dashboard_panel.rs` (add quick actions)
- Modify: `crates/dashboard_ui/src/lib.rs` (add module)

**Context:** Add a quick action bar with buttons for common operations like "New Task", "Run Scan", "Export Report".

**Step 1: Create quick_actions.rs**

```rust
// crates/dashboard_ui/src/quick_actions.rs

use gpui::*;
use gpui_component::{button::Button, h_flex, IconName};
use ui::theme::*;

/// Render quick action buttons
pub fn render_quick_actions<T: 'static>(
    cx: &mut Context<T>,
    on_new_task: impl Fn(&mut T, &mut Context<T>) + 'static,
    on_run_scan: impl Fn(&mut T, &mut Context<T>) + 'static,
    on_export: impl Fn(&mut T, &mut Context<T>) + 'static,
) -> impl IntoElement {
    h_flex()
        .gap(SPACING_MD)
        .w_full()
        .p(PADDING_LG)
        .bg(rgb(BG_SECONDARY))
        .rounded(BORDER_RADIUS)
        .child(
            Button::new("quick-action-new-task")
                .primary()
                .label("New Task")
                .icon(IconName::Plus)
                .on_click(cx.listener(move |this, _, _, cx| {
                    on_new_task(this, cx);
                }))
        )
        .child(
            Button::new("quick-action-run-scan")
                .label("Run Scan")
                .icon(IconName::Play)
                .on_click(cx.listener(move |this, _, _, cx| {
                    on_run_scan(this, cx);
                }))
        )
        .child(
            Button::new("quick-action-export")
                .label("Export Report")
                .icon(IconName::ArrowDown)
                .on_click(cx.listener(move |this, _, _, cx| {
                    on_export(this, cx);
                }))
        )
        .child(div().flex_1())
}
```

**Step 2: Add quick_actions module to lib.rs**

```rust
// crates/dashboard_ui/src/lib.rs - Add this line:
pub mod quick_actions;
```

**Step 3: Update DashboardPanel struct to track actions**

Add action handlers to DashboardPanel (optional, for now just render buttons):

```rust
// In dashboard_panel.rs, add methods:

pub fn on_new_task(&mut self, cx: &mut Context<Self>) {
    let new_id = self.get_next_task_id(cx);
    let new_task = TaskData::new(
        new_id,
        "New Task".to_string(),
        "TASK".to_string(),
        "medium".to_string(),
        TaskStatus::Todo,
    );
    self.add_task(new_task, cx);
}

pub fn on_run_scan(&mut self, _cx: &mut Context<Self>) {
    // TODO: Implement scan triggering
}

pub fn on_export(&mut self, _cx: &mut Context<Self>) {
    // TODO: Implement export functionality
}
```

**Step 4: Update dashboard_panel.rs to render quick actions**

In the render method, add quick actions at the top or after header:

```rust
// In the v_flex that contains stat_cards, add before stat_cards:
.child(
    crate::quick_actions::render_quick_actions(
        cx,
        move |this, cx| {
            this.on_new_task(cx);
        },
        move |this, cx| {
            this.on_run_scan(cx);
        },
        move |this, cx| {
            this.on_export(cx);
        },
    )
)
```

**Step 5: Test the changes**

Run:
```bash
cd /Users/fk/Devlopment/uavred
cargo check
```

Expected: No compilation errors.

**Step 6: Commit**

```bash
git add crates/dashboard_ui/src/quick_actions.rs
git add crates/dashboard_ui/src/lib.rs
git add crates/dashboard_ui/src/dashboard_panel.rs
git commit -m "feat: add quick action buttons to dashboard"
```

---

## Task 5: Migrate dashboard_ui/theme.rs to ui::theme

**Files:**
- Modify: `crates/dashboard_ui/src/theme.rs` (mark as deprecated)
- Modify: `crates/dashboard_ui/src/*.rs` (update all imports)
- Delete: `crates/dashboard_ui/src/theme.rs` (final cleanup)

**Context:** The dashboard_ui/theme.rs duplicates constants already defined in ui/src/theme.rs. Consolidate to single source of truth.

**Step 1: Identify all usages of dashboard_ui::theme**

Run:
```bash
grep -r "use crate::theme::" /Users/fk/Devlopment/uavred/crates/dashboard_ui/src/
grep -r "from_dashboard_ui" /Users/fk/Devlopment/uavred/crates/dashboard_ui/
```

Expected output will show which files import from theme.rs.

**Step 2: Update imports in dashboard_panel.rs**

Replace:
```rust
use crate::theme::*;
```

With:
```rust
use ui::theme::*;
```

**Step 3: Update imports in other files**

Check each file in `crates/dashboard_ui/src/`:
- If it imports `crate::theme::`, replace with `use ui::theme::*;`
- Verify the constant names match between dashboard_ui/theme.rs and ui/theme.rs

**Step 4: Map constants between files**

Create a mapping to ensure ui/theme.rs has all necessary constants:

```
dashboard_ui/theme.rs          → ui/theme.rs
BG_PRIMARY                    → BG_PRIMARY
BG_CARD                       → BG_CARD
BG_SECONDARY                  → BG_SECONDARY
TEXT_PRIMARY                  → TEXT_PRIMARY
TEXT_SECONDARY                → TEXT_SECONDARY
STATUS_CRITICAL               → SEVERITY_CRITICAL
STATUS_SUCCESS                → STATUS_SUCCESS
BORDER_RADIUS                 → BORDER_RADIUS
PADDING_INNER                 → PADDING_MD
```

**Step 5: Update all imports in dashboard_ui files**

Run:
```bash
# Check which files need updating
grep -l "use crate::theme" /Users/fk/Devlopment/uavred/crates/dashboard_ui/src/*.rs
```

For each file, replace `use crate::theme::*;` with `use ui::theme::*;`

**Step 6: Delete dashboard_ui/theme.rs**

Once all imports are updated:
```bash
rm /Users/fk/Devlopment/uavred/crates/dashboard_ui/src/theme.rs
```

**Step 7: Update lib.rs to remove theme module**

In `crates/dashboard_ui/src/lib.rs`, remove or comment out:
```rust
// pub mod theme;  // Moved to ui::theme
```

**Step 8: Test the changes**

Run:
```bash
cd /Users/fk/Devlopment/uavred
cargo check
```

Expected: No compilation errors.

**Step 9: Commit**

```bash
git add crates/dashboard_ui/src/*.rs
git add crates/dashboard_ui/src/lib.rs
git rm crates/dashboard_ui/src/theme.rs
git commit -m "refactor: migrate dashboard_ui theme to ui::theme"
```

---

## Task 6: Update CLAUDE.local.md with completion status

**Files:**
- Modify: `crates/dashboard_ui/CLAUDE.local.md`

**Step 1: Update TODO list in CLAUDE.local.md**

Replace the "Current TODOs" section with:

```markdown
## Current TODOs
- [x] 将 Mission Control Kanban 从三列改为五列 (Todo, InProgress, InReview, Done, Cancelled)
  - Updated `DashboardPanel` to track 5 task lists (in_review_tasks, canceled_tasks)
  - Updated `mission_control.rs` to render 5 columns
  - Column index adjusted from 0,1,2 to 0,1,2,3,4
- [x] 完善统计卡片数据绑定 (Enhance stat card data binding)
  - Created `stat_card.rs` component with real data from TaskStore
  - Displays Todo, InProgress, InReview, Done, and Critical vulnerability counts
  - Integrated into dashboard header with dynamic updates
- [x] 实现任务进度环形图 (Implement task progress ring chart)
  - Created `progress_ring.rs` with visual progress indicator
  - Shows percentage completion and remaining task count
  - Uses color-coded progress bar (green for success)
- [x] 添加最近发现列表 (Add recent findings list)
  - Created `recent_findings.rs` component
  - Displays 3-5 most recent vulnerability findings
  - Shows severity badges and timestamps
- [x] 实现快速操作按钮 (Implement quick action buttons)
  - Created `quick_actions.rs` component
  - Buttons: New Task, Run Scan, Export Report
  - Connected to DashboardPanel action handlers
- [x] 清理 dashboard_ui/theme.rs (迁移到 ui::theme)
  - Removed duplicate theme constants from dashboard_ui
  - Updated all imports to use ui::theme::*
  - Consolidated to single source of truth
```

**Step 2: Add implementation notes**

Add a new section:

```markdown
## Implementation Notes

### Stat Cards
- StatCard is a reusable component that accepts title, value, unit, and color
- Data is pulled from DashboardPanel's task lists on each render
- Colors use constants from ui::theme (SEVERITY_*, STATUS_SUCCESS, ACCENT_*)

### Progress Ring
- Calculates percentage as done_tasks / total_tasks
- Shows remaining task count alongside percentage
- Uses progress bar visualization (easier than true SVG ring in GPUI)

### Recent Findings
- Currently uses hardcoded sample data
- TODO: Connect to VulnData from data layer for real findings
- Shows severity color badges (critical, high, medium, low)

### Quick Actions
- New Task: Creates Todo task via on_new_task()
- Run Scan: Placeholder for future agent trigger implementation
- Export: Placeholder for PDF/CSV export functionality

### Theme Migration
- All dashboard_ui files now import from ui::theme::*
- No color or spacing hardcoding in dashboard_ui
- Single source of truth in crates/ui/src/theme.rs
```

**Step 3: Commit**

```bash
git add crates/dashboard_ui/CLAUDE.local.md
git commit -m "docs: update CLAUDE.local.md with completion status"
```

---

## Verification Checklist

After completing all tasks, run:

```bash
cd /Users/fk/Devlopment/uavred

# 1. Check compilation
cargo check

# 2. Build the project
cargo build

# 3. Run the application
timeout 20 cargo run

# 4. Verify no warnings
cargo clippy -- -D warnings

# 5. Format check
cargo fmt --check
```

Expected results:
- No compilation errors
- No clippy warnings
- No formatting issues
- Application runs successfully

---

## Summary of Changes

| Task | Files Created | Files Modified | LOC Added |
|------|---|---|---|
| Stat Cards | stat_card.rs | dashboard_panel.rs, lib.rs | ~150 |
| Progress Ring | progress_ring.rs | dashboard_panel.rs, lib.rs | ~100 |
| Recent Findings | recent_findings.rs | dashboard_panel.rs, lib.rs | ~150 |
| Quick Actions | quick_actions.rs | dashboard_panel.rs, lib.rs | ~80 |
| Theme Migration | - | All dashboard_ui files, lib.rs | -37 |
| Documentation | - | CLAUDE.local.md | ~25 |
| **Total** | **4 files** | **9 files** | **~600** |

