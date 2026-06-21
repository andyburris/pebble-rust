use core::ffi::CStr;

use crate::pebble::internal::types::{RawGFont, ResHandle};
use crate::pebble::internal::functions::declarations::*;

#[derive(Copy, Clone)]
pub struct FontKey {
    pub resource_id: &'static CStr,
    pub total_margin: u16,
    pub canonical_height: u16,
}

impl FontKey {
    pub const GOTHIC_09: FontKey = FontKey { resource_id: c"RESOURCE_ID_GOTHIC_09", total_margin: 0, canonical_height: 9 };
    pub const GOTHIC_14: FontKey = FontKey { resource_id: c"RESOURCE_ID_GOTHIC_14", total_margin: 5, canonical_height: 14 };
    pub const GOTHIC_14_BOLD: FontKey = FontKey { resource_id: c"RESOURCE_ID_GOTHIC_14_BOLD", total_margin: 5, canonical_height: 14 };
    pub const GOTHIC_18: FontKey = FontKey { resource_id: c"RESOURCE_ID_GOTHIC_18", total_margin: 7, canonical_height: 18 };
    pub const GOTHIC_18_BOLD: FontKey = FontKey { resource_id: c"RESOURCE_ID_GOTHIC_18_BOLD", total_margin: 7, canonical_height: 18 };
    pub const GOTHIC_24: FontKey = FontKey { resource_id: c"RESOURCE_ID_GOTHIC_24", total_margin: 0, canonical_height: 24 };
    pub const GOTHIC_24_BOLD: FontKey = FontKey { resource_id: c"RESOURCE_ID_GOTHIC_24_BOLD", total_margin: 0, canonical_height: 24 };
    pub const GOTHIC_28: FontKey = FontKey { resource_id: c"RESOURCE_ID_GOTHIC_28", total_margin: 0, canonical_height: 28 };
    pub const GOTHIC_28_BOLD: FontKey = FontKey { resource_id: c"RESOURCE_ID_GOTHIC_28_BOLD", total_margin: 0, canonical_height: 28 };
    pub const BITHAM_30_BLACK: FontKey = FontKey { resource_id: c"RESOURCE_ID_BITHAM_30_BLACK", total_margin: 0, canonical_height: 30 };
    pub const BITHAM_42_BOLD: FontKey = FontKey { resource_id: c"RESOURCE_ID_BITHAM_42_BOLD", total_margin: 0, canonical_height: 42 };
    pub const BITHAM_42_LIGHT: FontKey = FontKey { resource_id: c"RESOURCE_ID_BITHAM_42_LIGHT", total_margin: 0, canonical_height: 42 };
    pub const BITHAM_42_MEDIUM_NUMBERS: FontKey = FontKey { resource_id: c"RESOURCE_ID_BITHAM_42_MEDIUM_NUMBERS", total_margin: 0, canonical_height: 42 };
    pub const BITHAM_34_MEDIUM_NUMBERS: FontKey = FontKey { resource_id: c"RESOURCE_ID_BITHAM_34_MEDIUM_NUMBERS", total_margin: 0, canonical_height: 34 };
    pub const BITHAM_34_LIGHT_SUBSET: FontKey = FontKey { resource_id: c"RESOURCE_ID_BITHAM_34_LIGHT_SUBSET", total_margin: 0, canonical_height: 34 };
    pub const BITHAM_18_LIGHT_SUBSET: FontKey = FontKey { resource_id: c"RESOURCE_ID_BITHAM_18_LIGHT_SUBSET", total_margin: 0, canonical_height: 18 };
    pub const DROID_SERIF_28_BOLD: FontKey = FontKey { resource_id: c"RESOURCE_ID_DROID_SERIF_28_BOLD", total_margin: 0, canonical_height: 28 };
    pub const LECO_20_BOLD_NUMBERS: FontKey = FontKey { resource_id: c"RESOURCE_ID_LECO_20_BOLD_NUMBERS", total_margin: 0, canonical_height: 20 };
    pub const LECO_26_BOLD_NUMBERS_AM_PM: FontKey = FontKey { resource_id: c"RESOURCE_ID_LECO_26_BOLD_NUMBERS_AM_PM", total_margin: 0, canonical_height: 26 };
    pub const LECO_28_LIGHT_NUMBERS: FontKey = FontKey { resource_id: c"RESOURCE_ID_LECO_28_LIGHT_NUMBERS", total_margin: 0, canonical_height: 28 };
    pub const LECO_32_BOLD_NUMBERS: FontKey = FontKey { resource_id: c"RESOURCE_ID_LECO_32_BOLD_NUMBERS", total_margin: 0, canonical_height: 32 };
    pub const LECO_36_BOLD_NUMBERS: FontKey = FontKey { resource_id: c"RESOURCE_ID_LECO_36_BOLD_NUMBERS", total_margin: 0, canonical_height: 36 };
    pub const LECO_38_BOLD_NUMBERS: FontKey = FontKey { resource_id: c"RESOURCE_ID_LECO_38_BOLD_NUMBERS", total_margin: 0, canonical_height: 38 };
    pub const LECO_42_BOLD_NUMBERS: FontKey = FontKey { resource_id: c"RESOURCE_ID_LECO_42_NUMBERS", total_margin: 13, canonical_height: 42 };
}

pub struct GFont {
    pub internal: RawGFont,
    pub top_offset: u16,
    pub total_margin: u16,
}

impl GFont {
    pub fn get_system(key: FontKey) -> Self {
        unsafe {
            let internal = fonts_get_system_font(key.resource_id.as_ptr() as *const u8);
            Self { internal, top_offset: (key.total_margin / 2), total_margin: key.total_margin }
        }
    }

    pub fn get_custom_from_handle(res_handle: ResHandle) -> Self {
        unsafe {
            let internal = fonts_load_custom_font(res_handle);
            Self { internal, top_offset: 0, total_margin: 0 }
        }
    }

    pub fn get_custom(resource_id: u32) -> Self {
        unsafe {
            let res_handle = resource_get_handle(resource_id);
            Self::get_custom_from_handle(res_handle)
        }
    }
}
