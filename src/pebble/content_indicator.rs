//! `ContentIndicator` — arrows that hint more content lies above/below the visible
//! area. A translation of the Pebble C `ContentIndicator` API.
//!
//! A ContentIndicator is **not itself a layer**: a single instance drives both
//! directions, and each direction paints its arrow into a *host layer* you provide.

use crate::pebble::internal::functions::interface;
use crate::pebble::internal::types::{RawContentIndicator, ContentIndicatorConfig, ContentIndicatorConfigColors};
use crate::pebble::layer::AsLayer;
use crate::pebble::types::{GAlign, GColor};

pub use crate::pebble::internal::types::ContentIndicatorDirection;

/// An owned content indicator: created with `ContentIndicator::new`, destroyed on drop.
/// One instance serves both directions.
pub struct ContentIndicator {
    internal: *mut RawContentIndicator,
}

impl ContentIndicator {
    pub fn new() -> ContentIndicator {
        ContentIndicator { internal: interface::content_indicator_create() }
    }

    /// Configure one direction to paint its arrow into `host`. `times_out` hides the
    /// arrow automatically after a few seconds (as on the system status bar).
    pub fn configure_direction(
        &self,
        direction: ContentIndicatorDirection,
        host: &impl AsLayer,
        times_out: bool,
        alignment: GAlign,
        foreground: GColor,
        background: GColor,
    ) -> bool {
        let config = ContentIndicatorConfig {
            layer: host.as_raw(),
            times_out,
            alignment,
            colors: ContentIndicatorConfigColors { foreground, background },
        };
        interface::content_indicator_configure_direction(self.internal, direction, &config)
    }

    /// Show or hide the arrow for `direction`.
    pub fn set_content_available(&self, direction: ContentIndicatorDirection, available: bool) -> bool {
        interface::content_indicator_set_content_available(self.internal, direction, available)
    }

    pub fn get_content_available(&self, direction: ContentIndicatorDirection) -> bool {
        interface::content_indicator_get_content_available(self.internal, direction)
    }
}

impl Drop for ContentIndicator {
    fn drop(&mut self) {
        interface::content_indicator_destroy(self.internal);
    }
}
