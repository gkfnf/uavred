//! Asset Detail Panel - Displays detailed information about a selected asset
//!
//! ## Architecture
//!
//! This module uses a card-based architecture where each aspect of asset
//! information is rendered by a dedicated card component:
//!
//! - ZoneCard: Security zone information
//! - RiskCard: Risk score with progress bar
//! - StatusCard: Asset online/offline status
//! - PortsCard: Open ports list
//! - ServicesCard: Detected services
//! - CredentialsCard: Authentication info
//! - BusinessCard: Business purpose
//! - OwnerCard: Owner/team info
//! - ComplianceCard: Compliance badges
//! - ActionsCard: AI Analysis, Scan, Config buttons
//! - VulnStatsCard: Vulnerability count

use data::models::AssetNode;
use gpui::*;
use gpui_component::{h_flex, label::Label, v_flex, Icon, IconName};
use ui::theme::*;

use crate::config::{theme_ext::risk_color, ui_labels::panel};
use crate::events::AssetActionEvent;

mod cards;

use cards::*;

impl EventEmitter<AssetActionEvent> for AssetDetailPanel {}

/// Asset detail panel - displays comprehensive asset information
pub struct AssetDetailPanel {
    selected_node: Option<AssetNode>,
}

impl AssetDetailPanel {
    /// Create a new asset detail panel
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self {
            selected_node: None,
        }
    }

    /// Set the currently displayed node
    pub fn set_node(&mut self, node: AssetNode, cx: &mut Context<Self>) {
        self.selected_node = Some(node);
        cx.notify();
    }

    /// Clear the current selection
    pub fn clear_node(&mut self, cx: &mut Context<Self>) {
        self.selected_node = None;
        cx.notify();
    }

    /// Render the panel header with asset name and actions
    fn render_header(&self, node: &AssetNode, cx: &mut Context<Self>) -> impl IntoElement {
        let risk_color = risk_color(node.risk_score as u8);
        let node_id = node.id.clone();

        h_flex()
            .w_full()
            .p_4()
            .items_center()
            .gap_3()
            .child(
                Label::new(panel::DETAIL_TITLE)
                    .text_sm()
                    .text_color(rgb(TEXT_SECONDARY)),
            )
            .child(
                div()
                    .size(px(8.0))
                    .rounded_full()
                    .bg(rgb(risk_color)),
            )
            .child(
                Label::new(node.name.clone())
                    .text_base()
                    .font_weight(FontWeight::BOLD),
            )
            .child(
                Label::new(node.ip_address.clone())
                    .text_sm()
                    .text_color(rgb(TEXT_MUTED)),
            )
            .child(div().flex_1())
            .child(
                h_flex()
                    .gap_4()
                    .child(
                        Icon::new(IconName::Info)
                            .size(px(18.0))
                            .text_color(rgb(TEXT_MUTED)),
                    )
                    .child(
                        div()
                            .cursor_pointer()
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(move |_, _, _, cx| {
                                    cx.emit(AssetActionEvent::DeleteRequested(node_id.clone()));
                                }),
                            )
                            .child(
                                Icon::new(IconName::Delete)
                                    .size(px(18.0))
                                    .text_color(rgb(TEXT_MUTED)),
                            ),
                    )
                    .child(
                        Icon::new(IconName::ChevronDown)
                            .size(px(18.0))
                            .text_color(rgb(TEXT_MUTED)),
                    ),
            )
    }

    /// Render the main content grid (5 columns)
    fn render_content_grid(&self, node: &AssetNode, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .w_full()
            .p_4()
            .gap_4()
            .items_start()
            // Column 1: Zone, Risk Score, Status
            .child(
                v_flex()
                    .w(px(160.0))
                    .gap_3()
                    .child(ZoneCard::render(node))
                    .child(RiskCard::render(node))
                    .child(StatusCard::render(node)),
            )
            // Column 2: Open Ports & Protocol Info
            .child(PortsCard::render(node))
            // Column 3: Detected Services
            .child(ServicesCard::render(node))
            // Column 4: Credentials, Purpose, Owner, Compliance
            .child(
                v_flex()
                    .w(px(180.0))
                    .gap_3()
                    .child(CredentialsCard::render())
                    .child(BusinessCard::render(node))
                    .child(OwnerCard::render(node))
                    .child(ComplianceCard::render(node)),
            )
            // Column 5: Action Buttons & Vulnerability Stats
            .child(
                v_flex()
                    .child(ActionsCard::render(node, cx))
                    .child(VulnStatsCard::render(node)),
            )
    }

    /// Render empty state when no asset is selected
    fn render_empty_state(&self) -> AnyElement {
        v_flex()
            .size_full()
            .items_center()
            .justify_center()
            .bg(rgb(BG_CARD))
            .rounded_lg()
            .child(
                v_flex()
                    .items_center()
                    .gap_3()
                    .child(
                        Icon::new(IconName::SquareTerminal)
                            .size(px(48.0))
                            .text_color(rgb(TEXT_MUTED)),
                    )
                    .child(
                        Label::new(panel::NO_SELECTION_MESSAGE)
                            .text_sm()
                            .text_color(rgb(TEXT_MUTED)),
                    ),
            )
            .into_any_element()
    }
}

impl Render for AssetDetailPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if let Some(node) = self.selected_node.clone() {
            v_flex()
                .size_full()
                .gap_0()
                .bg(rgb(0xffffff))
                .child(self.render_header(&node, cx))
                .child(self.render_content_grid(&node, cx))
                .into_any_element()
        } else {
            self.render_empty_state()
        }
    }
}
