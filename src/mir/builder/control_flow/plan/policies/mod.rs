//! Compatibility wrapper for top-level cleanup policy helpers.

#[allow(unused_imports)]
pub(in crate::mir::builder) use crate::mir::builder::control_flow::cleanup::policies::{
    balanced_depth_scan_policy_box, cond_prelude_vocab, loop_simple_while_subset_policy,
    normalized_shadow_suffix_router_box, PolicyDecision,
};

pub(in crate::mir::builder) mod policies;
#[allow(unused_imports)]
pub(in crate::mir::builder) use policies::*;
