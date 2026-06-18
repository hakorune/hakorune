//! Tokenizer implementation root.

mod cursor;
mod engine;
mod env;
pub mod kinds;
mod lex_ident;
mod lex_number;
mod lex_string;
mod log;
mod whitespace;

pub use kinds::{Token, TokenType, TokenizeError};

/// Nyash tokenizer.
pub struct NyashTokenizer {
    pub(crate) input: Vec<char>,
    pub(crate) position: usize,
    pub(crate) line: usize,
    pub(crate) column: usize,
}
