use gpui::*;
use crate::ui::prelude::*;
use crate::ui::{v_flex, label};

pub struct ResultsPanel {
    // TODO: Add scan results display functionality
}

impl ResultsPanel {
    pub fn new(_cx: &mut ViewContext<Self>) -> Self {
        Self {}
    }
}

impl Render for ResultsPanel {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .child(label("Results Panel - Coming Soon"))
    }
}
