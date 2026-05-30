/*
 * This file is part of pebble-rust.
 * Copyright (c) 2019 RoccoDev
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
 * General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

use alloc::ffi::CString;

use crate::pebble::internal::functions::declarations::*;
use crate::pebble::internal::types::{tm, TimeUnits};

static mut MINUTE_HANDLERS: alloc::vec::Vec<(*mut (), fn(*mut ()))> = alloc::vec::Vec::new();

extern "C" fn tick_trampoline(_t: *mut tm, _u: TimeUnits) {
    unsafe {
        for &(ctx, f) in &*core::ptr::addr_of!(MINUTE_HANDLERS) {
            f(ctx);
        }
    }
}

/// Subscribe to minute-tick events with a context pointer.
/// Multiple subscribers are supported; each receives its own context on every tick.
pub fn subscribe_minute<T>(context: *mut T, handler: fn(&mut T)) {
    unsafe {
        (*core::ptr::addr_of_mut!(MINUTE_HANDLERS))
            .push((context as *mut (), core::mem::transmute::<fn(&mut T), fn(*mut ())>(handler)));
        tick_timer_service_subscribe(TimeUnits::MINUTE_UNIT, tick_trampoline);
    }
}

/// Remove the subscription registered with the given context pointer.
/// Unregisters from the Pebble SDK when no subscribers remain.
pub fn unsubscribe_minute<T>(context: *mut T) {
    unsafe {
        let handlers = &mut *core::ptr::addr_of_mut!(MINUTE_HANDLERS);
        handlers.retain(|&(ctx, _)| ctx != context as *mut ());
        if handlers.is_empty() {
            tick_timer_service_unsubscribe();
        }
    }
}

/// Remove all minute-tick subscribers and unregister from the Pebble SDK.
pub fn unsubscribe_tick() {
    unsafe {
        (*core::ptr::addr_of_mut!(MINUTE_HANDLERS)).clear();
        tick_timer_service_unsubscribe();
    }
}

/// Returns the current time as a null-terminated C string (e.g. "12:59 AM\0").
/// Points into a static buffer — valid until the next call to this function.
pub fn current_time_cstr() -> CString {
    let mut time_buf: [u8; 9] = [0; 9];
    unsafe {
        clock_copy_time_string(time_buf.as_mut_ptr(), time_buf.len() as u8);
        CString::from_vec_unchecked(time_buf.to_vec())
    }
}

/// Write the current time string into `buf`. The buffer must be at least 9 bytes
/// ("12:59 AM\0"). On return the buffer holds a null-terminated string.
pub fn copy_time_string(buf: &mut [u8]) {
    unsafe {
        clock_copy_time_string(buf.as_mut_ptr(), buf.len() as u8);
    }
}

pub fn is_24h() -> bool {
    unsafe {
        clock_is_24h_style() != 0
    }
}

pub fn get_timezone() -> alloc::string::String {
    let mut buf = [0u8; 32];

    unsafe {
        clock_get_timezone(buf.as_mut_ptr(), 32);
        alloc::string::String::from_utf8_unchecked(buf.to_vec())
    }
}
