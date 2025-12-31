# GPUI 实现指南

## 概述

本文档指导开发者如何使用 GPUI 和 gpui-component 库实现 UAVRED UI/UX 设计规范中定义的界面和交互。

---

## 1. GPUI 基础概念

### 1.1 什么是 GPUI

GPUI 是 Zed 编辑器的 UI 框架，具有以下特点：

- **高性能**: 使用 GPU 加速渲染
- **反应式**: 基于 Elm 架构的声明式 UI
- **类型安全**: 完全 Rust 编写

### 1.2 核心概念

| 概念 | 说明 | 示例 |
|------|------|------|
| Element | 最小渲染单位 | Text, Button, Input |
| Component | 可重用组件 | Card, Modal, ListItem |
| State | 组件状态 | selected, visible, data |
| Event | 用户交互 | on_click, on_input_change |
| View | 整个界面 | AppView, DashboardView |

### 1.3 应用架构

```
AppState (全局状态)
    ├── CurrentView (当前视图)
    ├── NavigationBar (导航状态)
    ├── SelectedTask (选中任务)
    ├── ScanConfig (扫描配置)
    └── AgentState (Agent 状态)
         ├── Logs (执行日志)
         └── Status (执行状态)
```

---

## 2. 环境配置

### 2.1 依赖配置

```toml
[dependencies]
gpui = { git = "https://github.com/zed-industries/zed", rev = "..." }
gpui-component = { git = "https://github.com/longbridge/gpui-component" }
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
```

### 2.2 项目结构

```
src/
├── main.rs           # 应用入口
├── app.rs            # 应用顶层组件
├── ui/
│   ├── mod.rs
│   ├── navigation.rs
│   ├── kanban.rs
│   ├── findings.rs
│   ├── agent_panel.rs
│   ├── components/
│   │   ├── button.rs
│   │   ├── card.rs
│   │   ├── modal.rs
│   │   └── ...
│   └── styles.rs     # 全局样式常量
├── models/
│   ├── task.rs
│   ├── finding.rs
│   ├── agent.rs
│   └── ...
└── handlers/
    ├── scan.rs
    └── ...
```

---

## 3. 应用入口

### 3.1 main.rs

```rust
use gpui::{actions, App, AppContext, Global, MouseButton, VisualContext};
use uavred::ui::AppView;
use uavred::app::AppState;

fn main() {
    App::new().run(|cx: &mut AppContext| {
        // 初始化全局状态
        cx.set_global(AppState::default());
        
        // 设置窗口属性
        let options = Default::default();
        cx.open_window(options, |cx| {
            AppView::new(cx)
        });
    })
}
```

### 3.2 app.rs

```rust
use gpui::{prelude::*, IntoElement};
use crate::ui::{NavigationBar, MainContent, AgentPanel};
use crate::models::AppState;

pub struct AppView;

impl AppView {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        Self
    }
    
    pub fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let app_state = cx.global::<AppState>();
        
        div()
            .size_full()
            .bg_color(gpui::rgb(0x1e1e1e))
            .flex()
            .flex_col()
            .child(
                NavigationBar::new(&app_state)
                    .on_tab_changed(|tab| {
                        // 处理 Tab 切换
                    })
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .size_full()
                    .child(
                        MainContent::new(&app_state)
                            .flex_grow(1)
                    )
                    .child(
                        AgentPanel::new(&app_state)
                            .width(px(450.0))
                    )
            )
    }
}

impl Render for AppView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        self.render(cx)
    }
}
```

---

## 4. 导航栏实现

### 4.1 NavigationBar 组件

