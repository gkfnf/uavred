# UI 实现计划 (uavred-ups)

**任务**: UI/UX 设计实现  
**验收标准**: 代码改动完成 + 编译通过 + 界面与 Figma 设计一致 + 交互合理

---

## 任务分解

### 步骤 1: 重构顶部导航栏 (navigation.rs)

**设计目标**:
- 浅色主题 (背景 #f5f5f5 / #ffffff)
- macOS 窗口控制按钮 (左上角 红/黄/绿)
- 主导航 Tab (Dashboard, Assets, Scan, Vulns, Traffic, Flows)
- 活跃 Tab 紫色圆形背景
- 紫色徽章显示计数
- 右侧: Settings, AI Active 指示, 时间

**代码文件**: `src/ui/navigation.rs`

**验收标准**:
- [ ] 背景色改为浅色
- [ ] macOS 风格按钮渲染
- [ ] Tab 选中样式为紫色圆形
- [ ] 徽章正确显示
- [ ] 编译通过
- [ ] 界面与 DashBoard_A.png 导航栏一致

---

### 步骤 2: 重构 Kanban 看板 (kanban.rs)

**设计目标**:
- 白色卡片背景
- 三列布局: To Do | In Progress | Done
- 列头显示计数和 + 按钮
- 任务卡片高度 70-90px
- 左侧灰色 Tag，右侧彩色优先级 Tag
- 卡片边框 1px 浅灰
- Hover 效果: 边框变紫色

**代码文件**: `src/ui/kanban.rs`

**验收标准**:
- [ ] 白色背景和卡片
- [ ] 三列布局正确
- [ ] 任务卡片样式与设计一致
- [ ] Tag 样式正确 (灰色 + 彩色)
- [ ] 编译通过
- [ ] 界面与 DashBoard_A.png Kanban 部分一致

---

### 步骤 3: 重构 Findings 视图 (findings.rs)

**设计目标**:
- Findings 列表视图 (DashBoard_B.png 参考)
- 统计信息: Total | Critical | High (彩色)
- 搜索框 + Export Report 按钮
- 过滤 Tabs: All, Critical, High, Medium, Low, Info
- Finding 列表项:
  - 左侧彩色圆点 (严重程度)
  - CVE 编号 + 状态徽章
  - 描述文字
  - 时间 + 资产信息
- 列表项边框 1px 浅灰，高度 80-100px

**代码文件**: `src/ui/findings.rs`

**验收标准**:
- [ ] 统计信息正确显示和配色
- [ ] 搜索框 + Export 按钮
- [ ] 过滤 Tab 样式和功能
- [ ] Finding 列表项布局正确
- [ ] 编译通过
- [ ] 界面与 DashBoard_B.png 一致

---

### 步骤 4: 创建 Assets 视图 (assets.rs)

**设计目标** (参考 Assets.png):
- 顶部折叠式标题: "网络拓扑 - 业务层级视图"
- 资产列表项:
  - 左侧绿色圆点 (在线状态)
  - 资产名称 (Telemetry Server)
  - IP 地址
  - 右侧操作按钮 (编辑、删除、展开)
- 列表项背景白色, 边框 1px 浅灰, 圆角 8px

**代码文件**: `src/ui/assets.rs` (新建)

**验收标准**:
- [ ] 新文件创建
- [ ] 折叠式标题实现
- [ ] 资产列表项布局和样式
- [ ] 编译通过
- [ ] 界面与 Assets.png 一致

---

### 步骤 5: 创建 Monitor/Images 视图 (monitor.rs)

**设计目标** (参考 Monitor.png):
- 标题: "容器镜像 & Agent 执行环境" + 计数 + "创建镜像" 按钮
- 容器卡片网格 (3 列):
  - 上部: 终端窗口样式代码块
  - 下部: Agent 信息 + 资源监控
- 进度条: 高度 4-6px, 圆角, 彩色填充
- 卡片高度 220-250px

**代码文件**: `src/ui/monitor.rs` (新建)

**验收标准**:
- [ ] 新文件创建
- [ ] 网格布局 3 列
- [ ] 容器卡片样式和内容
- [ ] 进度条正确渲染
- [ ] 编译通过
- [ ] 界面与 Monitor.png 一致

---

### 步骤 6: 创建 Traffic 视图 (traffic.rs)

**设计目标** (参考 Traffics.png):
- 顶部 TrafficQL 过滤器
- 流量列表表格:
  - 列: #, Time, Asset, Proto, Method, Path, Status, Size, Duration
- 下部三列: Request | Response | Actions
- 代码块背景深灰 (#2d2d2d)

**代码文件**: `src/ui/traffic.rs` (新建)

**验收标准**:
- [ ] 新文件创建
- [ ] 过滤器和查询框
- [ ] 表格布局和样式
- [ ] 三列详情展示
- [ ] 编译通过
- [ ] 界面与 Traffics.png 一致

---

### 步骤 7: 创建 Devices 视图 (devices.rs)

**设计目标** (参考 Devices.png):
- 左侧: 硬件设备列表
  - 搜索框、扫描和添加按钮
  - 设备卡片 (左侧彩色竖线，状态，路径)
- 右侧: 设备详情
  - 设备名称、状态、当前任务
  - 动作按钮 (停止、查看日志)
  - 设备信息卡片 (参数矩形框)
  - 设备状态 (温度条形图)
  - 快速操作按钮

**代码文件**: `src/ui/devices.rs` (新建)

**验收标准**:
- [ ] 新文件创建
- [ ] 两列布局 (列表 + 详情)
- [ ] 设备卡片样式
- [ ] 参数卡片和状态条
- [ ] 编译通过
- [ ] 界面与 Devices.png 一致

---

### 步骤 8: 创建 Vulns 详情视图 (vulns.rs)

**设计目标** (参考 Vulns.png):
- 左列: 漏洞列表
  - 过滤 Tab (Severity, Asset, MITRE)
  - 搜索框
  - 严重程度分组 (CRITICAL, HIGH)
  - 漏洞卡片 (左侧竖线，CVE，评分)
- 中列: 详情面板
  - CVE 号和 CWE
  - 漏洞描述
  - AI Security Analysis (进度条)
  - AI-Generated PoC (代码块)
- 右列: CVE Database
  - CVSS Score
  - Detection Time
  - Asset
  - Quick Actions

**代码文件**: `src/ui/vulns_detail.rs` (新建)

**验收标准**:
- [ ] 新文件创建
- [ ] 三列布局
- [ ] 漏洞列表样式
- [ ] 进度条渲染
- [ ] 代码块样式
- [ ] 编译通过
- [ ] 界面与 Vulns.png 一致

---

### 步骤 9: 更新主应用框架 (app.rs)

**改动目标**:
- 支持多个视图切换 (Dashboard, Assets, Scan, Vulns, Traffic, Flows)
- 根据当前选中 Tab 渲染对应视图
- 导航栏和视图内容的协调

**代码文件**: `src/app.rs`

**验收标准**:
- [ ] 视图切换逻辑
- [ ] Tab 选中状态管理
- [ ] 编译通过
- [ ] 应用整体结构正确

---

### 步骤 10: 全局样式和颜色常量 (styles.rs)

**创建**:
```
src/ui/styles.rs
```

**内容**:
```rust
// 浅色主题颜色
pub const BG_PRIMARY: Hsla = rgb(0xf5f5f5);      // 主背景
pub const BG_SECONDARY: Hsla = rgb(0xffffff);    // 卡片背景
pub const BG_TERTIARY: Hsla = rgb(0xf3f4f6);     // 输入框背景

pub const TEXT_PRIMARY: Hsla = rgb(0x1f2937);    // 主文字
pub const TEXT_SECONDARY: Hsla = rgb(0x6b7280);  // 次要文字
pub const TEXT_TERTIARY: Hsla = rgb(0x9ca3af);   // 灰色文字

pub const BORDER_COLOR: Hsla = rgb(0xe5e7eb);    // 边框
pub const BORDER_LIGHT: Hsla = rgb(0xf3f4f6);    // 浅边框

pub const ACCENT_PURPLE: Hsla = rgb(0x7c3aed);   // 紫色强调
pub const ACCENT_LIGHT_PURPLE: Hsla = rgb(0xa78bfa); // 浅紫

pub const SUCCESS: Hsla = rgb(0x10b981);         // 绿色
pub const ERROR: Hsla = rgb(0xef4444);           // 红色
pub const WARNING: Hsla = rgb(0xfbbf24);         // 黄色
pub const WARNING_ORANGE: Hsla = rgb(0xf97316);  // 橙色
pub const INFO: Hsla = rgb(0x3b82f6);            // 蓝色

// 尺寸
pub const BORDER_RADIUS: f32 = 6.0;
pub const BORDER_RADIUS_LG: f32 = 8.0;
pub const PADDING_SM: f32 = 8.0;
pub const PADDING_MD: f32 = 12.0;
pub const PADDING_LG: f32 = 16.0;
pub const GAP_SM: f32 = 8.0;
pub const GAP_MD: f32 = 12.0;
pub const GAP_LG: f32 = 16.0;
```

**验收标准**:
- [ ] 所有颜色常量定义正确
- [ ] 尺寸系统一致
- [ ] 其他文件正确引用

---

## 编译和验证流程

对于每一步完成后：

```bash
# 1. 编译检查
cargo build

# 2. 格式检查
cargo fmt

# 3. 运行应用
cargo run

# 4. 对比设计图
# 打开 interface_pic 中的对应设计图，对比：
#   - 颜色 (RGB 值)
#   - 布局 (间距、宽度、高度)
#   - 文字 (字体、大小、粗细)
#   - 组件 (边框、圆角、阴影)
#   - 交互 (点击、Hover、状态)

# 5. 提交
git add -A
git commit -m "Step X: 重构 [组件名]，对标 [设计图]"
git push
```

---

## 当前代码状态

| 文件 | 状态 | 需要改动 |
|------|------|---------|
| src/ui/navigation.rs | ✅ 存在 | ⚠️ 改为浅色主题 |
| src/ui/kanban.rs | ✅ 存在 | ⚠️ 改为白色卡片 |
| src/ui/findings.rs | ✅ 存在 | ⚠️ 改为浅色设计 |
| src/ui/agent_panel.rs | ✅ 存在 | ⚠️ 考虑移除或改为浅色 |
| src/ui/assets.rs | ❌ 不存在 | 需要创建 |
| src/ui/monitor.rs | ❌ 不存在 | 需要创建 |
| src/ui/traffic.rs | ❌ 不存在 | 需要创建 |
| src/ui/devices.rs | ❌ 不存在 | 需要创建 |
| src/ui/vulns_detail.rs | ❌ 不存在 | 需要创建 |
| src/ui/styles.rs | ❌ 不存在 | 需要创建 |
| src/app.rs | ✅ 存在 | ⚠️ 更新视图切换逻辑 |

---

## 最终验收清单

在提交前，验证：

- [ ] Step 1-7 代码全部实现
- [ ] Step 8-10 代码全部实现
- [ ] `cargo build` 编译通过
- [ ] `cargo fmt` 格式检查通过
- [ ] 所有 8 个视图都能在应用中切换
- [ ] 每个视图与对应的 Figma 设计图一致：
  - [ ] DashBoard_A / DashBoard_B (Dashboard 视图)
  - [ ] Assets.png (Assets 视图)
  - [ ] Monitor.png (Monitor 视图)
  - [ ] Traffics.png (Traffic 视图)
  - [ ] Devices.png (Devices 视图)
  - [ ] Vulns.png (Vulns 视图)
- [ ] 颜色系统: 浅色主题 + 紫色强调
- [ ] 布局间距: 一致的 Gap 和 Padding 系统
- [ ] 边框圆角: 6-8px 一致
- [ ] 阴影: 轻微阴影一致
- [ ] Tab 切换能工作
- [ ] 搜索、过滤基本功能
- [ ] 所有文件已 git commit 和 push

---

## 预期工作量

- 步骤 1-3: 改现有代码 (3 days)
- 步骤 4-8: 新建文件 (5 days)
- 步骤 9-10: 整合和优化 (2 days)
- **总计**: 8-10 天

---

**任务开始日期**: 2025-12-31  
**预期完成日期**: 2026-01-10  
**任务负责人**: [你的名字]

