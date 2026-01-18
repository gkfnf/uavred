use gpui::*;
use gpui_component::{h_flex, label::Label, IconName};
use ui::theme::*;

#[derive(Clone, Debug)]
pub struct CollapsibleRowState {
    pub title: SharedString,
    pub is_expanded: bool,
}

impl CollapsibleRowState {
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            is_expanded: true,
        }
    }

    pub fn expanded(mut self, expanded: bool) -> Self {
        self.is_expanded = expanded;
        self
    }

    pub fn toggle(&mut self) {
        self.is_expanded = !self.is_expanded;
    }
}

pub fn render_collapsible_row_header(title: impl Into<SharedString>, icon: IconName, is_expanded: bool) -> impl IntoElement {
    let title: SharedString = title.into();
    
    h_flex()
        .gap_2()
        .p_3()
        .border_b_1()
        .border_color(rgb(BORDER_COLOR))
        .bg(rgb(BG_PRIMARY))
        .items_center()
        .child(
            if is_expanded {
                IconName::ChevronDown
            } else {
                IconName::ChevronRight
            }
        )
        .child(icon)
        .child(
            Label::new(title)
                .text_sm()
                .font_weight(FontWeight::SEMIBOLD)
        )
}
