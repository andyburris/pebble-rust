#![crate_type = "staticlib"]
#![no_std]
#![no_builtins]

#[macro_use]
extern crate pebble_rust as pebble;

use pebble::{app, window, WindowPtr};
use pebble::window::{WindowHandlers, WindowRef};
use pebble::layer::{AsLayer, TextLayer};
use pebble::types::{GRect, GPoint, GSize};

// Keep the text layer alive for the app's lifetime (layer wrappers destroy their C
// layer on drop). A static is the minimal way to demonstrate this in an example.
static mut TEXT_LAYER: Option<TextLayer> = None;

#[unsafe(no_mangle)]
pub fn main() -> isize {
    pbl_log!("Loading app...");

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
    // `window` is destroyed when it drops at the end of main.

    pbl_log!("Exiting...");
    0
}

extern "C" fn load_handler(window: WindowPtr) {
    let window = WindowRef::from_raw(window);
    let root = window.get_root_layer();
    let bounds = root.get_bounds();

    let bounds = GRect {
        origin: GPoint { x: bounds.size.w / 4, y: bounds.size.h / 2 - 20 },
        size: GSize { w: bounds.size.w, h: 20 },
    };

    // pbl_log! / pbl_warn! / pbl_err! work like printf — format args are C-style.
    // Wrap string args in nt!() to null-terminate them.
    pbl_log!("This works like a %s, I can print numbers like %d", nt!("printf").as_ptr(), 25);
    pbl_warn!("This is a warning.");
    pbl_err!("Oops, something went wrong.");

    let text = TextLayer::new(bounds);
    text.set_text(c"Hello from Rust!");
    text.set_font(pebble::system::fonts::GFont::get_system(pebble::system::fonts::FontKey::GOTHIC_24));
    root.add_child(&text);
    unsafe { TEXT_LAYER = Some(text); }
}

extern "C" fn unload_handler(_window: WindowPtr) {}
extern "C" fn appear_handler(_window: WindowPtr) {}
extern "C" fn disappear_handler(_window: WindowPtr) {}
