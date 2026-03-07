use crate::ast::ASTNode;
use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;
use crate::mir::builder::control_flow::plan::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::plan::coreloop_body_contract::is_effect_only_stmt;
use crate::mir::builder::control_flow::plan::facts::expr_generic_loop::is_pure_value_expr_for_generic_loop;
use crate::mir::builder::control_flow::plan::facts::feature_facts::detect_nested_loop;
use crate::mir::builder::control_flow::plan::facts::no_exit_block::try_build_no_exit_block_recipe;
use crate::mir::builder::control_flow::plan::features::conditional_update_join;
use crate::mir::builder::control_flow::plan::generic_loop::facts_types::GenericLoopV1Facts;
use crate::mir::builder::control_flow::plan::normalizer::loop_body_lowering;
use crate::mir::builder::control_flow::plan::normalizer::PlanNormalizer;
use crate::mir::builder::control_flow::plan::parts;
use crate::mir::builder::control_flow::plan::steps::effects_to_plans;
use crate::mir::builder::control_flow::plan::{
    CoreEffectPlan, CoreExitPlan, CorePlan, LoweredRecipe,
};
use crate::mir::builder::MirBuilder;
use crate::mir::policies::BodyLoweringPolicy;
use crate::mir::{Effect, EffectMask};
use std::collections::BTreeMap;

use super::helpers::{
    apply_loop_final_values_to_bindings, lower_effect_only_stmt,
    lower_nested_loop_plan, matches_loop_increment,
};
use super::GENERIC_LOOP_ERR;

pub(in crate::mir::builder) fn lower_generic_loop_v1_body(
    builder: &mut MirBuilder,
    facts: &GenericLoopV1Facts,
    phi_bindings: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    ctx: &LoopRouteContext,
) -> Result<Vec<LoweredRecipe>, String> {
    let mut body_plans: Vec<LoweredRecipe> = Vec::new();
    let mut current_bindings = phi_bindings.clone();
    for (name, value_id) in phi_bindings {
        builder
            .variable_ctx
            .variable_map
            .insert(name.clone(), *value_id);
    }

    // M28: Under planner_required, lower via Facts-provided RecipeBlock (NoExit).
    // This avoids re-checking in lower and keeps acceptance Recipe-first.
    let has_nested_loop_stmt = detect_nested_loop(&facts.body.body)
        || body_has_blockexpr_prelude_loop(&facts.body.body);
    if crate::config::env::joinir_dev::planner_required_enabled() && !has_nested_loop_stmt {
        if let Some(body_no_exit) = facts.body_no_exit.as_ref() {
            return parts::entry::lower_loop_with_body_block(
                builder,
                &mut current_bindings,
                carrier_step_phis,
                &body_no_exit.arena,
                &body_no_exit.block,
                parts::LoopBodyContractKind::NoExit,
                GENERIC_LOOP_ERR,
            );
        }
    }

    for stmt in &facts.body.body {
        if matches_loop_increment(stmt, &facts.loop_var, &facts.loop_increment) {
            continue;
        }
        let plans = lower_body_stmt_v1(
            builder,
            &mut current_bindings,
            stmt,
            facts,
            &facts.loop_var,
            &facts.loop_increment,
            carrier_step_phis,
            ctx,
        )?;
        body_plans.extend(plans);
    }

    if !matches!(body_plans.last(), Some(CorePlan::Exit(_))) {
        body_plans.push(CorePlan::Exit(parts::exit::build_continue_with_phi_args(
            builder,
            carrier_step_phis,
            &current_bindings,
            GENERIC_LOOP_ERR,
        )?));
    }

    Ok(body_plans)
}

