# wut-rust demo

## Building

```
rustup target add powerpc-unknown-linux-gnu
rustup component add --toolchain nightly rust-src
cargo +nightly build --release -Z build-std=core,alloc --target powerpc-unknown-eabi.json
powerpc-eabi-strip -g  target/powerpc-unknown-eabi/release/wut-rust-demo
elf2rpl target/powerpc-unknown-eabi/release/wut-rust-demo wut-rust-demo.rpx
```
