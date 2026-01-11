pub mod vuln_list;
pub mod vuln_detail;
pub mod cve_panel;

pub use vuln_list::{VulnList, VulnFilterType, render_vuln_list};
pub use vuln_detail::render_vuln_detail;
pub use cve_panel::render_cve_panel;

use gpui::*;
use gpui_component::{label::Label, v_flex};

/// Vulns 面板
pub struct VulnsPanel {
    _placeholder: (),
}

impl VulnsPanel {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self { _placeholder: () }
    }
}

impl Render for VulnsPanel {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .items_center()
            .justify_center()
            .child(Label::new("Vulns Panel - Coming Soon"))
    }
}
