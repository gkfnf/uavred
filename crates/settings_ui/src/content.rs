use gpui::*;
use gpui_component::{h_flex, v_flex, button::Button, label::Label, div};

pub fn render_content() -> impl IntoElement {
    v_flex()
        .flex_1()
        .gap_4()
        .p_6()
        .child(
            v_flex()
                .gap_2()
                .pb_4()
                .border_b_1()
                .border_color(rgb(ui::theme::BORDER_COLOR))
                .child(Label::new("General").text_xl())
                .child(Label::new("通用设置配置"))
                .child(Button::new("Edit in settings.json")),
        )
        .child(
            v_flex()
                .gap_3()
                .children([
                    render_setting_item("Auto Update", true, false),
                    render_setting_item("Language", false, true),
                    render_setting_item("Startup View", false, true),
                ]),
        )
}

fn render_setting_item(label: &str, is_toggle: bool, is_dropdown: bool) -> impl IntoElement {
    v_flex()
        .gap_2()
        .p_3()
        .rounded_md()
        .bg(ui::theme::BG_CARD)
        .border_1()
        .border_color(rgb(ui::theme::BORDER_COLOR))
        .child(
            v_flex()
                .flex_1()
                .gap_2()
                .child(Label::new(label).text_sm())
                .when(is_toggle, |_| {
                    v_flex()
                        .w(px(40.0))
                        .h(px(24.0))
                        .rounded_full()
                        .bg(ui::theme::ACCENT_PURPLE)
                })
                .when(is_dropdown, |_| {
                    v_flex()
                        .w(px(120.0))
                        .h(px(32.0))
                        .rounded_md()
                        .bg(ui::theme::BG_SECONDARY)
                        .items_center()
                        .justify_center()
                        .child(Label::new("English").text_sm()),
                }),
        )
}
