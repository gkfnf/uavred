# Figma 设计图分析

**分析日期**: 2025-12-31  
**设计工具**: Figma  
**设计稿状态**: Make 生成的高保真原型

---

## 1. 总体设计特征

### 1.1 配色方案
- **主背景**: 浅灰色 / 白色 (#f5f5f5 或更亮)
- **次背景**: 白色卡片 (#ffffff)
- **强调色**: 紫色 (#7c3aed 或类似) - 按钮、选中状态
- **文字**: 深灰 / 黑色
- **边框**: 浅灰 (#e5e7eb)
- **状态颜色**:
  - 成功: 绿色 (#10b981)
  - 错误/Critical: 红色 (#ef4444)
  - 警告/High: 橙色 (#f97316)
  - 信息: 蓝色 (#3b82f6)
  - 中等: 黄色 (#fbbf24)

### 1.2 风格特点
- **现代化**: 圆角卡片、阴影、清爽布局
- **高信息密度**: 充分利用空间但不拥挤
- **系统化**: 图标统一、组件复用、色彩体系一致
- **可读性**: 足够的对比度、清晰的字体层次
- **交互感**: 徽章、标签、进度条等丰富的 UI 元素

---

## 2. 主界面分析

### 2.1 顶部导航栏

**高度**: 约 48-52px

**布局** (从左到右):
```
[App Icon] [Dashboard] [Assets] [Scan] [Vulns 2] [Traffic 4]...
                                                     [Settings] [• AI Active] [Time]
```

**元素**:
- macOS 应用风格的红/黄/绿 窗口控制按钮（左上角）
- 主导航 Tab (Dashboard, Assets, Scan, Vulns, Traffic, Flows)
- 主导航为圆形或略带背景的设计，活跃态为紫色圆形背景
- 徽章计数: 紫色圆形徽章，右上角显示数字
- 右侧: Settings 齿轮图标、AI Active 指示器 (绿色圆点)、时间显示

**样式**:
- 背景: 浅灰/白色
- 文字: 黑色/深灰
- 分割线: 底部 1px 浅灰线
- 高度: 48px

### 2.2 Dashboard 视图 (任务编排)

**子导航**:
```
[Mission Control] [Findings 5]    Target: mavic-3-pro-v2.local
```

**Kanban 看板**:
- 三列: "To Do 2" | "In Progress 2" | "Done 1"
- 列顶部有 + 按钮
- 列顶部显示计数
- 列背景: 白色
- 列间距: 16-20px

**任务卡片**:
```
┌─────────────────────────────┐
│ Analyze Flight Logs         │ ← 标题
│ ANALYSIS    medium          │ ← Tag (左侧灰色,右侧彩色)
└─────────────────────────────┘
```

**卡片样式**:
- 背景: 白色
- 边框: 1px 浅灰
- 圆角: 6-8px
- 阴影: 轻微阴影
- 高度: 70-90px
- 内边距: 12-16px
- Hover: 边框变紫色 或 阴影增强

### 2.3 Findings 视图 (DashBoard_B)

**顶部**:
```
[Mission Control] [Findings 5]    Target: mavic-3-pro-v2.local
```

**统计区**:
```
🔒 Security Findings    Total: 5  Critical: 2  High: 2
                        [🔍 Filter findings...]  [Export Report]
```

**过滤 Tabs**:
```
[All Findings] [Critical] [High] [Medium] [Low] [Info]
```

**Finding 列表**:
```
┌────────────────────────────────────────────────────────────┐
│ ● MAVLink Buffer Overflow  CVE-2024-1234  [✓ CONFIRMED] ► │
│ Detected potential MAVLink Buffer Overflow vulnerability   │
│ ⏱ 2m ago    📍 DJI Mavic 3                                 │
└────────────────────────────────────────────────────────────┘
```

**Finding 卡片样式**:
- 背景: 白色
- 边框: 1px 浅灰
- 圆角: 6-8px
- 高度: 80-100px
- 左侧彩色圆点 (对应严重程度)
- 右侧状态徽章 (CONFIRMED / VALIDATING / NEW)
- Hover: 背景变浅，显示 > 箭头

### 2.4 Assets 视图

**顶部**:
```
[网络拓扑 - 业务层级视图]  [折叠/展开]
```

**资产列表项**:
```
资产详情        🟢 Telemetry Server  10.0.1.51    [✎] [🗑] [▲]
```

**样式**:
- 背景: 白色
- 边框: 1px 浅灰
- 圆角: 8px
- 高度: 50-60px
- 右侧操作按钮 (编辑、删除、展开)

### 2.5 Monitor/Images 视图

**标题**:
```
🗂️ 容器镜像 & Agent 执行环境    4 运行中    [+ 创建镜像]
```

**Agent 容器卡片** (网格布局, 3 列):
```
┌─────────────────────────────┐
│ $ docker exec -it img-1      │ ← 终端样式
│ Agent: Agent-Alpha           │
│ Task: CVE-2024-1234 PoC 生成 │
│ [2h 34m] Running...          │
│ ↗ 67%                        │
├─────────────────────────────┤
│ ai-pentest-agent            │ ← Agent 名称
│ v2.3.1                       │
│ Agent-Alpha                  │
│ CVE-2024-1234 PoC 生成       │
│                             │
│ CPU         67%  [██████░]  │
│ Memory      45%  [███░░░░░] │
│ 脚本端口: 8080, 5900        │
│ 运行时长: 2h 34m            │
└─────────────────────────────┘
```

**卡片样式**:
- 背景: 白色
- 边框: 1px 浅灰
- 圆角: 8px
- 宽度: 均匀分配 (3 列)
- 高度: 220-250px
- 阴影: 轻微

**进度条样式**:
- 高度: 4-6px
- 圆角: 3px
- 背景: #e5e7eb
- 填充: 对应颜色 (CPU: 橙色, Memory: 蓝色, 等)

### 2.6 Vulns 视图 (详细分析)

**左侧列** (Vulnerabilities List):
```
🔒 Vulnerabilities  4

[Severity] [Asset] [MITRE]    ← Tab 过滤
[🔍 Search vulnerabilities...]

CRITICAL (2)
├─ [Buffer Overflow in Telemetry Parser]
│  CVE-2024-1234
│  AI 98% • PoC
│  
├─ [Default Admin Credentials]
│  SEC-UAV-002
│  AI 100% • PoC • ✓

HIGH (2)
├─ [MAVLink Command Injection]
│  SEC-UAV-003
│  AI 91% • PoC

└─ [SQL Injection in Flight Logs]
   SEC-UAV-004
   AI 89% • PoC
```

**左侧卡片样式**:
- 背景: 白色
- 边框: 1px 紫色/浅灰
- 圆角: 8px
- 列表项背景: #f9f5ff (浅紫)
- 左侧竖线: 对应严重程度颜色

**中间列** (Details & PoC):
```
CVE-2024-1234 • CWE-120

Buffer Overflow in Telemetry Parser

A buffer overflow vulnerability exists in the
telemetry data parser allowing remote code
execution through malformed telemetry packets.

Detected in: DJI Mavic 3 Pro · telemetry_parser.c:247

🔮 AI Security Analysis
  Confidence Score  [████████░░]  98%
  Exploitability    [██████████]  95%
  Potential Impact   [██████████░] 98%

⚡ AI-Generated PoC
  [✎ Edit] [→ Send]
  
  POST /api/v1/telemetry
  Content-Type: application/json
  
  {"data":"AAAA...AAAA"}
```

**右侧列** (CVE Database):
```
CVE Database

CVSS Score
9.8
v3.1 Base Score

Detection Time
2024-11-05 14:32:45

Asset
DJI Mavic 3 Pro

Quick Actions
[View Asset Details]
```

### 2.7 Traffic 视图

**顶部过滤**:
```
TrafficQL >  method:POST AND path:"/api/v1/telemetry"
            [🟢 Capturing] [Intercept] [⊕]
```

**流量列表**:
```
Captured Traffic (1)    1 anomalies 🟢 Live

#   Time      Asset        Proto  Method  Path              Status Size    Duration
33  AM.000    DJI Mavic 3  HTTP   POST   /api/v1/telemetry  200   511B    156ms
```

**三列详情**:
```
Request                Response               Actions
POST /api/v1/...       HTTP/1.1 500 Int...    Packet Info
Content-Type:...       {"error":"Buffer...    ID: 33, Size: 511B
{"data":"AAA...        
```

**样式**:
- 表格背景: 白色
- 表头背景: 淡灰
- 行高: 40px
- 代码块: 深灰背景 (#2d2d2d或类似)
- 代码文字: 浅色 (等宽字体)

### 2.8 Devices 视图

**左侧** (设备列表):
```
🔌 硬件设备      5 已连接

[🔍 搜索设备...]
[🔄 扫描设备] [+ 添加]

🔲 USRP B210          USRP
   忙碌              /dev/ttyUSB0
   📊 MAVLink 信号捕获

✓ HackRF One          HackRF
  就绪               /dev/ttyUSB1
  915 MHz · 38°C

✓ BladeRF x40         BladeRF
  就绪               /dev/ttyUSB2
  5.8 GHz · 35°C
```

**设备卡片样式**:
- 背景: 白色
- 左侧竖线: 对应状态颜色 (紫色/绿色)
- 字体: 主标题 Bold, 副标题灰色小号
- Hover: 突出显示

**右侧** (设备详情):
```
USRP_B210
USRP         🔗 忙碌

🎯 当前任务
📊 MAVLink 信号捕获

[🛑 停止任务] [🧠 查看日志]

设备信息
序列号         固件版本        端口
31F8A2B        4.1.0.0         /dev/ttyUSB0

无线参数
频率            采样率          带宽           增益
2.4 GHz         20 MS/s         56 MHz         45 dB

设备状态
🌡️ 温度
42°C [████████░░░░░░░░░]

快速操作
[⚙️ 配置参数] [🔄 固件更新]
```

**样式**:
- 背景: 白色
- 分组标题: 粗体，左侧灰色文字
- 参数框: 浅色背景 (#f3f4f6 或类似)
- 状态条: 背景灰，填充绿色
- 按钮: 紫色背景或灰色背景

---

## 3. 通用组件规范

### 3.1 Button
- **Primary**: 紫色背景 (#7c3aed), 白色文字, 12-14px
- **Secondary**: 白色背景, 1px 灰色边框, 黑色文字
- **Danger**: 红色背景, 白色文字
- 高度: 36-40px, 圆角: 6-8px

### 3.2 Badge
- **Count**: 圆形, 紫色背景, 白色数字, 16-20px
- **Status**: 矩形, 彩色背景, 白色/黑色文字, 6px 圆角
- **Tag**: 浅色背景, 可移除

### 3.3 Tabs
- 背景透明
- 选中: 紫色圆形背景或下划线
- 非选中: 灰色文字
- 间距: 12px

### 3.4 Input
- 高度: 36-40px
- 边框: 1px 灰色
- 圆角: 6px
- Focus: 边框变紫色
- Placeholder: 浅灰色

### 3.5 Cards
- 背景: 白色
- 边框: 1px 浅灰
- 圆角: 6-8px
- 阴影: 0 1px 3px rgba(0,0,0,0.1)

### 3.6 Colors
```
Purple:  #7c3aed / #a78bfa
Gray:    #e5e7eb / #d1d5db / #9ca3af / #6b7280 / #374151
Green:   #10b981
Red:     #ef4444
Orange:  #f97316
Yellow:  #fbbf24
Blue:    #3b82f6
```

---

## 4. 实现优先级

### Phase 1 (核心框架)
- [x] 顶部导航栏 (浅色主题)
- [x] Kanban 看板 (白色卡片, 三列)
- [x] Finding 列表 (过滤 Tab, 搜索)
- [ ] Assets 视图
- [ ] Monitor/Images 视图
- [ ] Traffic 视图
- [ ] Devices 视图

### Phase 2 (详情交互)
- [ ] 右侧详情 Drawer
- [ ] 模态框 (Modal)
- [ ] 进度条和状态指示
- [ ] 徽章和标签完整样式

### Phase 3 (交互完善)
- [ ] 拖拽支持
- [ ] 搜索和过滤动画
- [ ] 响应式布局
- [ ] 键盘快捷键

---

## 5. 需要改动的代码文件

基于当前的代码结构 (`src/ui/`), 需要重构:

| 文件 | 当前状态 | 需要改动 |
|------|---------|---------|
| src/ui/navigation.rs | ❌ 深色主题 | 改为浅色主题 + 紫色强调 |
| src/ui/kanban.rs | ❌ 深色主题 | 改为白色卡片 + 浅色背景 |
| src/ui/findings.rs | ❌ 深色主题 | 改为浅色设计 + 左侧竖线 |
| src/ui/agent_panel.rs | ❌ 深色主题 | 改为浅色 或 移除 (集成到其他视图) |
| src/app.rs | ❌ 结构过时 | 重构为多视图支持 |

---

## 6. 设计对标检查清单

在实现每个组件前，要验证：

- [ ] 颜色: 浅色主题，紫色强调
- [ ] 布局: 卡片式、网格式、列表式
- [ ] 边框: 1px 浅灰 (#e5e7eb)
- [ ] 圆角: 6-8px
- [ ] 阴影: 轻微阴影
- [ ] 字体: 系统字体，清晰层次
- [ ] 间距: 一致的间距系统
- [ ] 交互: Tab、过滤、搜索
- [ ] 组件: Badge、Tag、Button、Input
- [ ] 编译: 通过编译检查
- [ ] 界面: 与 Figma 设计图一致

