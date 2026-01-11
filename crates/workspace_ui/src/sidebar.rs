use gpui::*;
use gpui_component::label::Label;
use crate::events::SidebarEvent;

pub struct SimpleSidebar {
    pub is_open: bool,
    pub items: Vec<String>,
}

impl EventEmitter<SidebarEvent> for SimpleSidebar {}

impl SimpleSidebar {
    pub fn new() -> Self {
        Self {
            is_open: false,
            items: vec![
                "Dashboard".to_string(),
                "Assets".to_string(),
                "Scan".to_string(),
                "Vulns".to_string(),
                "Traffic".to_string(),
                "Flows".to_string(),
                "Devices".to_string(),
                "Settings".to_string(),
            ],
        }
    }

    pub fn toggle(&mut self, cx: &mut Context<Self>) {
        self.is_open = !self.is_open;
        if self.is_open {
            cx.emit(SidebarEvent::Opened);
        } else {
            cx.emit(SidebarEvent::Closed);
        }
    }

    pub fn open(&mut self, cx: &mut Context<Self>) {
        if !self.is_open {
            self.is_open = true;
            cx.emit(SidebarEvent::Opened);
        }
    }

    pub fn close(&mut self, cx: &mut Context<Self>) {
        if self.is_open {
            self.is_open = false;
            cx.emit(SidebarEvent::Closed);
        }
    }

    pub fn is_open(&self) -> bool {
        self.is_open
    }
}

impl Render for SimpleSidebar {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if self.is_open {
            div()
                .w(px(300.0))
                .h_full()
                .bg(rgb(0xf9fafb))
                .border_r_1()
                .border_color(rgb(0xe5e7eb))
                .p(px(16.0))
                .child(
                    div().flex().flex_col().gap(px(16.0)).children(
                        self.items
                            .iter()
                            .map(|item| div().p(px(8.0)).child(Label::new(item))),
                    ),
                )
        } else {
            div()
        }
    }
}
