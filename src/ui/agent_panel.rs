use gpui::*;
use crate::ui::prelude::*;
use crate::ui::divider;

#[derive(Debug, Clone)]
pub enum AgentLogType {
    History,
    Thought,
    Plan,
    Tool,
}

#[derive(Debug, Clone)]
pub struct AgentLog {
    log_type: AgentLogType,
    timestamp: String,
    content: String,
}

impl AgentLog {
    pub fn new(log_type: AgentLogType, timestamp: String, content: String) -> Self {
        Self {
            log_type,
            timestamp,
            content,
        }
    }
}

pub struct AgentPanel {
    agent_name: String,
    mission_objective: String,
    logs: Vec<AgentLog>,
    is_live: bool,
}

impl AgentPanel {
    pub fn new(_cx: &mut ViewContext<Self>) -> Self {
        Self {
            agent_name: "PENLIGENT AGENT".to_string(),
            mission_objective: "Analyze the target drone communication interface for injection vulnerabilities.".to_string(),
            logs: vec![],
            is_live: true,
        }
    }

    pub fn add_log(&mut self, log: AgentLog) {
        self.logs.push(log);
    }

    fn render_header(&self) -> impl IntoElement {
        v_flex()
            .gap_2()
            .pb_3()
            .child(
                h_flex()
                    .items_center()
                    .justify_between()
                    .child(
                        h_flex()
                            .items_center()
                            .gap_2()
                            .child(
                                label("🤖")
                                    .text_size(ui::TextSize::Large)
                            )
                            .child(
                                label(&self.agent_name)
                                    .text_size(ui::TextSize::Large)
                                    .text_color(gpui::rgb(0xa78bfa))
                            )
                    )
                    .child(
                        div()
                            .px_2()
                            .py_1()
                            .rounded_md()
                            .bg(gpui::rgb(0x10b981).opacity(0.2))
                            .child(
                                label("LIVE TRACE")
                                    .text_size(ui::TextSize::XSmall)
                                    .text_color(gpui::rgb(0x10b981))
                            )
                    )
            )
            .child(divider())
    }

    fn render_objective(&self) -> impl IntoElement {
        v_flex()
            .gap_2()
            .p_3()
            .bg(gpui::rgb(0x252525))
            .rounded_md()
            .child(
                label("🎯 MISSION OBJECTIVE")
                    .text_size(ui::TextSize::Small)
                    .text_color(gpui::rgb(0x9ca3af))
            )
            .child(
                label(&self.mission_objective)
                    .text_size(ui::TextSize::Small)
                    .text_color(gpui::white())
            )
    }

    fn render_log_item(&self, log: &AgentLog) -> impl IntoElement {
        let (icon, color, type_label) = match log.log_type {
            AgentLogType::History => ("⚪", gpui::rgb(0x6b7280), "HISTORY"),
            AgentLogType::Thought => ("💭", gpui::rgb(0xa78bfa), "THOUGHT"),
            AgentLogType::Plan => ("📋", gpui::rgb(0xfbbf24), "PLAN"),
            AgentLogType::Tool => ("🔧", gpui::rgb(0x60a5fa), "TOOL"),
        };

        v_flex()
            .gap_2()
            .py_3()
            .border_b_1()
            .border_color(gpui::rgb(0x2d2d2d))
            .child(
                h_flex()
                    .items_center()
                    .gap_2()
                    .child(label(icon))
                    .child(
                        label(type_label)
                            .text_size(ui::TextSize::XSmall)
                            .text_color(color)
                    )
                    .child(
                        label(&log.timestamp)
                            .text_size(ui::TextSize::XSmall)
                            .text_color(gpui::rgb(0x6b7280))
                    )
            )
            .child(
                label(&log.content)
                    .text_size(ui::TextSize::Small)
                    .text_color(gpui::rgb(0xd1d5db))
            )
    }

    fn render_code_execution(&self, command: &str, status: &str) -> impl IntoElement {
        v_flex()
            .gap_1()
            .p_3()
            .bg(gpui::rgb(0x1a1a1a))
            .rounded_md()
            .border_1()
            .border_color(gpui::rgb(0x2d2d2d))
            .child(
                h_flex()
                    .items_center()
                    .justify_between()
                    .child(
                        h_flex()
                            .gap_2()
                            .items_center()
                            .child(label(">_"))
                            .child(
                                label(command)
                                    .text_size(ui::TextSize::Small)
                                    .text_color(gpui::rgb(0x10b981))
                            )
                    )
                    .child(
                        div()
                            .px_2()
                            .py_0p5()
                            .rounded_md()
                            .bg(gpui::rgb(0x10b981).opacity(0.2))
                            .child(
                                label(status)
                                    .text_size(ui::TextSize::XSmall)
                                    .text_color(gpui::rgb(0x10b981))
                            )
                    )
            )
    }
}

impl Render for AgentPanel {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        v_flex()
            .w_96()
            .h_full()
            .p_4()
            .bg(gpui::rgb(0x1e1e1e))
            .border_l_1()
            .border_color(gpui::rgb(0x2d2d2d))
            .gap_3()
            .child(self.render_header())
            .child(self.render_objective())
            .child(
                v_flex()
                    .flex_1()
                    .gap_2()
                    .overflow_y_scroll()
                    .children(self.logs.iter().map(|log| self.render_log_item(log)))
            )
            .when(self.is_live, |this| {
                this.child(self.render_code_execution("curl", "Success"))
            })
    }
}