fn lower_body_stmt_v1(
    builder: &mut MirBuilder,
    phi_bindings: &mut std::collections::BTreeMap<String, crate::mir::ValueId>,
    stmt: &ASTNode,
    facts: &GenericLoopV1Facts,
    loop_var: &str,
    loop_increment: &ASTNode,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    ctx: &LoopRouteContext,
) -> Result<Vec<LoweredRecipe>, String> {
    match stmt {
        ASTNode::Assignment { target, value, .. } => {
            if matches!(target.as_ref(), ASTNode::Variable { .. }) {
                if let Some((value_id, plans)) = try_lower_blockexpr_loop_prelude_value(
                    builder,
                    phi_bindings,
                    carrier_step_phis,
                    facts,
                    loop_var,
                    loop_increment,
                    ctx,
                    value,
                )? {
                    let ASTNode::Variable { name, .. } = target.as_ref() else {
                        unreachable!();
                    };
                    builder.variable_ctx.variable_map.insert(name.clone(), value_id);
                    phi_bindings.insert(name.clone(), value_id);
                    return Ok(plans);
                }
            }
            let (name, value_id, effects) = loop_body_lowering::lower_assignment_value(
                builder,
                phi_bindings,
                target,
                value,
                GENERIC_LOOP_ERR,
            )?;
            builder.variable_ctx.variable_map.insert(name, value_id);
            if let ASTNode::Variable { name, .. } = target.as_ref() {
                phi_bindings.insert(name.clone(), value_id);
            }
            Ok(effects_to_plans(effects))
        }
        ASTNode::Local {
            variables,
            initial_values,
            ..
        } => {
            if variables.len() == 1 && initial_values.len() == 1 {
                if let Some(init) = initial_values[0].as_deref() {
                    if let Some((value_id, plans)) = try_lower_blockexpr_loop_prelude_value(
                        builder,
                        phi_bindings,
                        carrier_step_phis,
                        facts,
                        loop_var,
                        loop_increment,
                        ctx,
                        init,
                    )? {
                        let name = variables[0].clone();
                        builder.variable_ctx.variable_map.insert(name.clone(), value_id);
                        phi_bindings.insert(name, value_id);
                        return Ok(plans);
                    }
                }
            }
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
                    .insert(name.clone(), value_id);
                phi_bindings.insert(name, value_id);
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
            Ok(effects_to_plans(effects))
        }
        ASTNode::Program { statements, .. } => {
            let mut body_plans = Vec::new();
            for inner in statements {
                let plans = lower_body_stmt_v1(
                    builder,
                    phi_bindings,
                    inner,
                    facts,
                    loop_var,
                    loop_increment,
                    carrier_step_phis,
                    ctx,
                )?;
                body_plans.extend(plans);
            }
            Ok(body_plans)
        }
        ASTNode::If {
            condition,
            then_body,
            else_body,
            ..
        } => lower_if_stmt_v1(
            builder,
            phi_bindings,
            stmt,
            condition,
            then_body,
            else_body.as_ref(),
            facts,
            loop_var,
            loop_increment,
            carrier_step_phis,
            ctx,
        ),
        ASTNode::Loop {
            condition, body, ..
        } => {
            let nested = lower_nested_loop_plan(builder, condition, body, ctx)?;
            apply_loop_final_values_to_bindings(builder, phi_bindings, &nested);
            Ok(vec![nested])
        }
        ASTNode::Break { .. } | ASTNode::Continue { .. } | ASTNode::Return { .. } => {
            lower_exit_stmt_v1(builder, phi_bindings, carrier_step_phis, stmt)
        }
        _ => {
            if is_effect_only_stmt(stmt) {
                let effects = lower_effect_only_stmt(builder, phi_bindings, stmt)?;
                return Ok(effects_to_plans(effects));
            }
            Err("[normalizer] generic loop v1: unsupported body stmt".to_string())
        }
    }
}

fn try_lower_blockexpr_loop_prelude_value(
    builder: &mut MirBuilder,
    phi_bindings: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    facts: &GenericLoopV1Facts,
    loop_var: &str,
    loop_increment: &ASTNode,
    ctx: &LoopRouteContext,
    value: &ASTNode,
) -> Result<Option<(crate::mir::ValueId, Vec<LoweredRecipe>)>, String> {
    let ASTNode::BlockExpr {
        prelude_stmts,
        tail_expr,
        ..
    } = value
    else {
        return Ok(None);
    };
    if !prelude_stmts
        .iter()
        .any(|stmt| matches!(stmt, ASTNode::Loop { .. } | ASTNode::While { .. }))
    {
        return Ok(None);
    }
    for stmt in prelude_stmts {
        if stmt.contains_non_local_exit_outside_loops() {
            return Err(
                "[freeze:contract][blockexpr] exit stmt is forbidden in BlockExpr prelude"
                    .to_string(),
            );
        }
    }

    let mut bindings = phi_bindings.clone();
    let mut plans = Vec::new();
    for stmt in prelude_stmts {
        let mut stmt_plans = lower_body_stmt_v1(
            builder,
            &mut bindings,
            stmt,
            facts,
            loop_var,
            loop_increment,
            carrier_step_phis,
            ctx,
        )?;
        plans.append(&mut stmt_plans);
    }

    let (tail_id, tail_effects) =
        PlanNormalizer::lower_value_ast(tail_expr.as_ref(), builder, &bindings)?;
    plans.extend(effects_to_plans(tail_effects));
    Ok(Some((tail_id, plans)))
}

