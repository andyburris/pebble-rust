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

use crate::pebble::internal::{types, functions::interface};
use crate::pebble::types::GColor;
use crate::pebble::WindowPtr;
use crate::pebble::layer::LayerRef;

/// An owned window: created with `Window::new` and destroyed on drop.
pub struct Window {
    internal: *mut types::RawWindow
}

#[derive(Copy, Clone)]
pub struct WindowHandlers {
    pub load: extern "C" fn(WindowPtr),
    pub unload: extern "C" fn(WindowPtr),
    pub appear: extern "C" fn(WindowPtr),
    pub disappear: extern "C" fn(WindowPtr)
}

impl Window {
    pub fn new() -> Window {
        Window {
            internal: interface::window_create()
        }
    }

    pub fn push(&self, animate: bool) {
        interface::window_stack_push(self.internal, animate);
    }

    pub fn set_handlers(&self, handlers: WindowHandlers) {
        let WindowHandlers {load, unload,
            appear, disappear} = handlers;
        let converted = types::WindowHandlers {
            load, unload, appear, disappear
        };

        interface::window_set_window_handlers(self.internal, converted);
    }

    pub fn set_click_config_provider(&self, provider: extern "C" fn(WindowPtr)) {
        interface::window_set_click_config_provider(self.internal, provider);
    }

    pub fn set_background_color(&self, color: GColor) {
        interface::window_set_background_color(self.internal, color);
    }

    pub fn get_root_layer(&self) -> LayerRef {
        let layer_ptr = interface::window_get_root_layer(self.internal);
        LayerRef::from_raw(layer_ptr)
    }

    pub fn set_user_data<T>(&self, data: *mut T) {
        interface::window_set_user_data(self.internal, data);
    }

    pub fn get_user_data<T>(&self) -> *mut T {
        interface::window_get_user_data(self.internal)
    }

    pub fn raw(&self) -> WindowPtr {
        self.internal
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        interface::window_destroy(self.internal);
    }
}

/// A non-owning handle to a window the app does not own — used by the window
/// event callbacks (load/unload/appear/disappear), which receive a raw
/// `WindowPtr` owned by the window stack. Copyable; never destroys the window.
#[derive(Copy, Clone)]
pub struct WindowRef {
    internal: *mut types::RawWindow,
}

impl WindowRef {
    pub fn from_raw(ptr: WindowPtr) -> WindowRef {
        WindowRef { internal: ptr }
    }

    pub fn get_root_layer(&self) -> LayerRef {
        LayerRef::from_raw(interface::window_get_root_layer(self.internal))
    }

    pub fn get_user_data<T>(&self) -> *mut T {
        interface::window_get_user_data(self.internal)
    }

    pub fn raw(&self) -> WindowPtr {
        self.internal
    }
}
