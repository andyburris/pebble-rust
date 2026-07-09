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
#![allow(unused)]

use core::mem;
use core::ffi::CStr;

use crate::pebble::internal::types::*;

use crate::pebble::internal::functions::declarations;

pub fn app_event_loop() {
    unsafe {
        declarations::app_event_loop();
    }
}

pub fn window_create() -> *mut RawWindow {
    unsafe {
        declarations::window_create()
    }
}

pub fn window_destroy(window: *mut RawWindow) {
    unsafe {
        declarations::window_destroy(window);
    }
}

pub fn window_set_click_config_provider<T>(window: *mut RawWindow, func: extern "C" fn(*mut T)) {
    unsafe {
        declarations::window_set_click_config_provider(window, mem::transmute(func));
    }
}

pub fn window_set_click_config_provider_with_context<T>(window: *mut RawWindow, func: extern "C" fn(*mut T), ctx: *mut T) {
    unsafe {
        declarations::window_set_click_config_provider_with_context(window,
                                                                mem::transmute(func),
                                                                ctx as *mut u8);
    }
}

pub fn window_set_window_handlers(window: *mut RawWindow, handlers: WindowHandlers) {
    unsafe {
        declarations::window_set_window_handlers(window, handlers);
    }
}

pub fn window_set_background_color(window: *mut RawWindow, color: GColor) {
    unsafe {
        declarations::window_set_background_color(window, color);
    }
}

pub fn window_set_user_data<T>(window: *mut RawWindow, data: *mut T) {
    unsafe {
        declarations::window_set_user_data(window, data as *mut c_void);
    }
}

pub fn window_get_user_data<T>(window: *mut RawWindow) -> *mut T {
    unsafe {
        declarations::window_get_user_data(window) as *mut T
    }
}

pub fn window_stack_push(window: *mut RawWindow, animate: bool) {
    unsafe {
        if animate {
            declarations::window_stack_push(window, 1);
        } else {
            declarations::window_stack_push(window, 0);
        }
    }
}

pub fn window_get_root_layer(window: *mut RawWindow) -> *mut RawLayer {
    unsafe {
        declarations::window_get_root_layer(window)
    }
}

pub fn window_single_click_subscribe(button: u8, func: extern "C" fn(*mut ClickRecognizer, *mut u8)) {
    unsafe {
        declarations::window_single_click_subscribe(button, func);
    }
}

pub fn window_single_repeating_click_subscribe(button: u8, repeat_interval_ms: u16, func: extern "C" fn(*mut ClickRecognizer, *mut u8)) {
    unsafe {
        declarations::window_single_repeating_click_subscribe(button, repeat_interval_ms, func);
    }
}

pub fn window_long_click_subscribe(button: u8, delay_ms: u16, down: Option<extern "C" fn(*mut ClickRecognizer, *mut u8)>, up: Option<extern "C" fn(*mut ClickRecognizer, *mut u8)>) {
    unsafe {
        declarations::window_long_click_subscribe(button, delay_ms, down, up);
    }
}

pub fn window_multi_click_subscribe(button: u8, min_clicks: u8, max_clicks: u8, timeout: u16, last_click_only: bool, func: extern "C" fn(*mut ClickRecognizer, *mut u8)) {
    unsafe {
        declarations::window_multi_click_subscribe(button, min_clicks, max_clicks, timeout, last_click_only, func);
    }
}

pub fn layer_create(bounds: GRect) -> *mut RawLayer {
    unsafe {
        declarations::layer_create(bounds)
    }
}

pub fn layer_create_with_data(bounds: GRect, data_size: usize) -> *mut RawLayer {
    unsafe {
        declarations::layer_create_with_data(bounds, data_size)
    }
}

pub fn layer_get_data(layer: *const RawLayer) -> *mut c_void {
    unsafe {
        declarations::layer_get_data(layer)
    }
}

pub fn layer_destroy(layer: *mut RawLayer) {
    unsafe {
        declarations::layer_destroy(layer);
    }
}

