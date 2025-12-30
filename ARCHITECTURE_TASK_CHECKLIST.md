# 架构设计任务 - 执行检查清单

## Epic Task: uavred-b3d
主任务：Architecture Design: 安全测试意图编排平台 (Architect Agent Task)

---

## 子任务执行顺序

### ✅ 完成项

- [x] uavred-b3d - 创建主 Epic (P0)
- [x] uavred-2q4 - 创建需求梳理任务 (P0)
- [x] uavred-btg - 创建架构设计任务 (P0) [依赖: uavred-2q4]
- [x] uavred-ups - 创建 UI/UX 设计任务 (P0) [依赖: uavred-btg]
- [x] uavred-2v5 - 创建工程拆分任务 (P0) [依赖: uavred-ups]
- [x] uavred-8sm - 创建风险评估任务 (P1) [依赖: uavred-2v5]
- [x] uavred-eef - 创建最终交付任务 (P0) [依赖: uavred-8sm]

### 📋 待执行项

#### Phase 1: 架构师 Agent 启动

**Action 1: 分配架构师 Agent**
```bash
bd update uavred-b3d --status in_progress --assignee <architect-agent-name>
bd update uavred-2q4 --status in_progress --assignee <architect-agent-name>
```

**Action 2: Agent 开始需求梳理**
- 创建 `docs/requirements.md` 文档
- 与人类专家进行 1-2 轮需求评审
- 生成用户故事和使用场景（>=10）
- 预计完成: 5-8 天

**检查点 1: 需求梳理完成**
```bash
# 当需求梳理完成时
bd close uavred-2q4
bd update uavred-btg --status in_progress
```

---

#### Phase 2: 架构设计

**Action 3: 架构设计开始**
- 基于需求，设计系统各层架构
- 生成架构图和组件交互图
- 定义 Agent 标准接口
- 定义数据库 schema

**输出物检查**:
- [ ] `docs/architecture.md` (>=30 页)
- [ ] `docs/system_diagram.svg`
- [ ] `docs/agent_interface.md`
- [ ] `docs/database_schema.md`

**检查点 2: 架构设计完成**
```bash
bd close uavred-btg
bd update uavred-ups --status in_progress
```

---

#### Phase 3: UI/UX 设计

**Action 4: UI/UX 设计开始**
- 优化 Figma 原型
- 设计完整的 UI 规范和组件库
- 编写开发实现指南（GPUI 映射）

**输出物检查**:
- [ ] `docs/ui_design.md` (>=15 页)
- [ ] `docs/wireframes.fig` (更新后的)
- [ ] `docs/component_library.md`
- [ ] `docs/implementation_guide.md` (GPUI specific)

**检查点 3: UI/UX 设计完成**
```bash
bd close uavred-ups
bd update uavred-2v5 --status in_progress
```

---

#### Phase 4: 工程任务拆分

**Action 5: 创建工程任务**

这是最关键的一步，决定了后续 Agent 的开发效率。

需要创建的任务类别：

**Phase 1 子任务** (4-5 个):
```bash
# Example:
bd create "修复 GPUI 编译问题" --type task --priority 0 \
  --parent uavred-b3d --estimate 480 \
  --description "..."

bd create "搭建后端项目结构(API框架、DB初始化)" --type task --priority 0 \
  --parent uavred-b3d --estimate 480 \
  --description "..."

# 等等，共 4-5 个任务
```

**Phase 2-6 子任务** (15-20 个):
- 每个任务都应该有：
  - 清晰的标题
  - 详细的描述（需求、输入、输出）
  - 工作量估计（8-32 天）
  - >=3 条验收标准
  - 依赖关系
  - 标签（前端/后端/基础设施等）

**检查点 4: 所有工程任务创建完成**
```bash
# 验证任务创建
bd list --pretty | grep "parent:uavred-b3d"

# 预期: >=20 个任务
```

---

#### Phase 5: 风险评估

**Action 6: 完成风险评估**
- 识别 15+ 个关键风险
- 为每个风险制定缓解方案
- 生成风险矩阵图

**输出物检查**:
- [ ] `docs/risk_register.md`
- [ ] `docs/risk_matrix.svg`
- [ ] `docs/mitigation_plan.md`

**检查点 5: 风险评估完成**
```bash
bd close uavred-8sm
bd update uavred-eef --status in_progress
```

---

#### Phase 6: 最终交付

**Action 7: 整合所有文档**
- 合并所有设计文档到 `docs/ARCHITECTURE.md`
- 编写 `docs/AGENT_DEVELOPMENT_GUIDE.md`
- 整理>=10 条 ADR
- 准备最终评审

**输出物清单**:
- [ ] `docs/ARCHITECTURE.md` (>=50 页)
- [ ] `docs/AGENT_DEVELOPMENT_GUIDE.md` (>=20 页)
- [ ] `docs/ADR/` 目录 (>=10 条 ADR)
- [ ] 所有可视化产物 (架构图、依赖图等)
- [ ] 快速参考卡片

