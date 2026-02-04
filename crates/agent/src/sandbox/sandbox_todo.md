# Agent Sandbox 模块开发任务清单

## 概述

本清单详细列出了 Agent 沙箱调度模块的开发任务，每个任务都经过细化，确保可以在 1-2 次 Kimi 会话中完成。

---

## 第一阶段：核心 Trait 和类型定义 ✅

### Task 1.1: 定义核心 Trait 接口 ✅
**文件**: `crates/agent/src/sandbox/traits.rs`  
**状态**: ✅ 已完成  
**描述**: 定义 `SandboxBackend` 和 `SandboxInstance` trait  
**验证**: 编译通过，trait 定义完整

### Task 1.2: 定义配置类型 ✅
**文件**: `crates/agent/src/sandbox/config.rs`  
**状态**: ✅ 已完成  
**描述**: 定义 `SandboxConfig`, `ResourceLimits`, `NetworkPolicy`, `SecurityOptions`  
**验证**: 包含 Builder 模式，所有配置项可序列化

### Task 1.3: 定义实例类型 ✅
**文件**: `crates/agent/src/sandbox/instance.rs`  
**状态**: ✅ 已完成  
**描述**: 定义 `SandboxInstance`, `Execution`, `SandboxHandle`  
**验证**: 状态机转换正确，资源追踪完整

---

## 第二阶段：Backend 实现

### Task 2.1: Process Backend 完整实现 🔄
**文件**: `crates/agent/src/sandbox/backends/process.rs`  
**状态**: 🔄 骨架完成，需实现  
**依赖**: Task 1.1, Task 1.2

#### 2.1.1 实现进程启动和监控
```rust
// 需要实现:
- 使用 tokio::process::Command 启动进程
- 设置工作目录和环境变量
- 应用 rlimit 资源限制（使用 unsafe pre_exec）
- 捕获 stdout/stderr
```
**验收标准**:
- [ ] 可以启动简单命令（如 `echo hello`）
- [ ] 可以捕获 stdout 输出
- [ ] 可以正确获取 exit code
- [ ] 单测通过

#### 2.1.2 实现资源限制
```rust
// 需要实现:
- setrlimit for RLIMIT_AS (内存限制)
- setrlimit for RLIMIT_CPU (CPU 时间限制)
- setrlimit for RLIMIT_NOFILE (文件描述符限制)
- setrlimit for RLIMIT_NPROC (进程数限制)
```
**验收标准**:
- [ ] 内存限制生效（超出时进程被 kill）
- [ ] CPU 时间限制生效
- [ ] 单测通过

#### 2.1.3 实现文件操作
```rust
// 需要实现:
- copy_in: 使用 tokio::fs::copy
- copy_out: 使用 tokio::fs::copy
- 处理目录递归
```
**验收标准**:
- [ ] 可以复制文件到工作目录
- [ ] 可以复制文件从工作目录
- [ ] 单测通过

---

### Task 2.2: Docker Backend 完整实现 ⏳
**文件**: `crates/agent/src/sandbox/backends/docker.rs`  
**状态**: ⏳ 骨架完成  
**依赖**: Task 2.1

#### 2.2.1 集成 bollard Docker 客户端
```rust
// 需要实现:
- 使用 bollard 库连接 Docker daemon
- 实现版本检查和健康检查
- 错误处理（Docker 未运行时友好报错）
```
**新增依赖**:
```toml
bollard = "0.18"
```
**验收标准**:
- [ ] 可以连接到本地 Docker daemon
- [ ] Docker 不可用时返回清晰错误
- [ ] 单测通过（使用 mock 或跳过）

#### 2.2.2 实现容器生命周期管理
```rust
// 需要实现:
- create_container: 使用 Config 创建容器
- start_container: 启动容器
- stop_container: 优雅停止（带超时）
- remove_container: 删除容器
- 正确处理 ContainerCreateOptions
```
**验收标准**:
- [ ] 可以创建并启动容器
- [ ] 可以停止并删除容器
- [ ] 单测通过

#### 2.2.3 实现容器执行和输出流
```rust
// 需要实现:
- exec_container: 使用 Docker Exec API
- 流式获取 stdout/stderr（使用 attach_stream）
- 正确处理 TTY 模式
```
**验收标准**:
- [ ] 可以在运行中的容器执行命令
- [ ] 实时获取输出流
- [ ] 单测通过

