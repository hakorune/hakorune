//! Legacy minimal normalizer facade (normalizer-internal window).
//!
//! Composer-side shadow adopt should consume this module instead of
//! reaching into `pattern*.rs` modules directly.

pub(in crate::mir::builder) use super::pattern_escape_map::normalize_escape_map_minimal;
pub(in crate::mir::builder) use super::pattern_int_to_str::normalize_int_to_str_minimal;
pub(in crate::mir::builder) use super::pattern_is_integer::normalize_is_integer_minimal;
pub(in crate::mir::builder) use super::pattern_split_lines::normalize_split_lines_minimal;
pub(in crate::mir::builder) use super::pattern_starts_with::normalize_starts_with_minimal;
