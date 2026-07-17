use alloc::boxed::Box;
use crate::pebble::internal::{types, functions::interface};
use crate::pebble::window::Window;

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ButtonId {
    Back   = 0,
    Up     = 1,
    Select = 2,
    Down   = 3,
}

/// A long-press configuration for one button.
pub struct LongClick {
    /// How long the button must be held before `pressed` fires, in ms.
    pub delay_ms: u16,
    /// Fires when the hold registers (button pressed past `delay_ms`).
    pub pressed:  Option<Box<dyn Fn()>>,
    /// Fires when the button is released after a long press.
    pub released: Option<Box<dyn Fn()>>,
}

/// A repeating-click configuration: `handler` fires every `interval_ms` while held.
pub struct Repeating {
    pub interval_ms: u16,
    pub handler:     Box<dyn Fn()>,
}

/// A multi-click configuration (e.g. double-click).
pub struct MultiClick {
    pub min_clicks:      u8,
    pub max_clicks:      u8,
    /// Time after the last click within which another click still counts, in ms.
    pub timeout_ms:      u16,
    /// If true, `handler` fires once on the last click; otherwise on each.
    pub last_click_only: bool,
    pub handler:         Box<dyn Fn()>,
}

/// The handlers for a single button. A plain `click` and a `repeating` handler both
/// configure the same recognizer; if both are set, `repeating` wins.
#[derive(Default)]
pub struct ButtonHandlers {
    pub click:     Option<Box<dyn Fn()>>,
    pub long:      Option<LongClick>,
    pub repeating: Option<Repeating>,
    pub multi:     Option<MultiClick>,
}

/// Click handlers for a window — one `ButtonHandlers` per button.
///
/// Each handler is a boxed closure that owns whatever state it captured, so there is
/// no shared context pointer to manage.
///
/// Note on `back`: subscribing `click` overrides the default "pop window" behavior
/// (fully supported). `long`/`multi` on Back register but are effectively
/// system-owned — the OS quits to the watchface on a long Back press regardless.
#[derive(Default)]
pub struct ClickCallbacks {
    pub up:     ButtonHandlers,
    pub select: ButtonHandlers,
    pub down:   ButtonHandlers,
    pub back:   ButtonHandlers,
}

struct ClickContext {
    callbacks: ClickCallbacks,
}

/// Keeps the click callbacks alive for the window's lifetime. Drop it and the
/// window's click handlers stop firing.
pub struct WindowClickHandler {
    // Raw (not Box) so no `Box` move can invalidate the pointer C holds — deriving a
    // pointer from a Box and then moving the Box is UB (Box is noalias).
    ctx: *mut ClickContext,
}

impl Drop for WindowClickHandler {
    fn drop(&mut self) {
        unsafe { drop(Box::from_raw(self.ctx)); }
    }
}

impl Window {
    pub fn set_click_handlers(&self, callbacks: ClickCallbacks) -> WindowClickHandler {
        let ctx = Box::into_raw(Box::new(ClickContext { callbacks }));
        interface::window_set_click_config_provider_with_context(
            self.raw(),
            click_config_trampoline,
            ctx,
        );
        WindowClickHandler { ctx }
    }
}

// The provider runs each time the window gains focus; it (re-)subscribes the C
// recognizers. Click handlers receive the same context pointer as the provider.
extern "C" fn click_config_trampoline(ctx: *mut ClickContext) {
    let c = unsafe { &*ctx };
    for button in [ButtonId::Up, ButtonId::Select, ButtonId::Down, ButtonId::Back] {
        configure_button(button as u8, c.callbacks.for_button(button as u32));
    }
}

impl ClickCallbacks {
    // Handlers for a raw C ButtonId value (Back 0, Up 1, Select 2, Down 3).
    fn for_button(&self, id: u32) -> &ButtonHandlers {
        match id {
            1 => &self.up,
            2 => &self.select,
            3 => &self.down,
            _ => &self.back,
        }
    }
}

#[inline(never)]
fn configure_button(button: u8, h: &ButtonHandlers) {
    // `repeating` and `click` configure the same recognizer; repeating supersedes.
    if let Some(r) = h.repeating.as_ref() {
        interface::window_single_repeating_click_subscribe(button, r.interval_ms, tramp_click);
    } else if h.click.is_some() {
        interface::window_single_click_subscribe(button, tramp_click);
    }
    if let Some(l) = h.long.as_ref() {
        let down = l.pressed.as_ref().map(|_| tramp_long_pressed as extern "C" fn(*mut types::ClickRecognizer, *mut u8));
        let up = l.released.as_ref().map(|_| tramp_long_released as extern "C" fn(*mut types::ClickRecognizer, *mut u8));
        interface::window_long_click_subscribe(button, l.delay_ms, down, up);
    }
    if let Some(m) = h.multi.as_ref() {
        interface::window_multi_click_subscribe(button, m.min_clicks, m.max_clicks, m.timeout_ms, m.last_click_only, tramp_multi);
    }
}

// One trampoline per click *kind*; the button comes from the recognizer, so every
// button shares them (instead of one trampoline per button × kind).
fn handlers<'a>(rec: *mut types::ClickRecognizer, ctx: *mut u8) -> &'a ButtonHandlers {
    let c = unsafe { &*(ctx as *const ClickContext) };
    c.callbacks.for_button(interface::click_recognizer_get_button_id(rec))
}

extern "C" fn tramp_click(rec: *mut types::ClickRecognizer, ctx: *mut u8) {
    // Shared by the single and repeating recognizers (same slot in the C API).
    let h = handlers(rec, ctx);
    if let Some(r) = h.repeating.as_ref() { (r.handler)(); }
    else if let Some(f) = h.click.as_ref() { f(); }
}
extern "C" fn tramp_long_pressed(rec: *mut types::ClickRecognizer, ctx: *mut u8) {
    if let Some(l) = handlers(rec, ctx).long.as_ref() { if let Some(f) = l.pressed.as_ref() { f(); } }
}
extern "C" fn tramp_long_released(rec: *mut types::ClickRecognizer, ctx: *mut u8) {
    if let Some(l) = handlers(rec, ctx).long.as_ref() { if let Some(f) = l.released.as_ref() { f(); } }
}
extern "C" fn tramp_multi(rec: *mut types::ClickRecognizer, ctx: *mut u8) {
    if let Some(m) = handlers(rec, ctx).multi.as_ref() { (m.handler)(); }
}
