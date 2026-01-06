# Dashboard UI 设计图分析

基于提供的三张 Dashboard 界面设计图，详细分析界面布局、交互方式和展示内容。

## 整体布局结构

### 1. 顶部导航栏（Top Navigation Bar）
**位置：** 屏幕最顶部与窗口管理之类的处于同一水平线
**背景：** 白色
**内容：**
- **左侧导航项：**
  - Dashboard（当前激活，高亮显示）
  - Assets
  - Images
  - Vulns（红色徽章显示 "2"）
  - Traffic（红色徽章显示 "8"）
  - Flows
  - Devices
  - Settings

- **右侧状态信息：**
  - "AI Active" 状态指示器（绿色/蓝色圆点）
  - 当前时间显示（如 "10:05:56 AM"）

**交互：**
- 点击导航项切换不同页面
- 徽章显示未读/重要信息数量
- 状态指示器实时显示 AI 状态

### 2. 二级导航/上下文栏（Secondary Header Bar）
**位置：** 顶部导航栏下方
**背景：** 浅灰色或白色
**内容：**
- **左侧标签：**
  - "Mission Control"（紫色高亮，表示当前视图）
  - "Findings" + 数量（如 "Findings 5"）

**交互：**
- 点击标签切换 Mission Control 和 Findings 视图
- 目标信息显示当前操作对象

## 视图 1：Mission Control - Kanban 看板视图

### 布局结构
```
┌─────────────────────────────────────────────────────────┐
│  Top Navigation Bar                                     │
├─────────────────────────────────────────────────────────┤
│  Mission Control | Findings 5                           │
├──────────┬──────────┬──────────┬────────────────────────┤
│To Do     │InProgress│  Done    │  (空区域，或详情面板)  │
│---------------------------------------------------------│
│          │          │          │                        │
│          │          │          │                        │
└──────────┴──────────┴──────────┴────────────────────────┘
```

### 三列 Kanban 布局

#### 列结构
每列包含：
1. **列头（Column Header）**
   - 列标题（"To Do", "In Progress", "Done"）
   - 任务计数徽章（如 "2", "2", "1"）
   - 添加按钮（"+" 图标）

2. **任务卡片列表**
   - 垂直排列的任务卡片
   - 卡片之间有间距

#### 任务卡片（Task Card）结构
每个任务卡片显示：

1. **任务标题**
   - 字体：中等大小，加粗
   - 颜色：深灰色/黑色

2. **标签组（Tags）**
   - **类型标签（Task Type）：**
     - ANALYSIS（蓝色背景）
     - RECON（蓝色背景）
     - SCAN（蓝色背景）
     - EXPLOIT（蓝色背景）
     - PENTEST（蓝色背景）
   
   - **优先级标签（Priority）：**
     - "medium"（橙色背景，白色文字）
     - "low"（绿色背景，白色文字）
     - "high"（红色背景，白色文字）

**视觉设计：**
- 白色卡片背景
- 圆角边框
- 卡片之间有间距
- 选中状态：紫色边框高亮

### 示例任务数据

**To Do 列（2个任务）：**
1. "Analyze Flight Logs"
   - 类型：ANALYSIS
   - 优先级：medium

2. "Check Firmware Version"
   - 类型：RECON
   - 优先级：low

**In Progress 列（2个任务）：**
1. "Full Security Scan on Mavic 3"
   - 类型：SCAN
   - 优先级：high

2. "Generate PoC for CVE-2024-1234"
   - 类型：EXPLOIT
   - 优先级：high

**Done 列（1个任务）：**
1. "Verify Weak Auth"
   - 类型：PENTEST
   - 优先级：low

## 视图 2：Mission Control - 任务详情面板

### 布局结构
当选中任务卡片时，右侧显示详情面板：

```
┌──────────┬──────────┬──────────┬────────────────────────┐
│  To Do   │ In       │  Done    │  [任务详情面板]         │
│          │ Progress │          │                        │
│  [选中]  │          │          │  - 任务标题和ID         │
│          │          │          │  - 操作按钮             │
│          │          │          │  - MISSION OBJECTIVE    │
│          │          │          │  - AI AGENT 活动        │
└──────────┴──────────┴──────────┴────────────────────────┘
```

