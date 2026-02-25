//! Phase 29ca P1: Generic structured loop v0 normalizer (ExitIf + IfEffect, no carriers)

use crate::mir::builder::control_flow::plan::normalizer::helpers::create_phi_bindings;
use crate::mir::builder::control_flow::plan::normalizer::{
    build_pattern1_coreloop, loop_body_lowering, PlanNormalizer,
};
use crate::ast::ASTNode;
use crate::mir::builder::control_flow::joinir::patterns::router::LoopPatternContext;
use crate::mir::builder::control_flow::plan::coreloop_body_contract::is_effect_only_stmt;
use crate::mir::builder::control_flow::plan::{CoreEffectPlan, CoreExitPlan, CorePlan};
use crate::mir::builder::MirBuilder;
use crate::mir::{CompareOp, ConstValue, MirType};
use std::collections::BTreeMap;
use super::facts::{is_pure_value_expr, GenericLoopV0Facts, GenericLoopV1Facts};

const GENERIC_LOOP_ERR: &str = "[normalizer] generic loop v0";

pub(in crate::mir::builder) fn normalize_generic_loop_v0(
    builder: &mut MirBuilder,
    facts: &GenericLoopV0Facts,
    ctx: &LoopPatternContext,
) -> Result<CorePlan, String> {
    let mut loop_plan = build_pattern1_coreloop(
        builder,
        &facts.loop_var,
        &facts.condition,
        &facts.loop_increment,
        ctx,
    )?;

    let loop_var_current = loop_plan
        .final_values
        .iter()
        .find(|(name, _)| name == &facts.loop_var)
        .map(|(_, value)| *value)
        .ok_or_else(|| {
            format!(
                "[normalizer] loop var {} missing from final_values",
                facts.loop_var
            )
        })?;

    let phi_bindings = create_phi_bindings(&[(&facts.loop_var, loop_var_current)]);
    let mut body_plans: Vec<CorePlan> = Vec::new();

    for stmt in &facts.body {
        if matches_loop_increment(stmt, &facts.loop_var, &facts.loop_increment) {
            continue;
        }

        match stmt {
            ASTNode::Assignment { target, value, .. } => {
                let (name, value_id, effects) = loop_body_lowering::lower_assignment_value(
                    builder,
                    &phi_bindings,
                    target,
                    value,
                    GENERIC_LOOP_ERR,
                )?;
                builder
                    .variable_ctx
                    .variable_map
                    .insert(name, value_id);
                append_effects(&mut body_plans, effects);
            }
            ASTNode::Local {
                variables,
                initial_values,
                ..
            } => {
                let (inits, effects) = loop_body_lowering::lower_local_init_values(
                    builder,
                    &phi_bindings,
                    variables,
                    initial_values,
                    GENERIC_LOOP_ERR,
                )?;
                for (name, value_id) in inits {
                    builder
                        .variable_ctx
                        .variable_map
                        .insert(name, value_id);
                }
                append_effects(&mut body_plans, effects);
            }
            ASTNode::MethodCall { .. } => {
                append_effects(
                    &mut body_plans,
                    loop_body_lowering::lower_method_call_stmt(
                        builder,
                        &phi_bindings,
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
                        &phi_bindings,
                        stmt,
                        GENERIC_LOOP_ERR,
                    )?,
                );
            }
            ASTNode::Loop { condition, body, .. } => {
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
                        let (cond_id, mut cond_effects) =
                            loop_body_lowering::lower_bool_expr(
                                builder,
                                &phi_bindings,
                                condition,
                                GENERIC_LOOP_ERR,
                            )?;
                        let (cond_neg, mut neg_effects) =
                            lower_negated_bool_cond(builder, cond_id);
                        cond_effects.append(&mut neg_effects);
                        append_effects(&mut body_plans, cond_effects);
                        body_plans.push(CorePlan::Effect(CoreEffectPlan::ExitIf {
                            cond: cond_neg,
                            exit: CoreExitPlan::Break(1),
                        }));
                        continue;
                    }
                    if let Some(effect_stmts) =
                        split_break_else_effect_body(then_body, else_body)
                    {
                        let (cond_id, cond_effects) =
                            loop_body_lowering::lower_bool_expr(
                                builder,
                                &phi_bindings,
                                condition,
                                GENERIC_LOOP_ERR,
                            )?;
                        append_effects(&mut body_plans, cond_effects);
                        body_plans.push(CorePlan::Effect(CoreEffectPlan::ExitIf {
                            cond: cond_id,
                            exit: CoreExitPlan::Break(1),
                        }));
                        for stmt in effect_stmts {
                            append_effects(&mut body_plans, lower_effect_only_stmt(
                                builder,
                                &phi_bindings,
                                stmt,
                            )?);
                        }
                        continue;
                    }

                    let if_effects = lower_if_effect(
                        builder,
                        &phi_bindings,
                        condition,
                        then_body,
                        else_body,
                        &facts.loop_var,
                        &facts.loop_increment,
                    )?;
                    append_effects(&mut body_plans, if_effects);
                    continue;
                }
                let (cond_id, cond_effects) =
                    loop_body_lowering::lower_bool_expr(
                        builder,
                        &phi_bindings,
                        condition,
                        GENERIC_LOOP_ERR,
                    )?;
                append_effects(&mut body_plans, cond_effects);

                if let Some(effect_stmts) = split_continue_then_body(
                    then_body,
                    &facts.loop_var,
                    &facts.loop_increment,
                ) {
                    let mut then_effects = Vec::new();
                    for stmt in effect_stmts {
                        then_effects.extend(lower_effect_only_stmt(
                            builder,
                            &phi_bindings,
                            stmt,
                        )?);
                    }
                    then_effects.push(CoreEffectPlan::ExitIf {
                        cond: cond_id,
                        exit: CoreExitPlan::Continue(1),
                    });
                    body_plans.push(CorePlan::Effect(CoreEffectPlan::IfEffect {
                        cond: cond_id,
                        then_effects,
                        else_effects: None,
                    }));
                } else if then_body.len() == 1
                    && matches!(
                        then_body[0],
                        ASTNode::Break { .. }
                            | ASTNode::Continue { .. }
                            | ASTNode::Return { .. }
                    )
                {
                    let exit_effects = lower_exit_from_stmt(
                        builder,
                        &phi_bindings,
                        cond_id,
                        &then_body[0],
                    )?;
                    append_effects(&mut body_plans, exit_effects);
                } else {
                    let then_effects = lower_effect_block(
                        builder,
                        &phi_bindings,
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
            ASTNode::Break { .. }
            | ASTNode::Continue { .. }
            | ASTNode::Return { .. } => {
                let true_cond = builder.alloc_typed(MirType::Bool);
                body_plans.push(CorePlan::Effect(CoreEffectPlan::Const {
                    dst: true_cond,
                    value: ConstValue::Bool(true),
                }));
                let exit_effects =
                    lower_exit_from_stmt(builder, &phi_bindings, true_cond, stmt)?;
                append_effects(&mut body_plans, exit_effects);
            }
            _ => {
                return Err("[normalizer] generic loop v0: unsupported body stmt".to_string());
            }
        }
    }

    loop_plan.body = body_plans;
    Ok(CorePlan::Loop(loop_plan))
}

pub(in crate::mir::builder) fn normalize_generic_loop_v1(
    builder: &mut MirBuilder,
    facts: &GenericLoopV1Facts,
    ctx: &LoopPatternContext,
) -> Result<CorePlan, String> {
    let mut loop_plan = build_pattern1_coreloop(
        builder,
        &facts.loop_var,
        &facts.condition,
        &facts.loop_increment,
        ctx,
    )?;

    let loop_var_current = loop_plan
        .final_values
        .iter()
        .find(|(name, _)| name == &facts.loop_var)
        .map(|(_, value)| *value)
        .ok_or_else(|| {
            format!(
                "[normalizer] loop var {} missing from final_values",
                facts.loop_var
            )
        })?;

    let phi_bindings = create_phi_bindings(&[(&facts.loop_var, loop_var_current)]);
    let mut body_plans: Vec<CorePlan> = Vec::new();

    for stmt in &facts.body {
        if matches_loop_increment(stmt, &facts.loop_var, &facts.loop_increment) {
            continue;
        }
        let plans = lower_body_stmt_v1(
            builder,
            &phi_bindings,
            stmt,
            &facts.loop_var,
            &facts.loop_increment,
            ctx,
        )?;
        body_plans.extend(plans);
    }

    loop_plan.body = body_plans;
    Ok(CorePlan::Loop(loop_plan))
}

fn lower_nested_loop_plan(
    builder: &mut MirBuilder,
    condition: &ASTNode,
    body: &[ASTNode],
    ctx: &LoopPatternContext,
) -> Result<CorePlan, String> {
    use crate::mir::builder::control_flow::plan::single_planner;

    let nested_ctx = LoopPatternContext::new(
        condition,
        body,
        ctx.func_name,
        ctx.debug,
        ctx.in_static_box,
    );
    let (domain_plan, _outcome) = single_planner::try_build_domain_plan_with_outcome(&nested_ctx)?;
    let Some(domain_plan) = domain_plan else {
        return Err("[normalizer] generic loop v0: nested loop has no plan".to_string());
    };
    PlanNormalizer::normalize(builder, domain_plan, &nested_ctx)
}

fn lower_body_stmt_v1(
    builder: &mut MirBuilder,
    phi_bindings: &std::collections::BTreeMap<String, crate::mir::ValueId>,
    stmt: &ASTNode,
    loop_var: &str,
    loop_increment: &ASTNode,
    ctx: &LoopPatternContext,
) -> Result<Vec<CorePlan>, String> {
    match stmt {
        ASTNode::Assignment { target, value, .. } => {
            let (name, value_id, effects) = loop_body_lowering::lower_assignment_value(
                builder,
                phi_bindings,
                target,
                value,
                GENERIC_LOOP_ERR,
            )?;
            builder
                .variable_ctx
                .variable_map
                .insert(name, value_id);
            Ok(effects_to_plans(effects))
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
                builder
                    .variable_ctx
                    .variable_map
                    .insert(name, value_id);
            }
            Ok(effects_to_plans(effects))
        }
        ASTNode::MethodCall { .. } => {
            let effects = loop_body_lowering::lower_method_call_stmt(
                builder,
                phi_bindings,
                stmt,
                GENERIC_LOOP_ERR,
            )?;
            Ok(effects_to_plans(effects))
        }
        ASTNode::FunctionCall { .. } => {
            let effects = loop_body_lowering::lower_function_call_stmt(
                builder,
                phi_bindings,
                stmt,
                GENERIC_LOOP_ERR,
            )?;
            Ok(effects_to_plans(effects))
        }
        ASTNode::If {
            condition,
            then_body,
            else_body,
            ..
        } => lower_if_stmt_v1(
            builder,
            phi_bindings,
            condition,
            then_body,
            else_body.as_ref(),
            loop_var,
            loop_increment,
            ctx,
        ),
        ASTNode::Loop { condition, body, .. } => {
            let nested = lower_nested_loop_plan(builder, condition, body, ctx)?;
            Ok(vec![nested])
        }
        ASTNode::Break { .. }
        | ASTNode::Continue { .. }
        | ASTNode::Return { .. } => lower_exit_stmt_v1(builder, phi_bindings, stmt),
        _ => {
            if is_effect_only_stmt(stmt) {
                let effects = lower_effect_only_stmt(builder, phi_bindings, stmt)?;
                return Ok(effects_to_plans(effects));
            }
            Err("[normalizer] generic loop v1: unsupported body stmt".to_string())
        }
    }
}

