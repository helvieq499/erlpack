use byteorder::{BigEndian, ReadBytesExt};
use num_bigint::{BigInt, Sign};
use std::io::{Cursor, Read};

use crate::{Error, Result, Term};

pub struct Reader<'a> {
    pub data: Cursor<&'a [u8]>,
    pub version: Option<u8>,
}

impl<'a> Reader<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self {
            data: Cursor::new(bytes),
            version: None,
        }
    }

    pub fn read(&mut self) -> Result<Term> {
        match self.data.read_u8()? {
            131 => self.version = Some(131),
            x => return Err(Error::UnknownFormatVersion(x)),
        }

        self.read_term()
    }

    fn read_term(&mut self) -> Result<Term> {
        match self.data.read_u8()? {
            97 => Ok(Term::Integer(BigInt::from(self.data.read_u8()?))),
            98 => Ok(Term::Integer(BigInt::from(
                self.data.read_u32::<BigEndian>()?,
            ))),
            110 => {
                let size = self.data.read_u8()?;
                let sign = sign_bit_to_sign(self.data.read_u8()?);
                let buf = self.read_buffer(size)?;
                Ok(Term::Integer(BigInt::from_bytes_le(sign, &buf)))
            }
            111 => {
                let size = self.data.read_u32::<BigEndian>()?;
                let sign = sign_bit_to_sign(self.data.read_u8()?);
                let buf = self.read_buffer(size as usize)?;
                Ok(Term::Integer(BigInt::from_bytes_le(sign, &buf)))
            }

            70 => Ok(Term::Float(self.data.read_f64::<BigEndian>()?.into())),
            // TODO: type 99, FLOAT_EXT
            115 | 119 => {
                let size = self.data.read_u8()?;
                Ok(Term::Atom(
                    std::str::from_utf8(&self.read_buffer(size)?)?.to_string(),
                ))
            }
            100 | 118 => {
                let size = self.data.read_u16::<BigEndian>()?;
                Ok(Term::Atom(
                    std::str::from_utf8(&self.read_buffer(size as usize)?)?.to_string(),
                ))
            }

            104 => {
                let size = self.data.read_u8()?;
                let mut elements = Vec::with_capacity(size as usize);
                for _ in 0..size {
                    elements.push(self.read_term()?);
                }
                Ok(Term::Tuple(elements))
            }
            105 => {
                let size = self.data.read_u32::<BigEndian>()?;
                let mut elements = Vec::with_capacity(size as usize);
                for _ in 0..size {
                    elements.push(self.read_term()?);
                }
                Ok(Term::Tuple(elements))
            }

            116 => {
                let size = self.data.read_u32::<BigEndian>()?;
                let mut elements = Vec::with_capacity(size as usize);
                for _ in 0..size {
                    let key = self.read_term()?;
                    let val = self.read_term()?;
                    elements.push((key, val));
                }

                Ok(Term::Map(elements))
            }

            106 => Ok(Term::Nil),
            108 => {
                let size = self.data.read_u32::<BigEndian>()?;
                let mut elements = Vec::new();
                for _ in 0..size {
                    elements.push(self.read_term()?);
                }

                Ok(Term::List(elements, Box::new(self.read_term()?)))
            }

            107 => {
                let size = self.data.read_u16::<BigEndian>()?;
                Ok(Term::Binary(self.read_buffer(size)?))
            }
            109 => {
                let size = self.data.read_u32::<BigEndian>()?;
                Ok(Term::Binary(self.read_buffer(size as usize)?))
            }

            x => return Err(Error::UnknownTermType(x)),
        }
    }

    fn read_buffer<T: Into<usize>>(&mut self, size: T) -> Result<Vec<u8>> {
        let mut buf = vec![0; size.into()];
        self.data.read_exact(&mut buf)?;
        Ok(buf)
    }
}

fn sign_bit_to_sign(bit: u8) -> Sign {
    if bit == 1 {
        Sign::Minus
    } else {
        Sign::Plus
    }
}
