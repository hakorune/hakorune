//! Contextual-word recognition shared by the parser.
//!
//! Ownership marks (`copy`, `share`, `move`, `take`, `edit`, `release`) and
//! other contextual words stay ordinary identifiers unless the next lexical
//! item on the same source line continues the form. This is the one place that
//! decides "word here, accepted follower on the same line"; each caller passes
//! the followers its word accepts.

use super::common::ParserUtils;
use super::NyashParser;
use crate::tokenizer::TokenType;

impl NyashParser {
    /// True when the current token is the identifier `word`.
    pub(super) fn current_context_word(&self, word: &str) -> bool {
        matches!(&self.current_token().token_type, TokenType::IDENTIFIER(name) if name == word)
    }

    /// Returns the member of `words` spelled by the identifier at `index` when
    /// the following token sits on the same line and `next_ok` accepts it.
    pub(super) fn context_word_then(
        &self,
        index: usize,
        words: &[&'static str],
        next_ok: impl Fn(&TokenType) -> bool,
    ) -> Option<&'static str> {
        let head = self.tokens.get(index)?;
        let TokenType::IDENTIFIER(name) = &head.token_type else {
            return None;
        };
        let word = words.iter().copied().find(|word| name == word)?;
        self.token_on_line(index + 1, head.line, next_ok).then_some(word)
    }

    /// True when the token at `index` sits on `line` and `accepts` it.
    pub(super) fn token_on_line(
        &self,
        index: usize,
        line: usize,
        accepts: impl Fn(&TokenType) -> bool,
    ) -> bool {
        self.tokens
            .get(index)
            .is_some_and(|token| token.line == line && accepts(&token.token_type))
    }
}
