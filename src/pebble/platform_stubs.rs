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
