use gpui::*;

use data::models::AssetNode;

pub struct NodeDetailStub {
    _placeholder: (),
}

impl NodeDetailStub {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self { _placeholder: () }
    }

    pub fn set_node(&mut self, _node: AssetNode, _cx: &mut Context<Self>) {}

    pub fn clear_node(&mut self, _cx: &mut Context<Self>) {}
}

impl Render for NodeDetailStub {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .child("Node detail stub placeholder")
    }
}
