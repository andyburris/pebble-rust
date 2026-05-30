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
    pub unsafe fn window_create() -> *mut Window;
    pub unsafe fn window_destroy(window: *mut Window);
    pub unsafe fn window_set_click_config_provider(window: *mut Window, func: extern "C" fn(*mut c_void));
    pub unsafe fn window_set_click_config_provider_with_context(window: *mut Window, func: extern "C" fn(*mut u8), ctx: *mut u8);
    pub unsafe fn window_set_window_handlers(window: *mut Window, handlers: WindowHandlers);
    pub unsafe fn window_set_background_color(window: *mut Window, color: GColor);
    pub unsafe fn window_set_user_data(window: *mut Window, data: *mut c_void);
    pub unsafe fn window_get_user_data(window: *mut Window) -> *mut c_void;
    pub unsafe fn window_stack_push(window: *mut Window, animate: u8);
    pub unsafe fn window_get_root_layer(window: *mut Window) -> *mut Layer;
    pub unsafe fn window_single_click_subscribe(button: u8, func: extern "C" fn(*mut ClickRecognizer, *mut u8));

    // Layer
    pub unsafe fn layer_create(bounds: GRect) -> *mut Layer;
    pub unsafe fn layer_create_with_data(bounds: GRect, data_size: usize) -> *mut Layer;
    pub unsafe fn layer_get_data(layer: *const Layer) -> *mut c_void;
    pub unsafe fn layer_destroy(layer: *mut Layer);
    pub unsafe fn layer_get_frame(layer: *mut Layer) -> GRect;
    pub unsafe fn layer_get_bounds(layer: *mut Layer) -> GRect;
    pub unsafe fn layer_add_child(layer: *mut Layer, child: *mut Layer);
    pub unsafe fn layer_mark_dirty(layer: *mut Layer);
    pub unsafe fn layer_set_update_proc(layer: *mut Layer, func: extern "C" fn(*mut Layer, *mut GContext));
    pub unsafe fn layer_set_hidden(layer: *mut Layer, hidden: bool);

    // TextLayer
    pub unsafe fn text_layer_create(bounds: GRect) -> *mut TextLayer;
    pub unsafe fn text_layer_set_text(layer: *mut TextLayer, text: *const c_char);
    pub unsafe fn text_layer_get_layer(layer: *mut TextLayer) -> *mut Layer;
    pub unsafe fn text_layer_set_font(layer: *mut TextLayer, font: GFont);
    pub unsafe fn text_layer_set_background_color(layer: *mut TextLayer, color: GColor);
    pub unsafe fn text_layer_set_text_color(layer: *mut TextLayer, color: GColor);
    pub unsafe fn text_layer_set_text_alignment(layer: *mut TextLayer, alignment: GTextAlignment);

    // GBitmap
    pub unsafe fn gbitmap_create_with_resource(id: u32) -> *mut GBitmap;
    pub unsafe fn gbitmap_destroy(bitmap: *mut GBitmap);

    // BitmapLayer
    pub unsafe fn bitmap_layer_create(frame: GRect) -> *mut BitmapLayer;
    pub unsafe fn bitmap_layer_set_bitmap(layer: *mut BitmapLayer, bitmap: *mut GBitmap);
    pub unsafe fn bitmap_layer_set_compositing_mode(layer: *mut BitmapLayer, mode: GCompOp);
    pub unsafe fn bitmap_layer_get_layer(layer: *mut BitmapLayer) -> *mut Layer;

    // Graphics
    pub unsafe fn graphics_context_set_fill_color(ctx: *mut GContext, color: GColor);
    pub unsafe fn graphics_context_set_stroke_color(ctx: *mut GContext, color: GColor);
    pub unsafe fn graphics_context_set_stroke_width(ctx: *mut GContext, stroke_width: u8);
    pub unsafe fn graphics_fill_circle(ctx: *mut GContext, center: GPoint, radius: u16);
    pub unsafe fn graphics_fill_rect(ctx: *mut GContext, rect: GRect, corner_radius: u16, corner_mask: GCornerMask);
    pub unsafe fn graphics_draw_line(ctx: *mut GContext, p0: GPoint, p1: GPoint);

    // Trig
    pub unsafe fn sin_lookup(angle: i32) -> i32;
    pub unsafe fn cos_lookup(angle: i32) -> i32;

    // Wall Time
    pub unsafe fn clock_copy_time_string(buffer: *mut c_char, size: u8);
    pub unsafe fn clock_is_24h_style() -> u8;
    pub unsafe fn clock_get_timezone(buffer: *mut c_char, size: usize);

    pub unsafe fn tick_timer_service_subscribe(unit: TimeUnits, func: extern "C" fn(*mut tm, TimeUnits));
    pub unsafe fn tick_timer_service_unsubscribe();

    // Standard C - Time
    pub unsafe fn time(t: *mut usize) -> usize;
    pub unsafe fn localtime(now: *const usize) -> *mut tm;
    pub unsafe fn gmtime(now: *const usize) -> *mut tm;

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
    pub unsafe fn fonts_get_system_font(key: *const c_char) -> GFont;
    pub unsafe fn fonts_load_custom_font(res: ResHandle) -> GFont;

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

    // MenuLayer
    pub unsafe fn menu_layer_create(frame: GRect) -> *mut MenuLayer;
    pub unsafe fn menu_layer_destroy(menu_layer: *mut MenuLayer);
    pub unsafe fn menu_layer_get_layer(menu_layer: *mut MenuLayer) -> *mut Layer;
    pub unsafe fn menu_layer_set_callbacks(menu_layer: *mut MenuLayer, callback_context: *mut c_void, callbacks: MenuLayerCallbacks);
    pub unsafe fn menu_layer_set_click_config_onto_window(menu_layer: *mut MenuLayer, window: *mut Window);
    pub unsafe fn menu_layer_reload_data(menu_layer: *mut MenuLayer);
    pub unsafe fn menu_cell_basic_draw(ctx: *mut GContext, cell_layer: *const Layer, title: *const c_char, subtitle: *const c_char, icon: *mut GBitmap);
    pub unsafe fn menu_cell_basic_header_draw(ctx: *mut GContext, cell_layer: *const Layer, title: *const c_char);
}
