//! Language v1 grammar-profile checks shared by active Rust parser entries.

use hakorune_frontend_grammar::contract::{find_row, GrammarProfile, GrammarStatus};

use crate::parser::ParseError;
use crate::tokenizer::TokenType;

pub(crate) fn require_semantic_entry(
    row_id: &str,
    profile: GrammarProfile,
    found: TokenType,
    line: usize,
) -> Result<(), ParseError> {
    let row = find_row(row_id, profile).ok_or_else(|| ParseError::UnexpectedToken {
        found: found.clone(),
        expected: "[freeze:contract][parser/registry_row_missing] grammar row is required"
            .to_string(),
        line,
    })?;

    match row.status {
        GrammarStatus::Canonical | GrammarStatus::CompatibilityOnly => Ok(()),
        GrammarStatus::Reserved | GrammarStatus::Rejected => Err(ParseError::UnexpectedToken {
            found,
            expected: format!(
                "[freeze:contract][{}] grammar profile rejects `{}`",
                row.stable_reject_tag, row.spelling_id
            ),
            line,
        }),
    }
}
