# UI Subagent 配置与协调指南

## 概述

本文档定义了如何配置和协调多个 UI 开发 subagents 并行工作。

## Subagent 划分策略

### 按 Crate 划分 (推荐)

| Agent 名称 | Crate | 职责 | 复杂度 |
|-----------|-------|------|--------|
| dashboard-agent | `dashboard_ui/` | 仪表盘、任务概览、发现汇总 | 中 |
| vulns-agent | `vulns_ui/` | 漏洞列表、详情、CVE面板 | 高 |
| traffic-agent | `traffic_ui/` | 流量分析、协议解码 | 高 |
| assets-agent | `assets_ui/` | 资产拓扑、节点详情 | 高 |
| flows-agent | `flows_ui/` | 工作流 DAG、执行监控 | 高 |
| devices-agent | `devices_ui/` | 设备列表、设备详情 | 中 |
| monitor-agent | `monitor_ui/` | 容器监控、系统状态 | 低 |
| settings-agent | `settings_ui/` | 设置面板 | 低 |

### 共享资源 (禁止 Subagent 修改)

```
crates/ui/src/
├── theme.rs    # 🔒 主题常量 - 由 UI Lead 统一管理
├── events.rs   # 🔒 事件定义 - 需协调修改
└── actions.rs  # 🔒 Action 定义 - 需协调修改

crates/data/src/
└── models.rs   # 🔒 数据模型 - 由 Data Agent 管理
```

## CLAUDE.local.md 模板

每个 UI crate 应包含 `CLAUDE.local.md`:

```markdown
# [Panel Name] UI Agent Instructions

## Scope
This agent is responsible ONLY for `crates/[name]_ui/`.

## Module Structure
[列出所有文件及其职责]

## Key Data Models
[列出该 panel 使用的数据模型]

## Required Imports Pattern
[标准导入代码]

## Theme Constants to Use
[该 panel 常用的主题常量]

## Component Patterns
[关键代码模式示例]

## DO NOT
- 修改共享文件
- 超出 scope 的修改

## Current TODOs
[具体任务列表]
```

## 启动 Subagent 的命令模式

### 方式 1: 目录隔离启动

```bash
# 终端 1: Vulns Agent
cd /Users/fk/Devlopment/uavred/crates/vulns_ui
claude "你是 vulns-ui-agent，负责漏洞管理面板开发。请阅读 CLAUDE.local.md 了解你的职责范围。"

# 终端 2: Traffic Agent
cd /Users/fk/Devlopment/uavred/crates/traffic_ui
claude "你是 traffic-ui-agent，负责流量分析面板开发。请阅读 CLAUDE.local.md 了解你的职责范围。"

# 终端 3: Assets Agent
cd /Users/fk/Devlopment/uavred/crates/assets_ui
claude "你是 assets-ui-agent，负责资产拓扑面板开发。请阅读 CLAUDE.local.md 了解你的职责范围。"
```

### 方式 2: 项目根目录 + 明确 Scope

```bash
cd /Users/fk/Devlopment/uavred

# 使用 --allowedTools 限制工具范围
claude --allowedTools "Read,Write,Edit,Glob,Grep,Bash" \
  "你是 vulns-ui-agent。你的工作范围仅限于 crates/vulns_ui/ 目录。
   禁止修改: ui/theme.rs, data/models.rs, workspace.rs
   请先阅读 crates/vulns_ui/CLAUDE.local.md"
```

### 方式 3: 使用 Task Agent (在主会话中)

```
我需要你启动一个 UI subagent 来完成 vulns_ui 的开发任务：
- Scope: crates/vulns_ui/
- 任务: 实现漏洞状态变更功能
- 约束: 不修改共享文件
```

## 协调规则

### 1. 接口先行

在启动 subagents 前，确保以下接口已稳定:

```rust
// ui/src/events.rs - 定义好所有 Panel 间通信事件
pub enum WorkspaceEvent {
    ViewChanged(AppView),
    VulnSelected(String),      // vulns_ui 发出
    AssetSelected(String),     // assets_ui 发出
    TrafficFiltered(Filter),   // traffic_ui 发出
}

// data/src/models.rs - 数据模型不再变更
pub struct VulnData { ... }
pub struct TrafficEntry { ... }
```

### 2. 分支策略

```
master
  └── feature/ui-parallel-dev
        ├── ui/vulns-panel      # vulns-agent 工作分支
        ├── ui/traffic-panel    # traffic-agent 工作分支
        ├── ui/assets-panel     # assets-agent 工作分支
        └── ui/flows-panel      # flows-agent 工作分支
```

### 3. 合并顺序

```
1. 先合并无依赖的 panels (monitor_ui, settings_ui)
2. 再合并有交互的 panels (vulns_ui, assets_ui)
3. 最后合并需要跨 panel 通信的功能
```

## 任务分配示例

### Sprint 1: 基础 Panel 完善

| Agent | 任务 | 预期产出 |
|-------|------|---------|
| vulns-agent | 完善漏洞列表筛选和详情展示 | vuln_list.rs, vuln_detail.rs |
| traffic-agent | 实现流量表格虚拟滚动 | traffic_table.rs |
| assets-agent | 完善资产拓扑画布交互 | topology_canvas.rs |
| flows-agent | 实现 DAG 节点拖拽 | dag_canvas.rs |

### Sprint 2: 交互功能

| Agent | 任务 | 依赖 |
|-------|------|------|
| vulns-agent | 漏洞->资产跳转 | assets-agent 完成资产选择 API |
| traffic-agent | 流量->漏洞关联 | vulns-agent 完成漏洞选择事件 |

## 验证清单

每个 Subagent 完成任务后需验证:

- [ ] `cargo check -p [crate_name]` 通过
- [ ] `cargo clippy -p [crate_name]` 无警告
- [ ] 未修改共享文件 (`git diff --name-only` 检查)
- [ ] 代码使用 `ui::theme::*` 常量，无硬编码颜色
- [ ] 组件实现 `Render` trait
- [ ] 状态变更调用 `cx.notify()`

## 常见问题

### Q: Subagent 需要新的数据模型怎么办?

A: 暂停该 agent，由主会话或 data-agent 添加模型到 `data/models.rs`，然后继续。

### Q: 需要新的主题常量?

A: 记录需求，由 UI Lead 统一添加到 `ui/theme.rs`，避免冲突。

### Q: Panel 间需要通信?

A: 在 `ui/events.rs` 添加事件类型，通过 `cx.emit()` 和 `cx.subscribe()` 实现。
