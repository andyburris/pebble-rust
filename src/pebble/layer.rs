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
use core::marker::PhantomData;
use crate::pebble::internal::functions::interface::text_layer_set_background_color;
use crate::pebble::internal::{functions::interface, types};
use crate::pebble::internal::types::{RawMenuLayer, MenuLayerCallbacks, MenuIndex, MenuRowAlign};
use crate::pebble::types::{GBitmap, GCompOp, GRect};
use crate::system::fonts::GFont;
use crate::pebble::window::Window;
use crate::types::GColor;

/// Common operations on any layer, owned or borrowed.
pub trait AsLayer {
    fn as_raw(&self) -> *mut types::RawLayer;
    fn get_bounds(&self) -> GRect {
        interface::layer_get_bounds(self.as_raw())
    }
    fn get_frame(&self) -> GRect {
        interface::layer_get_frame(self.as_raw())
    }
    fn set_frame(&self, frame: GRect) {
        interface::layer_set_frame(self.as_raw(), frame);
    }
    fn add_child(&self, layer: &dyn AsLayer) {
        interface::layer_add_child(self.as_raw(), layer.as_raw())
    }
    fn remove_from_parent(&self) {
        interface::layer_remove_from_parent(self.as_raw());
    }
    fn mark_dirty(&self) {
        interface::layer_mark_dirty(self.as_raw());
    }
    fn set_hidden(&self, hidden: bool) {
        interface::layer_set_hidden(self.as_raw(), hidden);
    }
}

/// An owned layer: created with `Layer::new` and destroyed on drop.
pub struct Layer {
    internal: *mut types::RawLayer
}

impl AsLayer for Layer {
    fn as_raw(&self) -> *mut types::RawLayer {
        self.internal
    }
}

impl Layer {
    pub fn new(bounds: GRect) -> Layer {
        Layer {
            internal: interface::layer_create(bounds)
        }
    }

    pub fn set_update_proc(&self, func: extern "C" fn(*mut types::RawLayer, *mut types::GContext)) {
        interface::layer_set_update_proc(self.internal, func);
    }
}

impl Drop for Layer {
    fn drop(&mut self) {
        interface::layer_destroy(self.internal);
    }
}

/// A non-owning handle to a layer the app does not own — e.g. a window's root layer
/// (owned by the window), or a layer referenced only to mark it dirty. Copyable and
/// never destroys the underlying layer.
#[derive(Copy, Clone)]
pub struct LayerRef {
    raw: *mut types::RawLayer,
}

impl LayerRef {
    pub fn from_raw(raw: *mut types::RawLayer) -> LayerRef {
        LayerRef { raw }
    }
}

impl AsLayer for LayerRef {
    fn as_raw(&self) -> *mut types::RawLayer {
        self.raw
    }
}

pub struct TextLayer {
    internal: *mut types::RawTextLayer,
    inner: *mut types::RawLayer
}

pub struct BitmapLayer {
    internal: *mut types::RawBitmapLayer,
    inner: *mut types::RawLayer
}

impl AsLayer for TextLayer {
    fn as_raw(&self) -> *mut types::RawLayer {
        self.inner
    }
}

impl TextLayer {
    pub fn new(bounds: GRect) -> TextLayer {
        let internal = interface::text_layer_create(bounds);
        text_layer_set_background_color(internal, GColor::Clear);
        let inner = interface::text_layer_get_layer(internal);

        TextLayer {
            internal, inner
        }
    }

    pub fn set_text(&self, text: &CStr) {
        interface::text_layer_set_text(self.internal, text);
    }

    pub fn set_font(&self, font: GFont) {
        interface::text_layer_set_font(self.internal, font.internal)
    }

    pub fn set_background_color(&self, color: types::GColor) {
        interface::text_layer_set_background_color(self.internal, color);
    }

    pub fn set_text_color(&self, color: types::GColor) {
        interface::text_layer_set_text_color(self.internal, color);
    }

    pub fn set_text_alignment(&self, alignment: types::GTextAlignment) {
        interface::text_layer_set_text_alignment(self.internal, alignment);
    }
}

impl Drop for TextLayer {
    fn drop(&mut self) {
        // Frees the text layer and its inner layer; don't also destroy `inner`.
        interface::text_layer_destroy(self.internal);
    }
}