### 详情面板结构

#### 1. 面板头部（Panel Header）
- **任务标题** + "ID: 2"
- **操作按钮组（右侧）：**
  - 眼睛图标（查看）
  - 下载箭头（导出）
  - X 图标（关闭面板）

#### 2. MISSION OBJECTIVE 部分
**标题：** "MISSION OBJECTIVE"
**内容格式：** 列表项，每项以 ">" 开头
**示例：**
- "> Analyze the target drone communication interface for injection vulnerabilities."
- "> Focus on legacy PHP endpoints."

**展示目的：** 清晰说明任务目标和重点

#### 3. AI PENLIGENT AGENT 部分
**标题：** "AI PENLIGENT AGENT"
**内容结构：** 时间线格式，包含多个条目

每个条目包含：
- **类型标签** + **时间戳**
  - HISTORY（历史记录）- 灰色
  - THOUGHT（思考过程）- 紫色
  - PLAN（计划步骤）- 黄色

- **内容文本**
  - HISTORY：记录已完成的操作和发现
  - THOUGHT：AI 的分析和推理过程
  - PLAN：下一步行动计划（编号列表）

**示例条目：**

**HISTORY 14:30:05:**
"Initial reconnaissance completed. Target appears to be running OpenResty + PHP 5.6.40. Several potentially vulnerable parameters identified."

**THOUGHT 14:31:12:**
"Detected suspicious parameter `?ip=` in the URL. This pattern suggests a potential Command Injection vulnerability. The legacy PHP version (5.6.40) increases the likelihood of unpatched security flaws."

**PLAN 14:31:15:**
1. "Verify connection to target."
2. "Fuzz the `ip` parameter with common injection payloads."
3. "Analyze response time and content for execution indicators."

**视觉设计：**
- 不同类型用不同颜色区分
- 时间戳显示在类型标签后
- 内容文本清晰易读
- 计划步骤使用编号列表

## 视图 3：Findings - 安全发现列表

### 布局结构
切换到 "Findings" 标签后，显示安全发现列表视图：

```
┌─────────────────────────────────────────────────────────┐
│  Top Navigation Bar                                      │
├─────────────────────────────────────────────────────────┤
│  Mission Control | Findings 5    Target: mavic-3...   │
├─────────────────────────────────────────────────────────┤
│  Security Findings                                       │
│  Total: 5  Critical: 2  High: 2                        │
│  [All Findings] [Critical] [High] [Medium] [Low] [Info] │
│  [Q Filter findings...] [Export Report]                 │
├─────────────────────────────────────────────────────────┤
│  [发现卡片列表]                                          │
│  - MAVLink Buffer Overflow                               │
│  - DJI Auth Bypass                                       │
│  - MySQL Default Creds                                   │
│  - RTSP Stream Injection                                 │
└─────────────────────────────────────────────────────────┘
```

### Findings 视图结构

#### 1. 页面头部
- **标题：** "Security Findings"
- **统计信息：**
  - "Total: 5"
  - "Critical: 2"（红色高亮）
  - "High: 2"（橙色高亮）

#### 2. 过滤和搜索栏
- **过滤标签按钮：**
  - "All Findings"（当前选中，紫色高亮）
  - "Critical"
  - "High"
  - "Medium"
  - "Low"
  - "Info"

- **搜索框：** "Q Filter findings..."
- **导出按钮：** "Export Report"

#### 3. 发现卡片（Finding Card）

每个发现卡片包含：

1. **严重程度指示器**
   - 红色圆点（Critical/High）
   - 橙色圆点（Medium/High）

2. **标题**
   - 字体：中等大小，加粗
   - 示例："MAVLink Buffer Overflow"

3. **CVE 标签（可选）**
   - 灰色背景
   - 显示 CVE ID，如 "CVE-2024-1234"

4. **描述文本**
   - 详细说明漏洞信息
   - 格式："Detected potential [漏洞类型] vulnerability on [目标] via [协议]."

5. **元信息行**
   - **时间戳：** "2m ago", "5m ago" 等
   - **受影响实体：** "DJI Mavic 3"（带外部链接图标）
   - **状态标签：**
     - "CONFIRMED"（绿色，带勾选图标）
     - "VALIDATING"（黄色，带时钟图标）
     - "NEW"（浅蓝色，带感叹号图标）

