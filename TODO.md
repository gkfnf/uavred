# UAV Red Team - TODO

## ⭐ Architecture Design Phase (BLOCKER - COMPLETE FIRST)

**Epic Task**: uavred-b3d - Architecture Design: 安全测试意图编排平台

The following design tasks MUST complete before parallel development begins:

- [ ] **uavred-2q4** - 需求梳理 (Target: Week 1)
  - [ ] Interview human expert and document requirements
  - [ ] Confirm UAV ecosystem support scope (MAVLink, DJI, ArduPilot, PX4)
  - [ ] Define web-side testing capabilities
  - [ ] Define agent execution environment requirements
  - [ ] Generate `requirements.md` document
  
- [ ] **uavred-btg** - 架构设计 (Target: Week 1-2, depends on uavred-2q4)
  - [ ] Design system architecture (frontend, backend, container, agent capability)
  - [ ] Generate architecture diagrams and specifications
  - [ ] Define Agent standard interfaces
  - [ ] Generate `docs/architecture.md` document
  
- [ ] **uavred-ups** - UI/UX 设计 (Target: Week 2-3, depends on uavred-btg)
  - [ ] Optimize Figma prototypes
  - [ ] Design high-density information display
  - [ ] Generate UI/UX specification and component library
  - [ ] GPUI implementation guidelines
  
- [ ] **uavred-2v5** - 工程任务拆分 (Target: Week 3-5, depends on uavred-ups)
  - [ ] Break down into 20+ concrete development tasks
  - [ ] Define Phase 1-6 roadmap
  - [ ] Create all engineering tasks in BD
  - [ ] Set dependencies for all tasks
  - [ ] **CRITICAL**: When complete, parallel development can begin
  
- [ ] **uavred-8sm** - 风险评估 (Target: Week 5-6, depends on uavred-2v5)
  - [ ] Identify 15+ technical risks
  - [ ] Document mitigation strategies
  - [ ] Test plans for high-risk items
  
- [ ] **uavred-eef** - 最终交付 (Target: Week 6-8, depends on uavred-8sm)
  - [ ] Merge all design documents
  - [ ] Generate `docs/ARCHITECTURE.md` (>=50 pages)
  - [ ] Generate `docs/AGENT_DEVELOPMENT_GUIDE.md`
  - [ ] Create >=10 ADRs (Architecture Decision Records)
  - [ ] Final review and sign-off

**Status**: Architecture tasks created in BD, awaiting architect-agent assignment

---

## Phase 1: UI Implementation (STARTS AFTER ARCHITECTURE COMPLETE)
- [x] Project initialization
- [x] Basic module structure
- [x] GPUI dependency resolution
- [x] Top Navigation Bar
  - [x] Tab navigation (Dashboard, Assets, Scan, Vulns, Traffic, Flows)
  - [x] Target display
  - [x] Settings and AI status
- [x] Mission Control Dashboard (Kanban View)
  - [x] Three-column layout (To Do, In Progress, Done)
  - [x] Task cards with tags and priority
  - [ ] Drag-and-drop support
  - [ ] Right panel for task details
- [x] Findings View
  - [x] Security findings list
  - [x] Severity badges (Critical, High, Medium, Low)
  - [x] Status indicators (Confirmed, Validating, New)
  - [x] Filter functionality
  - [x] Export report button
- [x] AI Agent Panel
  - [x] Live trace display
  - [x] History timeline
  - [x] Thought/Plan/Tool sections
  - [x] Code execution display

### UI 开发进展总结 (2025-12-31)

**已完成**:
- ✅ 升级到最新的 GPUI API (使用 Context 而非 ViewContext)
- ✅ 集成 gpui-component 组件库
- ✅ 实现浅色主题 (符合 Figma 设计)
- ✅ 完整的导航栏 (6个 Tab + 徽章 + Target 显示 + AI 状态)
- ✅ Mission Control Dashboard (Kanban 看板)
- ✅ Security Findings 视图 (统计 + 搜索 + 过滤 + 列表)
- ✅ AI Agent 面板 (实时日志时间线)
- ✅ 全局样式系统 (`src/ui/styles.rs`)
- ✅ 视图切换框架 (6个视图的占位符)

**代码变更**:
- 更新 `src/main.rs`: 使用 gpui_component_assets Assets, 初始化浅色主题
- 重写 `src/app.rs`: 800+ 行代码，整合所有 UI 组件
- 清理旧版 UI 文件: 删除 navigation.rs, kanban.rs, findings.rs, agent_panel.rs
- 更新 `src/ui/styles.rs`: 浅色主题颜色常量
- 添加 gpui-component-assets 依赖

**编译状态**: ✅ 通过 (仅有少量未使用导入警告)

**待实现** (按优先级):
1. 🔥 Kanban 拖拽功能 (使用 gpui 拖拽 API)
2. 🔥 Tab 点击切换视图 (连接导航栏事件)
3. 🔥 搜索和过滤功能的实际逻辑
4. 📋 Assets 视图 (参考 Assets.png)
5. 📋 Scan 视图
6. 📋 Vulns 详情视图 (参考 Vulns.png)
7. 📋 Traffic 视图 (参考 Traffics.png)
8. 📋 Devices 视图 (参考 Devices.png)
9. 📋 Monitor/Images 视图 (参考 Monitor.png)
10. 📋 Workflows 视图 (参考 WorkFlows.png)
11. 🎨 对比 Figma 设计图微调样式
12. ⚡ 添加实时更新逻辑 (连接 Agent 系统)

## Phase 1b: Core Infrastructure
- [x] Agent system framework
- [x] Task management system
- [x] Vulnerability database

## Phase 2: Network Scanning
- [ ] Implement port scanner
- [ ] UAV device detection (MAVLink, DJI protocols)
- [ ] Service fingerprinting
- [ ] Network mapping visualization

## Phase 3: Protocol Analysis
- [ ] MAVLink parser
- [ ] DJI protocol parser
- [ ] ArduPilot protocol support
- [ ] PX4 protocol support
- [ ] Protocol weakness detection

## Phase 4: Firmware Analysis
- [ ] Firmware extraction tools
- [ ] String analysis
- [ ] Binary analysis
- [ ] Vulnerability pattern matching
- [ ] Credential detection

## Phase 5: UI Enhancement
- [ ] Real-time agent status display
- [ ] Interactive task management
- [ ] Results visualization
- [ ] Export functionality (JSON, PDF)
- [ ] Dark/Light theme support

## Phase 6: Security & Testing
- [ ] Unit tests for all modules
- [ ] Integration tests
- [ ] Security audit
- [ ] Permission management
- [ ] Encrypted storage

## Phase 7: Documentation
- [ ] API documentation
- [ ] User guide
- [ ] Architecture diagrams
- [ ] Security best practices
- [ ] Example workflows
