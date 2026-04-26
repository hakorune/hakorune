//! Top-level owner surface for cleanup-side policy helpers.
//!
//! During folderization, cleanup-side policy support can move here first.
//! Route-local policies that still depend on `plan/` stay behind the compat layer.

pub use crate::mir::policies::PolicyDecision;

pub(in crate::mir::builder) mod balanced_depth_scan_policy_box;
pub(in crate::mir::builder) mod body_local_derived_slot;
pub(in crate::mir::builder) mod cond_prelude_vocab;
pub(in crate::mir::builder) mod loop_simple_while_subset_policy;
pub(in crate::mir::builder) mod loop_true_read_digits_policy;
pub(in crate::mir::builder) mod p5b_escape_derived_policy;
pub(in crate::mir::builder) mod read_digits_break_condition_box;
pub(in crate::mir::builder) mod trim_policy;
