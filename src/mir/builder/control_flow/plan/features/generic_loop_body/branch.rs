use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::facts::expr_generic_loop::is_pure_value_expr_for_generic_loop;
use crate::mir::builder::control_flow::plan::parts::conditional_update;
use crate::mir::builder::control_flow::plan::steps::effects_to_plans;
use crate::mir::builder::control_flow::plan::LoweredRecipe;
use crate::mir::builder::MirBuilder;
use std::collections::BTreeMap;

use super::{body_plans_exit_on_all_paths, lower_body_stmt_v1, GENERIC_LOOP_ERR};
use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;
use crate::mir::builder::control_flow::plan::generic_loop::facts_types::GenericLoopV1Facts;
use crate::mir::builder::control_flow::plan::normalizer::PlanNormalizer;

pub(super) fn try_lower_blockexpr_loop_prelude_value(
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
        .any(|stmt| matches!(stmt, ASTNode::Loop { .. }))
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
        if body_plans_exit_on_all_paths(&plans) {
            break;
        }
    }

    let (tail_id, tail_effects) =
        PlanNormalizer::lower_value_ast(tail_expr.as_ref(), builder, &bindings)?;
    plans.extend(effects_to_plans(tail_effects));
    Ok(Some((tail_id, plans)))
}
pub(super) fn body_has_blockexpr_prelude_loop(body: &[ASTNode]) -> bool {
    body.iter().any(stmt_has_blockexpr_prelude_loop)
}

pub(super) fn stmt_has_blockexpr_prelude_loop(stmt: &ASTNode) -> bool {
    match stmt {
        ASTNode::Assignment { value, .. } => expr_has_blockexpr_prelude_loop(value),
        ASTNode::Local { initial_values, .. } => initial_values
            .iter()
            .flatten()
            .any(|v| expr_has_blockexpr_prelude_loop(v)),
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
        } => expr_has_blockexpr_prelude_loop(condition) || body_has_blockexpr_prelude_loop(body),
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

pub(super) fn expr_has_blockexpr_prelude_loop(expr: &ASTNode) -> bool {
    match expr {
        ASTNode::BlockExpr {
            prelude_stmts,
            tail_expr,
            ..
        } => {
            prelude_stmts
                .iter()
                .any(|stmt| matches!(stmt, ASTNode::Loop { .. }))
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

pub(super) fn try_lower_conditional_update_if(
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
    conditional_update::try_lower_conditional_update_if(
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

pub(super) fn can_attempt_conditional_update_branch(body: &[ASTNode]) -> bool {
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

pub(super) fn has_non_loop_assignment(body: &[ASTNode], loop_var: &str) -> bool {
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
