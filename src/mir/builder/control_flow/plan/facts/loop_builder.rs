//! Main builder functions for LoopFacts

use crate::ast::ASTNode;
use std::collections::BTreeMap;

use super::scan_shapes::{scan_condition_observation, ConditionShape, StepShape};
use super::skeleton_facts::try_extract_loop_skeleton_facts;
use super::feature_facts::try_extract_loop_feature_facts;
use super::loop_simple_while_facts::try_extract_loop_simple_while_facts;
use super::loop_char_map_facts::try_extract_loop_char_map_facts;
use super::loop_array_join_facts::try_extract_loop_array_join_facts;
use super::string_is_integer_facts::try_extract_string_is_integer_facts;
use super::starts_with_facts::try_extract_starts_with_facts;
use super::int_to_str_facts::try_extract_int_to_str_facts;
use super::escape_map_facts::try_extract_escape_map_facts;
use super::split_lines_facts::try_extract_split_lines_facts;
use super::skip_whitespace_facts::try_extract_skip_whitespace_facts;
use crate::mir::builder::control_flow::plan::generic_loop::facts::extract::{
    has_generic_loop_v1_recipe_hint, try_extract_generic_loop_v0_facts,
    try_extract_generic_loop_v1_facts,
};
use super::loop_true_early_exit_facts::try_extract_loop_true_early_exit_facts;
use super::{try_extract_if_phi_join_facts, try_extract_loop_continue_only_facts};
use crate::mir::builder::control_flow::plan::loop_true_break_continue::facts::try_extract_loop_true_break_continue_facts;
use crate::mir::builder::control_flow::plan::loop_cond::break_continue_types::{
    LoopCondBreakAcceptKind, LoopCondBreakContinueFacts,
};
use crate::mir::builder::control_flow::plan::loop_cond::break_continue_entry::{
    try_extract_loop_cond_break_continue_facts,
    try_extract_loop_cond_break_continue_facts_with_limit,
};
use crate::mir::builder::control_flow::plan::loop_cond::break_continue_recipe::LoopCondBreakContinueItem;
use super::nested_loop_profile::CLUSTER_PROFILES;
use crate::mir::builder::control_flow::plan::loop_cond::continue_only_facts::try_extract_loop_cond_continue_only_facts;
use crate::mir::builder::control_flow::plan::loop_cond::continue_with_return_facts::try_extract_loop_cond_continue_with_return_facts;
use crate::mir::builder::control_flow::plan::loop_cond::return_in_body_facts::try_extract_loop_cond_return_in_body_facts;
use crate::mir::builder::control_flow::plan::loop_scan_v0::try_extract_loop_scan_v0_facts;
use crate::mir::builder::control_flow::plan::loop_scan_methods_v0::try_extract_loop_scan_methods_v0_facts;
use crate::mir::builder::control_flow::plan::loop_scan_methods_block_v0::try_extract_loop_scan_methods_block_v0_facts;
use crate::mir::builder::control_flow::plan::loop_scan_phi_vars_v0::try_extract_loop_scan_phi_vars_v0_facts;
use crate::mir::builder::control_flow::plan::loop_collect_using_entries_v0::try_extract_loop_collect_using_entries_v0_facts;
use crate::mir::builder::control_flow::plan::loop_bundle_resolver_v0::try_extract_loop_bundle_resolver_v0_facts;
use super::nested_loop_minimal_facts::try_extract_nested_loop_minimal_facts;
use super::bool_predicate_scan_facts::try_extract_bool_predicate_scan_facts;
use super::accum_const_loop_facts::try_extract_accum_const_loop_facts;
use super::loop_break_core::try_extract_loop_break_facts;
use super::loop_break_body_local_facts::try_extract_loop_break_body_local_facts;
use super::stmt_view::flatten_scope_boxes;
use crate::mir::builder::control_flow::plan::planner::{Freeze, PlannerContext};

use super::loop_types::LoopFacts;
use super::loop_condition_shape::try_extract_condition_shape;
use super::loop_step_shape::try_extract_step_shape;
use super::loop_scan_with_init::try_extract_scan_with_init_facts;
use super::loop_split_scan::try_extract_split_scan_facts;

