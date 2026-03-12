//! Main entry point for loop_cond_break_continue pattern pipeline.
//!
//! This module provides the `lower_loop_cond_break_continue` function which
//! was previously defined in the loop_cond_break_continue_pipeline/mod.rs.
//! The function delegates to the specialized helper modules.

use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;
use crate::mir::builder::control_flow::plan::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::plan::edgecfg_facade::Frag;
use crate::mir::builder::control_flow::plan::facts::exit_only_block::try_build_exit_allowed_block_recipe;
use crate::mir::builder::control_flow::plan::features::carriers;
use crate::mir::builder::control_flow::plan::features::edgecfg_stubs;
use crate::mir::builder::control_flow::plan::features::loop_carriers;
use crate::mir::builder::control_flow::plan::features::step_mode;
use crate::mir::builder::control_flow::plan::loop_cond::break_continue_types::LoopCondBreakAcceptKind;
use crate::mir::builder::control_flow::plan::loop_cond::break_continue_types::LoopCondBreakContinueFacts;
use crate::mir::builder::control_flow::plan::normalizer::{
    helpers::LoopBlocksStandard5, lower_loop_header_cond,
};
use crate::mir::builder::control_flow::plan::parts;
use crate::mir::builder::control_flow::plan::steps::empty_carriers_args;
use crate::mir::builder::control_flow::plan::{
    CoreEffectPlan, CoreLoopPlan, CorePlan, LoweredRecipe,
};
use crate::mir::builder::MirBuilder;
use crate::mir::policies::BodyLoweringPolicy;
use crate::mir::MirType;
use std::collections::{BTreeMap, BTreeSet};

pub(super) const LOOP_COND_ERR: &str = "[normalizer] loop_cond_break_continue";

