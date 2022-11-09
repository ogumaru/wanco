mod utils;
use encoding_rs::{SHIFT_JIS, UTF_8, EUC_JP};
use js_sys::{Error, JsString};
use serde::{Deserialize, Serialize};
use unicode_normalization::UnicodeNormalization;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Serialize, Deserialize)]
pub struct EncodeResult {
    pub bytes: Vec<u8>,
    pub encoding: String,
    pub has_unmappable: bool,
}

#[wasm_bindgen]
pub fn encode(js_string: &JsString, encode_to: &JsString) -> Result<JsValue, JsValue> {
    let encoding = if encode_to.is_undefined() {
        UTF_8
    } else {
        match String::from(encode_to).to_lowercase().as_str() {
            "euc-jp" | "eucjp" => EUC_JP,
            "shift_jis" | "sjis" | "shift-jis" => SHIFT_JIS,
            "utf-8" | "utf8" => UTF_8,
            unsupported => {
                let error_message = format!("Not supported endoding: {}", unsupported);
                let error = Error::new(&error_message);
                return Err(JsValue::from(error));
            }
        }
    };

    let rust_string = String::from(js_string);
    let normalized = rust_string.chars().nfc().collect::<String>();
    let (encoded, encoded_as, has_unmappable) = encoding.encode(normalized.as_str());
    let arr = Vec::from(encoded);

    let tuple = EncodeResult {
        bytes: arr,
        encoding: String::from(encoded_as.name()),
        has_unmappable,
    };
    Ok(serde_wasm_bindgen::to_value(&tuple)?)
}
