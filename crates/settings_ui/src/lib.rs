pub mod sidebar;
pub mod content;

pub use sidebar::render_sidebar;
pub use content::render_content;

use gpui::*;
use gpui_component::{h_flex, v_flex};

/// Settings 面板 - T1-24: 整合设置视图
pub struct SettingsPanel {
    selected_category: Option<String>,
}

impl SettingsPanel {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self {
            selected_category: Some("General".to_string()),
        }
    }
}

impl Render for SettingsPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .size_full()
            .gap_0()
            .child(render_sidebar(cx))
            .child(render_content())
    }
}
