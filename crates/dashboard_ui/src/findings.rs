// Findings 子面板 - 完整的漏洞发现面板

use gpui::*;
use gpui_component::{
    button::{Button, ButtonVariants as _},
    group_box::{GroupBox, GroupBoxVariants as _},
    h_flex,
    label::Label,
    tag::Tag,
    v_flex, Selectable, Sizable, WindowExt,
};

use crate::dashboard_panel::DashboardPanel;

/// Findings 主视图
pub fn render_findings_view(
    _panel: &mut DashboardPanel,
    window: &mut Window,
    cx: &mut Context<DashboardPanel>,
) -> impl IntoElement {
    v_flex()
        .flex_1()
        .pt(px(0.0))
        .px(px(24.0))
        .pb(px(24.0))
        .gap(px(0.0))
        .child(
            // 标题行
            h_flex()
                .h(px(48.0))
                .items_center()
                .justify_between()
                .child(
                    Label::new("Security Findings")
                        .text_xl()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(0x1f2937)),
                )
                .child(
                    h_flex()
                        .gap(px(16.0))
                        .items_center()
                        .child(Label::new("Total: 5").text_sm().text_color(rgb(0x6b7280)))
                        .child(
                            Label::new("Critical: 2")
                                .text_sm()
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(rgb(0xef4444)),
                        )
                        .child(
                            Label::new("High: 2")
                                .text_sm()
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(rgb(0xf97316)),
                        ),
                ),
        )
        .child(
            // 内容区域
            v_flex()
                .flex_1()
                .gap(px(16.0))
                .pt(px(12.0))
                .child(render_filter_and_search(window, cx))
                .child(render_filter_tabs(cx))
                .child(render_findings_list()),
        )
}

/// Filter and Search
fn render_filter_and_search(
    _window: &mut Window,
    cx: &mut Context<DashboardPanel>,
) -> impl IntoElement {
    h_flex()
        .gap(px(8.0))
        .items_center()
        .child(
            div()
                .flex_1()
                .h(px(32.0))
                .px(px(8.0))
                .bg(rgb(0xf9fafb))
                .border(px(1.0))
                .border_color(rgb(0xe5e7eb))
                .rounded(px(4.0))
                .items_center()
                .child(
                    Label::new("Q Filter findings...")
                        .text_sm()
                        .text_color(rgb(0x9ca3af)),
                ),
        )
        .child(
            Button::new("export-findings")
                .label("Export Report")
                .on_click(
                    cx.listener(move |_this: &mut DashboardPanel, _, window, cx| {
                        window.open_dialog(cx, |dialog, _, _| {
                            dialog
                                .title("Export Report")
                                .child(
                                    v_flex().gap(px(12.0)).child(Label::new(
                                        "Findings report will be exported as PDF",
                                    )),
                                )
                                .footer(|_, _window, _cx, _| {
                                    vec![
                                        Button::new("export-confirm")
                                            .primary()
                                            .label("Export")
                                            .on_click(|_, window, cx| {
                                                window.push_notification(
                                                    "Report exported successfully",
                                                    cx,
                                                );
                                                window.close_dialog(cx);
                                            }),
                                        Button::new("export-cancel").label("Cancel").on_click(
                                            |_, window, cx| {
                                                window.close_dialog(cx);
                                            },
                                        ),
                                    ]
                                })
                        });
                    }),
                ),
        )
}

/// Filter Tabs
fn render_filter_tabs(cx: &mut Context<DashboardPanel>) -> impl IntoElement {
    h_flex().gap(px(4.0)).children(vec![
        render_finding_filter_tab(cx, "All Findings", true),
        render_finding_filter_tab(cx, "Critical", false),
        render_finding_filter_tab(cx, "High", false),
        render_finding_filter_tab(cx, "Medium", false),
        render_finding_filter_tab(cx, "Low", false),
        render_finding_filter_tab(cx, "Info", false),
    ])
}

