//! Recipe Matcher (Recipe-first migration Phase C2).
//!
//! Matches Facts to RecipeContract without pattern names.
//! Phase C2: Centralizes Pattern2Break verification.

use super::contracts::{RecipeContract, RecipeContractKind, StmtConstraint};
use crate::mir::builder::control_flow::plan::facts::feature_facts::ExitKindFacts;
use crate::mir::builder::control_flow::plan::normalize::CanonicalLoopFacts;
use crate::mir::builder::control_flow::plan::planner::Freeze;

mod utils;
mod patterns;
mod loop_scan;
mod loop_cond;

use patterns::*;
use loop_scan::*;
use loop_cond::*;

/// Macro to simplify CondProfile-based pattern verification.
///
/// Pattern1CharMap, Pattern1ArrayJoin, Pattern8, Pattern9 all follow this pattern:
/// - 4 debug_observe_cond_profile_* calls
/// - 1 accept_via_cond_profile_* call
/// - 1 verify_pattern*_recipe call
macro_rules! verify_cond_profile_pattern {
    (
        // The pattern facts variable
        $pattern:ident,
        // The facts reference for step_shape
        $facts:expr,
        // The pattern name (for debug logging)
        $pattern_name:ident,
        // The verifier function name
        $verifier:ident,
        // The accept function name
        $accept_func:ident
    ) => {{
        crate::mir::builder::control_flow::plan::verifier::debug_observe_cond_profile_idx_var(
            &$pattern.loop_var,
            &$pattern.cond_profile,
        );
        crate::mir::builder::control_flow::plan::verifier::debug_observe_cond_profile_step_mismatch(
            &$facts.facts.step_shape,
            &$pattern.cond_profile,
        );
        crate::mir::builder::control_flow::plan::verifier::debug_observe_cond_profile_completeness(
            &$pattern.cond_profile,
        );
        crate::mir::builder::control_flow::plan::verifier::debug_observe_cond_profile_priority(
            &$pattern.cond_profile,
        );
        crate::mir::builder::control_flow::plan::verifier::$accept_func($pattern)?;
        $verifier($pattern)?;
    }};
}

/// Macro to simplify standard pattern verification.
///
/// Pattern2, 3, 4, 5, 6, 7 all follow this pattern:
/// - if let Some(pattern) = &facts.facts.*
/// - verify_pattern*_recipe(pattern)?
macro_rules! verify_pattern {
    (
        // The facts reference
        $facts:expr,
        // The field name in facts.facts.*
        $field:ident,
        // The verifier function name
        $verifier:ident
    ) => {{
        if let Some(pattern) = &$facts.facts.$field {
            $verifier(pattern)?;
        }
    }};
}

pub(in crate::mir::builder) struct RecipeMatcher;

