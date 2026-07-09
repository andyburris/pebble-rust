//! `StatusBarLayer` — the thin bar showing time (and optionally battery/connection)
//! at the top of a window. A translation of the Pebble C `StatusBarLayer` API.

use crate::pebble::internal::{functions::interface, types};
use crate::pebble::internal::types::RawStatusBarLayer;
use crate::pebble::layer::AsLayer;
use crate::pebble::types::GColor;

pub use crate::pebble::internal::types::StatusBarLayerSeparatorMode;

/// Height of a status bar, in pixels (`STATUS_BAR_LAYER_HEIGHT`).
pub const STATUS_BAR_LAYER_HEIGHT: i16 = 16;

/// An owned status bar layer: created with `StatusBarLayer::new`, destroyed on drop.
pub struct StatusBarLayer {
    internal: *mut RawStatusBarLayer,
    inner:    *mut types::RawLayer,
}

impl StatusBarLayer {
    pub fn new() -> StatusBarLayer {
        let internal = interface::status_bar_layer_create();
        let inner = interface::status_bar_layer_get_layer(internal);
        StatusBarLayer { internal, inner }
    }

    pub fn set_colors(&self, background: GColor, foreground: GColor) {
        interface::status_bar_layer_set_colors(self.internal, background, foreground);
    }

    pub fn set_separator_mode(&self, mode: StatusBarLayerSeparatorMode) {
        interface::status_bar_layer_set_separator_mode(self.internal, mode);
    }

    pub fn get_foreground_color(&self) -> GColor {
        interface::status_bar_layer_get_foreground_color(self.internal)
    }

    pub fn get_background_color(&self) -> GColor {
        interface::status_bar_layer_get_background_color(self.internal)
    }
}

impl AsLayer for StatusBarLayer {
    fn as_raw(&self) -> *mut types::RawLayer {
        self.inner
    }
}

impl Drop for StatusBarLayer {
    fn drop(&mut self) {
        interface::status_bar_layer_destroy(self.internal);
    }
}
