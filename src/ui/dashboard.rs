use gpui::*;
use crate::ui::prelude::*;
use crate::ui::{card, button, badge, divider};
use crate::core::task::TaskStatus;

pub struct Dashboard {
    active_agents: usize,
    total_scans: usize,
    vulnerabilities_found: usize,
}

impl Dashboard {
    pub fn new(_cx: &mut ViewContext<Self>) -> Self {
        Self {
            active_agents: 0,
            total_scans: 0,
            vulnerabilities_found: 0,
        }
    }

    fn render_header(&self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        h_flex()
            .items_center()
            .justify_between()
            .p_4()
            .border_b_1()
            .border_color(gpui::rgb(0x2d2d2d))
            .child(
                label("UAV Red Team")
                    .text_size(ui::TextSize::XLarge)
                    .text_color(gpui::rgb(0xffffff))
            )
            .child(
                h_flex()
                    .gap_2()
                    .child(self.status_badge("Active Agents", self.active_agents))
                    .child(self.status_badge("Total Scans", self.total_scans))
                    .child(self.status_badge("Vulnerabilities", self.vulnerabilities_found))
            )
    }

    fn status_badge(&self, text: &str, value: usize) -> impl IntoElement {
        let badge_text = format!("{}: {}", text, value);
        badge(badge_text.as_str())
            .small()
    }

    fn render_content(&self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        v_flex()
            .p_4()
            .gap_4()
            .child(
                card()
                    .child(
                        v_flex()
                            .gap_2()
                            .child(
                                label("Agent Control Panel")
                                    .text_size(ui::TextSize::Medium)
                            )
                            .child(divider())
                            .child(
                                label("No active agents. Start a new scan to deploy agents.")
                                    .text_size(ui::TextSize::Small)
                                    .text_color(gpui::rgb(0x9ca3af))
                            )
                            .child(
                                h_flex()
                                    .gap_2()
                                    .mt_4()
                                    .child(button("Start Network Scan").small())
                                    .child(button("Analyze Protocol").small())
                                    .child(button("Scan Firmware").small())
                            )
                    )
            )
    }
}

impl Render for Dashboard {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .bg(gpui::rgb(0x1e1e1e))
            .child(self.render_header(cx))
            .child(self.render_content(cx))
    }
}
