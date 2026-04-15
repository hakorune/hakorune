//! Top-level owner surface for control-flow facts.
//!
//! Facts-owned modules should live here first. Remaining `plan::facts` residue
//! stays behind a named facts-local forwarding module so non-`plan/` callers can
//! depend on this owner surface without inheriting broad `plan::facts` imports.

pub(crate) mod ast_feature_extractor;
pub(in crate::mir::builder) mod block_policies;
pub(in crate::mir::builder) mod canon;
pub(in crate::mir::builder) mod escape_shape_recognizer;
pub(in crate::mir::builder) mod expr_bool;
pub(in crate::mir::builder) mod expr_value;
pub(in crate::mir::builder) mod extractors;
pub(in crate::mir::builder) mod if_phi_join_facts;
pub(in crate::mir::builder) mod loop_bundle_resolver_v0;
pub(in crate::mir::builder) mod loop_bundle_resolver_v0_helpers;
pub(in crate::mir::builder) mod loop_bundle_resolver_v0_recipe_builder;
pub(in crate::mir::builder) mod loop_bundle_resolver_v0_shape_routes;
pub(in crate::mir::builder) mod loop_collect_using_entries_v0;
pub(in crate::mir::builder) mod loop_collect_using_entries_v0_helpers;
pub(in crate::mir::builder) mod loop_collect_using_entries_v0_recipe_builder;
pub(in crate::mir::builder) mod loop_collect_using_entries_v0_shape_routes;
pub(in crate::mir::builder) mod loop_scan_methods_block_v0;
pub(in crate::mir::builder) mod loop_scan_methods_block_v0_helpers;
pub(in crate::mir::builder) mod loop_scan_methods_block_v0_recipe_builder;
pub(in crate::mir::builder) mod loop_scan_methods_block_v0_shape_routes;
pub(in crate::mir::builder) mod loop_scan_methods_v0;
pub(in crate::mir::builder) mod no_exit_block;
mod plan_residue;
pub(in crate::mir::builder) mod route_shape_recognizers;
pub(in crate::mir::builder) mod stmt_view;
pub(in crate::mir::builder) mod stmt_walk;

#[allow(unused_imports)]
pub(in crate::mir::builder) use if_phi_join_facts::{
    try_extract_if_phi_join_facts, IfPhiJoinFacts,
};
#[allow(unused_imports)]
pub(in crate::mir::builder) use plan_residue::*;
