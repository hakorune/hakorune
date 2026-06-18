//! Tokenizer-local environment facade.
//!
//! Crate-split preparation rule: tokenizer modules should depend on this
//! facade instead of importing `crate::config::env` directly.

#[inline]
pub(crate) fn grammar_diff() -> bool {
    crate::config::env::grammar_diff()
}

#[inline]
pub(crate) fn parser_allow_semicolon() -> bool {
    crate::config::env::parser_allow_semicolon()
}

#[inline]
pub(crate) fn parser_decode_unicode() -> bool {
    crate::config::env::parser_decode_unicode()
}

#[inline]
pub(crate) fn parser_metadata_annotations_enabled() -> bool {
    crate::config::env::parser_metadata_annotations_enabled()
}

#[inline]
pub(crate) fn parser_stage3_enabled() -> bool {
    crate::config::env::parser_stage3_enabled()
}

#[inline]
pub(crate) fn strict_12_7() -> bool {
    crate::config::env::strict_12_7()
}

#[inline]
pub(crate) fn tok_trace() -> bool {
    crate::config::env::tok_trace()
}
