//! Configuration layer for assets_ui
//!
//! This module contains all static configuration including:
//! - Zone metadata (names, colors, descriptions)
//! - UI labels and text constants (i18n ready)
//! - Theme extensions and color constants

pub mod theme_ext;
pub mod ui_labels;
pub mod zone_config;

pub use zone_config::ZoneTypeExt;
