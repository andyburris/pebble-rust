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

use alloc::boxed::Box;
use core::ffi::CStr;
use crate::pebble::internal::{functions::interface, types};
use crate::pebble::internal::types::{MenuLayer as RawMenuLayer, MenuLayerCallbacks, MenuIndex};
use crate::pebble::types::{Bitmap, GCompOp, GRect};
use crate::system::fonts::Font;
use crate::pebble::internal::functions::declarations::text_layer_set_text;
use crate::pebble::window::Window;

pub struct Layer {
    internal: *mut types::Layer
}

pub struct TextLayer {
    internal: *mut types::TextLayer,
    inner: *mut types::Layer
}

pub struct BitmapLayer {
    internal: *mut types::BitmapLayer,
    inner: *mut types::Layer
}

pub trait ILayer {
    fn get_bounds(&self) -> GRect;
    fn get_frame(&self) -> GRect;
    fn add_child(&self, layer: &dyn ILayer);
    fn mark_dirty(&self);
    fn set_hidden(&self, hidden: bool);
    fn get_internal(&self) -> *mut types::Layer;
}

impl ILayer for Layer {
    fn get_bounds(&self) -> GRect {
        interface::layer_get_bounds(self.internal)
    }

    fn get_frame(&self) -> GRect {
        interface::layer_get_frame(self.internal)
    }

    fn add_child(&self, layer: &dyn ILayer) {
        interface::layer_add_child(self.internal, layer.get_internal())
    }

    fn mark_dirty(&self) {
        interface::layer_mark_dirty(self.internal);
    }

    fn set_hidden(&self, hidden: bool) {
        interface::layer_set_hidden(self.internal, hidden);
    }

    fn get_internal(&self) -> *mut types::Layer {
        self.internal
    }
}

impl Layer {
    pub fn new(bounds: GRect) -> Layer {
        Layer {
            internal: interface::layer_create(bounds)
        }
    }

    pub fn from_raw(ptr: *mut types::Layer) -> Layer {
        Layer {
            internal: ptr
        }
    }
}

impl ILayer for TextLayer {
    fn get_bounds(&self) -> GRect {
        interface::layer_get_bounds(self.inner)
    }

    fn get_frame(&self) -> GRect {
        interface::layer_get_frame(self.inner)
    }

    fn add_child(&self, layer: &dyn ILayer) {
        interface::layer_add_child(self.inner, layer.get_internal());
    }

    fn mark_dirty(&self) {
        interface::layer_mark_dirty(self.inner);
    }

    fn set_hidden(&self, hidden: bool) {
        interface::layer_set_hidden(self.inner, hidden);
    }

    fn get_internal(&self) -> *mut types::Layer {
        self.inner
    }
}

impl TextLayer {
    pub fn new(bounds: GRect) -> TextLayer {
        let internal = interface::text_layer_create(bounds);
        let inner = interface::text_layer_get_layer(internal);

        TextLayer {
            internal, inner
        }
    }

    pub fn set_text(&self, text: &CStr) {
        interface::text_layer_set_text(self.internal, text);
    }

    pub fn set_font(&self, font: Font) {
        interface::text_layer_set_font(self.internal, font.internal)
    }
}

impl ILayer for BitmapLayer {
    fn get_bounds(&self) -> GRect {
        interface::layer_get_bounds(self.inner)
    }

    fn get_frame(&self) -> GRect {
        interface::layer_get_frame(self.inner)
    }

    fn add_child(&self, layer: &dyn ILayer) {
        interface::layer_add_child(self.inner, layer.get_internal())
    }

    fn mark_dirty(&self) {
        interface::layer_mark_dirty(self.inner)
    }

    fn set_hidden(&self, hidden: bool) {
        interface::layer_set_hidden(self.inner, hidden)
    }

    fn get_internal(&self) -> *mut types::Layer {
        self.inner
    }
}

