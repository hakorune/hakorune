//! Legacy minimal pattern normalizer facade for composer-side shadow adopt.
//!
//! This keeps `shadow_adopt` imports stable while the underlying
//! `normalizer/pattern*.rs` modules are being phased out.

pub(super) use crate::mir::builder::control_flow::plan::normalizer::legacy_minimals::{
    normalize_escape_map_minimal, normalize_int_to_str_minimal,
    normalize_is_integer_minimal, normalize_skip_ws_minimal,
    normalize_split_lines_minimal, normalize_starts_with_minimal,
};
