# UAVRed UI 并行开发任务分解
基于界面设计图，将 UI 开发任务细分为可由多个 GLM4.7 Agent 并行执行的独立任务。
## 项目概述
**技术栈**: Rust + GPUI + gpui-component
**设计风格**: 高信息密度、简洁、Zed 风格
**目标界面**: Dashboard, Assets, Vulns, Traffic, Flows, Devices, Monitor, Settings
## 任务分层策略
### 层级说明
* **L0 基础层**: 共享组件、主题、类型定义 (必须先完成)
* **L1 独立视图层**: 各个独立的 Panel/View 实现 (可并行)
* **L2 组合层**: 视图内的子组件 (可在 L1 完成后并行)
* **L3 集成层**: 最终整合和交互连接 (依赖前置层)
## L0 基础层任务 (串行执行，优先完成)
### T0-1: 创建全局主题和颜色常量
**文件**: `crates/ui/src/theme.rs`
**职责**: 定义所有 UI 使用的颜色、间距、圆角等常量
**输入**: 设计图中的颜色值
**输出**:
```rust
pub const BG_PRIMARY: u32 = 0xf5f5f5;
pub const BG_CARD: u32 = 0xffffff;
pub const TEXT_PRIMARY: u32 = 0x1f2937;
pub const TEXT_SECONDARY: u32 = 0x6b7280;
pub const ACCENT_PURPLE: u32 = 0x7c3aed;
pub const SEVERITY_CRITICAL: u32 = 0xef4444;
pub const SEVERITY_HIGH: u32 = 0xf97316;
pub const SEVERITY_MEDIUM: u32 = 0xfbbf24;
pub const SEVERITY_LOW: u32 = 0x10b981;
pub const BORDER_COLOR: u32 = 0xe5e7eb;
// 尺寸常量
pub const BORDER_RADIUS: f32 = 6.0;
pub const PADDING_SM: f32 = 8.0;
pub const PADDING_MD: f32 = 12.0;
pub const PADDING_LG: f32 = 16.0;
```
**依赖**: 无
**预计时间**: 30分钟
### T0-2: 创建共享数据类型
**文件**: `crates/data/src/models.rs` (扩展)
**职责**: 定义各视图需要的数据结构
**输出**:
* `AssetNode` (资产节点)
* `VulnData` (漏洞数据)
* `TrafficEntry` (流量条目)
* `FlowNode` (工作流节点)
* `DeviceInfo` (设备信息)
* `ContainerStatus` (容器状态)
**依赖**: 无
**预计时间**: 45分钟
## L1 独立视图层任务 (可并行执行)
### T1-1: Assets 网络拓扑视图 - 拓扑画布组件
**文件**: `crates/assets_ui/src/topology_canvas.rs`
**参考设计**: `Assets_Expand.png`
**职责**: 实现网络拓扑图的画布区域
**功能**:
* 5 个区域列 (Z1-Z5) 显示
* 节点圆形绘制 (带风险颜色)
* 节点连接线 (虚线)
* 节点 hover 显示 tooltip
**依赖**: T0-1
**预计时间**: 2小时
### T1-2: Assets 网络拓扑视图 - 节点详情面板
**文件**: `crates/assets_ui/src/node_detail.rs`
**参考设计**: `Assets_Node_Details.png`
**职责**: 实现节点详情面板
**功能**:
* 区域标签 (Z4 飞控设备层)
* 风险评分条
* 开放端口列表
* 检测到的服务列表
* 认证凭证显示
* 业务用途、负责人
* 扫描进度条
* 合规标准标签
**依赖**: T0-1, T0-2
**预计时间**: 2小时
### T1-3: Assets 视图 - 主 Panel 整合
**文件**: `crates/assets_ui/src/lib.rs` (重写)
**参考设计**: `Assets_Expand.png`, `Assets_Clicked_Node.png`
**职责**: 整合拓扑画布和详情面板
**功能**:
* 顶部标题栏 (网络拓扑 - 业务层级视图)
* 风险等级图例
* 统计信息 (8 资产 · 19 连接)
* 底部资产详情栏 (可折叠)
**依赖**: T1-1, T1-2
**预计时间**: 1.5小时
### T1-4: Vulns 漏洞详情视图 - 漏洞列表组件
**文件**: `crates/vulns_ui/src/vuln_list.rs`
**参考设计**: `Vulns.png` 左侧
**职责**: 实现漏洞列表面板
**功能**:
* 过滤 Tab (Severity, Asset, MITRE)
* 搜索框
* 严重程度分组 (CRITICAL, HIGH)
* 漏洞卡片 (左侧彩色竖线、CVE 号、AI 置信度、PoC 标签)
**依赖**: T0-1, T0-2
**预计时间**: 2小时
### T1-5: Vulns 漏洞详情视图 - 详情面板组件
**文件**: `crates/vulns_ui/src/vuln_detail.rs`
**参考设计**: `Vulns.png` 中间
**职责**: 实现漏洞详情面板
**功能**:
* CVE/CWE 标签
* 漏洞描述
* 检测位置信息
* AI Security Analysis 进度条 (Confidence, Exploitability, Impact)
* AI-Generated PoC 代码块
* MITRE ATT&CK 标签
**依赖**: T0-1, T0-2
**预计时间**: 2小时
### T1-6: Vulns 漏洞详情视图 - CVE 数据库面板
**文件**: `crates/vulns_ui/src/cve_panel.rs`
**参考设计**: `Vulns.png` 右侧
**职责**: 实现 CVE Database 信息面板
**功能**:
* CVSS Score 显示
* Detection Time
* Asset 链接
* Quick Actions 按钮
**依赖**: T0-1
**预计时间**: 1小时
### T1-7: Vulns 视图 - 主 Panel 整合
**文件**: `crates/vulns_ui/src/lib.rs` (重写)
**参考设计**: `Vulns.png`
**职责**: 整合三栏布局
**功能**:
* 三栏布局 (列表 | 详情 | CVE)
* 漏洞选择状态管理
* 响应式宽度
**依赖**: T1-4, T1-5, T1-6
**预计时间**: 1小时
### T1-8: Traffic 流量分析视图 - TrafficQL 查询栏
**文件**: `crates/traffic_ui/src/query_bar.rs`
**参考设计**: `Traffics.png` 顶部
**职责**: 实现 TrafficQL 过滤器
**功能**:
* TrafficQL 输入框
* Capturing 切换按钮
* Intercept 切换按钮
* 过滤图标按钮
**依赖**: T0-1
**预计时间**: 1小时
### T1-9: Traffic 流量分析视图 - 流量表格组件
**文件**: `crates/traffic_ui/src/traffic_table.rs`
**参考设计**: `Traffics.png` 中间表格
**职责**: 实现流量列表表格
**功能**:
* 表头 (#, Time, Asset, Proto, Method, Path, Status, Size, Duration)
* 表格行渲染
* 行点击选择
* 异常标记 (anomalies)
* Live 状态指示
**依赖**: T0-1, T0-2
**预计时间**: 2小时
### T1-10: Traffic 流量分析视图 - 请求响应面板
**文件**: `crates/traffic_ui/src/request_response.rs`
**参考设计**: `Traffics.png` 下方左中两栏
**职责**: 实现 Request/Response 代码块
**功能**:
* Request 代码块 (深色背景)
* Response 代码块
* 复制按钮
* 编辑按钮
**依赖**: T0-1
**预计时间**: 1.5小时
### T1-11: Traffic 流量分析视图 - Actions 面板
**文件**: `crates/traffic_ui/src/actions_panel.rs`
**参考设计**: `Traffics.png` 下方右栏
**职责**: 实现操作面板
**功能**:
* Packet Info 卡片
* Anomaly Detected 标记
* Replay 按钮
* FUZZ 按钮
* Export as cURL 按钮
* Statistics 卡片
* Protocols 分布条
**依赖**: T0-1
**预计时间**: 1.5小时
### T1-12: Traffic 视图 - 主 Panel 整合
**文件**: `crates/traffic_ui/src/lib.rs` (重写)
**参考设计**: `Traffics.png`
**职责**: 整合流量分析视图
**功能**:
* 上下布局 (查询栏 + 表格 + 详情)
* 流量选择状态管理
* 三栏底部布局
**依赖**: T1-8, T1-9, T1-10, T1-11
**预计时间**: 1小时
### T1-13: Flows 工作流视图 - 工作流列表组件
**文件**: `crates/flows_ui/src/flow_list.rs`
**参考设计**: `WorkFlows.png` 左侧
**职责**: 实现工作流列表面板
**功能**:
* 新建按钮
* 搜索框
* 过滤下拉框 (层级、类别)
* 工作流分组 (原子工作流、组合工作流、任务工作流)
* 工作流卡片 (步骤数、时间、目标列表)
**依赖**: T0-1, T0-2
**预计时间**: 2小时
### T1-14: Flows 工作流视图 - DAG 画布组件
**文件**: `crates/flows_ui/src/dag_canvas.rs`
**参考设计**: `WorkFlows.png` 右侧
**职责**: 实现 DAG 依赖关系图
**功能**:
* 节点绘制 (矩形，带标题和 ID)
* 连接线 (带箭头，不同颜色表示类型)
* 关键路径高亮
* 节点统计信息 (节点数、最大并行、耗时、成功率、运行次数)
**依赖**: T0-1, T0-2
**预计时间**: 3小时
### T1-15: Flows 工作流视图 - 底部操作栏
**文件**: `crates/flows_ui/src/action_bar.rs`
**参考设计**: `WorkFlows.png` 底部
**职责**: 实现底部操作栏
**功能**:
* 运行工作流按钮 (紫色)
* 模拟测试按钮
* 删除按钮
* 统计计数 (原子、组合、任务)
**依赖**: T0-1
**预计时间**: 45分钟
### T1-16: Flows 视图 - 主 Panel 整合
**文件**: `crates/flows_ui/src/lib.rs` (重写)
**参考设计**: `WorkFlows.png`
**职责**: 整合工作流视图
**功能**:
* 左右布局 (列表 | DAG)
* 工作流选择状态
* 底部操作栏
**依赖**: T1-13, T1-14, T1-15
**预计时间**: 1小时
### T1-17: Devices 设备管理视图 - 设备列表组件
**文件**: `crates/devices_ui/src/device_list.rs` (新建 crate)
**参考设计**: `Devices.png` 左侧
**职责**: 实现设备列表面板
**功能**:
* 标题 (硬件设备) + 连接数徽章
* 搜索框
* 扫描设备按钮
* 添加按钮
* 设备卡片 (图标、名称、状态标签、路径、当前任务)
**依赖**: T0-1, T0-2
**预计时间**: 2小时
### T1-18: Devices 设备管理视图 - 设备详情面板
**文件**: `crates/devices_ui/src/device_detail.rs`
**参考设计**: `Devices.png` 右侧
**职责**: 实现设备详情面板
**功能**:
* 设备名称 + 状态
* 当前任务显示
* 停止任务 / 查看日志 按钮
* 设备信息卡片组 (序列号、固件版本、端口)
* 无线参数卡片组 (频率、采样率、带宽、增益)
* 设备状态 - 温度条
* 快速操作按钮 (配置参数、固件更新)
**依赖**: T0-1, T0-2
**预计时间**: 2.5小时
### T1-19: Devices 视图 - 主 Panel 整合
**文件**: `crates/devices_ui/src/lib.rs` (新建)
**参考设计**: `Devices.png`
**职责**: 整合设备管理视图
**功能**:
* 左右布局 (列表 | 详情)
* 设备选择状态
* 响应式宽度
**依赖**: T1-17, T1-18
**预计时间**: 1小时
### T1-20: Monitor 容器监控视图 - 容器卡片组件
**文件**: `crates/monitor_ui/src/container_card.rs` (新建 crate)
**参考设计**: `Monitor.png`
**职责**: 实现单个容器卡片
**功能**:
* 终端窗口样式头部 (docker exec 命令)
* Agent 信息
* Task 名称
* Running 时间
* CPU/Memory 进度条
* 状态标签 (RUNNING, STOPPED, BUILDING)
* 暴露端口
* 运行时长
**依赖**: T0-1, T0-2
**预计时间**: 2小时
### T1-21: Monitor 视图 - 主 Panel 整合
**文件**: `crates/monitor_ui/src/lib.rs` (新建)
**参考设计**: `Monitor.png`
**职责**: 整合容器监控视图
**功能**:
* 标题栏 (容器镜像 & Agent 执行环境)
* 运行中计数
* 创建镜像按钮
* 网格布局 (3列)
**依赖**: T1-20
**预计时间**: 1小时
### T1-22: Settings 设置视图 - 侧边栏菜单
**文件**: `crates/settings_ui/src/sidebar.rs` (新建 crate)
**参考设计**: `Settings.png` 左侧
**职责**: 实现设置侧边栏
**功能**:
* 搜索框
* 分类列表 (General, Appearance, AI, Security, Network, Workflow, Scanner, Storage, Advanced)
* 分类图标
* 选中状态
**依赖**: T0-1
**预计时间**: 1小时
### T1-23: Settings 设置视图 - 设置内容面板
**文件**: `crates/settings_ui/src/content.rs`
**参考设计**: `Settings.png` 右侧
**职责**: 实现设置内容区
**功能**:
* 标题 + 描述
* Edit in settings.json 按钮
* 设置项组件 (开关、下拉框)
* Auto Update 开关
* Language 下拉框
* Startup View 下拉框
**依赖**: T0-1
**预计时间**: 1.5小时
### T1-24: Settings 视图 - 主 Panel 整合
**文件**: `crates/settings_ui/src/lib.rs` (新建)
**参考设计**: `Settings.png`
**职责**: 整合设置视图
**功能**:
* 左右布局 (侧边栏 | 内容)
* 分类切换状态
**依赖**: T1-22, T1-23
**预计时间**: 45分钟
## L2 组合层任务 (优化和增强)
### T2-1: Dashboard 导航栏增强
**文件**: `crates/dashboard_ui/src/navigation.rs` (如存在) 或 `crates/workspace_ui/src/navigation.rs`
**职责**: 优化顶部导航栏
**功能**:
* Tab 徽章数字更新
* AI Active 状态实时指示
* 时间显示更新
**依赖**: L1 完成
**预计时间**: 1小时
### T2-2: Dashboard Mission Control 优化
**文件**: `crates/dashboard_ui/src/mission_control.rs`
**职责**: 优化看板交互
**功能**:
* 拖拽排序支持 (可选)
* 任务状态更新动画
* 添加任务表单
**依赖**: L1 完成
**预计时间**: 2小时
### T2-3: 共享组件提取
**文件**: `crates/ui/src/components/`
**职责**: 提取可复用组件
**功能**:
* `SeverityIndicator` - 严重程度指示器
* `StatusTag` - 状态标签
* `ProgressBar` - 进度条
* `CodeBlock` - 代码块
* `StatCard` - 统计卡片
**依赖**: L1 完成
**预计时间**: 2小时
## L3 集成层任务 (最终整合)
### T3-1: Workspace 集成所有 Panel
**文件**: `crates/workspace_ui/src/lib.rs`
**职责**: 在 Workspace 中集成所有视图
**功能**:
* 注册所有 Panel 类型
* Tab 切换逻辑
* 默认布局配置
**依赖**: 所有 L1 任务
**预计时间**: 2小时
### T3-2: 主入口集成
**文件**: `crates/uavred/src/main.rs`
**职责**: 启动和初始化
**功能**:
* 初始化所有 crate
* 配置窗口
* 启动应用
**依赖**: T3-1
**预计时间**: 1小时
## 任务依赖图
```warp-runnable-command
L0 (串行)
  T0-1 ──┬──> T0-2
         │
         v
L1 (并行组 A - Assets)
  T1-1 ──┐
  T1-2 ──┼──> T1-3
         │
L1 (并行组 B - Vulns)
  T1-4 ──┐
  T1-5 ──┼──> T1-7
  T1-6 ──┘
L1 (并行组 C - Traffic)
  T1-8 ──┐
  T1-9 ──┤
  T1-10 ─┼──> T1-12
  T1-11 ─┘
L1 (并行组 D - Flows)
  T1-13 ─┐
  T1-14 ─┼──> T1-16
  T1-15 ─┘
L1 (并行组 E - Devices)
  T1-17 ─┬──> T1-19
  T1-18 ─┘
L1 (并行组 F - Monitor)
  T1-20 ──> T1-21
L1 (并行组 G - Settings)
  T1-22 ─┬──> T1-24
  T1-23 ─┘
         │
         v
L2 (并行)
  T2-1, T2-2, T2-3
         │
         v
L3 (串行)
  T3-1 ──> T3-2
```
## 并行执行建议
### 执行波次
**Wave 1** (2 agents):
* Agent 1: T0-1 主题常量
* Agent 2: T0-2 数据类型
**Wave 2** (7 agents 并行):
* Agent A: T1-1, T1-2 (Assets 组件)
* Agent B: T1-4, T1-5, T1-6 (Vulns 组件)
* Agent C: T1-8, T1-9 (Traffic 查询和表格)
* Agent D: T1-10, T1-11 (Traffic 详情面板)
* Agent E: T1-13, T1-14, T1-15 (Flows 组件)
* Agent F: T1-17, T1-18 (Devices 组件)
* Agent G: T1-20 (Monitor 卡片)
**Wave 3** (7 agents 并行):
* Agent A: T1-3 (Assets 整合)
* Agent B: T1-7 (Vulns 整合)
* Agent C: T1-12 (Traffic 整合)
* Agent D: T1-16 (Flows 整合)
* Agent E: T1-19 (Devices 整合)
* Agent F: T1-21 (Monitor 整合)
* Agent G: T1-22, T1-23, T1-24 (Settings 完整)
**Wave 4** (3 agents 并行):
* Agent 1: T2-1 导航栏
* Agent 2: T2-2 Mission Control
* Agent 3: T2-3 共享组件
**Wave 5** (1 agent):
* Agent: T3-1, T3-2 (最终集成)
## 新建 Crate 列表
需要在 `Cargo.toml` 中添加：
```toml
[workspace]
members = [
    # ... 现有 crates
    "crates/devices_ui",
    "crates/monitor_ui",
    "crates/settings_ui",
]
```
每个新 crate 的 `Cargo.toml` 模板：
```toml
[package]
name = "devices_ui"
version.workspace = true
edition.workspace = true
[dependencies]
gpui.workspace = true
gpui-component.workspace = true
data = { path = "../data" }
ui = { path = "../ui" }
```
## 验收标准
每个任务完成后必须满足：
1. `cargo build` 编译通过
2. `cargo fmt` 格式正确
3. 无 `unwrap()` 调用，使用 `?` 传播错误
4. 界面与对应设计图一致
5. 遵循项目 AGENTS.md 中的编码规范