impl BitmapLayer {
    pub fn new(bounds: GRect) -> BitmapLayer {
        let internal = interface::bitmap_layer_create(bounds);
        let inner = interface::bitmap_layer_get_layer(internal);

        BitmapLayer {
            internal, inner
        }
    }

    pub fn set_bitmap(&self, bitmap: &Bitmap) {
        interface::bitmap_layer_set_bitmap(self.internal, bitmap.internal);
    }

    pub fn set_compositing_mode(&self, mode: GCompOp) {
        interface::bitmap_layer_set_compositing_mode(self.internal, mode);
    }
}

pub struct MenuLayer<T> {
    internal: *mut RawMenuLayer,
    inner:    *mut types::Layer,
    _ctx:     Box<MenuLayerContext<T>>,
}

impl<T> ILayer for MenuLayer<T> {
    fn get_bounds(&self) -> GRect { interface::layer_get_bounds(self.inner) }
    fn get_frame(&self) -> GRect { interface::layer_get_frame(self.inner) }
    fn add_child(&self, layer: &dyn ILayer) { interface::layer_add_child(self.inner, layer.get_internal()) }
    fn mark_dirty(&self) { interface::layer_mark_dirty(self.inner) }
    fn set_hidden(&self, hidden: bool) { interface::layer_set_hidden(self.inner, hidden) }
    fn get_internal(&self) -> *mut types::Layer { self.inner }
}

impl<T> MenuLayer<T> {
    pub fn new(frame: GRect, context: *mut T, callbacks: TypedMenuCallbacks<T>) -> Self {
        let internal = interface::menu_layer_create(frame);
        let mut ctx = Box::new(MenuLayerContext { context, callbacks });
        let ctx_ptr: *mut MenuLayerContext<T> = &mut *ctx;
        let raw_cbs = MenuLayerCallbacks {
            get_num_sections:      ctx.callbacks.get_num_sections      .map(|_| trampoline_num_sections::<T>      as extern "C" fn(*mut u8, *mut ()) -> u16),
            get_num_rows:          ctx.callbacks.get_num_rows          .map(|_| trampoline_num_rows::<T>          as extern "C" fn(*mut u8, u16, *mut ()) -> u16),
            get_cell_height:       ctx.callbacks.get_cell_height       .map(|_| trampoline_cell_height::<T>       as extern "C" fn(*mut u8, *const MenuIndex, *mut ()) -> i16),
            get_header_height:     ctx.callbacks.get_header_height     .map(|_| trampoline_header_height::<T>     as extern "C" fn(*mut u8, u16, *mut ()) -> i16),
            draw_row:              ctx.callbacks.draw_row              .map(|_| trampoline_draw_row::<T>          as extern "C" fn(*mut types::GContext, *const types::Layer, *const MenuIndex, *mut ())),
            draw_header:           ctx.callbacks.draw_header           .map(|_| trampoline_draw_header::<T>       as extern "C" fn(*mut types::GContext, *const types::Layer, u16, *mut ())),
            select_click:          ctx.callbacks.select_click          .map(|_| trampoline_select_click::<T>      as extern "C" fn(*mut u8, *const MenuIndex, *mut ())),
            select_long_click:     ctx.callbacks.select_long_click     .map(|_| trampoline_select_long_click::<T> as extern "C" fn(*mut u8, *const MenuIndex, *mut ())),
            selection_changed:     ctx.callbacks.selection_changed     .map(|_| trampoline_selection_changed::<T> as extern "C" fn(*mut u8, MenuIndex, MenuIndex, *mut ())),
            get_separator_height:  ctx.callbacks.get_separator_height  .map(|_| trampoline_separator_height::<T>  as extern "C" fn(*mut u8, *const MenuIndex, *mut ()) -> i16),
            draw_separator:        ctx.callbacks.draw_separator        .map(|_| trampoline_draw_separator::<T>    as extern "C" fn(*mut types::GContext, *const types::Layer, *const MenuIndex, *mut ())),
            selection_will_change: ctx.callbacks.selection_will_change .map(|_| trampoline_selection_will_change::<T> as extern "C" fn(*mut u8, *mut MenuIndex, MenuIndex, *mut ())),
            draw_background:       ctx.callbacks.draw_background       .map(|_| trampoline_draw_background::<T>   as extern "C" fn(*mut types::GContext, *const types::Layer, bool, *mut ())),
        };
        interface::menu_layer_set_callbacks(internal, ctx_ptr, raw_cbs);
        let inner = interface::menu_layer_get_layer(internal);
        MenuLayer { internal, inner, _ctx: ctx }
    }

