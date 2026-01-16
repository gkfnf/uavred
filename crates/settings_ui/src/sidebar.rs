use gpui::*;
use gpui_component::{button::Button, h_flex, input::InputState, label::Label, v_flex};

pub fn render_sidebar(cx: &mut Context<()>) -> impl IntoElement {
    v_flex()
        .w(px(280.0))
        .gap_4()
        .p_4()
        .border_r_1()
        .border_color(rgb(ui::theme::BORDER_COLOR))
        .child(
            v_flex()
                .gap_3()
                .pb_4()
                .border_b_1()
                .border_color(rgb(ui::theme::BORDER_COLOR))
                .child(Label::new("搜索").text_sm())
                .child(
                    v_flex()
                        .h(px(32.0))
                        .rounded_md()
                        .bg(ui::theme::BG_CARD)
                        .border_1()
                        .border_color(rgb(ui::theme::BORDER_COLOR))
                        .items_center()
                        .p_2()
                        .child(Label::new("输入以搜索...")),
                ),
        )
        .child(v_flex().flex_1().gap_2().children([
            render_category("General", true),
            render_category("Appearance", false),
            render_category("AI", false),
            render_category("Security", false),
            render_category("Network", false),
            render_category("Workflow", false),
            render_category("Scanner", false),
            render_category("Storage", false),
            render_category("Advanced", false),
        ]))
}

fn render_category(name: &str, selected: bool) -> impl IntoElement {
    v_flex()
        .p_2()
        .rounded_md()
        .h(px(36.0))
        .items_center()
        .when(selected, |this| {
            this.bg(ui::theme::ACCENT_PURPLE.with_opacity(0.1))
                .border_l_2()
                .border_color(rgb(ui::theme::ACCENT_PURPLE))
        })
        .when(!selected, |this| {
            this.bg(ui::theme::BG_CARD)
                .border_1()
                .border_color(rgb(ui::theme::BORDER_COLOR))
        })
        .child(Label::new(name).text_sm())
}
