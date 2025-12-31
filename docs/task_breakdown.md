# UAV Red Team — Task Breakdown (Agent-sized) / 任务拆分（可执行最小颗粒度）

Status / 状态: Draft (derived from uavred-2q4)
Source / 来源: `docs/requirements.md` + `interface_pic/*.png`
Last updated / 更新时间: 2025-12-30

## 0. Conventions / 约定
- Each task is sized to be independently executable and reviewable (“agent-sized”).
- 每个任务必须具备：清晰输入/输出、依赖、>=3 条验收标准。
- Labels / 标签建议：`ui`, `core`, `db`, `traffic`, `protocol`, `agent`, `vm`, `security`, `assets`, `devices`, `reporting`.

## 1. Milestone Focus (v0.1) / v0.1 里程碑重点
P0: HTTP/FTP/TELNET + Traffic (Caido + TrafficQL) + Mission/Flows + SQLite + Approvals + Reporting.
P1: MAVLink module + DJI placeholder + Zones (Z1-Z5) + CVE enrichment.
P2 (roadmap): C2/distributed agents + RF hardware agents.

## 2. Task List / 任务列表

### A. Project foundation / 工程基础
A-01 (P0) Define domain models (workspace/asset/mission/flow/run/task/event/artifact/finding/approval)
Owner: core-agent
Deps: none
Acceptance:
- Data model structs defined with clear ownership and IDs.
- Events support categories (history/thought/plan/tool/result).
- Artifacts can reference file paths + hashes.

A-02 (P0) Define severity + status enums (findings, tasks, approvals)
Owner: core-agent
Deps: A-01
Acceptance:
- Finding status supports: new/validating/confirmed/false_positive.
- Task status supports: todo/in_progress/done/failed/canceled.
- Approval state supports: pending/approved/denied + “allow all for session” flag.

### B. SQLite persistence / SQLite 持久化
B-01 (P0) SQLite schema + migrations for core entities
Owner: db-agent
Deps: A-01,A-02
Acceptance:
- Migrations create tables for all core entities.
- Foreign keys for relations (finding→asset/run/task, artifact→task).
- Basic CRUD is covered by unit tests.

B-02 (P0) Artifact store layout + hashing
Owner: db-agent
Deps: B-01
Acceptance:
- Deterministic on-disk layout per workspace/run.
- Hash (sha256) computed and stored for each artifact.
- Missing file references are detected and surfaced in UI.

B-03 (P0) Event ingestion API (append-only)
Owner: core-agent
Deps: B-01
Acceptance:
- Events appended without rewriting history.
- Query by run_id + task_id + time range.
- Tested with large event counts (basic performance sanity).

### C. Execution environments (VM-first) / 执行环境（VM 优先）
C-01 (P0) Execution environment abstraction (VM/Container) + lifecycle interface
Owner: vm-agent
Deps: A-01
Acceptance:
- Interface supports: create/start/stop/destroy + status + logs.
- Can attach metadata: agent_id, task_id, workspace_id.
- Clear error taxonomy for start failures.

C-02 (P0) Local VM runner (v0.1)
Owner: vm-agent
Deps: C-01
Acceptance:
- Launch a minimal VM image and run a no-op command.
- Enforce CPU/memory/time limits.
- Cleanup destroys environment after task completion.

C-03 (P0) Toolchain availability report from env
Owner: vm-agent
Deps: C-01,C-02
Acceptance:
- Env reports installed tools/versions.
- Scheduler can reject tasks when tools missing.
- Report persisted to SQLite per run.

### D. Agent scheduler & runtime / 调度与运行时
D-01 (P0) Agent registry + heartbeat + health
Owner: agent-core
Deps: A-01
Acceptance:
- Agents register with capabilities.
- Heartbeat updates last_seen; stale agents marked unhealthy.
- UI can show active agents count.

D-02 (P0) Scheduler: capability-based task assignment
Owner: agent-core
Deps: D-01,C-01,B-01
Acceptance:
- Tasks matched to agent capabilities and env constraints.
- Retries with backoff; clear failure reason.
- Deterministic state transitions stored in DB.

D-03 (P0) Approval gate integration
Owner: security-agent
Deps: D-02,B-01
Acceptance:
- Sensitive tasks require approval before execution.
- UI can approve/deny; “allow all for session” works.
- Audit events recorded for each decision.

