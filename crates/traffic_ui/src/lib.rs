<<<<<<< Current (Your changes)
// Traffic UI 模块
// T1-8, T1-9: Traffic 查询栏和表格组件
=======
pub mod request_response;
pub mod actions_panel;

pub use request_response::RequestResponsePanel;
pub use actions_panel::{ActionsPanel, TrafficStatistics, ProtocolStat};
>>>>>>> Incoming (Background Agent changes)

use gpui::*;
use gpui_component::{label::Label, v_flex};

mod query_bar;
mod traffic_table;

pub use query_bar::QueryBar;
pub use traffic_table::TrafficTable;

/// Traffic 面板
pub struct TrafficPanel {
    _placeholder: (),
}

impl TrafficPanel {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self { _placeholder: () }
    }
}

impl Render for TrafficPanel {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .items_center()
            .justify_center()
            .child(Label::new("Traffic Panel - Coming Soon"))
    }
}
