#![crate_type = "staticlib"]
#![no_std]
#![no_builtins]
extern crate alloc;
#[macro_use]
extern crate pebble_rust as pebble;

use pebble::{app, window, WindowPtr};
use pebble::app_message::{AppMessage, AppMessageDict};
use pebble::layer::{ILayer, TextLayer};
use pebble::types::{DictPtr, GPoint, GRect, GSize, VoidPtr};
use pebble::window::WindowHandlers;
use crate::pebble::std::ToCString;

const MESSAGE_KEY_EXAMPLE: u32 = 1768777472;

// THIS WORKS FOR THE DEMO BUT IS NOT GOOD PRACTICE
static mut TEXT_LAYER: Option<TextLayer> = None;

#[unsafe(no_mangle)]
pub fn main() -> isize {
    AppMessage::register_inbox(message_received);

    let app = app::App::new();
    let window = window::Window::new();
    window.set_handlers(WindowHandlers {
        load: load_handler,
        unload: unload_handler,
        appear: appear_handler,
        disappear: disappear_handler,
    });
    window.push(false);
    app.run_event_loop();
    window.clean_exit();

    pbl_log!("Exiting.");
    0
}

extern "C" fn message_received(dict_ptr: DictPtr, _ctx: VoidPtr) {
    let dict = AppMessageDict::from_raw(dict_ptr);
    if let Some(text) = dict.find_str(MESSAGE_KEY_EXAMPLE) {
        // Heap-allocate so the string outlives this callback frame
        let owned = alloc::string::String::from(text);
        unsafe {
            if let Some(layer) = &*core::ptr::addr_of!(TEXT_LAYER) {
                layer.set_text(&owned.to_cstring());
            }
        }
    }
}

extern "C" fn load_handler(window: WindowPtr) {
    pbl_log!("Window loaded at address %p", window);

    let window = window::Window::from_raw(window);
    let root = window.get_root_layer();
    let bounds = root.get_bounds();

    let bounds = GRect {
        origin: GPoint { x: bounds.size.w / 9, y: bounds.size.h / 2 - 20 },
        size: GSize { w: bounds.size.w, h: 20 },
    };

    let text = TextLayer::new(bounds);
    text.set_text(c"Loading...");
    root.add_child(&text);


    unsafe { TEXT_LAYER = Some(text); }

    AppMessage::open(200, 200);
}

extern "C" fn unload_handler(_window: WindowPtr) {}
extern "C" fn appear_handler(_window: WindowPtr) {}
extern "C" fn disappear_handler(_window: WindowPtr) {}
