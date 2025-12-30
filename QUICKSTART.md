# Quick Start Guide

快速开始使用 UAV Red Team 进行无人机渗透测试。

## 前置要求

### 系统要求
- macOS 10.15+ / Linux / Windows 10+
- Rust 1.75 或更高版本
- Git

### 安装 Rust

如果尚未安装 Rust:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

验证安装:
```bash
rustc --version
cargo --version
```

## 安装步骤

### 1. 克隆项目
```bash
cd /Users/fk/Devlopment/uavred
```

### 2. 运行设置脚本
```bash
make setup
# 或
./scripts/setup.sh
```

### 3. 构建项目
```bash
# 开发构建
cargo build

# 或使用 Makefile
make build
```

## 运行应用

### 开发模式
```bash
cargo run
# 或
make run
```

### 自动重载(推荐开发时使用)
```bash
cargo watch -x run
# 或
make dev
```

### 发布构建
```bash
cargo build --release
# 或
make release
```

## 基本使用

### 1. 启动应用
运行应用后,你将看到主控制面板 Dashboard。

### 2. 开始网络扫描
- 点击 "Start Network Scan" 按钮
- 输入目标网络范围(如: 192.168.1.0/24)
- Agent 将自动扫描并识别无人机设备

### 3. 查看结果
- 扫描完成后,结果将显示在 Results 面板
- 发现的漏洞会自动记录到漏洞数据库

### 4. 协议分析
- 对发现的设备进行协议分析
- 支持 MAVLink, DJI, ArduPilot 等协议
- 自动检测协议弱点

## 开发工作流

### 代码检查
```bash
make check    # 快速检查
make clippy   # 运行 linter
make fmt      # 格式化代码
```

### 测试
```bash
make test     # 运行测试
cargo test -- --nocapture  # 显示输出
```

### 完整流程
```bash
make all      # 格式化 + lint + 测试 + 构建
```

## 配置

### 创建配置文件
```bash
cp config.example.toml config.toml
```

### 编辑配置
```toml
[network]
timeout_seconds = 30
max_concurrent_scans = 10

[ui]
theme = "dark"
window_width = 1280
window_height = 800
```

## 故障排除

### GPUI 依赖问题
如果遇到 GPUI 相关的编译错误:
```bash
# 清理并重新构建
cargo clean
cargo build
```

### 权限问题
某些扫描功能需要管理员权限:
```bash
sudo cargo run
```

### 日志调试
查看详细日志:
```bash
RUST_LOG=debug cargo run
```

## 下一步

- 阅读 [AGENTS.md](./AGENTS.md) 了解 Agent 系统
- 查看 [docs/ARCHITECTURE.md](./docs/ARCHITECTURE.md) 了解架构
- 参考 [docs/UI_COMPONENTS.md](./docs/UI_COMPONENTS.md) 学习 UI 开发

## 安全提醒

⚠️ **重要**: 
- 仅在授权环境中使用
- 遵守当地法律法规
- 不要对未授权目标进行测试
- 妥善保管测试结果

## 获取帮助

- 查看 [README.md](./README.md) 获取更多信息
- 阅读 [CONTRIBUTING.md](./CONTRIBUTING.md) 参与贡献
- 提交 Issue 报告问题

## 快捷命令参考

```bash
make help       # 显示所有可用命令
make build      # 构建项目
make run        # 运行应用
make test       # 运行测试
make dev        # 开发模式(自动重载)
make clean      # 清理构建产物
make fmt        # 格式化代码
make clippy     # 运行 linter
make release    # 发布构建
```
