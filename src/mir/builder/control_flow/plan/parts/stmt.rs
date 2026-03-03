//! Stmt lowering helpers (Parts).
//!
//! Scope: behavior-preserving extraction of existing lowering logic.
//! SSOT for return prelude statement lowering.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::plan::facts::exit_only_block::try_build_exit_allowed_block_recipe;
use crate::mir::builder::control_flow::plan::facts::no_exit_block::try_build_no_exit_block_recipe;
use crate::mir::builder::control_flow::plan::facts::return_prelude::{
    try_build_return_prelude_container_recipe, ReturnPreludeContainerRecipe,
};
use crate::mir::builder::control_flow::plan::recipe_tree::{ExitKind, RecipeBlock, RecipeItem};
use super::if_exit::{lower_if_exit_stmt_view, lower_if_exit_stmt_with_break_phi_args_view};
use super::super::steps::effects_to_plans;
use crate::mir::builder::control_flow::plan::normalizer::{loop_body_lowering, PlanNormalizer};
use crate::mir::builder::control_flow::plan::{
    CoreEffectPlan, CorePlan, LoweredRecipe,
};
use crate::mir::builder::MirBuilder;
use crate::mir::{BinaryOp, ConstValue, Effect, EffectMask, ValueId};
use std::collections::BTreeMap;