```rust
use gpui::{prelude::*, IntoElement, MouseButton};
use gpui_component::Badge;
use crate::models::{AppState, ViewTab};

pub struct NavigationBar {
    state: AppState,
}

impl NavigationBar {
    pub fn new(state: &AppState) -> Self {
        Self {
            state: state.clone(),
        }
    }
    
    pub fn render(&self) -> impl IntoElement {
        div()
            .w_full()
            .h(px(44.0))
            .bg_color(rgb(0x1e1e1e))
            .border_b(px(1.0))
            .border_color(rgb(0x2d2d2d))
            .px(px(16.0))
            .flex()
            .items_center()
            .justify_between()
            .child(
                // Logo 和标题
                div()
                    .flex()
                    .items_center()
                    .gap(px(8.0))
                    .child(
                        div()
                            .w(px(24.0))
                            .h(px(24.0))
                            .rounded_full()
                            .bg_color(rgb(0xef4444)) // 红色圆点
                    )
                    .child(
                        Text::new("UAVRED")
                            .font_family(MONOSPACE_FONT)
                            .weight(FontWeight::Bold)
                            .text_color(rgb(0xffffff))
                    )
            )
            .child(self.render_tabs())
            .child(self.render_status())
    }
    
    fn render_tabs(&self) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .gap(px(12.0))
            .child(self.render_tab("Dashboard", ViewTab::Dashboard))
            .child(self.render_tab("Assets", ViewTab::Assets))
            .child(self.render_tab("Scan", ViewTab::Scan))
            .child(self.render_tab_with_badge(
                "Vulns",
                ViewTab::Vulns,
                self.state.vulns_count(),
            ))
            .child(self.render_tab_with_badge(
                "Traffic",
                ViewTab::Traffic,
                self.state.traffic_count(),
            ))
            .child(self.render_tab("Flows", ViewTab::Flows))
    }
    
    fn render_tab(&self, label: &str, tab: ViewTab) -> impl IntoElement {
        button()
            .child(Text::new(label))
            .on_click(move |_ev, cx| {
                // 切换视图
                cx.emit(TabSwitched(tab));
            })
            .text_color(if self.state.current_view == tab {
                rgb(0xffffff)
            } else {
                rgb(0x808080)
            })
            .bg_color(if self.state.current_view == tab {
                rgba(0xa78bfa, 0.3) // 紫色 30% 透明度
            } else {
                transparent()
            })
            .px(px(12.0))
            .py(px(8.0))
            .rounded(px(6.0))
            .hover(|style| {
                style.bg_color(rgb(0x252525))
            })
    }
    
    fn render_tab_with_badge(
        &self,
        label: &str,
        tab: ViewTab,
        count: usize,
    ) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .gap(px(6.0))
            .child(self.render_tab(label, tab))
            .when(count > 0, |div| {
                div.child(
                    Badge::new(Text::new(count.to_string()))
                        .bg_color(rgb(0xef4444))
                        .text_color(rgb(0xffffff))
                )
            })
    }
    
    fn render_status(&self) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .gap(px(8.0))
            .child(
                Text::new(&format!("Target: {}", self.state.current_target()))
                    .text_color(rgb(0xcccccc))
                    .size(px(12.0))
            )
            .child(
                div()
                    .w(px(8.0))
                    .h(px(8.0))
                    .rounded_full()
                    .bg_color(if self.state.agent_active {
                        rgb(0x10b981) // 绿色
                    } else {
                        rgb(0xef4444) // 红色
                    })
            )
            .child(
                Text::new(if self.state.agent_active {
                    "AI Active"
                } else {
                    "AI Idle"
                })
                .text_color(rgb(0xcccccc))
                .size(px(12.0))
            )
    }
}

impl Render for NavigationBar {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        self.render()
    }
}
```

---

## 5. Kanban 看板实现

### 5.1 KanbanBoard 组件