fn body_has_blockexpr_prelude_loop(body: &[ASTNode]) -> bool {
    body.iter().any(stmt_has_blockexpr_prelude_loop)
}

fn stmt_has_blockexpr_prelude_loop(stmt: &ASTNode) -> bool {
    match stmt {
        ASTNode::Assignment { value, .. } => expr_has_blockexpr_prelude_loop(value),
        ASTNode::Local {
            initial_values, ..
        } => initial_values.iter().flatten().any(|v| expr_has_blockexpr_prelude_loop(v)),
        ASTNode::If {
            condition,
            then_body,
            else_body,
            ..
        } => {
            expr_has_blockexpr_prelude_loop(condition)
                || body_has_blockexpr_prelude_loop(then_body)
                || else_body
                    .as_ref()
                    .is_some_and(|b| body_has_blockexpr_prelude_loop(b))
        }
        ASTNode::Program { statements, .. } => body_has_blockexpr_prelude_loop(statements),
        ASTNode::Loop {
            condition, body, ..
        }
        | ASTNode::While {
            condition, body, ..
        } => {
            expr_has_blockexpr_prelude_loop(condition) || body_has_blockexpr_prelude_loop(body)
        }
        ASTNode::Return { value, .. } => value
            .as_ref()
            .is_some_and(|v| expr_has_blockexpr_prelude_loop(v)),
        ASTNode::Print { expression, .. } => expr_has_blockexpr_prelude_loop(expression),
        ASTNode::MethodCall {
            object, arguments, ..
        } => {
            expr_has_blockexpr_prelude_loop(object)
                || arguments.iter().any(expr_has_blockexpr_prelude_loop)
        }
        ASTNode::FunctionCall { arguments, .. } => {
            arguments.iter().any(expr_has_blockexpr_prelude_loop)
        }
        ASTNode::Call {
            callee, arguments, ..
        } => {
            expr_has_blockexpr_prelude_loop(callee)
                || arguments.iter().any(expr_has_blockexpr_prelude_loop)
        }
        _ => false,
    }
}

fn expr_has_blockexpr_prelude_loop(expr: &ASTNode) -> bool {
    match expr {
        ASTNode::BlockExpr {
            prelude_stmts,
            tail_expr,
            ..
        } => {
            prelude_stmts
                .iter()
                .any(|stmt| matches!(stmt, ASTNode::Loop { .. } | ASTNode::While { .. }))
                || body_has_blockexpr_prelude_loop(prelude_stmts)
                || expr_has_blockexpr_prelude_loop(tail_expr)
        }
        ASTNode::BinaryOp { left, right, .. } => {
            expr_has_blockexpr_prelude_loop(left) || expr_has_blockexpr_prelude_loop(right)
        }
        ASTNode::UnaryOp { operand, .. } => expr_has_blockexpr_prelude_loop(operand),
        ASTNode::MethodCall {
            object, arguments, ..
        } => {
            expr_has_blockexpr_prelude_loop(object)
                || arguments.iter().any(expr_has_blockexpr_prelude_loop)
        }
        ASTNode::FunctionCall { arguments, .. } => {
            arguments.iter().any(expr_has_blockexpr_prelude_loop)
        }
        ASTNode::Call {
            callee, arguments, ..
        } => {
            expr_has_blockexpr_prelude_loop(callee)
                || arguments.iter().any(expr_has_blockexpr_prelude_loop)
        }
        ASTNode::Index { target, index, .. } => {
            expr_has_blockexpr_prelude_loop(target) || expr_has_blockexpr_prelude_loop(index)
        }
        ASTNode::FieldAccess { object, .. } => expr_has_blockexpr_prelude_loop(object),
        ASTNode::ArrayLiteral { elements, .. } => {
            elements.iter().any(expr_has_blockexpr_prelude_loop)
        }
        ASTNode::MapLiteral { entries, .. } => entries
            .iter()
            .any(|(_, value)| expr_has_blockexpr_prelude_loop(value)),
        ASTNode::If {
            condition,
            then_body,
            else_body,
            ..
        } => {
            expr_has_blockexpr_prelude_loop(condition)
                || body_has_blockexpr_prelude_loop(then_body)
                || else_body
                    .as_ref()
                    .is_some_and(|b| body_has_blockexpr_prelude_loop(b))
        }
        ASTNode::Program { statements, .. } => body_has_blockexpr_prelude_loop(statements),
        _ => false,
    }
}

