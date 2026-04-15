//! loop_bundle_resolver_v0: bundle/using resolver loop plan (BoxCount).
//!
//! Accepts a single one-shape `loop(i < n)` style loop where:
//! - the loop step is `i = next_i` (var-to-var),
//! - `next_i` is declared as a loop-local,
//! - the body contains a nested `return` (exit-bearing), so generic_loop_v* doesn't match.
//!
//! This is intended to unblock selfhost Stage-B bundling/using code paths under
//! `strict/dev + planner_required` without widening release defaults.

mod pipeline;

#[allow(unused_imports)]
pub(in crate::mir::builder) use crate::mir::builder::control_flow::facts::loop_bundle_resolver_v0::{
    try_extract_loop_bundle_resolver_v0_facts, LoopBundleResolverV0Facts,
};
pub(in crate::mir::builder) use pipeline::lower_loop_bundle_resolver_v0;
#[allow(unused_imports)]
pub(in crate::mir::builder) use crate::mir::builder::control_flow::recipes::loop_bundle_resolver_v0::LoopBundleResolverV0Recipe;