pub(in crate::mir::builder) fn lower_return_prelude_stmt(
    builder: &mut MirBuilder,
    branch_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: Option<&BTreeMap<String, crate::mir::ValueId>>,
    stmt: &ASTNode,
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    fn block_contains_break_or_continue(block: &RecipeBlock) -> bool {
        for item in &block.items {
            match item {
                RecipeItem::Exit { kind, .. } => {
                    if matches!(kind, ExitKind::Break { .. } | ExitKind::Continue { .. }) {
                        return true;
                    }
                }
                RecipeItem::IfV2 {
                    then_block,
                    else_block,
                    ..
                } => {
                    if block_contains_break_or_continue(then_block)
                        || else_block
                            .as_ref()
                            .is_some_and(|b| block_contains_break_or_continue(b))
                    {
                        return true;
                    }
                }
                // `break/continue` inside nested loops are handled by the loop skeleton itself,
                // not by the surrounding (caller-provided) `break_phi_dsts` map.
                //
                // Therefore, do not treat nested `LoopV0` bodies as requiring `break_phi_dsts`
                // at the current lowering site.
                RecipeItem::LoopV0 { .. } => {}
                RecipeItem::Stmt(_) => {}
            }
        }
        false
    }

    if let Some(recipe) = try_build_return_prelude_container_recipe(stmt, true) {
        match recipe {
            ReturnPreludeContainerRecipe::NoExit(recipe) => {
                return super::entry::lower_no_exit_block(
                    builder,
                    branch_bindings,
                    carrier_step_phis,
                    break_phi_dsts,
                    &recipe.arena,
                    &recipe.block,
                    error_prefix,
                );
            }
            ReturnPreludeContainerRecipe::ExitAllowed(recipe) => {
                let empty_break_phi_dsts = BTreeMap::new();
                let break_phi_dsts = match break_phi_dsts {
                    Some(break_phi_dsts) => break_phi_dsts,
                    None => {
                        if block_contains_break_or_continue(&recipe.block) {
                            return Err(format!(
                                "[freeze:contract][recipe] return_prelude_exit_allowed_requires_loop ctx={}",
                                error_prefix
                            ));
                        }
                        &empty_break_phi_dsts
                    }
                };
                return super::entry::lower_exit_allowed_block(
                    builder,
                    branch_bindings,
                    carrier_step_phis,
                    break_phi_dsts,
                    &recipe.arena,
                    &recipe.block,
                    error_prefix,
                );
            }
        }
    }

    match stmt {
        ASTNode::Break { .. } => {
            let Some(break_phi_dsts) = break_phi_dsts else {
                return Err(format!(
                    "[freeze:contract][recipe] return_prelude_break_requires_loop ctx={}",
                    error_prefix
                ));
            };
            Ok(vec![CorePlan::Exit(
                super::exit::build_break_with_phi_args(break_phi_dsts, branch_bindings, error_prefix)?,
            )])
        }
        ASTNode::Continue { .. } => Ok(vec![CorePlan::Exit(
            super::exit::build_continue_with_phi_args(
                builder,
                carrier_step_phis,
                branch_bindings,
                error_prefix,
            )?,
        )]),
        ASTNode::Return { value, .. } => super::exit::lower_return_stmt_with_effects(
            builder,
            value.as_ref().map(|v| v.as_ref()),
            branch_bindings,
            error_prefix,
        ),
        ASTNode::Assignment { target, value, .. } => {
            if let ASTNode::Variable { name, .. } = target.as_ref() {
                if value_has_blockexpr_prelude_loop(value) {
                    let (value_id, plans) = lower_value_with_blockexpr_loop_prelude_stmt(
                        builder,
                        branch_bindings,
                        carrier_step_phis,
                        break_phi_dsts,
                        value,
                        error_prefix,
                    )?;
                    branch_bindings.insert(name.clone(), value_id);
                    builder
                        .variable_ctx
                        .variable_map
                        .insert(name.clone(), value_id);
                    return Ok(plans);
                }
            }

            let (binding, effects) = loop_body_lowering::lower_assignment_stmt(
                builder,
                branch_bindings,
                target,
                value,
                error_prefix,
            )?;
            debug_log_stmt_binop_lit3(builder, &effects, "assignment");
            if let Some((name, value_id)) = binding {
                branch_bindings.insert(name.clone(), value_id);
                builder.variable_ctx.variable_map.insert(name, value_id);
            }
            Ok(effects_to_plans(effects))
        }
        ASTNode::Local {
            variables,
            initial_values,
            ..
        } => {
            if variables.len() != initial_values.len() {
                return Err(format!("{error_prefix}: local init arity mismatch"));
            }
            if initial_values
                .iter()
                .flatten()
                .any(|value| value_has_blockexpr_prelude_loop(value))
            {
                let mut plans = Vec::new();
                for (name, init) in variables.iter().zip(initial_values.iter()) {
                    let init_node = loop_body_lowering::local_init_node_or_null(init.as_ref());
                    let (value_id, mut init_plans) = lower_value_with_blockexpr_loop_prelude_stmt(
                        builder,
                        branch_bindings,
                        carrier_step_phis,
                        break_phi_dsts,
                        init_node.as_ref(),
                        error_prefix,
                    )?;
                    plans.append(&mut init_plans);
                    branch_bindings.insert(name.clone(), value_id);
                    builder.variable_ctx.variable_map.insert(name.clone(), value_id);
                }
                return Ok(plans);
            }

            let (inits, effects) = loop_body_lowering::lower_local_init_values(
                builder,
                branch_bindings,
                variables,
                initial_values,
                error_prefix,
            )?;
            debug_log_stmt_binop_lit3(builder, &effects, "local");
            for (name, value_id) in inits {
                branch_bindings.insert(name.clone(), value_id);
                builder.variable_ctx.variable_map.insert(name, value_id);
            }
            Ok(effects_to_plans(effects))
        }
        ASTNode::MethodCall { .. } => {
            let effects = loop_body_lowering::lower_method_call_stmt(
                builder,
                branch_bindings,
                stmt,
                error_prefix,
            )?;
            Ok(effects_to_plans(effects))
        }
        ASTNode::FunctionCall { .. } => {
            let effects = loop_body_lowering::lower_function_call_stmt(
                builder,
                branch_bindings,
                stmt,
                error_prefix,
            )?;
            Ok(effects_to_plans(effects))
        }
        ASTNode::Print { expression, .. } => {
            let (value_id, mut effects) =
                PlanNormalizer::lower_value_ast(expression, builder, branch_bindings)?;
            effects.push(CoreEffectPlan::ExternCall {
                dst: None,
                iface_name: "env.console".to_string(),
                method_name: "log".to_string(),
                args: vec![value_id],
                effects: EffectMask::PURE.add(Effect::Io),
            });
            debug_log_stmt_binop_lit3(builder, &effects, "print");
            Ok(effects_to_plans(effects))
        }
        ASTNode::Call { .. } => {
            let (_value_id, effects) =
                PlanNormalizer::lower_value_ast(stmt, builder, branch_bindings)?;
            debug_log_stmt_binop_lit3(builder, &effects, "call");
            Ok(effects_to_plans(effects))
        }
        ASTNode::If {
            condition,
            then_body,
            else_body,
            ..
        } => {
            let cond_view = CondBlockView::from_expr(condition);

            // Keep no-exit `if` prelude lowering available in release as well.
            // Without this, mixed continue/break branches that contain nested no-exit `if`
            // in prelude can fail in exit-branch lowering and fall through to distant
            // "nested loop has no plan" errors.
            if let Some(recipe) = try_build_no_exit_block_recipe(std::slice::from_ref(stmt), true) {
                let plans = super::entry::lower_no_exit_block(
                    builder,
                    branch_bindings,
                    carrier_step_phis,
                    break_phi_dsts,
                    &recipe.arena,
                    &recipe.block,
                    error_prefix,
                )?;
                return Ok(plans);
            }

            let strict_or_dev = crate::config::env::joinir_dev::strict_enabled()
                || crate::config::env::joinir_dev_enabled();
            let planner_required =
                strict_or_dev && crate::config::env::joinir_dev::planner_required_enabled();

            if planner_required {
                let allow_extended = planner_required;
                if let Some(recipe) =
                    try_build_no_exit_block_recipe(std::slice::from_ref(stmt), allow_extended)
                {
                    let plans = super::entry::lower_no_exit_block(
                        builder,
                        branch_bindings,
                        carrier_step_phis,
                        break_phi_dsts,
                        &recipe.arena,
                        &recipe.block,
                        error_prefix,
                    )?;
                    return Ok(plans);
                }

                if let Some(recipe) =
                    try_build_exit_allowed_block_recipe(std::slice::from_ref(stmt), allow_extended)
                {
                    // Avoid infinite recursion: a join-bearing `if` is represented as `Stmt`
                    // in exit-allowed recipes and must be lowered via the non-exit path instead.
                    if !matches!(recipe.block.items.as_slice(), [RecipeItem::Stmt(_)]) {
                        let empty_break_phi_dsts = BTreeMap::new();
                        let break_phi_dsts = match break_phi_dsts {
                            Some(break_phi_dsts) => break_phi_dsts,
                            None => {
                                if block_contains_break_or_continue(&recipe.block) {
                                    return Err(format!(
                                        "[freeze:contract][recipe] return_prelude_exit_allowed_requires_loop ctx={}",
                                        error_prefix
                                    ));
                                }
                                &empty_break_phi_dsts
                            }
                        };

                        let plans = super::entry::lower_exit_allowed_block(
                            builder,
                            branch_bindings,
                            carrier_step_phis,
                            break_phi_dsts,
                            &recipe.arena,
                            &recipe.block,
                            error_prefix,
                        )?;
                        return Ok(plans);
                    }
                }

                fn tail_is_exit(body: &[ASTNode]) -> bool {
                    matches!(
                        body.last(),
                        Some(ASTNode::Return { .. } | ASTNode::Break { .. } | ASTNode::Continue { .. })
                    )
                }

                // If it is not an exit-if shape, do not fall back to exit-if lowering.
                // That fallback produces long-distance errors like "if body must be single-exit"
                // for general join-bearing `if` forms (e.g., nested-if + assignments).
                let is_exit_if_shape =
                    tail_is_exit(then_body) && else_body.as_ref().map_or(true, |b| tail_is_exit(b));
                if !is_exit_if_shape {
                    let should_update_binding =
                        |name: &str, bindings: &BTreeMap<String, crate::mir::ValueId>| {
                            bindings.contains_key(name)
                        };

                    let lower_stmt_list = |builder: &mut MirBuilder,
                                               bindings: &mut BTreeMap<
                        String,
                        crate::mir::ValueId,
                    >,
                                               stmts: &[ASTNode]| {
                        let mut plans = Vec::new();
                        for stmt in stmts {
                            plans.extend(lower_return_prelude_stmt(
                                builder,
                                bindings,
                                carrier_step_phis,
                                break_phi_dsts,
                                stmt,
                                error_prefix,
                            )?);
                        }
                        Ok::<_, String>(plans)
                    };

                    let mut lower_then = |builder: &mut MirBuilder,
                                          bindings: &mut BTreeMap<String, crate::mir::ValueId>| {
                        lower_stmt_list(builder, bindings, then_body)
                    };

                    if let Some(else_body) = else_body.as_ref() {
                        let mut lower_else =
                            |builder: &mut MirBuilder,
                             bindings: &mut BTreeMap<String, crate::mir::ValueId>| {
                                lower_stmt_list(builder, bindings, else_body)
                            };
                        return super::entry::lower_if_join_with_branch_lowerers(
                            builder,
                            branch_bindings,
                            &cond_view,
                            error_prefix,
                            &mut lower_then,
                            Some(&mut lower_else),
                            &should_update_binding,
                        );
                    }

                    return super::entry::lower_if_join_with_branch_lowerers(
                        builder,
                        branch_bindings,
                        &cond_view,
                        error_prefix,
                        &mut lower_then,
                        None,
                        &should_update_binding,
                    );
                }
            }
            match break_phi_dsts {
                Some(break_phi_dsts) => lower_if_exit_stmt_with_break_phi_args_view(
                    builder,
                    branch_bindings,
                    carrier_step_phis,
                    break_phi_dsts,
                    &cond_view,
                    then_body,
                    else_body.as_ref(),
                    error_prefix,
                ),
                None => lower_if_exit_stmt_view(
                    builder,
                    branch_bindings,
                    carrier_step_phis,
                    &cond_view,
                    then_body,
                    else_body.as_ref(),
                    error_prefix,
                ),
            }
        }
        _ => Err(format!(
            "[freeze:contract][recipe] return_prelude_unsupported_stmt kind={} ctx={}",
            stmt.node_type(),
            error_prefix
        )),
    }
}

