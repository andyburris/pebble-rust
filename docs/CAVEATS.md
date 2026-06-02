# Caveats and Limitations

## Tuple size limit
This library can only handle tuples up to **580 bytes**.

## Formatting numbers
Pebble can't handle the stack depth of `core::fmt`, so anything using it (e.g. `format!()` or `8.to_string()`) will crash. Use `pbl_format!()` instead.