pub fn layer_get_frame(layer: *mut RawLayer) -> GRect {
    unsafe {
        declarations::layer_get_frame(layer)
    }
}

pub fn layer_set_frame(layer: *mut RawLayer, frame: GRect) {
    unsafe {
        declarations::layer_set_frame(layer, frame);
    }
}

pub fn layer_get_bounds(layer: *mut RawLayer) -> GRect {
    unsafe {
        declarations::layer_get_bounds(layer)
    }
}

pub fn layer_add_child(layer: *mut RawLayer, child: *mut RawLayer) {
    unsafe {
        declarations::layer_add_child(layer, child);
    }
}

pub fn layer_remove_from_parent(layer: *mut RawLayer) {
    unsafe {
        declarations::layer_remove_from_parent(layer);
    }
}

pub fn layer_mark_dirty(layer: *mut RawLayer) {
    unsafe {
        declarations::layer_mark_dirty(layer);
    }
}

pub fn layer_set_update_proc(layer: *mut RawLayer, func: extern "C" fn(*mut RawLayer, *mut GContext)) {
    unsafe {
        declarations::layer_set_update_proc(layer, func);
    }
}

pub fn layer_set_hidden(layer: *mut RawLayer, hidden: bool) {
    unsafe {
        declarations::layer_set_hidden(layer, hidden);
    }
}

pub fn text_layer_create(bounds: GRect) -> *mut RawTextLayer {
    unsafe {
        declarations::text_layer_create(bounds)
    }
}

pub fn text_layer_destroy(layer: *mut RawTextLayer) {
    unsafe {
        declarations::text_layer_destroy(layer);
    }
}

pub fn text_layer_set_text(layer: *mut RawTextLayer, text: &CStr) {
    unsafe {
        declarations::text_layer_set_text(layer, text.as_ptr() as *const u8);
    }
}

pub fn text_layer_set_font(layer: *mut RawTextLayer, font: RawGFont) {
    unsafe {
        declarations::text_layer_set_font(layer, font);
    }
}

pub fn text_layer_get_layer(layer: *mut RawTextLayer) -> *mut RawLayer {
    unsafe {
        declarations::text_layer_get_layer(layer)
    }
}

pub fn text_layer_set_background_color(layer: *mut RawTextLayer, color: GColor) {
    unsafe { declarations::text_layer_set_background_color(layer, color); }
}

pub fn text_layer_set_text_color(layer: *mut RawTextLayer, color: GColor) {
    unsafe { declarations::text_layer_set_text_color(layer, color); }
}

pub fn text_layer_set_text_alignment(layer: *mut RawTextLayer, alignment: GTextAlignment) {
    unsafe { declarations::text_layer_set_text_alignment(layer, alignment); }
}

pub fn gbitmap_create_with_resource(id: u32) -> *mut RawGBitmap {
    unsafe {
        declarations::gbitmap_create_with_resource(id)
    }
}

pub fn bitmap_layer_create(frame: GRect) -> *mut RawBitmapLayer {
    unsafe {
        declarations::bitmap_layer_create(frame)
    }
}

pub fn bitmap_layer_destroy(layer: *mut RawBitmapLayer) {
    unsafe {
        declarations::bitmap_layer_destroy(layer);
    }
}

pub fn bitmap_layer_set_bitmap(layer: *mut RawBitmapLayer, bitmap: *mut RawGBitmap) {
    unsafe {
        declarations::bitmap_layer_set_bitmap(layer, bitmap);
    }
}

pub fn bitmap_layer_set_compositing_mode(layer: *mut RawBitmapLayer, mode: GCompOp) {
    unsafe {
        declarations::bitmap_layer_set_compositing_mode(layer, mode);
    }
}

pub fn bitmap_layer_get_layer(layer: *mut RawBitmapLayer) -> *mut RawLayer {
    unsafe {
        declarations::bitmap_layer_get_layer(layer)
    }
}

