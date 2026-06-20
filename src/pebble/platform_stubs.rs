// Minimal newlib syscall stubs required for bare-metal ARM targets.
//
// Pebble's SDK is built on ARM newlib, a C standard library that expects a
// small set of OS-interface functions to be defined by the application. On a
// real OS these would call into the kernel; on bare-metal Pebble there is no
// kernel, so we stub them out. Without these the linker produces "undefined
// reference to `_exit`" (and similar) errors when the app uses alloc or any
// SDK function that pulls in newlib internals.

#[unsafe(no_mangle)]
pub extern "C" fn _exit(_status: i32) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _kill(_pid: i32, _sig: i32) -> i32 {
    -1
}

#[unsafe(no_mangle)]
pub extern "C" fn _getpid() -> i32 {
    1
}

// ── Tiny mem* + ARM EABI aliases ──────────────────────────────────────────────
// thumbv7m codegen emits the ARM EABI names (__aeabi_memcpy/__aeabi_memmove4/…),
// which otherwise resolve to compiler_builtins' weak aliases and drag in its large
// mem impls. Define strong ones (delegating to a tiny self-contained impl) so
// --gc-sections drops the big versions. The crate is #![no_builtins], so these
// loop bodies are not re-lowered back into memcpy/memset calls.

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memcpy(d: *mut u8, s: *const u8, n: usize) -> *mut u8 {
    unsafe { let mut i = 0; while i < n { *d.add(i) = *s.add(i); i += 1; } }
    d
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn memmove(d: *mut u8, s: *const u8, n: usize) -> *mut u8 {
    unsafe {
        if (d as usize) < (s as usize) { let mut i = 0; while i < n { *d.add(i) = *s.add(i); i += 1; } }
        else { let mut i = n; while i > 0 { i -= 1; *d.add(i) = *s.add(i); } } // overlap-safe
    }
    d
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn memset(d: *mut u8, c: i32, n: usize) -> *mut u8 {
    unsafe { let b = c as u8; let mut i = 0; while i < n { *d.add(i) = b; i += 1; } }
    d
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn memcmp(a: *const u8, b: *const u8, n: usize) -> i32 {
    unsafe {
        let mut i = 0;
        while i < n {
            let (x, y) = (*a.add(i), *b.add(i));
            if x != y { return x as i32 - y as i32; }
            i += 1;
        }
    }
    0
}

// __aeabi_memcpy/memmove (+ 4/8-aligned variants) — same unaligned impl.
macro_rules! aeabi_copy { ($($n:ident => $f:ident),* $(,)?) => {$(
    #[unsafe(no_mangle)] pub unsafe extern "C" fn $n(d: *mut u8, s: *const u8, n: usize) { unsafe { $f(d, s, n); } }
)*}; }
aeabi_copy! {
    __aeabi_memcpy => memcpy, __aeabi_memcpy4 => memcpy, __aeabi_memcpy8 => memcpy,
    __aeabi_memmove => memmove, __aeabi_memmove4 => memmove, __aeabi_memmove8 => memmove,
}
// NOTE the swapped arg order: __aeabi_memset(dest, n, c), not C's (dest, c, n).
macro_rules! aeabi_set { ($($n:ident),* $(,)?) => {$(
    #[unsafe(no_mangle)] pub unsafe extern "C" fn $n(d: *mut u8, n: usize, c: i32) { unsafe { memset(d, c, n); } }
)*}; }
aeabi_set! { __aeabi_memset, __aeabi_memset4, __aeabi_memset8 }
macro_rules! aeabi_clr { ($($n:ident),* $(,)?) => {$(
    #[unsafe(no_mangle)] pub unsafe extern "C" fn $n(d: *mut u8, n: usize) { unsafe { memset(d, 0, n); } }
)*}; }
aeabi_clr! { __aeabi_memclr, __aeabi_memclr4, __aeabi_memclr8 }

// ── EHABI personality stubs ───────────────────────────────────────────────────
// Empty strong defs keep libgcc's unwinder (and everything it references) out of
// the link. Never actually called under panic=abort.
#[unsafe(no_mangle)] pub extern "C" fn __aeabi_unwind_cpp_pr0() {}
#[unsafe(no_mangle)] pub extern "C" fn __aeabi_unwind_cpp_pr1() {}
#[unsafe(no_mangle)] pub extern "C" fn __aeabi_unwind_cpp_pr2() {}
