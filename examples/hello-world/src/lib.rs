#![crate_type = "staticlib"]
#![no_std]
#![no_builtins]

#[macro_use]
extern crate pebble_rust as pebble;

use pebble::{app, window, WindowPtr};
use pebble::window::WindowHandlers;
use pebble::layer::{ILayer, TextLayer};
use pebble::types::{GRect, GPoint, GSize};

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
    window.clean_exit();

    pbl_log!("Exiting...");
    0
}

extern "C" fn load_handler(window: WindowPtr) {
    let window = window::Window::from_raw(window);
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
    text.set_font(pebble::system::fonts::Font::get_system(pebble::system::fonts::FontKey::GOTHIC_24));
    root.add_child(&text);
}

extern "C" fn unload_handler(_window: WindowPtr) {}
extern "C" fn appear_handler(_window: WindowPtr) {}
extern "C" fn disappear_handler(_window: WindowPtr) {}
