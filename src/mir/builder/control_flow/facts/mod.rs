//! Owner-facing facts infrastructure for control-flow lowering.
//!
//! Facts modules live here first, but owner-only helpers stay sealed instead of
//! re-growing compatibility surfaces after the facade cleanup. Plan-resident
//! facts continue to live behind `plan::facts`.

pub(crate) mod ast_feature_extractor;
pub(in crate::mir::builder) mod block_policies;
pub(in crate::mir::builder) mod canon;
pub(in crate::mir::builder) mod escape_shape_recognizer;
pub(in crate::mir::builder) mod expr_bool;
mod expr_value;
pub(in crate::mir::builder) mod extractors;
mod if_phi_join_facts;
pub(in crate::mir::builder) mod loop_bundle_resolver_v0;
mod loop_bundle_resolver_v0_recipe_builder;
mod loop_bundle_resolver_v0_shape_routes;
pub(in crate::mir::builder) mod loop_collect_using_entries_v0;
mod loop_collect_using_entries_v0_recipe_builder;
mod loop_collect_using_entries_v0_shape_routes;
pub(in crate::mir::builder) mod loop_cond_break_continue;
pub(in crate::mir::builder) mod loop_cond_continue_only;
pub(in crate::mir::builder) mod loop_cond_continue_with_return;
pub(in crate::mir::builder) mod loop_cond_return_in_body;
pub(in crate::mir::builder) mod loop_scan_methods_block_v0;
mod loop_scan_methods_block_v0_recipe_builder;
mod loop_scan_methods_block_v0_shape_routes;
pub(in crate::mir::builder) mod loop_scan_methods_v0;
pub(in crate::mir::builder) mod loop_scan_phi_vars_v0;
mod loop_scan_phi_vars_v0_shape_routes;
pub(in crate::mir::builder) mod no_exit_block;
pub(in crate::mir::builder) mod route_shape_recognizers;
pub(in crate::mir::builder) mod scan_common_predicates;
pub(in crate::mir::builder) mod stmt_view;
pub(in crate::mir::builder) mod stmt_walk;

#[allow(unused_imports)]
pub(in crate::mir::builder) use if_phi_join_facts::{
    try_extract_if_phi_join_facts, IfPhiJoinFacts,
};
