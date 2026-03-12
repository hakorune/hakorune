//! Loop parts (scaffold).
//!
//! Purpose (L0):
//! - Provide a Parts entry for lowering a loop body represented as `RecipeBlock`.
//! - Keep the contract explicit and fail-fast (no silent fallback).
//!
//! NOTE:
//! - This is an implementation-prep step. Producers are unchanged.

use crate::config::env::joinir_dev;
use crate::mir::builder::control_flow::plan::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::plan::edgecfg_facade::Frag;
use crate::mir::builder::control_flow::plan::facts::stmt_view::StmtOnlyBlockRecipe;
use crate::mir::builder::control_flow::plan::features::carriers;
use crate::mir::builder::control_flow::plan::features::coreloop_frame::{
    build_coreloop_frame, build_header_step_phis,
};
use crate::mir::builder::control_flow::plan::features::loop_carriers;
use crate::mir::builder::control_flow::plan::features::nested_loop_depth1::lower_nested_loop_depth1_any;
use crate::mir::builder::control_flow::plan::features::step_mode;
use crate::mir::builder::control_flow::plan::nested_loop_depth1::try_lower_nested_loop_depth1;
use crate::mir::builder::control_flow::plan::normalizer::lower_loop_header_cond;
use crate::mir::builder::control_flow::plan::planner::Freeze;
use crate::mir::builder::control_flow::plan::recipe_tree::{
    BlockContractKind, RecipeBlock, RecipeBodies,
};
use crate::mir::builder::control_flow::plan::scan_loop_segments::NestedLoopRecipe;
use crate::mir::builder::control_flow::plan::steps::empty_carriers_args;
use crate::mir::builder::control_flow::plan::{
    CoreEffectPlan, CoreLoopPlan, CorePhiInfo, CorePlan, LoweredRecipe,
};
use crate::mir::builder::MirBuilder;
use crate::mir::{BinaryOp, ConstValue, MirType, ValueId};
use std::collections::{BTreeMap, BTreeSet};

use super::{stmt as parts_stmt, verify};

pub(in crate::mir::builder) type LoopBodyContractKind = BlockContractKind;

pub(in crate::mir::builder) fn lower_loop_with_body_block(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    arena: &RecipeBodies,
    body_block: &RecipeBlock,
    contract: LoopBodyContractKind,
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    lower_loop_with_body_block_internal(
        builder,
        current_bindings,
        carrier_step_phis,
        None, // break_phi_dsts
        arena,
        body_block,
        contract,
        error_prefix,
    )
}

pub(in crate::mir::builder) fn lower_loop_with_body_block_with_break_phi_dsts(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: &BTreeMap<String, crate::mir::ValueId>,
    arena: &RecipeBodies,
    body_block: &RecipeBlock,
    contract: LoopBodyContractKind,
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    lower_loop_with_body_block_internal(
        builder,
        current_bindings,
        carrier_step_phis,
        Some(break_phi_dsts),
        arena,
        body_block,
        contract,
        error_prefix,
    )
}

