// 设备列表组件

use gpui::*;
use gpui_component::{
    button::{Button, ButtonVariants as _},
    group_box::GroupBox,
    h_flex,
    input::Input,
    label::Label,
    tag::Tag,
    v_flex, IconName, Sizable,
};
use data::models::{DeviceInfo, DeviceStatus};

pub struct DeviceList {
    devices: Vec<DeviceInfo>,
    search_query: String,
    selected_device_id: Option<String>,
    on_device_select: Option<Box<dyn Fn(String)>>,
}

impl DeviceList {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            devices: vec![],
            search_query: String::new(),
            selected_device_id: None,
            on_device_select: None,
        }
    }

    pub fn with_devices(mut self, devices: Vec<DeviceInfo>) -> Self {
        self.devices = devices;
        self
    }

    pub fn set_on_device_select(&mut self, callback: impl Fn(String) + 'static) {
        self.on_device_select = Some(Box::new(callback));
    }

    fn filtered_devices(&self) -> Vec<&DeviceInfo> {
        if self.search_query.is_empty() {
            return self.devices.iter().collect();
        }

        let query = self.search_query.to_lowercase();
        self.devices
            .iter()
            .filter(|device| {
                device.name.to_lowercase().contains(&query)
                    || device.path.to_lowercase().contains(&query)
                    || device.device_type.to_lowercase().contains(&query)
            })
            .collect()
    }

    pub fn render_device_card<T: 'static>(
        cx: &mut Context<T>,
        device: &DeviceInfo,
        is_selected: bool,
        on_select: impl Fn(&mut T, &mut Context<T>, String) + 'static,
    ) -> impl IntoElement {
        let device_id = device.id.clone();
        let device_name = device.name.clone();
        let device_path = device.path.clone();
        let current_task = device.current_task.clone();

        // 状态颜色
        let (status_bg, status_text) = match device.status {
            DeviceStatus::Online => (rgb(0xdcfce7), rgb(0x166534)),
            DeviceStatus::Offline => (rgb(0xf3f4f6), rgb(0x6b7280)),
            DeviceStatus::Busy => (rgb(0xfef3c7), rgb(0x92400e)),
            DeviceStatus::Error => (rgb(0xfee2e2), rgb(0x991b1b)),
            DeviceStatus::Scanning => (rgb(0xdbeafe), rgb(0x1e40af)),
        };

        let status_label = match device.status {
            DeviceStatus::Online => "在线",
            DeviceStatus::Offline => "离线",
            DeviceStatus::Busy => "忙碌",
            DeviceStatus::Error => "错误",
            DeviceStatus::Scanning => "扫描中",
        };

        // 设备图标颜色（根据设备类型）
        let icon_color = match device.icon {
            data::models::DeviceIcon::Antenna => rgb(0x3b82f6),
            data::models::DeviceIcon::Radio => rgb(0x7c3aed),
            data::models::DeviceIcon::Satellite => rgb(0x10b981),
            data::models::DeviceIcon::Usb => rgb(0xf59e0b),
            data::models::DeviceIcon::Network => rgb(0x6366f1),
        };

        let mut card = GroupBox::new()
            .outline()
            .child(
                v_flex()
                    .gap(px(8.0))
                    .p(px(12.0))
                    .child(
                        h_flex()
                            .gap(px(8.0))
                            .items_center()
                            .child(
                                // 左侧彩色竖线
                                div()
                                    .w(px(4.0))
                                    .h(px(48.0))
                                    .bg(icon_color)
                                    .rounded(px(2.0)),
                            )
                            .child(
                                v_flex()
                                    .gap(px(4.0))
                                    .flex_1()
                                    .child(
                                        h_flex()
                                            .gap(px(8.0))
                                            .items_center()
                                            .child(
                                                Label::new(device_name)
                                                    .text_sm()
                                                    .font_weight(FontWeight::MEDIUM)
                                                    .text_color(rgb(0x1f2937)),
                                            )
                                            .child(
                                                Tag::new()
                                                    .small()
                                                    .bg(status_bg)
                                                    .text_color(status_text)
                                                    .child(Label::new(status_label).text_xs()),
                                            ),
                                    )
                                    .child(
                                        Label::new(device_path)
                                            .text_xs()
                                            .text_color(rgb(0x6b7280)),
                                    )
                                    .when_some(current_task, |this, task| {
                                        this.child(
                                            Label::new(format!("当前任务: {}", task))
                                                .text_xs()
                                                .text_color(rgb(0x7c3aed)),
                                        )
                                    }),
                            ),
                    ),
            );

        // 选中状态：紫色边框
        if is_selected {
            card = card.border(px(2.0)).border_color(rgb(0x7c3aed));
        }

        div()
            .id(("device-card", device_id.clone()))
            .w_full()
            .cursor_pointer()
            .child(card)
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(move |this: &mut T, _, _, cx: &mut Context<T>| {
                    on_select(this, cx, device_id.clone());
                }),
            )
    }
}

impl Render for DeviceList {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let search_query = self.search_query.clone();
        let filtered_devices = self.filtered_devices();
        let selected_id = self.selected_device_id.clone();

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
                                self.render_device_card(
                                    cx,
                                    device,
                                    is_selected,
                                    |this, cx, id| {
                                        this.selected_device_id = Some(id.clone());
                                        if let Some(ref callback) = this.on_device_select {
                                            callback(id);
                                        }
                                        cx.notify();
                                    },
                                )
                            })
                            .collect::<Vec<_>>(),
                    ),
            )
    }
}
