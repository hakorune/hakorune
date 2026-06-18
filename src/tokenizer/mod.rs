/*!
 * Nyash Tokenizer compatibility facade.
 */

mod kinds;

pub use kinds::{Token, TokenType, TokenizeError};

/// Main-crate tokenizer facade.
///
/// The implementation lives in `hakorune-frontend-parser`; this wrapper keeps
/// the historical `crate::tokenizer::NyashTokenizer` API and installs the main
/// runtime host before tokenization.
pub struct NyashTokenizer {
    inner: hakorune_frontend_parser::tokenizer::NyashTokenizer,
}

impl NyashTokenizer {
    /// Create a tokenizer through the main crate host boundary.
    pub fn new(input: impl Into<String>) -> Self {
        crate::frontend_host::install_frontend_parser_host();
        Self {
            inner: hakorune_frontend_parser::tokenizer::NyashTokenizer::new(input),
        }
    }

    /// Tokenize the input.
    pub fn tokenize(&mut self) -> Result<Vec<Token>, TokenizeError> {
        self.inner.tokenize()
    }
}
