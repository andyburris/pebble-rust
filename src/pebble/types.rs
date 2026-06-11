/*
 * This file is part of pebble-rust.
 * Copyright (c) 2019 RoccoDev
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
 * General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

pub use crate::pebble::internal::types::{GColor, GRect, GPoint, GSize, GCornerMask, GTextAlignment, GTextOverflowMode, tm, TimeUnits, GCompOp, AppMessageResult,
                                         Tuple, TupleValue, MenuIndex, MenuLayerCallbacks, GBitmap,
                                         AnimationCurve, AnimationProgress, ANIMATION_NORMALIZED_MAX,
                                         GPathInfo};
use crate::pebble::internal::types::{GPathRaw, GContext};
use crate::pebble::internal::functions::{interface, declarations};

pub type VoidPtr = *const crate::pebble::internal::types::c_void;
pub type DictPtr = *mut crate::pebble::internal::types::DictionaryIterator;

pub struct Bitmap {
    pub internal: *mut GBitmap
}

impl Bitmap {
    pub fn new(resource_id: u32) -> Bitmap {
        let internal = interface::gbitmap_create_with_resource(resource_id);
        Bitmap {internal}
    }

    pub fn clean(self) {
        unsafe {
            declarations::gbitmap_destroy(self.internal);
        }
        drop(self);
    }
}

pub struct GPath(*mut GPathRaw);

impl GPath {
    pub fn new(info: &GPathInfo) -> Self {
        GPath(interface::gpath_create(info))
    }
    pub fn move_to(&mut self, point: GPoint) {
        interface::gpath_move_to(self.0, point);
    }
    pub fn rotate_to(&mut self, angle: i32) {
        interface::gpath_rotate_to(self.0, angle);
    }
    pub fn draw_filled(&self, ctx: &mut GContext) {
        interface::gpath_draw_filled(ctx, self.0);
    }
    pub fn draw_outline(&self, ctx: &mut GContext) {
        interface::gpath_draw_outline(ctx, self.0);
    }
    pub fn draw_outline_open(&self, ctx: &mut GContext) {
        interface::gpath_draw_outline_open(ctx, self.0);
    }
}

impl Drop for GPath {
    fn drop(&mut self) {
        interface::gpath_destroy(self.0);
    }
}