fn lower_if_stmt_v1(
    builder: &mut MirBuilder,
    phi_bindings: &mut std::collections::BTreeMap<String, crate::mir::ValueId>,
    stmt: &ASTNode,
    condition: &ASTNode,
    then_body: &[ASTNode],
    else_body: Option<&Vec<ASTNode>>,
    facts: &GenericLoopV1Facts,
    loop_var: &str,
    loop_increment: &ASTNode,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    ctx: &LoopRouteContext,
) -> Result<Vec<LoweredRecipe>, String> {
    if let Some(if_plans) = try_lower_conditional_update_if(
        builder,
        phi_bindings,
        carrier_step_phis,
        condition,
        then_body,
        else_body,
        loop_var,
    )? {
        return Ok(if_plans);
    }

    let cond_view = CondBlockView::from_expr(condition);

    // M27: Under planner_required, use RecipeBlock-based lowering for stmt-only if(no-exit)
    if crate::config::env::joinir_dev::planner_required_enabled() {
        if let Some(recipe) = try_build_no_exit_block_recipe(std::slice::from_ref(stmt), true) {
            let empty_carrier_phis = BTreeMap::new();
            let verified = parts::entry::verify_no_exit_block_with_pre(
                &recipe.arena,
                &recipe.block,
                GENERIC_LOOP_ERR,
                Some(phi_bindings),
            )?;
            return parts::entry::lower_no_exit_block_verified(
                builder,
                phi_bindings,
                &empty_carrier_phis,
                None, // break_phi_dsts
                verified,
                GENERIC_LOOP_ERR,
            );
        }
    }

    // Fallback: inline lowering with branch closures
    let pre_map = builder.variable_ctx.variable_map.clone();

    let mut lower_then =
        |builder: &mut MirBuilder, bindings: &mut BTreeMap<String, crate::mir::ValueId>| {
            let mut then_bindings = bindings.clone();
            builder.variable_ctx.variable_map = pre_map.clone();
            let plans = lower_body_block_v1(
                builder,
                &mut then_bindings,
                facts,
                then_body,
                loop_var,
                loop_increment,
                carrier_step_phis,
                ctx,
            )?;
            *bindings = then_bindings;
            Ok(plans)
        };

    let mut lower_else_closure =
        |builder: &mut MirBuilder, bindings: &mut BTreeMap<String, crate::mir::ValueId>| {
            let Some(body) = else_body else {
                return Ok(Vec::new());
            };
            let mut else_bindings = bindings.clone();
            builder.variable_ctx.variable_map = pre_map.clone();
            let plans = lower_body_block_v1(
                builder,
                &mut else_bindings,
                facts,
                body,
                loop_var,
                loop_increment,
                carrier_step_phis,
                ctx,
            )?;
            *bindings = else_bindings;
            Ok(plans)
        };

    let lower_else = else_body.as_ref().map(|_| {
        &mut lower_else_closure
            as &mut dyn FnMut(
                &mut MirBuilder,
                &mut BTreeMap<String, crate::mir::ValueId>,
            ) -> Result<Vec<LoweredRecipe>, String>
    });

    parts::entry::lower_if_join_with_branch_lowerers(
        builder,
        phi_bindings,
        &cond_view,
        GENERIC_LOOP_ERR,
        &mut lower_then,
        lower_else,
        &|_name, _bindings| true,
    )
}