    pub fn set_click_config_onto_window(&self, window: &Window) {
        interface::menu_layer_set_click_config_onto_window(self.internal, window.raw());
    }

    pub fn reload_data(&self) {
        interface::menu_layer_reload_data(self.internal);
    }
}

pub const MENU_CELL_BASIC_HEADER_HEIGHT: i16 = 16;

pub struct TypedMenuCallbacks<T> {
    pub get_num_sections:      Option<fn(&T) -> u16>,
    pub get_num_rows:          Option<fn(&T, u16) -> u16>,
    pub get_cell_height:       Option<fn(&T, &MenuIndex) -> i16>,
    pub get_header_height:     Option<fn(&T, u16) -> i16>,
    pub draw_row:              Option<fn(*mut types::GContext, *const types::Layer, &MenuIndex, &T)>,
    pub draw_header:           Option<fn(*mut types::GContext, *const types::Layer, u16, &T)>,
    pub select_click:          Option<fn(&T, &MenuIndex)>,
    pub select_long_click:     Option<fn(&T, &MenuIndex)>,
    pub selection_changed:     Option<fn(&T, MenuIndex, MenuIndex)>,
    pub get_separator_height:  Option<fn(&T, &MenuIndex) -> i16>,
    pub draw_separator:        Option<fn(*mut types::GContext, *const types::Layer, &MenuIndex, &T)>,
    pub selection_will_change: Option<fn(&T, &mut MenuIndex, MenuIndex)>,
    pub draw_background:       Option<fn(*mut types::GContext, *const types::Layer, bool, &T)>,
}

impl<T> Default for TypedMenuCallbacks<T> {
    fn default() -> Self {
        TypedMenuCallbacks {
            get_num_sections:      None,
            get_num_rows:          None,
            get_cell_height:       None,
            get_header_height:     None,
            draw_row:              None,
            draw_header:           None,
            select_click:          None,
            select_long_click:     None,
            selection_changed:     None,
            get_separator_height:  None,
            draw_separator:        None,
            selection_will_change: None,
            draw_background:       None,
        }
    }
}

struct MenuLayerContext<T> {
    context:   *mut T,
    callbacks: TypedMenuCallbacks<T>,
}

