use data::{ContainerExecutionStatus, ContainerStatus};
use gpui::*;
use gpui_component::{h_flex, v_flex, label::Label, tag::Tag, Sizable};

/// Image/Container card component
pub struct ImageCard {
    container: ContainerStatus,
}

impl ImageCard {
    pub fn new(container: ContainerStatus, _cx: &mut Context<Self>) -> Self {
        Self { container }
    }

    pub fn container(&self) -> &ContainerStatus {
        &self.container
    }
}

impl Render for ImageCard {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let container = self.container.clone();

        // Status colors
        let (status_bg, status_text_color, status_label) = match container.status {
            ContainerExecutionStatus::Running => (
                rgb(0x10b981),
                rgb(0xffffff),
                "RUNNING"
            ),
            ContainerExecutionStatus::Stopped => (
                rgb(0x9ca3af),
                rgb(0xffffff),
                "STOPPED"
            ),
            ContainerExecutionStatus::Building => (
                rgb(0xf59e0b),
                rgb(0xffffff),
                "BUILDING"
            ),
        };

        let cpu_percentage = container.cpu_usage_percent / 100.0;
        let memory_percentage = container.memory_usage_percent() / 100.0;
        
        // CPU color based on usage
        let cpu_bar_color = if container.cpu_usage_percent < 50.0 {
            rgb(0xf59e0b) // Yellow
        } else if container.cpu_usage_percent < 80.0 {
            rgb(0xf97316) // Orange
        } else {
            rgb(0xef4444) // Red
        };

        // Memory color based on usage
        let memory_bar_color = if container.memory_usage_percent() < 60.0 {
            rgb(0x3b82f6) // Blue
        } else if container.memory_usage_percent() < 85.0 {
            rgb(0xf59e0b) // Yellow
        } else {
            rgb(0xef4444) // Red
        };

