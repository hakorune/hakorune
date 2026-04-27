//! Phase 29bq P2.x: LoopCondContinueWithReturn Pipeline
//!
//! Minimal implementation for continue-only loops with nested return.
//! Reuses exit_if_map (return PHI) + continue-if handling (continue PHI).

use crate::mir::builder::control_flow::facts::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;
use crate::mir::builder::control_flow::edgecfg::api::Frag;
use crate::mir::builder::control_flow::plan::features::body_view::BodyView;
use crate::mir::builder::control_flow::plan::features::carriers;
use crate::mir::builder::control_flow::plan::features::coreloop_frame::build_coreloop_frame;
use crate::mir::builder::control_flow::plan::features::loop_cond_continue_with_return_body_helpers::lower_continue_with_return_block;
use crate::mir::builder::control_flow::plan::features::loop_cond_continue_with_return_cleanup::apply_fallthrough_continue_exit;
use crate::mir::builder::control_flow::plan::features::loop_cond_continue_with_return_phi_materializer::LoopCondContinueWithReturnPhiMaterializer;
use crate::mir::builder::control_flow::facts::loop_cond_continue_with_return::LoopCondContinueWithReturnFacts;
use crate::mir::builder::control_flow::plan::features::loop_cond_continue_with_return_verifier::verify_loop_cond_continue_with_return_phi_closure;
use crate::mir::builder::control_flow::plan::features::step_mode;
use crate::mir::builder::control_flow::plan::normalizer::lower_loop_header_cond;
use crate::mir::builder::control_flow::plan::steps::{
    build_standard5_internal_wires, collect_carrier_inits, empty_carriers_args,
};
use crate::mir::builder::control_flow::plan::{
    CoreEffectPlan, CoreLoopPlan, CorePlan, LoweredRecipe,
};
use crate::mir::builder::MirBuilder;
use std::collections::{BTreeMap, BTreeSet};

const LOOP_COND_CONTINUE_WITH_RETURN_ERR: &str = "[normalizer] loop_cond_continue_with_return";

/// Helper for compact map/set trace logging.
/// Format: `[plan/trace] tag: len=N`
#[inline]
fn trace_collection_len(tag: &str, len: usize) {
    if crate::config::env::is_joinir_debug() {
        let ring0 = crate::runtime::get_global_ring0();
        ring0
            .log
            .debug(&format!("[plan/trace] {}: len={}", tag, len));
    }
}

pub(in crate::mir::builder) fn lower_loop_cond_continue_with_return(
    builder: &mut MirBuilder,
    facts: LoopCondContinueWithReturnFacts,
    _ctx: &LoopRouteContext,
) -> Result<LoweredRecipe, String> {
    let carrier_sets = carriers::collect_from_recipe_continue_with_return(&facts.recipe);
    let carrier_vars: BTreeSet<String> = carrier_sets.vars.into_iter().collect();

    let carrier_inits = collect_carrier_inits(
        builder,
        carrier_vars.iter().cloned(),
        LOOP_COND_CONTINUE_WITH_RETURN_ERR,
    )?;

    lower_loop_cond_continue_with_return_stepbb(builder, facts, &carrier_vars, &carrier_inits)
}

fn lower_loop_cond_continue_with_return_stepbb(
    builder: &mut MirBuilder,
    facts: LoopCondContinueWithReturnFacts,
    carrier_vars: &BTreeSet<String>,
    carrier_inits: &BTreeMap<String, crate::mir::ValueId>,
) -> Result<LoweredRecipe, String> {
    let frame = build_coreloop_frame(
        builder,
        carrier_vars,
        carrier_inits,
        LOOP_COND_CONTINUE_WITH_RETURN_ERR,
    )?;

    let preheader_bb = frame.preheader_bb;
    let header_bb = frame.header_bb;
    let body_bb = frame.body_bb;
    let step_bb = frame.step_bb;
    let after_bb = frame.after_bb;
    let phi_materializer = LoopCondContinueWithReturnPhiMaterializer::prepare(builder, &frame);
    let continue_target = phi_materializer.continue_target();

    if frame.carrier_header_phis.is_empty() && !carrier_vars.is_empty() {
        return Err(format!(
            "{LOOP_COND_CONTINUE_WITH_RETURN_ERR}: no loop carriers"
        ));
    }

    let mut current_bindings = phi_materializer.current_bindings().clone();
    trace_collection_len("init_current_bindings", current_bindings.len());

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
        LOOP_COND_CONTINUE_WITH_RETURN_ERR,
    )?;

    let internal_wires = build_standard5_internal_wires(&frame, empty_carriers_args());
    let frag = Frag {
        entry: header_bb,
        block_params: BTreeMap::new(),
        exits: BTreeMap::new(),
        wires: internal_wires,
        branches: header_result.branches,
    };

    let mut carrier_updates = BTreeMap::new();
    let body_view = BodyView::Recipe(&facts.recipe.body);
    let mut body_plans = lower_continue_with_return_block(
        builder,
        &mut current_bindings,
        &frame.carrier_header_phis,
        &frame.carrier_step_phis,
        &mut carrier_updates,
        &body_view,
        &facts.recipe.items,
        LOOP_COND_CONTINUE_WITH_RETURN_ERR,
    )?;

    trace_collection_len("carrier_vars", carrier_vars.len());
    trace_collection_len("carrier_inits", carrier_inits.len());
    trace_collection_len("current_bindings", current_bindings.len());
    trace_collection_len("carrier_step_phis", frame.carrier_step_phis.len());

    apply_fallthrough_continue_exit(
        builder,
        &mut body_plans,
        &frame.carrier_step_phis,
        &current_bindings,
        LOOP_COND_CONTINUE_WITH_RETURN_ERR,
    )?;

    let phi_closure = phi_materializer.close(&frame)?;
    verify_loop_cond_continue_with_return_phi_closure(
        &phi_closure,
        &body_plans,
        continue_target,
        step_bb,
        frame.carrier_header_phis.len(),
        LOOP_COND_CONTINUE_WITH_RETURN_ERR,
    )?;

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
        phis: phi_closure.phis().to_vec(),
        frag,
        final_values: phi_closure.final_values().to_vec(),
        step_mode,
        has_explicit_step,
    }))
}
