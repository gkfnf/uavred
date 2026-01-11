use gpui::*;
use gpui_component::{
    h_flex,
    label::Label,
    list::ListItem,
    tree::{tree, TreeItem, TreeState},
    v_flex, IconName,
};

pub struct AssetsPanel {
    tree_state: Entity<TreeState>,
}

impl AssetsPanel {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let tree_state = cx.new(|cx| {
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
        });

        Self { tree_state }
    }
}

impl Render for AssetsPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .size_full()
            .gap_4()
            .child(
                v_flex()
                    .flex_1()
                    .gap_4()
                    .p_4()
                    .border_r_1()
                    .border_color(rgb(0xe5e7eb))
                    .rounded_lg()
                    .overflow_hidden()
                    .children(vec![
                        {
                            div()
                                .items_center()
                                .justify_between()
                                .p_4()
                                .border_b_1()
                                .border_color(rgb(0xe5e7eb))
                                .child(Label::new("Assets").text_lg())
                        },
                        {
                            v_flex().flex_1().p_3().child(tree(
                                &self.tree_state,
                                |ix, entry, selected, _, _| {
                                    let item = entry.item();
                                    let icon = if !entry.is_folder() {
                                        IconName::File
                                    } else if entry.is_expanded() {
                                        IconName::FolderOpen
                                    } else {
                                        IconName::Folder
                                    };

                                    ListItem::new(ix)
                                        .selected(selected)
                                        .pl(px(16.) * entry.depth())
                                        .child(
                                            h_flex()
                                                .gap_2()
                                                .items_center()
                                                .child(icon)
                                                .child(item.label.clone()),
                                        )
                                },
                            ))
                        },
                    ]),
            )
            .child({
                let selected_item = self.tree_state.read(cx).selected_item().cloned();

                v_flex()
                    .flex_1()
                    .p_4()
                    .border_l_1()
                    .border_color(rgb(0xe5e7eb))
                    .rounded_lg()
                    .overflow_hidden()
                    .child(if let Some(item) = selected_item {
                        let label = item.label.to_string();
                        let item_id = item.id.to_string();
                        let children_count = item.children.len().to_string();
                        let is_expanded = if item.is_expanded() {
                            "Yes".to_string()
                        } else {
                            "No".to_string()
                        };
                        let is_folder = if item.is_folder() {
                            "Folder".to_string()
                        } else {
                            "File".to_string()
                        };
                        let path = item.label.to_string();

                        div()
                            .flex_1()
                            .p_6()
                            .child(
                                div()
                                    .flex_1()
                                    .gap_2()
                                    .pb_4()
                                    .border_b_1()
                                    .border_color(rgb(0xe5e7eb))
                                    .child(
                                        h_flex()
                                            .gap_2()
                                            .items_center()
                                            .child(IconName::Folder)
                                            .child(Label::new(label).text_xl()),
                                    ),
                            )
                            .child(div().flex_1().gap_3().children(vec![
                                        div()
                                            .flex_grow()
                                            .gap_3()
                                            .p_2()
                                            .rounded_md()
                                            .bg(rgb(0xf9fafb))
                                            .children(vec![
                                                Label::new("ID")
                                                    .text_sm()
                                                    .text_color(rgb(0x6b7280))
                                                    .font_weight(FontWeight::MEDIUM)
                                                    .w(px(80.)),
                                                Label::new(&item_id)
                                                    .text_sm()
                                                    .text_color(rgb(0x1f2937))
                                                    .flex_1(),
                                            ]),
                                        div()
                                            .flex_grow()
                                            .gap_3()
                                            .p_2()
                                            .rounded_md()
                                            .bg(rgb(0xf9fafb))
                                            .children(vec![
                                                Label::new("Type")
                                                    .text_sm()
                                                    .text_color(rgb(0x6b7280))
                                                    .font_weight(FontWeight::MEDIUM)
                                                    .w(px(80.)),
                                                Label::new(&is_folder)
                                                    .text_sm()
                                                    .text_color(rgb(0x1f2937))
                                                    .flex_1(),
                                            ]),
                                        div()
                                            .flex_grow()
                                            .gap_3()
                                            .p_2()
                                            .rounded_md()
                                            .bg(rgb(0xf9fafb))
                                            .children(vec![
                                                Label::new("Children")
                                                    .text_sm()
                                                    .text_color(rgb(0x6b7280))
                                                    .font_weight(FontWeight::MEDIUM)
                                                    .w(px(80.)),
                                                Label::new(&children_count)
                                                    .text_sm()
                                                    .text_color(rgb(0x1f2937))
                                                    .flex_1(),
                                            ]),
                                        div()
                                            .flex_grow()
                                            .gap_3()
                                            .p_2()
                                            .rounded_md()
                                            .bg(rgb(0xf9fafb))
                                            .children(vec![
                                                Label::new("Expanded")
                                                    .text_sm()
                                                    .text_color(rgb(0x6b7280))
                                                    .font_weight(FontWeight::MEDIUM)
                                                    .w(px(80.)),
                                                Label::new(&is_expanded)
                                                    .text_sm()
                                                    .text_color(rgb(0x1f2937))
                                                    .flex_1(),
                                            ]),
                                        div()
                                            .flex_grow()
                                            .gap_3()
                                            .p_2()
                                            .rounded_md()
                                            .bg(rgb(0xf9fafb))
                                            .children(vec![
                                                Label::new("Path")
                                                    .text_sm()
                                                    .text_color(rgb(0x6b7280))
                                                    .font_weight(FontWeight::MEDIUM)
                                                    .w(px(80.)),
                                                Label::new(&path)
                                                    .text_sm()
                                                    .text_color(rgb(0x1f2937))
                                                    .flex_1(),
                                            ]),
                                    ]))
                    } else {
                        div().flex_1().items_center().justify_center().p_6().child(
                            Label::new("Select an asset to view details")
                                .text_sm()
                                .text_color(rgb(0x9ca3af)),
                        )
                    })
            })
    }
}
