// 设备详情面板组件

use gpui::*;
use gpui_component::{
    button::{Button, ButtonVariants as _},
    group_box::GroupBox,
    h_flex,
    label::Label,
    tag::Tag,
    v_flex, IconName, Sizable,
};
use data::models::{DeviceInfo, DeviceStatus, TemperatureStatus};

pub struct DeviceDetail {
    device: Option<DeviceInfo>,
}

impl DeviceDetail {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self { device: None }
    }

    pub fn set_device(&mut self, device: Option<DeviceInfo>) {
        self.device = device;
    }

    fn render_info_card(title: &str, value: &str) -> impl IntoElement {
        GroupBox::new()
            .outline()
            .child(
                v_flex()
                    .gap(px(4.0))
                    .p(px(12.0))
                    .child(
                        Label::new(title)
                            .text_xs()
                            .text_color(rgb(0x6b7280)),
                    )
                    .child(
                        Label::new(value)
                            .text_sm()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgb(0x1f2937)),
                    ),
            )
    }

    fn render_temperature_bar(temperature: f32, status: TemperatureStatus) -> impl IntoElement {
        let (bar_color, text_color) = match status {
            TemperatureStatus::Normal => (rgb(0x10b981), rgb(0x166534)),
            TemperatureStatus::Warning => (rgb(0xfbbf24), rgb(0x92400e)),
            TemperatureStatus::Critical => (rgb(0xef4444), rgb(0x991b1b)),
        };

        let percentage = (temperature / 100.0).min(1.0).max(0.0);

        v_flex()
            .gap(px(4.0))
            .child(
                h_flex()
                    .w_full()
                    .items_center()
                    .justify_between()
                    .child(
                        Label::new("设备温度")
                            .text_xs()
                            .text_color(rgb(0x6b7280)),
                    )
                    .child(
                        Label::new(format!("{:.1}°C", temperature))
                            .text_xs()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(text_color),
                    ),
            )
            .child(
                div()
                    .w_full()
                    .h(px(6.0))
                    .bg(rgb(0xf3f4f6))
                    .rounded(px(3.0))
                    .overflow_hidden()
                    .child(
                        div()
                            .h_full()
                            .w(Percentage::new(percentage * 100.0))
                            .bg(bar_color)
                            .rounded(px(3.0)),
                    ),
            )
    }
}

impl Render for DeviceDetail {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if let Some(ref device) = self.device {
            let device_name = device.name.clone();
            let current_task = device.current_task.clone();
            let task_run_id = device.task_run_id.clone();

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

            v_flex()
                .size_full()
                .gap(px(16.0))
                .p(px(16.0))
                .overflow_y_auto()
                .child(
                    // 设备名称和状态
                    h_flex()
                        .w_full()
                        .items_center()
                        .justify_between()
                        .child(
                            h_flex()
                                .gap(px(12.0))
                                .items_center()
                                .child(
                                    Label::new(device_name)
                                        .text_xl()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(rgb(0x1f2937)),
                                )
                                .child(
                                    Tag::new()
                                        .bg(status_bg)
                                        .text_color(status_text)
                                        .child(Label::new(status_label).text_sm()),
                                ),
                        ),
                )
                .when_some(current_task.clone(), |this, task| {
                    this.child(
                        // 当前任务
                        GroupBox::new()
                            .outline()
                            .child(
                                v_flex()
                                    .gap(px(8.0))
                                    .p(px(12.0))
                                    .child(
                                        Label::new("当前任务")
                                            .text_xs()
                                            .text_color(rgb(0x6b7280)),
                                    )
                                    .child(
                                        Label::new(task)
                                            .text_sm()
                                            .font_weight(FontWeight::MEDIUM)
                                            .text_color(rgb(0x1f2937)),
                                    )
                                    .child(
                                        h_flex()
                                            .gap(px(8.0))
                                            .child(
                                                Button::new("stop-task")
                                                    .ghost()
                                                    .icon(IconName::X)
                                                    .small()
                                                    .label("停止任务")
                                                    .on_click(cx.listener(move |this, _, _, _| {
                                                        // TODO: 实现停止任务功能
                                                        cx.notify();
                                                    })),
                                            )
                                            .child(
                                                Button::new("view-logs")
                                                    .ghost()
                                                    .icon(IconName::FileText)
                                                    .small()
                                                    .label("查看日志")
                                                    .on_click(cx.listener(move |this, _, _, _| {
                                                        // TODO: 实现查看日志功能
                                                        cx.notify();
                                                    })),
                                            ),
                                    ),
                            ),
                    )
                })
                .child(
                    // 设备信息卡片组
                    v_flex()
                        .gap(px(12.0))
                        .child(
                            Label::new("设备信息")
                                .text_sm()
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(rgb(0x1f2937)),
                        )
                        .child(
                            h_flex()
                                .w_full()
                                .gap(px(12.0))
                                .child(
                                    Self::render_info_card("序列号", &device.serial_number)
                                        .flex_1(),
                                )
                                .child(
                                    Self::render_info_card("固件版本", &device.firmware_version)
                                        .flex_1(),
                                )
                                .child(
                                    Self::render_info_card("端口", &device.port).flex_1(),
                                ),
                        ),
                )
                .child(
                    // 无线参数卡片组
                    v_flex()
                        .gap(px(12.0))
                        .child(
                            Label::new("无线参数")
                                .text_sm()
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(rgb(0x1f2937)),
                        )
                        .child(
                            h_flex()
                                .w_full()
                                .gap(px(12.0))
                                .child(
                                    Self::render_info_card("频率", &device.frequency).flex_1(),
                                )
                                .child(
                                    Self::render_info_card("采样率", &device.sampling_rate)
                                        .flex_1(),
                                )
                                .child(
                                    Self::render_info_card("带宽", &device.bandwidth).flex_1(),
                                )
                                .child(
                                    Self::render_info_card("增益", &device.gain).flex_1(),
                                ),
                        ),
                )
                .child(
                    // 设备状态 - 温度条
                    GroupBox::new()
                        .outline()
                        .child(
                            v_flex()
                                .gap(px(8.0))
                                .p(px(12.0))
                                .child(Self::render_temperature_bar(
                                    device.temperature,
                                    device.temperature_status,
                                )),
                        ),
                )
                .child(
                    // 快速操作按钮
                    v_flex()
                        .gap(px(12.0))
                        .child(
                            Label::new("快速操作")
                                .text_sm()
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(rgb(0x1f2937)),
                        )
                        .child(
                            h_flex()
                                .w_full()
                                .gap(px(8.0))
                                .child(
                                    Button::new("configure-params")
                                        .icon(IconName::Settings)
                                        .label("配置参数")
                                        .flex_1()
                                        .on_click(cx.listener(|this, _, _, _| {
                                            // TODO: 实现配置参数功能
                                            cx.notify();
                                        })),
                                )
                                .child(
                                    Button::new("firmware-update")
                                        .icon(IconName::Download)
                                        .label("固件更新")
                                        .flex_1()
                                        .on_click(cx.listener(|this, _, _, _| {
                                            // TODO: 实现固件更新功能
                                            cx.notify();
                                        })),
                                ),
                        ),
                )
        } else {
            // 无设备选中时的占位符
            v_flex()
                .size_full()
                .items_center()
                .justify_center()
                .child(
                    Label::new("请选择一个设备查看详情")
                        .text_base()
                        .text_color(rgb(0x9ca3af)),
                )
        }
    }
}
