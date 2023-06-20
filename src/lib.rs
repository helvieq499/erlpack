#![warn(clippy::cargo, clippy::nursery, clippy::pedantic)]

mod term;
pub use term::Term;

mod error;
pub use error::{Error, Result};

mod decoder;

#[cfg(target_arch = "wasm32")]
mod wasm;

#[cfg(not(target_arch = "wasm32"))]
pub fn unpack(bytes: &[u8]) -> Result<Term> {
    Term::from_bytes(bytes)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn pack(term: Term) -> Vec<u8> {
    term.to_bytes()
}