```rust
use gpui::{prelude::*, IntoElement, DragEvent, DropEvent};
use crate::models::{Task, TaskStatus};

pub struct KanbanBoard {
    tasks: Vec<Task>,
    dragging_task: Option<Task>,
}

impl KanbanBoard {
    pub fn new(tasks: Vec<Task>) -> Self {
        Self {
            tasks,
            dragging_task: None,
        }
    }
    
    pub fn render(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .w_full()
            .h_full()
            .gap(px(16.0))
            .flex()
            .flex_row()
            .px(px(16.0))
            .py(px(16.0))
            .child(self.render_column(TaskStatus::Todo, cx))
            .child(self.render_column(TaskStatus::InProgress, cx))
            .child(self.render_column(TaskStatus::Done, cx))
    }
    
    fn render_column(
        &self,
        status: TaskStatus,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        let column_tasks: Vec<_> = self.tasks
            .iter()
            .filter(|t| t.status == status)
            .collect();
        
        let title = match status {
            TaskStatus::Todo => "To Do",
            TaskStatus::InProgress => "In Progress",
            TaskStatus::Done => "Done",
        };
        
        div()
            .flex()
            .flex_col()
            .min_w(px(300.0))
            .bg_color(rgb(0x252525))
            .rounded(px(8.0))
            .p(px(12.0))
            .gap(px(8.0))
            .child(
                div()
                    .flex()
                    .justify_between()
                    .items_center()
                    .child(
                        div()
                            .flex()
                            .gap(px(6.0))
                            .child(
                                Text::new(title)
                                    .weight(FontWeight::Bold)
                                    .text_color(rgb(0xffffff))
                            )
                            .child(
                                Badge::new(
                                    Text::new(column_tasks.len().to_string())
                                        .text_color(rgb(0x808080))
                                )
                                .bg_color(rgb(0x2d2d2d))
                            )
                    )
                    .child(
                        button()
                            .child(Text::new("+"))
                            .on_click(move |_ev, cx| {
                                // 新增任务
                                cx.emit(CreateTask(status.clone()));
                            })
                    )
            )
            .child(
                // 任务卡片列表（可滚动）
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .when(!column_tasks.is_empty(), |div| {
                        column_tasks.iter().fold(div, |div, task| {
                            div.child(self.render_task_card(task, status.clone(), cx))
                        })
                    })
                    .when(column_tasks.is_empty(), |div| {
                        div.child(
                            div()
                                .w_full()
                                .py(px(20.0))
                                .text_center()
                                .child(
                                    Text::new("No tasks")
                                        .text_color(rgb(0x808080))
                                )
                        )
                    })
            )
    }
    
    fn render_task_card(
        &self,
        task: &Task,
        _status: TaskStatus,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        div()
            .w_full()
            .min_h(px(80.0))
            .bg_color(rgb(0x1e1e1e))
            .border(px(1.0))
            .border_color(rgb(0x2d2d2d))
            .rounded(px(6.0))
            .p(px(12.0))
            .gap(px(8.0))
            .flex()
            .flex_col()
            .on_mouse_down(|_ev, cx| {
                // 处理拖拽
                cx.emit(TaskDragStart(task.id.clone()));
            })
            .hover(|style| {
                style.bg_color(rgb(0x252525))
                    .border_color(rgb(0x3d3d3d))
            })
            // 左侧彩色指示条
            .child(
                div()
                    .w(px(4.0))
                    .h(px(40.0))
                    .absolute()
                    .left(px(0.0))
                    .top(px(12.0))
                    .rounded_l(px(6.0))
                    .bg_color(task.priority.color())
            )
            .child(
                // 标题和 Tag
                div()
                    .flex()
                    .justify_between()
                    .items_center()
                    .pl(px(12.0))
                    .child(
                        div()
                            .flex()
                            .gap(px(6.0))
                            .child(
                                Text::new(&task.title)
                                    .weight(FontWeight::Bold)
                                    .text_color(rgb(0xffffff))
                            )
                    )
                    .child(
                        badge()
                            .label(task.task_type.to_string())
                            .bg_color(task.task_type.color())
                    )
            )
            .child(
                // 元数据
                div()
                    .flex()
                    .justify_between()
                    .text_color(rgb(0x808080))
                    .size(px(12.0))
                    .child(Text::new(&format!("📍 {}", task.target)))
                    .child(Text::new(&task.created_at.format("%Y-%m-%d").to_string()))
            )
    }
}

impl Render for KanbanBoard {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        self.render(cx)
    }
}
```

---

## 6. Findings 列表实现

### 6.1 FindingsView 组件

