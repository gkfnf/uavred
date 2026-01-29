//! Panel Header Component
//!
//! 统一的栏头部组件，确保三栏分界线对齐

use gpui::prelude::FluentBuilder as _;
use gpui::*;
use gpui_component::{h_flex, v_flex};
use ui::theme::*;

/// 栏头部组件（无下边框，用于左侧栏）
#[derive(IntoElement)]
pub struct PanelHeader {
    title: SharedString,
    badge: Option<SharedString>,
    badge_color: Option<u32>,
    actions: Vec<AnyElement>,
    show_border: bool,
}

impl PanelHeader {
    /// 创建新的栏头部
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            badge: None,
            badge_color: None,
            actions: Vec::new(),
            show_border: true,
        }
    }

    /// 设置徽章
    pub fn badge(mut self, text: impl Into<SharedString>, color: u32) -> Self {
        self.badge = Some(text.into());
        self.badge_color = Some(color);
        self
    }

    /// 添加操作按钮
    pub fn action(mut self, action: impl IntoElement) -> Self {
        self.actions.push(action.into_any_element());
        self
    }

    /// 设置是否显示下边框
    pub fn show_border(mut self, show: bool) -> Self {
        self.show_border = show;
        self
    }
}

impl RenderOnce for PanelHeader {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        // 内容区域高度（不包含边框）
        let content_height: f32 = HEADER_HEIGHT.into();
        let content_height = if self.show_border {
            px(content_height - 1.0)
        } else {
            HEADER_HEIGHT
        };

        let mut element = h_flex()
            .w_full()
            .h(content_height)
            .px(PADDING_LG)
            .items_center()
            .justify_between()
            // 左侧：标题 + 徽章
            .child(
                h_flex()
                    .items_center()
                    .gap(SPACING_SM)
                    .child(
                        div()
                            .text_base()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(TEXT_PRIMARY))
                            .child(self.title),
                    )
                    .when_some(self.badge, |this, badge| {
                        let color = self.badge_color.unwrap_or(TEXT_MUTED);
                        this.child(
                            div()
                                .px(PADDING_SM)
                                .py(px(2.0))
                                .rounded_full()
                                .bg(rgb(color))
                                .text_xs()
                                .text_color(rgb(0xffffff))
                                .child(badge),
                        )
                    }),
            );

        // 右侧：操作按钮
        if !self.actions.is_empty() {
            element = element.child(h_flex().items_center().gap(SPACING_SM).children(self.actions));
        }

        // 添加下边框
        if self.show_border {
            element = element.border_b_1().border_color(rgb(BORDER_COLOR));
        }

        element
    }
}

/// 筛选区域组件 - 放在分界线下方
#[derive(IntoElement)]
pub struct FilterSection {
    children: Vec<AnyElement>,
}

impl FilterSection {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }
}

impl RenderOnce for FilterSection {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        v_flex()
            .w_full()
            .p(PADDING_LG)
            .gap(SPACING_SM)
            .children(self.children)
    }
}
