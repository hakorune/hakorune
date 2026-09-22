//! Located-source sibling of `lower_loop_cond_break_continue`.
//!
//! This entry consumes the already co-sealed `SourceLoopCondPhysicalInputV1`
//! end to end: the issued Facts/Recipe stay the semantic authority while the
//! located source port supplies the condition expression and body statement
//! carriers. Carrier collection still observes the co-sealed condition/body
//! AST (it collects names only, issuing no authority), and the body items run
//! through `lower_loop_cond_source_item` — never through the raw
//! `lower_loop_cond_item` or `LoopRouteContext`.
//!
//! Must-not: no AST/name remap, no second recipe/JoinSig authority, no
//! unconditional fallback, no route re-entry.

use crate::mir::builder::control_flow::edgecfg::api::Frag;
use crate::mir::builder::control_flow::facts::loop_cond_break_continue::LoopCondBreakContinueFacts;
use crate::mir::builder::control_flow::plan::features::carriers;
use crate::mir::builder::control_flow::plan::features::edgecfg_stubs;
use crate::mir::builder::control_flow::plan::features::loop_cond_bc::{
    collect_carrier_vars_from_condition, extend_unique_carriers, pin_accept_kind_contract,
    LOOP_COND_ERR,
};
use crate::mir::builder::control_flow::plan::features::loop_cond_bc_cleanup::apply_loop_cond_break_continue_cleanup;
use crate::mir::builder::control_flow::plan::features::loop_cond_bc_phi_materializer::LoopCondBreakContinuePhiMaterializer;
use crate::mir::builder::control_flow::plan::features::loop_cond_bc_verifier::verify_loop_cond_break_continue_phi_closure;
use crate::mir::builder::control_flow::plan::features::step_mode;
use crate::mir::builder::control_flow::plan::normalizer::cond_lowering_loop_header::lower_loop_header_cond_with_port;
use crate::mir::builder::control_flow::plan::normalizer::helpers::LoopBlocksStandard5;
use crate::mir::builder::control_flow::plan::parts;
use crate::mir::builder::control_flow::plan::steps::empty_carriers_args;
use crate::mir::builder::control_flow::plan::{
    lower_loop_cond_source_item, CoreEffectPlan, CoreLoopPlan, CorePlan, LoweredRecipe,
};
use crate::mir::builder::normal_callable_loop_source_facts::SourceLoopCondPhysicalInputV1;
use crate::mir::builder::normal_callable_loop_source_port::{
    CallableLoopSourceBodyInputV1, CallableLoopSourceExpressionPortV1,
};
use crate::mir::builder::MirBuilder;
use crate::mir::policies::BodyLoweringPolicy;
use crate::mir::ValueId;
use std::collections::BTreeMap;