impl AsLayer for BitmapLayer {
    fn as_raw(&self) -> *mut types::RawLayer {
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

    pub fn set_bitmap(&self, bitmap: &GBitmap) {
        interface::bitmap_layer_set_bitmap(self.internal, bitmap.internal);
    }

    pub fn set_compositing_mode(&self, mode: GCompOp) {
        interface::bitmap_layer_set_compositing_mode(self.internal, mode);
    }
}

impl Drop for BitmapLayer {
    fn drop(&mut self) {
        // Frees the bitmap layer and its inner layer; don't also destroy `inner`.
        interface::bitmap_layer_destroy(self.internal);
    }
}

pub struct MenuLayer<T> {
    internal: *mut RawMenuLayer,
    inner:    *mut types::RawLayer,
    _ctx:     Box<MenuLayerContext<T>>,
}

impl<T> AsLayer for MenuLayer<T> {
    fn as_raw(&self) -> *mut types::RawLayer { self.inner }
}

impl<T> MenuLayer<T> {
    /// Create a menu layer. `context` is owned by the layer (moved into a stable heap
    /// box) and handed to every callback as `&T`; retrieve it later with `context()`.
    pub fn new(frame: GRect, context: T, callbacks: TypedMenuCallbacks<T>) -> Self {
        let internal = interface::menu_layer_create(frame);
        let mut ctx = Box::new(MenuLayerContext { menu: internal, context, callbacks });
        let ctx_ptr: *mut MenuLayerContext<T> = &mut *ctx;
        let raw_cbs = MenuLayerCallbacks {
            get_num_sections:      ctx.callbacks.get_num_sections      .map(|_| trampoline_num_sections::<T>      as extern "C" fn(*mut u8, *mut ()) -> u16),
            get_num_rows:          ctx.callbacks.get_num_rows          .map(|_| trampoline_num_rows::<T>          as extern "C" fn(*mut u8, u16, *mut ()) -> u16),
            get_cell_height:       ctx.callbacks.get_cell_height       .map(|_| trampoline_cell_height::<T>       as extern "C" fn(*mut u8, *const MenuIndex, *mut ()) -> i16),
            get_header_height:     ctx.callbacks.get_header_height     .map(|_| trampoline_header_height::<T>     as extern "C" fn(*mut u8, u16, *mut ()) -> i16),
            draw_row:              ctx.callbacks.draw_row              .map(|_| trampoline_draw_row::<T>          as extern "C" fn(*mut types::GContext, *const types::RawLayer, *const MenuIndex, *mut ())),
            draw_header:           ctx.callbacks.draw_header           .map(|_| trampoline_draw_header::<T>       as extern "C" fn(*mut types::GContext, *const types::RawLayer, u16, *mut ())),
            select_click:          ctx.callbacks.select_click          .map(|_| trampoline_select_click::<T>      as extern "C" fn(*mut u8, *const MenuIndex, *mut ())),
            select_long_click:     ctx.callbacks.select_long_click     .map(|_| trampoline_select_long_click::<T> as extern "C" fn(*mut u8, *const MenuIndex, *mut ())),
            selection_changed:     ctx.callbacks.selection_changed     .map(|_| trampoline_selection_changed::<T> as extern "C" fn(*mut u8, MenuIndex, MenuIndex, *mut ())),
            get_separator_height:  ctx.callbacks.get_separator_height  .map(|_| trampoline_separator_height::<T>  as extern "C" fn(*mut u8, *const MenuIndex, *mut ()) -> i16),
            draw_separator:        ctx.callbacks.draw_separator        .map(|_| trampoline_draw_separator::<T>    as extern "C" fn(*mut types::GContext, *const types::RawLayer, *const MenuIndex, *mut ())),
            selection_will_change: ctx.callbacks.selection_will_change .map(|_| trampoline_selection_will_change::<T> as extern "C" fn(*mut u8, *mut MenuIndex, MenuIndex, *mut ())),
            draw_background:       ctx.callbacks.draw_background       .map(|_| trampoline_draw_background::<T>   as extern "C" fn(*mut types::GContext, *const types::RawLayer, bool, *mut ())),
        };
        interface::menu_layer_set_callbacks(internal, ctx_ptr, raw_cbs);
        let inner = interface::menu_layer_get_layer(internal);
        MenuLayer { internal, inner, _ctx: ctx }
    }

    /// The context handed to callbacks.
    pub fn context(&self) -> &T { &self._ctx.context }

    pub fn set_normal_colors(&self, background: GColor, foreground: GColor) {
        interface::menu_layer_set_normal_colors(self.internal, background, foreground);
    }

    pub fn set_highlight_colors(&self, background: GColor, foreground: GColor) {
        interface::menu_layer_set_highlight_colors(self.internal, background, foreground);
    }


