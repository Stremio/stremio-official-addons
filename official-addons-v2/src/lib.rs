#[cfg(all(feature = "json", feature = "deflate", not(feature = "raw_bytes")))]
use flate_macro::decompress;
#[cfg(all(feature = "json", feature = "raw_bytes", not(feature = "deflate")))]
use flate_macro::flate_static;

#[cfg(feature = "json")]
use once_cell::sync::Lazy;
#[cfg(feature = "json")]
pub use serde_json::Value;

/// The JSON file's content
#[cfg(all(feature = "raw_bytes", not(feature = "deflate")))]
pub const ADDONS: &'static [u8] = include_bytes!("../addons.json");

#[cfg(all(feature = "raw_bytes", feature = "deflate"))]
flate_static!(pub static ADDONS: [u8] from "../addons.json");

/// The JSON file content but after json validation
#[cfg(all(feature = "json", feature = "raw_bytes", not(feature = "deflate")))]
pub const ADDONS_JSON: Lazy<Value> =
    Lazy::new(|| serde_json::from_slice(ADDONS).expect("Valid JSON"));

#[cfg(all(feature = "json", feature = "deflate", not(feature = "raw_bytes")))]
// Decompress from slice and discard it from memory
pub const ADDONS_JSON: Lazy<Value> = Lazy::new(|| {
    serde_json::from_slice(decompress!("addons.json").as_slice()).expect("Valid JSON")
});

#[cfg(all(feature = "json", feature = "raw_bytes", feature = "deflate"))]
/// The JSON file content but after json validation
pub const ADDONS_JSON: Lazy<Value> =
    Lazy::new(|| serde_json::from_slice(&ADDONS).expect("Valid JSON")); // ADDONS is now static

// todo: parsed addons manifests

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[cfg(feature = "json")]
    fn addons_file_is_valid_json() {
        let addons_json = ADDONS_JSON.clone();

        assert_ne!(addons_json, serde_json::Value::Null);
    }
}