```rust
use gpui::{prelude::*, IntoElement};
use gpui_component::Input;
use crate::models::{Finding, Severity};

pub struct FindingsView {
    findings: Vec<Finding>,
    filter: Severity,
    search_text: String,
}

impl FindingsView {
    pub fn new(findings: Vec<Finding>) -> Self {
        Self {
            findings,
            filter: Severity::All,
            search_text: String::new(),
        }
    }
    
    pub fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .w_full()
            .h_full()
            .bg_color(rgb(0x1e1e1e))
            .flex()
            .flex_col()
            .gap(px(16.0))
            .p(px(16.0))
            .child(self.render_stats())
            .child(self.render_controls(cx))
            .child(self.render_filter_tabs(cx))
            .child(self.render_list(cx))
    }
    
    fn render_stats(&self) -> impl IntoElement {
        div()
            .w_full()
            .bg_color(rgb(0x252525))
            .rounded(px(8.0))
            .px(px(16.0))
            .py(px(12.0))
            .flex()
            .gap(px(16.0))
            .child(
                self.render_stat_item(
                    "Total",
                    self.findings.len(),
                    rgb(0xffffff),
                )
            )
            .child(
                self.render_stat_item(
                    "Critical",
                    self.findings.iter().filter(|f| f.severity == Severity::Critical).count(),
                    rgb(0xef4444),
                )
            )
            .child(
                self.render_stat_item(
                    "High",
                    self.findings.iter().filter(|f| f.severity == Severity::High).count(),
                    rgb(0xf97316),
                )
            )
    }
    
    fn render_stat_item(
        &self,
        label: &str,
        count: usize,
        color: Hsla,
    ) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .gap(px(6.0))
            .child(
                div()
                    .w(px(8.0))
                    .h(px(8.0))
                    .rounded_full()
                    .bg_color(color)
            )
            .child(
                Text::new(format!("{}: {}", label, count))
                    .text_color(rgb(0xcccccc))
                    .size(px(13.0))
            )
    }
    
    fn render_controls(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .w_full()
            .flex()
            .items_center()
            .gap(px(12.0))
            .child(
                Input::new()
                    .placeholder("Search findings...")
                    .on_change(|text, cx| {
                        cx.emit(SearchChanged(text));
                    })
                    .flex_grow(1)
            )
            .child(
                button()
                    .child(Text::new("Export"))
                    .on_click(|_ev, cx| {
                        cx.emit(ExportFindings);
                    })
            )
    }
    
    fn render_filter_tabs(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .w_full()
            .flex()
            .gap(px(8.0))
            .child(self.render_filter_tab(Severity::All, cx))
            .child(self.render_filter_tab(Severity::Critical, cx))
            .child(self.render_filter_tab(Severity::High, cx))
            .child(self.render_filter_tab(Severity::Medium, cx))
            .child(self.render_filter_tab(Severity::Low, cx))
    }
    
    fn render_filter_tab(
        &mut self,
        severity: Severity,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        button()
            .child(Text::new(severity.label()))
            .on_click(move |_ev, cx| {
                cx.emit(FilterChanged(severity.clone()));
            })
            .bg_color(if self.filter == severity {
                rgb(0xa78bfa)
            } else {
                transparent()
            })
            .text_color(if self.filter == severity {
                rgb(0xffffff)
            } else {
                rgb(0x808080)
            })
            .px(px(12.0))
            .py(px(8.0))
            .rounded(px(6.0))
    }
    
    fn render_list(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let filtered: Vec<_> = self.findings
            .iter()
            .filter(|f| {
                let matches_filter = self.filter == Severity::All || f.severity == self.filter;
                let matches_search = self.search_text.is_empty() 
                    || f.title.to_lowercase().contains(&self.search_text.to_lowercase());
                matches_filter && matches_search
            })
            .collect();
        
        div()
            .w_full()
            .flex()
            .flex_col()
            .gap(px(8.0))
            .when(!filtered.is_empty(), |div| {
                filtered.iter().fold(div, |div, finding| {
                    div.child(self.render_finding_item(finding, cx))
                })
            })
    }
    
    fn render_finding_item(
        &mut self,
        finding: &Finding,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        div()
            .w_full()
            .min_h(px(100.0))
            .bg_color(rgb(0x252525))
            .border(px(1.0))
            .border_color(rgb(0x2d2d2d))
            .rounded(px(8.0))
            .p(px(12.0))
            .gap(px(8.0))
            .flex()
            .flex_col()
            .on_click(|_ev, cx| {
                // 打开详情 Drawer
                cx.emit(ShowFindingDetail(finding.id.clone()));
            })
            .hover(|style| {
                style.bg_color(rgb(0x2d2d2d))
                    .border_color(rgb(0x3d3d3d))
            })
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(8.0))
                    .child(
                        div()
                            .w(px(8.0))
                            .h(px(8.0))
                            .rounded_full()
                            .bg_color(finding.severity.color())
                    )
                    .child(
                        Text::new(&format!("CVE-{}", finding.cve))
                            .weight(FontWeight::Bold)
                            .text_color(rgb(0xffffff))
                    )
                    .child(
                        badge()
                            .label(finding.status.to_string())
                            .bg_color(finding.status.color())
                    )
            )
            .child(
                Text::new(&finding.title)
                    .weight(FontWeight::Bold)
                    .size(px(14.0))
                    .text_color(rgb(0xffffff))
            )
            .child(
                Text::new(&finding.description)
                    .size(px(12.0))
                    .text_color(rgb(0xcccccc))
            )
            .child(
                div()
                    .flex()
                    .justify_between()
                    .text_color(rgb(0x808080))
                    .size(px(12.0))
                    .child(Text::new(&finding.target))
                    .child(Text::new(&format!("{}m ago", finding.time_ago)))
            )
    }
}

impl Render for FindingsView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        self.render(cx)
    }
}
```