/// Finding Filter Tab
fn render_finding_filter_tab(
    _cx: &mut Context<DashboardPanel>,
    label: &str,
    active: bool,
) -> impl IntoElement {
    let label_str = label.to_string();
    Button::new(format!("finding-filter-{}", label))
        .ghost()
        .small()
        .label(label_str)
        .selected(active)
        .on_click(|_, _, _| {})
}

/// Findings List
fn render_findings_list() -> impl IntoElement {
    v_flex()
        .flex_1()
        .gap(px(8.0))
        .children(vec![
            render_finding_card(
                "MAVLink Buffer Overflow",
                Some("CVE-2024-1234"),
                "Detected potential MAVLink Buffer Overflow vulnerability on DJI Mavic 3 via MAVLink protocol.",
                "2m ago",
                "DJI Mavic 3",
                "CONFIRMED",
                "critical",
            ),
            render_finding_card(
                "DJI Auth Bypass",
                None,
                "Detected potential DJI Auth Bypass vulnerability on DJI Mavic 3 via DJI protocol.",
                "5m ago",
                "DJI Mavic 3",
                "VALIDATING",
                "critical",
            ),
            render_finding_card(
                "MySQL Default Creds",
                None,
                "Detected potential MySQL Default Creds vulnerability on GCS Primary via MySQL protocol.",
                "8m ago",
                "GCS Primary",
                "CONFIRMED",
                "high",
            ),
            render_finding_card(
                "RTSP Stream Injection",
                None,
                "Detected potential RTSP Stream Injection vulnerability on DJI Mavic 3 via RTSP protocol.",
                "12m ago",
                "DJI Mavic 3",
                "NEW",
                "high",
            ),
        ])
}

/// Finding Card
fn render_finding_card(
    title: &str,
    cve: Option<&str>,
    description: &str,
    time: &str,
    affected: &str,
    status: &str,
    severity: &str,
) -> impl IntoElement {
    let title_str = title.to_string();
    let cve_str = cve.map(|s| s.to_string());
    let description_str = description.to_string();
    let time_str = time.to_string();
    let affected_str = affected.to_string();
    let status_str = status.to_string();

    // Severity indicator color
    let severity_color = match severity {
        "critical" => rgb(0xef4444),
        "high" => rgb(0xf97316),
        "medium" => rgb(0xfbbf24),
        "low" => rgb(0x60a5fa),
        _ => rgb(0x9ca3af),
    };

    // Status color
    let (status_bg, status_text) = match status {
        "CONFIRMED" => (rgb(0xdcfce7), rgb(0x166534)),
        "VALIDATING" => (rgb(0xfef3c7), rgb(0x92400e)),
        "NEW" => (rgb(0xdbeafe), rgb(0x1e40af)),
        _ => (rgb(0xf3f4f6), rgb(0x6b7280)),
    };

    GroupBox::new().outline().child(
        h_flex()
            .gap(px(12.0))
            .p(px(16.0))
            .items_start()
            .child(
                // Severity indicator
                div()
                    .w(px(4.0))
                    .h_full()
                    .bg(severity_color)
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
                                Label::new(title_str)
                                    .text_base()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(rgb(0x1f2937)),
                            )
                            .children(if let Some(ref cve) = cve_str {
                                vec![Tag::new()
                                    .small()
                                    .bg(rgb(0xe5e7eb))
                                    .text_color(rgb(0x6b7280))
                                    .child(Label::new(cve.clone()).text_xs())
                                    .into_any_element()]
                            } else {
                                vec![]
                            }),
                    )
                    .child(
                        Label::new(description_str)
                            .text_sm()
                            .text_color(rgb(0x6b7280)),
                    )
                    .child(
                        h_flex()
                            .gap(px(8.0))
                            .items_center()
                            .child(Label::new(time_str).text_xs().text_color(rgb(0x9ca3af)))
                            .child(Label::new("•").text_xs().text_color(rgb(0x9ca3af)))
                            .child(Label::new(affected_str).text_xs().text_color(rgb(0x9ca3af))),
                    ),
            )
            .child(
                Tag::new()
                    .small()
                    .bg(status_bg)
                    .text_color(status_text)
                    .child(Label::new(status_str).text_xs()),
            ),
    )
}