fn lower_if_stmt_v1(
    builder: &mut MirBuilder,
    phi_bindings: &std::collections::BTreeMap<String, crate::mir::ValueId>,
    condition: &ASTNode,
    then_body: &[ASTNode],
    else_body: Option<&Vec<ASTNode>>,
    loop_var: &str,
    loop_increment: &ASTNode,
    ctx: &LoopPatternContext,
) -> Result<Vec<CorePlan>, String> {
    if then_body.is_empty() {
        return Err("[normalizer] generic loop v1: empty if body".to_string());
    }

    if let Some(plans) = try_lower_conditional_update_if(
        builder,
        phi_bindings,
        condition,
        then_body,
        else_body,
        loop_var,
    )? {
        return Ok(plans);
    }

    let (cond_id, cond_effects) = loop_body_lowering::lower_bool_expr(
        builder,
        phi_bindings,
        condition,
        GENERIC_LOOP_ERR,
    )?;
    let mut plans = effects_to_plans(cond_effects);

    let then_plans = lower_body_block_v1(
        builder,
        phi_bindings,
        then_body,
        loop_var,
        loop_increment,
        ctx,
    )?;
    let else_plans = match else_body {
        Some(body) => Some(lower_body_block_v1(
            builder,
            phi_bindings,
            body,
            loop_var,
            loop_increment,
            ctx,
        )?),
        None => None,
    };

    plans.push(CorePlan::If(crate::mir::builder::control_flow::plan::CoreIfPlan {
        condition: cond_id,
        then_plans,
        else_plans,
    }));
    Ok(plans)
}

