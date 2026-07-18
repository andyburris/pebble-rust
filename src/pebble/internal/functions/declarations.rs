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
use crate::pebble::internal::types::*;

unsafe extern "C" {
    // App
    pub unsafe fn app_event_loop();

    // Window
    pub unsafe fn window_create() -> *mut RawWindow;
    pub unsafe fn window_destroy(window: *mut RawWindow);
    pub unsafe fn window_set_click_config_provider(window: *mut RawWindow, func: extern "C" fn(*mut c_void));
    pub unsafe fn window_set_click_config_provider_with_context(window: *mut RawWindow, func: extern "C" fn(*mut u8), ctx: *mut u8);
    pub unsafe fn window_set_window_handlers(window: *mut RawWindow, handlers: WindowHandlers);
    pub unsafe fn window_set_background_color(window: *mut RawWindow, color: GColor);
    pub unsafe fn window_set_user_data(window: *mut RawWindow, data: *mut c_void);
    pub unsafe fn window_get_user_data(window: *mut RawWindow) -> *mut c_void;
    pub unsafe fn window_stack_push(window: *mut RawWindow, animate: u8);
    pub unsafe fn window_get_root_layer(window: *mut RawWindow) -> *mut RawLayer;
    pub unsafe fn window_single_click_subscribe(button: u8, func: extern "C" fn(*mut ClickRecognizer, *mut u8));
    pub unsafe fn window_single_repeating_click_subscribe(button: u8, repeat_interval_ms: u16, func: extern "C" fn(*mut ClickRecognizer, *mut u8));
    pub unsafe fn window_long_click_subscribe(button: u8, delay_ms: u16, down_handler: Option<extern "C" fn(*mut ClickRecognizer, *mut u8)>, up_handler: Option<extern "C" fn(*mut ClickRecognizer, *mut u8)>);
    pub unsafe fn window_multi_click_subscribe(button: u8, min_clicks: u8, max_clicks: u8, timeout: u16, last_click_only: bool, func: extern "C" fn(*mut ClickRecognizer, *mut u8));
    // Returns the C `ButtonId` enum (int-sized on ARM EABI).
    pub unsafe fn click_recognizer_get_button_id(recognizer: *mut ClickRecognizer) -> u32;

    // RawLayer
    pub unsafe fn layer_create(bounds: GRect) -> *mut RawLayer;
    pub unsafe fn layer_create_with_data(bounds: GRect, data_size: usize) -> *mut RawLayer;
    pub unsafe fn layer_get_data(layer: *const RawLayer) -> *mut c_void;
    pub unsafe fn layer_destroy(layer: *mut RawLayer);
    pub unsafe fn layer_get_frame(layer: *mut RawLayer) -> GRect;
    pub unsafe fn layer_set_frame(layer: *mut RawLayer, frame: GRect);
    pub unsafe fn layer_get_bounds(layer: *mut RawLayer) -> GRect;
    pub unsafe fn layer_add_child(layer: *mut RawLayer, child: *mut RawLayer);
    pub unsafe fn layer_remove_from_parent(layer: *mut RawLayer);
    pub unsafe fn layer_mark_dirty(layer: *mut RawLayer);
    pub unsafe fn layer_set_update_proc(layer: *mut RawLayer, func: extern "C" fn(*mut RawLayer, *mut GContext));
    pub unsafe fn layer_set_hidden(layer: *mut RawLayer, hidden: bool);

    // RawTextLayer
    pub unsafe fn text_layer_create(bounds: GRect) -> *mut RawTextLayer;
    pub unsafe fn text_layer_destroy(layer: *mut RawTextLayer);
    pub unsafe fn text_layer_set_text(layer: *mut RawTextLayer, text: *const c_char);
    pub unsafe fn text_layer_get_layer(layer: *mut RawTextLayer) -> *mut RawLayer;
    pub unsafe fn text_layer_set_font(layer: *mut RawTextLayer, font: RawGFont);
    pub unsafe fn text_layer_set_background_color(layer: *mut RawTextLayer, color: GColor);
    pub unsafe fn text_layer_set_text_color(layer: *mut RawTextLayer, color: GColor);
    pub unsafe fn text_layer_set_text_alignment(layer: *mut RawTextLayer, alignment: GTextAlignment);

    // RawGBitmap
    pub unsafe fn gbitmap_create_with_resource(id: u32) -> *mut RawGBitmap;
    pub unsafe fn gbitmap_destroy(bitmap: *mut RawGBitmap);

    // RawBitmapLayer
    pub unsafe fn bitmap_layer_create(frame: GRect) -> *mut RawBitmapLayer;
    pub unsafe fn bitmap_layer_destroy(layer: *mut RawBitmapLayer);
    pub unsafe fn bitmap_layer_set_bitmap(layer: *mut RawBitmapLayer, bitmap: *mut RawGBitmap);
    pub unsafe fn bitmap_layer_set_compositing_mode(layer: *mut RawBitmapLayer, mode: GCompOp);
    pub unsafe fn bitmap_layer_get_layer(layer: *mut RawBitmapLayer) -> *mut RawLayer;

    // Graphics
    pub unsafe fn graphics_context_set_fill_color(ctx: *mut GContext, color: GColor);
    pub unsafe fn graphics_context_set_text_color(ctx: *mut GContext, color: GColor);
    pub unsafe fn graphics_context_set_stroke_color(ctx: *mut GContext, color: GColor);
    pub unsafe fn graphics_context_set_stroke_width(ctx: *mut GContext, stroke_width: u8);
    pub unsafe fn graphics_context_set_compositing_mode(ctx: *mut GContext, mode: GCompOp);
    pub unsafe fn graphics_draw_bitmap_in_rect(ctx: *mut GContext, bitmap: *const RawGBitmap, dest_rect: GRect);
    pub unsafe fn graphics_fill_circle(ctx: *mut GContext, center: GPoint, radius: u16);
    pub unsafe fn graphics_fill_rect(ctx: *mut GContext, rect: GRect, corner_radius: u16, corner_mask: GCornerMask);
    pub unsafe fn graphics_draw_rect(ctx: *mut GContext, rect: GRect);
    pub unsafe fn graphics_draw_line(ctx: *mut GContext, p0: GPoint, p1: GPoint);
    pub unsafe fn graphics_draw_text(ctx: *mut GContext, text: *const c_char, font: RawGFont, rect: GRect, overflow: GTextOverflowMode, alignment: GTextAlignment, text_attributes: *mut c_void);
    pub unsafe fn graphics_text_layout_get_content_size(text: *const c_char, font: RawGFont, rect: GRect, overflow: GTextOverflowMode, alignment: GTextAlignment) -> GSize;

    // GRect / GPoint / GSize geometry helpers
    pub unsafe fn gpoint_equal(point_a: *const GPoint, point_b: *const GPoint) -> bool;
    pub unsafe fn gsize_equal(size_a: *const GSize, size_b: *const GSize) -> bool;
    pub unsafe fn grect_equal(rect_a: *const GRect, rect_b: *const GRect) -> bool;
    pub unsafe fn grect_is_empty(rect: *const GRect) -> bool;
    pub unsafe fn grect_standardize(rect: *mut GRect);
    pub unsafe fn grect_clip(rect_to_clip: *mut GRect, rect_clipper: *const GRect);
    pub unsafe fn grect_contains_point(rect: *const GRect, point: *const GPoint) -> bool;
    pub unsafe fn grect_center_point(rect: *const GRect) -> GPoint;
    pub unsafe fn grect_crop(rect: GRect, crop_size_px: i32) -> GRect;
    pub unsafe fn grect_inset(rect: GRect, insets: GEdgeInsets) -> GRect;
    pub unsafe fn grect_align(rect: *mut GRect, inside_rect: *const GRect, alignment: GAlign, clip: bool);

    // GPath
    pub unsafe fn gpath_create(init: *const GPathInfo) -> *mut RawGPath;
    pub unsafe fn gpath_destroy(path: *mut RawGPath);
    pub unsafe fn gpath_draw_filled(ctx: *mut GContext, path: *const RawGPath);
    pub unsafe fn gpath_draw_outline(ctx: *mut GContext, path: *const RawGPath);
    pub unsafe fn gpath_draw_outline_open(ctx: *mut GContext, path: *const RawGPath);
    pub unsafe fn gpath_move_to(path: *mut RawGPath, point: GPoint);
    pub unsafe fn gpath_rotate_to(path: *mut RawGPath, angle: i32);

    // Trig
    pub unsafe fn sin_lookup(angle: i32) -> i32;
    pub unsafe fn cos_lookup(angle: i32) -> i32;

    // Wall Time
    pub unsafe fn clock_copy_time_string(buffer: *mut c_char, size: u8);
    pub unsafe fn clock_is_24h_style() -> u8;
    pub unsafe fn clock_get_timezone(buffer: *mut c_char, size: usize);

    // `units_changed` is a bitmask (may combine units), so it's taken as a raw u32
    // rather than the `TimeUnits` enum to avoid an invalid-discriminant on the Rust side.
    pub unsafe fn tick_timer_service_subscribe(unit: TimeUnits, func: extern "C" fn(*mut tm, u32));
    pub unsafe fn tick_timer_service_unsubscribe();

    // Standard C - Time
    pub unsafe fn time(t: *mut time_t) -> time_t;
    pub unsafe fn localtime(now: *const time_t) -> *mut tm;
    pub unsafe fn gmtime(now: *const time_t) -> *mut tm;
    pub unsafe fn mktime(tb: *mut tm) -> time_t;
    pub unsafe fn strftime(s: *mut c_char, maxsize: usize, format: *const c_char, tm_p: *const tm) -> usize;

    // Standard C - Locale
    pub unsafe fn setlocale(category: i32, locale: *const c_char) -> *const c_char;

    // Standard C - Math
    pub unsafe fn rand() -> i32;
    pub unsafe fn srand(seed: u32) -> i32;

    // Standard C - Strings
    pub unsafe fn strcmp(str1: *const c_char, str2: *const c_char) -> i32;
    pub unsafe fn strncmp(str1: *const c_char, str2: *const c_char, num_bytes: usize) -> i32;
    pub unsafe fn strcpy(destination: *const c_char, source: *const c_char) -> *const c_char;
    pub unsafe fn strncpy(destination: *const c_char, source: *const c_char, num_bytes: usize) -> *const c_char;
    pub unsafe fn strcat(destination: *const c_char, source: *const c_char) -> *const c_char;
    pub unsafe fn strncat(destination: *const c_char, source: *const c_char, num_bytes: usize) -> *const c_char;
    pub unsafe fn strlen(str: *const c_char) -> usize;

    // Standard C - Format
    pub unsafe fn snprintf(buf: *const c_char, max: usize, fmt: *const c_char, ...) -> usize;

    // Fonts
    pub unsafe fn fonts_get_system_font(key: *const c_char) -> RawGFont;
    pub unsafe fn fonts_load_custom_font(res: ResHandle) -> RawGFont;

    // Resources
    pub unsafe fn resource_get_handle(id: u32) -> ResHandle;

    // Dictionary
    pub unsafe fn dict_calc_buffer_size(tuple_count: u8) -> u32;
    pub unsafe fn dict_size(iter: *mut DictionaryIterator) -> u32;
    pub unsafe fn dict_write_begin(iter: *mut DictionaryIterator, buffer: *mut u8, size: u16) -> DictionaryResult;
    pub unsafe fn dict_write_data(iter: *mut DictionaryIterator, key: u32, data: *mut u8, size: u16) -> DictionaryResult;
    pub unsafe fn dict_write_cstring(iter: *mut DictionaryIterator, key: u32, cstring: *const c_char) -> DictionaryResult;
    pub unsafe fn dict_write_int(iter: *mut DictionaryIterator, key: u32, int: *const c_void,
                            len_bytes: u8, signed: bool) -> DictionaryResult;
    pub unsafe fn dict_write_end(iter: *mut DictionaryIterator) -> u32;
    pub unsafe fn dict_read_begin_from_buffer(iter: *mut DictionaryIterator, buffer: *mut u8, size: u16) -> *mut Tuple;
    pub unsafe fn dict_read_next(iter: *mut DictionaryIterator) -> *mut Tuple;
    pub unsafe fn dict_read_first(iter: *mut DictionaryIterator) -> *mut Tuple;
    pub unsafe fn dict_find(iter: *mut DictionaryIterator, key: u32) -> *mut Tuple;

    // AppMessage
    pub unsafe fn app_message_open(size_in: u32, size_out: u32);
    pub unsafe fn app_message_register_inbox_received(callback: extern "C" fn(iter: *mut DictionaryIterator,
        ctx: *const c_void));
    pub unsafe fn app_message_register_outbox_sent(callback: extern "C" fn(iter: *mut DictionaryIterator,
                                                                   ctx: *const c_void));
    pub unsafe fn app_message_register_inbox_dropped(callback: extern "C" fn(reason: AppMessageResult, ctx: *const c_void));
    pub unsafe fn app_message_outbox_begin(iter: *mut *mut DictionaryIterator);
    pub unsafe fn app_message_outbox_send();

    // EVENTS
    // Battery
    pub unsafe fn battery_state_service_subscribe(handler: extern "C" fn(state: BatteryChargeState));
    pub unsafe fn battery_state_service_unsubscribe();
    pub unsafe fn battery_state_service_peek() -> BatteryChargeState;

    // Connection
    pub unsafe fn connection_service_peek_pebble_app_connection() -> bool;
    pub unsafe fn connection_service_peek_pebblekit_connection() -> bool;
    pub unsafe fn connection_service_unsubscribe();
    pub unsafe fn connection_service_subscribe(handlers: ConnectionHandlers);

    // Logging
    pub unsafe fn app_log(level: u8, filename: *const c_char, line_num: u32, msg: *const c_char, ...);

    // RawMenuLayer
    pub unsafe fn menu_layer_create(frame: GRect) -> *mut RawMenuLayer;
    pub unsafe fn menu_layer_destroy(menu_layer: *mut RawMenuLayer);
    pub unsafe fn menu_layer_get_layer(menu_layer: *mut RawMenuLayer) -> *mut RawLayer;
    pub unsafe fn menu_layer_set_callbacks(menu_layer: *mut RawMenuLayer, callback_context: *mut c_void, callbacks: MenuLayerCallbacks);
    pub unsafe fn menu_layer_set_click_config_onto_window(menu_layer: *mut RawMenuLayer, window: *mut RawWindow);
    pub unsafe fn menu_layer_set_highlight_colors(menu_layer: *mut RawMenuLayer, background: GColor, foreground: GColor);
    pub unsafe fn menu_layer_set_normal_colors(menu_layer: *mut RawMenuLayer, background: GColor, foreground: GColor);
    pub unsafe fn menu_layer_reload_data(menu_layer: *mut RawMenuLayer);
    pub unsafe fn menu_layer_get_selected_index(menu_layer: *mut RawMenuLayer) -> MenuIndex;
    pub unsafe fn menu_layer_is_index_selected(menu_layer: *mut RawMenuLayer, index: *const MenuIndex) -> bool;
    pub unsafe fn menu_layer_get_center_focused(menu_layer: *mut RawMenuLayer) -> bool;
    pub unsafe fn menu_layer_set_center_focused(menu_layer: *mut RawMenuLayer, center_focused: bool);
    pub unsafe fn menu_layer_set_selected_index(menu_layer: *mut RawMenuLayer, index: MenuIndex, scroll_align: MenuRowAlign, animated: bool);
    pub unsafe fn menu_layer_set_selected_next(menu_layer: *mut RawMenuLayer, up: bool, scroll_align: MenuRowAlign, animated: bool);
    pub unsafe fn menu_layer_pad_bottom_enable(menu_layer: *mut RawMenuLayer, enable: bool);
    pub unsafe fn menu_cell_basic_draw(ctx: *mut GContext, cell_layer: *const RawLayer, title: *const c_char, subtitle: *const c_char, icon: *mut RawGBitmap);
    pub unsafe fn menu_cell_basic_header_draw(ctx: *mut GContext, cell_layer: *const RawLayer, title: *const c_char);
    pub unsafe fn menu_cell_title_draw(ctx: *mut GContext, cell_layer: *const RawLayer, title: *const c_char);
    pub unsafe fn menu_cell_layer_is_highlighted(cell_layer: *const RawLayer) -> bool;

    // RawAnimation
    pub unsafe fn animation_create() -> *mut RawAnimation;
    pub unsafe fn animation_destroy(animation: *mut RawAnimation) -> bool;
    pub unsafe fn animation_schedule(animation: *mut RawAnimation) -> bool;
    pub unsafe fn animation_unschedule(animation: *mut RawAnimation) -> bool;
    pub unsafe fn animation_set_duration(animation: *mut RawAnimation, duration_ms: u32);
    pub unsafe fn animation_set_delay(animation: *mut RawAnimation, delay_ms: u32);
    pub unsafe fn animation_set_play_count(animation: *mut RawAnimation, play_count: u32);
    pub unsafe fn animation_set_reverse(animation: *mut RawAnimation, reverse: bool);
    pub unsafe fn animation_set_curve(animation: *mut RawAnimation, curve: AnimationCurve);
    pub unsafe fn animation_set_handlers(animation: *mut RawAnimation, handlers: AnimationHandlers, context: *mut u8);
    pub unsafe fn animation_get_context(animation: *const RawAnimation) -> *mut u8;
    pub unsafe fn animation_set_implementation(animation: *mut RawAnimation, implementation: *const AnimationImplementation);

    // RawStatusBarLayer
    pub unsafe fn status_bar_layer_create() -> *mut RawStatusBarLayer;
    pub unsafe fn status_bar_layer_destroy(status_bar_layer: *mut RawStatusBarLayer);
    pub unsafe fn status_bar_layer_get_layer(status_bar_layer: *mut RawStatusBarLayer) -> *mut RawLayer;
    pub unsafe fn status_bar_layer_set_colors(status_bar_layer: *mut RawStatusBarLayer, background: GColor, foreground: GColor);
    pub unsafe fn status_bar_layer_set_separator_mode(status_bar_layer: *mut RawStatusBarLayer, mode: StatusBarLayerSeparatorMode);
    pub unsafe fn status_bar_layer_get_foreground_color(status_bar_layer: *mut RawStatusBarLayer) -> GColor;
    pub unsafe fn status_bar_layer_get_background_color(status_bar_layer: *mut RawStatusBarLayer) -> GColor;

    // RawContentIndicator
    pub unsafe fn content_indicator_create() -> *mut RawContentIndicator;
    pub unsafe fn content_indicator_destroy(content_indicator: *mut RawContentIndicator);
    pub unsafe fn content_indicator_configure_direction(content_indicator: *mut RawContentIndicator, direction: ContentIndicatorDirection, config: *const ContentIndicatorConfig) -> bool;
    pub unsafe fn content_indicator_set_content_available(content_indicator: *mut RawContentIndicator, direction: ContentIndicatorDirection, available: bool) -> bool;
    pub unsafe fn content_indicator_get_content_available(content_indicator: *mut RawContentIndicator, direction: ContentIndicatorDirection) -> bool;
}