/// Lower the selected LoopCond route through the located source port.
///
/// `input` is the move-only co-sealed transport issued by the same source
/// owner; `validate_for_source_port` re-runs here so the physical boundary
/// still holds when a caller forgets it. Every failure is a named terminal —
/// this entry never falls back to the raw `LoopRouteContext` route.
pub(in crate::mir::builder) fn lower_loop_cond_break_continue_source(
    builder: &mut MirBuilder,
    input: &SourceLoopCondPhysicalInputV1<'_, '_>,
) -> Result<LoweredRecipe, String> {
    input.validate_for_source_port()?;
    let facts = input.loop_cond_facts()?;
    pin_accept_kind_contract(facts.accept_kind);

    let blocks = LoopBlocksStandard5::allocate(builder)?;
    let LoopBlocksStandard5 {
        preheader_bb,
        header_bb,
        body_bb,
        step_bb,
        after_bb,
    } = blocks;

    // Item 5: carrier discovery still observes the co-sealed condition/body
    // AST; it collects names only and issues no authority.
    let carrier_sets = carriers::collect_outer_from_body(builder, input.body());
    let mut carrier_vars = carrier_sets.vars;
    extend_unique_carriers(
        &mut carrier_vars,
        collect_carrier_vars_from_condition(builder, input.condition()),
    );
    let continue_branch_has_prelude_effect = facts
        .continue_branches
        .iter()
        .any(|sig| sig.has_assignment || sig.has_local);
    let use_header_continue_target = crate::config::env::joinir_dev::strict_enabled()
        && crate::config::env::joinir_dev::planner_required_enabled()
        && !facts.continue_branches.is_empty()
        && !continue_branch_has_prelude_effect;
    let phi_materializer = LoopCondBreakContinuePhiMaterializer::prepare(
        builder,
        &carrier_vars,
        use_header_continue_target,
        !facts.continue_branches.is_empty(),
        header_bb,
        step_bb,
        LOOP_COND_ERR,
    )?;
    let carrier_phis = phi_materializer.carrier_phis().clone();
    let carrier_step_phis = phi_materializer.carrier_step_phis().clone();
    let break_phi_dsts = phi_materializer.break_phi_dsts().clone();

    let mut current_bindings = builder.function_state.variable_ctx.variable_map.clone();
    for (name, value_id) in phi_materializer.phi_bindings() {
        current_bindings.insert(name.clone(), value_id);
    }

    let port = *input.source_port();
    let condition_input = port
        .expr(input.condition(), input.condition_source())
        .map_err(|error| format!("{LOOP_COND_ERR}: source condition input: {error}"))?;
    let header_result = lower_loop_header_cond_with_port(
        builder,
        &current_bindings,
        &port,
        condition_input,
        header_bb,
        body_bb,
        after_bb,
        empty_carriers_args(),
        empty_carriers_args(),
        LOOP_COND_ERR,
    )?;

    let mut after_cond_preds = header_result.preds_to(after_bb);
    if after_cond_preds.is_empty() {
        after_cond_preds.insert(header_bb);
    }

    for (name, value_id) in &carrier_phis {
        current_bindings.insert(name.clone(), *value_id);
        parts::var_map_scope::publish_emission_cache(builder, name.clone(), *value_id);
    }

    let wires = vec![
        edgecfg_stubs::build_loop_back_edge(body_bb, step_bb),
        edgecfg_stubs::build_loop_back_edge(step_bb, header_bb),
    ];

    let frag = Frag {
        entry: header_bb,
        block_params: BTreeMap::new(),
        exits: BTreeMap::new(),
        wires,
        branches: header_result.branches,
    };

    let body_carrier = port
        .body(input.body(), input.body_source())
        .map_err(|error| format!("{LOOP_COND_ERR}: source body input: {error}"))?;
    let mut body_plans = match facts.body_lowering_policy {
        BodyLoweringPolicy::ExitAllowed { .. } => {
            let Some(body_exit_allowed) = facts.body_exit_allowed.as_ref() else {
                return Err(format!(
                    "[freeze:contract][loop_cond_break_continue] body_lowering_policy=ExitAllowed but body_exit_allowed=None: ctx={LOOP_COND_ERR}"
                ));
            };
            lower_loop_cond_source_body(
                port,
                &body_carrier,
                body_exit_allowed,
                facts,
                builder,
                &mut current_bindings,
                &carrier_phis,
                &carrier_step_phis,
                &break_phi_dsts,
            )?
        }
        BodyLoweringPolicy::RecipeOnly => {
            if facts.body_exit_allowed.is_some() {
                return Err(format!(
                    "[freeze:contract][loop_cond_break_continue] RecipeOnly with body_exit_allowed: ctx={LOOP_COND_ERR}"
                ));
            }
            lower_loop_cond_source_items(
                port,
                &body_carrier,
                facts,
                builder,
                &mut current_bindings,
                &carrier_phis,
                &carrier_step_phis,
                &break_phi_dsts,
            )?
        }
    };

    let body_entry_bindings = current_bindings.clone();
    let cleanup = apply_loop_cond_break_continue_cleanup(
        builder,
        &mut body_plans,
        &carrier_step_phis,
        &current_bindings,
        &body_entry_bindings,
        LOOP_COND_ERR,
    )?;
    let body_exits_all_paths = cleanup.body_exits_all_paths();
    let phi_closure = phi_materializer.close(
        preheader_bb,
        header_bb,
        step_bb,
        after_bb,
        &after_cond_preds,
        body_exits_all_paths,
    )?;
    verify_loop_cond_break_continue_phi_closure(
        &phi_closure,
        &body_plans,
        &break_phi_dsts,
        phi_materializer.continue_target(),
        header_bb,
        step_bb,
        use_header_continue_target,
        body_exits_all_paths,
        !facts.continue_branches.is_empty(),
        carrier_phis.len(),
        LOOP_COND_ERR,
    )?;

    let mut block_effects: Vec<(crate::mir::BasicBlockId, Vec<CoreEffectPlan>)> =
        vec![(preheader_bb, vec![])];
    for (bb, effects) in header_result.block_effects {
        block_effects.push((bb, effects));
    }
    block_effects.push((body_bb, vec![]));
    block_effects.push((step_bb, vec![]));
    block_effects.push((after_bb, vec![]));

    let continue_target = phi_materializer.continue_target();

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
        final_values: phi_closure.final_values().to_vec().into(),
        step_mode,
        has_explicit_step,
    }))
}

