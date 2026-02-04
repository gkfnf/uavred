use crate::image_card::ImageCard;
use crate::sandbox_manager::{SandboxEvent, SandboxManager};
use data::ContainerStatus;
use gpui::*;
use gpui::prelude::FluentBuilder;
use gpui_component::{h_flex, v_flex, label::Label, button::{Button, ButtonVariants}, Sizable, scroll::ScrollableElement};

/// Images panel - main container for image/agent management
pub struct ImagesPanel {
    containers: Vec<ContainerStatus>,
    sandbox_manager: Entity<SandboxManager>,
    _subscriptions: Vec<Subscription>,
}

impl ImagesPanel {
    pub fn new(cx: &mut Context<Self>) -> Self {
        // Create sandbox manager
        let sandbox_manager = cx.new(|cx| SandboxManager::new(cx));
        
        // Get initial containers from sandbox manager
        let initial_containers = sandbox_manager.read(cx).containers().to_vec();
        
        // Subscribe to sandbox events
        let _subscriptions = vec![
            cx.subscribe(&sandbox_manager, |this: &mut Self, _, event: &SandboxEvent, cx| {
                match event {
                    SandboxEvent::ContainersUpdated(containers) => {
                        this.containers = containers.clone();
                        cx.notify();
                    }
                    SandboxEvent::ContainerStarted(_) => {
                        cx.notify();
                    }
                    SandboxEvent::ContainerStopped(_) => {
                        cx.notify();
                    }
                    SandboxEvent::Error(err) => {
                        tracing::error!("Sandbox error: {}", err);
                    }
                }
            }),
        ];

        Self {
            containers: initial_containers,
            sandbox_manager,
            _subscriptions,
        }
    }

    pub fn sandbox_manager(&self) -> &Entity<SandboxManager> {
        &self.sandbox_manager
    }

    fn running_count(&self) -> usize {
        self.containers
            .iter()
            .filter(|c| matches!(c.status, data::ContainerExecutionStatus::Running))
            .count()
    }

    fn handle_create_image(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        tracing::info!("Create image clicked");
        cx.notify();
    }
}

impl Render for ImagesPanel {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let running_count = self.running_count();
        
        // Clone containers for the grid
        let containers = self.containers.clone();
        let container_cards: Vec<Entity<ImageCard>> = containers
            .into_iter()
            .map(|container| cx.new(|cx| ImageCard::new(container, cx)))
            .collect();

        v_flex()
            .size_full()
            .bg(rgb(0xf3f4f6))
            .child(
                // Header
                h_flex()
                    .w_full()
                    .items_center()
                    .justify_between()
                    .px(px(24.0))
                    .py(px(16.0))
                    .bg(rgb(0xffffff))
                    .border_b_1()
                    .border_color(rgb(0xe5e7eb))
                    .child(
                        h_flex()
                            .items_center()
                            .gap(px(12.0))
                            .child(
                                h_flex()
                                    .gap(px(8.0))
                                    .items_center()
                                    .child(
                                        Label::new("📦")
                                            .text_color(rgb(0x7c3aed))
                                    )
                                    .child(
                                        Label::new("容器镜像 & Agent 执行环境")
                                            .text_lg()
                                            .font_weight(FontWeight::SEMIBOLD)
                                    )
                            )
                            .when(running_count > 0, |this: Div| {
                                this.child(
                                    h_flex()
                                        .px(px(12.0))
                                        .py(px(4.0))
                                        .rounded_full()
                                        .border_1()
                                        .border_color(rgb(0x7c3aed))
                                        .child(
                                            Label::new(format!("{} 运行中", running_count))
                                                .text_sm()
                                                .text_color(rgb(0x7c3aed))
                                        )
                                )
                            })
                    )
                    .child(
                        Button::new("create-image-btn")
                            .primary()
                            .small()
                            .icon(gpui_component::IconName::Plus)
                            .label("创建镜像")
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.handle_create_image(window, cx);
                            }))
                    )
            )
            .child(
                // Grid container
                div()
                    .flex_1()
                    .p(px(24.0))
                    .overflow_y_scrollbar()
                    .child(
                        div()
                            .w_full()
                            .flex()
                            .flex_wrap()
                            .gap(px(20.0))
                            .children(container_cards)
                            .when(self.containers.is_empty(), |this: Div| {
                                this.child(
                                    v_flex()
                                        .w_full()
                                        .h(px(400.0))
                                        .items_center()
                                        .justify_center()
                                        .child(
                                            v_flex()
                                                .items_center()
                                                .gap(px(16.0))
                                                .child(
                                                    div()
                                                        .w(px(80.0))
                                                        .h(px(80.0))
                                                        .rounded_full()
                                                        .bg(rgb(0xf3f4f6))
                                                        .flex()
                                                        .items_center()
                                                        .justify_center()
                                                        .child(
                                                            Label::new("📦")
                                                                .text_2xl()
                                                        )
                                                )
                                                .child(
                                                    Label::new("暂无运行中的 Agent 沙箱")
                                                        .text_base()
                                                        .text_color(rgb(0x6b7280))
                                                )
                                                .child(
                                                    Label::new("点击右上角'创建镜像'按钮创建第一个沙箱")
                                                        .text_sm()
                                                        .text_color(rgb(0x9ca3af))
                                                )
                                        )
                                )
                            })
                    )
            )
    }
}