    pub fn set_click_config_onto_window(&self, window: &Window) {
        interface::menu_layer_set_click_config_onto_window(self.internal, window.raw());
    }

    pub fn reload_data(&self) {
        interface::menu_layer_reload_data(self.internal);
    }

    pub fn get_selected_index(&self) -> MenuIndex {
        interface::menu_layer_get_selected_index(self.internal)
    }

    pub fn is_index_selected(&self, index: &MenuIndex) -> bool {
        interface::menu_layer_is_index_selected(self.internal, index)
    }

    pub fn get_center_focused(&self) -> bool {
        interface::menu_layer_get_center_focused(self.internal)
    }

    pub fn set_center_focused(&self, center_focused: bool) {
        interface::menu_layer_set_center_focused(self.internal, center_focused);
    }

    pub fn set_selected_index(&self, index: MenuIndex, scroll_align: MenuRowAlign, animated: bool) {
        interface::menu_layer_set_selected_index(self.internal, index, scroll_align, animated);
    }

    pub fn set_selected_next(&self, up: bool, scroll_align: MenuRowAlign, animated: bool) {
        interface::menu_layer_set_selected_next(self.internal, up, scroll_align, animated);
    }

    pub fn pad_bottom_enable(&self, enable: bool) {
        interface::menu_layer_pad_bottom_enable(self.internal, enable);
    }
}

impl<T> Drop for MenuLayer<T> {
    fn drop(&mut self) {
        // Frees the menu layer and its inner layer; the boxed context drops after.
        interface::menu_layer_destroy(self.internal);
    }
}

// ── DrawLayer ─────────────────────────────────────────────────────────────────

struct DrawLayerContext<T> {
    context: T,
    draw: fn(&mut types::GContext, &mut T, GRect),
}

pub struct DrawLayer<T> {
    internal: *mut types::RawLayer,
    _ctx: Box<DrawLayerContext<T>>,
}

impl<T> AsLayer for DrawLayer<T> {
    fn as_raw(&self) -> *mut types::RawLayer { self.internal }
}

impl<T> DrawLayer<T> {
    /// Create a custom-drawn layer. `context` is owned by the layer (moved into a
    /// stable heap box); the draw callback gets `&mut T` and the layer's current
    /// bounds, read fresh each paint. Retrieve the context later with `context()`.
    pub fn new(frame: GRect, context: T, draw: fn(&mut types::GContext, &mut T, GRect)) -> DrawLayer<T> {
        let internal = interface::layer_create_with_data(frame, core::mem::size_of::<*mut DrawLayerContext<T>>());
        let mut ctx = Box::new(DrawLayerContext { context, draw });
        let ctx_ptr: *mut DrawLayerContext<T> = &mut *ctx;
        // Store a pointer to the context in the layer's extra data bytes.
        let data = interface::layer_get_data(internal) as *mut *mut DrawLayerContext<T>;
        unsafe { *data = ctx_ptr; }
        interface::layer_set_update_proc(internal, draw_trampoline::<T>);
        DrawLayer { internal, _ctx: ctx }
    }

