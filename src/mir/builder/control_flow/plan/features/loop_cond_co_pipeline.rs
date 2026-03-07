//! Main pipeline functions for loop_cond_continue_only pattern.

use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;
use crate::mir::builder::control_flow::plan::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::plan::edgecfg_facade::Frag;
use crate::mir::builder::control_flow::plan::features::body_view::BodyView;
use crate::mir::builder::control_flow::plan::features::carriers;
use crate::mir::builder::control_flow::plan::features::coreloop_frame::{
    build_coreloop_frame, build_header_step_phis,
};
use crate::mir::builder::control_flow::plan::features::step_mode;
use crate::mir::builder::control_flow::plan::loop_cond::continue_only_facts::LoopCondContinueOnlyFacts;
use crate::mir::builder::control_flow::plan::normalizer::lower_loop_header_cond;
use crate::mir::builder::control_flow::plan::parts;
use crate::mir::builder::control_flow::plan::steps::{
    build_standard5_internal_wires, collect_carrier_inits, empty_carriers_args,
};
use crate::mir::builder::control_flow::plan::{
    CoreEffectPlan, CoreLoopPlan, CorePlan, LoweredRecipe,
};
use crate::mir::builder::MirBuilder;
use std::collections::{BTreeMap, BTreeSet};

use super::loop_cond_co_block::lower_continue_only_block;

const LOOP_COND_CONTINUE_ONLY_ERR: &str = "[normalizer] loop_cond_continue_only";

pub(in crate::mir::builder) fn lower_loop_cond_continue_only(
    builder: &mut MirBuilder,
    facts: LoopCondContinueOnlyFacts,
    _ctx: &LoopRouteContext,
) -> Result<LoweredRecipe, String> {
    // Pass 1: Collect carrier variables (recipe-specific, SSOT entry)
    let carrier_vars_vec = carriers::collect_from_recipe_continue_only(&facts.recipe).vars;
    let carrier_vars: BTreeSet<String> = carrier_vars_vec.iter().cloned().collect();

    // Pass 2: Build carrier_inits (step: lookup from variable_map)
    let carrier_inits = collect_carrier_inits(
        builder,
        carrier_vars.iter().cloned(),
        LOOP_COND_CONTINUE_ONLY_ERR,
    )?;

    // Phase 11: StepBb mode only (HeaderBb legacy removed)
    lower_loop_cond_continue_only_stepbb(builder, facts, &carrier_vars, &carrier_inits)
}

/// StepBb mode implementation using CoreLoop Skeleton template.
///
/// This is the default mode for loop_cond_continue_only.
/// Uses the coreloop_skeleton template for block allocation and PHI management.
fn lower_loop_cond_continue_only_stepbb(
    builder: &mut MirBuilder,
    facts: LoopCondContinueOnlyFacts,
    carrier_vars: &BTreeSet<String>,
    carrier_inits: &BTreeMap<String, crate::mir::ValueId>,
) -> Result<LoweredRecipe, String> {
    // Use template for block allocation + PHI dst allocation
    let frame = build_coreloop_frame(
        builder,
        carrier_vars,
        carrier_inits,
        LOOP_COND_CONTINUE_ONLY_ERR,
    )?;

    // Extract block IDs from frame
    let preheader_bb = frame.preheader_bb;
    let header_bb = frame.header_bb;
    let body_bb = frame.body_bb;
    let step_bb = frame.step_bb;
    let after_bb = frame.after_bb;
    let continue_target = frame.continue_target; // Always step_bb in StepBb mode

    // Check for empty carriers (fail-fast)
    if frame.carrier_header_phis.is_empty() && !carrier_vars.is_empty() {
        return Err(format!("{LOOP_COND_CONTINUE_ONLY_ERR}: no loop carriers"));
    }

    // Set up current_bindings with header PHI destinations
    let mut current_bindings = frame.carrier_header_phis.clone();
    for (name, value_id) in &current_bindings {
        builder
            .variable_ctx
            .variable_map
            .insert(name.clone(), *value_id);
    }

    // Phase 2b-1: Short-circuit evaluation for loop header condition
    let cond_view = CondBlockView::from_expr(&facts.condition);
    let header_result = lower_loop_header_cond(
        builder,
        &current_bindings,
        &cond_view,
        header_bb,
        body_bb,
        after_bb,
        empty_carriers_args(),
        empty_carriers_args(),
        LOOP_COND_CONTINUE_ONLY_ERR,
    )?;

    // Build Frag: internal wires + short-circuit branches
    let internal_wires = build_standard5_internal_wires(&frame, empty_carriers_args());
    let frag = Frag {
        entry: header_bb,
        block_params: BTreeMap::new(),
        exits: BTreeMap::new(),
        wires: internal_wires,
        branches: header_result.branches,
    };

    // Lower loop body (using frame's PHI maps)
    let mut carrier_updates = BTreeMap::new();
    let body_view = BodyView::Recipe(&facts.recipe.body);
    let mut body_plans = lower_continue_only_block(
        builder,
        &mut current_bindings,
        &frame.carrier_header_phis,
        &frame.carrier_step_phis,
        &mut carrier_updates,
        &body_view,
        &facts.recipe.items,
    )?;

    // Add final continue exit using template helper
    body_plans.push(CorePlan::Exit(parts::exit::build_continue_with_phi_args(
        builder,
        &frame.carrier_step_phis,
        &current_bindings,
        LOOP_COND_CONTINUE_ONLY_ERR,
    )?));

    // Generate PHIs using template (StepBb mode: step + header PHIs)
    let phis = build_header_step_phis(&frame, "loop_cond_continue_only")?;

    // Build final_values from header PHIs
    let final_values: Vec<(String, crate::mir::ValueId)> = frame
        .carrier_header_phis
        .iter()
        .map(|(var, phi_dst)| (var.clone(), *phi_dst))
        .collect();

    // Build block_effects: merge header_result.block_effects + static entries
    let mut block_effects: Vec<(crate::mir::BasicBlockId, Vec<CoreEffectPlan>)> =
        vec![(preheader_bb, vec![])];
    for (bb, effects) in header_result.block_effects {
        block_effects.push((bb, effects));
    }
    block_effects.push((body_bb, vec![]));
    block_effects.push((step_bb, vec![]));

    let (step_mode, has_explicit_step) = step_mode::inline_in_body_no_explicit_step();

    Ok(CorePlan::Loop(CoreLoopPlan {
        preheader_bb,
        preheader_is_fresh: false,
        header_bb,
        body_bb,
        step_bb,
        continue_target,
        after_bb,
        found_bb: after_bb,
        body: body_plans,
        cond_loop: header_result.first_cond,
        cond_match: header_result.first_cond,
        block_effects,
        phis,
        frag,
        final_values,
        step_mode,
        has_explicit_step,
    }))
}
