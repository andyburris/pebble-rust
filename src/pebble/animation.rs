use alloc::boxed::Box;
use crate::pebble::internal::{types, functions::interface};
use crate::pebble::internal::types::{Animation, AnimationProgress, AnimationCurve, AnimationImplementation, AnimationHandlers, ANIMATION_NORMALIZED_MAX};

// ── ManagedAnimation<T> ───────────────────────────────────────────────────────

struct AnimContext<T> {
    impl_:   AnimationImplementation,  // stable heap address (inside Box)
    data:    T,
    update:  fn(&mut T, AnimationProgress),
    stopped: Option<fn(&mut T, bool)>,
}

extern "C" fn update_trampoline<T>(anim: *mut Animation, progress: AnimationProgress) {
    let ctx = unsafe { &mut *(interface::animation_get_context::<AnimContext<T>>(anim)) };
    (ctx.update)(&mut ctx.data, progress);
}

extern "C" fn stopped_trampoline<T>(_: *mut Animation, finished: bool, ctx_ptr: *mut u8) {
    let ctx = unsafe { &mut *(ctx_ptr as *mut AnimContext<T>) };
    if let Some(f) = ctx.stopped { f(&mut ctx.data, finished); }
}

pub struct ManagedAnimation<T> {
    raw:  *mut Animation,
    _ctx: Box<AnimContext<T>>,
}

impl<T> ManagedAnimation<T> {
    pub fn create(
        data:       T,
        update:     fn(&mut T, AnimationProgress),
        stopped:    Option<fn(&mut T, bool)>,
        duration_ms: u32,
        curve:      AnimationCurve,
    ) -> Self {
        let ctx = Box::new(AnimContext {
            impl_: AnimationImplementation {
                setup:    None,
                update:   Some(update_trampoline::<T>),
                teardown: None,
            },
            data,
            update,
            stopped,
        });
        // impl_ptr is into the heap allocation — stable after Box::into_raw
        let impl_ptr: *const AnimationImplementation = &ctx.impl_;
        let ctx_raw = Box::into_raw(ctx);

        let raw = interface::animation_create();
        interface::animation_set_duration(raw, duration_ms);
        interface::animation_set_curve(raw, curve);
        interface::animation_set_implementation(raw, impl_ptr);
        // set_handlers stores ctx_raw as the context (retrieved by animation_get_context in update_trampoline)
        interface::animation_set_handlers(raw, AnimationHandlers {
            started: None,
            stopped: stopped.map(|_| stopped_trampoline::<T> as extern "C" fn(*mut Animation, bool, *mut u8)),
        }, ctx_raw as *mut u8);

        ManagedAnimation { raw, _ctx: unsafe { Box::from_raw(ctx_raw) } }
    }

    pub fn schedule(&self)   { interface::animation_schedule(self.raw); }
    pub fn unschedule(&self) { interface::animation_unschedule(self.raw); }
}

impl<T> Drop for ManagedAnimation<T> {
    fn drop(&mut self) {
        interface::animation_unschedule(self.raw);
        interface::animation_destroy(self.raw);
    }
}

// ── Interpolatable ────────────────────────────────────────────────────────────

pub trait Interpolatable: Copy + PartialEq {
    fn interpolate(from: Self, to: Self, progress: AnimationProgress) -> Self;
}

impl Interpolatable for i32 {
    fn interpolate(from: i32, to: i32, progress: AnimationProgress) -> i32 {
        // Shift progress right by 1 (0..32767) so delta*p fits in i32 when |delta| <= TRIG_MAX_ANGLE
        let delta = to - from;
        let p = (progress >> 1) as i32;
        from + delta * p / (ANIMATION_NORMALIZED_MAX as i32 >> 1)
    }
}

// ── AnimationState<T, C> ─────────────────────────────────────────────────────
//
// High-level declarative animation. The public type is a thin newtype around a
// Box, so it can be stored as a plain struct field and moved freely — the raw
// pointer captured by `animate_towards` always points into the stable heap
// allocation inside the Box, not into the outer struct.

struct AnimStateCtx<T: Interpolatable, C> {
    state:     *mut AnimStateData<T, C>,
    context:   *mut C,
    on_update: fn(&mut C, &AnimStateData<T, C>),
}

fn anim_state_update<T: Interpolatable, C>(ctx: &mut AnimStateCtx<T, C>, progress: AnimationProgress) {
    let data = unsafe { &mut *ctx.state };
    if let (Some(from), Some(to)) = (data.previous, data.target) {
        data.progress = progress;
        data.current  = Some(T::interpolate(from, to, progress));
    }
    (ctx.on_update)(unsafe { &mut *ctx.context }, unsafe { &*ctx.state });
}

pub struct AnimStateData<T: Interpolatable, C> {
    pub previous: Option<T>,
    pub current:  Option<T>,
    pub target:   Option<T>,
    pub progress: AnimationProgress,
    context_ptr: *mut C,
    on_update:   fn(&mut C, &AnimStateData<T, C>),
    anim:        Option<ManagedAnimation<AnimStateCtx<T, C>>>,
}

impl<T: Interpolatable, C> AnimStateData<T, C> {
    pub fn context(&self) -> *mut C { self.context_ptr }

    /// Call from the draw callback each frame. Returns the current animated value.
    /// If `value` differs from the current target, cancels any in-flight animation
    /// and starts a new one from the current position toward the new target.
    /// On the very first call (all Options are None) snaps to `value` immediately.
    pub fn animate_towards(&mut self, value: T) -> T {
        match self.target {
            None => {
                self.previous = Some(value);
                self.current  = Some(value);
                self.target   = Some(value);
                value
            }
            Some(t) if t == value => self.current.unwrap_or(value),
            Some(_) => {
                let from = self.current.unwrap_or(value);
                self.previous = Some(from);
                self.current  = Some(from);
                self.target   = Some(value);
                self.progress = 0;
                // Assigning drops the old ManagedAnimation → unschedule+destroy mid-flight
                self.anim = Some(ManagedAnimation::create(
                    AnimStateCtx {
                        state:     self as *mut Self,
                        context:   self.context_ptr,
                        on_update: self.on_update,
                    },
                    anim_state_update::<T, C>,
                    None,
                    300,
                    AnimationCurve::EaseInOut,
                ));
                self.anim.as_ref().unwrap().schedule();
                from
            }
        }
    }
}

pub struct AnimationState<T: Interpolatable, C>(Box<AnimStateData<T, C>>);

impl<T: Interpolatable, C> AnimationState<T, C> {
    pub fn new(context: *mut C, on_update: fn(&mut C, &AnimStateData<T, C>)) -> Self {
        AnimationState(Box::new(AnimStateData {
            previous:    None,
            current:     None,
            target:      None,
            progress:    0,
            context_ptr: context,
            on_update,
            anim:        None,
        }))
    }

    /// Returns the stable heap address for use as a DrawLayer context pointer.
    /// Safe to store before the containing struct is moved — the Box heap data
    /// doesn't move, only the Box pointer itself moves.
    pub fn data_ptr(&mut self) -> *mut AnimStateData<T, C> {
        self.0.as_mut() as *mut _
    }
}

impl<T: Interpolatable, C> core::ops::Deref for AnimationState<T, C> {
    type Target = AnimStateData<T, C>;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl<T: Interpolatable, C> core::ops::DerefMut for AnimationState<T, C> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}