impl RecipeMatcher {
    /// Try to match loop facts to a recipe contract.
    ///
    /// Returns a descriptive contract (minimal, not precise yet).
    /// Phase C2: Pattern2Break verification is included.
    pub fn try_match_loop(facts: &CanonicalLoopFacts) -> Result<Option<RecipeContract>, Freeze> {
        let has_break = facts.exit_kinds_present.contains(&ExitKindFacts::Break);
        let has_continue = facts.exit_kinds_present.contains(&ExitKindFacts::Continue);
        let has_return = facts.exit_kinds_present.contains(&ExitKindFacts::Return);

        // Phase C2: Pattern2Break verification (planner_required only)
        verify_pattern!(facts, pattern2_break, verify_pattern2_break_recipe);

        // Phase C6: Pattern3IfPhi verification (planner_required only)
        verify_pattern!(facts, pattern3_ifphi, verify_pattern3_ifphi_recipe);

        // Phase C9: Pattern4Continue verification (planner_required only)
        verify_pattern!(facts, pattern4_continue, verify_pattern4_continue_recipe);

        // Phase C10: Pattern5InfiniteEarlyExit verification (planner_required only)
        verify_pattern!(facts, pattern5_infinite_early_exit, verify_pattern5_infinite_early_exit_recipe);

        // Phase C11: Pattern1SimpleWhile verification (planner_required only)
        verify_pattern!(facts, pattern1_simplewhile, verify_pattern1_simple_while_recipe);

        // Phase C12: Pattern1CharMap verification (planner_required only)
        if let Some(pattern1_cm) = &facts.facts.pattern1_char_map {
            verify_cond_profile_pattern!(
                pattern1_cm,
                facts,
                Pattern1CharMap,
                verify_pattern1_char_map_recipe,
                accept_via_cond_profile_pattern1_char_map
            );
        }

        // Phase C13: Pattern1ArrayJoin verification (planner_required only)
        if let Some(pattern1_aj) = &facts.facts.pattern1_array_join {
            verify_cond_profile_pattern!(
                pattern1_aj,
                facts,
                Pattern1ArrayJoin,
                verify_pattern1_array_join_recipe,
                accept_via_cond_profile_pattern1_array_join
            );
        }

        // Phase C14: Pattern6 ScanWithInit verification (planner_required only)
        verify_pattern!(facts, scan_with_init, verify_pattern6_scan_with_init_recipe);

        // Phase C14: Pattern7 SplitScan verification (planner_required only)
        verify_pattern!(facts, split_scan, verify_pattern7_split_scan_recipe);

        // Phase C14: Pattern8 BoolPredicateScan verification (planner_required only)
        if let Some(pattern8) = &facts.facts.pattern8_bool_predicate_scan {
            verify_cond_profile_pattern!(
                pattern8,
                facts,
                Pattern8BoolPredicateScan,
                verify_pattern8_bool_predicate_scan_recipe,
                accept_via_cond_profile_pattern8_bool_predicate_scan
            );
        }

        // Phase C14: Pattern9 AccumConstLoop verification (planner_required only)
        if let Some(pattern9) = &facts.facts.pattern9_accum_const_loop {
            verify_cond_profile_pattern!(
                pattern9,
                facts,
                Pattern9AccumConstLoop,
                verify_pattern9_accum_const_loop_recipe,
                accept_via_cond_profile_pattern9_accum_const_loop
            );
        }

        // Phase C15: LoopScanMethodsV0 verification (planner_required only)
        verify_pattern!(facts, loop_scan_methods_v0, verify_loop_scan_methods_v0_recipe);

        // Phase C15: LoopScanMethodsBlockV0 verification (planner_required only)
        verify_pattern!(facts, loop_scan_methods_block_v0, verify_loop_scan_methods_block_v0_recipe);

        // Phase C15: LoopScanPhiVarsV0 verification (planner_required only)
        verify_pattern!(facts, loop_scan_phi_vars_v0, verify_loop_scan_phi_vars_v0_recipe);

        // Phase C15: LoopScanV0 verification (planner_required only)
        verify_pattern!(facts, loop_scan_v0, verify_loop_scan_v0_recipe);

        // Phase C16: LoopCollectUsingEntriesV0 verification (planner_required only)
        verify_pattern!(facts, loop_collect_using_entries_v0, verify_loop_collect_using_entries_v0_recipe);

        // Phase C16: LoopBundleResolverV0 verification (planner_required only)
        verify_pattern!(facts, loop_bundle_resolver_v0, verify_loop_bundle_resolver_v0_recipe);

        // Phase C16: LoopTrueBreakContinue verification (planner_required only)
        verify_pattern!(facts, loop_true_break_continue, verify_loop_true_break_continue_recipe);

        // Phase C17: loop_cond_break_continue verification (planner_required only)
        verify_pattern!(facts, loop_cond_break_continue, verify_loop_cond_break_continue_recipe);

        // Phase C17: loop_cond_continue_only verification (planner_required only)
        verify_pattern!(facts, loop_cond_continue_only, verify_loop_cond_continue_only_recipe);

        // Phase C17: loop_cond_continue_with_return verification (planner_required only)
        verify_pattern!(facts, loop_cond_continue_with_return, verify_loop_cond_continue_with_return_recipe);

        // Phase C17: loop_cond_return_in_body verification (planner_required only)
        verify_pattern!(facts, loop_cond_return_in_body, verify_loop_cond_return_in_body_recipe);

        // Phase C18: generic_loop_v1 recipe verification (planner_required only)
        verify_pattern!(facts, generic_loop_v1, verify_generic_loop_v1_recipe);

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