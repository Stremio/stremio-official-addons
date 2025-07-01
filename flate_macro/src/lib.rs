use flate2::write::GzEncoder;
use flate2::Compression;
use proc_macro::TokenStream;
use quote::quote;
use std::io::Write;
use syn::{parse_macro_input, LitStr};

/// Creates a lazy static that caches decompressed data (like flate!)
#[proc_macro]
pub fn flate_static(input: TokenStream) -> TokenStream {
    // Parse: static NAME: [u8] from "path"
    let input_str = input.to_string();
    let parts: Vec<&str> = input_str.split_whitespace().collect();

    if parts.len() < 5
        || parts[0] != "static"
        || parts[2] != ":"
        || parts[3] != "[u8]"
        || parts[4] != "from"
    {
        return syn::Error::new(
            proc_macro2::Span::call_site(),
            "Expected format: static NAME: [u8] from \"path\"",
        )
        .to_compile_error()
        .into();
    }

    let static_name = &parts[1];
    let file_path = parts[5].trim_matches('"');

    // Read and compress file
    let content = match std::fs::read(file_path) {
        Ok(content) => content,
        Err(e) => {
            return syn::Error::new(
                proc_macro2::Span::call_site(),
                format!("Failed to read file '{}': {}", file_path, e),
            )
            .to_compile_error()
            .into();
        }
    };

    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&content).unwrap();
    let compressed = encoder.finish().unwrap();

    let bytes = compressed.iter().map(|&b| b as u8);
    let static_ident = syn::Ident::new(static_name, proc_macro2::Span::call_site());

    // Generate lazy static that decompresses once and caches
    let expanded = quote! {
        static #static_ident: once_cell::sync::Lazy<Vec<u8>> = once_cell::sync::Lazy::new(|| {
            use flate2::read::GzDecoder;
            use std::io::Read;

            const COMPRESSED: &[u8] = &[#(#bytes),*];
            let mut decoder = GzDecoder::new(COMPRESSED);
            let mut decompressed = Vec::new();
            decoder.read_to_end(&mut decompressed).expect("Decompression failed");
            decompressed
        });
    };

    TokenStream::from(expanded)
}

/// Macro 2: Returns decompressed bytes as temporary value (no caching)
/// Usage: let data = include_decompressed_bytes!("path/to/file");
#[proc_macro]
pub fn decompress(input: TokenStream) -> TokenStream {
    let file_path = parse_macro_input!(input as LitStr);
    let path = file_path.value();

    // Read and compress file at compile time
    let content = match std::fs::read(&path) {
        Ok(content) => content,
        Err(e) => {
            return syn::Error::new_spanned(
                file_path,
                format!("Failed to read file '{}': {}", path, e),
            )
            .to_compile_error()
            .into();
        }
    };

    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&content).unwrap();
    let compressed = encoder.finish().unwrap();

    let bytes = compressed.iter().map(|&b| b as u8);

    // Generate code that decompresses at each call (temporary)
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