fn try_lower_conditional_update_if(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    condition: &ASTNode,
    then_body: &[ASTNode],
    else_body: Option<&Vec<ASTNode>>,
    loop_var: &str,
) -> Result<Option<Vec<LoweredRecipe>>, String> {
    let has_update = has_non_loop_assignment(then_body, loop_var)
        || else_body.map_or(false, |body| has_non_loop_assignment(body, loop_var));
    if !has_update {
        return Ok(None);
    }

    if !can_attempt_conditional_update_branch(then_body)
        || else_body.is_some_and(|body| !can_attempt_conditional_update_branch(body))
    {
        return Ok(None);
    }

    let carrier_phis = BTreeMap::new();
    let mut carrier_updates = BTreeMap::new();
    conditional_update_join::try_lower_conditional_update_if(
        builder,
        current_bindings,
        &carrier_phis,
        carrier_step_phis,
        &mut carrier_updates,
        condition,
        then_body,
        else_body,
        GENERIC_LOOP_ERR,
    )
}

fn can_attempt_conditional_update_branch(body: &[ASTNode]) -> bool {
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
                // Allow loop-var updates inside conditional branches.
                // In generic_loop_v1, loop step is emitted from skeleton loop_increment,
                // so branch-local loop-var assignment is modeled as a pure conditional update.
                if !is_pure_value_expr_for_generic_loop(value) {
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

fn has_non_loop_assignment(body: &[ASTNode], loop_var: &str) -> bool {
    body.iter().any(|stmt| match stmt {
        ASTNode::Assignment { target, .. } => match target.as_ref() {
            ASTNode::Variable { name, .. } => name != loop_var,
            _ => false,
        },
        ASTNode::If {
            then_body,
            else_body,
            ..
        } => {
            has_non_loop_assignment(then_body, loop_var)
                || else_body
                    .as_ref()
                    .map_or(false, |body| has_non_loop_assignment(body, loop_var))
        }
        _ => false,
    })
}

fn lower_body_block_v1(
    builder: &mut MirBuilder,
    phi_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    facts: &GenericLoopV1Facts,
    body: &[ASTNode],
    loop_var: &str,
    loop_increment: &ASTNode,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    ctx: &LoopRouteContext,
) -> Result<Vec<LoweredRecipe>, String> {
    // One-part lowering path (ExitAllowed RecipeBlock), restricted to blocks that do not contain
    // `break`/`continue` so we don't introduce phi-arg requirements into generic_loop_v1.
    if matches!(
        facts.body_lowering_policy,
        BodyLoweringPolicy::ExitAllowed { .. }
    ) {
        let Some(recipe) = facts.body_exit_allowed.as_ref() else {
            return Err(format!(
                "[freeze:contract][generic_loop_v1] body_lowering_policy=ExitAllowed but body_exit_allowed=None: ctx={GENERIC_LOOP_ERR}"
            ));
        };
        let empty_break_phi_dsts = BTreeMap::new();
        let verified = parts::entry::verify_exit_allowed_block_with_pre(
            &recipe.arena,
            &recipe.block,
            GENERIC_LOOP_ERR,
            Some(phi_bindings),
        )?;
        return parts::entry::lower_exit_allowed_block_verified(
            builder,
            phi_bindings,
            carrier_step_phis,
            &empty_break_phi_dsts,
            verified,
            GENERIC_LOOP_ERR,
        );
    }

    let mut body_plans = Vec::new();
    for stmt in body {
        let plans = lower_body_stmt_v1(
            builder,
            phi_bindings,
            stmt,
            facts,
            loop_var,
            loop_increment,
            carrier_step_phis,
            ctx,
        )?;
        body_plans.extend(plans);
    }
    Ok(body_plans)
}

fn lower_exit_stmt_v1(
    builder: &mut MirBuilder,
    phi_bindings: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    stmt: &ASTNode,
) -> Result<Vec<LoweredRecipe>, String> {
    match stmt {
        ASTNode::Break { .. } => Ok(vec![CorePlan::Exit(CoreExitPlan::Break(1))]),
        ASTNode::Continue { .. } => Ok(vec![CorePlan::Exit(
            parts::exit::build_continue_with_phi_args(
                builder,
                carrier_step_phis,
                phi_bindings,
                GENERIC_LOOP_ERR,
            )?,
        )]),
        ASTNode::Return { value, .. } => parts::entry::lower_return_with_effects(
            builder,
            value.as_ref().map(|v| v.as_ref()),
            phi_bindings,
            GENERIC_LOOP_ERR,
        ),
        _ => Err(format!("{GENERIC_LOOP_ERR}: unsupported exit stmt")),
    }
}
