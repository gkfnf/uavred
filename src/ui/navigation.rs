use gpui::*;
use gpui::prelude::FluentBuilder;
use ui::{h_flex, label};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NavigationTab {
    Dashboard,
    Assets,
    Scan,
    Vulns,
    Traffic,
    Flows,
}

pub struct NavigationBar {
    active_tab: NavigationTab,
    target: String,
    ai_active: bool,
    vuln_count: usize,
    traffic_count: usize,
}

impl NavigationBar {
    pub fn new(_cx: &mut ViewContext<Self>) -> Self {
        Self {
            active_tab: NavigationTab::Dashboard,
            target: "mavic-3-pro-v2.local".to_string(),
            ai_active: true,
            vuln_count: 2,
            traffic_count: 8,
        }
    }

    pub fn set_target(&mut self, target: String) {
        self.target = target;
    }

    pub fn set_active_tab(&mut self, tab: NavigationTab) {
        self.active_tab = tab;
    }

    fn render_tab(&self, tab: NavigationTab, label_text: &str, icon: Option<&str>, badge_count: Option<usize>) -> impl IntoElement {
        let is_active = self.active_tab == tab;
        
        h_flex()
            .items_center()
            .gap_1()
            .px_3()
            .py_2()
            .rounded_md()
            .when(is_active, |this| {
                this.bg(gpui::rgb(0x6366f1))
            })
            .when(!is_active, |this| {
                this.hover(|style| style.bg(gpui::rgb(0x2d2d2d)))
            })
            .child(label(label_text))
            .when_some(badge_count, |this, count| {
                this.child(
                    div()
                        .px_2()
                        .py_0p5()
                        .rounded_full()
                        .bg(gpui::rgb(0xef4444))
                        .child(
                            label(&count.to_string())
                                .text_size(ui::TextSize::XSmall)
                                .text_color(gpui::white())
                        )
                )
            })
    }

    fn render_target_display(&self) -> impl IntoElement {
        h_flex()
            .items_center()
            .gap_2()
            .px_3()
            .py_1()
            .bg(gpui::rgb(0x252525))
            .rounded_md()
            .child(label("Target:").text_color(gpui::rgb(0x9ca3af)))
            .child(label(&self.target).text_color(gpui::white()))
    }

    fn render_status_indicator(&self) -> impl IntoElement {
        h_flex()
            .items_center()
            .gap_2()
            .px_3()
            .py_1()
            .bg(gpui::rgb(0x252525))
            .rounded_md()
            .child(
                div()
                    .w_2()
                    .h_2()
                    .rounded_full()
                    .bg(if self.ai_active { 
                        gpui::rgb(0x4ade80) 
                    } else { 
                        gpui::rgb(0x6b7280) 
                    })
            )
            .child(
                label(if self.ai_active { "AI Active" } else { "AI Inactive" })
                    .text_color(if self.ai_active { 
                        gpui::rgb(0x4ade80) 
                    } else { 
                        gpui::rgb(0x6b7280) 
                    })
            )
    }
}

impl Render for NavigationBar {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .w_full()
            .items_center()
            .justify_between()
            .px_4()
            .py_2()
            .bg(gpui::rgb(0x1e1e1e))
            .border_b_1()
            .border_color(gpui::rgb(0x2d2d2d))
            .child(
                h_flex()
                    .gap_1()
                    .child(self.render_tab(NavigationTab::Dashboard, "Dashboard", None, None))
                    .child(self.render_tab(NavigationTab::Assets, "Assets", None, None))
                    .child(self.render_tab(NavigationTab::Scan, "Scan", None, None))
                    .child(self.render_tab(NavigationTab::Vulns, "Vulns", None, Some(self.vuln_count)))
                    .child(self.render_tab(NavigationTab::Traffic, "Traffic", None, Some(self.traffic_count)))
                    .child(self.render_tab(NavigationTab::Flows, "Flows", None, None))
            )
            .child(
                h_flex()
                    .gap_2()
                    .child(self.render_target_display())
                    .child(self.render_status_indicator())
            )
    }
}
