//! Phase 29aj P0: Planner outcome (facts + plan) SSOT

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::facts::{
    try_build_loop_facts, try_build_loop_facts_with_ctx, LoopFacts,
};
use crate::mir::builder::control_flow::plan::normalize::{
    canonicalize_loop_facts, CanonicalLoopFacts,
};
use crate::mir::builder::control_flow::plan::recipe_tree::contracts::RecipeContract;
use super::context::PlannerContext;
use super::Freeze;

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct PlanBuildOutcome {
    pub facts: Option<CanonicalLoopFacts>,
    /// Recipe contract (Phase B: parallel path, planner_required only).
    pub recipe_contract: Option<RecipeContract>,
}

pub(in crate::mir::builder) fn build_plan_with_facts(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Result<PlanBuildOutcome, Freeze> {
    let facts = try_build_loop_facts(condition, body)?;
    let legacy_ctx = PlannerContext::default_for_legacy();
    build_plan_from_facts_opt_with(&legacy_ctx, facts)
}

pub(in crate::mir::builder) fn build_plan_with_facts_ctx(
    ctx: &PlannerContext,
    condition: &ASTNode,
    body: &[ASTNode],
) -> Result<PlanBuildOutcome, Freeze> {
    let facts = try_build_loop_facts_with_ctx(ctx, condition, body)?;
    build_plan_from_facts_opt_with(ctx, facts)
}

fn build_plan_from_facts_opt_with(
    ctx: &PlannerContext,
    facts: Option<LoopFacts>,
) -> Result<PlanBuildOutcome, Freeze> {
    let Some(facts) = facts else {
        return Ok(PlanBuildOutcome {
            facts: None,
            recipe_contract: None,
        });
    };
    let canonical = canonicalize_loop_facts(facts);
    let _ = ctx;

    Ok(PlanBuildOutcome {
        facts: Some(canonical),
        recipe_contract: None,
    })
}
