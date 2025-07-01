#[cfg(feature = "deflate")]
use flate_macro::decompress;
use std::borrow::Cow;

/// The JSON file's content
#[cfg(not(feature = "deflate"))]
const ADDONS: &'static [u8] = include_bytes!("../addons.json");

pub fn get_addons_string() -> Cow<'static, [u8]> {
    #[cfg(not(feature = "deflate"))]
    return Cow::Borrowed(ADDONS);

    #[cfg(feature = "deflate")]
    return Cow::Owned(decompress!("addons.json"));
}
// todo: parsed addons manifests

#[cfg(test)]
mod test {
    use super::*;

    pub use serde_json::Value;

    #[test]
    fn addons_file_is_valid_json() {
        let addons_json: Value =
            serde_json::from_slice(&get_addons_string()).expect("Could not decode");

        assert_ne!(addons_json, serde_json::Value::Null);
    }
}