struct CondUpdateBranch {
    updates: BTreeMap<String, crate::mir::ValueId>,
    effects: Vec<CoreEffectPlan>,
    exit: Option<CoreExitPlan>,
    saw_assignment: bool,
}

fn try_lower_conditional_update_if(
    builder: &mut MirBuilder,
    phi_bindings: &BTreeMap<String, crate::mir::ValueId>,
    condition: &ASTNode,
    then_body: &[ASTNode],
    else_body: Option<&Vec<ASTNode>>,
    loop_var: &str,
) -> Result<Option<Vec<CorePlan>>, String> {
    let has_update = has_non_loop_assignment(then_body, loop_var)
        || else_body.map_or(false, |body| has_non_loop_assignment(body, loop_var));
    if !has_update {
        return Ok(None);
    }

    // Conditional-update lowering is an optional fast-path (Select/ExitIf).
    // Be conservative: if either branch contains non-trivial statements
    // (nested if/local/methodcall/etc), fall back to full CorePlan::If lowering.
    if !can_attempt_conditional_update_branch(then_body, loop_var)
        || else_body.is_some_and(|body| !can_attempt_conditional_update_branch(body, loop_var))
    {
        return Ok(None);
    }

    let then_branch = collect_conditional_update_branch(builder, phi_bindings, then_body, loop_var)?;
    let else_branch = match else_body {
        Some(body) => collect_conditional_update_branch(builder, phi_bindings, body, loop_var)?,
        None => CondUpdateBranch {
            updates: BTreeMap::new(),
            effects: Vec::new(),
            exit: None,
            saw_assignment: false,
        },
    };

    if !then_branch.saw_assignment && !else_branch.saw_assignment {
        return Ok(None);
    }

    let (cond_id, cond_effects) = loop_body_lowering::lower_bool_expr(
        builder,
        phi_bindings,
        condition,
        GENERIC_LOOP_ERR,
    )?;

    let mut plans = effects_to_plans(cond_effects);

    let mut select_effects = Vec::new();
    select_effects.extend(then_branch.effects);
    select_effects.extend(else_branch.effects);

    let mut update_vars = BTreeMap::<String, ()>::new();
    for key in then_branch.updates.keys() {
        update_vars.insert(key.clone(), ());
    }
    for key in else_branch.updates.keys() {
        update_vars.insert(key.clone(), ());
    }

    for (var, _) in update_vars {
        let current = current_value_for_join(builder, phi_bindings, &var)?;
        let then_val = then_branch.updates.get(&var).copied().unwrap_or(current);
        let else_val = else_branch.updates.get(&var).copied().unwrap_or(current);
        let ty = builder
            .type_ctx
            .get_type(current)
            .cloned()
            .unwrap_or(MirType::Unknown);
        let dst = builder.alloc_typed(ty);
        select_effects.push(CoreEffectPlan::Select {
            dst,
            cond: cond_id,
            then_val,
            else_val,
        });
        builder.variable_ctx.variable_map.insert(var, dst);
    }

    plans.extend(effects_to_plans(select_effects));

    if let Some(exit) = then_branch.exit {
        plans.push(CorePlan::Effect(CoreEffectPlan::ExitIf {
            cond: cond_id,
            exit,
        }));
    }
    if let Some(exit) = else_branch.exit {
        let (cond_neg, neg_effects) = lower_negated_bool_cond(builder, cond_id);
        plans.extend(effects_to_plans(neg_effects));
        plans.push(CorePlan::Effect(CoreEffectPlan::ExitIf {
            cond: cond_neg,
            exit,
        }));
    }

    Ok(Some(plans))
}

