# Epsilon sample app in Rust

## How to build

```
brew install rustup brew install numworks/tap/arm-none-eabi-gcc # Or equivalent on your OS
rustup target add thumbv7em-none-eabihf
pip3 install lz4 pypng
cargo build
```

## How to run

```
brew install dfu-util # Or equivalent
# Plug your NumWorks calculator via USB
cargo run
```
