# Pebble bindings for Rust

This crate provides a subset of the [Pebble SDK](https://developer.rebble.io), using somewhat modern [Rust](https://rust-lang.org).

## [Documentation](https://docs.rs/pebble-rust)
### [Caveats and Limitations](docs/CAVEATS.md)

## Usage

Look at the [examples](examples) to get started. Three examples are included:

| Example | Description |
| ------- | ----------- |
| [hello-world](examples/hello-world) | Basic window with a TextLayer |
| [bitmap](examples/bitmap) | Display a PNG resource with BitmapLayer |
| [js-appmessage](examples/js-appmessage) | AppMessage communication between watch (Rust) and phone (TypeScript) |

### Requirements

* [Pebble SDK](https://developer.rebble.io/developer.pebble.com/sdk/download/index.html) (includes the `arm-none-eabi` toolchain)
* Rust nightly with the `thumbv7m-none-eabi` target:
  ```
  rustup target add thumbv7m-none-eabi
  ```
* [Bun](https://bun.sh) or Node — for the js-appmessage example's TypeScript build step

### Building

Each example has a `build.sh` that handles the full pipeline:

```sh
./build.sh
```

Once built, install to an emulator or device with:

```sh
pebble install --emulator basalt
pebble logs --emulator basalt
```

## Roadmap

| Feature | Priority | Done? |
| ------- | -------- | ----- |
| App | - | Yes |
| Window | - | Yes |
| C STL (`pebble::std`) | - | Yes |
| Dictionary, AppMessage | - | Yes |
| Fonts | - | Yes |
| Click handler | Medium | Yes |
| Animations | Low | Yes |
| Events | Medium | Partially |
| Layer | Medium | Partially |

## Library crates

Two higher-level crates build on top of pebble-rust:

* **[taconite](../taconite)** — reactive screen framework (typed state, auto-render, AppMessage routing)
* **[pebble-transit](../../pebble-transit)** — example app using taconite

## License

This project is licensed under **both** the [GPLv3](LICENSE-GPLv3) and [BSD-3-Clause](LICENSE-BSD-3.0) licenses.
Derivatives of this project should comply with both.

## Credits

Special thanks to [andars](https://github.com/andars). This project uses some files of their [pebble.rs](https://github.com/andars/pebble.rs) project.
