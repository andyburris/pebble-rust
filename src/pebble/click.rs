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
    _ctx: Box<ClickContext>,
}

impl Window {
    pub fn set_click_handlers(&self, callbacks: ClickCallbacks) -> WindowClickHandler {
        let mut ctx = Box::new(ClickContext { callbacks });
        let ctx_ptr: *mut ClickContext = &mut *ctx;
        interface::window_set_click_config_provider_with_context(
            self.raw(),
            click_config_trampoline,
            ctx_ptr,
        );
        WindowClickHandler { _ctx: ctx }
    }
}

// The provider runs each time the window gains focus; it (re-)subscribes the C
// recognizers. Click handlers receive the same context pointer as the provider.
extern "C" fn click_config_trampoline(ctx: *mut ClickContext) {
    let c = unsafe { &*ctx };
    configure_button(ButtonId::Up as u8, &c.callbacks.up, tramp_up_click, tramp_up_rep, tramp_up_lp, tramp_up_lr, tramp_up_multi);
    configure_button(ButtonId::Select as u8, &c.callbacks.select, tramp_select_click, tramp_select_rep, tramp_select_lp, tramp_select_lr, tramp_select_multi);
    configure_button(ButtonId::Down as u8, &c.callbacks.down, tramp_down_click, tramp_down_rep, tramp_down_lp, tramp_down_lr, tramp_down_multi);
    configure_button(ButtonId::Back as u8, &c.callbacks.back, tramp_back_click, tramp_back_rep, tramp_back_lp, tramp_back_lr, tramp_back_multi);
}

type Trampoline = extern "C" fn(*mut types::ClickRecognizer, *mut u8);

fn configure_button(
    button: u8,
    h: &ButtonHandlers,
    click: Trampoline,
    repeating: Trampoline,
    long_pressed: Trampoline,
    long_released: Trampoline,
    multi: Trampoline,
) {
    // `repeating` and `click` configure the same recognizer; repeating supersedes.
    if let Some(r) = h.repeating.as_ref() {
        interface::window_single_repeating_click_subscribe(button, r.interval_ms, repeating);
    } else if h.click.is_some() {
        interface::window_single_click_subscribe(button, click);
    }
    if let Some(l) = h.long.as_ref() {
        let down = l.pressed.as_ref().map(|_| long_pressed);
        let up = l.released.as_ref().map(|_| long_released);
        interface::window_long_click_subscribe(button, l.delay_ms, down, up);
    }
    if let Some(m) = h.multi.as_ref() {
        interface::window_multi_click_subscribe(button, m.min_clicks, m.max_clicks, m.timeout_ms, m.last_click_only, multi);
    }
}

macro_rules! button_trampolines {
    ($click:ident, $rep:ident, $lp:ident, $lr:ident, $multi:ident, $field:ident) => {
        extern "C" fn $click(_: *mut types::ClickRecognizer, ctx: *mut u8) {
            let c = unsafe { &*(ctx as *const ClickContext) };
            if let Some(f) = c.callbacks.$field.click.as_ref() { f(); }
        }
        extern "C" fn $rep(_: *mut types::ClickRecognizer, ctx: *mut u8) {
            let c = unsafe { &*(ctx as *const ClickContext) };
            if let Some(r) = c.callbacks.$field.repeating.as_ref() { (r.handler)(); }
        }
        extern "C" fn $lp(_: *mut types::ClickRecognizer, ctx: *mut u8) {
            let c = unsafe { &*(ctx as *const ClickContext) };
            if let Some(l) = c.callbacks.$field.long.as_ref() { if let Some(f) = l.pressed.as_ref() { f(); } }
        }
        extern "C" fn $lr(_: *mut types::ClickRecognizer, ctx: *mut u8) {
            let c = unsafe { &*(ctx as *const ClickContext) };
            if let Some(l) = c.callbacks.$field.long.as_ref() { if let Some(f) = l.released.as_ref() { f(); } }
        }
        extern "C" fn $multi(_: *mut types::ClickRecognizer, ctx: *mut u8) {
            let c = unsafe { &*(ctx as *const ClickContext) };
            if let Some(m) = c.callbacks.$field.multi.as_ref() { (m.handler)(); }
        }
    };
}

button_trampolines!(tramp_up_click,     tramp_up_rep,     tramp_up_lp,     tramp_up_lr,     tramp_up_multi,     up);
button_trampolines!(tramp_select_click, tramp_select_rep, tramp_select_lp, tramp_select_lr, tramp_select_multi, select);
button_trampolines!(tramp_down_click,   tramp_down_rep,   tramp_down_lp,   tramp_down_lr,   tramp_down_multi,   down);
button_trampolines!(tramp_back_click,   tramp_back_rep,   tramp_back_lp,   tramp_back_lr,   tramp_back_multi,   back);
