# Monitor UI Agent Instructions

## Scope
This agent is responsible ONLY for `crates/monitor_ui/` - the container/agent execution monitoring panel.

## Module Structure
```
monitor_ui/
├── lib.rs            # MonitorPanel - 主面板 (容器网格布局)
└── container_card.rs # 容器卡片组件 (终端样式 + 资源监控)
```

## Key Data Models (from `data::models`)
```rust
pub struct ContainerStatus {
    pub container_id: String,       // Docker 容器 ID
    pub agent: String,              // Agent 名称
    pub task_name: String,          // 任务名称
    pub docker_exec_command: String,// 执行命令
    pub status: ContainerExecutionStatus,
    pub running_duration: String,   // "2m 30s"
    pub cpu_usage_percent: f64,     // 0.0 - 100.0
    pub memory_usage_mb: u64,
    pub memory_limit_mb: u64,
    pub exposed_ports: Vec<String>,
}

pub enum ContainerExecutionStatus {
    Running,   // 运行中
    Stopped,   // 已停止
    Building,  // 构建中
}
```

## Required Imports Pattern
```rust
use gpui::*;
use gpui_component::{
    h_flex, v_flex,
    button::Button,
    label::Label,
    tag::Tag,
    group_box::{GroupBox, GroupBoxVariants},
    Sizable,
};
use data::{ContainerStatus, ContainerExecutionStatus};
use ui::theme::*;
```

## Container Card Design (Terminal Style)
```
┌─────────────────────────────────────────┐
│ $ docker exec -it agent-scan-001...    │ ← 深色终端头部
│ Agent: network-scanner                  │
│ Task: port-scan                         │
│ [2m 30s] Running...            ↗ 45%   │
├─────────────────────────────────────────┤
│ Container ID: abc123                    │ ← 浅色信息区域
│ Agent: network-scanner                  │
│                                         │
│ CPU    45%  ████████░░░░░░░░░░░        │
│ Memory 67%  ████████████░░░░░░░        │
│                                         │
│ [Running]  端口: 8080, 8443             │
│ 运行时长: 2m 30s                        │
└─────────────────────────────────────────┘
```

## Theme Constants to Use
- Terminal header: `BG_DARK` (0x1f2937)
- Terminal text: `STATUS_SUCCESS` (green for command)
- Status colors:
  - Running: `STATUS_SUCCESS`
  - Building: `STATUS_WARNING`
  - Stopped: `TEXT_SECONDARY`
- Progress bars: `STATUS_WARNING` (CPU orange), `ACCENT_BLUE` (Memory)
- Card body: `BG_CARD`, `TEXT_PRIMARY`, `TEXT_SECONDARY`

## Current Code Issues to Fix
```rust
// container_card.rs:27-29 - 硬编码状态背景色
let status_bg = match container.status {
    Running => rgb(0xf0fdf4),   // ❌ 需要定义 STATUS_SUCCESS_BG
    Stopped => rgb(0xf3f4f6),   // ❌ 需要定义 STATUS_MUTED_BG
    Building => rgb(0xfffbeb),  // ❌ 需要定义 STATUS_WARNING_BG
};

// container_card.rs:229, 245 - 进度条背景硬编码
.bg(rgb(0xe5e7eb))  // ❌ 改为 rgb(BORDER_COLOR) 或定义 PROGRESS_BG
```

## Progress Bar Pattern
```rust
fn render_progress_bar(percentage: f64, fill_color: Rgba) -> impl IntoElement {
    let clamped = percentage.clamp(0.0, 1.0);

    h_flex()
        .flex_1()
        .h(px(6.0))
        .bg(rgb(BG_SECONDARY))  // 使用 theme 常量
        .rounded(BORDER_RADIUS_SM)
        .overflow_hidden()
        .child(
            div()
                .h_full()
                .bg(fill_color)
                .w(DefiniteLength::Fraction(clamped as f32))
        )
}
```

## DO NOT
- 修改共享文件
- 实现实际的 Docker 操作 (属于 `agent` crate)
- 添加新的容器状态类型

## Current TODOs
- [ ] 替换硬编码颜色为 theme 常量
- [ ] 添加状态背景色常量到 theme.rs (协调)
- [ ] 实现"创建镜像"按钮功能 (lib.rs:45)
- [ ] 添加容器操作按钮 (启动/停止/删除)
- [ ] 实现日志查看弹窗
- [ ] 添加资源使用趋势图
- [ ] 实现容器状态实时更新 (WebSocket/轮询)
