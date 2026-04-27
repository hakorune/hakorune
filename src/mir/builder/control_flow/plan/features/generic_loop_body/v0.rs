use crate::ast::ASTNode;
use crate::mir::builder::control_flow::facts::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;
use crate::mir::builder::control_flow::plan::normalizer::PlanNormalizer;
use crate::mir::builder::control_flow::plan::normalizer::{loop_body_lowering, lower_cond_value};
use crate::mir::builder::control_flow::plan::{CoreEffectPlan, CorePlan, LoweredRecipe};
use crate::mir::builder::MirBuilder;
use crate::mir::{ConstValue, Effect, EffectMask, MirType};

use super::super::exit_branch;
use super::helpers::{
    append_effects, lower_effect_block, lower_effect_only_stmt, lower_exit_from_stmt,
    lower_if_effect, lower_negated_bool_cond, lower_nested_loop_plan, matches_loop_increment,
};
use super::GENERIC_LOOP_ERR;
use crate::mir::builder::control_flow::plan::generic_loop::facts_types::GenericLoopV0Facts;

pub(in crate::mir::builder) fn lower_generic_loop_v0_body(
    builder: &mut MirBuilder,
    facts: &GenericLoopV0Facts,
    phi_bindings: &std::collections::BTreeMap<String, crate::mir::ValueId>,
    ctx: &LoopRouteContext,
) -> Result<Vec<LoweredRecipe>, String> {
    let mut body_plans: Vec<LoweredRecipe> = Vec::new();

    for stmt in &facts.body.body {
        if matches_loop_increment(stmt, &facts.loop_var, &facts.loop_increment) {
            continue;
        }

        match stmt {
            ASTNode::Assignment { target, value, .. } => {
                let (binding, effects) = loop_body_lowering::lower_assignment_stmt(
                    builder,
                    phi_bindings,
                    target,
                    value,
                    GENERIC_LOOP_ERR,
                )?;
                if let Some((name, value_id)) = binding {
                    builder.variable_ctx.variable_map.insert(name, value_id);
                }
                append_effects(&mut body_plans, effects);
            }
            ASTNode::Local {
                variables,
                initial_values,
                ..
            } => {
                let (inits, effects) = loop_body_lowering::lower_local_init_values(
                    builder,
                    phi_bindings,
                    variables,
                    initial_values,
                    GENERIC_LOOP_ERR,
                )?;
                for (name, value_id) in inits {
                    builder.variable_ctx.variable_map.insert(name, value_id);
                }
                append_effects(&mut body_plans, effects);
            }
            ASTNode::MethodCall { .. } => {
                append_effects(
                    &mut body_plans,
                    loop_body_lowering::lower_method_call_stmt(
                        builder,
                        phi_bindings,
                        stmt,
                        GENERIC_LOOP_ERR,
                    )?,
                );
            }
            ASTNode::FunctionCall { .. } => {
                append_effects(
                    &mut body_plans,
                    loop_body_lowering::lower_function_call_stmt(
                        builder,
                        phi_bindings,
                        stmt,
                        GENERIC_LOOP_ERR,
                    )?,
                );
            }
            ASTNode::Print { expression, .. } => {
                let (value_id, mut effects) =
                    PlanNormalizer::lower_value_ast(expression, builder, phi_bindings)?;
                effects.push(CoreEffectPlan::ExternCall {
                    dst: None,
                    iface_name: "env.console".to_string(),
                    method_name: "log".to_string(),
                    args: vec![value_id],
                    effects: EffectMask::PURE.add(Effect::Io),
                });
                append_effects(&mut body_plans, effects);
            }
            ASTNode::Loop {
                condition, body, ..
            } => {
                let nested = lower_nested_loop_plan(builder, condition, body, ctx)?;
                body_plans.push(nested);
            }
            ASTNode::If {
                condition,
                then_body,
                else_body,
                ..
            } => {
                if then_body.is_empty() {
                    return Err("[normalizer] generic loop v0: unsupported if form".to_string());
                }
                if let Some(else_body) = else_body {
                    if split_break_else_body(
                        then_body,
                        else_body,
                        &facts.loop_var,
                        &facts.loop_increment,
                    ) {
                        let cond_view = CondBlockView::from_expr(condition);
                        let (cond_id, mut cond_effects) =
                            lower_cond_value(builder, phi_bindings, &cond_view, GENERIC_LOOP_ERR)?;
                        let (cond_neg, mut neg_effects) = lower_negated_bool_cond(builder, cond_id);
                        cond_effects.append(&mut neg_effects);
                        append_effects(&mut body_plans, cond_effects);
                        body_plans.push(CorePlan::Effect(CoreEffectPlan::ExitIf {
                            cond: cond_neg,
                            exit: exit_branch::build_break_exit_plan(1),
                        }));
                        continue;
                    }
                    if let Some(effect_stmts) = split_break_else_effect_body(then_body, else_body) {
                        let cond_view = CondBlockView::from_expr(condition);
                        let (cond_id, cond_effects) =
                            lower_cond_value(builder, phi_bindings, &cond_view, GENERIC_LOOP_ERR)?;
                        append_effects(&mut body_plans, cond_effects);
                        body_plans.push(CorePlan::Effect(CoreEffectPlan::ExitIf {
                            cond: cond_id,
                            exit: exit_branch::build_break_exit_plan(1),
                        }));
                        for stmt in effect_stmts {
                            append_effects(
                                &mut body_plans,
                                lower_effect_only_stmt(builder, phi_bindings, stmt)?,
                            );
                        }
                        continue;
                    }

                    let if_effects = lower_if_effect(
                        builder,
                        phi_bindings,
                        condition,
                        then_body,
                        else_body,
                        &facts.loop_var,
                        &facts.loop_increment,
                    )?;
                    append_effects(&mut body_plans, if_effects);
                    continue;
                }
                let cond_view = CondBlockView::from_expr(condition);
                let (cond_id, cond_effects) =
                    lower_cond_value(builder, phi_bindings, &cond_view, GENERIC_LOOP_ERR)?;
                append_effects(&mut body_plans, cond_effects);

                if let Some(effect_stmts) = split_continue_then_body(then_body) {
                    let mut then_effects = Vec::new();
                    for stmt in effect_stmts {
                        then_effects.extend(lower_effect_only_stmt(builder, phi_bindings, stmt)?);
                    }
                    then_effects.push(CoreEffectPlan::ExitIf {
                        cond: cond_id,
                        exit: exit_branch::build_continue_exit_plan(1),
                    });
                    body_plans.push(CorePlan::Effect(CoreEffectPlan::IfEffect {
                        cond: cond_id,
                        then_effects,
                        else_effects: None,
                    }));
                } else if then_body.len() == 1
                    && matches!(
                        then_body[0],
                        ASTNode::Break { .. } | ASTNode::Continue { .. } | ASTNode::Return { .. }
                    )
                {
                    let exit_effects =
                        lower_exit_from_stmt(builder, phi_bindings, cond_id, &then_body[0])?;
                    append_effects(&mut body_plans, exit_effects);
                } else {
                    let then_effects = lower_effect_block(
                        builder,
                        phi_bindings,
                        then_body,
                        &facts.loop_var,
                        &facts.loop_increment,
                    )?;
                    body_plans.push(CorePlan::Effect(CoreEffectPlan::IfEffect {
                        cond: cond_id,
                        then_effects,
                        else_effects: None,
                    }));
                }
            }
            ASTNode::Break { .. } | ASTNode::Continue { .. } | ASTNode::Return { .. } => {
                let true_cond = builder.alloc_typed(MirType::Bool);
                body_plans.push(CorePlan::Effect(CoreEffectPlan::Const {
                    dst: true_cond,
                    value: ConstValue::Bool(true),
                }));
                let exit_effects = lower_exit_from_stmt(builder, phi_bindings, true_cond, stmt)?;
                append_effects(&mut body_plans, exit_effects);
            }
            _ => {
                return Err("[normalizer] generic loop v0: unsupported body stmt".to_string());
            }
        }
    }

    Ok(body_plans)
}

fn split_continue_then_body<'a>(then_body: &'a [ASTNode]) -> Option<&'a [ASTNode]> {
    if then_body.is_empty() {
        return None;
    }
    if !matches!(then_body.last(), Some(ASTNode::Continue { .. })) {
        return None;
    }
    let body = &then_body[..then_body.len() - 1];
    Some(body)
}

fn split_break_else_body(
    then_body: &[ASTNode],
    else_body: &[ASTNode],
    loop_var: &str,
    loop_increment: &ASTNode,
) -> bool {
    if else_body.len() != 1 || !matches!(else_body[0], ASTNode::Break { .. }) {
        return false;
    }
    for stmt in then_body {
        if matches_loop_increment(stmt, loop_var, loop_increment) {
            return false;
        }
    }
    true
}

fn split_break_else_effect_body<'a>(
    then_body: &'a [ASTNode],
    else_body: &'a [ASTNode],
) -> Option<&'a [ASTNode]> {
    if then_body.len() != 1 || !matches!(then_body[0], ASTNode::Break { .. }) {
        return None;
    }
    if else_body.is_empty() {
        return None;
    }
    Some(else_body)
}