    /// The context handed to the draw callback.
    pub fn context(&self) -> &T { &self._ctx.context }
}

impl<T> Drop for DrawLayer<T> {
    fn drop(&mut self) {
        interface::layer_destroy(self.internal);
    }
}

extern "C" fn draw_trampoline<T>(layer: *mut types::RawLayer, ctx: *mut types::GContext) {
    let data = interface::layer_get_data(layer) as *const *mut DrawLayerContext<T>;
    let draw_ctx = unsafe { &mut **data };
    let frame = interface::layer_get_bounds(layer);   // draw-time bounds (survives moves)
    let f = draw_ctx.draw;
    f(unsafe { &mut *ctx }, &mut draw_ctx.context, frame);
}

pub const MENU_CELL_BASIC_HEADER_HEIGHT: i16 = 16;

pub struct TypedMenuCallbacks<T> {
    pub get_num_sections:      Option<fn(MenuLayerRef, &T) -> u16>,
    pub get_num_rows:          Option<fn(MenuLayerRef, &T, u16) -> u16>,
    pub get_cell_height:       Option<fn(MenuLayerRef, &T, &MenuIndex) -> i16>,
    pub get_header_height:     Option<fn(MenuLayerRef, &T, u16) -> i16>,
    pub draw_row:              Option<fn(MenuLayerRef, &mut types::GContext, &MenuCellLayer, &MenuIndex, &T)>,
    pub draw_header:           Option<fn(MenuLayerRef, &mut types::GContext, &MenuCellLayer, u16, &T)>,
    pub select_click:          Option<fn(MenuLayerRef, &T, &MenuIndex)>,
    pub select_long_click:     Option<fn(MenuLayerRef, &T, &MenuIndex)>,
    pub selection_changed:     Option<fn(MenuLayerRef, &T, MenuIndex, MenuIndex)>,
    pub get_separator_height:  Option<fn(MenuLayerRef, &T, &MenuIndex) -> i16>,
    pub draw_separator:        Option<fn(MenuLayerRef, &mut types::GContext, &MenuCellLayer, &MenuIndex, &T)>,
    pub selection_will_change: Option<fn(MenuLayerRef, &T, &mut MenuIndex, MenuIndex)>,
    pub draw_background:       Option<fn(MenuLayerRef, &mut types::GContext, &MenuCellLayer, bool, &T)>,
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
    menu:      *mut RawMenuLayer,
    context:   T,
    callbacks: TypedMenuCallbacks<T>,
}

extern "C" fn trampoline_num_sections<T>(_: *mut u8, ctx: *mut ()) -> u16 {
    let c = unsafe { &*(ctx as *const MenuLayerContext<T>) };
    if let Some(f) = c.callbacks.get_num_sections { f(MenuLayerRef::from_raw(c.menu), &c.context) } else { 1 }
}
extern "C" fn trampoline_num_rows<T>(_: *mut u8, section: u16, ctx: *mut ()) -> u16 {
    let c = unsafe { &*(ctx as *const MenuLayerContext<T>) };
    if let Some(f) = c.callbacks.get_num_rows { f(MenuLayerRef::from_raw(c.menu), &c.context, section) } else { 0 }
}
extern "C" fn trampoline_cell_height<T>(_: *mut u8, index: *const MenuIndex, ctx: *mut ()) -> i16 {
    let c = unsafe { &*(ctx as *const MenuLayerContext<T>) };
    if let Some(f) = c.callbacks.get_cell_height { f(MenuLayerRef::from_raw(c.menu), &c.context, unsafe { &*index }) } else { 0 }
}
extern "C" fn trampoline_header_height<T>(_: *mut u8, section: u16, ctx: *mut ()) -> i16 {
    let c = unsafe { &*(ctx as *const MenuLayerContext<T>) };
    if let Some(f) = c.callbacks.get_header_height { f(MenuLayerRef::from_raw(c.menu), &c.context, section) } else { 0 }
}
extern "C" fn trampoline_draw_row<T>(gctx: *mut types::GContext, cell: *const types::RawLayer, index: *const MenuIndex, ctx: *mut ()) {
    let c = unsafe { &*(ctx as *const MenuLayerContext<T>) };
    if let Some(f) = c.callbacks.draw_row { f(MenuLayerRef::from_raw(c.menu), unsafe { &mut *gctx }, &MenuCellLayer::from_raw(cell), unsafe { &*index }, &c.context) }
}
extern "C" fn trampoline_draw_header<T>(gctx: *mut types::GContext, cell: *const types::RawLayer, section: u16, ctx: *mut ()) {
    let c = unsafe { &*(ctx as *const MenuLayerContext<T>) };
    if let Some(f) = c.callbacks.draw_header { f(MenuLayerRef::from_raw(c.menu), unsafe { &mut *gctx }, &MenuCellLayer::from_raw(cell), section, &c.context) }
}
extern "C" fn trampoline_select_click<T>(_: *mut u8, index: *const MenuIndex, ctx: *mut ()) {
    let c = unsafe { &*(ctx as *const MenuLayerContext<T>) };
    if let Some(f) = c.callbacks.select_click { f(MenuLayerRef::from_raw(c.menu), &c.context, unsafe { &*index }) }
}
extern "C" fn trampoline_select_long_click<T>(_: *mut u8, index: *const MenuIndex, ctx: *mut ()) {
    let c = unsafe { &*(ctx as *const MenuLayerContext<T>) };
    if let Some(f) = c.callbacks.select_long_click { f(MenuLayerRef::from_raw(c.menu), &c.context, unsafe { &*index }) }
}
extern "C" fn trampoline_selection_changed<T>(_: *mut u8, new_index: MenuIndex, old_index: MenuIndex, ctx: *mut ()) {
    let c = unsafe { &*(ctx as *const MenuLayerContext<T>) };
    if let Some(f) = c.callbacks.selection_changed { f(MenuLayerRef::from_raw(c.menu), &c.context, new_index, old_index) }
}
extern "C" fn trampoline_separator_height<T>(_: *mut u8, index: *const MenuIndex, ctx: *mut ()) -> i16 {
    let c = unsafe { &*(ctx as *const MenuLayerContext<T>) };
    if let Some(f) = c.callbacks.get_separator_height { f(MenuLayerRef::from_raw(c.menu), &c.context, unsafe { &*index }) } else { 0 }
}
extern "C" fn trampoline_draw_separator<T>(gctx: *mut types::GContext, cell: *const types::RawLayer, index: *const MenuIndex, ctx: *mut ()) {
    let c = unsafe { &*(ctx as *const MenuLayerContext<T>) };
    if let Some(f) = c.callbacks.draw_separator { f(MenuLayerRef::from_raw(c.menu), unsafe { &mut *gctx }, &MenuCellLayer::from_raw(cell), unsafe { &*index }, &c.context) }
}
extern "C" fn trampoline_selection_will_change<T>(_: *mut u8, new_index: *mut MenuIndex, old_index: MenuIndex, ctx: *mut ()) {
    let c = unsafe { &*(ctx as *const MenuLayerContext<T>) };
    if let Some(f) = c.callbacks.selection_will_change { f(MenuLayerRef::from_raw(c.menu), &c.context, unsafe { &mut *new_index }, old_index) }
}
extern "C" fn trampoline_draw_background<T>(gctx: *mut types::GContext, cell: *const types::RawLayer, highlighted: bool, ctx: *mut ()) {
    let c = unsafe { &*(ctx as *const MenuLayerContext<T>) };
    if let Some(f) = c.callbacks.draw_background { f(MenuLayerRef::from_raw(c.menu), unsafe { &mut *gctx }, &MenuCellLayer::from_raw(cell), highlighted, &c.context) }
}

/// A menu cell, handed to the draw callbacks. Wraps the cell layer for the
/// duration of the callback; draw into it with these methods instead of touching
/// the raw `*const RawLayer` / `*mut GContext`.
pub struct MenuCellLayer<'a> {
    raw:   *const types::RawLayer,
    _life: PhantomData<&'a ()>,
}

