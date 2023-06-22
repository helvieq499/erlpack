#![warn(clippy::cargo, clippy::nursery, clippy::pedantic)]
#![allow(clippy::missing_errors_doc, clippy::missing_panics_doc)]

mod term;
pub use term::Term;

mod error;
pub use error::{Error, Result};

mod decoder;
mod encoder;

pub fn unpack(bytes: &[u8]) -> Result<Term> {
    Term::from_bytes(bytes)
}

#[must_use]
pub fn pack(term: &Term) -> Result<Vec<u8>> {
    term.to_bytes()
}
