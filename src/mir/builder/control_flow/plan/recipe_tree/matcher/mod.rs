//! Recipe Matcher (Recipe-first migration Phase C2).
//!
//! Matches Facts to RecipeContract using route semantics.
//! Phase C2: centralizes loop-break verification.

use super::contracts::{RecipeContract, RecipeContractKind, StmtConstraint};
use crate::mir::builder::control_flow::plan::facts::feature_facts::ExitKindFacts;
use crate::mir::builder::control_flow::lower::normalize::CanonicalLoopFacts;
use crate::mir::builder::control_flow::plan::planner::Freeze;

mod loop_cond;
mod loop_scan;
mod patterns;
mod utils;

use loop_cond::*;
use loop_scan::*;
use patterns::*;

/// Macro to simplify CondProfile-based route verification.
///
/// Route families that use CondProfile all follow this shape:
/// - 4 debug_observe_cond_profile_* calls
/// - 1 accept_via_cond_profile_* call
/// - 1 verify_*_recipe call
macro_rules! verify_cond_profile_route {
    (
        // The route facts variable
        $route_facts:ident,
        // The facts reference for step_shape
        $facts:expr,
        // The route label (for debug logging)
        $route_label:ident,
        // The verifier function name
        $verifier:ident,
        // The accept function name
        $accept_func:ident
    ) => {{
        crate::mir::builder::control_flow::verify::verifier::debug_observe_cond_profile_idx_var(
            &$route_facts.loop_var,
            &$route_facts.cond_profile,
        );
        crate::mir::builder::control_flow::verify::verifier::debug_observe_cond_profile_step_mismatch(
            &$facts.facts.step_shape,
            &$route_facts.cond_profile,
        );
        crate::mir::builder::control_flow::verify::verifier::debug_observe_cond_profile_completeness(
            &$route_facts.cond_profile,
        );
        crate::mir::builder::control_flow::verify::verifier::debug_observe_cond_profile_priority(
            &$route_facts.cond_profile,
        );
        crate::mir::builder::control_flow::verify::verifier::$accept_func($route_facts)?;
        $verifier($route_facts)?;
    }};
}

/// Macro to simplify standard route verification.
///
/// Most route variants follow this shape:
/// - `if let Some(route_facts) = &facts.facts.*`
/// - `verify_*_recipe(route_facts)?`
macro_rules! verify_route {
    (
        // The optional route facts expression
        $route_facts:expr,
        // The verifier function name
        $verifier:ident
    ) => {{
        if let Some(route_facts) = $route_facts {
            $verifier(route_facts)?;
        }
    }};
}

pub(in crate::mir::builder) struct RecipeMatcher;

