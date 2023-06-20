use wasm_bindgen::prelude::*;

mod from_term;
mod from_js_value;

#[wasm_bindgen]
pub fn unpack(bytes: &[u8]) -> JsValue {
    crate::Term::from_bytes(bytes).unwrap().into()
}

#[wasm_bindgen]
pub fn pack(value: JsValue) -> Vec<u8> {
    let term: crate::Term = value.into();
    term.to_bytes()
}