fn lower_value_with_blockexpr_loop_prelude_stmt(
    builder: &mut MirBuilder,
    branch_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: Option<&BTreeMap<String, crate::mir::ValueId>>,
    value: &ASTNode,
    error_prefix: &str,
) -> Result<(ValueId, Vec<LoweredRecipe>), String> {
    let ASTNode::BlockExpr {
        prelude_stmts,
        tail_expr,
        ..
    } = value
    else {
        let (value_id, effects) = PlanNormalizer::lower_value_ast(value, builder, branch_bindings)?;
        return Ok((value_id, effects_to_plans(effects)));
    };

    if !prelude_stmts
        .iter()
        .any(stmt_has_loop_stmt_recursive)
    {
        let (value_id, effects) = PlanNormalizer::lower_value_ast(value, builder, branch_bindings)?;
        return Ok((value_id, effects_to_plans(effects)));
    }

    for stmt in prelude_stmts {
        if stmt.contains_non_local_exit_outside_loops() {
            return Err(format!(
                "[freeze:contract][blockexpr] {error_prefix}: exit stmt is forbidden in BlockExpr prelude"
            ));
        }
    }

    let mut block_bindings = branch_bindings.clone();
    let mut plans = Vec::new();
    for stmt in prelude_stmts {
        let mut stmt_plans = lower_return_prelude_stmt(
            builder,
            &mut block_bindings,
            carrier_step_phis,
            break_phi_dsts,
            stmt,
            error_prefix,
        )?;
        plans.append(&mut stmt_plans);
    }

    let (tail_id, tail_effects) =
        PlanNormalizer::lower_value_ast(tail_expr.as_ref(), builder, &block_bindings)?;
    plans.extend(effects_to_plans(tail_effects));
    Ok((tail_id, plans))
}