fn can_attempt_conditional_update_branch(body: &[ASTNode], loop_var: &str) -> bool {
    use std::collections::BTreeSet;

    let mut updated_vars: BTreeSet<String> = BTreeSet::new();
    let mut saw_exit = false;

    for (idx, stmt) in body.iter().enumerate() {
        let is_last = idx + 1 == body.len();
        match stmt {
            ASTNode::Assignment { target, value, .. } => {
                if saw_exit {
                    return false;
                }
                let ASTNode::Variable { name, .. } = target.as_ref() else {
                    return false;
                };
                if name == loop_var {
                    return false;
                }
                if !is_pure_value_expr(value) {
                    return false;
                }
                if !updated_vars.insert(name.clone()) {
                    return false;
                }
            }
            ASTNode::Break { .. } | ASTNode::Continue { .. } => {
                if !is_last || saw_exit {
                    return false;
                }
                saw_exit = true;
            }
            _ => return false,
        }
    }

    true
}

fn collect_conditional_update_branch(
    builder: &mut MirBuilder,
    phi_bindings: &BTreeMap<String, crate::mir::ValueId>,
    body: &[ASTNode],
    loop_var: &str,
) -> Result<CondUpdateBranch, String> {
    let mut updates = BTreeMap::new();
    let mut effects = Vec::new();
    let mut exit = None;
    let mut saw_assignment = false;

    for (idx, stmt) in body.iter().enumerate() {
        let is_last = idx + 1 == body.len();
        match stmt {
            ASTNode::Assignment { target, value, .. } => {
                let ASTNode::Variable { name, .. } = target.as_ref() else {
                    return Err(format!("{GENERIC_LOOP_ERR}: conditional update target"));
                };
                if name == loop_var {
                    return Err(format!("{GENERIC_LOOP_ERR}: conditional update touches loop var"));
                }
                if !is_pure_value_expr(value) {
                    return Err(format!("{GENERIC_LOOP_ERR}: conditional update not pure"));
                }
                if updates.contains_key(name) {
                    return Err(format!("{GENERIC_LOOP_ERR}: duplicate update for {}", name));
                }
                let (var, value_id, mut new_effects) = loop_body_lowering::lower_assignment_value(
                    builder,
                    phi_bindings,
                    target,
                    value,
                    GENERIC_LOOP_ERR,
                )?;
                effects.append(&mut new_effects);
                updates.insert(var, value_id);
                saw_assignment = true;
            }
            ASTNode::Break { .. } => {
                if !is_last || exit.is_some() {
                    return Err(format!("{GENERIC_LOOP_ERR}: break not at tail"));
                }
                exit = Some(CoreExitPlan::Break(1));
            }
            ASTNode::Continue { .. } => {
                if !is_last || exit.is_some() {
                    return Err(format!("{GENERIC_LOOP_ERR}: continue not at tail"));
                }
                exit = Some(CoreExitPlan::Continue(1));
            }
            _ => {
                return Err(format!("{GENERIC_LOOP_ERR}: conditional update has unsupported stmt"));
            }
        }
    }

    Ok(CondUpdateBranch {
        updates,
        effects,
        exit,
        saw_assignment,
    })
}

