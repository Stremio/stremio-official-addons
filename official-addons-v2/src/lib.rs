#[cfg(feature = "json")]
pub use serde_json::Value;
#[cfg(feature = "json")]
use once_cell::sync::Lazy;

/// The JSON file's content
pub const ADDONS: &'static [u8] = include_bytes!("../addons.json");

#[cfg(feature = "json")]
/// The JSON file content but after json validation
pub const ADDONS_JSON: Lazy<Value> =
    Lazy::new(|| serde_json::from_slice(ADDONS).expect("Valid JSON"));

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