---

## 7. Agent Panel 实现

### 7.1 AgentPanel 组件

```rust
use gpui::{prelude::*, IntoElement};
use crate::models::{AgentState, AgentLog, AgentLogType};

pub struct AgentPanel {
    agent_state: AgentState,
    logs: Vec<AgentLog>,
    scroll_to_bottom: bool,
}

impl AgentPanel {
    pub fn new(agent_state: AgentState) -> Self {
        Self {
            agent_state,
            logs: Vec::new(),
            scroll_to_bottom: true,
        }
    }
    
    pub fn render(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .w(px(350.0))
            .h_full()
            .bg_color(rgb(0x1e1e1e))
            .border_l(px(1.0))
            .border_color(rgb(0x2d2d2d))
            .p(px(12.0))
            .flex()
            .flex_col()
            .gap(px(8.0))
            .child(self.render_header())
            .child(self.render_mission())
            .child(self.render_logs(cx))
            .child(self.render_controls())
    }
    
    fn render_header(&self) -> impl IntoElement {
        div()
            .w_full()
            .flex()
            .justify_between()
            .items_center()
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(6.0))
                    .child(Text::new("🤖"))
                    .child(
                        Text::new(&self.agent_state.name)
                            .weight(FontWeight::Bold)
                            .text_color(rgb(0xffffff))
                    )
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(4.0))
                    .child(
                        div()
                            .w(px(6.0))
                            .h(px(6.0))
                            .rounded_full()
                            .bg_color(if self.agent_state.running {
                                rgb(0xef4444) // 红色
                            } else {
                                rgb(0x10b981) // 绿色
                            })
                    )
                    .child(
                        Text::new(if self.agent_state.running {
                            "LIVE TRACE"
                        } else {
                            "IDLE"
                        })
                        .text_color(rgb(0x808080))
                        .size(px(11.0))
                    )
            )
    }
    
    fn render_mission(&self) -> impl IntoElement {
        div()
            .w_full()
            .bg_color(rgb(0x252525))
            .rounded(px(6.0))
            .p(px(10.0))
            .flex()
            .gap(px(6.0))
            .child(Text::new("🎯"))
            .child(
                Text::new(&self.agent_state.mission_objective)
                    .text_color(rgb(0xffffff))
                    .size(px(12.0))
            )
    }
    
    fn render_logs(&self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .w_full()
            .flex_grow(1)
            .flex()
            .flex_col()
            .gap(px(2.0))
            .bg_color(rgb(0x252525))
            .rounded(px(6.0))
            .p(px(8.0))
            .overflow_y_scroll()
            // 虚拟滚动（如果日志很多）
            .when(!self.logs.is_empty(), |div| {
                self.logs.iter().fold(div, |div, log| {
                    div.child(self.render_log_item(log))
                })
            })
    }
    
    fn render_log_item(&self, log: &AgentLog) -> impl IntoElement {
        div()
            .w_full()
            .flex()
            .gap(px(6.0))
            .min_h(px(18.0))
            .child(
                // 日志类型 emoji
                Text::new(match log.log_type {
                    AgentLogType::History => "⚪",
                    AgentLogType::Thought => "💭",
                    AgentLogType::Plan => "📋",
                    AgentLogType::Tool => "🔧",
                })
                .size(px(12.0))
            )
            .child(
                // 时间戳
                Text::new(&log.timestamp)
                    .text_color(rgb(0x808080))
                    .size(px(10.0))
                    .font_family(MONOSPACE_FONT)
                    .min_w(px(60.0))
            )
            .child(
                // 内容
                Text::new(&log.message)
                    .text_color(match log.log_type {
                        AgentLogType::History => rgb(0xcccccc),
                        AgentLogType::Thought => rgb(0xa78bfa),
                        AgentLogType::Plan => rgb(0xfbbf24),
                        AgentLogType::Tool => rgb(0x60a5fa),
                    })
                    .size(px(11.0))
                    .font_family(MONOSPACE_FONT)
                    .flex_grow(1)
            )
    }
    
    fn render_controls(&self) -> impl IntoElement {
        div()
            .w_full()
            .flex()
            .gap(px(6.0))
            .child(
                button()
                    .child(Text::new(if self.agent_state.running {
                        "⏸"
                    } else {
                        "▶"
                    }))
                    .on_click(|_ev, cx| {
                        cx.emit(ToggleAgentPause);
                    })
                    .flex_grow(1)
            )
            .child(
                button()
                    .child(Text::new("⏹"))
                    .on_click(|_ev, cx| {
                        cx.emit(StopAgent);
                    })
                    .flex_grow(1)
            )
    }
}

impl Render for AgentPanel {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        self.render(cx)
    }
}
```

