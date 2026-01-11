// 流量表格组件
// T1-9: Traffic 流量分析视图 - 流量表格组件

use gpui::*;
use gpui_component::{
    h_flex,
    label::Label,
    v_flex,
};
use data::models::TrafficEntry;
use ui::theme::*;

/// 表格列定义
#[derive(Debug, Clone, Copy)]
enum TableColumn {
    Index,
    Time,
    Asset,
    Proto,
    Method,
    Path,
    Status,
    Size,
    Duration,
}

impl TableColumn {
    fn header(&self) -> &'static str {
        match self {
            TableColumn::Index => "#",
            TableColumn::Time => "Time",
            TableColumn::Asset => "Asset",
            TableColumn::Proto => "Proto",
            TableColumn::Method => "Method",
            TableColumn::Path => "Path",
            TableColumn::Status => "Status",
            TableColumn::Size => "Size",
            TableColumn::Duration => "Duration",
        }
    }

    fn width(&self) -> Pixels {
        match self {
            TableColumn::Index => px(50.0),
            TableColumn::Time => px(120.0),
            TableColumn::Asset => px(150.0),
            TableColumn::Proto => px(80.0),
            TableColumn::Method => px(80.0),
            TableColumn::Path => px(300.0),
            TableColumn::Status => px(80.0),
            TableColumn::Size => px(100.0),
            TableColumn::Duration => px(100.0),
        }
    }
}

/// 渲染表格表头
fn render_table_header() -> impl IntoElement {
    let columns = [
        TableColumn::Index,
        TableColumn::Time,
        TableColumn::Asset,
        TableColumn::Proto,
        TableColumn::Method,
        TableColumn::Path,
        TableColumn::Status,
        TableColumn::Size,
        TableColumn::Duration,
    ];

    h_flex()
        .w_full()
        .h(px(36.0))
        .px(PADDING_MD)
        .gap(SPACING_SM)
        .items_center()
        .bg(rgb(BG_SECONDARY))
        .border_b(px(1.0))
        .border_color(rgb(BORDER_COLOR))
        .children(columns.iter().map(|col| {
            h_flex()
                .w(col.width())
                .items_center()
                .child(
                    Label::new(col.header())
                        .text_sm()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(TEXT_SECONDARY)),
                )
        }).collect::<Vec<_>>())
}

/// 格式化时间戳
fn format_time(time_str: &str) -> String {
    // 简化处理，实际应该解析时间戳并格式化
    if time_str.len() > 19 {
        time_str[..19].to_string()
    } else {
        time_str.to_string()
    }
}

/// 格式化文件大小
fn format_size(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    }
}

/// 格式化持续时间
fn format_duration(ms: u64) -> String {
    if ms < 1000 {
        format!("{} ms", ms)
    } else {
        format!("{:.2} s", ms as f64 / 1000.0)
    }
}

/// 获取状态码颜色
fn get_status_color(status: u16) -> u32 {
    match status {
        200..=299 => STATUS_SUCCESS,
        300..=399 => STATUS_WARNING,
        400..=499 => STATUS_ERROR,
        500..=599 => SEVERITY_CRITICAL,
        _ => TEXT_SECONDARY,
    }
}

