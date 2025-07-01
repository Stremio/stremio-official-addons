use flate2::write::GzEncoder;
use flate2::Compression;
use proc_macro::TokenStream;
use quote::quote;
use std::io::Write;
use syn::{parse_macro_input, LitStr};

/// Returns decompressed bytes as temporary value (no caching)
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
