//! Legacy minimal pattern normalizer facade for composer-side shadow adopt.
//!
//! This keeps `shadow_adopt` imports stable while the underlying
//! `normalizer/pattern*.rs` modules are being phased out.

pub(super) use super::legacy_minimals::escape_map::normalize_escape_map_minimal;
pub(super) use super::legacy_minimals::int_to_str::normalize_int_to_str_minimal;
pub(super) use super::legacy_minimals::is_integer::normalize_is_integer_minimal;
pub(super) use super::legacy_minimals::skip_ws::normalize_skip_ws_minimal;
pub(super) use super::legacy_minimals::split_lines::normalize_split_lines_minimal;
pub(super) use super::legacy_minimals::starts_with::normalize_starts_with_minimal;
