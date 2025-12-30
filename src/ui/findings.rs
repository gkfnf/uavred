use gpui::*;
use crate::ui::prelude::*;
use crate::ui::{badge, card, divider, button, input};
use crate::scanner::{Finding, Severity};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FindingStatus {
    Confirmed,
    Validating,
    New,
}

pub struct FindingItem {
    title: String,
    description: String,
    severity: Severity,
    status: FindingStatus,
    cve: Option<String>,
    target: String,
    discovered_time: String,
}

impl FindingItem {
    pub fn new(
        title: String,
        description: String,
        severity: Severity,
        status: FindingStatus,
        cve: Option<String>,
        target: String,
        discovered_time: String,
    ) -> Self {
        Self {
            title,
            description,
            severity,
            status,
            cve,
            target,
            discovered_time,
        }
    }

    fn render_severity_indicator(&self) -> impl IntoElement {
        let color = match self.severity {
            Severity::Critical => gpui::rgb(0xef4444),
            Severity::High => gpui::rgb(0xf97316),
            Severity::Medium => gpui::rgb(0xfbbf24),
            Severity::Low => gpui::rgb(0x10b981),
        };

        div()
            .w_3()
            .h_3()
            .rounded_full()
            .bg(color)
    }

    fn render_status_badge(&self) -> impl IntoElement {
        let (text, color, bg_color) = match self.status {
            FindingStatus::Confirmed => ("CONFIRMED", gpui::rgb(0x10b981), gpui::rgb(0x10b981).opacity(0.2)),
            FindingStatus::Validating => ("VALIDATING", gpui::rgb(0xfbbf24), gpui::rgb(0xfbbf24).opacity(0.2)),
            FindingStatus::New => ("NEW", gpui::rgb(0x60a5fa), gpui::rgb(0x60a5fa).opacity(0.2)),
        };

        div()
            .px_2()
            .py_1()
            .rounded_md()
            .bg(bg_color)
            .child(
                label(text)
                    .text_size(ui::TextSize::XSmall)
                    .text_color(color)
            )
    }
}

impl Render for FindingItem {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        card()
            .p_4()
            .child(
                h_flex()
                    .items_start()
                    .gap_3()
                    .child(self.render_severity_indicator())
                    .child(
                        v_flex()
                            .flex_1()
                            .gap_2()
                            .child(
                                h_flex()
                                    .items_center()
                                    .gap_2()
                                    .child(
                                        label(&self.title)
                                            .text_size(ui::TextSize::Medium)
                                            .text_color(gpui::white())
                                    )
                                    .when_some(self.cve.as_ref(), |this, cve| {
                                        this.child(
                                            div()
                                                .px_2()
                                                .py_0p5()
                                                .rounded_md()
                                                .bg(gpui::rgb(0x2d2d2d))
                                                .child(
                                                    label(cve)
                                                        .text_size(ui::TextSize::XSmall)
                                                        .text_color(gpui::rgb(0x9ca3af))
                                                )
                                        )
                                    })
                            )
                            .child(
                                label(&self.description)
                                    .text_size(ui::TextSize::Small)
                                    .text_color(gpui::rgb(0x9ca3af))
                            )
                            .child(
                                h_flex()
                                    .gap_3()
                                    .items_center()
                                    .child(
                                        label(&format!("🕒 {}", self.discovered_time))
                                            .text_size(ui::TextSize::XSmall)
                                            .text_color(gpui::rgb(0x6b7280))
                                    )
                                    .child(
                                        label(&format!("🎯 {}", self.target))
                                            .text_size(ui::TextSize::XSmall)
                                            .text_color(gpui::rgb(0x6b7280))
                                    )
                            )
                    )
                    .child(self.render_status_badge())
            )
    }
}

pub struct FindingsView {
    findings: Vec<FindingItem>,
    filter_severity: Option<Severity>,
    search_query: String,
    critical_count: usize,
    high_count: usize,
}

impl FindingsView {
    pub fn new(_cx: &mut ViewContext<Self>) -> Self {
        Self {
            findings: vec![],
            filter_severity: None,
            search_query: String::new(),
            critical_count: 2,
            high_count: 2,
        }
    }

    pub fn add_finding(&mut self, finding: FindingItem) {
        self.findings.push(finding);
    }

    fn render_header(&self) -> impl IntoElement {
        h_flex()
            .items_center()
            .justify_between()
            .pb_4()
            .child(
                h_flex()
                    .gap_2()
                    .items_center()
                    .child(
                        label("🔍 Security Findings")
                            .text_size(ui::TextSize::Large)
                            .text_color(gpui::white())
                    )
                    .child(
                        label(&format!("Total: {} ", self.findings.len()))
                            .text_color(gpui::rgb(0x9ca3af))
                    )
                    .child(
                        label(&format!("Critical: {} ", self.critical_count))
                            .text_color(gpui::rgb(0xef4444))
                    )
                    .child(
                        label(&format!("High: {}", self.high_count))
                            .text_color(gpui::rgb(0xf97316))
                    )
            )
            .child(
                h_flex()
                    .gap_2()
                    .child(
                        div()
                            .px_3()
                            .py_2()
                            .rounded_md()
                            .bg(gpui::rgb(0x252525))
                            .child(label("🔍 Filter findings...").text_color(gpui::rgb(0x6b7280)))
                    )
                    .child(
                        button("📤 Export Report")
                            .primary()
                    )
            )
    }

    fn render_filter_tabs(&self) -> impl IntoElement {
        h_flex()
            .gap_2()
            .pb_4()
            .child(self.render_filter_tab("All Findings", true))
            .child(self.render_filter_tab("Critical", false))
            .child(self.render_filter_tab("High", false))
            .child(self.render_filter_tab("Medium", false))
            .child(self.render_filter_tab("Low", false))
            .child(self.render_filter_tab("Info", false))
    }

    fn render_filter_tab(&self, label_text: &str, is_active: bool) -> impl IntoElement {
        div()
            .px_3()
            .py_2()
            .rounded_md()
            .when(is_active, |this| {
                this.bg(gpui::rgb(0x6366f1))
            })
            .when(!is_active, |this| {
                this.hover(|style| style.bg(gpui::rgb(0x2d2d2d)))
            })
            .child(
                label(label_text)
                    .text_size(ui::TextSize::Small)
                    .text_color(if is_active { gpui::white() } else { gpui::rgb(0x9ca3af) })
            )
    }
}

impl Render for FindingsView {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        v_flex()
            .w_full()
            .p_4()
            .gap_3()
            .child(self.render_header())
            .child(self.render_filter_tabs())
            .child(
                v_flex()
                    .gap_2()
                    .children(self.findings.iter().map(|finding| {
                        // Clone the finding data to create a new FindingItem
                        FindingItem::new(
                            finding.title.clone(),
                            finding.description.clone(),
                            finding.severity.clone(),
                            finding.status,
                            finding.cve.clone(),
                            finding.target.clone(),
                            finding.discovered_time.clone(),
                        )
                    }))
            )
    }
}
