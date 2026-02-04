pub mod image_card;
pub mod images_panel;
pub mod sandbox_manager;

pub use image_card::ImageCard;
pub use images_panel::ImagesPanel;
pub use sandbox_manager::{SandboxManager, SandboxEvent};

use gpui::*;
use gpui::prelude::FluentBuilder;
use gpui_component::{h_flex, label::Label, button::{Button, ButtonVariants}, Sizable};
use data::ContainerStatus;

/// Header component for Images panel
pub fn render_images_header(
    running_count: usize,
    on_create_click: impl Fn(&mut Window, &mut App) + 'static,
) -> impl IntoElement {
    h_flex()
        .w_full()
        .items_center()
        .justify_between()
        .px(px(24.0))
        .py(px(16.0))
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
                .on_click(move |_, window, cx| on_create_click(window, cx))
        )
}

/// Format bytes to human readable string
pub fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    format!("{:.1} {}", size, UNITS[unit_index])
}

/// Get color for CPU usage
pub fn cpu_color(percentage: f64) -> Rgba {
    match percentage {
        p if p < 50.0 => rgb(0x10b981), // Green
        p if p < 80.0 => rgb(0xf59e0b), // Yellow/Orange
        _ => rgb(0xef4444),             // Red
    }
}

/// Get color for memory usage
pub fn memory_color(percentage: f64) -> Rgba {
    match percentage {
        p if p < 60.0 => rgb(0x3b82f6), // Blue
        p if p < 85.0 => rgb(0xf59e0b), // Yellow/Orange
        _ => rgb(0xef4444),             // Red
    }
}
