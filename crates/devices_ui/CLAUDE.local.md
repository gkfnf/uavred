# Devices UI Agent Instructions

## Scope
This agent is responsible ONLY for `crates/devices_ui/` - the hardware device management panel.

## Module Structure
```
devices_ui/
├── lib.rs           # DevicesPanel - 主面板 (左列表 + 右详情)
├── device_list.rs   # 设备列表组件 (卡片渲染)
└── device_detail.rs # 设备详情面板
```

## Key Data Models (from `data::models`)
```rust
// 设备完整信息
pub struct DeviceInfo {
    pub id: String,
    pub name: String,              // "USRP B210", "HackRF One"
    pub path: String,              // "/dev/ttyUSB0"
    pub device_type: String,       // "USRP", "HackRF"
    pub status: DeviceStatus,      // Online/Offline/Busy/Error
    pub icon: DeviceIcon,          // Radio/Antenna/Chip/Network
    pub current_task: Option<String>,
    pub serial_number: String,
    pub firmware_version: String,
    pub temperature: f64,
    pub temperature_status: TemperatureStatus,
    pub capabilities: Vec<DeviceCapability>,
    // ... 更多字段
}

pub enum DeviceStatus { Online, Offline, Busy, Error }
pub enum DeviceIcon { Radio, Antenna, Chip, Network, Unknown }
pub enum TemperatureStatus { Normal, Warning, Critical }
pub enum DeviceCapability { RFTransmit, RFReceive, MAVLink, DJI, SpectrumAnalysis }
```

## Required Imports Pattern
```rust
use gpui::*;
use gpui_component::{
    h_flex, v_flex,
    button::{Button, ButtonVariants},
    input::{Input, InputState, InputEvent},
    label::Label,
    tag::Tag,
    IconName, Sizable,
};
use data::models::{DeviceInfo, DeviceStatus, DeviceCapability};
use ui::theme::*;
```

## Current Code Issues to Fix
```rust
// lib.rs:140 - 硬编码颜色，应使用 theme 常量
.bg(rgb(0xe5e7eb))  // ❌ 改为 rgb(BORDER_COLOR)
.bg(rgb(0xffffff))  // ❌ 改为 rgb(BG_CARD)

// lib.rs:208 - 硬编码颜色
.text_color(rgb(0x1f2937))  // ❌ 改为 rgb(TEXT_PRIMARY)
```

## Device Card Pattern
```rust
fn render_device_card(device: &DeviceInfo, is_selected: bool) -> impl IntoElement {
    let status_color = match device.status {
        DeviceStatus::Online => STATUS_SUCCESS,
        DeviceStatus::Busy => STATUS_AI,
        DeviceStatus::Error => STATUS_ERROR,
        DeviceStatus::Offline => TEXT_MUTED,
    };

    let temp_color = match device.temperature_status {
        TemperatureStatus::Normal => STATUS_SUCCESS,
        TemperatureStatus::Warning => STATUS_WARNING,
        TemperatureStatus::Critical => STATUS_ERROR,
    };

    // ... render card
}
```

## Theme Constants to Use
- Status: `STATUS_SUCCESS` (Online), `STATUS_AI` (Busy), `STATUS_ERROR`, `TEXT_MUTED` (Offline)
- Temperature: `STATUS_SUCCESS`, `STATUS_WARNING`, `STATUS_ERROR`
- Card: `BG_CARD`, `BORDER_COLOR`, `BORDER_RADIUS`
- Selection: `BORDER_FOCUSED`, `ACCENT_BLUE`

## Entity Pattern (已实现)
```rust
pub struct DevicesPanel {
    device_detail: Entity<DeviceDetail>,  // ✓ 正确使用 Entity
    search_input: Entity<InputState>,     // ✓ 正确使用 Entity
    _subscriptions: Vec<Subscription>,    // ✓ 订阅自动清理
}
```

## DO NOT
- 修改共享文件 (`ui/theme.rs`, `data/models.rs`)
- 实现实际的设备扫描逻辑 (属于 `scanner` crate)
- 添加新的设备类型定义

## Current TODOs
- [ ] 替换所有硬编码颜色为 theme 常量
- [ ] 实现"扫描设备"按钮功能 (line 235-238)
- [ ] 实现"添加设备"按钮功能 (line 246-249)
- [ ] 完善设备详情面板内容
- [ ] 添加设备状态实时更新
- [ ] 实现设备能力图标展示
- [ ] 添加温度警告提示
