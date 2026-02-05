//! 解析意图预览卡片

use super::{confidence_color, format_confidence, test_type_display, test_type_icon};
use core::intent_parser::security::ParsedSecurityIntent;
use gpui::*;
use gpui_component::{h_flex, label::Label, v_flex};

/// 解析意图预览卡片
pub struct ParsedIntentPreview {
    result: ParsedSecurityIntent,
}

impl ParsedIntentPreview {
    /// 创建新的预览卡片
    pub fn new(result: ParsedSecurityIntent) -> Self {
        Self { result }
    }

    /// 获取结果
    pub fn result(&self) -> &ParsedSecurityIntent {
        &self.result
    }
}

impl RenderOnce for ParsedIntentPreview {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let intent = &self.result.security_intent;
        let test_type = intent.test_type;
        let icon = test_type_icon(test_type);
        let display_name = test_type_display(test_type);
        let confidence = self.result.confidence;
        let confidence_str = format_confidence(confidence);
        let conf_color = confidence_color(confidence.overall);

        v_flex()
            .gap(px(12.0))
            .p(px(16.0))
            .rounded_md()
            .bg(rgb(0xf9fafb))
            .border_1()
            .border_color(rgb(0xe5e7eb))
            // 头部：图标和类型
            .child(
                h_flex()
                    .gap(px(12.0))
                    .items_center()
                    .child(
                        div()
                            .size(px(40.0))
                            .rounded_md()
                            .bg(rgb(0xf3f4f6))
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(Label::new(icon).text_xl())
                    )
                    .child(
                        v_flex()
                            .gap(px(4.0))
                            .flex_1()
                            .child(
                                Label::new(display_name)
                                    .text_base()
                                    .font_weight(FontWeight::SEMIBOLD)
                            )
                            .child(
                                Label::new(test_type.description())
                                    .text_xs()
                                    .text_color(rgb(0x6b7280))
                            )
                    )
                    .child(
                        div()
                            .px(px(8.0))
                            .py(px(2.0))
                            .rounded_full()
                            .bg(rgb(conf_color))
                            .child(
                                Label::new(confidence_str)
                                    .text_xs()
                                    .text_color(rgb(0xffffff))
                            )
                    )
            )
            // 分隔线
            .child(div().w_full().h(px(1.0)).bg(rgb(0xe5e7eb)))
            // 目标信息
            .child(
                v_flex()
                    .gap(px(8.0))
                    .child(
                        Label::new("目标")
                            .text_xs()
                            .text_color(rgb(0x6b7280))
                    )
                    .children(intent.targets.iter().map(|target| {
                        h_flex()
                            .gap(px(8.0))
                            .items_center()
                            .child(
                                div()
                                    .size(px(6.0))
                                    .rounded_full()
                                    .bg(rgb(0x9ca3af))
                            )
                            .child(
                                Label::new(&target.address)
                                    .text_sm()
                                    .font_weight(FontWeight::MEDIUM)
                            )
                            .child(
                                Label::new(format!("({})", target.target_type.as_str()))
                                    .text_xs()
                                    .text_color(rgb(0x6b7280))
                            )
                            .into_any_element()
                    }))
            )
            // 参数信息（如果有）
            .children(intent.params.params.iter().take(5).map(|(k, v)| {
                h_flex()
                    .gap(px(8.0))
                    .items_center()
                    .child(
                        Label::new(format!("{}:", k))
                            .text_xs()
                            .text_color(rgb(0x6b7280))
                    )
                    .child(
                        Label::new(v.to_string())
                            .text_sm()
                    )
                    .into_any_element()
            }))
            // 扫描配置
            .child(
                h_flex()
                    .gap(px(16.0))
                    .mt(px(8.0))
                    .child(
                        h_flex()
                            .gap(px(4.0))
                            .items_center()
                            .child(
                                Label::new("强度:")
                                    .text_xs()
                                    .text_color(rgb(0x6b7280))
                            )
                            .child(
                                Label::new(match intent.scan_config.intensity {
                                    core::intent_parser::ScanIntensity::Light => "轻度",
                                    core::intent_parser::ScanIntensity::Normal => "正常",
                                    core::intent_parser::ScanIntensity::Aggressive => "激进",
                                    core::intent_parser::ScanIntensity::Custom => "定制",
                                })
                                    .text_xs()
                            )
                    )
                    .child(
                        h_flex()
                            .gap(px(4.0))
                            .items_center()
                            .child(
                                Label::new("线程:")
                                    .text_xs()
                                    .text_color(rgb(0x6b7280))
                            )
                            .child(
                                Label::new(intent.scan_config.threads.to_string())
                                    .text_xs()
                            )
                    )
                    .child(
                        h_flex()
                            .gap(px(4.0))
                            .items_center()
                            .child(
                                Label::new("超时:")
                                    .text_xs()
                                    .text_color(rgb(0x6b7280))
                            )
                            .child(
                                Label::new(format!("{}s", intent.scan_config.timeout_seconds))
                                    .text_xs()
                            )
                    )
            )
    }
}
