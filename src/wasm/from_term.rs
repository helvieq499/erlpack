use wasm_bindgen::JsValue;

impl From<crate::Term> for JsValue {
    fn from(term: crate::Term) -> JsValue {
        match term {
            crate::Term::Integer(bigint) => JsValue::bigint_from_str(&bigint.to_string()),
            crate::Term::Float(flt) => JsValue::from_f64(flt),
            crate::Term::Atom(str) => JsValue::from_str(&str),
            crate::Term::Tuple(elems) => {
                let array = js_sys::Array::new_with_length(elems.len() as u32);
                for elem in elems {
                    array.push(&elem.into());
                }
                array.into()
            }
            crate::Term::List(elems, tail) => {
                let array = js_sys::Array::new_with_length(elems.len() as u32);
                for elem in elems {
                    array.push(&elem.into());
                }
                
                let tail_js: JsValue = (*tail).into();
                array.push(&tail_js);
                array.into()
            }
            crate::Term::Nil => JsValue::UNDEFINED,
            crate::Term::Map(elems) => {
                let map = js_sys::Map::new();
                for (key, value) in elems {
                    map.set(&key.into(), &value.into());
                }
                map.into()
            }
            crate::Term::Binary(bytes) => {
                if let Ok(str) = std::str::from_utf8(&bytes) {
                    JsValue::from_str(str)
                } else {
                    let array = js_sys::Array::new_with_length(bytes.len() as u32);
                    for byte in bytes {
                        array.push(&JsValue::from_f64(byte as f64));
                    }
                    array.into()
                }
            }
        }
    }
}
