use core::alloc::{GlobalAlloc, Layout};

pub struct Allocator;

unsafe extern "C" {
    pub unsafe fn malloc(size: usize) -> *mut u8;
    pub unsafe fn calloc(count: usize, size: usize) -> *mut u8;
    pub unsafe fn realloc(ptr: *mut u8, size: usize) -> *mut u8;
    pub unsafe fn free(ptr: *mut u8);

    pub unsafe fn memcmp(ptr1: *const u8, ptr2: *const u8, num_bytes: usize) -> i32;
    pub unsafe fn memcpy(dest: *mut u8, src: *const u8, num_bytes: usize) -> *mut u8;
    pub unsafe fn memmove(dest: *mut u8, src: *const u8, num_bytes: usize) -> *mut u8;
    pub unsafe fn memset(dest: *mut u8, assign: i32, num_bytes: usize) -> *mut u8;
}

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        malloc(layout.size())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        free(ptr);
    }

    unsafe fn realloc(&self, ptr: *mut u8, _layout: Layout, new_size: usize) -> *mut u8 {
        realloc(ptr, new_size)
    }
}