---

## 8. 样式常量

### 8.1 styles.rs

```rust
use gpui::Hsla;

// 颜色定义
pub const COLOR_BG_PRIMARY: Hsla = rgb(0x1e1e1e);
pub const COLOR_BG_SECONDARY: Hsla = rgb(0x252525);
pub const COLOR_BG_TERTIARY: Hsla = rgb(0x2d2d2d);

pub const COLOR_FG_PRIMARY: Hsla = rgb(0xffffff);
pub const COLOR_FG_SECONDARY: Hsla = rgb(0xcccccc);
pub const COLOR_FG_TERTIARY: Hsla = rgb(0x808080);

pub const COLOR_SUCCESS: Hsla = rgb(0x10b981);
pub const COLOR_WARNING: Hsla = rgb(0xfbbf24);
pub const COLOR_ERROR: Hsla = rgb(0xef4444);
pub const COLOR_INFO: Hsla = rgb(0x60a5fa);

pub const COLOR_ACCENT_PRIMARY: Hsla = rgb(0xa78bfa);
pub const COLOR_ACCENT_SECONDARY: Hsla = rgb(0xf97316);

// 间距
pub const GAP_1: f32 = 4.0;
pub const GAP_2: f32 = 8.0;
pub const GAP_3: f32 = 12.0;
pub const GAP_4: f32 = 16.0;
pub const GAP_5: f32 = 24.0;

// 圆角
pub const RADIUS_SM: f32 = 4.0;
pub const RADIUS_MD: f32 = 6.0;
pub const RADIUS_LG: f32 = 8.0;

// 字体
pub const FONT_SIZE_SM: f32 = 11.0;
pub const FONT_SIZE_BASE: f32 = 12.0;
pub const FONT_SIZE_MD: f32 = 13.0;
pub const FONT_SIZE_LG: f32 = 14.0;
pub const FONT_SIZE_XL: f32 = 16.0;

pub const MONOSPACE_FONT: &str = "Courier New";
pub const SANS_FONT: &str = "-apple-system, segoe-ui, sans-serif";

// 高度
pub const NAV_BAR_HEIGHT: f32 = 44.0;
pub const AGENT_PANEL_WIDTH: f32 = 350.0;

// 过渡时间
pub const TRANSITION_FAST: f32 = 100.0;
pub const TRANSITION_BASE: f32 = 200.0;
pub const TRANSITION_SLOW: f32 = 300.0;
```

---

## 9. 状态管理

### 9.1 AppState 定义

