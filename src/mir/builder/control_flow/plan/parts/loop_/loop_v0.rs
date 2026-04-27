use crate::config::env::joinir_dev;
use crate::mir::builder::control_flow::edgecfg::api::Frag;
use crate::mir::builder::control_flow::facts::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::plan::features::carriers;
use crate::mir::builder::control_flow::plan::features::coreloop_frame::{
    build_coreloop_frame, build_header_step_phis,
};
use crate::mir::builder::control_flow::plan::features::loop_carriers;
use crate::mir::builder::control_flow::plan::features::step_mode;
use crate::mir::builder::control_flow::plan::normalizer::lower_loop_header_cond;
use crate::mir::builder::control_flow::plan::planner::Freeze;
use crate::mir::builder::control_flow::plan::recipe_tree::{
    BlockContractKind, RecipeBlock, RecipeBodies,
};
use crate::mir::builder::control_flow::plan::steps::empty_carriers_args;
use crate::mir::builder::control_flow::plan::{
    CoreEffectPlan, CoreExitPlan, CoreLoopPlan, CorePhiInfo, CorePlan, LoweredRecipe,
};
use crate::mir::builder::MirBuilder;
use crate::mir::{ConstValue, MirType, ValueId};
use std::collections::{BTreeMap, BTreeSet};

use super::super::{dispatch, exit, stmt as parts_stmt};
use super::analysis::collect_defined_values_from_plans;
use super::debug::debug_log_block_effects_binop_lit3;
use super::vars::{collect_assigned_vars, collect_carriers_from_condition, condition_vars};

