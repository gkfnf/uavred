use gpui::*;

#[derive(IntoElement)]
pub struct Card {
    child: AnyElement,
}

impl Card {
    pub fn new(child: impl IntoElement) -> Self {
        Self {
            child: child.into_any_element(),
        }
    }
}

impl RenderOnce for Card {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        div()
            .bg(gpui::rgb(0x2d2d2d))
            .rounded_md()
            .p_4()
            .child(self.child)
    }
}

pub fn card(child: impl IntoElement) -> Card {
    Card::new(child)
}