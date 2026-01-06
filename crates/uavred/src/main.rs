use uavred::Workspace;
use gpui::*;
use gpui_component::Root;
use gpui_component_assets::Assets;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let app = Application::new().with_assets(Assets);

    app.run(move |cx| {
        // Initialize GPUI Component
        gpui_component::init(cx);

        // Set light theme as default (matching Figma design)
        gpui_component::Theme::change(gpui_component::ThemeMode::Light, None, cx);

        cx.spawn(async move |cx| {
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: Point {
                            x: px(0.0),
                            y: px(0.0),
                        },
                        size: Size {
                            width: px(1920.0),
                            height: px(1080.0),
                        },
                    })),
                    titlebar: Some(gpui_component::TitleBar::title_bar_options()),
                    ..Default::default()
                },
                |window, cx| {
                    let view = Workspace::new(cx);
                    // Root component is required as first level
                    cx.new(|cx| Root::new(view, window, cx))
                },
            )?;
            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
}