pub(in crate::mir::builder) fn try_build_loop_facts(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Result<Option<LoopFacts>, Freeze> {
    try_build_loop_facts_inner(condition, body)
}

pub(in crate::mir::builder) fn try_build_loop_facts_with_ctx(
    _ctx: &PlannerContext,
    condition: &ASTNode,
    body: &[ASTNode],
) -> Result<Option<LoopFacts>, Freeze> {
    try_build_loop_facts_inner(condition, body)
}

fn try_build_loop_facts_inner(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Result<Option<LoopFacts>, Freeze> {
    // Phase 29ai P4/P7: keep Facts conservative; only return Some when we can
    // build a concrete route fact set (no guesses / no hardcoded names).
    //
    // NOTE: Some BoxCount routes intentionally match on `ScopeBox`/block wrapper
    // boundaries (analysis-only observation). Those must run on the original body,
    // before `flatten_scope_boxes()` strips wrapper nodes.
    let loop_scan_methods_block_v0 =
        try_extract_loop_scan_methods_block_v0_facts(condition, body)?;
    let flat_body = flatten_scope_boxes(body);
    let body = flat_body.as_slice();

    let condition_shape =
        try_extract_condition_shape(condition)?.unwrap_or(ConditionShape::Unknown);
    let step_shape = try_extract_step_shape(body)?.unwrap_or(StepShape::Unknown);
    let observation = scan_condition_observation(&condition_shape, &step_shape);
    let scan_with_init =
        try_extract_scan_with_init_facts(condition, body, &condition_shape, &step_shape)?;
    let split_scan = try_extract_split_scan_facts(condition, body)?;
    let loop_simple_while = try_extract_loop_simple_while_facts(condition, body)?;
    let loop_char_map = try_extract_loop_char_map_facts(condition, body, &observation)?;
    let loop_array_join = try_extract_loop_array_join_facts(condition, body, &observation)?;
    let string_is_integer = try_extract_string_is_integer_facts(condition, body)?;
    let starts_with = try_extract_starts_with_facts(condition, body)?;
    let int_to_str = try_extract_int_to_str_facts(condition, body)?;
    let escape_map = try_extract_escape_map_facts(condition, body)?;
    let split_lines = try_extract_split_lines_facts(condition, body)?;
    let skip_whitespace = try_extract_skip_whitespace_facts(condition, body)?;
    let loop_scan_methods_v0 = try_extract_loop_scan_methods_v0_facts(condition, body)?;
    let loop_scan_v0 = try_extract_loop_scan_v0_facts(condition, body)?;
    let loop_scan_phi_vars_v0 = try_extract_loop_scan_phi_vars_v0_facts(condition, body)?;
    let loop_collect_using_entries_v0 =
        try_extract_loop_collect_using_entries_v0_facts(condition, body)?;
    let loop_bundle_resolver_v0 = try_extract_loop_bundle_resolver_v0_facts(condition, body)?;
    // Phase 29bq: Extract loop_cond_break_continue BEFORE generic_loop_v0
    // Reason: loop_cond_break_continue handles shapes (like ExitIfTree) that generic_loop_v0
    // would reject with a Freeze in strict mode. By trying loop_cond_break_continue first,
    // we give it a chance to match before generic_loop_v0 sees the shape and freezes.
    // Table-driven cluster extraction (SSOT: nested_loop_profile::CLUSTER_PROFILES)
    // Priority order: cluster5 > cluster4 > cluster3 > base
    let loop_cond_break_continue = {
        let cluster_map = extract_cluster_facts_map(condition, body)?;
        // Check cluster profiles in priority order (5 -> 4 -> 3)
        let mut selected: Option<LoopCondBreakContinueFacts> = None;
        for profile in CLUSTER_PROFILES {
            if let Some(facts) = cluster_map.get(&profile.required_count) {
                selected = Some(facts.clone());
                break;
            }
        }
        // Fall back to base if no cluster matched
        selected.or(try_extract_loop_cond_break_continue_facts(condition, body)?)
    };
    let loop_cond_continue_only =
        try_extract_loop_cond_continue_only_facts(condition, body)?;
    let loop_cond_continue_with_return =
        try_extract_loop_cond_continue_with_return_facts(condition, body)?;
    let loop_cond_return_in_body =
        try_extract_loop_cond_return_in_body_facts(condition, body)?;
    // Phase 29bq: Skip generic_loop_v0/v1 extraction when loop_cond_* routes matched.
    // generic_loop_v0 would freeze on shapes like ExitIfTree that loop_cond_break_continue
    // can handle. By skipping when we have a specific match, we avoid the freeze.
    let has_generic_v1_recipe_hint = loop_cond_break_continue
        .as_ref()
        .is_some_and(|_| has_generic_loop_v1_recipe_hint(condition, body));
    let loop_cond_break_blocks_generic = loop_cond_break_continue.as_ref().is_some_and(|facts| {
        if has_generic_v1_recipe_hint {
            return false;
        }
        if matches!(
            facts.accept_kind,
            LoopCondBreakAcceptKind::NestedLoopOnly | LoopCondBreakAcceptKind::ProgramBlockNoExit
        ) {
            return false;
        }
        let effect_only = facts.recipe.items.iter().all(|item| {
            matches!(
                item,
                LoopCondBreakContinueItem::Stmt(_)
                    | LoopCondBreakContinueItem::GeneralIf(_)
                    | LoopCondBreakContinueItem::ProgramBlock { .. }
                    | LoopCondBreakContinueItem::NestedLoopDepth1 { .. }
            )
        });
        !effect_only
    });
    let loop_cond_any_matched = loop_cond_break_blocks_generic
        || loop_cond_continue_only.is_some()
        || loop_cond_continue_with_return.is_some()
        || loop_cond_return_in_body.is_some();
    let generic_loop_v0 = if loop_cond_any_matched {
        None
    } else {
        try_extract_generic_loop_v0_facts(condition, body)?
    };
    let generic_loop_v1 = if loop_cond_any_matched {
        None
    } else {
        try_extract_generic_loop_v1_facts(condition, body)?
    };
    let if_phi_join = try_extract_if_phi_join_facts(condition, body)?;
    let loop_continue_only = try_extract_loop_continue_only_facts(condition, body)?;
    let loop_true_early_exit =
        try_extract_loop_true_early_exit_facts(condition, body)?;
    let loop_true_break_continue =
        try_extract_loop_true_break_continue_facts(condition, body)?;
    let nested_loop_minimal =
        try_extract_nested_loop_minimal_facts(condition, body)?;
    let bool_predicate_scan =
        try_extract_bool_predicate_scan_facts(condition, body, &observation)?;
    let accum_const_loop =
        try_extract_accum_const_loop_facts(condition, body, &observation)?;
    let loop_break = try_extract_loop_break_facts(condition, body)?;
    let loop_break_body_local = try_extract_loop_break_body_local_facts(condition, body)?;

    let has_any = scan_with_init.is_some()
        || split_scan.is_some()
        || loop_simple_while.is_some()
        || loop_char_map.is_some()
        || loop_array_join.is_some()
        || string_is_integer.is_some()
        || starts_with.is_some()
        || int_to_str.is_some()
        || escape_map.is_some()
        || split_lines.is_some()
        || skip_whitespace.is_some()
        || loop_scan_methods_block_v0.is_some()
        || loop_scan_methods_v0.is_some()
        || loop_scan_v0.is_some()
        || loop_scan_phi_vars_v0.is_some()
        || loop_collect_using_entries_v0.is_some()
        || loop_bundle_resolver_v0.is_some()
        || generic_loop_v0.is_some()
        || generic_loop_v1.is_some()
        || if_phi_join.is_some()
        || loop_continue_only.is_some()
        || loop_true_early_exit.is_some()
        || loop_true_break_continue.is_some()
        || loop_cond_break_continue.is_some()
        || loop_cond_continue_only.is_some()
        || loop_cond_continue_with_return.is_some()
        || loop_cond_return_in_body.is_some()
        || nested_loop_minimal.is_some()
        || bool_predicate_scan.is_some()
        || accum_const_loop.is_some()
        || loop_break.is_some()
        || loop_break_body_local.is_some();
    if !has_any {
        return Ok(None);
    }

    let skeleton = match try_extract_loop_skeleton_facts(condition, body)? {
        Some(skeleton) => skeleton,
        None => {
            return Err(Freeze::bug(
                "loop facts require skeleton when patterns are present",
            ));
        }
    };
    let features = try_extract_loop_feature_facts(body)?;

    let facts = LoopFacts {
        condition_shape,
        step_shape,
        skeleton,
        features,
        scan_with_init,
        split_scan,
        loop_simple_while,
        loop_char_map,
        loop_array_join,
        string_is_integer,
        starts_with,
        int_to_str,
        escape_map,
        split_lines,
        skip_whitespace,
        generic_loop_v0,
        generic_loop_v1,
        if_phi_join: if_phi_join,
        loop_continue_only: loop_continue_only,
        loop_true_early_exit,
        loop_true_break_continue,
        loop_cond_break_continue,
        loop_cond_continue_only,
        loop_cond_continue_with_return,
        loop_cond_return_in_body,
        loop_scan_v0,
        loop_scan_methods_block_v0,
        loop_scan_methods_v0,
        loop_scan_phi_vars_v0,
        loop_collect_using_entries_v0,
        loop_bundle_resolver_v0,
        nested_loop_minimal,
        bool_predicate_scan,
        accum_const_loop,
        loop_break,
        loop_break_body_local,
    };
    if crate::config::env::joinir_dev::debug_enabled() {
        let ring0 = crate::runtime::get_global_ring0();
        ring0.log.debug(&format!(
            "[plan/trace:facts_summary] ctx=loop_facts scan_methods={} scan_methods_block={} loop_scan={} loop_scan_phi_vars={} collect_using_entries={} bundle_resolver={}",
            facts.loop_scan_methods_v0.is_some() as u8,
            facts.loop_scan_methods_block_v0.is_some() as u8,
            facts.loop_scan_v0.is_some() as u8,
            facts.loop_scan_phi_vars_v0.is_some() as u8,
            facts.loop_collect_using_entries_v0.is_some() as u8,
            facts.loop_bundle_resolver_v0.is_some() as u8
        ));
    }
    Ok(Some(facts))
}

/// Table-driven cluster facts extraction (SSOT: nested_loop_profile::CLUSTER_PROFILES).
///
/// Returns a BTreeMap keyed by required_count (3, 4, 5, ...).
/// Adding cluster6+ only requires a single line in CLUSTER_PROFILES.
pub(super) fn extract_cluster_facts_map(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Result<BTreeMap<u8, LoopCondBreakContinueFacts>, Freeze> {
    let mut map = BTreeMap::new();
    for profile in CLUSTER_PROFILES {
        let count = profile.required_count as usize;
        if let Some(facts) = try_extract_loop_cond_break_continue_facts_with_limit(
            condition,
            body,
            count,
            Some(count),
        )? {
            map.insert(profile.required_count, facts);
        }
    }
    Ok(map)
}
