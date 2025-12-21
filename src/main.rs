use gpui::*;
use std::sync::Arc;

mod kanban;
use kanban::KanbanBoard;

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.open_window(
            WindowOptions {
                titlebar: Some(TitlebarOptions {
                    title: Some("VibeKanban".into()),
                    appears_transparent: false,
                    traffic_light_position: None,
                }),
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: point(px(100.0), px(100.0)),
                    size: size(px(1200.0), px(800.0)),
                })),
                focus: true,
                show: true,
                kind: WindowKind::Normal,
                is_movable: true,
                display_id: None,
                window_background: WindowBackgroundAppearance::Opaque,
                app_id: None,
            },
            |cx| cx.new_view(|cx| KanbanBoard::new(cx)),
        )
        .unwrap();
    });
}
