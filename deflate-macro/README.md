# deflate-macro

Macro for compressing file content on compile time while decompressing on runtime

## How to use:
In `Cargo.toml` add the two dependencies:

```toml
[dependencies]
flate2 = { version = "1", default-features = false, features = ["rust_backend"] }
deflate-macro = "0.1"
```

```rust
/// Path relative to the cargo crate
pub static COMPILE_TIME_COMPRESSED_RUNTIME_DECOMPRESSED: String = deflate_macro::decompress!("addons.json");
```

## License
### MIT