/// Lower a `RecipeItem::LoopV0` using only RecipeTree (`RecipeBlock`) + `CondBlockView`.
///
/// Contract:
/// - Does NOT call `lower_nested_loop_depth1_any` / AST-loop lowering.
/// - Uses Verified lowering for the body contract.
/// - Emits fallthrough `ContinueWithPhiArgs` to populate step-join PHIs.
pub(in crate::mir::builder) fn lower_loop_v0(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    cond_view: &CondBlockView,
    body_contract: BlockContractKind,
    arena: &RecipeBodies,
    body_block: &RecipeBlock,
    error_prefix: &str,
) -> Result<LoweredRecipe, String> {
    const LOOP_V0_ERR: &str = "[freeze:contract][loop_v0]";
    let strict_or_dev = joinir_dev::strict_enabled() || crate::config::env::joinir_dev_enabled();
    let planner_required = strict_or_dev && joinir_dev::planner_required_enabled();
    let contract_err = |detail: &str| -> String {
        if planner_required {
            Freeze::contract(format!("loop_v0 {detail} ctx={error_prefix}")).to_string()
        } else {
            format!("{LOOP_V0_ERR} {detail} ctx={error_prefix}")
        }
    };

    let body_recipe = arena
        .get(body_block.body_id)
        .ok_or_else(|| format!("{LOOP_V0_ERR} invalid_body_id: ctx={error_prefix}"))?;
    let pre_loop_map = builder.variable_ctx.variable_map.clone();

    let mut carrier_vars = BTreeSet::from_iter(carriers::collect_from_body(&body_recipe.body).vars);
    let mut assigned_vars = BTreeSet::new();
    for stmt in &body_recipe.body {
        collect_assigned_vars(stmt, &mut assigned_vars);
    }
    for name in assigned_vars {
        if current_bindings.contains_key(&name)
            || builder.variable_ctx.variable_map.contains_key(&name)
        {
            carrier_vars.insert(name);
        }
    }
    if carrier_vars.is_empty() {
        carrier_vars = collect_carriers_from_condition(
            condition_vars(&cond_view.tail_expr),
            builder,
            current_bindings,
        );
        if carrier_vars.is_empty() {
            return Err(contract_err("no_loop_carriers:"));
        }
    }

    let mut carrier_inits = BTreeMap::new();
    for var in &carrier_vars {
        let init_val = current_bindings
            .get(var)
            .copied()
            .or_else(|| builder.variable_ctx.variable_map.get(var).copied())
            .ok_or_else(|| contract_err(&format!("carrier_init_missing var={var}")))?;
        carrier_inits.insert(var.clone(), init_val);
    }

    let frame = build_coreloop_frame(builder, &carrier_vars, &carrier_inits, LOOP_V0_ERR)?;

    let mut break_phi_dsts = BTreeMap::new();
    let mut after_phis: Vec<CorePhiInfo> = Vec::new();
    for var in &carrier_vars {
        let Some(&init_val) = carrier_inits.get(var) else {
            continue;
        };
        let Some(&header_phi_dst) = frame.carrier_header_phis.get(var) else {
            continue;
        };
        let ty = builder
            .type_ctx
            .get_type(init_val)
            .cloned()
            .unwrap_or(MirType::Unknown);
        let after_phi_dst = builder.alloc_typed(ty);
        break_phi_dsts.insert(var.clone(), after_phi_dst);
        after_phis.push(loop_carriers::build_after_merge_phi_info(
            frame.after_bb,
            after_phi_dst,
            [frame.header_bb],
            header_phi_dst,
            format!("loop_v0_after_{}", var),
        ));
    }

    // Bind carriers to header PHI values during condition + body lowering.
    let mut loop_bindings = current_bindings.clone();
    for (name, value_id) in &frame.carrier_header_phis {
        loop_bindings.insert(name.clone(), *value_id);
        builder
            .variable_ctx
            .variable_map
            .insert(name.clone(), *value_id);
    }

    // Header short-circuit lowering (CondBlockView-first).
    let header_result = lower_loop_header_cond(
        builder,
        &loop_bindings,
        cond_view,
        frame.header_bb,
        frame.body_bb,
        frame.after_bb,
        empty_carriers_args(),
        empty_carriers_args(),
        LOOP_V0_ERR,
    )?;

    // Body lowering (Verifier-gated), in a fresh binding map.
    let mut body_bindings = loop_bindings;
    let mut pre_bindings_for_verify = body_bindings.clone();
    for var in &carrier_vars {
        if pre_bindings_for_verify.contains_key(var) {
            continue;
        }
        if let Some(value_id) = builder.variable_ctx.variable_map.get(var) {
            pre_bindings_for_verify.insert(var.clone(), *value_id);
        }
    }
    let body_entry_bindings = body_bindings.clone();
    let body_plans = match body_contract {
        BlockContractKind::StmtOnly => {
            let verified = super::super::entry::verify_stmt_only_block_with_pre(
                arena,
                body_block,
                LOOP_V0_ERR,
                Some(&pre_bindings_for_verify),
            )?;
            super::super::entry::lower_stmt_only_block_verified(
                builder,
                &mut body_bindings,
                &frame.carrier_step_phis,
                Some(&break_phi_dsts),
                verified,
                LOOP_V0_ERR,
                |builder, bindings, carrier_step_phis, break_phi_dsts, stmt, error_prefix| {
                    parts_stmt::lower_return_prelude_stmt(
                        builder,
                        bindings,
                        carrier_step_phis,
                        break_phi_dsts,
                        stmt,
                        error_prefix,
                    )
                },
            )?
        }
        BlockContractKind::NoExit => {
            let verified = super::super::entry::verify_no_exit_block_with_pre(
                arena,
                body_block,
                LOOP_V0_ERR,
                Some(&pre_bindings_for_verify),
            )?;
            super::super::entry::lower_no_exit_block_verified(
                builder,
                &mut body_bindings,
                &frame.carrier_step_phis,
                Some(&break_phi_dsts),
                verified,
                LOOP_V0_ERR,
            )?
        }
        BlockContractKind::ExitAllowed => {
            let verified = super::super::entry::verify_exit_allowed_block_with_pre(
                arena,
                body_block,
                LOOP_V0_ERR,
                Some(&pre_bindings_for_verify),
            )?;
            super::super::entry::lower_exit_allowed_block_verified(
                builder,
                &mut body_bindings,
                &frame.carrier_step_phis,
                &break_phi_dsts,
                verified,
                LOOP_V0_ERR,
            )?
        }
        BlockContractKind::ExitOnly => {
            let verified = super::super::entry::verify_exit_only_block_with_pre(
                arena,
                body_block,
                LOOP_V0_ERR,
                Some(&pre_bindings_for_verify),
            )?;
            super::super::entry::lower_exit_only_block_verified(
                builder,
                &mut body_bindings,
                &frame.carrier_step_phis,
                &break_phi_dsts,
                verified,
                LOOP_V0_ERR,
            )?
        }
    };

    // Fallthrough: explicit backedge with carrier values (fills step-join PHIs).
    let mut body_plans = body_plans;
    let mut body_defined_values = BTreeSet::new();
    collect_defined_values_from_plans(&body_plans, &mut body_defined_values);
    let body_entry_values: BTreeSet<ValueId> = body_entry_bindings.values().copied().collect();
    if crate::config::env::joinir_dev::strict_planner_required_debug_enabled() {
        let mut lit3_dsts = Vec::new();
        let mut lit3_spans = Vec::new();
        for plan in &body_plans {
            if let CorePlan::Effect(CoreEffectPlan::Const { dst, value }) = plan {
                if matches!(value, ConstValue::Integer(3)) {
                    if let Some(span) = builder.metadata_ctx.value_span(*dst) {
                        lit3_dsts.push(*dst);
                        lit3_spans.push(span.to_string());
                    }
                }
            }
        }
        if !lit3_dsts.is_empty() {
            let fn_name = builder
                .scope_ctx
                .current_function
                .as_ref()
                .map(|f| f.signature.name.as_str())
                .unwrap_or("<none>");
            let const_int3_dsts = lit3_dsts
                .iter()
                .map(|v| format!("%{}", v.0))
                .collect::<Vec<_>>()
                .join(",");
            let origin_spans = lit3_spans.join(",");
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!(
                "[loop_parts/body_plans:lit3_origin] fn={} bb={:?} plans_len={} const_int3_dsts=[{}] origin_spans=[{}]",
                fn_name,
                builder.current_block,
                body_plans.len(),
                const_int3_dsts,
                origin_spans
            ));
        }
    }
    let body_exits_all_paths = dispatch::plans_exit_on_all_paths(&body_plans);
    if !body_exits_all_paths {
        for (name, _) in &frame.carrier_step_phis {
            let selected = builder
                .variable_ctx
                .variable_map
                .get(name)
                .copied()
                .and_then(|candidate| {
                    if body_defined_values.contains(&candidate)
                        || body_entry_values.contains(&candidate)
                    {
                        Some(candidate)
                    } else {
                        body_entry_bindings.get(name).copied()
                    }
                })
                .or_else(|| body_entry_bindings.get(name).copied());
            if let Some(value_id) = selected {
                body_bindings.insert(name.clone(), value_id);
            } else if planner_required {
                return Err(contract_err(&format!("carrier_map_missing var={name}")));
            }
        }
        body_plans.push(CorePlan::Exit(exit::build_continue_with_phi_args(
            builder,
            &frame.carrier_step_phis,
            &body_bindings,
            LOOP_V0_ERR,
        )?));
    }

    // Build block_effects: merge header_result.block_effects + static entries
    let mut block_effects: Vec<(crate::mir::BasicBlockId, Vec<CoreEffectPlan>)> =
        vec![(frame.preheader_bb, vec![])];
    for (bb, effects) in header_result.block_effects {
        debug_log_block_effects_binop_lit3(builder, &effects);
        block_effects.push((bb, effects));
    }
    block_effects.push((frame.body_bb, vec![]));
    block_effects.push((frame.step_bb, vec![]));
    block_effects.push((frame.after_bb, vec![]));

    // PHIs: only allocate step-join PHIs when a continue edge can actually populate them.
    let body_has_continue_edge = plans_require_continue_phi_args(&body_plans);
    let mut phis = if body_has_continue_edge {
        build_header_step_phis(&frame, "loop_v0")?
    } else {
        let mut phis = Vec::new();
        for (var, header_phi_dst) in &frame.carrier_header_phis {
            let Some(&init_val) = frame.carrier_inits.get(var) else {
                return Err(format!(
                    "[coreloop_skeleton] loop_v0: carrier_inits missing '{}' during PHI build",
                    var
                ));
            };
            phis.push(loop_carriers::build_preheader_only_phi_info(
                frame.header_bb,
                frame.preheader_bb,
                *header_phi_dst,
                init_val,
                format!("loop_v0_carrier_{}", var),
            ));
        }
        phis
    };
    phis.append(&mut after_phis);

    // Frag: short-circuit branches + standard5 internal wires.
    let wires = crate::mir::builder::control_flow::plan::steps::build_standard5_internal_wires(
        &frame,
        empty_carriers_args(),
    );
    let frag = Frag {
        entry: frame.header_bb,
        block_params: BTreeMap::new(),
        exits: BTreeMap::new(),
        wires,
        branches: header_result.branches,
    };

    // Final values: after-phi dsts for carrier vars.
    // Restore the outer lexical map first so loop-body locals do not leak past
    // the loop boundary. Only carrier final values are allowed to escape.
    builder.variable_ctx.variable_map = pre_loop_map;
    let mut final_values = Vec::new();
    for (name, after_phi_dst) in &break_phi_dsts {
        final_values.push((name.clone(), *after_phi_dst));
        builder
            .variable_ctx
            .variable_map
            .insert(name.clone(), *after_phi_dst);
        if current_bindings.contains_key(name) {
            current_bindings.insert(name.clone(), *after_phi_dst);
        }
    }

    let (step_mode, has_explicit_step) = step_mode::inline_in_body_no_explicit_step();

    Ok(CorePlan::Loop(CoreLoopPlan {
        preheader_bb: frame.preheader_bb,
        preheader_is_fresh: false,
        header_bb: frame.header_bb,
        body_bb: frame.body_bb,
        step_bb: frame.step_bb,
        continue_target: frame.continue_target,
        after_bb: frame.after_bb,
        found_bb: frame.after_bb,
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

fn plans_require_continue_phi_args(plans: &[LoweredRecipe]) -> bool {
    plans.iter().any(plan_requires_continue_phi_args)
}

fn plan_requires_continue_phi_args(plan: &LoweredRecipe) -> bool {
    match plan {
        CorePlan::Exit(CoreExitPlan::ContinueWithPhiArgs { .. } | CoreExitPlan::Continue(_)) => {
            true
        }
        CorePlan::If(if_plan) => {
            plans_require_continue_phi_args(&if_plan.then_plans)
                || if_plan
                    .else_plans
                    .as_ref()
                    .is_some_and(|plans| plans_require_continue_phi_args(plans))
        }
        CorePlan::BranchN(branch) => {
            branch
                .arms
                .iter()
                .any(|arm| plans_require_continue_phi_args(&arm.plans))
                || branch
                    .else_plans
                    .as_ref()
                    .is_some_and(|plans| plans_require_continue_phi_args(plans))
        }
        CorePlan::Seq(plans) => plans_require_continue_phi_args(plans),
        _ => false,
    }
}
