//! Vulnerability List Panel - Left column with filter tabs and collapsible groups

use gpui::*;
use gpui::prelude::FluentBuilder;
use gpui::AnyElement;
use gpui_component::{
    h_flex, v_flex,
    label::Label,
    badge::Badge,
    scroll::ScrollableElement,
};
use data::{VulnStore, GroupBy, VulnerabilityWithFindings};
use ui::theme::*;
use crate::{severity_color, severity_label};

/// Render the left column vulnerability list
pub fn render_vuln_list(
    vulnerabilities: &[&VulnerabilityWithFindings],
    group_by: GroupBy,
    vuln_store: &Entity<VulnStore>,
    selected_vuln_id: Option<String>,
    search_query: &str,
    cx: &App,
) -> impl IntoElement {
    // Calculate total count for header badge
    let total_count: usize = vulnerabilities.iter().map(|v| v.finding_count()).sum();

    v_flex()
        .w(px(320.0))
        .h_full()
        .gap(SPACING_SM)
        // Header with title and count
        .child(render_header(total_count))
        // Filter tabs
        .child(render_filter_tabs(group_by, vuln_store))
        // Search box
        .child(render_search_box(vuln_store, search_query))
        // Grouped list with collapsible sections
        .child(
            v_flex()
                .flex_1()
                .overflow_y_scrollbar()
                .gap(SPACING_SM)
                .children(render_grouped_list(vulnerabilities, group_by, vuln_store, selected_vuln_id, cx))
        )
}

/// Header with title and vulnerability count badge
fn render_header(total_count: usize) -> impl IntoElement {
    h_flex()
        .px(SPACING_MD)
        .py(SPACING_SM)
        .items_center()
        .gap(SPACING_SM)
        .child(
            Label::new("Vulnerabilities")
                .text_size(TEXT_SIZE_LG)
                .font_weight(FontWeight::SEMIBOLD)
        )
        .child(
            Badge::new()
                .count(total_count)
        )
}

/// Compact filter tabs (Severity / Asset / MITRE)
fn render_filter_tabs(
    current_group_by: GroupBy,
    vuln_store: &Entity<VulnStore>,
) -> impl IntoElement {
    let tabs = vec![
        (GroupBy::Severity, "Severity"),
        (GroupBy::Asset, "Asset"),
        (GroupBy::Mitre, "MITRE"),
    ];

    h_flex()
        .px(SPACING_SM)
        .gap(SPACING_XS)
        .children(tabs.into_iter().map(move |(group_by, label)| {
            let is_active = current_group_by == group_by;
            let vuln_store = vuln_store.clone();
            let label = label.to_string();
            
            // Compact pill-style tab
            h_flex()
                .px(SPACING_SM)
                .py(SPACING_XS)
                .rounded_md()
                .cursor_pointer()
                .text_size(TEXT_SIZE_SM)
                .font_weight(FontWeight::MEDIUM)
                .when(is_active, |this| {
                    this.bg(rgb(ACCENT_BLUE))
                        .text_color(gpui::white())
                })
                .when(!is_active, |this| {
                    this.text_color(rgb(TEXT_SECONDARY))
                        .hover(|s| s.bg(rgb(BG_CARD_HOVER)))
                })
                .child(label)
                .on_mouse_down(gpui::MouseButton::Left, move |_event, _window, cx| {
                    vuln_store.update(cx, |store, cx| {
                        store.set_group_by(group_by, cx);
                    });
                })
        }))
}

/// Search box - custom implementation with click to focus
fn render_search_box(
    vuln_store: &Entity<VulnStore>,
    search_query: &str,
) -> impl IntoElement {
    let vuln_store_clear = vuln_store.clone();
    let vuln_store_click = vuln_store.clone();
    let has_query = !search_query.is_empty();
    let query_display = if search_query.is_empty() {
        "Search...".to_string()
    } else {
        search_query.to_string()
    };
    
    h_flex()
        .mx(SPACING_SM)
        .px(SPACING_SM)
        .py(SPACING_XS)
        .gap(SPACING_XS)
        .items_center()
        .bg(rgb(BG_CARD))
        .rounded_md()
        .border_1()
        .border_color(rgb(BORDER_DEFAULT))
        .cursor_pointer()
        // Click to clear search or trigger search modal (simplified for now)
        .on_mouse_down(gpui::MouseButton::Left, move |_event, _window, cx| {
            if has_query {
                // Clear search on click if has content
                vuln_store_click.update(cx, |store, cx| {
                    store.set_search_query(String::new(), cx);
                });
            }
        })
        .child(
            // Search icon
            h_flex()
                .w(px(16.0))
                .h(px(16.0))
                .text_color(rgb(TEXT_MUTED))
                .child(Label::new("🔍").text_size(TEXT_SIZE_SM))
        )
        .child(
            Label::new(query_display)
                .text_size(TEXT_SIZE_SM)
                .text_color(if has_query { rgb(TEXT_PRIMARY) } else { rgb(TEXT_MUTED) })
        )
        // Clear button (X) when has query
        .children(if has_query {
            Some(
                h_flex()
                    .w(px(16.0))
                    .h(px(16.0))
                    .items_center()
                    .justify_center()
                    .cursor_pointer()
                    .child(Label::new("✕").text_size(TEXT_SIZE_SM))
                    .on_mouse_down(gpui::MouseButton::Left, move |_event, _window, cx| {
                        vuln_store_clear.update(cx, |store, cx| {
                            store.set_search_query(String::new(), cx);
                        });
                    })
            )
        } else {
            None
        })
}

