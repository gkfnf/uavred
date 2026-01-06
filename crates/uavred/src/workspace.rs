// Workspace - 顶层协调者，类似 zed 的 Workspace

use assets_ui::AssetsPanel;
use dashboard_ui::DashboardPanel;
use flows_ui::FlowsPanel;
use gpui::*;
use gpui_component::{
    badge::Badge,
    button::{Button, ButtonVariants as _},
    h_flex,
    input::InputState,
    label::Label,
    tree::{TreeItem, TreeState},
    v_flex, Root, Selectable, Sizable, TitleBar,
};
use traffic_ui::TrafficPanel;
use ui::events::WorkspaceEvent;
use vulns_ui::VulnsPanel;
use workspace::{AppView, VulnFilter};

/// Workspace - 顶层应用，协调所有面板
pub struct Workspace {
    active_view: AppView,

    // 面板 Entities
    dashboard_panel: Option<Entity<DashboardPanel>>,
    assets_panel: Option<Entity<AssetsPanel>>,
    vulns_panel: Option<Entity<VulnsPanel>>,
    traffic_panel: Option<Entity<TrafficPanel>>,
    flows_panel: Option<Entity<FlowsPanel>>,

    // 全局状态
    scan_input: Option<Entity<InputState>>,
    assets_tree: Option<Entity<TreeState>>,
    #[allow(dead_code)]
    selected_asset: Option<String>,
    vuln_filter: VulnFilter,

    // 订阅
    _subscriptions: Vec<Subscription>,
}

impl EventEmitter<WorkspaceEvent> for Workspace {}

impl Workspace {
    pub fn new(cx: &mut App) -> Entity<Self> {
        cx.new(|_| Self {
            active_view: AppView::Dashboard,
            dashboard_panel: None,
            assets_panel: None,
            vulns_panel: None,
            traffic_panel: None,
            flows_panel: None,
            scan_input: None,
            assets_tree: None,
            selected_asset: None,
            vuln_filter: VulnFilter::All,
            _subscriptions: Vec::new(),
        })
    }

    pub fn set_active_view(&mut self, view: AppView, cx: &mut Context<Self>) {
        self.active_view = view;
        cx.emit(WorkspaceEvent::ViewChanged(view));
    }

    pub fn set_vuln_filter(&mut self, filter: VulnFilter, _cx: &mut Context<Self>) {
        self.vuln_filter = filter;
    }

    fn get_or_create_dashboard_panel(&mut self, cx: &mut Context<Self>) -> Entity<DashboardPanel> {
        if let Some(ref panel) = self.dashboard_panel {
            panel.clone()
        } else {
            let panel = cx.new(|cx| DashboardPanel::new(cx));
            self.dashboard_panel = Some(panel.clone());
            panel
        }
    }

    fn get_or_create_assets_panel(&mut self, cx: &mut Context<Self>) -> Entity<AssetsPanel> {
        if let Some(ref panel) = self.assets_panel {
            panel.clone()
        } else {
            let panel = cx.new(|cx| AssetsPanel::new(cx));
            self.assets_panel = Some(panel.clone());
            panel
        }
    }

    fn get_or_create_vulns_panel(&mut self, cx: &mut Context<Self>) -> Entity<VulnsPanel> {
        if let Some(ref panel) = self.vulns_panel {
            panel.clone()
        } else {
            let panel = cx.new(|cx| VulnsPanel::new(cx));
            self.vulns_panel = Some(panel.clone());
            panel
        }
    }

    fn get_or_create_traffic_panel(&mut self, cx: &mut Context<Self>) -> Entity<TrafficPanel> {
        if let Some(ref panel) = self.traffic_panel {
            panel.clone()
        } else {
            let panel = cx.new(|cx| TrafficPanel::new(cx));
            self.traffic_panel = Some(panel.clone());
            panel
        }
    }

    fn get_or_create_flows_panel(&mut self, cx: &mut Context<Self>) -> Entity<FlowsPanel> {
        if let Some(ref panel) = self.flows_panel {
            panel.clone()
        } else {
            let panel = cx.new(|cx| FlowsPanel::new(cx));
            self.flows_panel = Some(panel.clone());
            panel
        }
    }

