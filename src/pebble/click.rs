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

pub struct ClickCallbacks<T> {
    pub up:          Option<fn(&mut T)>,
    pub select:      Option<fn(&mut T)>,
    pub down:        Option<fn(&mut T)>,
    pub back:        Option<fn(&mut T)>,
    pub up_long:     Option<fn(&mut T)>,
    pub select_long: Option<fn(&mut T)>,
    pub down_long:   Option<fn(&mut T)>,
}

impl<T> Default for ClickCallbacks<T> {
    fn default() -> Self {
        ClickCallbacks {
            up: None, select: None, down: None, back: None,
            up_long: None, select_long: None, down_long: None,
        }
    }
}

struct ClickContext<T> {
    user_ctx:  *mut T,
    callbacks: ClickCallbacks<T>,
}

pub struct WindowClickHandler<T> {
    _ctx: Box<ClickContext<T>>,
}

impl Window {
    pub fn set_click_handlers<T>(&self, context: *mut T, callbacks: ClickCallbacks<T>) -> WindowClickHandler<T> {
        let ctx = Box::new(ClickContext { user_ctx: context, callbacks });
        let ctx_ptr = Box::into_raw(ctx);
        interface::window_set_click_config_provider_with_context(
            self.raw(),
            click_config_trampoline::<T>,
            ctx_ptr,
        );
        WindowClickHandler { _ctx: unsafe { Box::from_raw(ctx_ptr) } }
    }
}

extern "C" fn click_config_trampoline<T>(ctx: *mut ClickContext<T>) {
    let cbs = unsafe { &(*ctx).callbacks };
    if cbs.up.is_some() {
        interface::window_single_click_subscribe(ButtonId::Up as u8, trampoline_up::<T>);
    }
    if cbs.select.is_some() {
        interface::window_single_click_subscribe(ButtonId::Select as u8, trampoline_select::<T>);
    }
    if cbs.down.is_some() {
        interface::window_single_click_subscribe(ButtonId::Down as u8, trampoline_down::<T>);
    }
    if cbs.back.is_some() {
        interface::window_single_click_subscribe(ButtonId::Back as u8, trampoline_back::<T>);
    }
    if cbs.up_long.is_some() {
        interface::window_long_click_subscribe(ButtonId::Up as u8, trampoline_up_long::<T>);
    }
    if cbs.select_long.is_some() {
        interface::window_long_click_subscribe(ButtonId::Select as u8, trampoline_select_long::<T>);
    }
    if cbs.down_long.is_some() {
        interface::window_long_click_subscribe(ButtonId::Down as u8, trampoline_down_long::<T>);
    }
}

macro_rules! click_trampoline {
    ($name:ident, $field:ident) => {
        extern "C" fn $name<T>(_: *mut types::ClickRecognizer, ctx: *mut ClickContext<T>) {
            let ctx = unsafe { &mut *ctx };
            if let Some(f) = ctx.callbacks.$field {
                f(unsafe { &mut *ctx.user_ctx });
            }
        }
    };
}

click_trampoline!(trampoline_up,          up);
click_trampoline!(trampoline_select,      select);
click_trampoline!(trampoline_down,        down);
click_trampoline!(trampoline_back,        back);
click_trampoline!(trampoline_up_long,     up_long);
click_trampoline!(trampoline_select_long, select_long);
click_trampoline!(trampoline_down_long,   down_long);