/// 渲染表格行
fn render_table_row<T: 'static>(
    entry: &TrafficEntry,
    index: usize,
    is_selected: bool,
    on_select: impl Fn(&mut T, &mut Context<T>, i64) + 'static,
    cx: &mut Context<T>,
) -> impl IntoElement {
    let entry_id = entry.id;
    let has_anomalies = !entry.anomalies.is_empty();
    let is_live = entry.is_live;

    let row_bg = if is_selected {
        rgb(0xf3e8ff) // 选中时紫色背景
    } else if has_anomalies {
        rgb(0xfef2f2) // 有异常时浅红色背景
    } else {
        rgb(BG_CARD)
    };

    let method_text = entry.method.as_ref()
        .map(|m| m.to_string())
        .unwrap_or_else(|| "-".to_string());
    
    let status_text = if entry.status > 0 {
        entry.status.to_string()
    } else {
        "-".to_string()
    };

    let status_color = if entry.status > 0 {
        get_status_color(entry.status)
    } else {
        TEXT_SECONDARY
    };

    h_flex()
        .id(("traffic-row", entry_id))
        .w_full()
        .h(px(32.0))
        .px(PADDING_MD)
        .gap(SPACING_SM)
        .items_center()
        .bg(row_bg)
        .border_b(px(1.0))
        .border_color(rgb(BORDER_COLOR))
        .when(has_anomalies, |this| {
            this.border_l(px(3.0)).border_color(rgb(SEVERITY_CRITICAL))
        })
        .cursor_pointer()
        .on_mouse_down(
            MouseButton::Left,
            cx.listener(move |this: &mut T, _, _, cx: &mut Context<T>| {
                on_select(this, cx, entry_id);
            }),
        )
        .children(vec![
            // # 列
            h_flex()
                .w(TableColumn::Index.width())
                .items_center()
                .child(
                    Label::new(index.to_string())
                        .text_sm()
                        .text_color(rgb(TEXT_SECONDARY)),
                ),
            // Time 列
            h_flex()
                .w(TableColumn::Time.width())
                .items_center()
                .child(
                    Label::new(format_time(&entry.created_at))
                        .text_sm()
                        .font_family("monospace")
                        .text_color(rgb(TEXT_PRIMARY)),
                ),
            // Asset 列
            h_flex()
                .w(TableColumn::Asset.width())
                .items_center()
                .child(
                    Label::new(&entry.asset_name)
                        .text_sm()
                        .text_color(rgb(TEXT_PRIMARY)),
                ),
            // Proto 列
            h_flex()
                .w(TableColumn::Proto.width())
                .items_center()
                .child(
                    Label::new(entry.protocol.to_string())
                        .text_sm()
                        .font_family("monospace")
                        .text_color(rgb(TEXT_PRIMARY)),
                ),
            // Method 列
            h_flex()
                .w(TableColumn::Method.width())
                .items_center()
                .child(
                    Label::new(method_text)
                        .text_sm()
                        .font_family("monospace")
                        .text_color(rgb(TEXT_PRIMARY)),
                ),
            // Path 列
            h_flex()
                .w(TableColumn::Path.width())
                .items_center()
                .child(
                    Label::new(&entry.path)
                        .text_sm()
                        .font_family("monospace")
                        .text_color(rgb(TEXT_PRIMARY)),
                ),
            // Status 列
            h_flex()
                .w(TableColumn::Status.width())
                .items_center()
                .child(
                    Label::new(status_text)
                        .text_sm()
                        .font_family("monospace")
                        .text_color(rgb(status_color)),
                ),
            // Size 列
            h_flex()
                .w(TableColumn::Size.width())
                .items_center()
                .child(
                    Label::new(format_size(entry.response_size))
                        .text_sm()
                        .font_family("monospace")
                        .text_color(rgb(TEXT_PRIMARY)),
                ),
            // Duration 列
            h_flex()
                .w(TableColumn::Duration.width())
                .items_center()
                .gap(px(4.0))
                .child(
                    Label::new(format_duration(entry.duration_ms))
                        .text_sm()
                        .font_family("monospace")
                        .text_color(rgb(TEXT_PRIMARY)),
                )
                .when(is_live, |this| {
                    this.child(
                        h_flex()
                            .w(px(8.0))
                            .h(px(8.0))
                            .rounded_full()
                            .bg(rgb(STATUS_SUCCESS)),
                    )
                }),
        ])
}

/// 流量表格组件
pub struct TrafficTable {
    entries: Vec<TrafficEntry>,
    selected_id: Option<i64>,
}

impl TrafficTable {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            selected_id: None,
        }
    }

    pub fn set_entries(&mut self, entries: Vec<TrafficEntry>, cx: &mut App) {
        self.entries = entries;
        cx.notify();
    }

    pub fn select_entry(&mut self, id: i64, cx: &mut App) {
        self.selected_id = Some(id);
        cx.notify();
    }

    pub fn get_selected_entry(&self) -> Option<&TrafficEntry> {
        self.selected_id.and_then(|id| {
            self.entries.iter().find(|e| e.id == id)
        })
    }
}

impl Render for TrafficTable {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .w_full()
            .flex_1()
            .bg(rgb(BG_CARD))
            .child(render_table_header())
            .child(
                v_flex()
                    .flex_1()
                    .overflow_y_scroll()
                    .children(
                        self.entries
                            .iter()
                            .enumerate()
                            .map(|(index, entry)| {
                                let is_selected = self.selected_id == Some(entry.id);
                                render_table_row(
                                    entry,
                                    index + 1,
                                    is_selected,
                                    |this, cx, id| {
                                        this.select_entry(id, cx);
                                    },
                                    cx,
                                )
                            })
                            .collect::<Vec<_>>(),
                    ),
            )
    }
}