impl<'a> MenuCellLayer<'a> {
    fn from_raw(raw: *const types::RawLayer) -> Self {
        MenuCellLayer { raw, _life: PhantomData }
    }

    pub fn draw_basic(&self, ctx: &mut types::GContext, title: Option<&CStr>, subtitle: Option<&CStr>, icon: Option<&GBitmap>) {
        interface::menu_cell_basic_draw(ctx, self.raw, title, subtitle, icon.map(|b| b.internal));
    }

    pub fn draw_header(&self, ctx: &mut types::GContext, title: &CStr) {
        interface::menu_cell_basic_header_draw(ctx, self.raw, title);
    }

    pub fn draw_title(&self, ctx: &mut types::GContext, title: &CStr) {
        interface::menu_cell_title_draw(ctx, self.raw, title);
    }

    /// Whether this cell is the currently-selected (highlighted) one.
    pub fn is_highlighted(&self) -> bool {
        interface::menu_cell_layer_is_highlighted(self.raw)
    }
}

/// A borrowed view of the parent `MenuLayer`, handed to every menu callback as
/// its first argument. Read-only and valid only for the callback (don't store
/// it); the owned `MenuLayer<T>` carries the mutators. Lets a callback ask the
/// menu about its own state — e.g. `menu.get_selected_index()`.
#[derive(Copy, Clone)]
pub struct MenuLayerRef<'a> {
    raw:   *mut RawMenuLayer,
    _life: PhantomData<&'a ()>,
}

impl<'a> MenuLayerRef<'a> {
    fn from_raw(raw: *mut RawMenuLayer) -> Self {
        MenuLayerRef { raw, _life: PhantomData }
    }

    pub fn get_selected_index(&self) -> MenuIndex {
        interface::menu_layer_get_selected_index(self.raw)
    }

    pub fn is_index_selected(&self, index: &MenuIndex) -> bool {
        interface::menu_layer_is_index_selected(self.raw, index)
    }

    pub fn get_center_focused(&self) -> bool {
        interface::menu_layer_get_center_focused(self.raw)
    }
}

impl AsLayer for MenuCellLayer<'_> {
    fn as_raw(&self) -> *mut types::RawLayer {
        // The cell is handed to us as *const (the SDK owns it, transient for one
        // paint); cast for the AsLayer contract. Only sound for the read-only
        // methods (get_bounds/get_frame) — don't mutate a cell.
        self.raw as *mut types::RawLayer
    }
}