pub fn graphics_context_set_fill_color(ctx: *mut GContext, color: GColor) {
    unsafe {
        declarations::graphics_context_set_fill_color(ctx, color);
    }
}

pub fn graphics_context_set_text_color(ctx: *mut GContext, color: GColor) {
    unsafe {
        declarations::graphics_context_set_text_color(ctx, color);
    }
}

pub fn graphics_context_set_stroke_color(ctx: *mut GContext, color: GColor) {
    unsafe {
        declarations::graphics_context_set_stroke_color(ctx, color);
    }
}

pub fn graphics_context_set_stroke_width(ctx: *mut GContext, stroke_width: u8) {
    unsafe {
        declarations::graphics_context_set_stroke_width(ctx, stroke_width);
    }
}

pub fn graphics_context_set_compositing_mode(ctx: *mut GContext, mode: GCompOp) {
    unsafe {
        declarations::graphics_context_set_compositing_mode(ctx, mode);
    }
}

pub fn graphics_draw_bitmap_in_rect(ctx: *mut GContext, bitmap: *const RawGBitmap, dest_rect: GRect) {
    unsafe {
        declarations::graphics_draw_bitmap_in_rect(ctx, bitmap, dest_rect);
    }
}

pub fn graphics_fill_circle(ctx: *mut GContext, center: GPoint, radius: u16) {
    unsafe {
        declarations::graphics_fill_circle(ctx, center, radius);
    }
}

pub fn graphics_fill_rect(ctx: *mut GContext, rect: GRect, corner_radius: u16, corner_mask: GCornerMask) {
    unsafe {
        declarations::graphics_fill_rect(ctx, rect, corner_radius, corner_mask);
    }
}


pub fn graphics_draw_line(ctx: *mut GContext, p0: GPoint, p1: GPoint) {
    unsafe {
        declarations::graphics_draw_line(ctx, p0, p1);
    }
}

pub fn graphics_draw_text(ctx: *mut GContext, text: &CStr, font: RawGFont, rect: GRect, overflow: GTextOverflowMode, alignment: GTextAlignment) {
    unsafe {
        declarations::graphics_draw_text(ctx, text.as_ptr() as *const u8, font, rect, overflow, alignment, core::ptr::null_mut());
    }
}

pub fn graphics_text_layout_get_content_size(text: &CStr, font: RawGFont, rect: GRect, overflow: GTextOverflowMode, alignment: GTextAlignment) -> GSize {
    unsafe {
        declarations::graphics_text_layout_get_content_size(text.as_ptr() as *const u8, font, rect, overflow, alignment)
    }
}

pub fn gpoint_equal(a: &GPoint, b: &GPoint) -> bool {
    unsafe { declarations::gpoint_equal(a, b) }
}

pub fn gsize_equal(a: &GSize, b: &GSize) -> bool {
    unsafe { declarations::gsize_equal(a, b) }
}

pub fn grect_equal(a: &GRect, b: &GRect) -> bool {
    unsafe { declarations::grect_equal(a, b) }
}

pub fn grect_is_empty(rect: &GRect) -> bool {
    unsafe { declarations::grect_is_empty(rect) }
}

pub fn grect_standardize(rect: &mut GRect) {
    unsafe { declarations::grect_standardize(rect); }
}

pub fn grect_clip(rect_to_clip: &mut GRect, rect_clipper: &GRect) {
    unsafe { declarations::grect_clip(rect_to_clip, rect_clipper); }
}

pub fn grect_contains_point(rect: &GRect, point: &GPoint) -> bool {
    unsafe { declarations::grect_contains_point(rect, point) }
}

pub fn grect_center_point(rect: &GRect) -> GPoint {
    unsafe { declarations::grect_center_point(rect) }
}

pub fn grect_crop(rect: GRect, crop_size_px: i32) -> GRect {
    unsafe { declarations::grect_crop(rect, crop_size_px) }
}

pub fn grect_inset(rect: GRect, insets: GEdgeInsets) -> GRect {
    unsafe { declarations::grect_inset(rect, insets) }
}

