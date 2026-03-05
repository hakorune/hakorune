//! CoreLoop v1 composers for loop-break / if-phi-join / loop-true-early-exit.
//!
//! This module provides functions to compose CorePlan from CanonicalLoopFacts
//! for specific loop patterns that require value_join support.

use crate::mir::builder::control_flow::joinir::patterns::router::LoopRouteContext;
use crate::mir::builder::control_flow::plan::composer::coreloop_gates::{
    if_phi_join_value_join_gate, loop_break_value_join_gate,
    loop_true_early_exit_value_join_gate,
};
use crate::mir::builder::control_flow::plan::normalize::CanonicalLoopFacts;
use crate::mir::builder::control_flow::plan::recipe_tree::RecipeComposer;
use crate::mir::builder::control_flow::plan::LoweredRecipe;
use crate::mir::builder::MirBuilder;

pub(in crate::mir::builder) fn try_compose_core_loop_v1_loop_break(
    builder: &mut MirBuilder,
    facts: &CanonicalLoopFacts,
    ctx: &LoopRouteContext,
) -> Result<Option<LoweredRecipe>, String> {
    if !loop_break_value_join_gate(facts) {
        return Ok(None);
    }

    let Some(loop_break_facts) = facts.facts.pattern2_break.as_ref() else {
        return Ok(None);
    };
    let _ = loop_break_facts;
    let core = RecipeComposer::compose_loop_break_recipe(builder, facts, ctx)
        .map_err(|e| e.to_string())?;
    Ok(Some(core))
}

pub(in crate::mir::builder) fn try_compose_core_loop_v1_loop_true_early_exit(
    builder: &mut MirBuilder,
    facts: &CanonicalLoopFacts,
    ctx: &LoopRouteContext,
) -> Result<Option<LoweredRecipe>, String> {
    if !loop_true_early_exit_value_join_gate(facts) {
        return Ok(None);
    }

    let Some(loop_true_early_exit_facts) = facts.facts.pattern5_infinite_early_exit.as_ref() else {
        return Ok(None);
    };
    let _ = loop_true_early_exit_facts;
    let core = RecipeComposer::compose_loop_true_early_exit_recipe(builder, facts, ctx)
        .map_err(|e| e.to_string())?;
    Ok(Some(core))
}

pub(in crate::mir::builder) fn try_compose_core_loop_v1_if_phi_join(
    builder: &mut MirBuilder,
    facts: &CanonicalLoopFacts,
    ctx: &LoopRouteContext,
) -> Result<Option<LoweredRecipe>, String> {
    if !if_phi_join_value_join_gate(facts) {
        return Ok(None);
    }

    let Some(if_phi_join_facts) = facts.facts.pattern3_ifphi.as_ref() else {
        return Ok(None);
    };
    let _ = if_phi_join_facts;
    let core = RecipeComposer::compose_if_phi_join_recipe(builder, facts, ctx)
        .map_err(|e| e.to_string())?;
    Ok(Some(core))
}
