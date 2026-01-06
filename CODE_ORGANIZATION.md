# 代码组织结构说明

## 整体架构

项目采用**模块化分层架构**，将 UI 代码按功能视图拆分为独立模块，提高可维护性和可读性。

```
src/
├── app.rs                    # 应用核心：状态管理、路由、顶层渲染
├── main.rs                   # 应用入口
└── ui/                       # UI 模块目录
    ├── mod.rs               # UI 模块入口，统一导出
    ├── dashboard/           # Dashboard 视图模块
    │   ├── mod.rs
    │   ├── mission_control.rs  # Mission Control (Kanban 看板)
    │   ├── findings.rs         # Findings 视图
    │   └── components.rs       # Dashboard 通用组件
    ├── assets/              # Assets 视图模块
    │   ├── mod.rs
    │   ├── view.rs          # Assets 主视图
    │   └── components.rs    # Assets 组件
    ├── scan/                # Scan 视图模块
    │   ├── mod.rs
    │   └── view.rs
    ├── vulns/               # Vulnerabilities 视图模块
    │   ├── mod.rs
    │   ├── view.rs
    │   └── components.rs
    ├── traffic/             # Traffic 视图模块
    │   ├── mod.rs
    │   ├── view.rs
    │   └── components.rs
    └── flows/               # Flows 视图模块
        ├── mod.rs
        ├── view.rs
        └── components.rs
```

## 各文件职责

### 1. `src/app.rs` (核心应用文件)

**职责**：
- **应用状态管理**：定义 `UavRedTeamApp` 结构体，管理全局状态
- **路由分发**：`render_main_content` 根据 `active_view` 路由到不同视图
- **顶层渲染**：`render` 方法作为 GPUI 的入口点
- **导航栏**：`render_title_bar` 提供顶层导航

**保留的必要代码**：
- 结构体定义：`UavRedTeamApp`, `TaskData`, `AppView`, `DashboardView`, `VulnFilter`
- 状态初始化：`new()`, `set_active_view()`, `set_vuln_filter()`, `set_dashboard_view()`, `select_task()`
- 顶层渲染：`render()`, `render_title_bar()`, `render_main_content()`
- 路由方法：`render_dashboard()`, `render_dashboard_header()`
- 视图包装器：`render_assets()`, `render_scan()`, `render_vulns()`, `render_traffic()`, `render_flows()`

**可移除的冗余代码**：
- `render_navigation()` - 未使用（已被 `render_title_bar` 替代）
- `render_kanban_headers()`, `render_kanban_column_header()`, `render_kanban_board()`, `render_kanban_column_content()` - 已迁移到 `mission_control.rs`
- `render_task_card()`, `render_task_detail_panel()`, `render_ai_activity()` - 已迁移到 `mission_control.rs`
- `get_next_task_id()`, `get_task_data()` - 已迁移到 `mission_control.rs`
- `render_findings_view_old()` - 标记为 `dead_code`，应删除
- `render_finding_filter_tab()`, `render_finding_card()` - 已迁移到 `findings.rs`
- `render_dashboard_tab()` - 包装器，可直接内联

### 2. `src/ui/dashboard/` (Dashboard 模块)

**职责**：Dashboard 视图的所有实现

- **`mission_control.rs`**：
  - `render_mission_control()` - Mission Control 主视图
  - `render_kanban_headers()` - Kanban 三列标题
  - `render_kanban_board()` - Kanban 看板内容
  - `render_task_detail_panel()` - 任务详情面板
  - `get_next_task_id()`, `get_task_data()` - 任务数据辅助方法

- **`findings.rs`**：
  - `render_findings_view()` - Findings 主视图
  - `render_finding_filter_tab()` - 过滤标签
  - `render_finding_card()` - 发现项卡片

- **`components.rs`**：
  - `render_dashboard_tab()` - Dashboard 二级导航标签
  - `render_kanban_column_header()` - Kanban 列标题组件
  - `render_task_card()` - 任务卡片组件
  - `render_ai_activity()` - AI 活动组件

### 3. `src/ui/assets/`, `scan/`, `vulns/`, `traffic/`, `flows/`

**职责**：各自视图的完整实现

每个模块遵循相同结构：
- **`view.rs`**：主视图渲染逻辑
- **`components.rs`**：可复用的 UI 组件
- **`mod.rs`**：模块入口，统一导出

## 代码调用流程

```
main.rs
  └─> App::new() → UavRedTeamApp::new()
      └─> Window::open() → UavRedTeamApp::render()
          └─> render_title_bar()        # 顶层导航栏
          └─> render_main_content()     # 路由分发
              ├─> render_dashboard()
              │   ├─> render_dashboard_header()  # 二级导航
              │   └─> render_mission_control()   # 或 render_findings_view()
              │       └─> (调用 ui/dashboard/mission_control.rs)
              ├─> render_assets()
              │   └─> (调用 ui/assets/view.rs)
              ├─> render_scan()
              │   └─> (调用 ui/scan/view.rs)
              └─> ... (其他视图)
```

## 设计原则

1. **单一职责**：每个模块只负责一个视图的渲染
2. **关注点分离**：状态管理在 `app.rs`，UI 渲染在 `ui/` 模块
3. **可复用性**：通用组件提取到 `components.rs`
4. **可维护性**：代码按功能组织，易于查找和修改

## 优化建议

1. **移除冗余代码**：删除 `app.rs` 中已迁移的旧实现
2. **简化包装器**：将 `render_*` 包装器方法内联到 `render_main_content`
3. **提取通用逻辑**：将 `render_dashboard_header` 移到 `dashboard/mod.rs`
4. **统一组件接口**：确保所有 `components.rs` 使用一致的函数签名
