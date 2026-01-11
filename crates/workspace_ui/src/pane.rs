use super::item::ItemHandle;
use gpui::{FocusHandle, ParentElement, Render, Styled, Window};

#[derive(Clone, Debug, PartialEq)]
pub enum PaneAction {
    CloseActiveItem,
    CloseOtherItems,
    CloseAllItems,
    ActivateItem(usize),
    SplitHorizontal,
    SplitVertical,
    Zoom,
    Unzoom,
}

pub struct Pane {
    items: Vec<Box<dyn ItemHandle>>,
    active_item_index: usize,
    focus_handle: FocusHandle,
    zoomed: bool,
}

impl gpui::EventEmitter<PaneAction> for Pane {}

impl Pane {
    pub fn new(focus_handle: FocusHandle) -> Self {
        Self {
            items: Vec::new(),
            active_item_index: 0,
            focus_handle,
            zoomed: false,
        }
    }

    pub fn add_item(&mut self, item: Box<dyn ItemHandle>) {
        self.items.push(item);
    }

    pub fn activate_item(&mut self, index: usize) {
        if index < self.items.len() {
            self.active_item_index = index;
        }
    }

    pub fn active_item(&self) -> Option<&dyn ItemHandle> {
        if self.active_item_index < self.items.len() {
            Some(self.items[self.active_item_index].as_ref())
        } else {
            None
        }
    }

    pub fn close_item(&mut self, index: usize) {
        if index < self.items.len() {
            self.items.remove(index);
            if self.active_item_index >= self.items.len() && !self.items.is_empty() {
                self.active_item_index = self.items.len() - 1;
            }
        }
    }

    pub fn close_active_item(&mut self) {
        if !self.items.is_empty() {
            self.close_item(self.active_item_index);
        }
    }

    pub fn toggle_zoom(&mut self) {
        self.zoomed = !self.zoomed;
    }

    pub fn is_zoomed(&self) -> bool {
        self.zoomed
    }

    pub fn item_count(&self) -> usize {
        self.items.len()
    }

    pub fn focus_handle(&self) -> &FocusHandle {
        &self.focus_handle
    }
}

impl Render for Pane {
    fn render(
        &mut self,
        _window: &mut Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        if let Some(item) = self.active_item() {
            let _title = item.title();
            let _icon = item.icon();

            gpui::div()
                .flex()
                .flex_col()
                .size_full()
                .child(gpui::div().child(format!("Pane: {}", item.title())))
        } else {
            gpui::div()
                .flex()
                .flex_col()
                .size_full()
                .items_center()
                .justify_center()
                .child(gpui::div().child("Empty Pane"))
        }
    }
}
