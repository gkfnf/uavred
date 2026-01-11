// 容器卡片组件

use gpui::*;
use gpui_component::{
    group_box::{GroupBox, GroupBoxVariants as _},
    h_flex,
    label::Label,
    tag::Tag,
    v_flex, Sizable,
};
use data::{ContainerExecutionStatus, ContainerStatus};
use ui::theme::*;

/// 渲染容器卡片
pub fn render_container_card<T: 'static>(
    cx: &mut Context<T>,
    container: &ContainerStatus,
) -> impl IntoElement {
    let status_color = match container.status {
        ContainerExecutionStatus::Running => rgb(STATUS_SUCCESS),
        ContainerExecutionStatus::Stopped => rgb(TEXT_SECONDARY),
        ContainerExecutionStatus::Building => rgb(STATUS_WARNING),
    };

    let status_bg = match container.status {
        ContainerExecutionStatus::Running => rgb(0xf0fdf4),
        ContainerExecutionStatus::Stopped => rgb(0xf3f4f6),
        ContainerExecutionStatus::Building => rgb(0xfffbeb),
    };

    // CPU 进度条颜色（橙色）
    let cpu_color = rgb(0xf97316);
    // Memory 进度条颜色（蓝色）
    let memory_color = rgb(0x2563eb);

    GroupBox::new()
        .outline()
        .child(
            v_flex()
                .gap(px(0.0))
                .w_full()
                .h(px(240.0))
                // 终端窗口样式头部
                .child(
                    v_flex()
                        .gap(px(4.0))
                        .p(px(12.0))
                        .bg(rgb(0x1f2937))
                        .rounded_t(px(8.0))
                        .child(
                            Label::new(format!("$ {}", container.docker_exec_command))
                                .text_sm()
                                .font_family("monospace")
                                .text_color(rgb(0x10b981)),
                        )
                        .child(
                            Label::new(format!("Agent: {}", container.agent))
                                .text_xs()
                                .font_family("monospace")
                                .text_color(rgb(0xe5e7eb)),
                        )
                        .child(
                            Label::new(format!("Task: {}", container.task_name))
                                .text_xs()
                                .font_family("monospace")
                                .text_color(rgb(0xe5e7eb)),
                        )
                        .child(
                            h_flex()
                                .gap(px(8.0))
                                .items_center()
                                .child(
                                    Label::new(format!("[{}] {}", container.running_duration, 
                                        match container.status {
                                            ContainerExecutionStatus::Running => "Running...",
                                            ContainerExecutionStatus::Stopped => "Stopped",
                                            ContainerExecutionStatus::Building => "Building...",
                                        }))
                                        .text_xs()
                                        .font_family("monospace")
                                        .text_color(rgb(0xe5e7eb)),
                                )
                                .child(
                                    Label::new(format!("↗ {:.0}%", container.cpu_usage_percent))
                                        .text_xs()
                                        .font_family("monospace")
                                        .text_color(rgb(0x10b981)),
                                ),
                        ),
                )
                // 卡片主体内容
                .child(
                    v_flex()
                        .gap(px(12.0))
                        .p(px(12.0))
                        .bg(rgb(BG_CARD))
                        .rounded_b(px(8.0))
                        // Agent 信息区域
                        .child(
                            v_flex()
                                .gap(px(4.0))
                                .child(
                                    Label::new(&container.container_id)
                                        .text_sm()
                                        .font_weight(FontWeight::MEDIUM)
                                        .text_color(rgb(TEXT_PRIMARY)),
                                )
                                .child(
                                    Label::new(&container.agent)
                                        .text_xs()
                                        .text_color(rgb(TEXT_SECONDARY)),
                                )
                                .child(
                                    Label::new(&container.task_name)
                                        .text_xs()
                                        .text_color(rgb(TEXT_SECONDARY)),
                                ),
                        )
                        // CPU 和 Memory 进度条
                        .child(
                            v_flex()
                                .gap(px(8.0))
                                .child(
                                    h_flex()
                                        .gap(px(8.0))
                                        .items_center()
                                        .w_full()
                                        .child(
                                            Label::new("CPU")
                                                .text_xs()
                                                .w(px(60.0))
                                                .text_color(rgb(TEXT_SECONDARY)),
                                        )
                                        .child(
                                            Label::new(format!("{:.0}%", container.cpu_usage_percent))
                                                .text_xs()
                                                .w(px(40.0))
                                                .text_color(rgb(TEXT_PRIMARY)),
                                        )
                                        .child(
                                            render_progress_bar(
                                                container.cpu_usage_percent / 100.0,
                                                cpu_color,
                                            ),
                                        ),
                                )
                                .child(
                                    h_flex()
                                        .gap(px(8.0))
                                        .items_center()
                                        .w_full()
                                        .child(
                                            Label::new("Memory")
                                                .text_xs()
                                                .w(px(60.0))
                                                .text_color(rgb(TEXT_SECONDARY)),
                                        )
                                        .child(
                                            Label::new(format!("{:.0}%", container.memory_usage_percent()))
                                                .text_xs()
                                                .w(px(40.0))
                                                .text_color(rgb(TEXT_PRIMARY)),
                                        )
                                        .child(
                                            render_progress_bar(
                                                container.memory_usage_percent() / 100.0,
                                                memory_color,
                                            ),
                                        ),
                                ),
                        )
                        // 状态标签和端口信息
                        .child(
                            v_flex()
                                .gap(px(8.0))
                                .child(
                                    Tag::new()
                                        .small()
                                        .bg(status_bg)
                                        .text_color(status_color)
                                        .child(
                                            Label::new(container.status.to_string())
                                                .text_xs(),
                                        ),
                                )
                                .when(!container.exposed_ports.is_empty(), |this| {
                                    this.child(
                                        h_flex()
                                            .gap(px(4.0))
                                            .items_center()
                                            .child(
                                                Label::new("脚本端口: ")
                                                    .text_xs()
                                                    .text_color(rgb(TEXT_SECONDARY)),
                                            )
                                            .child(
                                                Label::new(container.exposed_ports.join(", "))
                                                    .text_xs()
                                                    .text_color(rgb(TEXT_PRIMARY)),
                                            ),
                                    )
                                })
                                .child(
                                    h_flex()
                                        .gap(px(4.0))
                                        .items_center()
                                        .child(
                                            Label::new("运行时长: ")
                                                .text_xs()
                                                .text_color(rgb(TEXT_SECONDARY)),
                                        )
                                        .child(
                                            Label::new(&container.running_duration)
                                                .text_xs()
                                                .text_color(rgb(TEXT_PRIMARY)),
                                        ),
                                ),
                        ),
                ),
        )
}

/// 渲染进度条
fn render_progress_bar(percentage: f64, fill_color: Rgb) -> impl IntoElement {
    let clamped_percentage = percentage.min(1.0).max(0.0);
    
    // 使用背景容器和填充层，通过 flex 布局实现
    h_flex()
        .flex_1()
        .h(px(6.0))
        .bg(rgb(0xe5e7eb))
        .rounded(px(3.0))
        .overflow_hidden()
        .child(
            div()
                .flex_grow()
                .h_full()
                .bg(fill_color)
                .when(clamped_percentage > 0.0, |this| {
                    this.w(DefiniteLength::Fraction(clamped_percentage))
                })
        )
        .child(
            div()
                .flex_grow()
                .h_full()
                .bg(rgb(0xe5e7eb))
                .when(clamped_percentage < 1.0, |this| {
                    this.w(DefiniteLength::Fraction(1.0 - clamped_percentage))
                })
        )
}