fn current_value_for_join(
    builder: &MirBuilder,
    phi_bindings: &BTreeMap<String, crate::mir::ValueId>,
    name: &str,
) -> Result<crate::mir::ValueId, String> {
    if let Some(value) = phi_bindings.get(name) {
        return Ok(*value);
    }
    if let Some(value) = builder.variable_ctx.variable_map.get(name) {
        return Ok(*value);
    }
    Err(format!(
        "{GENERIC_LOOP_ERR}: join value {} not found",
        name
    ))
}

fn has_non_loop_assignment(body: &[ASTNode], loop_var: &str) -> bool {
    body.iter().any(|stmt| {
        let ASTNode::Assignment { target, .. } = stmt else {
            return false;
        };
        let ASTNode::Variable { name, .. } = target.as_ref() else {
            return false;
        };
        name != loop_var
    })
}

fn lower_body_block_v1(
    builder: &mut MirBuilder,
    phi_bindings: &std::collections::BTreeMap<String, crate::mir::ValueId>,
    body: &[ASTNode],
    loop_var: &str,
    loop_increment: &ASTNode,
    ctx: &LoopPatternContext,
) -> Result<Vec<CorePlan>, String> {
    let mut plans = Vec::new();
    for (idx, stmt) in body.iter().enumerate() {
        let stmt_plans =
            lower_body_stmt_v1(builder, phi_bindings, stmt, loop_var, loop_increment, ctx)?;
        if stmt_plans.iter().any(|plan| matches!(plan, CorePlan::Exit(_)))
            && idx + 1 != body.len()
        {
            return Err("[normalizer] generic loop v1: exit not last in block".to_string());
        }
        plans.extend(stmt_plans);
    }
    Ok(plans)
}

