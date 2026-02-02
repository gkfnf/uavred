use gpui::*;
use gpui_component::{h_flex, label::Label, v_flex};
use ui::theme::*;

#[derive(Clone)]
pub struct PortItem {
    pub port: u16,
    pub protocol: String,
    pub service: Option<String>,
}

pub fn render_port_list(ports: &[PortItem]) -> impl IntoElement {
    if ports.is_empty() {
        return div()
            .flex_grow()
            .p_3()
            .rounded_md()
            .bg(rgb(BG_SECONDARY))
            .items_center()
            .justify_center()
            .child(
                Label::new("No open ports")
                    .text_sm()
                    .text_color(rgb(TEXT_MUTED)),
            )
            .into_any_element();
    }

    v_flex()
        .flex_grow()
        .gap_2()
        .p_3()
        .rounded_md()
        .bg(rgb(BG_SECONDARY))
        .max_h(px(200.0))
        .overflow_hidden()
        .children(ports.iter().map(|port| {
            h_flex()
                .gap_2()
                .items_center()
                .py_1()
                .px_2()
                .rounded_sm()
                .bg(rgb(BG_PRIMARY))
                .children(vec![
                    Label::new(format!("{}", port.port))
                        .text_xs()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(ACCENT_BLUE))
                        .w(px(40.)),
                    Label::new(port.protocol.clone())
                        .text_xs()
                        .text_color(rgb(TEXT_SECONDARY))
                        .w(px(50.)),
                    Label::new(
                        port.service
                            .clone()
                            .unwrap_or_else(|| "Unknown".to_string()),
                    )
                    .text_xs()
                    .text_color(rgb(TEXT_MUTED))
                    .flex_1(),
                ])
        }))
        .into_any_element()
}
