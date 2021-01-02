# Testing project for Rust Wii U

```
rustup target add powerpc-unknown-linux-gnu
rustup component add --toolchain nightly rust-src
cargo +nightly build -Z build-std=core,alloc --target powerpc-unknown-eabi.json
elf2rpl target/powerpc-unknown-eabi/debug/rust-wii rust-wii.rpx
```
