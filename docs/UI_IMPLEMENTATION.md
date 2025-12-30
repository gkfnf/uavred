# UI Implementation Guide

基于 Figma 设计实现的 UI 组件。

## 已实现的组件

### 1. NavigationBar (`navigation.rs`)
顶部导航栏,包含:
- **Tab 导航**: Dashboard, Assets, Scan, Vulns, Traffic, Flows
- **徽章计数**: Vulns(2), Traffic(8) 显示红色徽章
- **目标显示**: Target: mavic-3-pro-v2.local
- **AI 状态**: AI Active (绿色指示灯)

**交互**:
- 点击 Tab 切换视图
- Hover 效果(未激活的 Tab)
- 激活 Tab 显示紫色背景

### 2. KanbanBoard (`kanban.rs`)
Mission Control 看板视图:
- **三列布局**: To Do → In Progress → Done
- **任务卡片**: 
  - 任务标题
  - 类型标签 (SCAN, ANALYSIS, RECON, EXPLOIT)
  - 优先级标签 (critical, high, medium, low)
- **列头**: 带计数和添加按钮

**交互**:
- 点击 "+" 添加新任务
- (TODO) 拖拽卡片在列之间移动
- 点击卡片查看详情

### 3. FindingsView (`findings.rs`)
安全发现列表:
- **Header**:
  - 标题和统计 (Total, Critical, High)
  - 搜索框
  - Export Report 按钮
- **过滤 Tabs**: All, Critical, High, Medium, Low, Info
- **Finding 卡片**:
  - 严重程度指示器(彩色圆点)
  - 标题和 CVE 编号
  - 描述
  - 时间和目标信息
  - 状态标签 (CONFIRMED, VALIDATING, NEW)

**交互**:
- 点击过滤 Tab 筛选发现
- 搜索框过滤
- 点击 Export 导出报告
- 点击卡片查看详情

### 4. AgentPanel (`agent_panel.rs`)
AI Agent 实时追踪面板:
- **Header**: Agent 名称和 LIVE TRACE 状态
- **Mission Objective**: 显示当前任务目标
- **日志时间线**:
  - HISTORY (⚪ 灰色)
  - THOUGHT (💭 紫色)
  - PLAN (📋 黄色)
  - TOOL (🔧 蓝色)
- **代码执行**: 显示命令和执行状态

**交互**:
- 实时滚动显示日志
- 自动跟随最新日志
- 点击日志展开详情

## 颜色系统

### 语义化颜色
```rust
// 状态颜色
Success:  0x10b981  // 绿色 - 成功/确认/低优先级
Warning:  0xfbbf24  // 黄色 - 警告/验证中/中等优先级
Error:    0xef4444  // 红色 - 错误/严重/高优先级
Info:     0x60a5fa  // 蓝色 - 信息/新发现
Purple:   0xa78bfa  // 紫色 - Agent/分析
Orange:   0xf97316  // 橙色 - 高优先级

// 背景颜色
Primary:   0x1e1e1e  // 主背景
Secondary: 0x252525  // 卡片背景
Tertiary:  0x2d2d2d  // 边框/分割线
Hover:     0x2d2d2d  // Hover 状态

// 文字颜色
Text:      0xffffff  // 主文字(白色)
Muted:     0x9ca3af  // 次要文字(灰色)
Disabled:  0x6b7280  // 禁用文字
```

### 优先级颜色
```rust
Critical: 0xef4444  // 红色
High:     0xf97316  // 橙色
Medium:   0xfbbf24  // 黄色
Low:      0x10b981  // 绿色
```

## 布局系统

### 主布局
```
┌─────────────────────────────────────────────────┐
│           NavigationBar (顶部导航栏)              │
├─────────────────────────────────────┬───────────┤
│                                     │           │
│         Main Content Area           │  Agent    │
│     (Kanban or Findings View)       │  Panel    │
│                                     │  (右侧)   │
│                                     │           │
└─────────────────────────────────────┴───────────┘
```

### Kanban 布局
```
┌──────────────┬──────────────┬──────────────┐
│   To Do      │ In Progress  │    Done      │
│   ● 2        │   ● 2        │    ● 1       │
│  ┌────────┐  │  ┌────────┐  │  ┌────────┐  │
│  │ Task 1 │  │  │ Task 3 │  │  │ Task 5 │  │
│  └────────┘  │  └────────┘  │  └────────┘  │
│  ┌────────┐  │  ┌────────┐  │              │
│  │ Task 2 │  │  │ Task 4 │  │              │
│  └────────┘  │  └────────┘  │              │
└──────────────┴──────────────┴──────────────┘
```

## 组件尺寸

### 间距
- `gap_1`: 0.25rem (小间距)
- `gap_2`: 0.5rem (标准间距)
- `gap_3`: 0.75rem (中等间距)
- `gap_4`: 1rem (大间距)

### Padding
- `p_1` / `px_1` / `py_1`: 0.25rem
- `p_2` / `px_2` / `py_2`: 0.5rem
- `p_3` / `px_3` / `py_3`: 0.75rem
- `p_4` / `px_4` / `py_4`: 1rem

### 圆角
- `rounded_md`: 中等圆角(卡片)
- `rounded_full`: 完全圆角(徽章、指示器)

### 边框
- `border_1`: 1px 边框
- `border_b_1`: 底部 1px 边框
- `border_l_1`: 左侧 1px 边框

## 字体大小

```rust
TextSize::XSmall   // 标签、时间戳
TextSize::Small    // 描述文字
TextSize::Medium   // 标题、正文
TextSize::Large    // 页面标题
TextSize::XLarge   // 主标题
```

## 待实现功能

### Phase 1 - 基础交互
- [ ] Tab 切换视图
- [ ] 任务卡片点击显示详情
- [ ] Finding 卡片点击显示详情
- [ ] 过滤和搜索功能

### Phase 2 - 高级交互
- [ ] Kanban 拖拽功能
- [ ] 任务添加表单
- [ ] Finding 导出功能
- [ ] Agent 日志实时更新

### Phase 3 - 数据集成
- [ ] 连接 Agent 系统
- [ ] 连接扫描模块
- [ ] 连接漏洞数据库
- [ ] WebSocket 实时通信

### Phase 4 - 动画和过渡
- [ ] 页面切换动画
- [ ] 卡片添加/删除动画
- [ ] Loading 状态
- [ ] Toast 通知

## 设计原则遵循

✅ **高信息密度**: 紧凑的布局,充分利用空间
✅ **简洁优雅**: 最小化装饰,专注功能
✅ **语义化颜色**: 颜色传达状态和优先级
✅ **功能性图标**: 仅使用表意清晰的 emoji
✅ **等宽字体**: 代码和技术信息
✅ **简洁动效**: 避免过度动画

## 参考资源

- Figma 设计原型(用户提供的 3 张截图)
- gpui-component 文档
- 项目 AGENTS.md 设计指南
