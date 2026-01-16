//! 动画辅助函数
//!
//! 封装 squeeze-style 动画和缓动函数

use gpui::Pixels;
use std::time::Duration;

/// Squeeze 动画持续时间
pub const SQUEEZE_DURATION: Duration = Duration::from_millis(200);

/// 快速动画持续时间
pub const ANIMATION_FAST: Duration = Duration::from_millis(150);

/// 标准动画持续时间
pub const ANIMATION_NORMAL: Duration = Duration::from_millis(200);

/// 慢速动画持续时间
pub const ANIMATION_SLOW: Duration = Duration::from_millis(300);

/// Ease-out-expo 缓动控制点
/// cubic_bezier(0.32, 0.72, 0.0, 1.0)
pub const EASE_OUT_EXPO: (f32, f32, f32, f32) = (0.32, 0.72, 0.0, 1.0);

/// 计算 squeeze 动画的当前宽度
///
/// # Arguments
/// * `delta` - 动画进度 (0.0 - 1.0)
/// * `target_width` - 目标宽度
///
/// # Returns
/// 当前帧的宽度
#[inline]
pub fn squeeze_width(delta: f32, target_width: Pixels) -> Pixels {
    target_width * delta
}

/// 计算反向 squeeze 动画的当前宽度 (关闭面板)
///
/// # Arguments
/// * `delta` - 动画进度 (0.0 - 1.0)
/// * `start_width` - 起始宽度
///
/// # Returns
/// 当前帧的宽度
#[inline]
pub fn squeeze_width_reverse(delta: f32, start_width: Pixels) -> Pixels {
    start_width * (1.0 - delta)
}
