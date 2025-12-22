# UAVRed UI 设计蓝图（供 UI Agent 执行）

> 技术栈：Rust + `gpui`（可选：`gpui-components`）。  
> 产品目标：面向无人机生态红队测试的“多 agent 非线性调度”桌面工作台；以 **可观测、可审计、可回放** 的方式，将自然语言意图 → 任务编排 → 虚拟执行环境（devbox/docker）→ 证据收集 → 风险评分 → 报告输出 串成闭环。

---

### 0) 现状基线（必须遵循的实现风格）

当前项目已采用 gpui 的声明式元素与链式样式 API（例如 `KanbanBoard` 的 `render_*`）作为 UI 基线：

```90:214:src/kanban.rs
    fn render_card(&self, card: &Card) -> impl IntoElement {
        div()
            .bg(card.color)
            .rounded_md()
            .p_3()
            .mb_2()
            .shadow_md()
            .cursor_pointer()
            .hover(|style| style.bg(hsla(0.55, 0.3, 0.90, 1.0)))
            .child(
                div()
                    .text_color(rgb(0x1a1a1a))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_size(px(14.0))
                    .mb_1()
                    .child(card.title.clone()),
            )
            .child(
                div()
                    .text_color(rgb(0x4a4a4a))
                    .text_size(px(12.0))
                    .child(card.description.clone()),
            )
    }
// ... render_column / render ...
```

**约束**：UI agent 的新增界面必须保持一致的 API 风格（声明式 + 链式 style），并优先抽出可复用组件（不要把所有 UI 堆在一个 `render()` 里）。

---

### 1) 北极星原则（必须）

- **可观测优先**：任何自动化测试/agent 行为都要有可视化“证据链”：输入 → 执行 → 输出 → 影响（资产/风险）。
- **可审计与可回放**：所有重要动作（生成环境、执行 PoC、发送流量、触发 RF 发射、风险评分）都必须能追溯到：谁（agent/用户）、何时、对哪些资产、基于什么意图/配置。
- **非线性工作流**：允许用户并行创建/暂停/重排多个 agent 任务；UI 不能强制线性向导，但要提供“最短闭环路径”。
- **渐进披露**：默认只展示“决策所需的信息”；深层日志/PCAP/RF 谱图/浏览器录屏等放在 Inspector/Artifacts 里按需展开。
- **安全与稳健**：默认不展示 secret；危险操作（发包/发射/破坏性 exploit）必须有明确确认与范围提示。

---

### 2) 信息架构（IA）：三个主区 + 一条证据链

**App Shell 建议布局（固定）**
- **左侧全局导航（Nav）**：Workspace/Scopes、Assets、Agents、Runs、Findings、Risk Model、Reports、Settings
- **中间工作区（Workspace）**：核心“非线性调度”视图，默认是 Kanban（可切换为 Timeline/Graph）
- **右侧检查器（Inspector）**：选中对象的详情、日志、证据、风险、操作按钮（Start/Stop/Retry/Export）

**核心对象（UI 需围绕这些对象设计）**
- **Intent（意图）**：用户自然语言 + 约束（资产范围、禁止项、时间/成本、权限）
- **Plan（计划）**：agent 拆解的任务图（可编辑/可重排）
- **Run（执行）**：一次可回放执行（含环境、工具链、日志、证据、资源消耗）
- **Artifact（证据）**：PCAP、HTTP 流量、RF IQ、截图、录屏、payload、脚本、报告片段
- **Finding（发现）**：漏洞/弱点/攻击面条目（证据指针 + 影响范围）
- **Risk（风险）**：基于用户/专家评分模型的计算结果（可解释）

---

### 3) 核心交互蓝图（VibeKanban 风格的“多 agent 调度”）

#### 3.1 工作区默认视图：Agent Kanban

- **列（Column）**：表示生命周期阶段（例：Backlog / Planned / Running / Evidence / Review / Done / Blocked）
- **卡片（Card）**：一个“可执行单元”，可代表：
  - 单个任务（Task）
  - 子流程（Sub-run）
  - 或一个 agent 的“会话片段”（Session chunk）
- **泳道（Lane，可选）**：按 agent/资产/链路分组（用户可切换）

**卡片必须显示的最小信息（信息密度规则）**
- **标题**：动词 + 目标（例：“枚举 MAVLink 端口暴露”）
- **对象标签**：资产/协议/工具（AssetTag / ProtoTag / ToolTag）
- **状态与进度**：Queued/Running/Failed/Needs-Approval + ETA/step count
- **风险提示（可选）**：对潜在破坏/发射/外联的标识

