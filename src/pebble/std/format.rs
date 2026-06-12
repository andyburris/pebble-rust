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

pub use crate::pebble::internal::functions::declarations::snprintf;

// ── Numeric helpers ───────────────────────────────────────────────────────────

// Append an i32 to a String without fmt machinery (alloc::format! crashes at
// this call depth on Pebble due to stack usage in core::fmt).
pub fn push_i32(s: &mut alloc::string::String, mut n: i32) {
    if n == 0 { s.push_str("0"); return; }
    if n < 0 { s.push_str("-"); n = if n == i32::MIN { i32::MAX } else { -n }; }
    let mut buf = [0u8; 10];
    let mut len = 0usize;
    while n > 0 { buf[len] = (n % 10) as u8 + b'0'; len += 1; n /= 10; }
    buf[..len].reverse();
    s.push_str(unsafe { core::str::from_utf8_unchecked(&buf[..len]) });
}

pub fn push_u32(s: &mut alloc::string::String, mut n: u32) {
    if n == 0 { s.push('0'); return; }
    let mut buf = [0u8; 10];
    let mut len = 0usize;
    while n > 0 { buf[len] = (n % 10) as u8 + b'0'; len += 1; n /= 10; }
    buf[..len].reverse();
    s.push_str(unsafe { core::str::from_utf8_unchecked(&buf[..len]) });
}

// ── PblDisplay ────────────────────────────────────────────────────────────────

/// Lightweight alternative to `core::fmt::Display` for use with `pbl_format!`.
/// Only `{}` substitution is supported; no format specifiers.
pub trait PblDisplay {
    fn pbl_fmt(&self, out: &mut alloc::string::String);
}

impl PblDisplay for i8  { fn pbl_fmt(&self, o: &mut alloc::string::String) { push_i32(o, *self as i32); } }
impl PblDisplay for i16 { fn pbl_fmt(&self, o: &mut alloc::string::String) { push_i32(o, *self as i32); } }
impl PblDisplay for i32 { fn pbl_fmt(&self, o: &mut alloc::string::String) { push_i32(o, *self); } }
impl PblDisplay for u8  { fn pbl_fmt(&self, o: &mut alloc::string::String) { push_u32(o, *self as u32); } }
impl PblDisplay for u16 { fn pbl_fmt(&self, o: &mut alloc::string::String) { push_u32(o, *self as u32); } }
impl PblDisplay for u32 { fn pbl_fmt(&self, o: &mut alloc::string::String) { push_u32(o, *self); } }
impl PblDisplay for usize { fn pbl_fmt(&self, o: &mut alloc::string::String) { push_u32(o, *self as u32); } }
impl PblDisplay for bool { fn pbl_fmt(&self, o: &mut alloc::string::String) { o.push_str(if *self { "true" } else { "false" }); } }
impl PblDisplay for str  { fn pbl_fmt(&self, o: &mut alloc::string::String) { o.push_str(self); } }
impl PblDisplay for alloc::string::String { fn pbl_fmt(&self, o: &mut alloc::string::String) { o.push_str(self); } }
impl PblDisplay for alloc::ffi::CString {
    fn pbl_fmt(&self, o: &mut alloc::string::String) {
        o.push_str(unsafe { core::str::from_utf8_unchecked(self.as_bytes()) });
    }
}

// ── pbl_format! ───────────────────────────────────────────────────────────────

#[doc(hidden)]
pub fn pbl_format_impl(fmt: &str, args: &[&dyn PblDisplay]) -> alloc::string::String {
    let mut out = alloc::string::String::new();
    let bytes = fmt.as_bytes();
    let mut start = 0usize;
    let mut i = 0usize;
    let mut arg_idx = 0usize;
    while i < bytes.len() {
        if i + 1 < bytes.len() && bytes[i] == b'{' && bytes[i + 1] == b'}' {
            // Safety: `{` and `}` are ASCII, so `i` and `i+2` are always char
            // boundaries in valid UTF-8 format strings.
            out.push_str(unsafe { fmt.get_unchecked(start..i) });
            if arg_idx < args.len() {
                args[arg_idx].pbl_fmt(&mut out);
                arg_idx += 1;
            }
            i += 2;
            start = i;
        } else {
            i += 1;
        }
    }
    out.push_str(unsafe { fmt.get_unchecked(start..) });
    out
}

/// Like `format!`, but uses lightweight `PblDisplay` instead of `core::fmt`
/// to avoid the stack overflow that `format!` causes deep in Pebble callbacks.
/// Only `{}` placeholders are supported.
#[macro_export]
macro_rules! pbl_format {
    ($fmt:literal) => {
        alloc::string::String::from($fmt)
    };
    ($fmt:literal, $($arg:expr),+ $(,)?) => {
        $crate::std::format::pbl_format_impl($fmt, &[$(&$arg as &dyn $crate::std::format::PblDisplay),+])
    };
}