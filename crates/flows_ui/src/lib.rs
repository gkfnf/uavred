use gpui::*;
use gpui_component::{label::Label, v_flex};

/// Flows 面板
pub struct FlowsPanel {
    _placeholder: (),
}

impl FlowsPanel {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self { _placeholder: () }
    }
}

impl Render for FlowsPanel {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .items_center()
            .justify_center()
            .child(Label::new("Flows Panel - Coming Soon"))
    }
}
