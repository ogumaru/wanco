//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use std::assert_eq;

use js_sys::JsString;
use serde_wasm_bindgen::from_value;
use wanco::{encode, EncodeResult};
use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn to_default_mappable() {
    let js_string = JsString::from("ほげふが");
    let encode_to = JsString::from(JsValue::UNDEFINED);
    let result = encode(&js_string, &encode_to);
    let encoded_result = from_value::<EncodeResult>(result.unwrap());
    let encoded = encoded_result.unwrap();
    assert_eq!(
        encoded.bytes,
        [227, 129, 187, 227, 129, 146, 227, 129, 181, 227, 129, 140]
    );
    assert_eq!(encoded.has_unmappable, false);
}

#[wasm_bindgen_test]
fn to_default_emoji() {
    let js_string = JsString::from("🍣");
    let encode_to = JsString::from(JsValue::UNDEFINED);
    let result = encode(&js_string, &encode_to);
    let encoded_result = from_value::<EncodeResult>(result.unwrap());
    let encoded = encoded_result.unwrap();
    assert_eq!(encoded.bytes, [240, 159, 141, 163]);
    assert_eq!(encoded.has_unmappable, false);
}

#[wasm_bindgen_test]
fn to_default_nfc() {
    let js_string = JsString::from("①②～パぱ");
    let encode_to = JsString::from(JsValue::UNDEFINED);
    let result = encode(&js_string, &encode_to);
    let encoded_result = from_value::<EncodeResult>(result.unwrap());
    let encoded = encoded_result.unwrap();
    assert_eq!(
        encoded.bytes,
        [226, 145, 160, 226, 145, 161, 239, 189, 158, 227, 131, 145, 227, 129, 177]
    );
    assert_eq!(encoded.has_unmappable, false);
}

#[wasm_bindgen_test]
fn to_utf8_mappable() {
    let js_string = JsString::from("ほげふが");
    let encode_to = JsString::from("utf-8");
    let result = encode(&js_string, &encode_to);
    let encoded_result = from_value::<EncodeResult>(result.unwrap());
    let encoded = encoded_result.unwrap();
    assert_eq!(
        encoded.bytes,
        [227, 129, 187, 227, 129, 146, 227, 129, 181, 227, 129, 140]
    );
    assert_eq!(encoded.has_unmappable, false);
}

#[wasm_bindgen_test]
fn to_utf8_emoji() {
    let js_string = JsString::from("🍣");
    let encode_to = JsString::from("utf-8");
    let result = encode(&js_string, &encode_to);
    let encoded_result = from_value::<EncodeResult>(result.unwrap());
    let encoded = encoded_result.unwrap();
    assert_eq!(encoded.bytes, [240, 159, 141, 163]);
    assert_eq!(encoded.has_unmappable, false);
}

#[wasm_bindgen_test]
fn to_utf8_nfc() {
    let js_string = JsString::from("①②～パぱ");
    let encode_to = JsString::from("utf-8");
    let result = encode(&js_string, &encode_to);
    let encoded_result = from_value::<EncodeResult>(result.unwrap());
    let encoded = encoded_result.unwrap();
    assert_eq!(
        encoded.bytes,
        [226, 145, 160, 226, 145, 161, 239, 189, 158, 227, 131, 145, 227, 129, 177]
    );
    assert_eq!(encoded.has_unmappable, false);
}

#[wasm_bindgen_test]
fn to_shiftjis_mappable() {
    let js_string = JsString::from("ほげふが");
    let encode_to = JsString::from("shift_jis");
    let result = encode(&js_string, &encode_to);
    let encoded_result = from_value::<EncodeResult>(result.unwrap());
    let encoded = encoded_result.unwrap();
    assert_eq!(encoded.bytes, [130, 217, 130, 176, 130, 211, 130, 170]);
    assert_eq!(encoded.has_unmappable, false);
}

#[wasm_bindgen_test]
fn to_shiftjis_nfc() {
    let js_string = JsString::from("①②～パぱ");
    let encode_to = JsString::from("shift_jis");
    let result = encode(&js_string, &encode_to);
    let encoded_result = from_value::<EncodeResult>(result.unwrap());
    let encoded = encoded_result.unwrap();
    assert_eq!(
        encoded.bytes,
        [135, 64, 135, 65, 129, 96, 131, 112, 130, 207]
    );
    assert_eq!(encoded.has_unmappable, false);
}

#[wasm_bindgen_test]
fn to_shiftjis_unmappble() {
    let js_string = JsString::from("🍣");
    let encode_to = JsString::from("shift_jis");
    let result = encode(&js_string, &encode_to);
    let encoded_result = from_value::<EncodeResult>(result.unwrap());
    let encoded = encoded_result.unwrap();
    // Python3.10.6:
    // [b for b in unicodedata.normalize("NFC", "🍣").encode("cp932", "xmlcharrefreplace")]
    assert_eq!(encoded.bytes, [38, 35, 49, 50, 55, 56, 52, 51, 59]);
    assert_eq!(encoded.has_unmappable, true);
}

#[wasm_bindgen_test]
fn to_eucjp_mappable() {
    let js_string = JsString::from("ほげふが");
    let encode_to = JsString::from("euc-jp");
    let result = encode(&js_string, &encode_to);
    let encoded_result = from_value::<EncodeResult>(result.unwrap());
    let encoded = encoded_result.unwrap();
    assert_eq!(encoded.bytes, [164, 219, 164, 178, 164, 213, 164, 172]);
    assert_eq!(encoded.has_unmappable, false);
}

#[wasm_bindgen_test]
fn to_eucjp_nfc() {
    let js_string = JsString::from("①②～パぱ");
    let encode_to = JsString::from("euc-jp");
    let result = encode(&js_string, &encode_to);
    let encoded_result = from_value::<EncodeResult>(result.unwrap());
    let encoded = encoded_result.unwrap();
    assert_eq!(
        encoded.bytes,
        [173, 161, 173, 162, 161, 193, 165, 209, 164, 209]
    );
    assert_eq!(encoded.has_unmappable, false);
}

#[wasm_bindgen_test]
fn to_eucjp_unmappble() {
    let js_string = JsString::from("🍣");
    let encode_to = JsString::from("euc-jp");
    let result = encode(&js_string, &encode_to);
    let encoded_result = from_value::<EncodeResult>(result.unwrap());
    let encoded = encoded_result.unwrap();
    // Python3.10.6:
    // [b for b in unicodedata.normalize("NFC", "🍣").encode("eucjp", "xmlcharrefreplace")]
    assert_eq!(encoded.bytes, [38, 35, 49, 50, 55, 56, 52, 51, 59]);
    assert_eq!(encoded.has_unmappable, true);
}
