use wasm_bindgen::JsValue;

pub fn term_to_value(term: erlpack::Term) -> JsValue {
    match term {
        erlpack::Term::Integer(bigint) => JsValue::bigint_from_str(&bigint.to_string()),
        erlpack::Term::Float(flt) => JsValue::from_f64(flt),
        erlpack::Term::Atom(str) => JsValue::from_str(&str),
        erlpack::Term::Tuple(elems) => {
            let array = js_sys::Array::new_with_length(elems.len() as u32);
            for elem in elems {
                array.push(&term_to_value(elem));
            }
            array.into()
        }
        erlpack::Term::List(elems, tail) => {
            let array = js_sys::Array::new_with_length(elems.len() as u32);
            for elem in elems {
                array.push(&term_to_value(elem));
            }

            let tail_js: JsValue = term_to_value(*tail);
            array.push(&tail_js);
            array.into()
        }
        erlpack::Term::Nil => JsValue::UNDEFINED,
        erlpack::Term::Map(elems) => {
            let map = js_sys::Map::new();
            for (key, value) in elems {
                map.set(&term_to_value(key), &term_to_value(value));
            }
            map.into()
        }
        erlpack::Term::Binary(bytes) => {
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
