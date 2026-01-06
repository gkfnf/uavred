use gpui::*;
use gpui_component::{label::Label, v_flex};

/// Scan 面板
pub struct ScanPanel {
    _placeholder: (),
}

impl ScanPanel {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self { _placeholder: () }
    }
}

impl Render for ScanPanel {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .items_center()
            .justify_center()
            .child(Label::new("Scan Panel - Coming Soon"))
    }
}