pub fn grect_align(rect: &mut GRect, inside_rect: &GRect, alignment: GAlign, clip: bool) {
    unsafe { declarations::grect_align(rect, inside_rect, alignment, clip); }
}

pub fn gpath_create(init: *const GPathInfo) -> *mut RawGPath {
    unsafe { declarations::gpath_create(init) }
}

pub fn gpath_destroy(path: *mut RawGPath) {
    unsafe { declarations::gpath_destroy(path); }
}

pub fn gpath_draw_filled(ctx: *mut GContext, path: *const RawGPath) {
    unsafe { declarations::gpath_draw_filled(ctx, path); }
}

pub fn gpath_draw_outline(ctx: *mut GContext, path: *const RawGPath) {
    unsafe { declarations::gpath_draw_outline(ctx, path); }
}

pub fn gpath_draw_outline_open(ctx: *mut GContext, path: *const RawGPath) {
    unsafe { declarations::gpath_draw_outline_open(ctx, path); }
}

pub fn gpath_move_to(path: *mut RawGPath, point: GPoint) {
    unsafe { declarations::gpath_move_to(path, point); }
}

pub fn gpath_rotate_to(path: *mut RawGPath, angle: i32) {
    unsafe { declarations::gpath_rotate_to(path, angle); }
}

pub fn sin_lookup(angle: i32) -> i32 {
    unsafe { declarations::sin_lookup(angle) }
}

pub fn cos_lookup(angle: i32) -> i32 {
    unsafe { declarations::cos_lookup(angle) }
}

pub fn clock_is_24h_style() -> bool {
    unsafe {
        let response = declarations::clock_is_24h_style();
        response != 0
    }
}

pub fn tick_timer_service_subscribe(unit: TimeUnits, func: extern "C" fn(*mut tm, u32)) {
    unsafe {
        declarations::tick_timer_service_subscribe(unit, func);
    }
}

pub fn time() -> time_t {
    unsafe {
        declarations::time(core::ptr::null_mut())
    }
}

pub fn localtime(now: time_t) -> *mut tm {
    unsafe {
        let now_ptr = &now as *const time_t;
        declarations::localtime(now_ptr)
    }
}

pub fn gmtime(now: time_t) -> *mut tm {
    unsafe {
        let now_ptr = &now as *const time_t;
        declarations::gmtime(now_ptr)
    }
}

pub fn mktime(t: &mut tm) -> time_t {
    unsafe { declarations::mktime(t) }
}

/// Format `t` into `buf` via C `strftime`; returns the number of bytes written
/// (excluding the terminating NUL). 0 if the result didn't fit.
pub fn strftime(buf: &mut [u8], format: &CStr, t: &tm) -> usize {
    unsafe {
        declarations::strftime(buf.as_mut_ptr(), buf.len(), format.as_ptr() as *const u8, t)
    }
}

pub fn app_log(level: u8, msg: &str, name: &str) {
    unsafe {
        declarations::app_log(level, name.as_ptr(), 2,
                              msg.as_ptr());
    }
}

pub fn menu_layer_create(frame: GRect) -> *mut RawMenuLayer {
    unsafe { declarations::menu_layer_create(frame) }
}

pub fn menu_layer_destroy(menu_layer: *mut RawMenuLayer) {
    unsafe { declarations::menu_layer_destroy(menu_layer); }
}

pub fn menu_layer_get_layer(menu_layer: *mut RawMenuLayer) -> *mut RawLayer {
    unsafe { declarations::menu_layer_get_layer(menu_layer) }
}

pub fn menu_layer_set_callbacks<T>(menu_layer: *mut RawMenuLayer, context: *mut T, callbacks: MenuLayerCallbacks) {
    unsafe { declarations::menu_layer_set_callbacks(menu_layer, context as *mut c_void, callbacks); }
}

pub fn menu_layer_set_click_config_onto_window(menu_layer: *mut RawMenuLayer, window: *mut RawWindow) {
    unsafe { declarations::menu_layer_set_click_config_onto_window(menu_layer, window); }
}

