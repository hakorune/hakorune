//! Top-level owner surface for cleanup-side policy helpers.
//!
//! During folderization, cleanup-side policy support can move here first.
//! Route-local policies that still depend on `plan/` stay behind the compat layer.

pub use crate::mir::policies::PolicyDecision;

pub(in crate::mir::builder) mod balanced_depth_scan_policy;
pub(in crate::mir::builder) mod balanced_depth_scan_policy_box;
pub(in crate::mir::builder) mod cond_prelude_vocab;
pub(in crate::mir::builder) mod loop_simple_while_subset_policy;
pub(in crate::mir::builder) mod normalized_shadow_suffix_router_box;
pub(in crate::mir::builder) mod post_loop_early_return_plan;
