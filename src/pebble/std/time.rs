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

//! Standard-C time (the libc `<time.h>` subset Pebble exposes). Pebble-specific
//! wall-clock helpers (24h style, timezone, tick subscriptions) live in `clock`.
//!
//! Formatting is delegated entirely to Pebble's `strftime` — `TimeInfo::format`
//! just removes the buffer/pointer boilerplate.

use alloc::string::String;
use core::ffi::CStr;

use crate::pebble::internal::functions::interface;
use crate::pebble::internal::types::{tm, time_t};

/// Current time as seconds since the Unix epoch.
pub fn time() -> time_t {
    interface::time()
}

/// Normalize `ti` and return it as a `time_t` (wraps C `mktime`).
pub fn mktime(ti: &mut TimeInfo) -> time_t {
    interface::mktime(&mut ti.0)
}

/// Broken-down time (a safe owner of C's `struct tm`). Build it from a `time_t`
/// with `from_local`/`from_utc`, then `format` it with a `strftime` pattern.
#[derive(Copy, Clone)]
pub struct TimeInfo(tm);

impl TimeInfo {
    /// Local-timezone broken-down time for the given timestamp (C `localtime`).
    pub fn from_local(t: time_t) -> TimeInfo {
        // localtime returns a pointer into a shared static buffer; copy out now.
        TimeInfo(unsafe { *interface::localtime(t) })
    }

    /// UTC broken-down time for the given timestamp (C `gmtime`).
    pub fn from_utc(t: time_t) -> TimeInfo {
        TimeInfo(unsafe { *interface::gmtime(t) })
    }

    /// Local-timezone broken-down time for *now*.
    pub fn now_local() -> TimeInfo {
        TimeInfo::from_local(time())
    }

    /// UTC broken-down time for *now*.
    pub fn now_utc() -> TimeInfo {
        TimeInfo::from_utc(time())
    }

    /// Wrap a raw C `struct tm` (e.g. the one handed to a tick handler).
    pub fn from_raw(raw: tm) -> TimeInfo {
        TimeInfo(raw)
    }

    /// The underlying C `struct tm`, if you need the raw broken-down fields.
    pub fn raw(&self) -> tm {
        self.0
    }

    /// Format with a `strftime` pattern (e.g. `c"%I:%M %p"`), letting Pebble do
    /// the locale-aware formatting. Output longer than 63 bytes is truncated.
    pub fn format(&self, fmt: &CStr) -> String {
        let mut buf = [0u8; 64];
        let len = interface::strftime(&mut buf, fmt, &self.0);
        unsafe { String::from_utf8_unchecked(buf[..len].to_vec()) }
    }
}