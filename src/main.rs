use gpui::*;
use vibekanban::KanbanBoard;

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(1200.0), px(800.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some("VibeKanban".into()),
                    appears_transparent: false,
                    traffic_light_position: None,
                }),
                ..Default::default()
            },
            |_, cx| cx.new(|cx| KanbanBoard::new(cx)),
        )
        .unwrap();
    });
}