pub(in crate::mir::builder) fn lower_loop_cond_break_continue(
    builder: &mut MirBuilder,
    facts: LoopCondBreakContinueFacts,
    _ctx: &LoopRouteContext,
) -> Result<LoweredRecipe, String> {
    // Facts->Lower contract: keep this match exhaustive.
    match facts.accept_kind {
        LoopCondBreakAcceptKind::ExitIf => (),
        LoopCondBreakAcceptKind::ContinueIf => (),
        LoopCondBreakAcceptKind::ConditionalUpdate => (),
        LoopCondBreakAcceptKind::ReturnInExitIf => (),
        LoopCondBreakAcceptKind::ReturnOnlyBody => (),
        LoopCondBreakAcceptKind::ElseOnlyReturn => (),
        LoopCondBreakAcceptKind::ElseOnlyBreak => (),
        LoopCondBreakAcceptKind::MixedIf => (),
        LoopCondBreakAcceptKind::NestedLoopOnly => (),
        LoopCondBreakAcceptKind::ProgramBlockNoExit => (),
    }

    let blocks = LoopBlocksStandard5::allocate(builder)?;
    let LoopBlocksStandard5 {
        preheader_bb,
        header_bb,
        body_bb,
        step_bb,
        after_bb,
    } = blocks;

    let carrier_sets = carriers::collect_from_body(&facts.recipe.body.body);
    let mut carrier_vars = carrier_sets.vars;
    if carrier_vars.is_empty() {
        carrier_vars = collect_carrier_vars_from_condition(builder, &facts.condition);
    }
    let use_header_continue_target = crate::config::env::joinir_dev::strict_enabled()
        && crate::config::env::joinir_dev::planner_required_enabled()
        && !facts.continue_branches.is_empty();
    if crate::config::env::joinir_trace_enabled() {
        let ring0 = crate::runtime::get_global_ring0();
        ring0.log.debug(&format!(
            "[joinir/loop_cond_break_continue] carriers={:?}",
            carrier_vars
        ));
        if facts.has_handled_guard_break {
            ring0.log.debug(&format!(
                "[joinir/loop_cond_break_continue] handled_guard={:?}",
                facts.handled_var_name
            ));
        }
        if !facts.continue_branches.is_empty() {
            let summaries: Vec<(usize, bool, bool)> = facts
                .continue_branches
                .iter()
                .map(|sig| (sig.stmt_count, sig.has_assignment, sig.has_local))
                .collect();
            ring0.log.debug(&format!(
                "[joinir/loop_cond_break_continue] continue_branches={:?}",
                summaries
            ));
        }
    }
    let mut carrier_inits = BTreeMap::new();
    let mut carrier_phis = BTreeMap::new();
    let mut carrier_step_phis = BTreeMap::new();
    let mut break_phi_dsts = BTreeMap::new();
    for var in &carrier_vars {
        let Some(&init_val) = builder.variable_ctx.variable_map.get(var) else {
            return Err(format!("{LOOP_COND_ERR}: carrier {} missing init", var));
        };
        let ty = builder
            .type_ctx
            .get_type(init_val)
            .cloned()
            .unwrap_or(MirType::Unknown);
        let phi_dst = builder.alloc_typed(ty.clone());
        let step_phi_dst = if use_header_continue_target {
            phi_dst
        } else {
            builder.alloc_typed(ty.clone())
        };
        let after_phi_dst = builder.alloc_typed(ty);
        carrier_inits.insert(var.clone(), init_val);
        carrier_phis.insert(var.clone(), phi_dst);
        carrier_step_phis.insert(var.clone(), step_phi_dst);
        break_phi_dsts.insert(var.clone(), after_phi_dst);
    }

    if carrier_phis.is_empty() {
        return Err(format!("{LOOP_COND_ERR}: no loop carriers"));
    }

    let mut current_bindings = builder.variable_ctx.variable_map.clone();
    for (name, value_id) in &carrier_phis {
        current_bindings.insert(name.clone(), *value_id);
        // NOTE: Do NOT insert into builder.variable_ctx.variable_map here.
        // PHI dst (value_id) is not yet defined at this point.
        // It will be defined by provisional PHI insertion in loop_lowering.rs Step 1.5.
    }
    // Phase 2b-3: Short-circuit evaluation for loop header condition
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
        LOOP_COND_ERR,
    )?;

    // The loop header condition can short-circuit through intermediate blocks (AND/OR).
    // Any block that branches directly to `after_bb` is a predecessor of `after_bb`, and
    // after_bb PHIs must include an input for every reachable predecessor.
    let after_cond_preds = header_result.preds_to(after_bb);

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

    let mut body_plans = match facts.body_lowering_policy {
        BodyLoweringPolicy::ExitAllowed { .. } => {
            let Some(body_exit_allowed) = facts.body_exit_allowed.as_ref() else {
                return Err(format!(
                    "[freeze:contract][loop_cond_break_continue] body_lowering_policy=ExitAllowed but body_exit_allowed=None: ctx={LOOP_COND_ERR}"
                ));
            };
            let verified = parts::entry::verify_exit_allowed_block_with_pre(
                &body_exit_allowed.arena,
                &body_exit_allowed.block,
                LOOP_COND_ERR,
                Some(&builder.variable_ctx.variable_map),
            )?;
            parts::entry::lower_exit_allowed_block_verified(
                builder,
                &mut current_bindings,
                &carrier_step_phis,
                &break_phi_dsts,
                verified,
                LOOP_COND_ERR,
            )
            .or_else(|err| {
                if !err.contains("if body must be single-exit") {
                    return Err(err);
                }
                lower_loop_cond_body_items(
                    builder,
                    &mut current_bindings,
                    &carrier_phis,
                    &carrier_step_phis,
                    &break_phi_dsts,
                    &facts.recipe.body,
                    &facts.recipe.items,
                    facts.propagate_nested_carriers,
                )
            })?
        }
        BodyLoweringPolicy::RecipeOnly => lower_loop_cond_body_items(
            builder,
            &mut current_bindings,
            &carrier_phis,
            &carrier_step_phis,
            &break_phi_dsts,
            &facts.recipe.body,
            &facts.recipe.items,
            facts.propagate_nested_carriers,
        )
        .or_else(|err| {
            if !err.contains("if body must be single-exit") {
                return Err(err);
            }

            let fallback = facts
                .body_exit_allowed
                .clone()
                .or_else(|| try_build_exit_allowed_block_recipe(&facts.recipe.body.body, true))
                .or_else(|| try_build_exit_allowed_block_recipe(&facts.recipe.body.body, false));
            let Some(body_exit_allowed) = fallback else {
                return Err(err);
            };

            let verified = parts::entry::verify_exit_allowed_block_with_pre(
                &body_exit_allowed.arena,
                &body_exit_allowed.block,
                LOOP_COND_ERR,
                Some(&builder.variable_ctx.variable_map),
            )?;
            parts::entry::lower_exit_allowed_block_verified(
                builder,
                &mut current_bindings,
                &carrier_step_phis,
                &break_phi_dsts,
                verified,
                LOOP_COND_ERR,
            )
        })?,
    };

    let body_entry_bindings = current_bindings.clone();
    let body_entry_values: BTreeSet<crate::mir::ValueId> =
        body_entry_bindings.values().copied().collect();
    let mut body_defined_values = BTreeSet::new();
    collect_defined_values_from_plans(&body_plans, &mut body_defined_values);

    // Fallthrough at end-of-body: explicit backedge with carrier values.
    // Use a pred-local snapshot so step-join inputs come from the current block.
    let mut fallthrough_bindings = current_bindings.clone();
    for (name, _) in &carrier_step_phis {
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
            fallthrough_bindings.insert(name.clone(), value_id);
        }
    }
    body_plans.push(CorePlan::Exit(parts::exit::build_continue_with_phi_args(
        builder,
        &carrier_step_phis,
        &fallthrough_bindings,
        LOOP_COND_ERR,
    )?));

    let mut phis = Vec::new();
    let mut final_values = Vec::new();
    for (var, header_phi_dst) in &carrier_phis {
        let init_val = match carrier_inits.get(var) {
            Some(value) => *value,
            None => continue,
        };
        let Some(after_phi_dst) = break_phi_dsts.get(var).copied() else {
            continue;
        };
        if use_header_continue_target {
            // Header PHI: init + per-edge continue inputs (filled later by ContinueWithPhiArgs).
            phis.push(loop_carriers::build_preheader_only_phi_info(
                header_bb,
                preheader_bb,
                *header_phi_dst,
                init_val,
                format!("loop_cond_carrier_{}", var),
            ));
        } else {
            let Some(step_phi_dst) = carrier_step_phis.get(var).copied() else {
                continue;
            };

            // Step join PHI: inputs are populated during lowering from per-edge ContinueWithPhiArgs.
            phis.push(loop_carriers::build_step_join_phi_info(
                step_bb,
                step_phi_dst,
                format!("loop_cond_step_join_{}", var),
            ));

            // Header PHI: chooses init vs the step-join value.
            phis.push(loop_carriers::build_loop_phi_info(
                header_bb,
                preheader_bb,
                step_bb,
                *header_phi_dst,
                init_val,
                step_phi_dst,
                format!("loop_cond_carrier_{}", var),
            ));
        }
        phis.push(loop_carriers::build_after_merge_phi_info(
            after_bb,
            after_phi_dst,
            after_cond_preds.iter().copied(),
            *header_phi_dst,
            format!("loop_cond_after_{}", var),
        ));
        final_values.push((var.clone(), after_phi_dst));
    }
    if crate::config::env::joinir_trace_enabled() {
        let after_phi_count = phis.iter().filter(|phi| phi.block == after_bb).count();
        let ring0 = crate::runtime::get_global_ring0();
        ring0.log.debug(&format!(
            "[joinir/loop_cond_break_continue] blocks preheader={:?} header={:?} body={:?} step={:?} after={:?} phis_total={} phis_after_bb={} final_values={}",
            preheader_bb,
            header_bb,
            body_bb,
            step_bb,
            after_bb,
            phis.len(),
            after_phi_count,
            final_values.len()
        ));
    }

    // Build block_effects: merge header_result.block_effects + static entries
    let mut block_effects: Vec<(crate::mir::BasicBlockId, Vec<CoreEffectPlan>)> =
        vec![(preheader_bb, vec![])];
    for (bb, effects) in header_result.block_effects {
        block_effects.push((bb, effects));
    }
    block_effects.push((body_bb, vec![]));
    block_effects.push((step_bb, vec![]));
    block_effects.push((after_bb, vec![]));

    let continue_target = if use_header_continue_target {
        header_bb
    } else {
        step_bb
    };

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