### E. Flows (DAG) & Mission Control (Kanban) / 工作流与看板
E-01 (P0) Flow definition format + validator
Owner: core-agent
Deps: A-01
Acceptance:
- DAG supports nodes, deps, params, retries.
- Validator catches cycles and missing targets.
- Stored and loaded from DB.

E-02 (P0) Flows UI: list + search + create/edit basic metadata
Owner: ui-agent
Deps: E-01
Acceptance:
- Matches mock `interface_pic/WorkFlows.png` left panel behavior.
- Can create a flow and persist.
- Displays metrics placeholders (nodes/parallelism/etc.).

E-03 (P0) Flows UI: DAG canvas rendering (read-only first)
Owner: ui-agent
Deps: E-02
Acceptance:
- Render nodes/edges with status color semantics.
- Highlight critical path (placeholder logic acceptable).
- Click node opens node details.

E-04 (P0) Mission Control Kanban (Dashboard)
Owner: ui-agent
Deps: B-01,E-01
Acceptance:
- Kanban columns: ToDo/In Progress/Done.
- Cards show type + priority tags (as mock).
- Drag/drop updates status and persists.

E-05 (P0) Task details side panel (Objective + Live Trace)
Owner: ui-agent
Deps: B-03
Acceptance:
- Shows mission objective text.
- Shows live trace stream grouped by event type.
- Supports “view logs” and copy actions.

### F. Traffic (Caido + TrafficQL) / 流量系统
F-01 (P0) Define traffic artifact format (export contract)
Owner: traffic-agent
Deps: B-01
Acceptance:
- Choose canonical internal format (JSONL or SQLite) for imported traffic.
- Document mapping from Caido export → internal format.
- Provide versioning for schema evolution.

F-02 (P0) TrafficQL parser + evaluator (HTTP subset first)
Owner: traffic-agent
Deps: F-01
Acceptance:
- Parse filters like `method:POST AND path~"/api/v1/telemetry"`.
- Apply filter to traffic rows and return IDs.
- Unit tests cover parsing + matching.

F-03 (P0) Traffic UI: table + request/response panels
Owner: ui-agent
Deps: F-01,F-02
Acceptance:
- Matches mock `interface_pic/Traffics.png` layout.
- Selecting a row shows request/response.
- Basic paging for large lists.

F-04 (P0) Caido environment binding + export by task
Owner: traffic-agent
Deps: C-01,C-02,D-02,F-01
Acceptance:
- Agent can instruct Caido to export traffic for a given time range or tag.
- Exported file stored as artifact and indexed.
- UI can open the exported artifact.

F-05 (P1) Replay action for HTTP requests
Owner: traffic-agent
Deps: F-03
Acceptance:
- Replay selected request and show response.
- Record replay result as artifact.
- Respect approvals policy if replay can be “high risk” (configurable).

F-06 (P1) Fuzz action (parameter fuzzing)
Owner: traffic-agent
Deps: F-05
Acceptance:
- Generate fuzz variants and execute.
- Mark anomalies (status/latency/errors).
- Summarize fuzz results into a finding candidate.

### G. Protocol tasks (v0.1 priorities) / 协议任务（v0.1）
G-01 (P0) HTTP service discovery agent task
Owner: protocol-agent
Deps: D-02
Acceptance:
- Detect HTTP services for a target (host:port).
- Save endpoints + banners as artifacts.
- Produce structured results stored in DB.

G-02 (P0) HTTP vuln checks baseline (auth, legacy endpoints, injection heuristics)
Owner: protocol-agent
Deps: G-01,F-02
Acceptance:
- Run a small set of checks and output findings candidates.
- Link findings to traffic rows and artifacts.
- Does not execute exploit without approval.

G-03 (P0) FTP detection + weak auth checks
Owner: protocol-agent
Deps: D-02
Acceptance:
- Detect FTP and attempt safe banner/auth checks.
- Record evidence.
- Produce a finding candidate when weak auth found.

G-04 (P0) Telnet detection + weak auth checks
Owner: protocol-agent
Deps: D-02
Acceptance:
- Detect Telnet and attempt safe auth checks.
- Evidence + finding candidate.
- Rate limiting and timeout controls.