#### 2.2.4 实现资源限制和网络配置
```rust
// 需要实现:
- 内存限制（--memory）
- CPU 限制（--cpus）
- PID 限制（--pids-limit）
- 网络模式（--network none/bridge）
- 端口映射（如有需要）
```
**验收标准**:
- [ ] 内存限制生效
- [ ] 网络隔离生效
- [ ] 单测通过

#### 2.2.5 实现镜像管理
```rust
// 需要实现:
- pull_image: 拉取镜像并显示进度
- image_exists: 检查本地镜像
- 支持镜像仓库认证（如果需要）
```
**验收标准**:
- [ ] 可以拉取镜像
- [ ] 重复拉取使用缓存
- [ ] 单测通过

---

### Task 2.3: Boxlite Backend 完整实现 ✅
**文件**: `crates/agent/src/sandbox/backends/boxlite.rs`  
**状态**: ✅ 已完成（特性标志控制）  
**依赖**: Task 2.2

#### 2.3.1 特性标志集成
```rust
// 通过 feature flag 条件编译
#[cfg(feature = "boxlite-backend")]
use boxlite::{BoxliteRuntime, LiteBox, ...};
```
**验收标准**:
- [x] 默认编译不包含 boxlite
- [x] 启用特性标志后集成 boxlite crate
- [x] 无特性时返回清晰的错误信息

#### 2.3.2 配置转换
```rust
// SandboxConfig -> BoxOptions
// ResourceLimits -> boxlite ResourceLimits
// SecurityOptions -> boxlite SecurityOptions
```
**验收标准**:
- [x] 配置正确转换
- [x] 资源限制生效
- [x] 安全选项传递

#### 2.3.3 完整生命周期管理
```rust
// 实现 SandboxInstance trait for BoxliteInstance
// - start/stop/kill
// - exec with streaming output
// - copy_in/copy_out
// - resource_usage
```
**验收标准**:
- [x] MicroVM 生命周期管理
- [x] 流式输出支持
- [x] 文件复制功能
- [x] 单测通过

#### 2.3.1 集成 Boxlite Runtime
```rust
// 需要实现:
- 初始化 BoxliteRuntime
- 配置 home_dir 和 options
- 健康检查（libkrun 可用性检查）
```
**新增依赖**:
```toml
boxlite = { path = "../../../src/boxlite/boxlite" }
```
**验收标准**:
- [ ] 可以初始化 BoxliteRuntime
- [ ] libkrun 不可用时返回清晰错误
- [ ] 单测通过

#### 2.3.2 实现 MicroVM 创建和启动
```rust
// 需要实现:
- 将 SandboxConfig 转换为 BoxOptions
- 创建 LiteBox
- 启动 MicroVM（调用 litebox.start()）
- 等待 guest agent 就绪
```
**验收标准**:
- [ ] 可以创建 LiteBox
- [ ] 可以启动 MicroVM
- [ ] 单测通过

#### 2.3.3 实现 MicroVM 执行和通信
```rust
// 需要实现:
- 使用 litebox.exec() 执行命令
- 流式获取 stdout/stderr
- 使用 vsock/gRPC 通信
```
**验收标准**:
- [ ] 可以在 MicroVM 中执行命令
- [ ] 实时获取输出流
- [ ] 单测通过

#### 2.3.4 实现文件系统操作
```rust
// 需要实现:
- 使用 litebox.copy_into()
- 使用 litebox.copy_out()
- 处理 virtiofs 共享目录
```
**验收标准**:
- [ ] 可以复制文件到 MicroVM
- [ ] 可以复制文件从 MicroVM
- [ ] 单测通过

#### 2.3.5 实现资源监控
```rust
// 需要实现:
- 使用 litebox.metrics() 获取资源使用
- CPU、内存、磁盘 I/O 统计
```
**验收标准**:
- [ ] 可以获取 MicroVM 资源使用
- [ ] 单测通过

---

## 第三阶段：Scheduler 实现

### Task 3.1: SandboxRegistry 实现 🔄
**文件**: `crates/agent/src/sandbox/mod.rs` (SandboxRegistry)  
**状态**: 🔄 部分完成  
**依赖**: Task 1.3

#### 3.1.1 完善 Registry 功能
```rust
// 需要实现:
- 并发安全的实例注册/注销
- 按状态过滤查询
- 自动清理 orphaned 实例
- 优雅关闭所有实例
```
**验收标准**:
- [ ] 可以并发注册多个实例
- [ ] 可以查询运行中的实例
- [ ] 关闭时优雅停止所有实例
- [ ] 单测通过

---

