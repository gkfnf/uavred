use super::pane::Pane;
use gpui::{Context, Entity, EventEmitter, InteractiveElement, IntoElement, Pixels, Point, Window};

#[derive(Debug, Clone)]
pub enum ResizeEvent {
    ResizeStarted,
    Resizing {
        axis: Axis,
        initial_position: Point<Pixels>,
    },
    ResizeEnded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    Horizontal,
    Vertical,
}

impl EventEmitter<ResizeEvent> for Pane {}

impl Pane {
    pub fn handle_resize_start(&mut self, _event: &ResizeEvent, cx: &mut Context<Self>) {
        cx.emit(ResizeEvent::ResizeStarted);
    }

    pub fn handle_resizing(&mut self, event: &ResizeEvent, cx: &mut Context<Self>) {
        if let ResizeEvent::Resizing {
            axis,
            initial_position,
        } = event
        {
            cx.emit(event.clone());
        }
    }

    pub fn handle_resize_end(&mut self, _event: &ResizeEvent, cx: &mut Context<Self>) {
        cx.emit(ResizeEvent::ResizeEnded);
    }
}

pub struct ResizeHandle {
    pane: Entity<Pane>,
    axis: Axis,
    width: Pixels,
}

impl ResizeHandle {
    pub fn new(pane: Entity<Pane>, axis: Axis) -> Self {
        Self {
            pane,
            axis,
            width: px(10.0),
        }
    }

    pub fn render(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let style = match self.axis {
            Axis::Horizontal => gpui::div()
                .size(px(10.0))
                .h(gpui::size_full().height() - px(20.0))
                .bg(rgb(0xe5e7eb))
                .cursor_col_resize()
                .when(true, |div| div.left(px(10.0)).w(px(10.0)).h_full())
                .when(true, |div| div.right(px(10.0)).w(px(10.0)).h_full()),
            Axis::Vertical => gpui::div()
                .size(gpui::size_full().width() - px(20.0), px(10.0))
                .bg(rgb(0xe5e7eb))
                .cursor_row_resize()
                .when(true, |div| div.top(px(10.0)).w_full().h(px(10.0)))
                .when(true, |div| div.bottom(px(10.0)).w_full().h(px(10.0))),
        };

        style
            .on_click(cx.listener(|this, _, cx| {
                this.pane.update(|pane, cx| {
                    pane.handle_resize_start(&ResizeEvent::ResizeStarted, cx);
                });
            }))
            .on_mouse_down(cx.listener(|this, event, cx| {
                this.pane.update(|pane, cx| {
                    pane.handle_resize_start(
                        &ResizeEvent::Resizing {
                            axis: this.axis,
                            initial_position: event.position,
                        },
                        cx,
                    );
                });
            }))
            .on_mouse_move(cx.listener(|this, event, cx| {
                this.pane.update(|pane, cx| {
                    pane.handle_resizing(
                        &ResizeEvent::Resizing {
                            axis: this.axis,
                            initial_position: Point {
                                x: px(0.0),
                                y: px(0.0),
                            },
                        },
                        cx,
                    );
                });
            }))
            .on_mouse_up(cx.listener(|this, _, cx| {
                this.pane.update(|pane, cx| {
                    pane.handle_resize_end(&ResizeEvent::ResizeEnded, cx);
                });
            }))
    }
}
