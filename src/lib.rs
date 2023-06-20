#![warn(clippy::cargo, clippy::nursery, clippy::pedantic)]

mod term;
pub use term::Term;

mod error;
pub use error::{Error, Result};

pub fn unpack(bytes: &[u8]) -> Result<Term> {
    todo!()
}

pub fn pack(term: Term) -> Vec<u8> {
    todo!()
}
