# cafeos-sys

bindgen bindings for Wii U's CafeOS system libraries.

## Generate

```
set DEVKITPRO=...
python wut-bindgen.py
```

## Testing

To run the bindgen layout tests ensure you run them from a 32 bit target otherwise the pointer size will not match, eg:

```
rustup run stable-i686-pc-windows-gnu cargo test
```
