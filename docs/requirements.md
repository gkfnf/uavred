# UAV Red Team — Requirements Specification / 需求规格说明

Status / 状态: Draft (uavred-2q4)
Owner / 负责人: warp-agent
Reviewers / 评审人: fk (+ others TBD)
Last updated / 更新时间: 2025-12-30

## 0. References / 参考
- UI mocks / 界面草图: `interface_pic/*.png` (e.g., Dashboard/Assets/Traffic/Vulns/Flows/Devices/Images/Settings)
- Project overview / 项目概述: `README.md`, `PROJECT_SUMMARY.md`, `docs/ARCHITECTURE.md`

## 1. Problem Statement / 问题陈述
CN: 我们要构建一个桌面端 UAV 红队测试平台，使人类专家能够表达**安全测试意图**，平台调度多个 Agent 在**隔离执行环境（虚拟机镜像为主）**中完成任务，并沉淀可复核的输出（发现、证据链、流量、日志、报告），同时保证隔离、审计与可控。
EN: Build a desktop UAV red-team testing platform where a human operator expresses **testing intent**, the system schedules agents to run tasks in **isolated execution environments (VM images first)**, and produces reviewable outputs (findings, evidence chain, traffic, logs, reports) with strong isolation, auditability, and control.

## 2. Goals / 目标
G1 / 目标1: Intent-driven workflow / 意图驱动流程：**Plan → Execute → Observe → Analyze → Report**.
G2 / 目标2: Protocol priority / 协议优先级：v0.1 以 **HTTP/FTP/TELNET 等传统协议**（在无人机生态中常用于图传/管理/服务）为第一优先级；**MAVLink + DJI** 为第二优先级（MAVLink 可优先落地；DJI 先做占位与可插拔框架）。
G3 / 目标3: Traffic workflow / 流量工作流：默认抓包与归档；按需在 UI 中以类似 Caido 的方式查看流量历史，并用 **TrafficQL**（兼容/覆盖 Caido HTTPQL 且可扩展到多协议）筛选与重放。
G4 / 目标4: Isolated execution / 隔离执行：本机运行主程序，由主程序为 Agent 分配/回收隔离环境（VM 镜像/容器），任务完成后按需销毁，避免污染与泄露。
G5 / 目标5: Persistence / 持久化：v0.1 起采用本地 **SQLite** 作为结构化数据存储；pcap/导出文件作为工件（artifact）落盘并在 DB 中索引。
G6 / 目标6: Governance / 治理：敏感动作（exploit/凭证使用/持久化/扫描强度提升等）必须人类审批（支持 “allow all for this session”）。
G7 / 目标7: High-density UI / 高信息密度 UI：紧凑、键盘友好、少装饰图标；以列表/表格/分栏提升信息密度。
G8 / 目标8: Roadmap / 路线：未来支持分布式与 C2（远程 agent、多主机、被控主机转 agent、射频硬件 agent）以及更完整的网络拓扑/攻击链推演（非 v0.1 必达）。

## 3. Non-Goals (v0.1) / 非目标（v0.1）
NG1: Uncontrolled targeting / 非授权目标与互联网范围扫描。
NG2: Fully autonomous offensive operations / 无人监督的全自动攻击闭环。
NG3: Remote multi-host orchestration in v0.1 / v0.1 不强制支持远程多主机（但设计需为未来预留）。
NG4: Serial/USB telemetry as a hard requirement / v0.1 不要求串口（USB/TTY）。

## 4. Assumptions / Constraints / 假设与约束
A1: Authorized environments only / 仅用于授权环境。
A2: Desktop app: Rust + GPUI + gpui-component / 桌面端技术栈如上。
A3: Execution env model / 执行环境：v0.1 本机编排 VM 镜像（或容器），按任务/意图生命周期管理；任务完成可销毁。
A4: Resource constraints / 资源约束：宿主机内存大约 100GB 级别；磁盘 1TB 级别（实际配额策略待定）。
A5: Capture defaults / 默认抓包：pcap 默认开启；UI 只有在用户点选“查看该意图/任务的流量”时才加载并展示。

## 5. Personas & Roles (RBAC) / 角色与权限
R1 Operator / 红队操作员：编排意图、执行、交互验证。
R2 Analyst / 分析员：复核流量/证据、整理报告。
R3 Admin / 管理员：配置镜像/工具链/资源限制。
R4 Auditor / 审计员：审计日志、审批记录与证据链。
R5 Agent / Agent：受限权限执行任务并上报。

## 6. Core Concepts / 核心概念
C1 Workspace / 工作空间：一个测试项目的逻辑容器（资产/任务/运行/流量/发现/证据/报告）。
C2 Target / 目标：IP:port/域名/服务端点/设备/固件文件/（未来）射频设备。
C3 Flow / 工作流：可复用的 DAG（如图中 Flows 页）。
C4 Mission (Intent) / 测试意图：对某个 target 的一次可执行编排（可映射到 Dashboard 看板任务）。
C5 Run / 运行：一次执行实例；产出事件流、工件与发现。
C6 Finding / 发现：可追溯的安全问题（包含 PoC、证据链、状态）。
C7 Artifact / 工件：pcap、导出的流量历史文件、脚本、截图、命令回放、markdown 笔记、ipynb 等。

