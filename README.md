# Sample Rust app for Epsilon

[![Build](https://github.com/numworks/epsilon-sample-app-rust/actions/workflows/build.yml/badge.svg)](https://github.com/numworks/epsilon-sample-app-rust/actions/workflows/build.yml)

<img src="/doc/screenshots.gif?raw=true" alt="Sample Rust app for the NumWorks graphing calculator" width="300" align="right">

This is a sample [Rust](https://www.rust-lang.org) app to use on a [NumWorks calculator](https://www.numworks.com). Yes, you can now use Rust to write code for a graphing calculator!

```rust
fn eadk_main() {
    for _ in 0..100 {
        let c = eadk::Color { rgb565: random_u16() };
        let r = eadk::Rect { x: random_coordinate(), y: random_coordinate(), width: random_coordinate(), height: random_coordinate() };
        eadk::display::push_rect_uniform(r, c);
    }
    loop {}
}
```

## Build the app

You need to install an embedded ARM toolchain as well as the corresponding rust target and a couple Python modules.

```shell
brew install rustup numworks/tap/arm-none-eabi-gcc # Or equivalent on your OS
rustup-init
rustup target add thumbv7em-none-eabihf
pip3 install lz4 pypng
cargo build
```

## Run the app

The app is sent over to the calculator using the DFU protocol over USB.

```shell
brew install dfu-util # Or equivalent on your OS
# Now connect your NumWorks calculator to your computer using the USB cable
cargo run
```

## Notes

The NumWorks calculator runs [Epsilon](http://github.com/numworks/epsilon), a tailor-made calculator operating system. Starting from version 16, Epsilon allows installing custom binary apps. To run this sample app, make sure your calculator is up-to-date by visiting https://my.numworks.com. Note that at the moment Epsilon 16 is in beta, so you'll need to register as [a beta tester](https://my.numworks.com/user/beta).

Epsilon expects apps to follow a certain layout in memory. Namely, they should start with the following header:

|Offset| Size | Value      | Description                  |
|------|------|------------|------------------------------|
| 0x00 | 0x04 | 0xDEC0BEBA | Magic start-of-header marker |
| 0x04 | 0x04 | 0x00000000 | API Level |
| 0x08 | 0x04 | -          | Offset from start of the app to a NULL-terminated NFKD UTF-8 string containing the app name |
| 0x0C | 0x04 | -          | Size of the icon data |
| 0x10 | 0x04 | -          | Offset from start of the app to the actual icon data. This data should be the result of LZ4-compressing a sequence of 55x56 RGB565 pixels |
| 0x14 | 0x04 | -          | Offset from start of the app to the entry point |
| 0x18 | 0x04 | -          | Size of the entire app |
| 0x22 | 0x04 | 0xDEC0BEBA | Magic end-of-header marker |

Generating the appropriate header is taken care of by a [linker script](/eadk/eadk.ld) when you run `cargo build`. Once the corresponding binary is built on your computer, you will need to install it at address `0x90350000` in your calculator's Flash memory. The included [run.py](/eadk/run.py) script will take care of this for you when you call `cargo run`.

Due to the embedded nature of Epsilon, the Rust app has to be `no_std`. The interface that an app can use to interact with the OS is essentially a short list of system calls. Feel free to browse the [code of Epsilon](http://github.com/numworks/epsilon) itself if you want to get an in-depth look.

Please note that any custom app is removed when resetting the calculator.

## License

This sample app is distributed under the terms of the BSD License. See LICENSE for details.

## Trademarks

NumWorks and Rust are a registered trademarks.
