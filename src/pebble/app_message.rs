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

pub use crate::pebble::internal::types::Tuple;

use crate::pebble::internal::types::{self, DictionaryIterator, c_void, AppMessageResult};
use crate::pebble::internal::functions::declarations::*;
use crate::pebble::types::{DictPtr, VoidPtr};

/// Represents a `DictionaryIterator`, essentially a list of `Tuple`s.
/// # Usage
/// ```ignore
/// use pebble_rust::app_message::{AppMessage, Dictionary};
///
/// let mut dictionary = Dictionary::new();
/// AppMessage::init_write(&mut dictionary);   // firmware fills in the iterator
/// dictionary.write_int(0, &2i32, 4, true);
/// AppMessage::send();
/// ```
pub struct Dictionary {
    internal: *mut DictionaryIterator
}

const NULL_TUPLE: *mut Tuple = 0 as *mut Tuple;

impl Dictionary {
    pub fn new() -> Self {
        // Starts null; `AppMessage::init_write` points it at the firmware-owned
        // iterator returned through `app_message_outbox_begin`.
        Self {
            internal: core::ptr::null_mut()
        }
    }

    /// Fetches the underlying dictionary from a raw pointer.
    pub fn from_raw(raw: DictPtr) -> Self {
        Self {
            internal: raw
        }
    }

    /// Prepares the dictionary for reading.
    /// Calling this is **required** after writing, before reading.
    pub fn init_read(&self, buffer: &mut [u8]) -> Option<Tuple> {
        unsafe {
            let ptr = dict_read_begin_from_buffer(self.internal, buffer.as_mut_ptr(),
                                             buffer.len() as u16);
            if ptr == NULL_TUPLE { None } else { Some(*ptr) }
        }
    }

    /// Prepares the dictionary for writing.
    /// You don't need to call this if you use `AppMessage`.
    pub fn init_write(&self, buffer: &mut [u8]) {
        unsafe {
            dict_write_begin(self.internal, buffer.as_mut_ptr(), buffer.len() as u16);
        }
    }

    /// Attempts to read the next `Tuple` in the dictionary.
    pub fn read_next(&self) -> Option<Tuple> {
        unsafe {
            let ptr = dict_read_next(self.internal);
            if ptr == NULL_TUPLE { None } else { Some(*ptr) }
        }
    }

    /// Resets the dictionary, and returns the first `Tuple`, if present.
    pub fn reset(&self) -> Option<Tuple> {
        unsafe {
            let ptr = dict_read_first(self.internal);
            if ptr == NULL_TUPLE { None } else { Some(*ptr) }
        }
    }

    /// Attempts to find a `Tuple` by its key.
    pub fn find(&self, key: u32) -> Option<Tuple> {
        unsafe {
            let ptr = dict_find(self.internal, key);
            if ptr == NULL_TUPLE { None } else { Some(*ptr) }
        }
    }

    pub fn write_string(&self, key: u32, string: &str) {
        unsafe { dict_write_cstring(self.internal, key, string.as_ptr()) };
    }

    pub fn prepare_for_read(&self) {
        unsafe { dict_write_end(self.internal) };
    }

    pub fn write_int<T: Integer>(&self, key: u32, int: T) {
        unsafe {
            let ptr = &int as *const T as *const c_void;
            dict_write_int(self.internal, key, ptr,
                           core::mem::size_of_val(&int) as u8, int.signed());
        }
    }
}

pub trait Integer {
    fn signed(&self) -> bool;
}

macro_rules! impl_signed {
    (for $($t:ty),+) => {
        $(impl Integer for $t {
            fn signed(&self) -> bool {
                true
            }
        })*
    }
}

macro_rules! impl_unsigned {
    (for $($t:ty),+) => {
        $(impl Integer for $t {
            fn signed(&self) -> bool {
                false
            }
        })*
    }
}

