use crate::ast::ASTNode;
use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;
use crate::mir::builder::control_flow::plan::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::plan::normalizer::PlanNormalizer;
use crate::mir::builder::control_flow::plan::normalizer::{loop_body_lowering, lower_cond_value};
use crate::mir::builder::control_flow::plan::steps::effects_to_plans;
use crate::mir::builder::control_flow::plan::trace as plan_trace;
use crate::mir::builder::control_flow::plan::{CoreEffectPlan, CorePlan, LoweredRecipe};
use crate::mir::builder::control_flow::verify::coreloop_body_contract::is_effect_only_stmt;
use crate::mir::builder::MirBuilder;
use crate::mir::{CompareOp, ConstValue, MirType};
use std::collections::{BTreeMap, BTreeSet};

use super::super::exit_branch;
use super::nested_loop_depth1_handoff::try_lower_generic_nested_loop_depth1_fastpath;
use super::nested_loop_recipe_fallback::try_compose_generic_nested_loop_recipe_fallback;
use super::nested_loop_reject_tail::finish_generic_nested_loop_reject_tail;
use super::GENERIC_LOOP_ERR;

pub(super) fn collect_loop_carriers(
    body: &[ASTNode],
    pre_loop_map: &BTreeMap<String, crate::mir::ValueId>,
    loop_var: &str,
) -> Vec<String> {
    let mut targets = BTreeSet::new();
    collect_assignment_targets(body, &mut targets);
    targets
        .into_iter()
        .filter(|name| name != loop_var && pre_loop_map.contains_key(name))
        .collect()
}

fn collect_assignment_targets(body: &[ASTNode], out: &mut BTreeSet<String>) {
    for stmt in body {
        match stmt {
            ASTNode::Assignment { target, .. } => {
                if let ASTNode::Variable { name, .. } = target.as_ref() {
                    out.insert(name.clone());
                }
            }
            ASTNode::If {
                then_body,
                else_body,
                ..
            } => {
                collect_assignment_targets(then_body, out);
                if let Some(else_body) = else_body {
                    collect_assignment_targets(else_body, out);
                }
            }
            ASTNode::Loop { body, .. } => {
                collect_assignment_targets(body, out);
            }
            _ => {}
        }
    }
}

pub(super) fn lower_nested_loop_plan(
    builder: &mut MirBuilder,
    condition: &ASTNode,
    body: &[ASTNode],
    ctx: &LoopRouteContext,
) -> Result<LoweredRecipe, String> {
    if let Some(plan) =
        try_lower_generic_nested_loop_depth1_fastpath(builder, condition, body, GENERIC_LOOP_ERR)
    {
        return Ok(plan);
    }

    use crate::mir::builder::control_flow::plan::single_planner;

    let nested_ctx =
        LoopRouteContext::new(condition, body, ctx.func_name, ctx.debug, ctx.in_static_box);
    let strict_or_dev = crate::config::env::joinir_dev::strict_enabled()
        || crate::config::env::joinir_dev_enabled();
    let planner_required =
        strict_or_dev && crate::config::env::joinir_dev::planner_required_enabled();

    let outcome = single_planner::try_build_outcome(&nested_ctx)?;
    plan_trace::trace_outcome_snapshot(
        "generic_loop_body::nested_loop_plan",
        false,
        outcome.facts.is_some(),
        outcome.recipe_contract.is_some(),
    );

    if let Some(recipe) = try_compose_generic_nested_loop_recipe_fallback(
        builder,
        &outcome,
        &nested_ctx,
        strict_or_dev,
        planner_required,
    )? {
        return Ok(recipe);
    }
    finish_generic_nested_loop_reject_tail(&outcome, &nested_ctx)
}

pub(super) fn apply_loop_final_values_to_bindings(
    builder: &mut MirBuilder,
    phi_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    plan: &LoweredRecipe,
) {
    let CorePlan::Loop(loop_plan) = plan else {
        return;
    };
    for (name, value_id) in &loop_plan.final_values {
        builder
            .variable_ctx
            .variable_map
            .insert(name.clone(), *value_id);
        if phi_bindings.contains_key(name) {
            phi_bindings.insert(name.clone(), *value_id);
        }
    }
}

pub(super) fn append_effects(body_plans: &mut Vec<LoweredRecipe>, effects: Vec<CoreEffectPlan>) {
    body_plans.extend(effects_to_plans(effects));
}

pub(super) fn matches_loop_increment(
    stmt: &ASTNode,
    loop_var: &str,
    loop_increment: &ASTNode,
) -> bool {
    let ASTNode::Assignment { target, value, .. } = stmt else {
        return false;
    };
    let ASTNode::Variable { name, .. } = target.as_ref() else {
        return false;
    };
    if name != loop_var {
        return false;
    }
    value.as_ref() == loop_increment
}

pub(super) fn lower_effect_only_stmt(
    builder: &mut MirBuilder,
    phi_bindings: &BTreeMap<String, crate::mir::ValueId>,
    stmt: &ASTNode,
) -> Result<Vec<CoreEffectPlan>, String> {
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
            Ok(effects)
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
            Ok(effects)
        }
        ASTNode::MethodCall { .. } => loop_body_lowering::lower_method_call_stmt(
            builder,
            phi_bindings,
            stmt,
            GENERIC_LOOP_ERR,
        ),
        ASTNode::FunctionCall { .. } => loop_body_lowering::lower_function_call_stmt(
            builder,
            phi_bindings,
            stmt,
            GENERIC_LOOP_ERR,
        ),
        ASTNode::Call { .. } => {
            let (_value_id, effects) =
                PlanNormalizer::lower_value_ast(stmt, builder, phi_bindings)?;
            Ok(effects)
        }
        _ => Err(format!(
            "{GENERIC_LOOP_ERR}: unsupported effect stmt {}",
            stmt.node_type()
        )),
    }
}