pub fn menu_layer_set_highlight_colors(menu_layer: *mut RawMenuLayer, background: GColor, foreground: GColor) {
    unsafe { declarations::menu_layer_set_highlight_colors(menu_layer, background, foreground); }
}

pub fn menu_layer_set_normal_colors(menu_layer: *mut RawMenuLayer, background: GColor, foreground: GColor) {
    unsafe { declarations::menu_layer_set_normal_colors(menu_layer, background, foreground); }
}

pub fn menu_layer_reload_data(menu_layer: *mut RawMenuLayer) {
    unsafe { declarations::menu_layer_reload_data(menu_layer); }
}

pub fn menu_layer_get_selected_index(menu_layer: *mut RawMenuLayer) -> MenuIndex {
    unsafe { declarations::menu_layer_get_selected_index(menu_layer) }
}

pub fn menu_layer_is_index_selected(menu_layer: *mut RawMenuLayer, index: &MenuIndex) -> bool {
    unsafe { declarations::menu_layer_is_index_selected(menu_layer, index) }
}

pub fn menu_layer_get_center_focused(menu_layer: *mut RawMenuLayer) -> bool {
    unsafe { declarations::menu_layer_get_center_focused(menu_layer) }
}

pub fn menu_layer_set_center_focused(menu_layer: *mut RawMenuLayer, center_focused: bool) {
    unsafe { declarations::menu_layer_set_center_focused(menu_layer, center_focused); }
}

pub fn menu_layer_set_selected_index(menu_layer: *mut RawMenuLayer, index: MenuIndex, scroll_align: MenuRowAlign, animated: bool) {
    unsafe { declarations::menu_layer_set_selected_index(menu_layer, index, scroll_align, animated); }
}

pub fn menu_layer_set_selected_next(menu_layer: *mut RawMenuLayer, up: bool, scroll_align: MenuRowAlign, animated: bool) {
    unsafe { declarations::menu_layer_set_selected_next(menu_layer, up, scroll_align, animated); }
}

pub fn menu_layer_pad_bottom_enable(menu_layer: *mut RawMenuLayer, enable: bool) {
    unsafe { declarations::menu_layer_pad_bottom_enable(menu_layer, enable); }
}

pub fn menu_cell_basic_draw(ctx: *mut GContext, cell: *const RawLayer, title: Option<&CStr>, subtitle: Option<&CStr>, icon: Option<*mut RawGBitmap>) {
    unsafe {
        declarations::menu_cell_basic_draw(
            ctx, cell,
            title.map_or(core::ptr::null_mut(), |mut t| t.as_ptr() as *const u8),
            subtitle.map_or(core::ptr::null_mut(), |mut s| s.as_ptr() as *const u8),
            icon.unwrap_or(core::ptr::null_mut()),
        );
    }
}

pub fn menu_cell_basic_header_draw(ctx: *mut GContext, cell: *const RawLayer, title: &CStr) {
    unsafe { declarations::menu_cell_basic_header_draw(ctx, cell, title.as_ptr() as *const u8); }
}

pub fn menu_cell_title_draw(ctx: *mut GContext, cell: *const RawLayer, title: &CStr) {
    unsafe { declarations::menu_cell_title_draw(ctx, cell, title.as_ptr() as *const u8); }
}

pub fn menu_cell_layer_is_highlighted(cell: *const RawLayer) -> bool {
    unsafe { declarations::menu_cell_layer_is_highlighted(cell) }
}

pub fn animation_create() -> *mut RawAnimation {
    unsafe { declarations::animation_create() }
}

pub fn animation_destroy(animation: *mut RawAnimation) -> bool {
    unsafe { declarations::animation_destroy(animation) }
}

pub fn animation_schedule(animation: *mut RawAnimation) -> bool {
    unsafe { declarations::animation_schedule(animation) }
}