impl RecipeMatcher {
    /// Try to match loop facts to a recipe contract.
    ///
    /// Returns a descriptive contract (minimal, not precise yet).
    /// Phase C2: loop-break verification is included.
    pub fn try_match_loop(facts: &CanonicalLoopFacts) -> Result<Option<RecipeContract>, Freeze> {
        let has_break = facts.exit_kinds_present.contains(&ExitKindFacts::Break);
        let has_continue = facts.exit_kinds_present.contains(&ExitKindFacts::Continue);
        let has_return = facts.exit_kinds_present.contains(&ExitKindFacts::Return);

        // Phase C2: loop_break verification (planner_required only)
        verify_route!(facts.facts.loop_break(), verify_loop_break_recipe);

        // Phase C6: if_phi_join verification (planner_required only)
        verify_route!(facts.facts.if_phi_join(), verify_if_phi_join_recipe);

        // Phase C9: loop_continue_only verification (planner_required only)
        verify_route!(
            facts.facts.loop_continue_only(),
            verify_loop_continue_only_recipe
        );

        // Phase C10: loop_true_early_exit verification (planner_required only)
        verify_route!(
            facts.facts.loop_true_early_exit(),
            verify_loop_true_early_exit_recipe
        );

        // Phase C11: loop_simple_while verification (planner_required only)
        verify_route!(
            facts.facts.loop_simple_while(),
            verify_loop_simple_while_recipe
        );

        // Phase C12: loop_char_map verification (planner_required only)
        if let Some(loop_char_map) = facts.facts.loop_char_map() {
            verify_cond_profile_route!(
                loop_char_map,
                facts,
                LoopCharMap,
                verify_loop_char_map_recipe,
                accept_via_cond_profile_loop_char_map
            );
        }

        // Phase C13: loop_array_join verification (planner_required only)
        if let Some(loop_array_join) = facts.facts.loop_array_join() {
            verify_cond_profile_route!(
                loop_array_join,
                facts,
                LoopArrayJoin,
                verify_loop_array_join_recipe,
                accept_via_cond_profile_loop_array_join
            );
        }

        // Phase C14: scan_with_init verification (planner_required only)
        verify_route!(facts.facts.scan_with_init(), verify_scan_with_init_recipe);

        // Phase C14: split_scan verification (planner_required only)
        verify_route!(facts.facts.split_scan(), verify_split_scan_recipe);

        // Phase C14: bool_predicate_scan verification (planner_required only)
        if let Some(bool_predicate_scan) = facts.facts.bool_predicate_scan() {
            verify_cond_profile_route!(
                bool_predicate_scan,
                facts,
                BoolPredicateScan,
                verify_bool_predicate_scan_recipe,
                accept_via_cond_profile_bool_predicate_scan
            );
        }

        // Phase C14: accum_const_loop verification (planner_required only)
        if let Some(accum_const_loop) = facts.facts.accum_const_loop() {
            verify_cond_profile_route!(
                accum_const_loop,
                facts,
                AccumConstLoop,
                verify_accum_const_loop_recipe,
                accept_via_cond_profile_accum_const_loop
            );
        }

        // Phase C15: LoopScanMethodsV0 verification (planner_required only)
        verify_route!(
            facts.facts.loop_scan_methods_v0(),
            verify_loop_scan_methods_v0_recipe
        );

        // Phase C15: LoopScanMethodsBlockV0 verification (planner_required only)
        verify_route!(
            facts.facts.loop_scan_methods_block_v0(),
            verify_loop_scan_methods_block_v0_recipe
        );

        // Phase C15: LoopScanPhiVarsV0 verification (planner_required only)
        verify_route!(
            facts.facts.loop_scan_phi_vars_v0(),
            verify_loop_scan_phi_vars_v0_recipe
        );

        // Phase C15: LoopScanV0 verification (planner_required only)
        verify_route!(facts.facts.loop_scan_v0(), verify_loop_scan_v0_recipe);

        // Phase C16: LoopCollectUsingEntriesV0 verification (planner_required only)
        verify_route!(
            facts.facts.loop_collect_using_entries_v0(),
            verify_loop_collect_using_entries_v0_recipe
        );

        // Phase C16: LoopBundleResolverV0 verification (planner_required only)
        verify_route!(
            facts.facts.loop_bundle_resolver_v0(),
            verify_loop_bundle_resolver_v0_recipe
        );

        // Phase C16: LoopTrueBreakContinue verification (planner_required only)
        verify_route!(
            facts.facts.loop_true_break_continue(),
            verify_loop_true_break_continue_recipe
        );

        // Phase C17: loop_cond_break_continue verification (planner_required only)
        verify_route!(
            facts.facts.loop_cond_break_continue(),
            verify_loop_cond_break_continue_recipe
        );

        // Phase C17: loop_cond_continue_only verification (planner_required only)
        verify_route!(
            facts.facts.loop_cond_continue_only(),
            verify_loop_cond_continue_only_recipe
        );

        // Phase C17: loop_cond_continue_with_return verification (planner_required only)
        verify_route!(
            facts.facts.loop_cond_continue_with_return(),
            verify_loop_cond_continue_with_return_recipe
        );

        // Phase C17: loop_cond_return_in_body verification (planner_required only)
        verify_route!(
            facts.facts.loop_cond_return_in_body(),
            verify_loop_cond_return_in_body_recipe
        );

        // Phase C18: generic_loop_v1 recipe verification (planner_required only)
        verify_route!(facts.facts.generic_loop_v1(), verify_generic_loop_v1_recipe);

        Ok(Some(RecipeContract {
            kind: RecipeContractKind::LoopWithExit {
                has_break,
                has_continue,
                has_return,
            },
            required_exits: vec![],
            allowed_stmts: StmtConstraint::Any,
        }))
    }
}
