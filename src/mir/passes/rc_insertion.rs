//! Phase 29z P0: RC insertion pass - Minimal overwrite release
//!
//! Facade module. The implementation lives in `rc_insertion_helpers`.

pub use super::rc_insertion_helpers::RcInsertionStats;

#[cfg(feature = "rc-insertion-minimal")]
pub use super::rc_insertion_helpers::insert_rc_instructions;