/// Fill missing carrier bindings from variable_map.
///
/// Do not overwrite existing carrier bindings in `current_bindings`.
/// Overwriting can clobber header-phi tracking with stale pre-loop values when
/// nested loop lowering does not intentionally update an outer carrier.
pub(super) fn sync_carrier_bindings(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
) {
    for (name, _) in carrier_phis {
        if current_bindings.contains_key(name) {
            continue;
        }
        if let Some(value_id) = builder.variable_ctx.variable_map.get(name) {
            current_bindings.insert(name.clone(), *value_id);
        }
    }
}

fn lower_loop_cond_body_items(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: &BTreeMap<String, crate::mir::ValueId>,
    body: &crate::mir::builder::control_flow::plan::recipes::RecipeBody,
    items: &[crate::mir::builder::control_flow::plan::loop_cond::break_continue_recipe::LoopCondBreakContinueItem],
    propagate_nested: bool,
) -> Result<Vec<LoweredRecipe>, String> {
    let mut carrier_updates = BTreeMap::new();
    let mut body_plans = Vec::new();
    for (idx, item) in items.iter().enumerate() {
        let mut plans = super::loop_cond_bc_item::lower_loop_cond_item(
            builder,
            current_bindings,
            carrier_phis,
            carrier_step_phis,
            break_phi_dsts,
            &mut carrier_updates,
            body,
            item,
            propagate_nested,
        )
        .map_err(|err| format!("{err} [loop_cond_item idx={idx} kind={item:?}]"))?;
        body_plans.append(&mut plans);
    }
    Ok(body_plans)
}

