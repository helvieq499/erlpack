use std::io::Cursor;
use byteorder::ReadBytesExt;

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

    pub fn read(&mut self) -> crate::Result<crate::Term> {
        match self.data.read_u8()? {
            131 => self.version = Some(131),
            x => return Err(crate::Error::UnknownFormatVersion(x)),
        }

        self.read_term()
    }

    fn read_term(&mut self) -> crate::Result<crate::Term> {
        todo!()
    }
}