pub(super) fn lower_effect_block(
    builder: &mut MirBuilder,
    phi_bindings: &BTreeMap<String, crate::mir::ValueId>,
    body: &[ASTNode],
    loop_var: &str,
    loop_increment: &ASTNode,
) -> Result<Vec<CoreEffectPlan>, String> {
    let mut effects = Vec::new();
    for stmt in body {
        if matches_loop_increment(stmt, loop_var, loop_increment) {
            continue;
        }
        if is_effect_only_stmt(stmt) {
            effects.extend(lower_effect_only_stmt(builder, phi_bindings, stmt)?);
            continue;
        }
        if let ASTNode::If {
            condition,
            then_body,
            else_body,
            ..
        } = stmt
        {
            let cond_view = CondBlockView::from_expr(condition);
            let (cond_id, cond_effects) =
                lower_cond_value(builder, phi_bindings, &cond_view, GENERIC_LOOP_ERR)?;
            effects.extend(cond_effects);

            let then_effects =
                lower_effect_block(builder, phi_bindings, then_body, loop_var, loop_increment)?;
            let else_effects = match else_body {
                Some(body) => Some(lower_effect_block(
                    builder,
                    phi_bindings,
                    body,
                    loop_var,
                    loop_increment,
                )?),
                None => None,
            };

            effects.push(CoreEffectPlan::IfEffect {
                cond: cond_id,
                then_effects,
                else_effects,
            });
            continue;
        }
        if let ASTNode::Assignment { target, .. } = stmt {
            if matches!(target.as_ref(), ASTNode::Variable { name, .. } if name == loop_var) {
                return Err(format!("{GENERIC_LOOP_ERR}: effect block writes loop var"));
            }
        }
        if matches!(stmt, ASTNode::Local { .. } | ASTNode::Assignment { .. }) {
            effects.extend(lower_effect_only_stmt(builder, phi_bindings, stmt)?);
            continue;
        }
        return Err(format!("{GENERIC_LOOP_ERR}: unsupported effect block"));
    }
    Ok(effects)
}

pub(super) fn lower_if_effect(
    builder: &mut MirBuilder,
    phi_bindings: &BTreeMap<String, crate::mir::ValueId>,
    condition: &ASTNode,
    then_body: &[ASTNode],
    else_body: &[ASTNode],
    loop_var: &str,
    loop_increment: &ASTNode,
) -> Result<Vec<CoreEffectPlan>, String> {
    let cond_view = CondBlockView::from_expr(condition);
    let (cond_id, cond_effects) =
        lower_cond_value(builder, phi_bindings, &cond_view, GENERIC_LOOP_ERR)?;
    let mut effects = cond_effects;

    let mut then_effects =
        lower_effect_block(builder, phi_bindings, then_body, loop_var, loop_increment)?;
    then_effects.push(CoreEffectPlan::ExitIf {
        cond: cond_id,
        exit: exit_branch::build_continue_exit_plan(1),
    });

    let else_effects =
        lower_effect_block(builder, phi_bindings, else_body, loop_var, loop_increment)?;

    effects.push(CoreEffectPlan::IfEffect {
        cond: cond_id,
        then_effects,
        else_effects: Some(else_effects),
    });
    Ok(effects)
}

pub(super) fn lower_exit_from_stmt(
    builder: &mut MirBuilder,
    phi_bindings: &BTreeMap<String, crate::mir::ValueId>,
    cond_id: crate::mir::ValueId,
    stmt: &ASTNode,
) -> Result<Vec<CoreEffectPlan>, String> {
    let mut effects = Vec::new();
    let exit = match stmt {
        ASTNode::Break { .. } => exit_branch::build_break_exit_plan(1),
        ASTNode::Continue { .. } => exit_branch::build_continue_exit_plan(1),
        ASTNode::Return { value, .. } => {
            let Some(value) = value.as_ref() else {
                return Err(format!("{GENERIC_LOOP_ERR}: return without value"));
            };
            let (value_id, mut return_effects) =
                PlanNormalizer::lower_value_ast(value, builder, phi_bindings)?;
            effects.append(&mut return_effects);
            exit_branch::build_return_exit_plan(value_id)
        }
        _ => return Err(format!("{GENERIC_LOOP_ERR}: unsupported exit stmt")),
    };

    effects.push(CoreEffectPlan::ExitIf {
        cond: cond_id,
        exit,
    });
    Ok(effects)
}

pub(super) fn lower_negated_bool_cond(
    builder: &mut MirBuilder,
    cond_id: crate::mir::ValueId,
) -> (crate::mir::ValueId, Vec<CoreEffectPlan>) {
    let false_id = builder.alloc_typed(MirType::Bool);
    let neg_id = builder.alloc_typed(MirType::Bool);
    let effects = vec![
        CoreEffectPlan::Const {
            dst: false_id,
            value: ConstValue::Bool(false),
        },
        CoreEffectPlan::Compare {
            dst: neg_id,
            lhs: cond_id,
            op: CompareOp::Eq,
            rhs: false_id,
        },
    ];
    (neg_id, effects)
}
