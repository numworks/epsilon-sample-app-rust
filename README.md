# Sample Rust app for Epsilon

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

First install the rust installer [rustup](https://rustup.rs/).
Then install an embedded ARM toolchain:
```shell
rustup-init
rustup target add thumbv7em-none-eabihf
```

Install [cargo-make](https://sagiegurari.github.io/cargo-make/) the build tool used to automate tasks.
```shell
cargo install --force cargo-make  
```

You must then install [nodejs](https://nodejs.org/en/) and [nwlink](https://www.npmjs.com/package/nwlink) the numworks tool used to load the app to the calculator.
Numworks use a tool called [nwlink](https://www.npmjs.com/package/nwlink) to load apps to the calculator.
This tool requires [nodejs] to run, if you are a Linux/macOS user, take a look at [nvm](https://github.com/nvm-sh/nvm).
After nodejs, install nwlink:
```shell
npm install -g nwlink
```

Finally, build using cargo-make:
```shell
cargo make build
```


## Run the app

The app is sent over to the calculator using the DFU protocol over USB.

```shell
# Now connect your NumWorks calculator to your computer using the USB cable
cargo make run
```

## Notes

The NumWorks calculator runs [Epsilon](http://github.com/numworks/epsilon), a tailor-made calculator operating system. Starting from version 16, Epsilon allows installing custom binary apps. To run this sample app, make sure your calculator is up-to-date by visiting https://my.numworks.com.

Due to the embedded nature of Epsilon, the Rust app has to be `no_std`. The interface that an app can use to interact with the OS is essentially a short list of system calls. Feel free to browse the [code of Epsilon](http://github.com/numworks/epsilon) itself if you want to get an in-depth look.

Please note that any custom app is removed when resetting the calculator.

## License

This sample app is distributed under the terms of the BSD License. See LICENSE for details.

## Trademarks

NumWorks and Rust are a registered trademarks.
