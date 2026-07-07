//! Runtime-adjacent ProgramJSON shadow guard boundary.
//!
//! This module is intentionally read-only. It records the boundary immediately
//! after Rust `try_build_outcome(ctx)` and before route registry candidate
//! selection. ProgramJSON shadow evidence is verified by lifecycle gates; this
//! Rust seam must not write shadow data into route authority.

use crate::mir::builder::control_flow::lower::PlanBuildOutcome;
use crate::mir::builder::control_flow::plan::recipe_tree::RecipeContractKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) struct RuntimeAdjacentShadowGuardReport {
    pub runtime_authority_is_rust_astnode: bool,
    pub boundary_after_try_build_outcome: bool,
    pub boundary_before_route_candidate_selection: bool,
    pub rust_recipe_contract_present: bool,
    pub matched_loop_with_exit: bool,
    pub has_break: bool,
    pub has_continue: bool,
    pub has_return: bool,
    pub programjson_runtime_route_authority: bool,
    pub runtime_route_switch: bool,
    pub recipe_matcher_input_authority: bool,
    pub writes_downstream: bool,
    pub runtime_fallback: bool,
}

/// Observe the Rust authority outcome at the runtime-adjacent shadow boundary.
///
/// This function must stay side-effect free:
/// - no mutation of `PlanBuildOutcome`;
/// - no route registry access;
/// - no lowering, MIR mutation, ID allocation, or fallback.
pub(in crate::mir::builder) fn observe_after_try_build_outcome_before_registry(
    outcome: &PlanBuildOutcome,
) -> RuntimeAdjacentShadowGuardReport {
    let mut report = RuntimeAdjacentShadowGuardReport {
        runtime_authority_is_rust_astnode: true,
        boundary_after_try_build_outcome: true,
        boundary_before_route_candidate_selection: true,
        rust_recipe_contract_present: outcome.recipe_contract.is_some(),
        matched_loop_with_exit: false,
        has_break: false,
        has_continue: false,
        has_return: false,
        programjson_runtime_route_authority: false,
        runtime_route_switch: false,
        recipe_matcher_input_authority: false,
        writes_downstream: false,
        runtime_fallback: false,
    };

    if let Some(contract) = outcome.recipe_contract.as_ref() {
        let &RecipeContractKind::LoopWithExit {
            has_break,
            has_continue,
            has_return,
        } = &contract.kind;
        report.matched_loop_with_exit = true;
        report.has_break = has_break;
        report.has_continue = has_continue;
        report.has_return = has_return;
    }

    report
}