G-05 (P1) MAVLink capture + parse module integration
Owner: protocol-agent
Deps: D-02,F-01
Acceptance:
- Capture/ingest MAVLink traffic from pcap.
- Decode basic message types.
- Surface anomalies into finding candidates.

G-06 (P1) DJI protocol placeholder interface
Owner: protocol-agent
Deps: A-01
Acceptance:
- Define DJI protocol adapter interface.
- Provide stub implementation that records “not supported yet” with metadata.
- UI indicates coverage state clearly.

### H. Findings & Vulns UI / 发现与漏洞界面
H-01 (P0) Findings list view (filters + status)
Owner: ui-agent
Deps: B-01
Acceptance:
- List findings with severity tabs.
- Status badges (validating/confirmed/new).
- Search/filter works.

H-02 (P0) Finding detail: AI analysis panel + PoC panel
Owner: ui-agent
Deps: H-01
Acceptance:
- Show confidence/exploitability/impact bars.
- Show PoC content as markdown.
- Link to related traffic and artifacts.

H-03 (P1) CVE enrichment panel
Owner: vuln-data-agent
Deps: H-02
Acceptance:
- Store CVE metadata and show CVSS score.
- Detection time and asset binding.
- Works offline with cached data.

### I. Assets & Topology UI / 资产与拓扑
I-01 (P0) Asset inventory CRUD
Owner: core-agent
Deps: B-01
Acceptance:
- Add/edit/delete assets.
- Tag assets and bind to runs.
- Show summary counts.

I-02 (P0) Topology graph rendering + node details drawer
Owner: ui-agent
Deps: I-01
Acceptance:
- Render nodes/edges with risk/vuln counts.
- Clicking node shows details (as mocks `Assets_Clicked_Node`, `Assets_Node_Details`).
- Highlight selected edges.

I-03 (P1) Z1-Z5 zone layout view
Owner: ui-agent
Deps: I-02
Acceptance:
- Visual columns for Z1..Z5.
- Assets placed into zones.
- Show zone counts + legend.

### J. Devices UI (RF hardware) / 设备界面
J-01 (P1) Device inventory model (USRP/HackRF/RTL-SDR)
Owner: devices-agent
Deps: B-01
Acceptance:
- Store device type, id, status, basic metrics.
- UI list shows connected/disconnected/error.
- Bind a device to a task/run (metadata only).

J-02 (P1) Device detail panel layout
Owner: ui-agent
Deps: J-01
Acceptance:
- Render device parameters cards (freq, sample rate, bandwidth, gain, temp).
- Start/stop task buttons (wired to stub actions).
- Log view opens.

### K. Images UI (execution env page) / 镜像与执行环境界面
K-01 (P0) Images page: list running envs + resource meters
Owner: ui-agent
Deps: C-01,B-01
Acceptance:
- Cards show agent, current task, CPU/mem, ports.
- Status badges: running/stopped/building.
- “Create image” button wired to stub.

K-02 (P1) Image build pipeline interface
Owner: vm-agent
Deps: C-01
Acceptance:
- Define build spec (base image + tools).
- Track build progress events.
- Persist build results.

### L. Settings / 全局设置
L-01 (P0) Approval settings + session policy
Owner: security-agent
Deps: D-03
Acceptance:
- Configure which actions require approval.
- Toggle allow-all-for-session.
- Persist settings per workspace.

L-02 (P0) Storage settings: workspace quotas semantics
Owner: db-agent
Deps: B-01
Acceptance:
- Define and store quotas for DB+artifacts.
- Warn when nearing quota.
- Does not conflate with runtime VM resources.

### M. Reporting / 报告
M-01 (P0) Markdown report generator (per run)
Owner: reporting-agent
Deps: B-01,H-01,F-03
Acceptance:
- Generate report.md with findings summary + links to artifacts.
- Export bundle as zip.
- Re-open report in UI.

M-02 (P1) Markdown notes ingestion + render
Owner: reporting-agent
Deps: B-01
Acceptance:
- Accept agent-authored markdown notes as artifacts.
- Render in UI with safe markdown renderer.
- Cross-link to tasks and traffic.

## 3. Notes for UI dev tasks / UI 开发备注
- UI must reference the mocks in `interface_pic/*.png` as the primary source of layout and interaction.
- Keep high information density, minimal decorative icons, keyboard-friendly interactions.