fn lower_exit_stmt_v1(
    builder: &mut MirBuilder,
    phi_bindings: &std::collections::BTreeMap<String, crate::mir::ValueId>,
    stmt: &ASTNode,
) -> Result<Vec<CorePlan>, String> {
    let mut plans = Vec::new();
    let exit = match stmt {
        ASTNode::Break { .. } => CoreExitPlan::Break(1),
        ASTNode::Continue { .. } => CoreExitPlan::Continue(1),
        ASTNode::Return { value, .. } => {
            let Some(value) = value.as_ref() else {
                return Err("[normalizer] generic loop v1: return without value".to_string());
            };
            let (value_id, effects) = PlanNormalizer::lower_value_ast(value, builder, phi_bindings)?;
            plans.extend(effects_to_plans(effects));
            CoreExitPlan::Return(Some(value_id))
        }
        _ => {
            return Err("[normalizer] generic loop v1: unsupported exit".to_string());
        }
    };
    plans.push(CorePlan::Exit(exit));
    Ok(plans)
}

fn effects_to_plans(effects: Vec<CoreEffectPlan>) -> Vec<CorePlan> {
    effects.into_iter().map(CorePlan::Effect).collect()
}

fn append_effects(body_plans: &mut Vec<CorePlan>, effects: Vec<CoreEffectPlan>) {
    for effect in effects {
        body_plans.push(CorePlan::Effect(effect));
    }
}

