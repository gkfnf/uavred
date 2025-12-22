use gpui::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Card {
    pub id: usize,
    pub title: String,
    pub description: String,
    pub color: Hsla,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Column {
    pub id: usize,
    pub title: String,
    pub cards: Vec<Card>,
    pub color: Hsla,
}

pub struct KanbanBoard {
    columns: Vec<Column>,
    next_card_id: usize,
    next_column_id: usize,
    dragging_card: Option<(usize, usize)>, // (column_id, card_id)
}

impl KanbanBoard {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        let mut board = Self {
            columns: Vec::new(),
            next_card_id: 0,
            next_column_id: 0,
            dragging_card: None,
        };

        // 初始化示例数据
        board.add_column("待办".to_string(), hsla(0.6, 0.5, 0.7, 1.0));
        board.add_column("进行中".to_string(), hsla(0.15, 0.7, 0.7, 1.0));
        board.add_column("已完成".to_string(), hsla(0.35, 0.6, 0.7, 1.0));

        // 添加示例卡片
        board.add_card(
            0,
            "设计 UI 界面".to_string(),
            "使用 GPUI 创建看板界面".to_string(),
        );
        board.add_card(
            0,
            "实现拖拽功能".to_string(),
            "添加卡片拖拽交互".to_string(),
        );
        board.add_card(
            1,
            "编写核心逻辑".to_string(),
            "实现看板数据模型".to_string(),
        );
        board.add_card(
            2,
            "项目初始化".to_string(),
            "创建 Cargo 项目并配置依赖".to_string(),
        );

        board
    }

    fn add_column(&mut self, title: String, color: Hsla) {
        let column = Column {
            id: self.next_column_id,
            title,
            cards: Vec::new(),
            color,
        };
        self.columns.push(column);
        self.next_column_id += 1;
    }

    fn add_card(&mut self, column_index: usize, title: String, description: String) {
        if let Some(column) = self.columns.get_mut(column_index) {
            let card = Card {
                id: self.next_card_id,
                title,
                description,
                color: hsla(0.55, 0.3, 0.95, 1.0),
            };
            column.cards.push(card);
            self.next_card_id += 1;
        }
    }

    fn render_card(&self, card: &Card) -> impl IntoElement {
        div()
            .bg(card.color)
            .rounded_md()
            .p_3()
            .mb_2()
            .shadow_md()
            .cursor_pointer()
            .hover(|style| style.bg(hsla(0.55, 0.3, 0.90, 1.0)))
            .child(
                div()
                    .text_color(rgb(0x1a1a1a))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_size(px(14.0))
                    .mb_1()
                    .child(card.title.clone()),
            )
            .child(
                div()
                    .text_color(rgb(0x4a4a4a))
                    .text_size(px(12.0))
                    .child(card.description.clone()),
            )
    }

    fn render_column(&self, column: &Column) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w(px(280.0))
            .h_full()
            .bg(hsla(0.0, 0.0, 0.96, 1.0))
            .rounded_lg()
            .p_3()
            .mr_4()
            .shadow_sm()
            .child(
                // 列标题
                div()
                    .flex()
                    .items_center()
                    .mb_3()
                    .pb_2()
                    .border_b_1()
                    .border_color(hsla(0.0, 0.0, 0.85, 1.0))
                    .child(
                        div()
                            .w(px(4.0))
                            .h(px(20.0))
                            .bg(column.color)
                            .rounded_sm()
                            .mr_2(),
                    )
                    .child(
                        div()
                            .text_color(rgb(0x2a2a2a))
                            .font_weight(FontWeight::BOLD)
                            .text_size(px(16.0))
                            .child(column.title.clone()),
                    )
                    .child(
                        div()
                            .ml_auto()
                            .text_color(rgb(0x6a6a6a))
                            .text_size(px(12.0))
                            .child(format!("{}", column.cards.len())),
                    ),
            )
            .child(
                // 卡片列表
                div()
                    .id(("column-cards", column.id))
                    .flex()
                    .flex_col()
                    .flex_1()
                    .overflow_y_scroll()
                    .children(column.cards.iter().map(|card| self.render_card(card))),
            )
    }
}

impl Render for KanbanBoard {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(hsla(0.6, 0.1, 0.98, 1.0))
            .child(
                // 顶部标题栏
                div()
                    .flex()
                    .items_center()
                    .w_full()
                    .h(px(60.0))
                    .px_6()
                    .bg(white())
                    .shadow_sm()
                    .border_b_1()
                    .border_color(hsla(0.0, 0.0, 0.9, 1.0))
                    .child(
                        div()
                            .text_color(rgb(0x1a1a1a))
                            .font_weight(FontWeight::BOLD)
                            .text_size(px(24.0))
                            .child("VibeKanban"),
                    )
                    .child(
                        div()
                            .ml_auto()
                            .text_color(rgb(0x6a6a6a))
                            .text_size(px(14.0))
                            .child(format!("{} 个看板列", self.columns.len())),
                    ),
            )
            .child(
                // 看板主体区域
                div()
                    .id("kanban-columns")
                    .flex()
                    .flex_1()
                    .overflow_x_scroll()
                    .p_6()
                    .children(self.columns.iter().map(|column| self.render_column(column))),
            )
    }
}
