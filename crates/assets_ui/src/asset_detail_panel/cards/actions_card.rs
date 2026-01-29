use data::models::AssetNode;
use gpui::*;
use gpui_component::{label::Label, v_flex, Icon, IconName};

use crate::config::{theme_ext::*, ui_labels::actions};
use crate::events::AssetActionEvent;

/// Action buttons card
pub struct ActionsCard;

impl ActionsCard {
    pub fn render(node: &AssetNode, cx: &mut Context<crate::asset_detail_panel::AssetDetailPanel>) -> impl IntoElement {
        v_flex()
            .w(px(140.0))
            .gap_2()
            .child(Self::render_ai_button(node, cx))
            .child(Self::render_scan_button(node, cx))
            .child(Self::render_config_button(node, cx))
    }

    fn render_ai_button(
        node: &AssetNode,
        cx: &mut Context<crate::asset_detail_panel::AssetDetailPanel>,
    ) -> impl IntoElement {
        let node = node.clone();

        div()
            .w_full()
            .py_2()
            .px_3()
            .rounded_lg()
            .bg(rgb(BUTTON_AI_BG))
            .border_1()
            .border_color(rgb(BUTTON_AI_BORDER))
            .flex()
            .items_center()
            .justify_center()
            .gap_2()
            .cursor_pointer()
            .on_mouse_down(MouseButton::Left, cx.listener(move |_, _, _, cx| {
                cx.emit(AssetActionEvent::ScanRequested(node.clone()));
            }))
            .child(
                Icon::new(IconName::Star)
                    .size(px(16.0))
                    .text_color(rgb(BUTTON_AI_TEXT)),
            )
            .child(
                Label::new(actions::AI_ANALYSIS)
                    .text_sm()
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(BUTTON_AI_TEXT)),
            )
    }

    fn render_scan_button(
        node: &AssetNode,
        cx: &mut Context<crate::asset_detail_panel::AssetDetailPanel>,
    ) -> impl IntoElement {
        let node = node.clone();

        div()
            .w_full()
            .py_2()
            .px_3()
            .rounded_lg()
            .bg(rgb(BUTTON_SCAN_BG))
            .flex()
            .items_center()
            .justify_center()
            .gap_2()
            .cursor_pointer()
            .on_mouse_down(MouseButton::Left, cx.listener(move |_, _, _, cx| {
                cx.emit(AssetActionEvent::ScanRequested(node.clone()));
            }))
            .child(
                Icon::new(IconName::TriangleAlert)
                    .size(px(16.0))
                    .text_color(rgb(BUTTON_SCAN_TEXT)),
            )
            .child(
                Label::new(actions::SCAN_ASSET)
                    .text_sm()
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(BUTTON_SCAN_TEXT)),
            )
    }

    fn render_config_button(
        node: &AssetNode,
        cx: &mut Context<crate::asset_detail_panel::AssetDetailPanel>,
    ) -> impl IntoElement {
        let node = node.clone();

        div()
            .w_full()
            .py_2()
            .px_3()
            .rounded_lg()
            .bg(rgb(BUTTON_CONFIG_BG))
            .border_1()
            .border_color(rgb(BUTTON_CONFIG_BORDER))
            .flex()
            .items_center()
            .justify_center()
            .gap_2()
            .cursor_pointer()
            .on_mouse_down(MouseButton::Left, cx.listener(move |_, _, _, cx| {
                cx.emit(AssetActionEvent::EditRequested(node.clone()));
            }))
            .child(
                Icon::new(IconName::Settings)
                    .size(px(16.0))
                    .text_color(rgb(BUTTON_CONFIG_TEXT)),
            )
            .child(
                Label::new(actions::CONFIGURE)
                    .text_sm()
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(BUTTON_CONFIG_TEXT)),
            )
    }
}
