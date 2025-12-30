use gpui::*;
use crate::ui::prelude::*;
use crate::ui::{v_flex, label};

pub struct AgentsPanel {
    // TODO: Add agent list and management functionality
}

impl AgentsPanel {
    pub fn new(_cx: &mut ViewContext<Self>) -> Self {
        Self {}
    }
}

impl Render for AgentsPanel {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .child(label("Agents Panel - Coming Soon"))
    }
}
