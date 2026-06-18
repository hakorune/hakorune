//! Tokenizer compatibility root scaffold.
//!
//! Active tokenizer implementation still lives in the main crate. This module
//! exists so future file-move rows can preserve `crate::tokenizer::*` paths
//! inside the extracted crate.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TokenizerBoundary;

impl TokenizerBoundary {
    pub const fn name(self) -> &'static str {
        "tokenizer"
    }
}
