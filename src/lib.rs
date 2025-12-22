// 库入口，用于导出模块供测试和外部使用

pub mod kanban;

// 重新导出主要类型，方便外部使用
pub use kanban::{Card, Column, KanbanBoard};
