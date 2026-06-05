# Caveats and Limitations

## Tuple size limit
This library can only handle tuples up to **580 bytes**.

## Formatting numbers
Pebble can't handle the stack depth of `core::fmt`, so anything using it (e.g. `format!()` or `8.to_string()`) will crash. Use `pbl_format!()` instead.

## Binary size
On aplite, we only get 24k of code to work with. Certain Rust library functions pull in large chunks of code that will quickly eat up that space. The following are some of the patterns I've found to avoid, but there are many others.

| Pattern | Approx. cost | Why | Alternative |
|---|---|---|---|
| `slice[i]` / `vec[i]` indexing | ~2400 bytes | Panic message formats the index and length via `core::fmt`, pulling in `pad_integral`, `do_count_chars`, and `Display` impls | `.get(i)` / `.get_mut(i)` |
| `format!()` / `write!()` / any `{}` | hundreds of bytes | Pulls in the full `core::fmt` machinery | `pbl_format!()` |
| `i64` / `u64` arithmetic | ~1000 bytes | ARM Cortex-M has no native 64-bit division; the compiler links a software routine (`u64_div_rem`) | Restructure as `i32`, or shift operands down before multiplying |
| `f32` / `f64` arithmetic | very large | Links the soft-float library | Use integer trig (Pebble's `sin_lookup` / `cos_lookup`) |

**Recommended**: add `#![deny(clippy::indexing_slicing)]` to your app crate to catch `[]` indexing at compile time. 

For anything not covered here, you can copy `tools/sym_sizes.sh` into the root of your project and use it to figure out the symbols that are taking up the most space. LLMs are usually quite good at tracking down the problem from there.