/// Render grouped list based on current group_by setting
fn render_grouped_list(
    vulnerabilities: &[&VulnerabilityWithFindings],
    group_by: GroupBy,
    vuln_store: &Entity<VulnStore>,
    selected_vuln_id: Option<String>,
    cx: &App,
) -> Vec<AnyElement> {
    match group_by {
        GroupBy::Severity => render_severity_groups(vulnerabilities, vuln_store, selected_vuln_id, cx),
        GroupBy::Asset => render_asset_groups(vulnerabilities, vuln_store, selected_vuln_id, cx),
        GroupBy::Mitre => render_mitre_groups(vulnerabilities, vuln_store, selected_vuln_id, cx),
    }
}

/// Group vulnerabilities by severity
fn render_severity_groups(
    vulnerabilities: &[&VulnerabilityWithFindings],
    vuln_store: &Entity<VulnStore>,
    selected_vuln_id: Option<String>,
    cx: &App,
) -> Vec<AnyElement> {
    use data::models::Severity::*;
    let severities = vec![Critical, High, Medium, Low, Info];

    severities
        .into_iter()
        .filter_map(|sev| {
            let vulns: Vec<&VulnerabilityWithFindings> = vulnerabilities
                .iter()
                .filter(|&&v| v.vulnerability.severity == sev)
                .copied()
                .collect();
            
            if vulns.is_empty() {
                None
            } else {
                let group_id = severity_label(&sev).to_string();
                let group_title = format!("{} ({})", group_id, vulns.len());
                Some(render_collapsible_group(
                    &group_id,
                    &group_title,
                    vulns,
                    vuln_store,
                    selected_vuln_id.clone(),
                    cx,
                ).into_any_element())
            }
        })
        .collect()
}

/// Group vulnerabilities by asset
fn render_asset_groups(
    vulnerabilities: &[&VulnerabilityWithFindings],
    vuln_store: &Entity<VulnStore>,
    selected_vuln_id: Option<String>,
    cx: &App,
) -> Vec<AnyElement> {
    use std::collections::BTreeMap;
    let mut groups: BTreeMap<String, Vec<&VulnerabilityWithFindings>> = BTreeMap::new();
    
    for vuln in vulnerabilities {
        if vuln.affected_assets.is_empty() {
            groups.entry("Unassigned".to_string()).or_default().push(vuln);
        } else {
            // Group by first affected asset for simplicity
            let asset_key = format!("Asset {}", vuln.affected_assets[0]);
            groups.entry(asset_key).or_default().push(vuln);
        }
    }

    groups
        .into_iter()
        .map(|(asset_name, vulns)| {
            let group_id = asset_name.clone();
            let group_title = format!("{} ({})", asset_name, vulns.len());
            render_collapsible_group(&group_id, &group_title, vulns, vuln_store, selected_vuln_id.clone(), cx)
                .into_any_element()
        })
        .collect()
}

/// Group vulnerabilities by MITRE technique
fn render_mitre_groups(
    vulnerabilities: &[&VulnerabilityWithFindings],
    vuln_store: &Entity<VulnStore>,
    selected_vuln_id: Option<String>,
    cx: &App,
) -> Vec<AnyElement> {
    use std::collections::BTreeMap;
    let mut groups: BTreeMap<String, Vec<&VulnerabilityWithFindings>> = BTreeMap::new();

    for vuln in vulnerabilities {
        let mut has_technique = false;
        
        // Get techniques from vulnerability
        for tech in &vuln.vulnerability.mitre_techniques {
            groups.entry(tech.clone()).or_default().push(vuln);
            has_technique = true;
        }
        
        // Also get from findings
        for finding in &vuln.findings {
            for tech in &finding.mitre_techniques {
                if !vuln.vulnerability.mitre_techniques.contains(tech) {
                    groups.entry(tech.clone()).or_default().push(vuln);
                    has_technique = true;
                }
            }
        }
        
        // If no techniques, put in "Uncategorized"
        if !has_technique {
            groups.entry("Uncategorized".to_string()).or_default().push(vuln);
        }
    }

    groups
        .into_iter()
        .map(|(tech, vulns)| {
            let group_id = tech.clone();
            let group_title = format!("{} ({})", tech, vulns.len());
            render_collapsible_group(&group_id, &group_title, vulns, vuln_store, selected_vuln_id.clone(), cx)
                .into_any_element()
        })
        .collect()
}

