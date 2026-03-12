//! Tokenizer-related environment flags
//!
//! Groups NYASH_TOK_* and tokenizer-facing parser toggles.

use crate::config::env::env_bool;

/// Tokenizer trace (NYASH_TOK_TRACE=1).
pub fn tok_trace() -> bool {
    env_bool("NYASH_TOK_TRACE")
}

/// Tokenizer vs grammar diff trace (NYASH_GRAMMAR_DIFF=1).
pub fn grammar_diff() -> bool {
    env_bool("NYASH_GRAMMAR_DIFF")
}

/// Allow optional semicolon separator (default ON).
/// Disable with NYASH_PARSER_ALLOW_SEMICOLON=0|false|off.
pub fn parser_allow_semicolon() -> bool {
    match std::env::var("NYASH_PARSER_ALLOW_SEMICOLON")
        .ok()
        .as_deref()
    {
        Some("0") | Some("false") | Some("off") => false,
        Some(_) => true,
        None => true,
    }
}

/// Strict 12.7 tokenizer mode (NYASH_STRICT_12_7=1).
pub fn strict_12_7() -> bool {
    env_bool("NYASH_STRICT_12_7")
}
