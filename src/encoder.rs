use byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};
use num_bigint::Sign;

pub struct Encoder {
    pub data: Vec<u8>,
}

impl Encoder {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn encode(mut self, term: &crate::Term) -> crate::Result<Vec<u8>> {
        self.data.write_u8(131)?;
        self.encode_term(term)?;

        Ok(self.data)
    }

    fn encode_term(&mut self, term: &crate::Term) -> crate::Result<()> {
        match term {
            crate::Term::Integer(num) => {
                let (sign, mut bytes) = num.to_bytes_le();

                if sign != Sign::Minus && bytes.len() == 1 {
                    self.data.write_u8(97)?;
                    self.data.write_u8(bytes[0])?
                } else if bytes.len() <= 4 {
                    let pad_len = 4 - bytes.len();
                    if pad_len > 0 {
                        let mut new_bytes = vec![0; pad_len];
                        new_bytes.append(&mut bytes);
                        bytes = new_bytes;
                    }

                    self.data.write_u8(98)?;
                    self.data.write_i32::<BigEndian>(
                        std::io::Cursor::new(bytes).read_i32::<LittleEndian>()?,
                    )?;
                } else if bytes.len() <= u8::MAX as usize {
                    self.data.write_u8(110)?;
                    self.data.write_u8(bytes.len() as u8)?;
                    self.data.write_u8(sign_to_sign_bit(sign))?;
                    self.data.append(&mut bytes);
                } else {
                    self.data.write_u8(111)?;
                    self.data.write_u32::<BigEndian>(bytes.len() as u32)?;
                    self.data.write_u8(sign_to_sign_bit(sign))?;
                    self.data.append(&mut bytes);
                }
            }
            crate::Term::Float(flt) => {
                self.data.write_u8(70)?;
                self.data.write_f64::<BigEndian>(*flt)?;
            }
            crate::Term::Atom(str) => {
                if str.len() < u8::MAX as usize {
                    self.data.write_u8(119)?;
                    self.data.write_u8(str.len() as u8)?;
                    self.data.extend_from_slice(str.as_bytes());
                } else {
                    self.data.write_u8(118)?;
                    self.data.write_u16::<BigEndian>(str.len() as u16)?;
                    self.data.extend_from_slice(str.as_bytes());
                }
            }
            crate::Term::Tuple(elems) => {
                if elems.len() <= u8::MAX as usize {
                    self.data.write_u8(104)?;
                    self.data.write_u8(elems.len() as u8)?;
                    for elem in elems {
                        self.encode_term(elem)?;
                    }
                } else {
                    self.data.write_u8(105)?;
                    self.data.write_u32::<BigEndian>(elems.len() as u32)?;
                    for elem in elems {
                        self.encode_term(elem)?;
                    }
                }
            }
            crate::Term::Nil => {
                self.data.write_u8(106)?;
            }
            crate::Term::List(elems, tail) => {
                self.data.write_u8(108)?;
                self.data.write_u32::<BigEndian>(elems.len() as u32)?;
                for elem in elems {
                    self.encode_term(elem)?;
                }
                self.encode_term(tail)?;
            }
            crate::Term::Map(elems) => {
                self.data.write_u8(116)?;
                self.data.write_u32::<BigEndian>(elems.len() as u32)?;
                for (key, value) in elems {
                    self.encode_term(key)?;
                    self.encode_term(value)?;
                }
            }
            crate::Term::Binary(bytes) => {
                self.data.write_u8(109)?;
                self.data.write_u32::<BigEndian>(bytes.len() as u32)?;
                self.data.extend_from_slice(&bytes);
            }
        }

        Ok(())
    }
}

const fn sign_to_sign_bit(sign: Sign) -> u8 {
    match sign {
        Sign::Minus => 1,
        _ => 0,
    }
}
