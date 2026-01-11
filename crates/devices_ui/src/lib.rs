// Devices 设备管理视图

use gpui::*;
use gpui_component::h_flex;

mod device_list;
mod device_detail;

pub use device_list::DeviceList;
pub use device_detail::DeviceDetail;

use data::models::DeviceInfo;

/// Devices 面板 - 整合设备列表和详情面板
pub struct DevicesPanel {
    device_detail: Entity<DeviceDetail>,
    selected_device_id: Option<String>,
    devices: Vec<DeviceInfo>,
    search_query: String,
}

impl DevicesPanel {
    pub fn new(cx: &mut Context<Self>) -> Self {
        // 创建示例设备数据
        let devices = vec![
            DeviceInfo {
                id: "device-1".to_string(),
                name: "USRP B210".to_string(),
                path: "/dev/ttyUSB0".to_string(),
                device_type: "USRP".to_string(),
                status: data::models::DeviceStatus::Online,
                connection_status: "已连接".to_string(),
                icon: data::models::DeviceIcon::Radio,
                current_task: Some("频谱扫描".to_string()),
                task_run_id: Some("task-123".to_string()),
                serial_number: "SN-2024-001".to_string(),
                firmware_version: "v2.1.0".to_string(),
                port: "USB 3.0".to_string(),
                frequency: "2.4 GHz".to_string(),
                sampling_rate: "20 MS/s".to_string(),
                bandwidth: "20 MHz".to_string(),
                gain: "30 dB".to_string(),
                temperature: 45.5,
                temperature_status: data::models::TemperatureStatus::Normal,
                last_seen: Some("2024-01-15 10:30:00".to_string()),
                uptime: Some("2h 15m".to_string()),
                protocol: "MAVLink".to_string(),
                capabilities: vec![
                    data::models::DeviceCapability::RFTransmit,
                    data::models::DeviceCapability::RFReceive,
                    data::models::DeviceCapability::MAVLink,
                ],
            },
            DeviceInfo {
                id: "device-2".to_string(),
                name: "HackRF One".to_string(),
                path: "/dev/ttyUSB1".to_string(),
                device_type: "HackRF".to_string(),
                status: data::models::DeviceStatus::Busy,
                connection_status: "已连接".to_string(),
                icon: data::models::DeviceIcon::Antenna,
                current_task: None,
                task_run_id: None,
                serial_number: "SN-2024-002".to_string(),
                firmware_version: "v1.0.0".to_string(),
                port: "USB 2.0".to_string(),
                frequency: "1.8 GHz".to_string(),
                sampling_rate: "20 MS/s".to_string(),
                bandwidth: "20 MHz".to_string(),
                gain: "47 dB".to_string(),
                temperature: 52.3,
                temperature_status: data::models::TemperatureStatus::Warning,
                last_seen: Some("2024-01-15 10:25:00".to_string()),
                uptime: Some("1h 30m".to_string()),
                protocol: "DJI".to_string(),
                capabilities: vec![
                    data::models::DeviceCapability::RFReceive,
                    data::models::DeviceCapability::SpectrumAnalysis,
                    data::models::DeviceCapability::DJI,
                ],
            },
        ];

        let device_detail = cx.new(|cx| DeviceDetail::new(cx));

        Self {
            device_detail,
            selected_device_id: None,
            devices,
            search_query: String::new(),
        }
    }

    fn update_selected_device(&mut self, device_id: String, cx: &mut Context<Self>) {
        self.selected_device_id = Some(device_id.clone());
        
        let device = self.devices.iter().find(|d| d.id == device_id).cloned();
        
        self.device_detail.update(cx, |detail, _| {
            detail.set_device(device);
        });
        
        cx.notify();
    }
}

