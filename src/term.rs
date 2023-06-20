use std::collections::HashMap;

pub enum Term {
    Integer(Vec<u8>),
    Float(f64),
    Atom(String),
    Tuple(Vec<Term>),
    Map(HashMap<Term, Term>),
    Nil,
    List(Vec<Term>, Option<Box<Term>>),
    Binary(Vec<u8>),
}

impl Term {
    pub fn from_bytes(bytes: &[u8]) -> crate::Result<Self> {
        crate::reader::Reader::new(bytes).read()
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        todo!();
    }
}
