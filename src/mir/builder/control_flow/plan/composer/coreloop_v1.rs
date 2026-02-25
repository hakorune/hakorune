//! CoreLoop v1 pattern composers for Pattern2, Pattern3, and Pattern5.
//!
//! This module provides functions to compose CorePlan from CanonicalLoopFacts
//! for specific loop patterns that require value_join support.

use crate::mir::builder::control_flow::joinir::patterns::router::LoopPatternContext;
use crate::mir::builder::control_flow::plan::composer::coreloop_gates::{
    pattern2_value_join_gate, pattern3_value_join_gate, pattern5_value_join_gate,
};
use crate::mir::builder::control_flow::plan::normalize::CanonicalLoopFacts;
use crate::mir::builder::control_flow::plan::normalizer::PlanNormalizer;
use crate::mir::builder::control_flow::plan::recipe_tree::RecipeComposer;
use crate::mir::builder::control_flow::plan::{LoweredRecipe, Pattern2BreakPlan};
use crate::mir::builder::MirBuilder;

pub(in crate::mir::builder) fn try_compose_core_loop_v1_pattern2_break(
    builder: &mut MirBuilder,
    facts: &CanonicalLoopFacts,
    ctx: &LoopPatternContext,
) -> Result<Option<LoweredRecipe>, String> {
    if !pattern2_value_join_gate(facts) {
        return Ok(None);
    }

    let Some(pattern2) = facts.facts.pattern2_break.as_ref() else {
        return Ok(None);
    };
    let parts = Pattern2BreakPlan {
        loop_var: pattern2.loop_var.clone(),
        carrier_var: pattern2.carrier_var.clone(),
        loop_condition: pattern2.loop_condition.clone(),
        break_condition: pattern2.break_condition.clone(),
        carrier_update_in_break: pattern2.carrier_update_in_break.clone(),
        carrier_update_in_body: pattern2.carrier_update_in_body.clone(),
        loop_increment: pattern2.loop_increment.clone(),
        step_placement: pattern2.step_placement,
        promotion: None,
    };
    let core =
        PlanNormalizer::normalize_pattern2_break(builder, parts, ctx).map_err(|e| e.to_string())?;
    Ok(Some(core))
}

pub(in crate::mir::builder) fn try_compose_core_loop_v1_pattern5_infinite_early_exit(
    builder: &mut MirBuilder,
    facts: &CanonicalLoopFacts,
    ctx: &LoopPatternContext,
) -> Result<Option<LoweredRecipe>, String> {
    if !pattern5_value_join_gate(facts) {
        return Ok(None);
    }

    let Some(pattern5) = facts.facts.pattern5_infinite_early_exit.as_ref() else {
        return Ok(None);
    };
    let _ = pattern5;
    let core = RecipeComposer::compose_pattern5_infinite_early_exit_recipe(builder, facts, ctx)
        .map_err(|e| e.to_string())?;
    Ok(Some(core))
}

pub(in crate::mir::builder) fn try_compose_core_loop_v1_pattern3_ifphi(
    builder: &mut MirBuilder,
    facts: &CanonicalLoopFacts,
    ctx: &LoopPatternContext,
) -> Result<Option<LoweredRecipe>, String> {
    if !pattern3_value_join_gate(facts) {
        return Ok(None);
    }

    let Some(pattern3) = facts.facts.pattern3_ifphi.as_ref() else {
        return Ok(None);
    };
    let _ = pattern3;
    let core = RecipeComposer::compose_pattern3_ifphi_recipe(builder, facts, ctx)
        .map_err(|e| e.to_string())?;
    Ok(Some(core))
}