impl Render for DevicesPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let selected_id = self.selected_device_id.clone();

        h_flex()
            .size_full()
            .gap(px(1.0))
            .bg(rgb(0xe5e7eb))
            .child(
                // 左侧：设备列表
                div()
                    .w(px(320.0))
                    .h_full()
                    .bg(rgb(0xffffff))
                    .child(self.render_device_list(cx, selected_id)),
            )
            .child(
                // 右侧：设备详情
                div()
                    .flex_1()
                    .h_full()
                    .bg(rgb(0xffffff))
                    .child(self.device_detail.clone().into_any_element()),
            )
    }
}

impl DevicesPanel {
    fn render_device_list(
        &mut self,
        cx: &mut Context<Self>,
        selected_id: Option<String>,
    ) -> impl IntoElement {
        use gpui_component::{
            button::{Button, ButtonVariants as _},
            group_box::GroupBox,
            h_flex,
            input::Input,
            label::Label,
            tag::Tag,
            v_flex, IconName, Sizable,
        };
        use data::models::DeviceStatus;

        let search_query = self.search_query.clone();
        let filtered_devices: Vec<_> = if search_query.is_empty() {
            self.devices.iter().collect()
        } else {
            let query = search_query.to_lowercase();
            self.devices
                .iter()
                .filter(|device| {
                    device.name.to_lowercase().contains(&query)
                        || device.path.to_lowercase().contains(&query)
                        || device.device_type.to_lowercase().contains(&query)
                })
                .collect()
        };

        v_flex()
            .size_full()
            .gap(px(12.0))
            .p(px(16.0))
            .child(
                // 标题栏
                h_flex()
                    .w_full()
                    .items_center()
                    .justify_between()
                    .child(
                        h_flex()
                            .gap(px(8.0))
                            .items_center()
                            .child(
                                Label::new("硬件设备")
                                    .text_lg()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(rgb(0x1f2937)),
                            )
                            .child(
                                Tag::new()
                                    .small()
                                    .bg(rgb(0xf3f4f6))
                                    .text_color(rgb(0x6b7280))
                                    .child(
                                        Label::new(format!("{}", self.devices.len()))
                                            .text_xs(),
                                    ),
                            ),
                    ),
            )
            .child(
                // 搜索框和操作按钮
                h_flex()
                    .w_full()
                    .gap(px(8.0))
                    .items_center()
                    .child(
                        Input::new("device-search")
                            .placeholder("搜索设备...")
                            .flex_1()
                            .value(search_query.clone())
                            .on_change(cx.listener(move |this, _, value, _| {
                                this.search_query = value;
                                cx.notify();
                            })),
                    )
                    .child(
                        Button::new("scan-devices")
                            .ghost()
                            .icon(IconName::Search)
                            .small()
                            .label("扫描设备")
                            .on_click(cx.listener(|this, _, _, _| {
                                // TODO: 实现扫描设备功能
                                cx.notify();
                            })),
                    )
                    .child(
                        Button::new("add-device")
                            .ghost()
                            .icon(IconName::Plus)
                            .small()
                            .label("添加")
                            .on_click(cx.listener(|this, _, _, _| {
                                // TODO: 实现添加设备功能
                                cx.notify();
                            })),
                    ),
            )
            .child(
                // 设备列表
                v_flex()
                    .flex_1()
                    .gap(px(8.0))
                    .overflow_y_auto()
                    .children(
                        filtered_devices
                            .into_iter()
                            .map(|device| {
                                let device_id = device.id.clone();
                                let is_selected = selected_id.as_ref() == Some(&device_id);
                                DeviceList::render_device_card(
                                    cx,
                                    device,
                                    is_selected,
                                    |this, cx, id| {
                                        this.selected_device_id = Some(id.clone());
                                        let device = this.devices.iter().find(|d| d.id == id).cloned();
                                        this.device_detail.update(cx, |detail, _| {
                                            detail.set_device(device);
                                        });
                                        cx.notify();
                                    },
                                )
                            })
                            .collect::<Vec<_>>(),
                    ),
            )
    }
}