fn lower_loop_with_body_block_internal(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: Option<&BTreeMap<String, crate::mir::ValueId>>,
    arena: &RecipeBodies,
    body_block: &RecipeBlock,
    contract: LoopBodyContractKind,
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    match contract {
        LoopBodyContractKind::StmtOnly => {
            let verified = super::entry::verify_stmt_only_block_with_pre(
                arena,
                body_block,
                error_prefix,
                Some(current_bindings),
            )?;
            super::entry::lower_stmt_only_block_verified(
                builder,
                current_bindings,
                carrier_step_phis,
                break_phi_dsts,
                verified,
                error_prefix,
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
            )
        }
        LoopBodyContractKind::NoExit => {
            verify::verify_no_exit_block_contract_if_enabled(arena, body_block, error_prefix)?;
            let verified = super::entry::verify_no_exit_block_with_pre(
                arena,
                body_block,
                error_prefix,
                Some(current_bindings),
            )?;
            super::entry::lower_no_exit_block_verified(
                builder,
                current_bindings,
                carrier_step_phis,
                break_phi_dsts,
                verified,
                error_prefix,
            )
        }
        LoopBodyContractKind::ExitAllowed => {
            let Some(break_phi_dsts) = break_phi_dsts else {
                return Err(format!(
                    "[freeze:contract][recipe] loop_body_contract_requires_break_phi_dsts: ctx={}",
                    error_prefix
                ));
            };
            let verified = super::entry::verify_exit_allowed_block_with_pre(
                arena,
                body_block,
                error_prefix,
                Some(current_bindings),
            )?;
            super::entry::lower_exit_allowed_block_verified(
                builder,
                current_bindings,
                carrier_step_phis,
                break_phi_dsts,
                verified,
                error_prefix,
            )
        }
        LoopBodyContractKind::ExitOnly => {
            let Some(break_phi_dsts) = break_phi_dsts else {
                return Err(format!(
                    "[freeze:contract][recipe] loop_body_contract_requires_break_phi_dsts: ctx={}",
                    error_prefix
                ));
            };
            let verified = super::entry::verify_exit_only_block_with_pre(
                arena,
                body_block,
                error_prefix,
                Some(current_bindings),
            )?;
            super::entry::lower_exit_only_block_verified(
                builder,
                current_bindings,
                carrier_step_phis,
                break_phi_dsts,
                verified,
                error_prefix,
            )
        }
    }
}

/// Lower a nested `loop(cond) { ... }` statement when the body is already represented
/// as a stmt-only `RecipeBlock` (Facts-provided payload).
///
/// This is a thin adapter to keep `features/*` free from re-scanning the inner loop body.
/// Behavior is intentionally aligned with the existing nested-loop lowering path.
pub(in crate::mir::builder) fn lower_nested_loop_depth1_stmt_only(
    builder: &mut MirBuilder,
    cond_view: &CondBlockView,
    body_recipe: &StmtOnlyBlockRecipe,
    error_prefix: &str,
) -> Result<LoweredRecipe, String> {
    if !cond_view.prelude_stmts.is_empty() {
        return Err(format!(
            "[freeze:contract][recipe] nested_loop_cond_prelude_unsupported: ctx={}",
            error_prefix
        ));
    }

    verify::verify_stmt_only_block_contract_if_enabled(
        &body_recipe.arena,
        &body_recipe.block,
        error_prefix,
    )?;

    let body = body_recipe
        .arena
        .get(body_recipe.block.body_id)
        .ok_or_else(|| {
            format!(
                "[freeze:contract][recipe] invalid_body_id: ctx={}",
                error_prefix
            )
        })?;

    match lower_nested_loop_depth1_any(builder, &cond_view.tail_expr, &body.body, error_prefix) {
        Ok(plan) => Ok(plan),
        Err(any_err) => match try_lower_nested_loop_depth1(
            builder,
            &cond_view.tail_expr,
            &body.body,
            error_prefix,
        )? {
            Some(plan) => Ok(plan),
            None => Err(any_err),
        },
    }
}

/// Lower a nested loop represented as `scan_loop_segments::NestedLoopRecipe` when the nested body
/// is available as a stmt-only recipe payload.
///
/// This is a scan-pipeline SSOT entry: it prefers the nested-loop stmt-only fastpath and otherwise
/// asks the caller to fall back to the single-planner route.
pub(in crate::mir::builder) fn lower_nested_loop_recipe_stmt_only(
    builder: &mut MirBuilder,
    _current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    _carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    _break_phi_dsts: &BTreeMap<String, crate::mir::ValueId>,
    nested: &NestedLoopRecipe,
    error_prefix: &str,
) -> Result<Option<Vec<LoweredRecipe>>, String> {
    let Some(body_stmt_only) = nested.body_stmt_only.as_ref() else {
        return Ok(None);
    };

    if !nested.cond_view.prelude_stmts.is_empty() {
        return Err(format!(
            "[freeze:contract][recipe] nested_loop_cond_prelude_unsupported: ctx={}",
            error_prefix
        ));
    }

    let plan = lower_nested_loop_depth1_stmt_only(
        builder,
        &nested.cond_view,
        body_stmt_only,
        error_prefix,
    )?;
    Ok(Some(vec![plan]))
}