pub fn animation_unschedule(animation: *mut RawAnimation) -> bool {
    unsafe { declarations::animation_unschedule(animation) }
}

pub fn animation_set_duration(animation: *mut RawAnimation, duration_ms: u32) {
    unsafe { declarations::animation_set_duration(animation, duration_ms) }
}

pub fn animation_set_delay(animation: *mut RawAnimation, delay_ms: u32) {
    unsafe { declarations::animation_set_delay(animation, delay_ms) }
}

pub fn animation_set_play_count(animation: *mut RawAnimation, play_count: u32) {
    unsafe { declarations::animation_set_play_count(animation, play_count) }
}

pub fn animation_set_reverse(animation: *mut RawAnimation, reverse: bool) {
    unsafe { declarations::animation_set_reverse(animation, reverse) }
}

pub fn animation_set_curve(animation: *mut RawAnimation, curve: AnimationCurve) {
    unsafe { declarations::animation_set_curve(animation, curve) }
}

pub fn animation_set_handlers(animation: *mut RawAnimation, handlers: AnimationHandlers, context: *mut u8) {
    unsafe { declarations::animation_set_handlers(animation, handlers, context) }
}

pub fn animation_get_context<T>(animation: *const RawAnimation) -> *mut T {
    unsafe { declarations::animation_get_context(animation) as *mut T }
}

pub fn animation_set_implementation(animation: *mut RawAnimation, implementation: *const AnimationImplementation) {
    unsafe { declarations::animation_set_implementation(animation, implementation) }
}

pub fn status_bar_layer_create() -> *mut RawStatusBarLayer {
    unsafe { declarations::status_bar_layer_create() }
}

pub fn status_bar_layer_destroy(status_bar_layer: *mut RawStatusBarLayer) {
    unsafe { declarations::status_bar_layer_destroy(status_bar_layer) }
}

pub fn status_bar_layer_get_layer(status_bar_layer: *mut RawStatusBarLayer) -> *mut RawLayer {
    unsafe { declarations::status_bar_layer_get_layer(status_bar_layer) }
}

pub fn status_bar_layer_set_colors(status_bar_layer: *mut RawStatusBarLayer, background: GColor, foreground: GColor) {
    unsafe { declarations::status_bar_layer_set_colors(status_bar_layer, background, foreground) }
}

pub fn status_bar_layer_set_separator_mode(status_bar_layer: *mut RawStatusBarLayer, mode: StatusBarLayerSeparatorMode) {
    unsafe { declarations::status_bar_layer_set_separator_mode(status_bar_layer, mode) }
}

pub fn status_bar_layer_get_foreground_color(status_bar_layer: *mut RawStatusBarLayer) -> GColor {
    unsafe { declarations::status_bar_layer_get_foreground_color(status_bar_layer) }
}

pub fn status_bar_layer_get_background_color(status_bar_layer: *mut RawStatusBarLayer) -> GColor {
    unsafe { declarations::status_bar_layer_get_background_color(status_bar_layer) }
}

pub fn content_indicator_create() -> *mut RawContentIndicator {
    unsafe { declarations::content_indicator_create() }
}

pub fn content_indicator_destroy(content_indicator: *mut RawContentIndicator) {
    unsafe { declarations::content_indicator_destroy(content_indicator) }
}

pub fn content_indicator_configure_direction(content_indicator: *mut RawContentIndicator, direction: ContentIndicatorDirection, config: *const ContentIndicatorConfig) -> bool {
    unsafe { declarations::content_indicator_configure_direction(content_indicator, direction, config) }
}

pub fn content_indicator_set_content_available(content_indicator: *mut RawContentIndicator, direction: ContentIndicatorDirection, available: bool) -> bool {
    unsafe { declarations::content_indicator_set_content_available(content_indicator, direction, available) }
}

pub fn content_indicator_get_content_available(content_indicator: *mut RawContentIndicator, direction: ContentIndicatorDirection) -> bool {
    unsafe { declarations::content_indicator_get_content_available(content_indicator, direction) }
}
