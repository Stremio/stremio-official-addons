//!
//!
//! ## How to use:
//!
//! In `Cargo.toml`:
//! ```toml
//! [dependencies]
//! flate2 = { version = "1", default-features = false, features = ["rust_backend"] }
//! deflate-macro = "0.1"
//! ```
//!
//! ```text
//! /// Path relative to the cargo crate
//! pub static COMPILE_TIME_COMPRESSED_RUNTIME_DECOMPRESSED: String = deflate_macro::decompress!("addons.json");
//! ```
//!
use std::io::Write;

use flate2::{write::GzEncoder, Compression};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

/// Returns decompressed bytes as temporary value (no caching)
#[proc_macro]
pub fn decompress(input: TokenStream) -> TokenStream {
    let file_path = parse_macro_input!(input as LitStr);
    let relative_path = file_path.value();

    // Stable workaround: resolve relative to caller crate’s CARGO_MANIFEST_DIR
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let absolute_path = std::path::Path::new(&manifest_dir).join(&relative_path);

    let content = match std::fs::read(&absolute_path) {
        Ok(content) => content,
        Err(e) => {
            return syn::Error::new_spanned(
                file_path,
                format!(
                    "Failed to read file '{}': {} (resolved to {:?})",
                    relative_path, e, absolute_path
                ),
            )
            .to_compile_error()
            .into();
        }
    };

    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&content).unwrap();
    let compressed = encoder.finish().unwrap();
    let bytes = compressed.iter().map(|&b| b as u8);

    let expanded = quote! {
        {
            use flate2::read::GzDecoder;
            use std::io::Read;

            const COMPRESSED: &[u8] = &[#(#bytes),*];
            let mut decoder = GzDecoder::new(COMPRESSED);
            let mut decompressed = Vec::new();
            decoder.read_to_end(&mut decompressed).expect("Decompression failed");
            decompressed
        }
    };

    TokenStream::from(expanded)
}
