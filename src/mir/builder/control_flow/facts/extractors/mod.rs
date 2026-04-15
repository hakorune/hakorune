//! Top-level descriptive extractors.
//!
//! Route-local helpers that still depend on `plan::canon` stay behind the
//! compatibility layer until their canon support moves.

pub(in crate::mir::builder) mod common_helpers;
pub(in crate::mir::builder) mod loop_simple_while;