fn matches_loop_increment(
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

fn split_continue_then_body<'a>(
    then_body: &'a [ASTNode],
    loop_var: &str,
    loop_increment: &ASTNode,
) -> Option<Vec<&'a ASTNode>> {
    if then_body.is_empty() {
        return None;
    }
    if !matches!(then_body.last(), Some(ASTNode::Continue { .. })) {
        return None;
    }
    if then_body.len() == 1 {
        return None;
    }

    let mut effects = Vec::new();
    let mut saw_increment = false;
    for (idx, stmt) in then_body.iter().enumerate() {
        let is_last = idx + 1 == then_body.len();
        if is_last {
            return Some(effects);
        }
        if matches_loop_increment(stmt, loop_var, loop_increment) {
            if idx + 2 != then_body.len() || saw_increment {
                return None;
            }
            saw_increment = true;
            continue;
        }
        if is_effect_only_stmt(stmt) {
            effects.push(stmt);
            continue;
        }
        return None;
    }
    None
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
    if then_body.is_empty() {
        return false;
    }
    let mut saw_increment = false;
    for stmt in then_body {
        if matches_loop_increment(stmt, loop_var, loop_increment) {
            if saw_increment {
                return false;
            }
            saw_increment = true;
            continue;
        }
        return false;
    }
    saw_increment
}

fn split_break_else_effect_body<'a>(
    then_body: &'a [ASTNode],
    else_body: &'a [ASTNode],
) -> Option<Vec<&'a ASTNode>> {
    if then_body.len() != 1 || !matches!(then_body[0], ASTNode::Break { .. }) {
        return None;
    }
    if else_body.is_empty() {
        return None;
    }
    let mut effects = Vec::new();
    for stmt in else_body {
        if matches!(
            stmt,
            ASTNode::Assignment { .. }
                | ASTNode::Local { .. }
                | ASTNode::MethodCall { .. }
                | ASTNode::FunctionCall { .. }
        ) {
            effects.push(stmt);
            continue;
        }
        if is_effect_only_stmt(stmt) {
            effects.push(stmt);
            continue;
        }
        return None;
    }
    Some(effects)
}