fn value_has_blockexpr_prelude_loop(value: &ASTNode) -> bool {
    let ASTNode::BlockExpr { prelude_stmts, .. } = value else {
        return false;
    };
    prelude_stmts.iter().any(stmt_has_loop_stmt_recursive)
}

fn stmt_has_loop_stmt_recursive(stmt: &ASTNode) -> bool {
    match stmt {
        ASTNode::Loop { .. } | ASTNode::While { .. } => true,
        ASTNode::If {
            then_body,
            else_body,
            ..
        } => {
            then_body.iter().any(stmt_has_loop_stmt_recursive)
                || else_body
                    .as_ref()
                    .is_some_and(|body| body.iter().any(stmt_has_loop_stmt_recursive))
        }
        ASTNode::Program { statements, .. } => statements.iter().any(stmt_has_loop_stmt_recursive),
        ASTNode::ScopeBox { body, .. } => body.iter().any(stmt_has_loop_stmt_recursive),
        _ => false,
    }
}

fn debug_log_stmt_binop_lit3(builder: &MirBuilder, effects: &[CoreEffectPlan], kind: &'static str) {
    if !crate::config::env::joinir_dev::strict_planner_required_debug_enabled() {
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
        "[stmt/effects:binop_lit3] fn={} kind={} bb={:?} effects_len={} const_int3_dsts=[{}] add_binops=[dst=%{} lhs=%{} rhs=%{}]",
        fn_name,
        kind,
        builder.current_block,
        effects.len(),
        const_int3_dsts,
        dst.0,
        lhs.0,
        rhs.0
    ));
}
