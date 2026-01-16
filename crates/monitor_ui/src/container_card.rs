use data::{ContainerExecutionStatus, ContainerStatus};
use gpui::*;
use gpui_component::{Sizable, h_flex, label::Label, tag::Tag, v_flex};
use ui::theme::*;

pub struct ContainerCard {
    container: ContainerStatus,
}

impl ContainerCard {
    pub fn new(container: ContainerStatus, _cx: &mut Context<Self>) -> Self {
        Self { container }
    }
}

impl Render for ContainerCard {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let container = self.container.clone();

        let status_bg = match container.status {
            ContainerExecutionStatus::Running => rgb(STATUS_SUCCESS_BG),
            ContainerExecutionStatus::Stopped => rgb(STATUS_MUTED_BG),
            ContainerExecutionStatus::Building => rgb(STATUS_WARNING_BG),
        };

        let status_color = match container.status {
            ContainerExecutionStatus::Running => rgb(STATUS_SUCCESS),
            ContainerExecutionStatus::Stopped => rgb(TEXT_SECONDARY),
            ContainerExecutionStatus::Building => rgb(STATUS_WARNING),
        };

        let status_text = container.status.to_string();
        let cpu_percentage = container.cpu_usage_percent / 100.0;
        let memory_percentage = container.memory_usage_percent() / 100.0;
        let status_text_clone = status_text.clone();

        v_flex()
            .w_full()
            .max_w(CARD_MAX_WIDTH)
            .min_w(CARD_MIN_WIDTH)
            .border_1()
            .border_color(rgb(BORDER_COLOR))
            .rounded(BORDER_RADIUS)
            .bg(rgb(BG_CARD))
            .overflow_hidden()
            .child(
                v_flex()
                    .w_full()
                    .min_h(px(48.0))
                    .p(PADDING_MD)
                    .bg(rgb(BG_DARK))
                    .child(
                        h_flex()
                            .gap(PADDING_SM)
                            .items_center()
                            .child(
                                Label::new("$")
                                    .text_sm()
                                    .font_family("monospace")
                                    .text_color(rgb(STATUS_SUCCESS)),
                            )
                            .child(
                                Label::new("docker exec -it")
                                    .text_sm()
                                    .font_family("monospace")
                                    .text_color(rgb(TEXT_PRIMARY)),
                            )
                            .child(
                                Label::new(&container.docker_exec_command)
                                    .text_sm()
                                    .font_family("monospace")
                                    .text_color(rgb(TEXT_MUTED)),
                            ),
                    )
                    .child(
                        h_flex()
                            .gap(PADDING_SM)
                            .items_center()
                            .child(Label::new("Agent:").text_sm().text_color(rgb(TEXT_MUTED)))
                            .child(
                                Label::new(&container.agent)
                                    .text_sm()
                                    .text_color(rgb(TEXT_PRIMARY)),
                            ),
                    )
                    .child(
                        h_flex()
                            .gap(PADDING_SM)
                            .items_center()
                            .child(Label::new("Task:").text_sm().text_color(rgb(TEXT_MUTED)))
                            .child(
                                Label::new(&container.task_name)
                                    .text_sm()
                                    .text_color(rgb(TEXT_PRIMARY)),
                            ),
                    )
                    .child(
                        h_flex()
                            .w_full()
                            .justify_between()
                            .items_center()
                            .mt(PADDING_SM)
                            .child(
                                h_flex()
                                    .gap(PADDING_XS)
                                    .items_center()
                                    .child(
                                        Label::new(&format!("[{}]", container.running_duration))
                                            .text_sm()
                                            .text_color(rgb(TEXT_PRIMARY)),
                                    )
                                    .child(
                                        Label::new(status_text_clone.clone())
                                            .text_sm()
                                            .text_color(status_color),
                                    ),
                            )
                            .child(
                                Label::new(format!("{}%", (container.cpu_usage_percent).round()))
                                    .text_sm()
                                    .text_color(rgb(STATUS_WARNING)),
                            ),
                    ),
            )
            .child(
                v_flex()
                    .w_full()
                    .gap(PADDING_MD)
                    .p(PADDING_MD)
                    .child(
                        h_flex().w_full().gap(PADDING_MD).child(
                            Label::new(&format!("Container ID: {}", container.container_id))
                                .text_sm()
                                .text_color(rgb(TEXT_SECONDARY)),
                        ),
                    )
                    .child(
                        v_flex()
                            .gap(PADDING_SM)
                            .child(
                                h_flex()
                                    .w_full()
                                    .justify_between()
                                    .child(
                                        Label::new("CPU").text_xs().text_color(rgb(TEXT_SECONDARY)),
                                    )
                                    .child(
                                        Label::new(format!(
                                            "{}%",
                                            container.cpu_usage_percent.round()
                                        ))
                                        .text_xs()
                                        .text_color(rgb(TEXT_PRIMARY)),
                                    ),
                            )
                            .child(render_progress_bar(cpu_percentage, rgb(STATUS_WARNING))),
                    )
                    .child(
                        v_flex()
                            .gap(PADDING_SM)
                            .child(
                                h_flex()
                                    .w_full()
                                    .justify_between()
                                    .child(
                                        Label::new("Memory")
                                            .text_xs()
                                            .text_color(rgb(TEXT_SECONDARY)),
                                    )
                                    .child(
                                        Label::new(format!(
                                            "{}MB / {}MB",
                                            container.memory_usage_mb, container.memory_limit_mb
                                        ))
                                        .text_xs()
                                        .text_color(rgb(TEXT_PRIMARY)),
                                    ),
                            )
                            .child(render_progress_bar(memory_percentage, rgb(ACCENT_BLUE))),
                    )
                    .child(
                        h_flex()
                            .w_full()
                            .gap(PADDING_SM)
                            .mt(PADDING_SM)
                            .child(
                                Tag::new().small().bg(status_bg).child(
                                    Label::new(status_text).text_xs().text_color(status_color),
                                ),
                            )
                            .child(
                                Label::new(&format!(
                                    "端口: {}",
                                    container.exposed_ports.join(", ")
                                ))
                                .text_sm()
                                .text_color(rgb(TEXT_SECONDARY)),
                            )
                            .child(
                                Label::new(&format!("运行时长: {}", container.running_duration))
                                    .text_sm()
                                    .text_color(rgb(TEXT_SECONDARY)),
                            ),
                    ),
            )
    }
}

fn render_progress_bar(percentage: f64, fill_color: Rgba) -> impl IntoElement {
    let clamped = percentage.clamp(0.0, 1.0);

    h_flex()
        .flex_1()
        .h(px(6.0))
        .bg(rgb(PROGRESS_BG))
        .rounded(BORDER_RADIUS_SM)
        .overflow_hidden()
        .child(
            div()
                .h_full()
                .bg(fill_color)
                .w(DefiniteLength::Fraction(clamped as f32)),
        )
}