    fn nav_item(&mut self, cx: &mut Context<Self>, view: AppView, label: &str) -> impl IntoElement {
        let is_active = self.active_view == view;
        let badge = match view {
            AppView::Vulns => Some(2),
            AppView::Traffic => Some(8),
            _ => None,
        };

        let label_text = label.to_string();
        let mut button = Button::new(format!("nav-{:?}", view))
            .ghost()
            .small()
            .label(label_text.clone())
            .selected(is_active)
            .on_click(cx.listener(move |this: &mut Self, _, _, cx| {
                this.set_active_view(view, cx);
            }));

        if let Some(badge_count) = badge {
            button = button.child(Badge::new().count(badge_count).color(rgb(0xef4444)).small());
        }

        button
    }

    fn render_title_bar(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        TitleBar::new()
            .child(h_flex().items_center().gap(px(16.0)).child(
                h_flex().gap(px(4.0)).items_center().children(vec![
                    self.nav_item(cx, AppView::Dashboard, "Dashboard"),
                    self.nav_item(cx, AppView::Assets, "Assets"),
                    self.nav_item(cx, AppView::Images, "Images"),
                    self.nav_item(cx, AppView::Vulns, "Vulns"),
                    self.nav_item(cx, AppView::Traffic, "Traffic"),
                    self.nav_item(cx, AppView::Flows, "Flows"),
                    self.nav_item(cx, AppView::Devices, "Devices"),
                    self.nav_item(cx, AppView::Settings, "Settings"),
                ]),
            ))
            .child(
                h_flex().items_center().gap(px(16.0)).child(
                    h_flex()
                        .gap(px(4.0))
                        .items_center()
                        .child(div().w(px(8.0)).h(px(8.0)).rounded_full().bg(rgb(0x10b981)))
                        .child(Label::new("AI Active").text_sm().text_color(rgb(0x10b981))),
                ),
            )
    }

    fn render_main_content(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        // 初始化 Entity（延迟初始化）
        if self.scan_input.is_none() {
            self.scan_input = Some(cx.new(|cx| InputState::new(window, cx)));
        }
        if self.assets_tree.is_none() {
            self.assets_tree = Some(cx.new(|cx| {
                TreeState::new(cx).items(vec![
                    TreeItem::new("network", "Network")
                        .expanded(true)
                        .children(vec![
                            TreeItem::new("device-1", "mavic-3-pro-v2.local")
                                .child(TreeItem::new("device-1-info", "Device Info")),
                            TreeItem::new("device-2", "phantom-4.local"),
                        ]),
                    TreeItem::new("firmware", "Firmware")
                        .expanded(false)
                        .children(vec![
                            TreeItem::new("fw-1", "DJI_FW_v1.0.0.bin"),
                            TreeItem::new("fw-2", "DJI_FW_v1.1.0.bin"),
                        ]),
                    TreeItem::new("configs", "Configurations"),
                ])
            }));
        }

        div().flex_1().child(match self.active_view {
            AppView::Dashboard => {
                let panel = self.get_or_create_dashboard_panel(cx);
                panel.clone().into_any_element()
            }
            AppView::Assets => {
                let panel = self.get_or_create_assets_panel(cx);
                panel.clone().into_any_element()
            }
            AppView::Images => {
                // TODO: 实现 Images 面板
                div()
                    .child(Label::new("Images - Coming Soon"))
                    .into_any_element()
            }
            AppView::Vulns => {
                let panel = self.get_or_create_vulns_panel(cx);
                panel.clone().into_any_element()
            }
            AppView::Traffic => {
                let panel = self.get_or_create_traffic_panel(cx);
                panel.clone().into_any_element()
            }
            AppView::Flows => {
                let panel = self.get_or_create_flows_panel(cx);
                panel.clone().into_any_element()
            }
            AppView::Devices => {
                // TODO: 实现 Devices 面板
                div()
                    .child(Label::new("Devices - Coming Soon"))
                    .into_any_element()
            }
            AppView::Settings => {
                // TODO: 实现 Settings 面板
                div()
                    .child(Label::new("Settings - Coming Soon"))
                    .into_any_element()
            }
        })
    }
}

impl Render for Workspace {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // 渲染对话框和通知层
        let dialog_layer = Root::render_dialog_layer(window, cx);
        let notification_layer = Root::render_notification_layer(window, cx);

        v_flex()
            .size_full()
            .bg(rgb(0xffffff))
            .child(self.render_title_bar(window, cx))
            .child(self.render_main_content(window, cx))
            .children(dialog_layer)
            .children(notification_layer)
    }
}