        v_flex()
            .w(px(380.0))
            .min_w(px(340.0))
            .flex_1()
            .min_h(px(420.0))
            .max_h(px(480.0))
            .bg(rgb(0xffffff))
            .rounded(px(12.0))
            .border_1()
            .border_color(rgb(0xe5e7eb))
            .overflow_hidden()
            // Terminal header section (dark)
            .child(
                v_flex()
                    .w_full()
                    .h(px(180.0))
                    .bg(rgb(0x1e293b))
                    .rounded_t(px(12.0))
                    .p(px(16.0))
                    .child(
                        // Status badge at top right
                        h_flex()
                            .w_full()
                            .justify_end()
                            .child(
                                div()
                                    .px(px(10.0))
                                    .py(px(4.0))
                                    .rounded_full()
                                    .bg(status_bg)
                                    .child(
                                        Label::new(status_label)
                                            .text_xs()
                                            .font_weight(FontWeight::MEDIUM)
                                            .text_color(status_text_color)
                                    )
                            )
                    )
                    .child(
                        // Docker exec command line
                        h_flex()
                            .gap(px(6.0))
                            .items_center()
                            .child(
                                Label::new("$")
                                    .text_sm()
                                    .font_family("monospace")
                                    .text_color(rgb(0x34d399))
                            )
                            .child(
                                Label::new("docker exec -it")
                                    .text_sm()
                                    .font_family("monospace")
                                    .text_color(rgb(0xe2e8f0))
                            )
                            .child(
                                Label::new(&container.docker_exec_command)
                                    .text_sm()
                                    .font_family("monospace")
                                    .text_color(rgb(0x94a3b8))
                            )
                    )
                    .child(
                        // Agent info
                        h_flex()
                            .gap(px(6.0))
                            .items_center()
                            .mt(px(8.0))
                            .child(
                                Label::new("Agent:")
                                    .text_sm()
                                    .font_family("monospace")
                                    .text_color(rgb(0x64748b))
                            )
                            .child(
                                Label::new(&container.agent)
                                    .text_sm()
                                    .font_family("monospace")
                                    .text_color(rgb(0xa855f7))
                            )
                    )
                    .child(
                        // Task info
                        h_flex()
                            .gap(px(6.0))
                            .items_center()
                            .mt(px(4.0))
                            .child(
                                Label::new("Task:")
                                    .text_sm()
                                    .font_family("monospace")
                                    .text_color(rgb(0x64748b))
                            )
                            .child(
                                Label::new(&container.task_name)
                                    .text_sm()
                                    .font_family("monospace")
                                    .text_color(rgb(0xa855f7))
                            )
                    )
                    .child(
                        // Duration and status at bottom
                        h_flex()
                            .w_full()
                            .mt_auto()
                            .pt(px(16.0))
                            .justify_between()
                            .items_center()
                            .child(
                                h_flex()
                                    .gap(px(8.0))
                                    .items_center()
                                    .child(
                                        Label::new(format!("[{}]", container.running_duration))
                                            .text_sm()
                                            .font_family("monospace")
                                            .text_color(rgb(0x94a3b8))
                                    )
                                    .child(
                                        Label::new("Running...")
                                            .text_sm()
                                            .font_family("monospace")
                                            .text_color(rgb(0x64748b))
                                    )
                            )
                            .child(
                                h_flex()
                                    .gap(px(4.0))
                                    .items_center()
                                    .child(
                                        Label::new("⚡")
                                            .text_sm()
                                    )
                                    .child(
                                        Label::new(format!("{:.0}%", container.cpu_usage_percent))
                                            .text_sm()
                                            .font_family("monospace")
                                            .text_color(rgb(0x34d399))
                                    )
                            )
                    )
            )
            // Info section (light)
            .child(
                v_flex()
                    .w_full()
                    .flex_1()
                    .p(px(16.0))
                    .gap(px(12.0))
                    .child(
                        // Image name and copy button
                        h_flex()
                            .w_full()
                            .justify_between()
                            .items_center()
                            .child(
                                v_flex()
                                    .gap(px(2.0))
                                    .child(
                                        Label::new(format!("ai-pentest-agent"))
                                            .text_base()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(rgb(0x1f2937))
                                    )
                                    .child(
                                        Label::new("v2.3.1")
                                            .text_sm()
                                            .text_color(rgb(0x6b7280))
                                    )
                            )
                            .child(
                                div()
                                    .cursor_pointer()
                                    .child(
                                        Label::new("📋")
                                            .text_sm()
                                    )
                            )
                    )
                    .child(
                        // Agent name badge
                        v_flex()
                            .gap(px(4.0))
                            .child(
                                Label::new("Agent")
                                    .text_xs()
                                    .text_color(rgb(0x6b7280))
                            )
                            .child(
                                div()
                                    .px(px(12.0))
                                    .py(px(6.0))
                                    .rounded(px(6.0))
                                    .border_1()
                                    .border_color(rgb(0xe5e7eb))
                                    .bg(rgb(0xf9fafb))
                                    .child(
                                        Label::new(&container.agent)
                                            .text_sm()
                                            .text_color(rgb(0x7c3aed))
                                    )
                            )
                    )
                    .child(
                        // Current task
                        v_flex()
                            .gap(px(4.0))
                            .child(
                                Label::new("当前任务")
                                    .text_xs()
                                    .text_color(rgb(0x6b7280))
                            )
                            .child(
                                Label::new(&container.task_name)
                                    .text_sm()
                                    .text_color(rgb(0x374151))
                            )
                    )
                    // CPU Progress bar
                    .child(
                        v_flex()
                            .gap(px(6.0))
                            .child(
                                h_flex()
                                    .w_full()
                                    .justify_between()
                                    .child(
                                        Label::new("CPU")
                                            .text_xs()
                                            .text_color(rgb(0x6b7280))
                                    )
                                    .child(
                                        Label::new(format!("{:.0}%", container.cpu_usage_percent))
                                            .text_xs()
                                            .text_color(rgb(0x374151))
                                    )
                            )
                            .child(render_progress_bar(cpu_percentage as f32, cpu_bar_color))
                    )
                    // Memory Progress bar
                    .child(
                        v_flex()
                            .gap(px(6.0))
                            .child(
                                h_flex()
                                    .w_full()
                                    .justify_between()
                                    .child(
                                        Label::new("Memory")
                                            .text_xs()
                                            .text_color(rgb(0x6b7280))
                                    )
                                    .child(
                                        Label::new(format!("{:.0}%", container.memory_usage_percent()))
                                            .text_xs()
                                            .text_color(rgb(0x374151))
                                    )
                            )
                            .child(render_progress_bar(memory_percentage as f32, memory_bar_color))
                    )
                    // Exposed ports
                    .child(
                        v_flex()
                            .gap(px(4.0))
                            .mt(px(4.0))
                            .child(
                                Label::new("暴露端口")
                                    .text_xs()
                                    .text_color(rgb(0x6b7280))
                            )
                            .child(
                                h_flex()
                                    .gap(px(8.0))
                                    .children(container.exposed_ports.iter().map(|port| {
                                        div()
                                            .px(px(8.0))
                                            .py(px(4.0))
                                            .rounded(px(4.0))
                                            .bg(rgb(0xf3f4f6))
                                            .border_1()
                                            .border_color(rgb(0xe5e7eb))
                                            .child(
                                                Label::new(port)
                                                    .text_xs()
                                                    .text_color(rgb(0x4b5563))
                                            )
                                    }))
                            )
                    )
                    // Running duration
                    .child(
                        h_flex()
                            .w_full()
                            .justify_between()
                            .mt(px(4.0))
                            .child(
                                Label::new("运行时长")
                                    .text_xs()
                                    .text_color(rgb(0x6b7280))
                            )
                            .child(
                                Label::new(&container.running_duration)
                                    .text_xs()
                                    .text_color(rgb(0x374151))
                            )
                    )
            )
    }
}

fn render_progress_bar(percentage: f32, fill_color: Rgba) -> impl IntoElement {
    let clamped = percentage.clamp(0.0, 1.0);

    h_flex()
        .flex_1()
        .h(px(6.0))
        .bg(rgb(0xe5e7eb))
        .rounded(px(3.0))
        .overflow_hidden()
        .child(
            div()
                .h_full()
                .bg(fill_color)
                .w(DefiniteLength::Fraction(clamped))
        )
}
