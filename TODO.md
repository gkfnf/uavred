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
- [ ] GPUI dependency resolution
- [ ] Top Navigation Bar
  - [ ] Tab navigation (Dashboard, Assets, Scan, Vulns, Traffic, Flows)
  - [ ] Target display
  - [ ] Settings and AI status
- [ ] Mission Control Dashboard (Kanban View)
  - [ ] Three-column layout (To Do, In Progress, Done)
  - [ ] Task cards with tags and priority
  - [ ] Drag-and-drop support
  - [ ] Right panel for task details
- [ ] Findings View
  - [ ] Security findings list
  - [ ] Severity badges (Critical, High, Medium, Low)
  - [ ] Status indicators (Confirmed, Validating, New)
  - [ ] Filter functionality
  - [ ] Export report button
- [ ] AI Agent Panel
  - [ ] Live trace display
  - [ ] History timeline
  - [ ] Thought/Plan/Tool sections
  - [ ] Code execution display

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
