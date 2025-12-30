# UAV Red Team

自主调度 Agent 驱动的无人机生态红队渗透测试工具

## 概述

UAV Red Team 是一个基于 Rust 和 [gpui-component](https://github.com/longbridge/gpui-component) 的跨平台桌面应用,专门用于对无人机生态系统进行自动化渗透测试。该工具利用智能 Agent 自主调度和执行各种安全测试任务。

使用 longbridge/gpui-component 提供的丰富 UI 组件(60+ 组件),构建高性能、原生体验的桌面应用界面。

## 特性

- 🤖 **智能 Agent 调度**: 自主规划和执行渗透测试任务
- 🎯 **无人机协议支持**: MAVLink, DJI, ArduPilot 等主流协议
- 🔍 **多维度扫描**: 网络、固件、通信链路、控制协议
- 📊 **现代化 UI**: 基于 gpui-component,提供 60+ 丰富组件
- 🎨 **高信息密度**: 简洁优雅的界面设计,避免过度装饰
- ⚡ **高性能渲染**: 虚拟化表格和列表,流畅处理大数据
- 🚀 **跨平台支持**: macOS, Linux, Windows
- 🔐 **漏洞数据库**: 内置无人机常见漏洞库
- 📝 **Markdown 支持**: 原生 Markdown 和简单 HTML 渲染
- 📈 **内置图表**: 数据可视化支持

## 架构

```
uav-redteam/
├── src/
│   ├── main.rs           # 应用入口
│   ├── app.rs            # 应用状态管理
│   ├── ui/               # UI 组件
│   │   ├── mod.rs
│   │   ├── dashboard.rs  # 主控制面板
│   │   ├── agents.rs     # Agent 管理界面
│   │   └── results.rs    # 结果展示
│   ├── agent/            # Agent 系统
│   │   ├── mod.rs
│   │   ├── scheduler.rs  # 任务调度器
│   │   └── executor.rs   # 任务执行器
│   ├── scanner/          # 扫描模块
│   │   ├── mod.rs
│   │   ├── network.rs    # 网络扫描
│   │   ├── protocol.rs   # 协议分析
│   │   └── firmware.rs   # 固件分析
│   └── core/             # 核心功能
│       ├── mod.rs
│       ├── task.rs       # 任务定义
│       └── vuln_db.rs    # 漏洞数据库
```

## 快速开始

### 环境要求

- Rust 1.75+
- macOS 10.15+ / Linux / Windows 10+

### 构建

```bash
cargo build --release
```

### 运行

```bash
cargo run
```

## 开发

```bash
# 运行测试
cargo test

# 代码检查
cargo clippy

# 格式化
cargo fmt
```

## License

MIT