fn collect_defined_values_from_plans(
    plans: &[LoweredRecipe],
    out: &mut BTreeSet<crate::mir::ValueId>,
) {
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

fn collect_defined_values_from_effect(
    effect: &CoreEffectPlan,
    out: &mut BTreeSet<crate::mir::ValueId>,
) {
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

fn collect_carrier_vars_from_condition(
    builder: &MirBuilder,
    condition: &crate::ast::ASTNode,
) -> Vec<String> {
    let mut vars = BTreeSet::<String>::new();
    collect_vars_from_expr(condition, &mut vars);

    let mut carriers = BTreeMap::<String, ()>::new();
    for name in vars {
        if builder.variable_ctx.variable_map.contains_key(&name) {
            carriers.insert(name, ());
        }
    }
    carriers.keys().cloned().collect()
}

fn collect_vars_from_expr(ast: &crate::ast::ASTNode, vars: &mut BTreeSet<String>) {
    use crate::ast::ASTNode;
    match ast {
        ASTNode::Variable { name, .. } => {
            vars.insert(name.clone());
        }
        ASTNode::Literal { .. } => {}
        ASTNode::UnaryOp { operand, .. } => collect_vars_from_expr(operand, vars),
        ASTNode::BinaryOp { left, right, .. } => {
            collect_vars_from_expr(left, vars);
            collect_vars_from_expr(right, vars);
        }
        ASTNode::GroupedAssignmentExpr { rhs, .. } => collect_vars_from_expr(rhs, vars),
        ASTNode::MethodCall {
            object, arguments, ..
        } => {
            collect_vars_from_expr(object, vars);
            for arg in arguments {
                collect_vars_from_expr(arg, vars);
            }
        }
        ASTNode::FunctionCall { arguments, .. } => {
            for arg in arguments {
                collect_vars_from_expr(arg, vars);
            }
        }
        ASTNode::Call {
            callee, arguments, ..
        } => {
            collect_vars_from_expr(callee, vars);
            for arg in arguments {
                collect_vars_from_expr(arg, vars);
            }
        }
        ASTNode::FieldAccess { object, .. } => collect_vars_from_expr(object, vars),
        ASTNode::Index { target, index, .. } => {
            collect_vars_from_expr(target, vars);
            collect_vars_from_expr(index, vars);
        }
        ASTNode::New { arguments, .. } => {
            for arg in arguments {
                collect_vars_from_expr(arg, vars);
            }
        }
        ASTNode::AwaitExpression { expression, .. } => collect_vars_from_expr(expression, vars),
        ASTNode::QMarkPropagate { expression, .. } => collect_vars_from_expr(expression, vars),
        ASTNode::MatchExpr {
            scrutinee,
            arms,
            else_expr,
            ..
        } => {
            collect_vars_from_expr(scrutinee, vars);
            for (_lit, expr) in arms {
                collect_vars_from_expr(expr, vars);
            }
            collect_vars_from_expr(else_expr, vars);
        }
        ASTNode::ArrayLiteral { elements, .. } => {
            for elem in elements {
                collect_vars_from_expr(elem, vars);
            }
        }
        ASTNode::MapLiteral { entries, .. } => {
            for (_k, v) in entries {
                collect_vars_from_expr(v, vars);
            }
        }
        ASTNode::Lambda { body, .. } => {
            for stmt in body {
                collect_vars_from_expr(stmt, vars);
            }
        }
        ASTNode::BlockExpr {
            prelude_stmts,
            tail_expr,
            ..
        } => {
            for stmt in prelude_stmts {
                collect_vars_from_expr(stmt, vars);
            }
            collect_vars_from_expr(tail_expr, vars);
        }
        ASTNode::Arrow {
            sender, receiver, ..
        } => {
            collect_vars_from_expr(sender, vars);
            collect_vars_from_expr(receiver, vars);
        }
        _ => {}
    }
}
