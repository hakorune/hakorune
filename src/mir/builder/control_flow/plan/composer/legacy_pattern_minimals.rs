//! Legacy minimal pattern normalizer facade for composer-side shadow adopt.
//!
//! This keeps `shadow_adopt` imports stable while the underlying
//! `normalizer/pattern*.rs` modules are being phased out.

pub(super) use crate::mir::builder::control_flow::plan::normalizer::pattern_escape_map::normalize_escape_map_minimal;
pub(super) use crate::mir::builder::control_flow::plan::normalizer::pattern_int_to_str::normalize_int_to_str_minimal;
pub(super) use crate::mir::builder::control_flow::plan::normalizer::pattern_is_integer::normalize_is_integer_minimal;
pub(super) use crate::mir::builder::control_flow::plan::normalizer::pattern_skip_ws::normalize_skip_ws_minimal;
pub(super) use crate::mir::builder::control_flow::plan::normalizer::pattern_split_lines::normalize_split_lines_minimal;
pub(super) use crate::mir::builder::control_flow::plan::normalizer::pattern_starts_with::normalize_starts_with_minimal;