### Task 3.2: SandboxScheduler 核心功能 ⏳
**文件**: `crates/agent/src/sandbox/scheduler.rs`  
**状态**: ⏳ 骨架完成  
**依赖**: Task 3.1, Task 2.1

#### 3.2.1 实现任务队列和优先级
```rust
// 需要实现:
- 优先队列（按 priority 排序）
- 任务去重（相同任务 ID）
- 队列大小限制和反压
```
**验收标准**:
- [ ] 高优先级任务先执行
- [ ] 队列满时返回错误
- [ ] 单测通过

#### 3.2.2 实现 Sandbox 池化管理
```rust
// 需要实现:
- Sandbox 复用（warm pool）
- 池大小限制
- 过期清理（TTL）
- 健康检查（不健康实例移出池）
```
**验收标准**:
- [ ] Sandbox 可以复用
- [ ] 池大小可配置
- [ ] 不健康实例被移除
- [ ] 单测通过

#### 3.2.3 实现任务执行和重试
```rust
// 需要实现:
- 异步任务执行
- 失败重试（指数退避）
- 超时处理
- 取消任务
```
**验收标准**:
- [ ] 任务可以异步执行
- [ ] 失败时自动重试
- [ ] 超时后强制终止
- [ ] 可以取消正在执行的任务
- [ ] 单测通过

#### 3.2.4 实现结果收集和统计
```rust
// 需要实现:
- 结果通道（mpsc）
- 执行统计（成功率、平均耗时）
- 资源使用统计
- 指标导出（prometheus 格式）
```
**验收标准**:
- [ ] 可以接收任务结果
- [ ] 统计信息准确
- [ ] 单测通过

---

## 第四阶段：Execution Drivers

### Task 4.1: CliDriver 完整实现 🔄
**文件**: `crates/agent/src/sandbox/drivers/cli.rs`  
**状态**: 🔄 骨架完成  
**依赖**: Task 3.2

#### 4.1.1 完善 CLI 驱动
```rust
// 需要实现:
- 命令构建和验证
- 环境变量传递
- 超时处理
- 结果格式化
```
**验收标准**:
- [ ] 可以执行任意 shell 命令
- [ ] 环境变量正确传递
- [ ] 单测通过

---

### Task 4.2: McpDriver 完整实现 ⏳
**文件**: `crates/agent/src/sandbox/drivers/mcp.rs`  
**状态**: ⏳ 骨架完成  
**依赖**: Task 4.1

#### 4.2.1 实现 MCP 配置生成
```rust
// 需要实现:
- 生成 mcp.json 配置文件
- 工具权限控制（允许/拒绝列表）
- MCP server 启动和连接
```
**验收标准**:
- [ ] 可以生成 MCP 配置
- [ ] 工具权限控制生效
- [ ] 单测通过

#### 4.2.2 实现 Claude Code 集成
```rust
// 需要实现:
- 构建 claude 命令行参数
- 传递系统提示词
- 处理 Claude Code 输出
```
**验收标准**:
- [ ] 可以启动 Claude Code
- [ ] 任务描述正确传递
- [ ] 单测通过

---

### Task 4.3: MetaToolDriver 完整实现 ⏳
**文件**: `crates/agent/src/sandbox/drivers/meta_tool.rs`  
**状态**: ⏳ 骨架完成  
**依赖**: Task 4.1

#### 4.3.1 实现 Python 代码包装
```rust
// 需要实现:
- 生成安全的 Python wrapper
- 工具集导入（browser, terminal, note, proxy）
- 结果序列化（JSON）
- 异常处理和 traceback
```
**验收标准**:
- [ ] Python 代码正确包装
- [ ] 工具集可用
- [ ] 异常时输出 traceback
- [ ] 单测通过

#### 4.3.2 实现工具集集成
```rust
// 需要实现:
- Browser 工具（网页访问、截图）
- Terminal 工具（命令执行）
- Note 工具（笔记记录）
- Proxy 工具（流量代理）
```
**验收标准**:
- [ ] 所有工具可用
- [ ] 工具调用正确记录
- [ ] 单测通过

---

## 第五阶段：集成和测试

### Task 5.1: 模块集成测试 ⏳
**文件**: `crates/agent/tests/`  
**状态**: ⏳ 未开始  
**依赖**: Task 2.1, Task 3.2

#### 5.1.1 编写 Backend 集成测试
```rust
// 测试内容:
- Process Backend 端到端测试
- Docker Backend 端到端测试（如有 Docker）
- Backend 自动选择逻辑
```
**验收标准**:
- [ ] Process Backend 测试通过
- [ ] Docker Backend 测试通过（有 Docker 时）
- [ ] 自动选择逻辑正确

