use num_bigint::Sign;

pub fn value_to_term(value: wasm_bindgen::JsValue) -> erlpack::Term {
    if value.is_bigint() {
        let mut str = js_sys::BigInt::from(value)
            .to_string(16)
            .unwrap()
            .as_string()
            .unwrap();
        let sign = if str.chars().nth(0).unwrap() == '-' {
            str = str[1..].to_string();
            Sign::Minus
        } else {
            Sign::Plus
        };

        if str.len() & 1 != 0 {
            str = "0".to_string() + &str;
        }

        let mut bytes = Vec::with_capacity(str.len() / 2);
        for i in 0..bytes.capacity() {
            let index = i * 2;
            bytes.push(u8::from_str_radix(&str[index..index + 2], 16).unwrap());
        }

        erlpack::Term::Integer(num_bigint::BigInt::from_bytes_be(sign, &bytes))
    } else if let Some(flt) = value.as_f64() {
        erlpack::Term::Float(flt)
    } else if let Some(string) = value.as_string() {
        erlpack::Term::Atom(string)
    } else if value.is_symbol() {
        let string = js_sys::Symbol::from(value).to_string().as_string().unwrap();
        let str = &string[7..string.len() - 1]; // Symbol(x)
        erlpack::Term::Atom(str.to_string())
    } else if value.is_array() {
        let array = js_sys::Array::from(&value);
        let len = array.length();
        let mut elems = Vec::with_capacity(len as usize);
        for i in 0..len {
            elems.push(value_to_term(array.at(i as i32)))
        }

        erlpack::Term::Tuple(elems)
    } else if value.is_function() || value.is_undefined() || value.is_null() {
        erlpack::Term::Nil
    } else if value.is_object() {
        let obj = js_sys::Object::from(value);
        let keys = js_sys::Object::keys(&obj);
        let values = js_sys::Object::values(&obj);
        assert_eq!(keys.length(), values.length());

        let len = keys.length();
        let mut elems = Vec::with_capacity(len as usize);
        for i in 0..len {
            elems.push((
                value_to_term(keys.at(i as i32)),
                value_to_term(values.at(i as i32)),
            ));
        }

        erlpack::Term::Map(elems)
    } else if value.is_truthy() {
        erlpack::Term::Atom("true".to_string())
    } else if value.is_falsy() {
        erlpack::Term::Atom("false".to_string())
    } else {
        panic!("unhandlable type");
    }
}
