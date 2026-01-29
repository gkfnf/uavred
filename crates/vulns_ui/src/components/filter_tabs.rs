//! Filter Tabs Component
//!
//! 筛选标签组件，用于 Vulnerabilities 栏的 Severity/Asset/MITRE 筛选

use gpui::*;
use gpui_component::h_flex;use std::rc::Rc;
use ui::theme::*;

/// 筛选标签项
#[derive(Clone, Debug)]
pub struct FilterTabItem {
    pub id: SharedString,
    pub label: SharedString,
}

impl FilterTabItem {
    pub fn new(id: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
        }
    }
}

/// 筛选标签组件
#[derive(IntoElement)]
pub struct FilterTabs {
    tabs: Vec<FilterTabItem>,
    active_id: SharedString,
    on_change: Option<Rc<dyn Fn(&SharedString, &mut Window, &mut App) + 'static>>,
}

impl FilterTabs {
    /// 创建新的筛选标签组
    pub fn new(active_id: impl Into<SharedString>) -> Self {
        Self {
            tabs: Vec::new(),
            active_id: active_id.into(),
            on_change: None,
        }
    }

    /// 添加标签
    pub fn tab(mut self, id: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        self.tabs.push(FilterTabItem::new(id, label));
        self
    }

    /// 设置切换回调（接受引用签名，适配 cx.listener）
    pub fn on_change(
        mut self,
        handler: impl Fn(&SharedString, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Rc::new(move |id, window, cx| {
            handler(&id, window, cx);
        }));
        self
    }
}

impl RenderOnce for FilterTabs {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        h_flex()
            .items_center()
            .gap(SPACING_XS)
            .children(self.tabs.iter().map(|tab| {
                let is_active = tab.id == self.active_id;
                let (bg_color, text_color) = if is_active {
                    (ACCENT_PURPLE, 0xffffff)
                } else {
                    (BG_SECONDARY, TEXT_SECONDARY)
                };

                let tab_id = tab.id.clone();
                let on_change = self.on_change.clone();

                div()
                    .px(PADDING_SM)
                    .py(px(4.0))
                    .rounded(BORDER_RADIUS_SM)
                    .bg(rgb(bg_color))
                    .text_xs()
                    .text_color(rgb(text_color))
                    .cursor_pointer()
                    .child(tab.label.clone())
                    .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                        if let Some(ref handler) = on_change {
                            handler(&tab_id, window, cx);
                        }
                    })
            }))
    }
}

/// 创建简单的标签按钮
pub fn tab_button(
    label: &str,
    is_active: bool,
    on_click: impl Fn(&MouseDownEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    let (bg_color, text_color) = if is_active {
        (ACCENT_PURPLE, 0xffffff)
    } else {
        (BG_SECONDARY, TEXT_SECONDARY)
    };

    div()
        .px(PADDING_MD)
        .py(px(6.0))
        .rounded(BORDER_RADIUS)
        .bg(rgb(bg_color))
        .text_sm()
        .text_color(rgb(text_color))
        .cursor_pointer()
        .child(label.to_string())
        .on_mouse_down(MouseButton::Left, on_click)
}
