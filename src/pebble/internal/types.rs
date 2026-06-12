/*
 * Copyright (c) 2019, Andrew Foote. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
    * Redistributions of source code must retain the above copyright
      notice, this list of conditions and the following disclaimer.
    * Redistributions in binary form must reproduce the above copyright
      notice, this list of conditions and the following disclaimer in the
      documentation and/or other materials provided with the distribution.
    * Neither the name of the copyright holder nor the
      names of its contributors may be used to endorse or promote products
      derived from this software without specific prior written permission.

 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER BE LIABLE FOR ANY DIRECT,
 * INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING,
 * BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
 * LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE
 * OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF
 * ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/

#![allow(non_camel_case_types)]

use crate::pebble::internal::functions::{interface::{graphics_context_set_compositing_mode, graphics_context_set_fill_color, graphics_context_set_stroke_color, graphics_context_set_stroke_width, graphics_context_set_text_color, graphics_draw_bitmap_in_rect, graphics_draw_line, graphics_draw_text, graphics_fill_circle, graphics_fill_rect, graphics_text_layout_get_content_size}};

pub enum Window {}
pub enum Layer {}
pub enum TextLayer {}
pub enum ClickRecognizer {}
pub enum GBitmap {}
pub enum GContext {}

impl GContext {
    pub fn set_fill_color(&mut self, color: GColor) {
        graphics_context_set_fill_color(self, color);
    }
    pub fn set_stroke_color(&mut self, color: GColor) {
        graphics_context_set_stroke_color(self, color);
    }
    pub fn set_text_color(&mut self, color: GColor) {
        graphics_context_set_text_color(self, color);
    }
    pub fn set_stroke_width(&mut self, stroke_width: u8) {
        graphics_context_set_stroke_width(self, stroke_width);
    }
    pub fn fill_circle(&mut self, center: GPoint, radius: u16) {
        graphics_fill_circle(self, center, radius);
    }
    pub fn fill_rect(&mut self, rect: GRect, corner_radius: u16, corner_mask: GCornerMask) {
        graphics_fill_rect(self, rect, corner_radius, corner_mask);
    }

    pub fn set_compositing_mode(&mut self, mode: GCompOp) {
        graphics_context_set_compositing_mode(self, mode);
    }
    pub fn draw_bitmap_in_rect(&mut self, bitmap: *const GBitmap, dest_rect: GRect) {
        graphics_draw_bitmap_in_rect(self, bitmap, dest_rect);
    }
    pub fn draw_line(&mut self, p0: GPoint, p1: GPoint) {
        graphics_draw_line(self, p0, p1);
    }
    pub fn draw_text(&mut self, text: &core::ffi::CStr, font: GFont, rect: GRect, overflow: GTextOverflowMode, alignment: GTextAlignment) {
        graphics_draw_text(self, text, font, rect, overflow, alignment);
    }
    pub fn measure_text(&self, text: &core::ffi::CStr, font: GFont, max_size: GSize) -> GSize {
        graphics_text_layout_get_content_size(
            text, font,
            GRect { origin: GPoint::ORIGIN, size: max_size },
            GTextOverflowMode::TrailingEllipsis,
            GTextAlignment::Left,
        )
    }
}

#[repr(C)]
pub struct GPathInfo {
    pub num_points: u32,
    pub points: *const GPoint,
}

#[repr(C)]
pub struct GPathRaw {
    pub num_points: u32,
    pub points: *const GPoint,
    pub rotation: i32,
    pub offset: GPoint,
}

pub enum BitmapLayer {}
pub enum MenuLayer {}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tm {
    pub tm_sec: u32,
    pub tm_min: u32,
    pub tm_hour: u32,
    pub tm_mday: u32,
    pub tm_mon: u32,
    pub tm_year: u32,
    pub tm_wday: u32,
    pub tm_yday: u32,
    pub tm_isdst: u32
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GPoint {
    pub x: u16,
    pub y: u16,
}

impl GPoint {
    pub const ORIGIN: GPoint = GPoint { x: 0, y: 0 };
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GSize {
    pub w: u16,
    pub h: u16,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GRect {
    pub origin: GPoint,
    pub size: GSize,
}

#[repr(C)]
pub struct WindowHandlers {
    pub load: extern "C" fn(*mut Window),
    pub appear: extern "C" fn(*mut Window),
    pub disappear: extern "C" fn(*mut Window),
    pub unload: extern "C" fn(*mut Window),
}

#[repr(C)]
pub enum GCompOp {
    GCompOpAssign,
    GCompOpAssignInverted,
    GCompOpOr,
    GCompOpAnd,
    GCompOpClear,
    GCompOpSet
}

#[repr(C)]
pub enum GCornerMask {
    GCornerNone = 0b0000,
    GCornerTopLeft = 0b0001,
    GCornerTopRight = 0b0010,
    GCornerBottomLeft = 0b0100,
    GCornerBottomRight = 0b1000,
    GCornersAll = 0b1111,
    GCornersTop = 0b0011,
    GCornersBottom = 0b1100,
    GCornersLeft = 0b0101,
    GCornersRight = 0b1010,
}

#[repr(C)]
pub enum GTextAlignment {
    Left = 0,
    Center = 1,
    Right = 2,
}

#[repr(C)]
pub enum GTextOverflowMode {
    WordWrap = 0,
    TrailingEllipsis = 1,
    Fill = 2,
}

/// SDK 3.x GColor — 1-byte packed ARGB (aa rr gg bb, 2 bits each).
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
#[allow(non_upper_case_globals)]
pub struct GColor(pub u8);

#[allow(non_upper_case_globals)]
impl GColor {
    pub const Clear: GColor = GColor(0x00);
    pub const Black: GColor = GColor(0xC0);
    pub const OxfordBlue: GColor = GColor(0xC1);
    pub const DukeBlue: GColor = GColor(0xC2);
    pub const Blue: GColor = GColor(0xC3);
    pub const DarkGreen: GColor = GColor(0xC4);
    pub const MidnightGreen: GColor = GColor(0xC5);
    pub const CobaltBlue: GColor = GColor(0xC6);
    pub const BlueMoon: GColor = GColor(0xC7);
    pub const IslamicGreen: GColor = GColor(0xC8);
    pub const JaegerGreen: GColor = GColor(0xC9);
    pub const TiffanyBlue: GColor = GColor(0xCA);
    pub const VividCerulean: GColor = GColor(0xCB);
    pub const Green: GColor = GColor(0xCC);
    pub const Malachite: GColor = GColor(0xCD);
    pub const MediumSpringGreen: GColor = GColor(0xCE);
    pub const Cyan: GColor = GColor(0xCF);
    pub const BulgarianRose: GColor = GColor(0xD0);
    pub const ImperialPurple: GColor = GColor(0xD1);
    pub const Indigo: GColor = GColor(0xD2);
    pub const ElectricUltramarine: GColor = GColor(0xD3);
    pub const ArmyGreen: GColor = GColor(0xD4);
    pub const DarkGray: GColor = GColor(0xD5);
    pub const Liberty: GColor = GColor(0xD6);
    pub const VeryLightBlue: GColor = GColor(0xD7);
    pub const KellyGreen: GColor = GColor(0xD8);
    pub const MayGreen: GColor = GColor(0xD9);
    pub const CadetBlue: GColor = GColor(0xDA);
    pub const PictonBlue: GColor = GColor(0xDB);
    pub const BrightGreen: GColor = GColor(0xDC);
    pub const ScreaminGreen: GColor = GColor(0xDD);
    pub const MediumAquamarine: GColor = GColor(0xDE);
    pub const ElectricBlue: GColor = GColor(0xDF);
    pub const DarkCandyAppleRed: GColor = GColor(0xE0);
    pub const JazzberryJam: GColor = GColor(0xE1);
    pub const Purple: GColor = GColor(0xE2);
    pub const VividViolet: GColor = GColor(0xE3);
    pub const WindsorTan: GColor = GColor(0xE4);
    pub const RoseVale: GColor = GColor(0xE5);
    pub const Purpureus: GColor = GColor(0xE6);
    pub const LavenderIndigo: GColor = GColor(0xE7);
    pub const Limerick: GColor = GColor(0xE8);
    pub const Brass: GColor = GColor(0xE9);
    pub const LightGray: GColor = GColor(0xEA);
    pub const BabyBlueEyes: GColor = GColor(0xEB);
    pub const SpringBud: GColor = GColor(0xEC);
    pub const Inchworm: GColor = GColor(0xED);
    pub const MintGreen: GColor = GColor(0xEE);
    pub const Celeste: GColor = GColor(0xEF);
    pub const Red: GColor = GColor(0xF0);
    pub const Folly: GColor = GColor(0xF1);
    pub const FashionMagenta: GColor = GColor(0xF2);
    pub const Magenta: GColor = GColor(0xF3);
    pub const Orange: GColor = GColor(0xF4);
    pub const SunsetOrange: GColor = GColor(0xF5);
    pub const BrilliantRose: GColor = GColor(0xF6);
    pub const ShockingPink: GColor = GColor(0xF7);
    pub const ChromeYellow: GColor = GColor(0xF8);
    pub const Rajah: GColor = GColor(0xF9);
    pub const Melon: GColor = GColor(0xFA);
    pub const RichBrilliantLavender: GColor = GColor(0xFB);
    pub const Yellow: GColor = GColor(0xFC);
    pub const Icterine: GColor = GColor(0xFD);
    pub const PastelYellow: GColor = GColor(0xFE);
    pub const White: GColor = GColor(0xFF);
}

#[repr(C)]
pub enum TimeUnits {
    SECOND_UNIT=1,
    MINUTE_UNIT,
    HOUR_UNIT,
    DAY_UNIT,
    MONTH_UNIT,
    YEAR_UNIT
}

pub type ResHandle = c_void;

#[repr(C)]
pub struct FontInfo;

pub type GFont = *mut FontInfo;

#[allow(non_camel_case_types)]
#[repr(u8)]
pub enum c_void {
    __variant1,
    __variant2,
}

#[allow(non_camel_case_types)]
pub type c_char = u8;

#[repr(C, align(1))]
#[derive(Copy, Clone, BitfieldStruct)]
pub struct Tuple {
    pub key: u32,
    #[bitfield(name = "t_type", ty = "u8", bits = "32..=39")]
    #[bitfield(name = "length", ty = "u16", bits = "40..=55")]
    pub t_type: [u8; 2],
    value: TupleValue
}

impl Tuple {
    unsafe fn read(&self) -> Option<TupleValue> {
        let ptr = (&self.key as *const u32) as usize;
        let value_ptr = ptr + 7;
        let t = self.t_type[0];
        unsafe {
            match t {
                0 => {
                    Some(TupleValue {
                        data: core::slice::from_raw_parts(value_ptr as *const u8, self.t_type[1] as usize)
                    })
                },
                1 => {
                    Some(TupleValue {
                        cstring: core::slice::from_raw_parts(value_ptr as *const u8, self.t_type[1] as usize)
                    })
                },
                2 => {
                    let value_ptr = value_ptr as *const u32;
                    Some(TupleValue {
                        uint32: *value_ptr
                    })
                },
                3 => {
                    let value_ptr = value_ptr as *const i32;
                    Some(TupleValue {
                        int32: *value_ptr
                    })
                },
                _ => {None}
            }
        }
    }

    pub fn get_string(&self) -> Option<&'static str> {
        unsafe {
            let opt = self.get_value();
            if let Some(opt) = opt {
                let cstr= opt.cstring;
                let str = core::str::from_utf8_unchecked(cstr);
                Some(str)
            } else {
                None
            }
        }
    }

    pub fn get_value(&self) -> Option<TupleValue> {
        unsafe {self.read()}
    }

}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub union TupleValue {
    data: &'static [u8],
    cstring: &'static [u8],
    pub uint32: u32,
    pub int32: i32,

    // Unions are as large as the largest item.
    // No space is wasted though.
    placeholder: [u8; u8::max_value() as usize + 325usize]
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum TupleType {
    BYTE_ARRAY, CSTRING, UINT, INT
}

#[repr(C)]
pub struct Dictionary;

#[repr(C)]
pub struct DictionaryIterator {
    pub dict: *mut Dictionary,
    pub end: *const c_void,
    pub cursor: *mut Tuple
}

#[repr(u8)]
pub enum DictionaryResult {
    DICT_OK, DICT_NOT_ENOUGH_STORAGE, DICT_INVALID_ARGS, DICT_INTERNAL_INCONSISTENCY,
    DICT_MALLOC_FAILED
}

#[repr(u8)]
pub enum AppMessageResult {
    OK, SEND_TIMEOUT, SEND_REJECTED, NOT_CONNECTED, NOT_RUNNING, INVALID_ARGS, BUSY, BUFFER_OVERFLOW,
    ALREADY_RELEASED, CALLBACK_ALREADY_REGISTERED, CALLBACK_NOT_REGISTERED, OUT_OF_MEMORY, CLOSED,
    INTERNAL_ERROR, INVALID_STATE
}

#[repr(C)]
pub struct BatteryChargeState {
    pub charge_percent: u8,
    pub is_charging: bool,
    pub is_plugged: bool
}

#[repr(C)]
pub struct ConnectionHandlers {
    pub app: extern "C" fn(bool),
    pub pebblekit: extern "C" fn(bool)
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MenuIndex {
    pub section: u16,
    pub row: u16,
}

impl MenuIndex {
    pub fn row_idx(&self) -> usize { self.row as usize }
    pub fn section_idx(&self) -> usize { self.section as usize }
}

// MenuLayer* is *mut u8 (callers never call methods on it).
// GContext* and Layer* are typed so callers can pass them directly to the draw helpers.
// The void* callback context uses *mut () (Rust-idiomatic void pointer).
#[repr(C)]
pub struct MenuLayerCallbacks {
    pub get_num_sections:      Option<extern "C" fn(*mut u8, *mut ()) -> u16>,
    pub get_num_rows:          Option<extern "C" fn(*mut u8, u16, *mut ()) -> u16>,
    pub get_cell_height:       Option<extern "C" fn(*mut u8, *const MenuIndex, *mut ()) -> i16>,
    pub get_header_height:     Option<extern "C" fn(*mut u8, u16, *mut ()) -> i16>,
    pub draw_row:              Option<extern "C" fn(*mut GContext, *const Layer, *const MenuIndex, *mut ())>,
    pub draw_header:           Option<extern "C" fn(*mut GContext, *const Layer, u16, *mut ())>,
    pub select_click:          Option<extern "C" fn(*mut u8, *const MenuIndex, *mut ())>,
    pub select_long_click:     Option<extern "C" fn(*mut u8, *const MenuIndex, *mut ())>,
    pub selection_changed:     Option<extern "C" fn(*mut u8, MenuIndex, MenuIndex, *mut ())>,
    pub get_separator_height:  Option<extern "C" fn(*mut u8, *const MenuIndex, *mut ()) -> i16>,
    pub draw_separator:        Option<extern "C" fn(*mut GContext, *const Layer, *const MenuIndex, *mut ())>,
    pub selection_will_change: Option<extern "C" fn(*mut u8, *mut MenuIndex, MenuIndex, *mut ())>,
    pub draw_background:       Option<extern "C" fn(*mut GContext, *const Layer, bool, *mut ())>,
}

pub enum Animation {}

pub type AnimationProgress = u32;
pub const ANIMATION_NORMALIZED_MAX: AnimationProgress = 65535;

#[repr(C)]
pub enum AnimationCurve {
    EaseIn    = 0,
    EaseOut   = 1,
    EaseInOut = 2,
    Linear    = 3,
}

#[repr(C)]
pub struct AnimationImplementation {
    pub setup:    Option<extern "C" fn(*mut Animation)>,
    pub update:   Option<extern "C" fn(*mut Animation, AnimationProgress)>,
    pub teardown: Option<extern "C" fn(*mut Animation)>,
}

#[repr(C)]
pub struct AnimationHandlers {
    pub started: Option<extern "C" fn(*mut Animation, *mut u8)>,
    pub stopped: Option<extern "C" fn(*mut Animation, bool, *mut u8)>,
}