## 7. UI Information Architecture (from mocks) / 界面信息架构（基于草图）
UI-1 Dashboard: 目标选择 + Mission Control(看板) + Findings(任务内发现) + 任务详情侧栏（Objective + Live Trace）。
UI-2 Assets: 资产与网络拓扑（Zone Z1-Z5 视图）+ 节点详情抽屉。
UI-3 Scan: 扫描入口（与 flows/mission 联动）。
UI-4 Vulns: 漏洞列表 + 详情/PoC + CVE 数据/评分。
UI-5 Traffic: TrafficQL 查询 + 捕获状态 + 流量表格 + Request/Response + Actions（Replay/Fuzz/Export）。
UI-6 Flows: 工作流模板树 + DAG 画布 + 运行/模拟 + 指标（并发/耗时/成功率/次数）。
UI-7 Images: 容器/VM 镜像 & agent 执行环境管理 + 运行中资源视图。
UI-8 Devices: 硬件设备（USRP/HackRF/RTL-SDR 等）管理与任务绑定（v0.1 可先做“资产/占位 + 基础监控”）。
UI-9 Settings: 全局设置（审批、工具链、存储、导出、键位等）。

## 8. Functional Requirements / 功能需求

### 8.1 Orchestration & Mission Control / 编排与意图看板
FR-ORCH-001 (P0) Plan/Flow editor (DAG) / 工作流 DAG 编辑与保存。
FR-ORCH-002 (P0) Mission kanban bound to a target / 针对目标的 Mission 看板（ToDo/InProgress/Done）。
FR-ORCH-003 (P0) Start/pause/cancel run / 运行控制（开始/暂停/取消）。
FR-ORCH-004 (P0) Task details side panel / 任务详情侧栏（Objective + Agent trace）。
FR-ORCH-005 (P1) Simulation mode / 工作流模拟执行（不产生高风险动作）。

### 8.2 Agent System / Agent 系统
FR-AGENT-001 (P0) Agent registry + heartbeat / Agent 注册、心跳、健康。
FR-AGENT-002 (P0) Capability-based matching / 按能力与环境匹配任务。
FR-AGENT-003 (P0) Lifecycle + retries / 生命周期与重试策略。
FR-AGENT-004 (P0) Structured event stream / 结构化事件流（History/Thought/Plan/Tool/Result 等）。

### 8.3 Traffic & Proxy Integration (Caido + TrafficQL) / 流量与代理集成
FR-TRAF-001 (P0) Default capture + pcap persistence / 默认抓包并生成可索引 pcap。
FR-TRAF-002 (P0) Caido in isolated env + workspace binding / 在隔离环境中启动 Caido，并与 workspace 绑定。
FR-TRAF-003 (P0) Agent can query/export Caido history / Agent 能使用 Caido 语法（HTTPQL）过滤/导出与本任务相关的流量历史文件并归档。
FR-TRAF-004 (P0) UI can import exported traffic history / UI 可解析导出的流量历史文件并以类似 Caido 的方式展示（table + request/response + actions）。
FR-TRAF-005 (P0) TrafficQL filter language / 支持 TrafficQL（覆盖/兼容 HTTPQL，且可扩展到多协议），用于筛选、聚合、定位异常。
FR-TRAF-006 (P1) Replay & fuzz / 对 HTTP 流量支持 Replay、Fuzz（最小闭环）。
FR-TRAF-007 (P1) Anomaly tagging / 异常检测与标注（与 findings 关联）。

### 8.4 Vulnerability & Findings / 漏洞与发现
FR-VULN-001 (P0) Findings list + status / 发现列表与状态（new/validating/confirmed）。
FR-VULN-002 (P0) PoC object + editor / PoC 展示与编辑（可引用 markdown/ipynb）。
FR-VULN-003 (P0) Link finding ↔ asset ↔ traffic ↔ artifacts / 发现与资产/流量/工件的关联。
FR-VULN-004 (P1) CVE enrichment panel / CVE 信息面板与 CVSS 评分。

### 8.5 Asset & Topology / 资产与拓扑
FR-ASSET-001 (P0) Asset inventory + tagging / 资产清单、标签、风险汇总。
FR-ASSET-002 (P0) Topology graph view / 拓扑图视图（节点/边、风险/漏洞计数、详情抽屉）。
FR-ASSET-003 (P1) Zone model (Z1-Z5) / 支持 Z1~Z5 分区视图与边界展示。
FR-ASSET-004 (P2) Attack path assist / 攻击链路辅助分析（与 C2/分布式联动）。

### 8.6 Protocol Coverage / 协议覆盖（v0.1 优先级）
FR-PROTO-HTTP-001 (P0) HTTP recon + vuln checks / HTTP 识别、路径发现与漏洞检测（可由 agent 驱动）。
FR-PROTO-FTP-001 (P0) FTP service detection / FTP 服务识别与基础弱点检查。
FR-PROTO-TELNET-001 (P0) Telnet detection + auth checks / Telnet 探测与鉴权弱点检查。
FR-PROTO-MAVLINK-001 (P1) MAVLink capture/parse module / MAVLink 抓取/解析模块（先集成开源能力）。
FR-PROTO-DJI-001 (P1) DJI protocol placeholder / DJI 协议能力占位（可插拔接口 + 未来加密处理）。

