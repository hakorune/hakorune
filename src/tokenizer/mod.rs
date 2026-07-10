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
        Self::with_grammar_profile(
            input,
            hakorune_frontend_parser::parser::GrammarProfile::Canonical,
        )
    }

    pub fn with_grammar_profile(
        input: impl Into<String>,
        grammar_profile: hakorune_frontend_parser::parser::GrammarProfile,
    ) -> Self {
        crate::frontend_host::install_frontend_parser_host();
        Self {
            inner: hakorune_frontend_parser::tokenizer::NyashTokenizer::with_grammar_profile(
                input,
                grammar_profile,
            ),
        }
    }

    pub fn grammar_profile(&self) -> hakorune_frontend_parser::parser::GrammarProfile {
        self.inner.grammar_profile()
    }

    /// Tokenize the input.
    pub fn tokenize(&mut self) -> Result<Vec<Token>, TokenizeError> {
        self.inner.tokenize()
    }
}
