//! Top-level owner surface for cleanup-side policy helpers.
//!
//! Policy boxes here own cleanup-side route decisions and small vocabularies.
//! Callers should import these owner modules directly instead of regrowing
//! plan-side compatibility shelves.

pub use crate::mir::policies::PolicyDecision;

pub(in crate::mir::builder) mod balanced_depth_scan_policy_box;
pub(in crate::mir::builder) mod cond_prelude_vocab;
pub(in crate::mir::builder) mod loop_simple_while_subset_policy;
