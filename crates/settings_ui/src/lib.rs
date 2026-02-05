//! Settings UI - Application settings panel

#![recursion_limit = "256"]

use gpui::*;
use gpui_component::{
    h_flex, v_flex,
    sidebar::{Sidebar, SidebarMenu, SidebarMenuItem},
    label::Label,
    IconName, Icon,
};

pub mod components;
pub mod config;
pub mod provider;
pub mod ai_client;
mod ai_settings;
pub use ai_settings::AiSettingsPanel;
pub use config::{Settings, AiSettings, AiProviderConfig};
pub use provider::{AiProvider, ProviderId, ProviderRegistry};
use ui::theme::*;

/// Setting category enum
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SettingCategory {
    General,
    Appearance,
    Ai,
    Security,
    Network,
    Workflow,
    Scanner,
    Storage,
    Advanced,
}

impl SettingCategory {
    pub fn name(&self) -> &'static str {
        match self {
            SettingCategory::General => "General",
            SettingCategory::Appearance => "Appearance",
            SettingCategory::Ai => "AI",
            SettingCategory::Security => "Security",
            SettingCategory::Network => "Network",
            SettingCategory::Workflow => "Workflow",
            SettingCategory::Scanner => "Scanner",
            SettingCategory::Storage => "Storage",
            SettingCategory::Advanced => "Advanced",
        }
    }

    pub fn icon(&self) -> IconName {
        match self {
            SettingCategory::General => IconName::Settings,
            SettingCategory::Appearance => IconName::Palette,
            SettingCategory::Ai => IconName::Bot,
            SettingCategory::Security => IconName::Settings2,
            SettingCategory::Network => IconName::Network,
            SettingCategory::Workflow => IconName::LayoutDashboard,
            SettingCategory::Scanner => IconName::Cpu,
            SettingCategory::Storage => IconName::HardDrive,
            SettingCategory::Advanced => IconName::SquareTerminal,
        }
    }
}

/// Settings Panel - Main settings container with sidebar navigation
pub struct SettingsPanel {
    selected_category: SettingCategory,
    ai_settings: Entity<AiSettingsPanel>,
}

impl SettingsPanel {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let ai_settings = cx.new(|cx| AiSettingsPanel::new(window, cx));
        Self {
            selected_category: SettingCategory::General,
            ai_settings,
        }
    }

    fn select_category(&mut self, category: SettingCategory, cx: &mut Context<Self>) {
        self.selected_category = category;
        cx.notify();
    }
}

impl Render for SettingsPanel {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let categories = vec![
            SettingCategory::General,
            SettingCategory::Appearance,
            SettingCategory::Ai,
            SettingCategory::Security,
            SettingCategory::Network,
            SettingCategory::Workflow,
            SettingCategory::Scanner,
            SettingCategory::Storage,
            SettingCategory::Advanced,
        ];

        let selected = self.selected_category;

        h_flex()
            .size_full()
            .gap_0()
            .child(
                Sidebar::new("settings-sidebar")
                    .collapsible(false)
                    .header(
                        v_flex()
                            .gap_3()
                            .child(Label::new("Settings").text_lg().font_weight(FontWeight::SEMIBOLD)),
                    )
                    .child(
                        SidebarMenu::new()
                            .children(categories.into_iter().map(|cat| {
                                SidebarMenuItem::new(cat.name())
                                    .icon(Icon::new(cat.icon()))
                                    .active(selected == cat)
                                    .on_click(cx.listener(move |this, _, _, cx| {
                                        this.select_category(cat, cx);
                                    }))
                            })),
                    ),
            )
            .child(
                div()
                    .flex_1()
                    .h_full()
                    .child(match selected {
                        SettingCategory::Ai => self.ai_settings.clone().into_any_element(),
                        _ => render_placeholder(selected.name()).into_any_element(),
                    })
            )
    }
}

fn render_placeholder(title: impl Into<SharedString>) -> impl IntoElement {
    v_flex()
        .flex_1()
        .h_full()
        .p_6()
        .child(
            v_flex()
                .gap_2()
                .pb_4()
                .border_b_1()
                .border_color(rgb(BORDER_COLOR))
                .child(Label::new(title).text_xl())
                .child(Label::new("Configure settings").text_color(rgb(TEXT_SECONDARY))),
        )
        .child(
            v_flex()
                .flex_1()
                .items_center()
                .justify_center()
                .child(Label::new("Settings coming soon").text_color(rgb(TEXT_MUTED))),
        )
}