6. **操作指示器**
   - 右箭头图标（表示可点击查看详情）

### 示例发现数据

1. **MAVLink Buffer Overflow**
   - CVE: CVE-2024-1234
   - 描述：Detected potential MAVLink Buffer Overflow vulnerability on DJI Mavic 3 via MAVLink protocol.
   - 时间：2m ago
   - 状态：CONFIRMED（绿色）

2. **DJI Auth Bypass**
   - 描述：Detected potential DJI Auth Bypass vulnerability on DJI Mavic 3 via DJI protocol.
   - 时间：5m ago
   - 状态：VALIDATING（黄色）

3. **MySQL Default Creds**
   - 描述：Detected potential MySQL Default Creds vulnerability on GCS Primary via MySQL protocol.
   - 时间：8m ago
   - 状态：CONFIRMED（绿色）

4. **RTSP Stream Injection**
   - 描述：Detected potential RTSP Stream Injection vulnerability on DJI Mavic 3 via RTSP protocol.
   - 时间：12m ago
   - 状态：NEW（浅蓝色）

## 交互方式总结

### 1. 导航交互
- **顶部导航：** 点击切换主要功能页面
- **二级导航：** 在 Mission Control 和 Findings 之间切换
- **徽章提示：** 显示重要信息数量

### 2. Kanban 看板交互
- **任务卡片点击：** 选中任务，右侧显示详情面板
- **添加任务：** 点击列头的 "+" 按钮
- **拖拽（推测）：** 任务卡片可能在列之间拖拽移动

### 3. 详情面板交互
- **查看按钮：** 查看任务详细信息
- **下载按钮：** 导出任务相关数据
- **关闭按钮：** 关闭详情面板，返回看板视图

### 4. Findings 交互
- **过滤：** 点击过滤标签按钮筛选不同严重程度的发现
- **搜索：** 在搜索框中输入关键词过滤
- **导出：** 点击 "Export Report" 导出报告
- **卡片点击：** 点击发现卡片查看详细信息

## 设计特点

### 1. 信息层次
- **三级导航：** 顶部导航 → 二级标签 → 内容区域
- **清晰分组：** 不同类型信息用不同区域展示

### 2. 视觉反馈
- **选中状态：** 紫色高亮表示当前激活项
- **状态指示：** 颜色编码（红色=严重，绿色=确认，黄色=验证中）
- **徽章提示：** 数字徽章显示重要信息

### 3. 实时性
- **时间戳：** 显示相对时间（"2m ago"）和绝对时间（"14:30:05"）
- **状态更新：** AI Agent 活动实时显示

### 4. 上下文信息
- **目标显示：** 始终显示当前操作目标
- **计数信息：** 显示任务和发现的数量

## 关键设计元素

### 颜色系统
- **紫色：** 激活/选中状态
- **红色：** 严重/高优先级
- **橙色：** 中等优先级/警告
- **绿色：** 低优先级/确认状态
- **黄色：** 验证中状态
- **蓝色：** 信息/新状态
- **灰色：** 次要信息/标签

### 标签系统
- **任务类型标签：** ANALYSIS, RECON, SCAN, EXPLOIT, PENTEST
- **优先级标签：** low, medium, high
- **状态标签：** CONFIRMED, VALIDATING, NEW
- **严重程度标签：** Critical, High, Medium, Low, Info

### 时间显示
- **相对时间：** "2m ago", "5m ago"（用于 Findings）
- **绝对时间：** "14:30:05"（用于 AI Agent 活动时间线）

## 用户体验设计

### 1. 渐进式信息披露
- 看板视图：显示任务概览
- 详情面板：显示完整信息
- 支持快速浏览和深入查看

### 2. 多视图切换
- Mission Control：任务管理视图
- Findings：安全发现视图
- 无缝切换，保持上下文

### 3. 实时反馈
- AI Agent 活动实时显示
- 状态更新及时反映
- 时间戳提供时间上下文

### 4. 操作便捷性
- 一键过滤和搜索
- 快速导出功能
- 清晰的视觉指示
