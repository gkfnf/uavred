# Vulns UI Agent Instructions

## Scope
This agent is responsible ONLY for `crates/vulns_ui/` - the vulnerability management panel.

## Module Structure
```
vulns_ui/
├── lib.rs          # VulnsPanel - 三栏布局主组件
├── vuln_list.rs    # 左栏 - 漏洞列表 (筛选/搜索/选择)
├── vuln_detail.rs  # 中栏 - 漏洞详情展示
└── cve_panel.rs    # 右栏 - CVE/CVSS/资产关联信息
```

## Key Data Models (from `data::models`)
- `VulnData` - 漏洞完整数据
- `VulnSeverity` - Critical/High/Medium/Low/Info
- `VulnStatus` - Open/Confirmed/InProgress/Resolved/FalsePositive
- `CvssScore` - CVSS 评分数据
- `AiSecurityAnalysis` - AI 分析结果

## Required Imports Pattern
```rust
use gpui::*;
use gpui_component::{h_flex, v_flex, button::Button, label::Label};
use data::models::{VulnData, VulnSeverity, VulnStatus};
use ui::theme::*;  // 所有颜色/间距常量
```

## Theme Constants to Use
- Severity colors: `SEVERITY_CRITICAL`, `SEVERITY_HIGH`, `SEVERITY_MEDIUM`, `SEVERITY_LOW`
- Status colors: `STATUS_SUCCESS`, `STATUS_WARNING`, `STATUS_ERROR`
- Layout: `PADDING_MD`, `SPACING_SM`, `BORDER_RADIUS`

## Component Patterns
1. 使用 `render_*` 函数模式返回 `impl IntoElement`
2. 回调使用闭包参数: `on_select: impl Fn(&mut VulnsPanel, &mut Context<VulnsPanel>, String)`
3. 状态变更后调用 `cx.notify()` 触发重渲染

## DO NOT
- 修改 `ui/theme.rs` - 这是共享文件
- 修改 `data/models.rs` - 这是共享文件
- 创建新的数据模型 - 使用现有的 `data::models`
- 添加新的 crate 依赖 - 除非必要

## Current TODOs
- [ ] 实现资产点击跳转 (line 83-84 in lib.rs)
- [ ] 实现快速操作按钮 (line 86-87 in lib.rs)
- [ ] 添加漏洞状态变更功能
- [ ] 实现批量操作选择