/// `BodyLoweringPolicy::ExitAllowed` arm: drive the issued exit-allowed body
/// recipe through the located block driver, falling back to the per-item loop
/// only on the same "if body must be single-exit" boundary the raw owner uses.
#[allow(clippy::too_many_arguments)]
fn lower_loop_cond_source_body<'view, 'ledger: 'view>(
    port: CallableLoopSourceExpressionPortV1<'ledger>,
    body: &CallableLoopSourceBodyInputV1<'view>,
    body_exit_allowed: &crate::mir::builder::control_flow::plan::facts::exit_only_block::ExitAllowedBlockRecipe,
    facts: &LoopCondBreakContinueFacts,
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, ValueId>,
    carrier_phis: &BTreeMap<String, ValueId>,
    carrier_step_phis: &BTreeMap<String, ValueId>,
    break_phi_dsts: &BTreeMap<String, ValueId>,
) -> Result<Vec<LoweredRecipe>, String> {
    let mut carrier_updates = BTreeMap::new();
    parts::lower_loop_cond_source_exit_allowed_body(
        port,
        body,
        body_exit_allowed,
        builder,
        current_bindings,
        carrier_phis,
        carrier_step_phis,
        break_phi_dsts,
        &mut carrier_updates,
        LOOP_COND_ERR,
    )
    .or_else(|err| {
        if !err.contains("if body must be single-exit") {
            return Err(err);
        }
        lower_loop_cond_source_items_inner(
            port,
            body,
            facts,
            builder,
            current_bindings,
            carrier_phis,
            carrier_step_phis,
            break_phi_dsts,
            carrier_updates,
        )
    })
}

/// `BodyLoweringPolicy::RecipeOnly` arm: drive each issued item through the
/// located dispatcher against the co-sealed body carrier.
#[allow(clippy::too_many_arguments)]
fn lower_loop_cond_source_items<'view, 'ledger: 'view>(
    port: CallableLoopSourceExpressionPortV1<'ledger>,
    body: &CallableLoopSourceBodyInputV1<'view>,
    facts: &LoopCondBreakContinueFacts,
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, ValueId>,
    carrier_phis: &BTreeMap<String, ValueId>,
    carrier_step_phis: &BTreeMap<String, ValueId>,
    break_phi_dsts: &BTreeMap<String, ValueId>,
) -> Result<Vec<LoweredRecipe>, String> {
    lower_loop_cond_source_items_inner(
        port,
        body,
        facts,
        builder,
        current_bindings,
        carrier_phis,
        carrier_step_phis,
        break_phi_dsts,
        BTreeMap::new(),
    )
}

#[allow(clippy::too_many_arguments)]
fn lower_loop_cond_source_items_inner<'view, 'ledger: 'view>(
    port: CallableLoopSourceExpressionPortV1<'ledger>,
    body: &CallableLoopSourceBodyInputV1<'view>,
    facts: &LoopCondBreakContinueFacts,
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, ValueId>,
    carrier_phis: &BTreeMap<String, ValueId>,
    carrier_step_phis: &BTreeMap<String, ValueId>,
    break_phi_dsts: &BTreeMap<String, ValueId>,
    mut carrier_updates: BTreeMap<String, ValueId>,
) -> Result<Vec<LoweredRecipe>, String> {
    let mut body_plans = Vec::new();
    for (idx, item) in facts.recipe.items.iter().enumerate() {
        let mut plans = lower_loop_cond_source_item(
            port,
            body,
            idx,
            item,
            builder,
            current_bindings,
            carrier_phis,
            carrier_step_phis,
            break_phi_dsts,
            &mut carrier_updates,
            LOOP_COND_ERR,
        )
        .map_err(|err| format!("{err} [loop_cond_item idx={idx} kind={item:?}]"))?;
        body_plans.append(&mut plans);
    }
    Ok(body_plans)
}
