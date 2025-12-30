# BD 工作流指南

本项目使用 [Beads (bd)](https://github.com/steveyegge/beads) 进行任务管理和协作。

## 快速开始

### 查看当前任务
```bash
# 查看所有 issues
bd list --pretty

# 查看可以开始的任务（无阻塞）
bd ready

# 查看被阻塞的任务
bd blocked

# 查看项目概览
bd status
```

### 创建新任务
```bash
# 基础创建
bd create "任务标题"

# 完整参数
bd create "实现 XXX 功能" \
  --type feature \
  --priority 0 \
  --assignee yourname \
  --description "详细描述" \
  --labels "agent,core"

# 任务类型: bug, feature, task, epic, chore
# 优先级: 0-4 (0 最高)
```

### 更新任务
```bash
# 开始工作
bd update uavred-xxx --status in_progress --assignee yourname

# 添加注释
bd comments uavred-xxx add "完成了基础结构"

# 关闭任务
bd close uavred-xxx
```

### 管理依赖
```bash
# 添加依赖（B 阻塞 A）
bd dep add uavred-A uavred-B

# 查看依赖树
bd dep tree uavred-xxx

# 检测循环依赖
bd dep cycles
```

### 同步到 Git
```bash
# 手动同步（通常不需要，会自动同步）
bd sync

# 推送到 GitHub
git push origin master
```

## Agent 工作流

### Agent 状态报告
```bash
# 报告 Agent 状态
bd agent state scanner-agent working
bd agent state scanner-agent stuck
bd agent state scanner-agent done

# 心跳
bd agent heartbeat scanner-agent

# 查看 Agent 状态
bd agent show scanner-agent
```

### Agent 协同
1. Scanner Agent 发现设备 → 创建分析任务
2. Protocol Analyzer 认领任务 → 更新状态为 in_progress
3. 发现漏洞 → 创建新的验证任务
4. Exploit Agent 认领验证任务

## 当前任务

- **uavred-y24**: 实现网络扫描 Agent (P0) ✅ Ready
- **uavred-m0j**: 实现 MAVLink 协议分析 Agent (P0) ✅ Ready
- **uavred-0u1**: 实现 Agent 调度系统 (P0) ✅ Ready
- **uavred-c80**: 设计漏洞数据库架构 (P1) ✅ Ready
- **uavred-4nf**: 构建 GPUI 基础 UI 框架 (P1) ⏳ Blocked

依赖关系：
- UI 框架依赖于网络扫描和调度系统

## 最佳实践

1. **始终使用 `bd ready`** 查看下一步工作
2. **小而频繁的 issues** 比大任务更易管理
3. **添加依赖关系** 让系统自动管理优先级
4. **Agent 自我报告状态** 保持可观测性
5. **定期 `bd doctor`** 检查系统健康
6. **使用标签** 方便过滤和分类

## 常用命令速查

```bash
bd ready              # 查看可做的任务
bd list --pretty      # 漂亮的列表显示
bd create "任务"      # 创建任务
bd update ID --status in_progress  # 开始任务
bd close ID           # 完成任务
bd search "关键词"    # 搜索任务
bd dep add A B        # 添加依赖
bd sync               # 同步到 Git
```

## 参考链接

- [Beads GitHub](https://github.com/steveyegge/beads)
- [完整命令列表](./docs/BD_COMMANDS.md)
- [项目架构](./AGENTS.md)
