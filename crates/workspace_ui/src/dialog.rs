use gpui::{Context, EventEmitter, IntoElement, Pixels, Render, Window};
use gpui_component::{button::Button, div, Modal};

#[derive(Debug, Clone)]
pub enum DialogEvent {
    Closed,
    Accepted,
}

pub struct SimpleDialog {
    title: String,
    content: String,
    is_open: bool,
}

impl EventEmitter<DialogEvent> for SimpleDialog {}

impl SimpleDialog {
    pub fn new(title: String, content: String) -> Self {
        Self {
            title,
            content,
            is_open: false,
        }
    }

    pub fn open(&mut self, cx: &mut Context<Self>) {
        self.is_open = true;
        cx.notify();
    }

    pub fn close(&mut self, cx: &mut Context<Self>) {
        self.is_open = false;
        cx.emit(DialogEvent::Closed);
    }

    pub fn is_open(&self) -> bool {
        self.is_open
    }
}

impl Render for SimpleDialog {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.is_open {
            Modal::new(window, cx).max_w(px(800.0)).child(
                div()
                    .flex()
                    .flex_col()
                    .bg(rgb(0xffffff))
                    .border_1()
                    .border_color(rgb(0xe5e7eb))
                    .rounded(px(8.0))
                    .p(px(24.0))
                    .min_h(px(200.0))
                    .children(vec![
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .pb(px(16.0))
                            .child(gpui::div().child(self.title.clone()))
                            .child(gpui::div().child("×")),
                        div()
                            .flex_1()
                            .pb(px(16.0))
                            .child(gpui::div().child(self.content.clone())),
                        div()
                            .flex()
                            .items_center()
                            .gap(px(12.0))
                            .pt(px(16.0))
                            .pb(px(16.0))
                            .children(vec![
                                Button::new("cancel").label("Cancel").ghost().on_click(
                                    cx.listener(|this, _, cx| {
                                        this.close(cx);
                                    }),
                                ),
                                Button::new("accept").label("OK").on_click(cx.listener(
                                    |this, _, cx| {
                                        this.close(cx);
                                        cx.emit(DialogEvent::Accepted);
                                    },
                                )),
                            ]),
                    ]),
            )
        } else {
            div()
        }
    }
}