```rust
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AppState {
    pub current_view: ViewTab,
    pub current_target: String,
    pub agent_active: bool,
    pub tasks: Vec<Task>,
    pub findings: Vec<Finding>,
    pub assets: Vec<Asset>,
    pub agent_state: AgentState,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ViewTab {
    Dashboard,
    Assets,
    Scan,
    Vulns,
    Traffic,
    Flows,
}

impl AppState {
    pub fn vulns_count(&self) -> usize {
        self.findings.iter()
            .filter(|f| f.severity == Severity::Critical || f.severity == Severity::High)
            .count()
    }
    
    pub fn traffic_count(&self) -> usize {
        // 返回最近的流量包计数
        0
    }
    
    pub fn current_target(&self) -> String {
        self.current_target.clone()
    }
}
```

---

## 10. 常见模式

### 10.1 列表渲染（带虚拟滚动）

```rust
// 对于大列表，使用虚拟滚动
div()
    .w_full()
    .h_full()
    .flex()
    .flex_col()
    .overflow_y_scroll()
    .when(!items.is_empty(), |div| {
        items.iter()
            .skip(scroll_offset)
            .take(visible_count)
            .fold(div, |div, item| {
                div.child(render_item(item))
            })
    })
```

### 10.2 Modal 对话框

```rust
// 显示 Modal
div()
    .when(show_modal, |div| {
        div.child(
            div()
                .fixed()
                .inset(px(0.0))
                .z_index(1000)
                .bg_color(rgba(0x000000, 0.5))
                .on_click(|_ev, cx| {
                    cx.emit(CloseModal);
                })
                .flex()
                .items_center()
                .justify_center()
                .child(
                    div()
                        .w(px(500.0))
                        .bg_color(rgb(0x252525))
                        .rounded(px(8.0))
                        .p(px(20.0))
                        .gap(px(12.0))
                        .flex()
                        .flex_col()
                        // Modal 内容
                        .on_click(|ev, cx| {
                            ev.stop_propagation(); // 阻止点击背景
                        })
                )
        )
    })
```

### 10.3 Drawer 侧边面板

```rust
div()
    .when(show_drawer, |div| {
        div.child(
            div()
                .fixed()
                .top(px(44.0))
                .right(px(0.0))
                .bottom(px(0.0))
                .w(px(350.0))
                .z_index(500)
                .bg_color(rgb(0x252525))
                .border_l(px(1.0))
                .border_color(rgb(0x2d2d2d))
                .flex()
                .flex_col()
                .gap(px(12.0))
                .p(px(16.0))
                .overflow_y_scroll()
                // Drawer 内容
        )
    })
```

---

## 11. 性能优化建议

### 11.1 减少重新渲染

```rust
// 使用 Memoization
let memoized_value = cx.memo(|_| expensive_computation());

// 只在必要时订阅状态变化
cx.subscribe(&item, |this, item, cx| {
    this.handle_item_changed(&item, cx);
})
```

### 11.2 异步操作

```rust
// 在后台加载数据
cx.spawn_weak(|this, mut cx| async move {
    let data = fetch_data().await;
    this.update(&mut cx, |this, cx| {
        this.data = data;
        cx.notify();
    });
})
```

### 11.3 虚拟滚动

对于大列表（>100 项），实现虚拟滚动以提高性能。

---

## 12. 调试技巧

### 12.1 日志输出

```rust
use log::{debug, info};

debug!("Current state: {:?}", self.state);
info!("User clicked button");
```

### 12.2 Layout 调试

```rust
// 显示元素边界
div()
    .border(px(1.0))
    .border_color(rgb(0xff0000))
    .child(...)
```

---

## 13. 测试编写

### 13.1 单元测试

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_kanban_column_rendering() {
        let tasks = vec![
            Task::new("Test 1", TaskStatus::Todo),
        ];
        let board = KanbanBoard::new(tasks);
        // 断言结果
    }
}
```

---

## 14. 部署和构建

### 14.1 构建应用

```bash
cargo build --release
```

### 14.2 打包为应用

```bash
# macOS
cargo bundle --release

# Windows/Linux
cargo build --release
```

---

## 附录：有用的资源

- [GPUI GitHub](https://github.com/zed-industries/zed)
- [gpui-component GitHub](https://github.com/longbridge/gpui-component)
- [Elm 架构](https://guide.elm-lang.org/architecture/)
- [GPUI 常见问题](https://github.com/zed-industries/zed/discussions)

