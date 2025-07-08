#[cfg(feature = "deflate")]
use deflate_macro::decompress;
use std::borrow::Cow;

pub fn get_addons_string() -> Cow<'static, [u8]> {
    #[cfg(not(feature = "deflate"))]
    let addons = Cow::Borrowed(include_bytes!("../addons.json").as_slice());

    #[cfg(feature = "deflate")]
    let addons = Cow::Owned(decompress!("addons.json"));

    addons
}

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

    #[test]
    #[cfg(feature = "deflate")]
    fn addons_file_is_valid_json_with_deflate() {
        let data = decompress!("addons.json");
        let addons_json: Value = serde_json::from_slice(&data).expect("Valid JSON with deflate");
        assert_ne!(addons_json, serde_json::Value::Null);
    }

    #[test]
    fn addons_file_is_valid_json_without_deflate() {
        let data = include_bytes!("../addons.json");
        let addons_json: Value = serde_json::from_slice(data).expect("Valid JSON with deflate");
        assert_ne!(addons_json, serde_json::Value::Null);
    }
}