/// Render a collapsible group with custom header
fn render_collapsible_group(
    group_id: &str,
    title: &str,
    vulns: Vec<&VulnerabilityWithFindings>,
    vuln_store: &Entity<VulnStore>,
    selected_vuln_id: Option<String>,
    cx: &App,
) -> impl IntoElement {
    let group_id = group_id.to_string();
    let title = title.to_string();
    let vuln_store_for_header = vuln_store.clone();
    let vuln_store_for_state = vuln_store.clone();
    
    // Check if group is collapsed
    let is_open = !vuln_store.read(cx).is_group_collapsed(&group_id);
    
    v_flex()
        .gap(SPACING_XS)
        // Group header (clickable to toggle)
        .child(
            h_flex()
                .px(SPACING_SM)
                .py(SPACING_XS)
                .gap(SPACING_XS)
                .items_center()
                .cursor_pointer()
                .rounded_md()
                .hover(|s| s.bg(rgb(BG_CARD_HOVER)))
                .child(
                    // Chevron icon
                    Label::new(if is_open { "▼" } else { "▶" })
                        .text_size(TEXT_SIZE_XS)
                        .text_color(rgb(TEXT_MUTED))
                )
                .child(
                    Label::new(title)
                        .text_size(TEXT_SIZE_SM)
                        .text_color(rgb(TEXT_SECONDARY))
                )
                .on_mouse_down(gpui::MouseButton::Left, move |_event, _window, cx| {
                    vuln_store_for_header.update(cx, |store, cx| {
                        store.toggle_group_collapsed(group_id.clone(), cx);
                    });
                })
        )
        // Group content (vuln cards) - only show if not collapsed
        .when(is_open, |this| {
            this.children(vulns.iter().map(|vuln| {
                let is_selected = selected_vuln_id.as_ref() == Some(&vuln.vulnerability.id);
                render_vuln_card(vuln, &vuln_store_for_state, is_selected)
            }))
        })
}

/// Render a vulnerability card with left severity indicator
fn render_vuln_card(
    vuln: &VulnerabilityWithFindings,
    vuln_store: &Entity<VulnStore>,
    is_selected: bool,
) -> impl IntoElement {
    let vuln_id = vuln.vulnerability.id.clone();
    let cve_id = if vuln.vulnerability.cve_id.is_empty() {
        vuln.vulnerability.id.clone()
    } else {
        vuln.vulnerability.cve_id.clone()
    };
    let name = vuln.vulnerability.name.clone();
    let severity = vuln.vulnerability.severity.clone();
    let finding_count = vuln.finding_count();
    let has_poc = vuln.findings.iter().any(|f| !f.poc_code.is_empty());
    let ai_confidence = vuln.findings.iter()
        .filter_map(|f| f.ai_confidence)
        .max();
    
    let vuln_store_clone = vuln_store.clone();
    let severity_color_val = severity_color(&severity);

    h_flex()
        .px(SPACING_SM)
        .py(SPACING_SM)
        .gap(SPACING_SM)
        .cursor_pointer()
        .rounded_md()
        .when(is_selected, |this| this.bg(rgb(BG_CARD_HOVER)))
        .when(!is_selected, |this| {
            this.hover(|s| s.bg(rgb(BG_CARD_HOVER)))
        })
        // Left severity indicator bar
        .child(
            h_flex()
                .w(px(3.0))
                .h_full()
                .rounded_full()
                .bg(rgb(severity_color_val))
        )
        .child(
            v_flex()
                .flex_1()
                .gap(SPACING_XS)
                // Title
                .child(
                    Label::new(name)
                        .text_size(TEXT_SIZE_SM)
                        .font_weight(FontWeight::MEDIUM)
                        .line_clamp(2)
                )
                // CVE ID and meta info row
                .child(
                    h_flex()
                        .gap(SPACING_SM)
                        .items_center()
                        .child(
                            Label::new(cve_id)
                                .text_size(TEXT_SIZE_XS)
                                .text_color(rgb(TEXT_MUTED))
                        )
                        // AI confidence badge
                        .children(ai_confidence.map(|conf| {
                            h_flex()
                                .px(SPACING_XS)
                                .py(px(1.0))
                                .rounded_sm()
                                .bg(rgb(STATUS_AI))
                                .child(
                                    Label::new(format!("AI {}%", conf))
                                        .text_size(TEXT_SIZE_XS)
                                        .text_color(gpui::white())
                                )
                        }))
                        // PoC badge
                        .children(if has_poc {
                            Some(
                                h_flex()
                                    .px(SPACING_XS)
                                    .py(px(1.0))
                                    .rounded_sm()
                                    .bg(rgb(ACCENT_BLUE))
                                    .child(
                                        Label::new("PoC")
                                            .text_size(TEXT_SIZE_XS)
                                            .text_color(gpui::white())
                                    )
                            )
                        } else {
                            None
                        })
                        // Finding count
                        .children(if finding_count > 0 {
                            Some(
                                Label::new(format!("{} findings", finding_count))
                                    .text_size(TEXT_SIZE_XS)
                                    .text_color(rgb(TEXT_MUTED))
                            )
                        } else {
                            None
                        })
                )
        )
        .on_mouse_down(gpui::MouseButton::Left, move |_event, _window, cx| {
            vuln_store_clone.update(cx, |store, cx| {
                store.select_vulnerability(vuln_id.clone(), cx);
            });
        })
}