fn lower_effect_only_stmt(
    builder: &mut MirBuilder,
    phi_bindings: &std::collections::BTreeMap<String, crate::mir::ValueId>,
    stmt: &ASTNode,
) -> Result<Vec<CoreEffectPlan>, String> {
    match stmt {
        ASTNode::Assignment { target, value, .. } => {
            let (name, value_id, effects) = loop_body_lowering::lower_assignment_value(
                builder,
                phi_bindings,
                target,
                value,
                GENERIC_LOOP_ERR,
            )?;
            builder
                .variable_ctx
                .variable_map
                .insert(name, value_id);
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
                builder
                    .variable_ctx
                    .variable_map
                    .insert(name, value_id);
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
        _ => Err(format!("{GENERIC_LOOP_ERR}: unsupported if-effect stmt")),
    }
}

fn lower_effect_block(
    builder: &mut MirBuilder,
    phi_bindings: &std::collections::BTreeMap<String, crate::mir::ValueId>,
    body: &[ASTNode],
    loop_var: &str,
    loop_increment: &ASTNode,
) -> Result<Vec<CoreEffectPlan>, String> {
    let mut effects = Vec::new();
    for stmt in body {
        if matches_loop_increment(stmt, loop_var, loop_increment) {
            return Err("[normalizer] generic loop v0: loop increment inside if".to_string());
        }
        match stmt {
            ASTNode::Assignment { target, value, .. } => {
                let (name, value_id, mut new_effects) = loop_body_lowering::lower_assignment_value(
                    builder,
                    phi_bindings,
                    target,
                    value,
                    GENERIC_LOOP_ERR,
                )?;
                builder
                    .variable_ctx
                    .variable_map
                    .insert(name, value_id);
                effects.append(&mut new_effects);
            }
            ASTNode::Local {
                variables,
                initial_values,
                ..
            } => {
                let (inits, mut new_effects) = loop_body_lowering::lower_local_init_values(
                    builder,
                    phi_bindings,
                    variables,
                    initial_values,
                    GENERIC_LOOP_ERR,
                )?;
                for (name, value_id) in inits {
                    builder
                        .variable_ctx
                        .variable_map
                        .insert(name, value_id);
                }
                effects.append(&mut new_effects);
            }
            ASTNode::MethodCall { .. } => {
                effects.extend(loop_body_lowering::lower_method_call_stmt(
                    builder,
                    phi_bindings,
                    stmt,
                    GENERIC_LOOP_ERR,
                )?);
            }
            ASTNode::FunctionCall { .. } => {
                effects.extend(loop_body_lowering::lower_function_call_stmt(
                    builder,
                    phi_bindings,
                    stmt,
                    GENERIC_LOOP_ERR,
                )?);
            }
            ASTNode::If {
                condition,
                then_body,
                else_body,
                ..
            } => {
                let Some(else_body) = else_body else {
                    let mut nested = lower_if_effect(
                        builder,
                        phi_bindings,
                        condition,
                        then_body,
                        &[],
                        loop_var,
                        loop_increment,
                    )?;
                    effects.append(&mut nested);
                    continue;
                };
                let mut nested = lower_if_effect(
                    builder,
                    phi_bindings,
                    condition,
                    then_body,
                    else_body,
                    loop_var,
                    loop_increment,
                )?;
                effects.append(&mut nested);
            }
            ASTNode::Break { .. }
            | ASTNode::Continue { .. }
            | ASTNode::Return { .. } => {
                return Err("[normalizer] generic loop v0: exit in if-effect".to_string());
            }
            _ => {
                return Err("[normalizer] generic loop v0: unsupported if-effect stmt".to_string());
            }
        }
    }
    Ok(effects)
}

fn lower_if_effect(
    builder: &mut MirBuilder,
    phi_bindings: &std::collections::BTreeMap<String, crate::mir::ValueId>,
    condition: &ASTNode,
    then_body: &[ASTNode],
    else_body: &[ASTNode],
    loop_var: &str,
    loop_increment: &ASTNode,
) -> Result<Vec<CoreEffectPlan>, String> {
    if then_body.is_empty() {
        return Err("[normalizer] generic loop v0: empty if body".to_string());
    }
    let (cond_id, mut cond_effects) =
        loop_body_lowering::lower_bool_expr(
            builder,
            phi_bindings,
            condition,
            GENERIC_LOOP_ERR,
        )?;
    let then_effects =
        lower_effect_block(builder, phi_bindings, then_body, loop_var, loop_increment)?;
    let else_effects = if else_body.is_empty() {
        None
    } else {
        Some(lower_effect_block(
            builder,
            phi_bindings,
            else_body,
            loop_var,
            loop_increment,
        )?)
    };
    cond_effects.push(CoreEffectPlan::IfEffect {
        cond: cond_id,
        then_effects,
        else_effects,
    });
    Ok(cond_effects)
}

fn lower_exit_from_stmt(
    builder: &mut MirBuilder,
    phi_bindings: &std::collections::BTreeMap<String, crate::mir::ValueId>,
    cond: crate::mir::ValueId,
    stmt: &ASTNode,
) -> Result<Vec<CoreEffectPlan>, String> {
    let mut effects = Vec::new();
    let exit = match stmt {
        ASTNode::Break { .. } => CoreExitPlan::Break(1),
        ASTNode::Continue { .. } => CoreExitPlan::Continue(1),
        ASTNode::Return { value, .. } => {
            let Some(value) = value.as_ref() else {
                return Err("[normalizer] generic loop v0: return without value".to_string());
            };
            let (value_id, mut return_effects) =
                PlanNormalizer::lower_value_ast(value, builder, phi_bindings)?;
            effects.append(&mut return_effects);
            CoreExitPlan::Return(Some(value_id))
        }
        _ => {
            return Err("[normalizer] generic loop v0: unsupported exit".to_string());
        }
    };

    effects.push(CoreEffectPlan::ExitIf { cond, exit });
    Ok(effects)
}

fn lower_negated_bool_cond(
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