#### 3.2 右侧 Inspector（必须）

选中卡片/发现/资产时，Inspector 统一提供：
- **Summary**：意图摘要、范围、关键参数
- **Actions**：Start/Stop/Retry/Approve/Export
- **Live Output**：结构化事件流 + 原始日志（可切换）
- **Artifacts**：证据列表（PCAP/RF/截图/脚本/报告片段）
- **Risk**：评分结果 + 可解释因子（为什么是这个分）

#### 3.3 Command Palette（强烈建议）

- 全局快捷入口：创建意图、添加资产、启动 agent、打开 run、导出报告、切换视图
- “可键盘操作”是桌面红队工具的核心效率点

---

### 4) 视觉与设计系统（Token 化）

**目标风格**：专业、信息密度高、低噪音、暗色优先（可切换浅色），类似安全分析工作台而非消费级 ToDo。

- **排版**
  - **标题**：16–20px（Semibold/Bold）
  - **正文**：12–14px（Regular）
  - **等宽**：日志/代码/命令（Monospace）
- **间距（建议）**
  - 基础单位：4px
  - 常用：8/12/16/24
- **颜色（语义优先）**
  - **状态色**：Running（蓝/青）、Success（绿）、Warning（黄）、Error（红）、Blocked（灰）
  - **风险等级色**：Info/Low/Med/High/Critical（必须全局一致）
  - **背景层级**：App 背景 / Panel 背景 / Card 背景需可区分（靠亮度而非花色）

**硬规则**
- 不用“纯色大面积高饱和”作为背景；风险色仅用于强调，不要污染整屏。
- 任何彩色都必须有语义（状态、风险、标签类别），否则用中性色。

---

### 5) 组件规范（最小组件库）

> UI agent 开发新界面时，必须复用/扩展这些组件，而不是每个页面重新拼样式。

- **AppShell**
  - 左 Nav + 中 Workspace + 右 Inspector 的布局容器
- **Topbar**
  - Workspace 标题、搜索、Command Palette、全局状态（环境队列/运行中任务数）
- **SidebarNavItem**
  - 图标 + 文本 + badge（数量/告警）
- **KanbanBoard / KanbanColumn / KanbanCard**
  - 统一卡片密度、hover、选中态、拖拽占位态
- **Tag**
  - Asset/Protocol/Tool/Agent 标签（同一尺寸体系）
- **StatusPill**
  - Queued/Running/Failed/NeedsApproval 等
- **InspectorPanel**
  - 可折叠 section：Summary/Actions/Output/Artifacts/Risk
- **EventStreamView**
  - 结构化事件（时间戳、层级、来源 agent、可展开 payload）
- **ArtifactList**
  - 列表 + 预览 + 导出（路径/哈希/来源 run）
- **RiskBreakdown**
  - 分项因子 + 权重 + 总分（可解释）
- **ConfirmDialog**
  - 危险操作确认（显示范围、影响、回滚/停止说明）

---

### 6) 状态模型与工程规则（gpui 可落地）

- **业务状态与渲染分离（必须）**
  - 业务层：Intent/Plan/Run/Finding/Risk 的纯 Rust model（可测试）
  - UI 层：gpui view/element 仅负责渲染与派发 action
- **事件驱动**
  - UI 不直接调用“执行引擎”，而是 dispatch action（StartRun/StopRun/ApproveDangerousOp）
  - 执行引擎回推 event（RunStarted/ArtifactAdded/FindingCreated/RiskUpdated）
- **列表虚拟化/增量渲染（建议）**
  - 日志与事件流必须支持“增量追加 + 过滤 + 折叠”
- **错误与离线**
  - 每个 async/外部工具调用都要有：loading、timeout、error、retry

---

### 7) 风险与合规（红队工具特有 UI 规则）

- **范围感知（Scope Awareness）**：界面任何地方都要持续提示当前资产范围（域/网段/设备/频段/协议）。
- **危险操作两段式确认**：展示“将要做什么”“对哪些资产”“预计副作用”“如何停止/回滚”。
- **证据默认保留策略可见**：Artifact 的保留/脱敏/导出策略必须可配置并可视化。

---

### 8) 你需要补充的信息（用于校准到 refRepo 的三项目）

当前 `refRepo/` 为空且无子模块配置。请提供以下之一：
- 三个参考项目的**仓库地址**（或你希望放进 `refRepo/` 的目录结构），或
- 你本地真实路径（我可指导如何以 submodule/拷贝导入到本仓库）。