impl_signed!(for i32, i64, i8, i16, isize);
impl_unsigned!(for u32, u64, u8, u16, usize);

// ── Received-message helpers ──────────────────────────────────────────────────

/// A non-copying reference to a Tuple in a received AppMessage dictionary.
///
/// Pebble's Tuple C struct is packed: key:u32 | type:u8 | length:u16 | value…
/// We work through a raw byte pointer to avoid the 580-byte copy bug in the
/// `TupleValue` union.
// 'a is tied to the AppMessageDict's underlying buffer lifetime, not to the
// TupleRef itself (which is just a thin raw-pointer wrapper).
pub struct TupleRef<'a>(*const u8, core::marker::PhantomData<&'a u8>);

impl<'a> TupleRef<'a> {
    // type: 0=byte_array 1=cstring 2=uint 3=int
    pub fn as_i32(&self) -> i32 {
        unsafe {
            let tp = self.0;
            let type_byte = *tp.add(4);
            let length = core::ptr::read_unaligned(tp.add(5) as *const u16);
            match (type_byte, length) {
                (2, 1) => *tp.add(7) as i32,
                (2, 2) => core::ptr::read_unaligned(tp.add(7) as *const u16) as i32,
                (3, 1) => (*tp.add(7) as i8) as i32,
                (3, 2) => core::ptr::read_unaligned(tp.add(7) as *const i16) as i32,
                _      => core::ptr::read_unaligned(tp.add(7) as *const i32),
            }
        }
    }

    // Returns &'a str so the slice lifetime is tied to the buffer, not to self.
    // This lets callers use .map(|t| t.as_str()) without a borrow-of-local error.
    pub fn as_str(&self) -> &'a str {
        unsafe {
            let tp = self.0;
            let len = core::ptr::read_unaligned(tp.add(5) as *const u16) as usize;
            let bytes = core::slice::from_raw_parts(tp.add(7), len.saturating_sub(1));
            core::str::from_utf8_unchecked(bytes)
        }
    }
}

/// A safe, non-copying view of a received AppMessage dictionary.
pub struct AppMessageDict(*mut DictionaryIterator);

impl AppMessageDict {
    pub fn from_raw(ptr: *mut DictionaryIterator) -> Self { AppMessageDict(ptr) }

    pub fn find(&self, key: u32) -> Option<TupleRef<'_>> {
        unsafe {
            let tp = dict_find(self.0, key);
            if tp.is_null() { None } else { Some(TupleRef(tp as *const u8, core::marker::PhantomData)) }
        }
    }

    pub fn find_i32(&self, key: u32) -> Option<i32> { self.find(key).map(|t| t.as_i32()) }
    pub fn find_str(&self, key: u32) -> Option<&str> { self.find(key).map(|t| t.as_str()) }
}

pub struct AppMessage;

impl AppMessage {
    pub fn open(size_inbound: u32, size_outbound: u32) {
        unsafe {
            app_message_open(size_inbound, size_outbound);
        }
    }

    pub fn register_inbox(callback: extern "C" fn(dict: DictPtr, ctx: VoidPtr)) {
        unsafe {
            app_message_register_inbox_received(callback);
        }
    }

    pub fn register_inbox_drop(callback: extern "C" fn(reason: AppMessageResult, ctx: VoidPtr)) {
        unsafe {
            app_message_register_inbox_dropped(callback);
        }
    }

    pub fn init_write(dictionary: &mut Dictionary) {
        unsafe {
            // Out-parameter: the firmware stores its iterator pointer through the
            // pointer we pass, so pass the address of our own field. (Passing the
            // old `internal` value here reinterpreted as the slot let the firmware
            // write through a dangling pointer — smashing a saved return address
            // on the stack under some opt levels.)
            app_message_outbox_begin(&mut dictionary.internal as *mut *mut DictionaryIterator);
        }
    }

    pub fn send() {
        unsafe { app_message_outbox_send(); }
    }
}