#### 5.1.2 编写 Scheduler 集成测试
```rust
// 测试内容:
- 任务提交和执行
- 并发任务处理
- 资源限制生效
- 优雅关闭
```
**验收标准**:
- [ ] 任务可以完整执行
- [ ] 并发任务不互相干扰
- [ ] 资源限制生效
- [ ] 关闭时清理资源

---

### Task 5.2: 示例和文档 ⏳
**文件**: `crates/agent/examples/`, `crates/agent/README.md`  
**状态**: ⏳ 未开始  
**依赖**: Task 5.1

#### 5.2.1 编写使用示例
```rust
// 示例内容:
- 基本使用示例（执行 shell 命令）
- MCP 任务示例（Claude Code）
- Meta-tool 示例（Python 代码执行）
- 自定义配置示例
```
**验收标准**:
- [ ] 示例可以编译运行
- [ ] 输出符合预期

#### 5.2.2 编写模块文档
```markdown
// 文档内容:
- 架构概述
- 快速开始
- Backend 选择指南
- 安全配置最佳实践
- API 文档
```
**验收标准**:
- [ ] 文档完整
- [ ] 代码示例正确
- [ ] cargo doc 生成无警告

---

## 第六阶段：性能优化和高级特性

### Task 6.1: 性能优化 ⏳
**状态**: ⏳ 未开始  
**依赖**: Task 5.2

#### 6.1.1 实现 Sandbox 预热
```rust
// 优化内容:
- 启动时预创建 Sandbox
- 保持最小可用池大小
- 快速分配（避免冷启动）
```
**验收标准**:
- [ ] 任务启动延迟 < 100ms（预热后）
- [ ] 资源使用合理

#### 6.1.2 实现镜像缓存策略
```rust
// 优化内容:
- 镜像预拉取
- 分层缓存
- 本地镜像优先
```
**验收标准**:
- [ ] 镜像拉取时间优化
- [ ] 缓存命中率 > 80%

---

### Task 6.2: 高级特性 ⏳
**状态**: ⏳ 未开始  
**依赖**: Task 6.1

#### 6.2.1 实现 Checkpoint/Restore
```rust
// 功能内容:
- Sandbox 状态快照
- 快速恢复
- 跨机器迁移（可选）
```
**验收标准**:
- [ ] 可以创建快照
- [ ] 可以从快照恢复
- [ ] 恢复后状态正确

#### 6.2.2 实现网络隔离策略
```rust
// 功能内容:
- 细粒度网络策略（端口、IP 白名单）
- 代理服务器集成
- DNS 过滤
```
**验收标准**:
- [ ] 网络策略生效
- [ ] 代理正确转发

---

## 附录：开发顺序建议

### 最小可用版本 (MVP)
按以下顺序完成可以尽快得到可用的沙箱功能：

1. ✅ Task 1.1-1.3: 核心类型定义
2. 🔄 Task 2.1: Process Backend（快速可用）
3. ⏳ Task 3.1: SandboxRegistry
4. ⏳ Task 3.2.1-3.2.2: Scheduler 核心（队列、池化）
5. 🔄 Task 4.1: CliDriver
6. ⏳ Task 5.1.1: 基础集成测试

**预期时间**: 1-2 天

### 生产可用版本
继续以下任务达到生产级别：

7. ⏳ Task 2.2: Docker Backend
8. ⏳ Task 2.3: Boxlite Backend
9. ⏳ Task 3.2.3-3.2.4: Scheduler 完整功能
10. ⏳ Task 4.2-4.3: MCP 和 MetaTool Drivers
11. ⏳ Task 5.1.2-5.2: 完整测试和文档

**预期时间**: 3-5 天

### 高级版本
最后完成高级特性：

12. ⏳ Task 6.1: 性能优化
13. ⏳ Task 6.2: 高级特性

**预期时间**: 2-3 天

---

## 附录：测试策略

### 单元测试
每个模块的单元测试覆盖率目标：
- config: 90%+
- instance: 85%+
- scheduler: 80%+
- backends: 70%+（依赖外部服务）
- drivers: 80%+

### 集成测试
- 使用 `#[ignore]` 标记需要外部服务的测试
- CI 中运行 Process Backend 测试
- 本地开发可选运行 Docker/Boxlite 测试

### 性能测试
- 任务启动延迟 < 500ms (cold), < 100ms (warm)
- 并发 Sandbox 数量 > 50
- 内存占用 < 100MB (scheduler)
