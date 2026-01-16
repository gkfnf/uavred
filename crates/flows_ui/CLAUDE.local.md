# Flows UI Agent Instructions

## Scope
This agent is responsible ONLY for `crates/flows_ui/` - the workflow DAG panel.

## Module Structure
```
flows_ui/
├── lib.rs          # FlowsPanel - 主面板布局
├── flow_list.rs    # 左侧工作流列表
├── dag_canvas.rs   # DAG 画布 (节点+连线)
└── action_bar.rs   # 底部操作栏 (执行/暂停/配置)
```

## Key Data Models (from `data::models`)
- `FlowDefinition` - 工作流定义
- `FlowNode` - DAG 节点 (扫描/分析/报告等)
- `FlowNodeType` - Scanner/Analyzer/Reporter/Conditional
- `FlowEdge` - 节点间连接
- `FlowExecution` - 执行实例状态

## Required Imports Pattern
```rust
use gpui::*;
use gpui_component::{h_flex, v_flex, canvas::Canvas, button::Button};
use data::models::{FlowDefinition, FlowNode, FlowNodeType, FlowEdge};
use ui::theme::*;
```

## DAG Layout Algorithm
```rust
// 使用拓扑排序 + 分层布局
fn layout_dag(&self, nodes: &[FlowNode], edges: &[FlowEdge]) -> HashMap<String, Point<Pixels>> {
    // 1. 拓扑排序确定层级
    // 2. 同层节点垂直分布
    // 3. 最小化边交叉
}
```

## Node State Rendering
```rust
// 根据执行状态渲染节点
fn render_node(&self, node: &FlowNode, execution: Option<&FlowExecution>) -> impl IntoElement {
    let bg_color = match execution.map(|e| &e.status) {
        Some(FlowStatus::Running) => STATUS_AI,      // 紫色-运行中
        Some(FlowStatus::Success) => STATUS_SUCCESS, // 绿色-成功
        Some(FlowStatus::Failed) => STATUS_ERROR,    // 红色-失败
        None => BG_CARD,                              // 默认-未执行
    };
}
```

## Theme Constants to Use
- Node backgrounds: `BG_CARD`, `STATUS_*` colors
- Edge lines: `BORDER_COLOR`, directional arrows
- Selection: `BORDER_FOCUSED`, `ACCENT_PURPLE`
- Action bar: `BG_SECONDARY`

## DO NOT
- 修改共享文件
- 实现实际的工作流执行 (属于 `agent/executor.rs`)
- 修改工作流数据结构

## Current TODOs
- [ ] 实现 DAG 自动布局算法
- [ ] 添加节点拖拽重排
- [ ] 实现边的创建/删除交互
- [ ] 添加节点配置弹窗
- [ ] 实现执行进度实时更新
