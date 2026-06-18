//! Tokenizer-local environment facade.
//!
//! Crate-split preparation rule: tokenizer modules should depend on this
//! facade instead of importing main-crate configuration directly.

fn env_bool(key: &str) -> bool {
    match std::env::var(key).ok() {
        Some(value) => {
            let value = value.to_ascii_lowercase();
            value == "1" || value == "true" || value == "on"
        }
        None => false,
    }
}

fn env_flag(var: &str) -> Option<bool> {
    std::env::var(var).ok().map(|value| {
        let value = value.to_ascii_lowercase();
        value == "1" || value == "true" || value == "on"
    })
}

#[inline]
pub(crate) fn grammar_diff() -> bool {
    env_bool("NYASH_GRAMMAR_DIFF")
}

#[inline]
pub(crate) fn parser_allow_semicolon() -> bool {
    match std::env::var("NYASH_PARSER_ALLOW_SEMICOLON")
        .ok()
        .as_deref()
    {
        Some("0") | Some("false") | Some("off") => false,
        Some(_) => true,
        None => true,
    }
}

#[inline]
pub(crate) fn parser_decode_unicode() -> bool {
    env_flag("HAKO_PARSER_DECODE_UNICODE")
        .or_else(|| env_flag("NYASH_PARSER_DECODE_UNICODE"))
        .unwrap_or(false)
}

#[inline]
pub(crate) fn parser_metadata_annotations_enabled() -> bool {
    true
}

#[inline]
pub(crate) fn parser_stage3_enabled() -> bool {
    crate::frontend_env::parser_stage3_enabled()
}

#[inline]
pub(crate) fn strict_12_7() -> bool {
    env_bool("NYASH_STRICT_12_7")
}

#[inline]
pub(crate) fn tok_trace() -> bool {
    env_bool("NYASH_TOK_TRACE")
}
