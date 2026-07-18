use alloc::boxed::Box;
use crate::pebble::internal::functions::interface;
use crate::pebble::internal::types::{RawAnimation, AnimationImplementation, AnimationHandlers};

// Re-export the value types so consumers use `pebble::animation::*` rather than
// reaching into `internal::types`.
pub use crate::pebble::internal::types::{AnimationProgress, AnimationCurve, ANIMATION_NORMALIZED_MAX};

/// Passed to [`Animation::set_play_count`] to repeat the animation forever. Matches the
/// C SDK's `ANIMATION_PLAY_COUNT_INFINITE` (`UINT32_MAX`).
pub const ANIMATION_PLAY_COUNT_INFINITE: u32 = u32::MAX;

// ── Animation ─────────────────────────────────────────────────────────────────
//
// A faithful wrapper over the Pebble C `Animation` API. The public type owns the
// `Animation*` and its heap context, drives a per-frame `update` closure, and
// unschedules + destroys itself on drop.

struct AnimContext {
    // `impl_` must live at a stable address — Pebble stores the pointer we hand it.
    impl_:   AnimationImplementation,
    update:  Box<dyn FnMut(AnimationProgress)>,
    stopped: Option<Box<dyn FnMut(bool)>>,
}

extern "C" fn update_trampoline(anim: *mut RawAnimation, progress: AnimationProgress) {
    let ctx = interface::animation_get_context::<AnimContext>(anim);
    let ctx = unsafe { &mut *ctx };
    (ctx.update)(progress);
}

extern "C" fn stopped_trampoline(_: *mut RawAnimation, finished: bool, ctx_ptr: *mut u8) {
    let ctx = unsafe { &mut *(ctx_ptr as *mut AnimContext) };
    if let Some(f) = ctx.stopped.as_mut() { f(finished); }
}

/// An owned Pebble `Animation`. Calls `update(progress)` each frame while
/// scheduled (progress runs `0..=ANIMATION_NORMALIZED_MAX` across the curved
/// duration). Unschedules and destroys itself on drop.
pub struct Animation {
    raw: *mut RawAnimation,
    // Raw (not Box) so no `Box` move can invalidate the pointers C holds into it —
    // deriving a pointer from a Box and then moving the Box is UB (Box is noalias).
    ctx: *mut AnimContext,
}

impl Animation {
    /// Create an animation driven by `update`. Configure it with the setters, then
    /// `schedule()` it.
    pub fn new(update: impl FnMut(AnimationProgress) + 'static) -> Self {
        let ctx = Box::into_raw(Box::new(AnimContext {
            impl_: AnimationImplementation {
                setup:    None,
                update:   Some(update_trampoline),
                teardown: None,
            },
            update:  Box::new(update),
            stopped: None,
        }));
        let impl_ptr = unsafe { &raw const (*ctx).impl_ };

        let raw = interface::animation_create();
        interface::animation_set_implementation(raw, impl_ptr);
        // The context handed here is retrieved by `animation_get_context` in the
        // update trampoline (and passed straight to the stopped trampoline).
        interface::animation_set_handlers(raw, AnimationHandlers { started: None, stopped: None }, ctx as *mut u8);

        Animation { raw, ctx }
    }

    pub fn set_duration(&self, ms: u32) { interface::animation_set_duration(self.raw, ms); }
    pub fn set_delay(&self, ms: u32) { interface::animation_set_delay(self.raw, ms); }
    pub fn set_curve(&self, curve: AnimationCurve) { interface::animation_set_curve(self.raw, curve); }
    pub fn set_play_count(&self, count: u32) { interface::animation_set_play_count(self.raw, count); }
    pub fn set_reverse(&self, reverse: bool) { interface::animation_set_reverse(self.raw, reverse); }

    /// Register a callback fired when the animation stops. `finished` is true if it
    /// ran to completion, false if it was cancelled/unscheduled early.
    pub fn on_stopped(&mut self, stopped: impl FnMut(bool) + 'static) {
        unsafe { (*self.ctx).stopped = Some(Box::new(stopped)); }
        interface::animation_set_handlers(
            self.raw,
            AnimationHandlers { started: None, stopped: Some(stopped_trampoline) },
            self.ctx as *mut u8,
        );
    }

    /// Schedule the animation. Returns false if scheduling failed. Note that a
    /// scheduled animation is immutable, and the firmware auto-destroys it once it
    /// stops — create a fresh `Animation` instead of reusing a stopped one.
    pub fn schedule(&self) -> bool { interface::animation_schedule(self.raw) }
    pub fn unschedule(&self) { interface::animation_unschedule(self.raw); }
}

impl Drop for Animation {
    fn drop(&mut self) {
        interface::animation_unschedule(self.raw);
        interface::animation_destroy(self.raw);
        unsafe { drop(Box::from_raw(self.ctx)); }
    }
}
