//! Main entry point for loop_cond_break_continue pattern pipeline.
//!
//! This module provides the `lower_loop_cond_break_continue` function which
//! was previously defined in the loop_cond_break_continue_pipeline/mod.rs.
//! The function delegates to the specialized helper modules.

use crate::mir::builder::control_flow::facts::loop_cond_break_continue::{
    LoopCondBreakAcceptKind, LoopCondBreakContinueFacts,
};
use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;
use crate::mir::builder::control_flow::plan::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::plan::edgecfg_facade::Frag;
use crate::mir::builder::control_flow::plan::facts::exit_only_block::try_build_exit_allowed_block_recipe;
use crate::mir::builder::control_flow::plan::features::carriers;
use crate::mir::builder::control_flow::plan::features::edgecfg_stubs;
use crate::mir::builder::control_flow::plan::features::loop_cond_bc_cleanup::apply_loop_cond_break_continue_cleanup;
use crate::mir::builder::control_flow::plan::features::loop_cond_bc_phi_materializer::LoopCondBreakContinuePhiMaterializer;
use crate::mir::builder::control_flow::plan::features::loop_cond_bc_verifier::verify_loop_cond_break_continue_phi_closure;
use crate::mir::builder::control_flow::plan::features::step_mode;
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
    let phi_materializer = LoopCondBreakContinuePhiMaterializer::prepare(
        builder,
        &carrier_vars,
        use_header_continue_target,
        header_bb,
        step_bb,
        LOOP_COND_ERR,
    )?;
    let carrier_phis = phi_materializer.carrier_phis().clone();
    let carrier_step_phis = phi_materializer.carrier_step_phis().clone();
    let break_phi_dsts = phi_materializer.break_phi_dsts().clone();

    let mut current_bindings = builder.variable_ctx.variable_map.clone();
    for (name, value_id) in phi_materializer.phi_bindings() {
        current_bindings.insert(name.clone(), value_id);
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
    let mut after_cond_preds = header_result.preds_to(after_bb);
    if after_cond_preds.is_empty() {
        // Simple header conditions can still exit directly from the header block
        // even when the branch stub set does not carry an explicit after-edge.
        // Seed the header false-edge conservatively; Step 4 filters any non-CFG
        // predecessor before patching the PHI.
        after_cond_preds.insert(header_bb);
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
        carrier_phis.len(),
        LOOP_COND_ERR,
    )?;
    if crate::config::env::joinir_trace_enabled() {
        let after_phi_count = phi_closure
            .phis()
            .iter()
            .filter(|phi| phi.block == after_bb)
            .count();
        let ring0 = crate::runtime::get_global_ring0();
        ring0.log.debug(&format!(
            "[joinir/loop_cond_break_continue] blocks preheader={:?} header={:?} body={:?} step={:?} after={:?} phis_total={} phis_after_bb={} final_values={}",
            preheader_bb,
            header_bb,
            body_bb,
            step_bb,
            after_bb,
            phi_closure.phis().len(),
            after_phi_count,
            phi_closure.final_values().len()
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
        final_values: phi_closure.final_values().to_vec(),
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
