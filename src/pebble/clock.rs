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

use alloc::boxed::Box;
use alloc::ffi::CString;
use alloc::vec::Vec;

use crate::pebble::internal::functions::declarations::*;
use crate::pebble::internal::functions::interface;
use crate::pebble::internal::types::{tm, TimeUnits};
use crate::pebble::std::time::TimeInfo;

// The Pebble tick service has a single global handler. This registry papers over
// that: many subscribers can coexist, the service is subscribed at the finest unit
// any of them wants, and each subscriber is filtered to the units it asked for.

struct Subscriber {
    id:      u32,
    units:   TimeUnits,
    handler: Box<dyn Fn(TimeInfo, u32)>,
}

static mut TICK_SUBSCRIBERS: Vec<Subscriber> = Vec::new();
static mut NEXT_TICK_ID: u32 = 0;

/// A live tick subscription. Dropping it unsubscribes; when the last subscription
/// is dropped, the underlying tick service is torn down.
pub struct TickSubscription {
    id: u32,
}

impl Drop for TickSubscription {
    fn drop(&mut self) {
        unsafe {
            let subs = &mut *core::ptr::addr_of_mut!(TICK_SUBSCRIBERS);
            subs.retain(|s| s.id != self.id);
            if subs.is_empty() {
                tick_timer_service_unsubscribe();
            } else {
                resubscribe();
            }
        }
    }
}

/// Subscribe to tick events at the given granularity. `handler` receives the tick
/// time and the bitmask of units that changed this tick. Hold onto the returned
/// `TickSubscription` for as long as you want ticks — dropping it unsubscribes.
pub fn subscribe(units: TimeUnits, handler: impl Fn(TimeInfo, u32) + 'static) -> TickSubscription {
    let id = unsafe {
        NEXT_TICK_ID = NEXT_TICK_ID.wrapping_add(1);
        NEXT_TICK_ID
    };
    unsafe {
        (*core::ptr::addr_of_mut!(TICK_SUBSCRIBERS)).push(Subscriber { id, units, handler: Box::new(handler) });
    }
    resubscribe();
    TickSubscription { id }
}

/// Convenience: subscribe to minute ticks.
pub fn subscribe_minute(handler: impl Fn(TimeInfo, u32) + 'static) -> TickSubscription {
    subscribe(TimeUnits::MINUTE_UNIT, handler)
}

// (Re)subscribe the C tick service at the finest (most frequent) unit any subscriber
// currently wants — the smallest bitmask value, since SECOND < MINUTE < HOUR < ….
fn resubscribe() {
    unsafe {
        let subs = &*core::ptr::addr_of!(TICK_SUBSCRIBERS);
        let finest = subs.iter().map(|s| s.units).min_by_key(|u| *u as u32);
        if let Some(finest) = finest {
            interface::tick_timer_service_subscribe(finest, tick_trampoline);
        }
    }
}

extern "C" fn tick_trampoline(t: *mut tm, units_changed: u32) {
    let info = TimeInfo::from_raw(unsafe { *t });
    unsafe {
        for s in &*core::ptr::addr_of!(TICK_SUBSCRIBERS) {
            // Fire only the subscribers whose requested unit rolled over this tick.
            if units_changed & (s.units as u32) != 0 {
                (s.handler)(info, units_changed);
            }
        }
    }
}

/// Returns the current time as a null-terminated C string (e.g. "12:59 AM").
pub fn current_time_cstr() -> CString {
    let mut buf: [u8; 9] = [0; 9];
    unsafe {
        clock_copy_time_string(buf.as_mut_ptr(), buf.len() as u8);
    }
    // The buffer is NUL-padded; keep only up to the first NUL so the CString has no
    // interior NUL bytes.
    let len = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
    unsafe { CString::from_vec_unchecked(buf[..len].to_vec()) }
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
