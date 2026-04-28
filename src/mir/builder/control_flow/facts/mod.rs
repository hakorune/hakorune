//! Top-level owner surface for control-flow facts.
//!
//! Facts-owned modules live here first. Plan-owned facts stay behind their
//! `plan::facts` owner instead of growing compatibility re-export surfaces here.

pub(crate) mod ast_feature_extractor;
pub(in crate::mir::builder) mod block_policies;
pub(in crate::mir::builder) mod canon;
pub(in crate::mir::builder) mod escape_shape_recognizer;
pub(in crate::mir::builder) mod expr_bool;
pub(in crate::mir::builder) mod expr_value;
pub(in crate::mir::builder) mod extractors;
pub(in crate::mir::builder) mod if_phi_join_facts;
pub(in crate::mir::builder) mod loop_bundle_resolver_v0;
pub(in crate::mir::builder) mod loop_bundle_resolver_v0_recipe_builder;
pub(in crate::mir::builder) mod loop_bundle_resolver_v0_shape_routes;
pub(in crate::mir::builder) mod loop_collect_using_entries_v0;
pub(in crate::mir::builder) mod loop_collect_using_entries_v0_recipe_builder;
pub(in crate::mir::builder) mod loop_collect_using_entries_v0_shape_routes;
pub(in crate::mir::builder) mod loop_cond_break_continue;
pub(in crate::mir::builder) mod loop_cond_continue_only;
pub(in crate::mir::builder) mod loop_cond_continue_with_return;
pub(in crate::mir::builder) mod loop_cond_return_in_body;
pub(in crate::mir::builder) mod loop_scan_methods_block_v0;
pub(in crate::mir::builder) mod loop_scan_methods_block_v0_recipe_builder;
pub(in crate::mir::builder) mod loop_scan_methods_block_v0_shape_routes;
pub(in crate::mir::builder) mod loop_scan_methods_v0;
pub(in crate::mir::builder) mod loop_scan_phi_vars_v0;
pub(in crate::mir::builder) mod loop_scan_phi_vars_v0_shape_routes;
pub(in crate::mir::builder) mod no_exit_block;
pub(in crate::mir::builder) mod route_shape_recognizers;
pub(in crate::mir::builder) mod scan_common_predicates;
pub(in crate::mir::builder) mod stmt_view;
pub(in crate::mir::builder) mod stmt_walk;

#[allow(unused_imports)]
pub(in crate::mir::builder) use if_phi_join_facts::{
    try_extract_if_phi_join_facts, IfPhiJoinFacts,
};
