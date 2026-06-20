#@IgnoreInspection BashAddShebang

target="thumbv7m-none-eabi"

# panic=immediate-abort lowers every panic (incl. bounds checks) straight to abort,
# so no format_args is built and the core::fmt tree gc's out (needs build-std).
# force-unwind-tables=no stops Rust emitting .ARM.exidx (drops the unwinder + shrinks .data/.bss).
export RUSTFLAGS="-C relocation-model=pie -C codegen-units=1 -C link-arg=--gc-sections -C link-arg=--build-id=sha1 -C link-arg=--emit-relocs -C debuginfo=2 -C panic=immediate-abort -C force-unwind-tables=no -Z unstable-options"

# Build the project through Cargo
cargo --version
cargo build --target $target --release || exit 1

# Extract the self-contained crate-type=staticlib output (lib<crate>.a bundles exactly
# the LTO'd core/alloc/compiler_builtins/app objects) into a FRESH dir each build, and
# link that — NOT the stale, un-LTO'd, accumulating deps/*.rcgu.o the stock script used.
PEBBLE_AR="$HOME/Library/Application Support/Pebble SDK/SDKs/current/toolchain/arm-none-eabi/bin/arm-none-eabi-ar"
LINK_OBJS="target/$target/release/link-objs"
rm -rf "$LINK_OBJS"; mkdir -p "$LINK_OBJS"
( cd "$LINK_OBJS" && "$PEBBLE_AR" x ../*.a )   # exactly one staticlib in release/

# Build through waf
pebble build