/// Lower an exit-only `RecipeBlock` in loop context.
///
/// This is a thin Parts entry (BoxShape): producers should pass an already-built exit-only block.
pub(in crate::mir::builder) fn lower_loop_with_exit_only_body_block(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: &BTreeMap<String, crate::mir::ValueId>,
    arena: &RecipeBodies,
    body_block: &RecipeBlock,
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    let verified = super::entry::verify_exit_only_block_with_pre(
        arena,
        body_block,
        error_prefix,
        Some(current_bindings),
    )?;
    super::entry::lower_exit_only_block_verified(
        builder,
        current_bindings,
        carrier_step_phis,
        break_phi_dsts,
        verified,
        error_prefix,
    )
}

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
            let verified = super::entry::verify_stmt_only_block_with_pre(
                arena,
                body_block,
                LOOP_V0_ERR,
                Some(&pre_bindings_for_verify),
            )?;
            super::entry::lower_stmt_only_block_verified(
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
            let verified = super::entry::verify_no_exit_block_with_pre(
                arena,
                body_block,
                LOOP_V0_ERR,
                Some(&pre_bindings_for_verify),
            )?;
            super::entry::lower_no_exit_block_verified(
                builder,
                &mut body_bindings,
                &frame.carrier_step_phis,
                Some(&break_phi_dsts),
                verified,
                LOOP_V0_ERR,
            )?
        }
        BlockContractKind::ExitAllowed => {
            let verified = super::entry::verify_exit_allowed_block_with_pre(
                arena,
                body_block,
                LOOP_V0_ERR,
                Some(&pre_bindings_for_verify),
            )?;
            super::entry::lower_exit_allowed_block_verified(
                builder,
                &mut body_bindings,
                &frame.carrier_step_phis,
                &break_phi_dsts,
                verified,
                LOOP_V0_ERR,
            )?
        }
        BlockContractKind::ExitOnly => {
            let verified = super::entry::verify_exit_only_block_with_pre(
                arena,
                body_block,
                LOOP_V0_ERR,
                Some(&pre_bindings_for_verify),
            )?;
            super::entry::lower_exit_only_block_verified(
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
                if matches!(value, crate::mir::ConstValue::Integer(3)) {
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
    let body_exits_all_paths = super::dispatch::plans_exit_on_all_paths(&body_plans);
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
        body_plans.push(CorePlan::Exit(super::exit::build_continue_with_phi_args(
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

    // PHIs: step/header + after merge.
    let mut phis = build_header_step_phis(&frame, "loop_v0")?;
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

fn collect_defined_values_from_plans(plans: &[LoweredRecipe], out: &mut BTreeSet<ValueId>) {
    for plan in plans {
        match plan {
            CorePlan::Effect(effect) => collect_defined_values_from_effect(effect, out),
            CorePlan::If(if_plan) => {
                let then_has_exit = plans_have_non_local_exit(&if_plan.then_plans);
                let else_has_exit = if_plan
                    .else_plans
                    .as_ref()
                    .is_some_and(|plans| plans_have_non_local_exit(plans));
                if !then_has_exit && !else_has_exit {
                    for join in &if_plan.joins {
                        out.insert(join.dst);
                    }
                }
                collect_defined_values_from_plans(&if_plan.then_plans, out);
                if let Some(else_plans) = &if_plan.else_plans {
                    collect_defined_values_from_plans(else_plans, out);
                }
            }
            CorePlan::BranchN(branch) => {
                for arm in &branch.arms {
                    collect_defined_values_from_plans(&arm.plans, out);
                }
                if let Some(else_plans) = &branch.else_plans {
                    collect_defined_values_from_plans(else_plans, out);
                }
            }
            CorePlan::Seq(inner) => collect_defined_values_from_plans(inner, out),
            CorePlan::Loop(loop_plan) => collect_defined_values_from_plans(&loop_plan.body, out),
            CorePlan::Exit(_) => {}
        }
    }
}

fn plans_have_non_local_exit(plans: &[LoweredRecipe]) -> bool {
    plans.iter().any(plan_has_non_local_exit)
}

fn plan_has_non_local_exit(plan: &LoweredRecipe) -> bool {
    match plan {
        CorePlan::Exit(_) => true,
        CorePlan::Effect(effect) => effect_has_non_local_exit(effect),
        CorePlan::If(if_plan) => {
            plans_have_non_local_exit(&if_plan.then_plans)
                || if_plan
                    .else_plans
                    .as_ref()
                    .is_some_and(|plans| plans_have_non_local_exit(plans))
        }
        CorePlan::BranchN(branch) => {
            branch
                .arms
                .iter()
                .any(|arm| plans_have_non_local_exit(&arm.plans))
                || branch
                    .else_plans
                    .as_ref()
                    .is_some_and(|plans| plans_have_non_local_exit(plans))
        }
        CorePlan::Seq(inner) => plans_have_non_local_exit(inner),
        CorePlan::Loop(loop_plan) => {
            plans_have_non_local_exit(&loop_plan.body)
                || loop_plan
                    .block_effects
                    .iter()
                    .any(|(_, effects)| effects.iter().any(effect_has_non_local_exit))
        }
    }
}

fn effect_has_non_local_exit(effect: &CoreEffectPlan) -> bool {
    match effect {
        CoreEffectPlan::ExitIf { .. } => true,
        CoreEffectPlan::IfEffect {
            then_effects,
            else_effects,
            ..
        } => {
            then_effects.iter().any(effect_has_non_local_exit)
                || else_effects
                    .as_ref()
                    .is_some_and(|effects| effects.iter().any(effect_has_non_local_exit))
        }
        _ => false,
    }
}

fn collect_defined_values_from_effect(effect: &CoreEffectPlan, out: &mut BTreeSet<ValueId>) {
    match effect {
        CoreEffectPlan::MethodCall { dst, .. }
        | CoreEffectPlan::GlobalCall { dst, .. }
        | CoreEffectPlan::ValueCall { dst, .. }
        | CoreEffectPlan::ExternCall { dst, .. } => {
            if let Some(dst) = dst {
                out.insert(*dst);
            }
        }
        CoreEffectPlan::NewBox { dst, .. }
        | CoreEffectPlan::BinOp { dst, .. }
        | CoreEffectPlan::Compare { dst, .. }
        | CoreEffectPlan::Select { dst, .. }
        | CoreEffectPlan::Const { dst, .. }
        | CoreEffectPlan::Copy { dst, .. } => {
            out.insert(*dst);
        }
        CoreEffectPlan::IfEffect {
            then_effects,
            else_effects,
            ..
        } => {
            for nested in then_effects {
                collect_defined_values_from_effect(nested, out);
            }
            if let Some(else_effects) = else_effects {
                for nested in else_effects {
                    collect_defined_values_from_effect(nested, out);
                }
            }
        }
        CoreEffectPlan::ExitIf { .. } => {}
    }
}

fn debug_log_block_effects_binop_lit3(builder: &MirBuilder, effects: &[CoreEffectPlan]) {
    if !joinir_dev::strict_planner_required_debug_enabled() {
        return;
    }

    let mut int3_dsts: Vec<ValueId> = Vec::new();
    let mut add_binop: Option<(ValueId, ValueId, ValueId)> = None;
    for effect in effects {
        match effect {
            CoreEffectPlan::Const { dst, value } => {
                if matches!(value, ConstValue::Integer(3)) {
                    int3_dsts.push(*dst);
                }
            }
            CoreEffectPlan::BinOp { dst, lhs, op, rhs } => {
                if *op == BinaryOp::Add && add_binop.is_none() {
                    add_binop = Some((*dst, *lhs, *rhs));
                }
            }
            _ => {}
        }
    }

    if int3_dsts.is_empty() || add_binop.is_none() {
        return;
    }

    let fn_name = builder
        .scope_ctx
        .current_function
        .as_ref()
        .map(|f| f.signature.name.as_str())
        .unwrap_or("<none>");
    let const_int3_dsts = int3_dsts
        .iter()
        .map(|v| format!("%{}", v.0))
        .collect::<Vec<_>>()
        .join(",");
    let (dst, lhs, rhs) = add_binop.unwrap();
    let ring0 = crate::runtime::get_global_ring0();
    ring0.log.debug(&format!(
        "[loop/block_effects:binop_lit3] fn={} bb={:?} effects_len={} const_int3_dsts=[{}] add_binops=[dst=%{} lhs=%{} rhs=%{}]",
        fn_name,
        builder.current_block,
        effects.len(),
        const_int3_dsts,
        dst.0,
        lhs.0,
        rhs.0
    ));
}

fn collect_carriers_from_condition(
    cond_vars: BTreeSet<String>,
    builder: &MirBuilder,
    current_bindings: &BTreeMap<String, crate::mir::ValueId>,
) -> BTreeSet<String> {
    cond_vars
        .into_iter()
        .filter(|name| {
            current_bindings.contains_key(name)
                || builder.variable_ctx.variable_map.contains_key(name)
        })
        .collect()
}

fn condition_vars(condition: &crate::ast::ASTNode) -> BTreeSet<String> {
    let mut vars = BTreeSet::<String>::new();
    collect_condition_vars(condition, &mut vars);
    vars
}

fn collect_condition_vars(ast: &crate::ast::ASTNode, vars: &mut BTreeSet<String>) {
    use crate::ast::ASTNode;
    match ast {
        ASTNode::Variable { name, .. } => {
            vars.insert(name.clone());
        }
        ASTNode::UnaryOp { operand, .. } => collect_condition_vars(operand, vars),
        ASTNode::BinaryOp { left, right, .. } => {
            collect_condition_vars(left, vars);
            collect_condition_vars(right, vars);
        }
        ASTNode::GroupedAssignmentExpr { rhs, .. } => collect_condition_vars(rhs, vars),
        _ => {}
    }
}

fn collect_assigned_vars(ast: &crate::ast::ASTNode, vars: &mut BTreeSet<String>) {
    use crate::ast::ASTNode;
    match ast {
        ASTNode::Assignment { target, .. } => {
            if let ASTNode::Variable { name, .. } = target.as_ref() {
                vars.insert(name.clone());
            }
        }
        ASTNode::Program { statements, .. } => {
            for stmt in statements {
                collect_assigned_vars(stmt, vars);
            }
        }
        ASTNode::If {
            then_body,
            else_body,
            ..
        } => {
            for stmt in then_body {
                collect_assigned_vars(stmt, vars);
            }
            if let Some(else_body) = else_body {
                for stmt in else_body {
                    collect_assigned_vars(stmt, vars);
                }
            }
        }
        ASTNode::Loop { body, .. }
        | ASTNode::While { body, .. }
        | ASTNode::ForRange { body, .. } => {
            for stmt in body {
                collect_assigned_vars(stmt, vars);
            }
        }
        ASTNode::ScopeBox { body, .. } => {
            for stmt in body {
                collect_assigned_vars(stmt, vars);
            }
        }
        _ => {}
    }
}
