# Architecture & naming

## The three layers

Each Pebble API is exposed at three layers, and a type's names share a root
across them (e.g. `RawWindow` → `window_*` → `Window`):

1. **`internal/functions/declarations.rs`** — `unsafe extern "C"` FFI. The raw C
   signatures, raw pointers, `unsafe` to call. Never used directly by apps.
2. **`internal/functions/interface.rs`** — thin safe-ish wrappers that call the
   declarations inside one `unsafe` block (the FFI call). Still take raw C types;
   internal plumbing.
3. **Wrapper structs + their methods** (`Window`, `Layer`, `GFont`, `GContext`, …)
   — what apps use. A method forwards down through `interface.rs` to the FFI.

So `Window::push` → `interface::window_stack_push` → `declarations::window_stack_push`.

## Naming: raw vs wrapper

- A C type that already carries a `G` prefix (`GFont`, `GBitmap`, `GContext`, …)
  keeps the **`G` name on the user-facing wrapper**; its raw form is **`RawG*`**
  (`RawGFont`, `RawGBitmap`).
- A non-`G` C handle (`Window`, `Layer`, …) gives its bare name to the wrapper;
  the raw form is **`Raw*`** (`RawWindow`, `RawLayer`).
- **Value types** (`GColor`, `GRect`, `GPoint`, `GSize`, `GCornerMask`,
  `GTextAlignment`, `MenuIndex`, `MenuRowAlign`, …) have no wrapper — apps use them
  directly, names unchanged.

## When a type needs a wrapper at all

A wrapper is an **ownership boundary**, not just a method bag:

- If app code only ever **borrows** a handle the SDK hands it (e.g. `&mut GContext`
  in a draw callback — never created or freed by the app), skip the wrapper and
  `impl` the methods straight on the raw type. (Future `GRect`/`GPoint` geometry
  helpers belong here too.)
  - **Exception — the nice name is already taken by an owned wrapper.** A menu
    callback borrows the parent menu, but the bare `MenuLayer` name belongs to the
    *owned* `MenuLayer<T>` (it carries the generic callback context `T`, which the
    callback shouldn't see). So the borrowed view is a distinct lifetime-bound
    newtype, `MenuLayerRef<'a>`, holding the raw `*mut RawMenuLayer` — read-only
    methods (`get_selected_index`, …); the mutators stay on the owned `MenuLayer<T>`.
    The `GContext`-style "methods on the raw type" trick only works when no owned
    wrapper claims the name.
- If app code **creates and owns** the handle (`window_create`, `layer_create`, a
  loaded `GFont`, …) it needs a **pointer-holding newtype**. It can't be a value
  type: the object lives in C memory and *is* identified by that pointer, so
  copying it out and freeing/using the copy is UB.
- A wrapper is also warranted when it carries **Rust-side data the handle lacks**
  (`GFont` holds `top_offset`, derived from the `FontKey`, not the `RawGFont`) or
  needs **`Drop`** (`GPath` frees its path).

## When a pointer stays raw

`interface.rs`/wrappers keep a raw pointer (not `&`/`&mut`) when it is:

- **Stored across a callback** — context pointers, `window_set_user_data`, the
  layer data slot, animation context. A `&mut` would imply a borrow that outlives
  its scope.
- **A returned handle** — `layer_create -> *mut RawLayer`; the wrapper takes
  ownership of it.
- **An `extern "C" fn` callback pointer type** or anything in `declarations.rs` —
  the C ABI.

Menu callbacks hand `&mut GContext` (a reference, reusing the SDK's pointer, never
moved), a `MenuCellLayer` (the cell), and a `MenuLayerRef` (a borrowed view of the
parent menu) as the first argument — rather than raw `*mut GContext` / `*const Layer`
/ `*mut MenuLayer`.
