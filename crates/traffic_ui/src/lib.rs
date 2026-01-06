use gpui::*;
use gpui_component::{label::Label, v_flex};

/// Traffic 面板
pub struct TrafficPanel {
    _placeholder: (),
}

impl TrafficPanel {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self { _placeholder: () }
    }
}

impl Render for TrafficPanel {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .items_center()
            .justify_center()
            .child(Label::new("Traffic Panel - Coming Soon"))
    }
}
