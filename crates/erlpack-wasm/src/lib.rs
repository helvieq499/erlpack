use wasm_bindgen::prelude::*;

mod term_to_value;
mod value_to_term;

#[wasm_bindgen]
pub fn unpack(bytes: &[u8]) -> JsValue {
    console_error_panic_hook::set_once();
    term_to_value::term_to_value(erlpack::Term::from_bytes(bytes).unwrap())
}

#[wasm_bindgen]
pub fn pack(value: JsValue) -> Vec<u8> {
    console_error_panic_hook::set_once();
    value_to_term::value_to_term(value).to_bytes().unwrap()
}
