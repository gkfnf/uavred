# Architecture Documentation

## 系统架构

```
┌─────────────────────────────────────────────────────────┐
│                    UAV Red Team                          │
│                  Desktop Application                     │
└─────────────────────────────────────────────────────────┘
                          │
        ┌─────────────────┼─────────────────┐
        │                 │                 │
   ┌────▼────┐      ┌────▼────┐      ┌────▼────┐
   │   UI    │      │  Agent  │      │  Core   │
   │  Layer  │◄────►│ System  │◄────►│  Layer  │
   └─────────┘      └─────────┘      └─────────┘
        │                 │                 │
        │                 │                 │
   ┌────▼─────────────────▼─────────────────▼────┐
   │            Scanner Modules                    │
   │  ┌──────────┐ ┌──────────┐ ┌──────────┐    │
   │  │ Network  │ │ Protocol │ │ Firmware │    │
   │  │ Scanner  │ │ Analyzer │ │ Analyzer │    │
   │  └──────────┘ └──────────┘ └──────────┘    │
   └───────────────────────────────────────────────┘
```

## 模块说明

### UI Layer (ui/)
**职责**: 用户界面展示和交互
- `dashboard.rs`: 主控制面板,显示整体状态
- `agents.rs`: Agent 管理界面
- `results.rs`: 扫描结果展示

**技术栈**:
- gpui: 底层 GUI 框架
- gpui-component: UI 组件库

### Agent System (agent/)
**职责**: 任务调度和执行
- `mod.rs`: Agent 定义和能力类型
- `scheduler.rs`: 任务调度器,管理任务队列
- `executor.rs`: 任务执行器,执行具体任务

**工作流程**:
1. Scheduler 接收任务
2. 根据 Agent 能力匹配任务
3. 分配给空闲 Agent
4. Executor 执行任务
5. 返回结果

### Core Layer (core/)
**职责**: 核心业务逻辑
- `task.rs`: 任务定义和状态管理
- `vuln_db.rs`: 漏洞数据库

**任务类型**:
- NetworkScan: 网络扫描
- ProtocolAnalysis: 协议分析
- FirmwareAnalysis: 固件分析
- Exploit: 漏洞利用

### Scanner Modules (scanner/)
**职责**: 具体扫描实现
- `network.rs`: 网络扫描
  - 端口扫描
  - 服务识别
  - UAV 设备检测
  
- `protocol.rs`: 协议分析
  - MAVLink 解析
  - DJI 协议分析
  - 命令注入测试
  
- `firmware.rs`: 固件分析
  - 固件提取
  - 字符串分析
  - 漏洞检测

## 数据流

```
用户操作
   │
   ▼
Dashboard (UI)
   │
   ▼
Scheduler (Agent System)
   │
   ├──► Task Queue
   │       │
   │       ▼
   │   Executor
   │       │
   │       ▼
   └──► Scanner Modules
           │
           ▼
       Results
           │
           ▼
    Vulnerability DB
           │
           ▼
    Results Panel (UI)
```

## Agent 协同工作流程

```
1. Network Scanner 发现设备
   │
   ▼
   [192.168.1.100:14550] (MAVLink)
   │
   ▼
2. Scheduler 触发 Protocol Analyzer
   │
   ▼
3. Protocol Analyzer 分析 MAVLink
   │
   ▼
   [发现: 未授权命令注入漏洞]
   │
   ▼
4. 记录到 Vulnerability DB
   │
   ▼
5. (可选) Scheduler 触发 Exploit Executor
   │
   ▼
6. 结果展示到 UI
```

## 异步架构

项目使用 Tokio 异步运行时:

```rust
// Task Executor 使用异步执行
pub async fn execute(&self, task: Task) -> Result<TaskResult>

// Scanner 使用异步扫描
pub async fn scan(&self) -> Result<ScanResult>
```

**优势**:
- 高并发性能
- 非阻塞 IO
- 多任务并行执行

## 扩展性设计

### 添加新 Agent 类型
1. 在 `AgentCapability` 枚举中添加新能力
2. 实现对应的扫描逻辑
3. 在 Scheduler 中注册
4. 添加 UI 展示

### 添加新协议支持
1. 在 `scanner/protocol.rs` 中添加协议枚举
2. 实现协议解析逻辑
3. 添加漏洞检测规则
4. 更新 Vulnerability DB

### 添加新 UI 组件
1. 在 `ui/` 下创建新组件文件
2. 使用 gpui-component 构建界面
3. 在主应用中注册
4. 遵循 UI 设计原则

## 安全考虑

- 所有 Agent 操作记录日志
- 敏感数据加密存储
- 仅在授权环境执行
- 权限最小化原则
- 定期安全审计

## 性能优化

- 使用虚拟化列表/表格处理大数据
- 异步并发执行扫描任务
- 任务队列优先级管理
- 结果缓存机制
