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
