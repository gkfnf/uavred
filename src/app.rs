use gpui::*;
use crate::ui::prelude::*;
use ui::{v_flex, h_flex};
use crate::ui::navigation::{NavigationBar, NavigationTab};
use crate::ui::kanban::KanbanBoard;
use crate::ui::findings::FindingsView;
use crate::ui::agent_panel::AgentPanel;
use crate::core::task::{Task, TaskType, TaskPriority};

pub struct UavRedTeamApp {
    navigation: View<NavigationBar>,
    kanban: View<KanbanBoard>,
    findings: View<FindingsView>,
    agent_panel: View<AgentPanel>,
    active_view: AppView,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppView {
    MissionControl,
    Findings,
}

impl UavRedTeamApp {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        // Create Kanban board with sample tasks
        let mut kanban = cx.new_view(|cx| KanbanBoard::new(cx));
        kanban.update(cx, |board, _| {
            board.add_task(Task::new(
                "Analyze Flight Logs".to_string(),
                TaskType::ProtocolAnalysis {
                    target: "mavic-3-pro".to_string(),
                    protocol: "MAVLink".to_string(),
                },
                TaskPriority::Medium,
            ));
            
            board.add_task(Task::new(
                "Check Firmware Version".to_string(),
                TaskType::FirmwareAnalysis {
                    path: "/firmware".to_string(),
                },
                TaskPriority::Low,
            ));
        });

        Self {
            navigation: cx.new_view(|cx| NavigationBar::new(cx)),
            kanban,
            findings: cx.new_view(|cx| FindingsView::new(cx)),
            agent_panel: cx.new_view(|cx| AgentPanel::new(cx)),
            active_view: AppView::MissionControl,
        }
    }
}

impl Render for UavRedTeamApp {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .bg(gpui::rgb(0x1e1e1e))
            .child(self.navigation.clone())
            .child(
                h_flex()
                    .flex_1()
                    .child(
                        match self.active_view {
                            AppView::MissionControl => self.kanban.clone().into_any_element(),
                            AppView::Findings => self.findings.clone().into_any_element(),
                        }
                    )
                    .child(self.agent_panel.clone())
            )
    }
}
