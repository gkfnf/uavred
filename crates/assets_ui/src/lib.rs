use gpui::*;
use gpui_component::{label::Label, v_flex};

/// Assets 面板
pub struct AssetsPanel {
    _placeholder: (),
}

impl AssetsPanel {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self { _placeholder: () }
    }
}

impl Render for AssetsPanel {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .items_center()
            .justify_center()
            .child(Label::new("Assets Panel - Coming Soon"))
    }
}
