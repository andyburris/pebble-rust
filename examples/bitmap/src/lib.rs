#![crate_type = "staticlib"]
#![no_std]
#![no_builtins]

extern crate pebble_rust as pebble;

use pebble::{app, window, WindowPtr};
use pebble::window::{WindowHandlers, WindowRef};
use pebble::layer::{AsLayer, BitmapLayer};
use pebble::types::{GCompOp, GBitmap};

// Keep the bitmap layer alive for the app's lifetime (it destroys its C layer on drop).
static mut BITMAP_LAYER: Option<BitmapLayer> = None;

#[unsafe(no_mangle)]
pub fn main() -> isize {
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
    0
}

extern "C" fn load_handler(window: WindowPtr) {
    let window = WindowRef::from_raw(window);
    let root = window.get_root_layer();
    let bounds = root.get_bounds();

    let bitmap = GBitmap::new(1);
    let bitmap_layer = BitmapLayer::new(bounds);
    bitmap_layer.set_bitmap(&bitmap);
    bitmap_layer.set_compositing_mode(GCompOp::GCompOpSet);

    root.add_child(&bitmap_layer);
    unsafe { BITMAP_LAYER = Some(bitmap_layer); }
}

extern "C" fn unload_handler(_window: WindowPtr) {}
extern "C" fn appear_handler(_window: WindowPtr) {}
extern "C" fn disappear_handler(_window: WindowPtr) {}
