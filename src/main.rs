mod app;
mod ui;
mod agent;
mod scanner;
mod core;

use gpui::*;
use app::UavRedTeamApp;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    App::new().run(|cx: &mut AppContext| {
        cx.open_window(WindowOptions::default(), |cx| {
            cx.new_view(|cx| UavRedTeamApp::new(cx))
        });
    });
}
