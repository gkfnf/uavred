pub mod container_card;

pub use container_card::ContainerCard;

use data::ContainerStatus;
use gpui::prelude::*;
use gpui::*;
use gpui_component::v_flex;

pub struct MonitorPanel {
    containers: Vec<ContainerStatus>,
}

impl MonitorPanel {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self {
            containers: Vec::new(),
        }
    }

    pub fn set_containers(&mut self, containers: Vec<ContainerStatus>, cx: &mut Context<Self>) {
        self.containers = containers;
        cx.notify();
    }
}

impl Render for MonitorPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let containers: Vec<Entity<ContainerCard>> = self
            .containers
            .iter()
            .map(|container| cx.new(|cx| ContainerCard::new(container.clone(), cx)))
            .collect();

        v_flex()
            .size_full()
            .bg(rgb(0xf5f5f5))
            .gap(px(16.0))
            .p(px(24.0))
            .when(!self.containers.is_empty(), |this: Div| {
                this.children(containers)
            })
            .when(self.containers.is_empty(), |this: Div| {
                this.child(
                    div().flex_1().items_center().justify_center().child(
                        gpui_component::label::Label::new("No containers running")
                            .text_lg()
                            .text_color(rgb(0x6b7280)),
                    ),
                )
            })
    }
}
