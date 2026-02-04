# Boxlite 集成指南

本文档介绍如何在 agent crate 中启用和使用 Boxlite 后端。

## 概述

Boxlite 是一个基于 libkrun 的 MicroVM 沙箱运行时，提供最高级别的隔离：

- **硬件虚拟化**: 使用 KVM (Linux) 或 Hypervisor.framework (macOS)
- **MicroVM**: 最小攻击面，快速启动
- **virtiofs**: 高效的文件共享
- **vsock**: 安全的通信通道

## 系统要求

### Linux
- KVM 支持（/dev/kvm 可访问）
- libkrun 库
- 内核版本 >= 5.10

### macOS
- macOS 11+ (Big Sur)
- Hypervisor.framework (内置)

## 启用 Boxlite 后端

### 1. 初始化 Git Submodules

Boxlite 依赖一些 C 库和 Go 工具，需要初始化子模块：

```bash
cd /Users/fk/.superset/worktrees/uavred/sandbox-intergration-dev
git submodule update --init --recursive
```

### 2. 安装构建依赖

**Ubuntu/Debian:**
```bash
sudo apt-get update
sudo apt-get install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    golang-go \
    libfuse-dev
```

**macOS:**
```bash
# 安装 Xcode Command Line Tools
xcode-select --install

# 使用 Homebrew 安装 Go (如果需要)
brew install go
```

### 3. 修改 Cargo.toml

在依赖于 agent crate 的项目中启用特性：

```toml
[dependencies]
agent = { 
    path = "../crates/agent", 
    features = ["boxlite-backend"] 
}
```

或者在主项目中添加 boxlite 到 workspace：

```toml
[workspace]
members = [
    # ... 其他成员
    "src/boxlite/boxlite",
    "src/boxlite/boxlite-shared",
    "src/boxlite/guest",
]
```

### 4. 编译测试

```bash
# 编译 agent crate 带 boxlite 特性
cargo build -p agent --features boxlite-backend

# 运行测试
cargo test -p agent --features boxlite-backend
```

## 使用示例

### 基本使用

```rust
use std::sync::Arc;
use agent::sandbox::{SandboxScheduler, SandboxConfig, TaskSpec};
use agent::sandbox::backends::BoxliteBackend;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 创建 Boxlite 后端
    let backend = Arc::new(BoxliteBackend::new().await?);
    
    // 创建调度器
    let scheduler = SandboxScheduler::new(backend).await?;
    
    // 配置沙箱
    let config = SandboxConfig::builder()
        .image("alpine:latest")
        .memory_limit_mb(512)
        .cpu_limit(1.0)
        .network_enabled(false)
        .build();
    
    // 创建任务
    let task = TaskSpec::new("security-scan")
        .with_type(TaskType::Cli)
        .with_command(vec!["nmap".to_string(), "-sV".to_string(), "target.com".to_string()])
        .with_sandbox_config(config);
    
    // 执行任务
    let result = scheduler.execute(task).await?;
    
    println!("Exit code: {}", result.exit_code);
    println!("Output: {}", result.stdout);
    
    Ok(())
}
```

### 自动选择最佳后端

```rust
use agent::sandbox::SandboxFactory;

// 自动选择: Boxlite > Docker > Process
let backend = SandboxFactory::create_best().await?;
println!("Using backend: {}", backend.name());
```

### 检查后端可用性

```rust
use agent::sandbox::backends::BoxliteBackend;

if BoxliteBackend::is_available().await {
    println!("Boxlite is available!");
} else {
    println!("Boxlite not available, falling back to Docker/Process");
}
```

## 特性对比

| 特性 | Process | Docker | Boxlite |
|------|---------|--------|---------|
| 隔离级别 | 进程级 | 容器级 | MicroVM |
| 启动时间 | <10ms | 100ms-1s | 100-300ms |
| 内存开销 | 低 | 中 | 低 |
| 系统调用过滤 | rlimit | seccomp + capability | VM 边界 |
| 网络隔离 | 无 | 命名空间 | 虚拟网卡 |
| 文件系统隔离 | 无 | overlayfs | virtiofs |
| 适用场景 | 可信代码 | 一般隔离 | 不可信代码 |

## 故障排除

### 编译错误：Vendored sources not found

**原因**: 未初始化 git submodules

**解决**:
```bash
git submodule update --init --recursive
```

### 编译错误：libkrun not found

**原因**: libkrun 构建失败

**解决**:
1. 检查系统依赖（libssl-dev, pkg-config）
2. 在 Linux 上确保 /dev/kvm 存在
3. 检查构建日志：`cargo build -p boxlite -vv`

### 运行时错误：Failed to initialize Boxlite runtime

**原因**: libkrun 运行时库未找到

**解决**:
1. 确保 libkrun.so 在系统库路径中
2. 设置 LD_LIBRARY_PATH（Linux）或 DYLD_LIBRARY_PATH（macOS）

### 权限错误：Cannot access /dev/kvm

**原因**: 当前用户没有 KVM 访问权限

**解决**:
```bash
# 将当前用户添加到 kvm 组
sudo usermod -a -G kvm $USER

# 重新登录或执行
newgrp kvm
```

## 性能优化

### 镜像预拉取

Boxlite 使用 OCI 镜像，建议预拉取常用镜像：

```rust
let backend = BoxliteBackend::new().await?;
let runtime = backend.runtime();

// 预拉取镜像
runtime.pull_image("alpine:latest").await?;
runtime.pull_image("ubuntu:22.04").await?;
```

### 沙箱预热

对于需要快速响应的场景，可以预热沙箱：

```rust
let scheduler = SandboxScheduler::new(backend).await?;

// 预创建沙箱
for _ in 0..5 {
    let config = SandboxConfig::builder()
        .image("alpine:latest")
        .build();
    scheduler.prewarm(config).await?;
}
```

## 安全建议

1. **优先使用 Boxlite**: 对于不可信的 AI 生成代码，优先使用 Boxlite 后端
2. **资源限制**: 始终设置内存和 CPU 限制
3. **网络隔离**: 默认禁用网络，按需开启
4. **只读根文件系统**: 启用 `read_only_rootfs` 选项
5. **定期清理**: 调用 `backend.cleanup()` 清理残留容器

## 相关链接

- [Boxlite 文档](../../src/boxlite/README.md)
- [libkrun 项目](https://github.com/containers/libkrun)
- [Agent Sandbox 架构](./src/sandbox/mod.rs)