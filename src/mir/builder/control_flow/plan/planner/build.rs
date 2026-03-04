//! Planner build_plan entrypoint (CandidateSet boundary SSOT)

#![allow(dead_code)]

use crate::ast::ASTNode;

use crate::mir::builder::control_flow::plan::facts::skeleton_facts::SkeletonKind;
use crate::mir::builder::control_flow::plan::normalize::CanonicalLoopFacts;
use crate::mir::builder::control_flow::plan::verifier::{
    debug_assert_value_join_invariants, debug_observe_cond_profile,
};
use crate::mir::builder::control_flow::plan::DomainPlan;

use super::candidates::{CandidateSet, PlanCandidate};
use super::context::PlannerContext;
use super::helpers::{infer_exit_usage, infer_skeleton_kind};
use super::outcome::build_plan_with_facts;
use super::validators::debug_assert_cleanup_kinds_match_exit_kinds;
use super::Freeze;

/// External-ish SSOT entrypoint used by single_planner.
pub(in crate::mir::builder) fn build_plan(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Result<Option<DomainPlan>, Freeze> {
    Ok(build_plan_with_facts(condition, body)?.plan)
}

pub(in crate::mir::builder) fn build_plan_from_facts(
    facts: CanonicalLoopFacts,
) -> Result<Option<DomainPlan>, Freeze> {
    build_plan_from_facts_ctx(&PlannerContext::default_for_legacy(), facts)
}

pub(in crate::mir::builder) fn build_plan_from_facts_ctx(
    _ctx: &PlannerContext,
    facts: CanonicalLoopFacts,
) -> Result<Option<DomainPlan>, Freeze> {
    // CandidateSet-based boundary (SSOT).
    //
    // Current behavior: DomainPlan candidates are intentionally minimal
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

    let mut candidates = CandidateSet::new();

    // Phase 29bq P2.x: Try to extract loop_cond_continue_with_return plan
    push_loop_cond_continue_with_return(&mut candidates, &facts)?;

    candidates.finalize()
}

fn push_loop_cond_continue_with_return(
    candidates: &mut CandidateSet,
    facts: &CanonicalLoopFacts,
) -> Result<(), Freeze> {
    use crate::mir::builder::control_flow::plan::DomainPlan;

    let Some(facts) = &facts.facts.loop_cond_continue_with_return else {
        return Ok(());
    };

    let plan = DomainPlan::LoopCondContinueWithReturn(
        crate::mir::builder::control_flow::plan::LoopCondContinueWithReturnPlan {
            condition: facts.condition.clone(),
            recipe: facts.recipe.clone(),
        }
    );

    candidates.push(PlanCandidate {
        plan,
        rule: "loop_cond_continue_with_return",
    });

    Ok(())
}