extern "C" fn trampoline_num_sections<T>(_: *mut u8, ctx: *mut ()) -> u16 {
    let c = unsafe { &*(ctx as *const MenuLayerContext<T>) };
    if let Some(f) = c.callbacks.get_num_sections { f(unsafe { &*c.context }) } else { 1 }
}
extern "C" fn trampoline_num_rows<T>(_: *mut u8, section: u16, ctx: *mut ()) -> u16 {
    let c = unsafe { &*(ctx as *const MenuLayerContext<T>) };
    if let Some(f) = c.callbacks.get_num_rows { f(unsafe { &*c.context }, section) } else { 0 }
}
extern "C" fn trampoline_cell_height<T>(_: *mut u8, index: *const MenuIndex, ctx: *mut ()) -> i16 {
    let c = unsafe { &*(ctx as *const MenuLayerContext<T>) };
    if let Some(f) = c.callbacks.get_cell_height { f(unsafe { &*c.context }, unsafe { &*index }) } else { 0 }
}
extern "C" fn trampoline_header_height<T>(_: *mut u8, section: u16, ctx: *mut ()) -> i16 {
    let c = unsafe { &*(ctx as *const MenuLayerContext<T>) };
    if let Some(f) = c.callbacks.get_header_height { f(unsafe { &*c.context }, section) } else { 0 }
}
extern "C" fn trampoline_draw_row<T>(gctx: *mut types::GContext, cell: *const types::Layer, index: *const MenuIndex, ctx: *mut ()) {
    let c = unsafe { &*(ctx as *const MenuLayerContext<T>) };
    if let Some(f) = c.callbacks.draw_row { f(gctx, cell, unsafe { &*index }, unsafe { &*c.context }) }
}
extern "C" fn trampoline_draw_header<T>(gctx: *mut types::GContext, cell: *const types::Layer, section: u16, ctx: *mut ()) {
    let c = unsafe { &*(ctx as *const MenuLayerContext<T>) };
    if let Some(f) = c.callbacks.draw_header { f(gctx, cell, section, unsafe { &*c.context }) }
}
extern "C" fn trampoline_select_click<T>(_: *mut u8, index: *const MenuIndex, ctx: *mut ()) {
    let c = unsafe { &*(ctx as *const MenuLayerContext<T>) };
    if let Some(f) = c.callbacks.select_click { f(unsafe { &*c.context }, unsafe { &*index }) }
}
extern "C" fn trampoline_select_long_click<T>(_: *mut u8, index: *const MenuIndex, ctx: *mut ()) {
    let c = unsafe { &*(ctx as *const MenuLayerContext<T>) };
    if let Some(f) = c.callbacks.select_long_click { f(unsafe { &*c.context }, unsafe { &*index }) }
}
extern "C" fn trampoline_selection_changed<T>(_: *mut u8, new_index: MenuIndex, old_index: MenuIndex, ctx: *mut ()) {
    let c = unsafe { &*(ctx as *const MenuLayerContext<T>) };
    if let Some(f) = c.callbacks.selection_changed { f(unsafe { &*c.context }, new_index, old_index) }
}
extern "C" fn trampoline_separator_height<T>(_: *mut u8, index: *const MenuIndex, ctx: *mut ()) -> i16 {
    let c = unsafe { &*(ctx as *const MenuLayerContext<T>) };
    if let Some(f) = c.callbacks.get_separator_height { f(unsafe { &*c.context }, unsafe { &*index }) } else { 0 }
}
extern "C" fn trampoline_draw_separator<T>(gctx: *mut types::GContext, cell: *const types::Layer, index: *const MenuIndex, ctx: *mut ()) {
    let c = unsafe { &*(ctx as *const MenuLayerContext<T>) };
    if let Some(f) = c.callbacks.draw_separator { f(gctx, cell, unsafe { &*index }, unsafe { &*c.context }) }
}
extern "C" fn trampoline_selection_will_change<T>(_: *mut u8, new_index: *mut MenuIndex, old_index: MenuIndex, ctx: *mut ()) {
    let c = unsafe { &*(ctx as *const MenuLayerContext<T>) };
    if let Some(f) = c.callbacks.selection_will_change { f(unsafe { &*c.context }, unsafe { &mut *new_index }, old_index) }
}
extern "C" fn trampoline_draw_background<T>(gctx: *mut types::GContext, cell: *const types::Layer, highlighted: bool, ctx: *mut ()) {
    let c = unsafe { &*(ctx as *const MenuLayerContext<T>) };
    if let Some(f) = c.callbacks.draw_background { f(gctx, cell, highlighted, unsafe { &*c.context }) }
}

pub fn menu_cell_basic_draw(ctx: *mut types::GContext, cell: *const types::Layer, title: &CStr, subtitle: &CStr, icon: Option<*mut types::GBitmap>) {
    interface::menu_cell_basic_draw(ctx, cell, title, subtitle, icon);
}

pub fn menu_cell_basic_header_draw(ctx: *mut types::GContext, cell: *const types::Layer, title: &CStr) {
    interface::menu_cell_basic_header_draw(ctx, cell, title);
}