**Action 8: 最终评审**
```bash
# 人类专家评审
bd comments uavred-eef add "Reviewed and approved. Ready for Agent development."

# 标记完成
bd close uavred-eef
```

---

## 关键里程碑和签字

| 里程碑 | 任务ID | 负责人 | 状态 | 签字 |
|------|--------|--------|------|------|
| 需求梳理完成 | uavred-2q4 | Architect Agent | [ ] | [ ] |
| 架构设计完成 | uavred-btg | Architect Agent | [ ] | [ ] |
| UI/UX 设计完成 | uavred-ups | Architect Agent | [ ] | [ ] |
| 工程任务创建 | uavred-2v5 | Architect Agent | [ ] | [ ] |
| 风险评估完成 | uavred-8sm | Architect Agent | [ ] | [ ] |
| 最终交付评审 | uavred-eef | Architect Agent + Human | [ ] | [ ] |

---

## 常用 BD 命令

```bash
# 查看架构相关任务
bd list --pretty | grep -E "(uavred-b3d|uavred-2q4|uavred-btg|uavred-ups|uavred-2v5|uavred-8sm|uavred-eef)"

# 查看当前可以开始的任务
bd ready

# 开始工作
bd update <task-id> --status in_progress

# 添加进度注释
bd comments <task-id> add "Progress: completed requirement analysis"

# 完成任务
bd close <task-id>

# 查看任务详情
bd show <task-id>

# 查看依赖关系
bd dep tree <task-id>

# 同步到 Git
bd sync
git push
```

---

## 关键文件位置

```
docs/
├── ARCHITECTURE.md                    # 最终主架构文档
├── AGENT_DEVELOPMENT_GUIDE.md         # Agent 开发指南
├── requirements.md                    # 需求规范
├── architecture.md                    # 架构详细设计
├── ui_design.md                       # UI/UX 规范
├── task_breakdown.md                  # 工程任务清单
├── risk_register.md                   # 风险登记
├── ADR/                               # 架构决策记录
│   ├── ADR-001-frontend-architecture.md
│   ├── ADR-002-gpui-choice.md
│   └── ...
└── diagrams/                          # 可视化图表
    ├── system_diagram.svg
    ├── component_diagram.svg
    └── ...

ARCHITECTURE_DESIGN_PLAN.md            # 本规划文档
ARCHITECTURE_TASK_CHECKLIST.md        # 本检查清单
```

---

## 预期输出物总结

### 文档
- ✓ requirements.md (需求规范)
- ✓ ARCHITECTURE.md (>=50 页主文档)
- ✓ AGENT_DEVELOPMENT_GUIDE.md (开发指南)
- ✓ task_breakdown.md (工程任务清单)
- ✓ risk_register.md (风险评估)
- ✓ >=10 条 ADR (架构决策记录)

### 图表
- ✓ system_diagram.svg (系统架构图)
- ✓ component_diagram.svg (组件交互)
- ✓ dependency_graph.svg (任务依赖)
- ✓ risk_matrix.svg (风险矩阵)

### 工程任务
- ✓ 20+ 个具体的 Agent 开发任务
- ✓ 每个任务包含验收标准、工作量、依赖

### 签字认可
- ✓ 人类专家确认需求
- ✓ 人类专家确认架构
- ✓ 架构师和人类专家共同确认最终交付

---

## 常见问题和陷阱

### Q1: 什么时候可以开始 Phase 1 开发任务?
A: 当 uavred-2v5 (工程拆分) 完成后，所有工程任务都被创建并有清晰的验收标准。此时新 Agent 可以开始认领任务。

### Q2: 需求变更怎么办?
A: 在需求梳理阶段（uavred-2q4），应该尽量完整。如果在后续设计中发现遗漏，通过 ADR 记录决策并通知所有 Agent。重大变更应该重新评审。

### Q3: 如果发现设计冲突怎么办?
A: Agent 应该在 BD comments 中提出，由人类专家或架构师决策。修改后通过 ADR 记录。

### Q4: 工作量估计偏差怎么处理?
A: 工程拆分完成后，Agent 在执行时如果发现工作量偏差，应该提前报告。项目经理可以重新调整。

---

## 成功验收标准

整个架构设计流程成功的指标：

- ✅ 6 个顺序任务全部完成
- ✅ 人类专家对需求、架构、UI 无遗漏的确认
- ✅ 20+ 个工程任务创建，每个都有明确的验收标准
- ✅ >=15 个风险被识别并有缓解方案
- ✅ 后续 Agent 可以直接从任务列表选择并独立开发
- ✅ 所有关键设计文档齐全，支持自主开发

---

**最后更新**: 2025-12-31  
**版本**: 1.0
