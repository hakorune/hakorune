//! Planner build_plan_from_facts_ctx entrypoint (single-plan boundary SSOT)

use crate::mir::builder::control_flow::plan::facts::skeleton_facts::SkeletonKind;
use crate::mir::builder::control_flow::plan::normalize::CanonicalLoopFacts;
use crate::mir::builder::control_flow::plan::verifier::{
    debug_assert_value_join_invariants, debug_observe_cond_profile,
};
use crate::mir::builder::control_flow::plan::DomainPlan;

use super::context::PlannerContext;
use super::helpers::{infer_exit_usage, infer_skeleton_kind};
use super::validators::debug_assert_cleanup_kinds_match_exit_kinds;
use super::Freeze;

pub(in crate::mir::builder) fn build_plan_from_facts_ctx(
    _ctx: &PlannerContext,
    facts: CanonicalLoopFacts,
) -> Result<Option<DomainPlan>, Freeze> {
    // Single-plan boundary (SSOT).
    //
    // Current behavior: DomainPlan is intentionally minimal
    // (`LoopCondContinueWithReturn` only). Other loop shapes are routed via
    // recipe-only paths outside DomainPlan.

    if !matches!(facts.skeleton_kind, SkeletonKind::Loop) {
        return Ok(None);
    }

    let _skeleton_kind = infer_skeleton_kind(&facts);
    let _exit_usage = infer_exit_usage(&facts);
    debug_assert_cleanup_kinds_match_exit_kinds(
        &facts.cleanup_kinds_present,
        &facts.exit_kinds_present,
    );
    debug_assert_value_join_invariants(&facts);
    debug_observe_cond_profile(&facts);

    let Some(facts) = &facts.facts.loop_cond_continue_with_return else {
        return Ok(None);
    };

    let plan = crate::mir::builder::control_flow::plan::LoopCondContinueWithReturnPlan {
        condition: facts.condition.clone(),
        recipe: facts.recipe.clone(),
    };
    Ok(Some(plan))
}
