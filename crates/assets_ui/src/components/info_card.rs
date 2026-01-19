use gpui::*;
use gpui_component::{h_flex, label::Label};
use ui::theme::*;

pub fn render_info_card(label: impl Into<SharedString>, value: impl Into<SharedString>) -> AnyElement {
    let label: SharedString = label.into();
    let value: SharedString = value.into();
    
    h_flex()
        .gap_3()
        .p_2()
        .rounded_md()
        .bg(rgb(BG_SECONDARY))
        .children(vec![
            Label::new(label)
                .text_sm()
                .text_color(rgb(TEXT_SECONDARY))
                .font_weight(FontWeight::MEDIUM)
                .w(px(80.)),
            Label::new(value)
                .text_sm()
                .text_color(rgb(TEXT_PRIMARY))
                .flex_1(),
        ])
        .into_any_element()
}

pub struct InfoCard {
    label: SharedString,
    value: SharedString,
}

impl InfoCard {
    pub fn new(label: impl Into<SharedString>, value: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
        }
    }
}

impl RenderOnce for InfoCard {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        render_info_card(self.label, self.value)
    }
}