### 8.7 Execution Environment (VM-first) / 执行环境（虚拟机优先）
FR-ENV-001 (P0) VM image lifecycle / 虚拟机镜像生命周期：创建、启动、分配给 agent、任务结束回收/销毁。
FR-ENV-002 (P0) Toolchain spec / 工具链规范：版本、可用性上报。
FR-ENV-003 (P0) Resource limits / 资源限制（CPU/内存/磁盘/时长）。
FR-ENV-004 (P1) Images page resource view / Images 页展示运行中资源、端口暴露、任务绑定。

### 8.8 Persistence (SQLite) & Evidence Chain / 持久化与证据链
FR-DATA-001 (P0) SQLite schema for core entities / SQLite 存储：workspace/asset/mission/flow/run/task/event/artifact/finding/approval。
FR-DATA-002 (P0) Artifact indexing / 工件索引（路径、hash、来源任务、生成时间）。
FR-DATA-003 (P1) Export bundle / 导出 bundle（markdown 报告 + 工件）。

### 8.9 Security & Approvals / 安全与审批
FR-SEC-001 (P0) Approval prompts for sensitive actions / exploit/凭证/持久化/扫描强度提升等必须审批。
FR-SEC-002 (P0) Allow-all-for-session / 支持 “本会话允许全部” 的审批模式（可撤销）。
FR-SEC-003 (P0) Audit log / 审计日志（谁在何时批准/执行了什么）。

### 8.10 Reporting / 报告
FR-REP-001 (P0) Markdown-first reporting / 报告优先 Markdown。
FR-REP-002 (P0) Agent notes as structured markdown / Agent 执行笔记为结构化 markdown，并在 UI 中格式化展示。
FR-REP-003 (P1) PoC as notebooks / PoC 可引用/展示 ipynb cell（先定义存储与渲染协议）。

### 8.11 Roadmap: Distributed/C2 / 路线：分布式与 C2（非 v0.1）
FR-C2-001 (P2) Remote agents / 远程 agent（多主机）。
FR-C2-002 (P2) Implant-to-agent / 被控主机变为受控 agent。
FR-C2-003 (P2) RF hardware agents / USRP 等射频硬件 agent。

## 9. Non-Functional Requirements / 非功能需求
NFR-SEC-1 Secrets never logged / 敏感信息不落日志明文。
NFR-UX-1 High information density / 高信息密度、键盘优先。
NFR-REL-1 Crash-safe persistence / 崩溃恢复：run/task/event 不应因崩溃丢失索引（可接受部分实时 UI 延迟）。
NFR-PERF-1 On-demand loading for large traffic / 大流量按需加载与分页。
NFR-PORT-1 Cross-platform target / 跨平台（功能分期）。

## 10. User Stories / 用户故事（节选）
US-01 作为操作员，我能为指定 target 创建 Mission 看板并拆分任务。
US-02 作为操作员，我能在任务详情中看到 Agent 的 Live Trace（history/thought/plan）。
US-03 作为分析员，我能用 TrafficQL 筛选某个任务产生的流量并导出/归档。
US-04 作为操作员，我能对 HTTP 流量 Replay/Fuzz 来验证 PoC。
US-05 作为审计员，我能看到 exploit/凭证使用等敏感动作的审批记录。
US-06 作为管理者，我能管理 VM 镜像与工具链版本，并查看资源占用。
US-07 作为分析员，我能在 Vulns 页查看 PoC、证据链与关联流量。
US-08 作为操作员，我能在 Assets 页查看 Z1-Z5 网络边界与拓扑。

## 11. Acceptance Criteria (high-level) / 验收标准（高层）
AC-1 能创建 workspace/target/mission/flow，并在 DB 中可追溯。
AC-2 pcap 默认记录；UI 可按需加载并展示对应任务的流量。
AC-3 支持 Caido 绑定 workspace；agent 能过滤并导出任务相关流量历史文件；UI 能解析并以三栏（table/request/response/actions）展示。
AC-4 敏感动作需审批，支持 allow-all-for-session，且审计日志可查询。
AC-5 报告可导出为 Markdown（含链接到工件）。

## 12. Open Questions / 待确认
Q1 Caido export format / Caido 导出流量历史的格式：JSON/SQLite/HTTP Archive(HAR)/自定义？（我们需要稳定可解析格式）
Q2 TrafficQL spec / TrafficQL 语法边界：是否需要聚合/时间窗口/跨协议字段？
Q3 Workspace quotas / Workspace 的“最大 GB”是指：DB+工件磁盘配额（推荐）还是运行时 VM 资源？（运行时资源计划由 scheduler 按需销毁）
Q4 v0.1 MVP boundary for C2 / v0.1 是否仅做“能力占位+接口”，还是需要最小可用的分布式通道？
