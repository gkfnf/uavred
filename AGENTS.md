# Agent System Architecture

## Overview

UAV Red Team 使用智能 Agent 系统来自主调度和执行渗透测试任务。每个 Agent 都具有特定的能力,可以独立执行任务或协同工作。

## Agent 类型

### 1. Network Scanner Agent
- **能力**: 网络扫描、端口探测、服务识别
- **目标**: 发现网络中的无人机设备
- **输出**: 设备列表、开放端口、运行服务

### 2. Protocol Analyzer Agent
- **能力**: 协议分析、流量解析、漏洞检测
- **支持协议**: MAVLink, DJI, ArduPilot, PX4
- **输出**: 协议弱点、潜在攻击向量

### 3. Firmware Analyzer Agent
- **能力**: 固件提取、静态分析、漏洞搜索
- **检测项**: 硬编码凭证、后门、已知漏洞
- **输出**: 安全问题列表、风险评估

### 4. Exploit Executor Agent
- **能力**: 漏洞利用、权限提升、持久化
- **安全**: 仅在授权测试环境中使用
- **输出**: 利用结果、获取的访问权限

## Agent 调度策略

### 优先级队列
- Critical: 立即执行
- High: 优先执行
- Medium: 按序执行
- Low: 资源充足时执行

### 任务分配
1. 检查 Agent 能力与任务需求匹配
2. 选择空闲且能力匹配的 Agent
3. 分配任务并监控执行状态
4. 处理结果并触发后续任务

### 协同工作
- Network Scanner 发现设备 → Protocol Analyzer 分析协议
- Protocol Analyzer 发现漏洞 → Exploit Executor 尝试利用
- 所有发现汇总到漏洞数据库

## UI 设计原则

遵循高信息密度、简洁优雅的设计风格:

- **布局**: 紧凑但清晰的信息展示
- **颜色**: 使用语义化颜色(成功/警告/错误)
- **图标**: 最小化装饰性图标,仅用于功能性标识
- **字体**: 使用等宽字体展示技术信息
- **动画**: 简洁的状态转换,避免过度动效

## Development Guidelines

### Adding New Agents

1. 定义 Agent 能力类型 (`AgentCapability`)
2. 实现 Agent 执行逻辑
3. 在 Scheduler 中注册
4. 添加 UI 展示组件

### Testing Agents

```bash
# 单元测试
cargo test agent::

# 集成测试
cargo test --test agent_integration
```

## Security Considerations

- 所有 Agent 操作必须记录日志
- 仅在授权环境中执行攻击性测试
- 敏感数据(凭证、密钥)必须加密存储
- 定期审计 Agent 行为

## Landing the Plane (Session Completion)

**When ending a work session**, you MUST complete ALL steps below. Work is NOT complete until `git push` succeeds.

**MANDATORY WORKFLOW:**

1. **File issues for remaining work** - Create issues for anything that needs follow-up
2. **Run quality gates** (if code changed) - Tests, linters, builds
3. **Update issue status** - Close finished work, update in-progress items
4. **PUSH TO REMOTE** - This is MANDATORY:
   ```bash
   git pull --rebase
   bd sync
   git push
   git status  # MUST show "up to date with origin"
   ```
5. **Clean up** - Clear stashes, prune remote branches
6. **Verify** - All changes committed AND pushed
7. **Hand off** - Provide context for next session

**CRITICAL RULES:**
- Work is NOT complete until `git push` succeeds
- NEVER stop before pushing - that leaves work stranded locally
- NEVER say "ready to push when you are" - YOU must push
- If push fails, resolve and retry until it succeeds
