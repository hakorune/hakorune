use super::{parts_exit, CondUpdateBranch};
use crate::ast::{ASTNode, BinaryOperator};
use crate::mir::builder::control_flow::plan::normalizer::loop_body_lowering;
use crate::mir::builder::control_flow::plan::{CoreEffectPlan, CoreExitPlan};
use crate::mir::builder::MirBuilder;
use std::collections::BTreeMap;

pub(in crate::mir::builder) fn attach_phi_args_if_continue_or_break(
    builder: &MirBuilder,
    exit: CoreExitPlan,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: Option<&BTreeMap<String, crate::mir::ValueId>>,
    current_bindings: &BTreeMap<String, crate::mir::ValueId>,
    error_prefix: &str,
) -> Result<CoreExitPlan, String> {
    match exit {
        CoreExitPlan::Continue(depth) => {
            if depth != 1 {
                return Err(format!(
                    "[freeze:contract][exit_depth] {error_prefix}: continue depth must be 1"
                ));
            }
            if carrier_step_phis.is_empty() {
                return Ok(CoreExitPlan::Continue(depth));
            }
            parts_exit::build_continue_with_phi_args(
                builder,
                carrier_step_phis,
                current_bindings,
                error_prefix,
            )
        }
        CoreExitPlan::Break(depth) => {
            let Some(break_phi_dsts) = break_phi_dsts else {
                return Ok(CoreExitPlan::Break(depth));
            };
            if depth != 1 {
                return Err(format!(
                    "[freeze:contract][exit_depth] {error_prefix}: break depth must be 1"
                ));
            }
            parts_exit::build_break_with_phi_args(break_phi_dsts, current_bindings, error_prefix)
        }
        other => Ok(other),
    }
}

pub(in crate::mir::builder) fn collect_conditional_update_branch(
    builder: &mut MirBuilder,
    current_bindings: &BTreeMap<String, crate::mir::ValueId>,
    body: &[ASTNode],
    error_prefix: &str,
) -> Result<CondUpdateBranch, String> {
    let mut updates = BTreeMap::new();
    let mut effects = Vec::new();
    let mut exit = None;
    let mut saw_assignment = false;

    for (idx, stmt) in body.iter().enumerate() {
        let is_last = idx + 1 == body.len();
        if collect_conditional_update_assignment_or_local(
            builder,
            current_bindings,
            stmt,
            &mut updates,
            &mut effects,
            &mut saw_assignment,
            error_prefix,
        )? {
            continue;
        }
        match stmt {
            ASTNode::Break { .. } => {
                if !is_last || exit.is_some() {
                    return Err(format!("{error_prefix}: break not at tail"));
                }
                exit = Some(CoreExitPlan::Break(1));
            }
            ASTNode::Continue { .. } => {
                if !is_last || exit.is_some() {
                    return Err(format!("{error_prefix}: continue not at tail"));
                }
                exit = Some(CoreExitPlan::Continue(1));
            }
            _ => {
                return Err(format!(
                    "{error_prefix}: conditional update has unsupported stmt"
                ));
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

pub(in crate::mir::builder) fn collect_conditional_update_assignment_or_local(
    builder: &mut MirBuilder,
    current_bindings: &BTreeMap<String, crate::mir::ValueId>,
    stmt: &ASTNode,
    updates: &mut BTreeMap<String, crate::mir::ValueId>,
    effects: &mut Vec<CoreEffectPlan>,
    saw_assignment: &mut bool,
    error_prefix: &str,
) -> Result<bool, String> {
    match stmt {
        ASTNode::Assignment { target, value, .. } => {
            let ASTNode::Variable { .. } = target.as_ref() else {
                return Err(format!("{error_prefix}: conditional update target"));
            };
            if !is_pure_value_expr(value) {
                return Err(format!("{error_prefix}: conditional update not pure"));
            }
            let (var, value_id, mut new_effects) = loop_body_lowering::lower_assignment_value(
                builder,
                current_bindings,
                target,
                value,
                error_prefix,
            )?;
            if updates.contains_key(&var) {
                return Err(format!("{error_prefix}: duplicate update for {}", var));
            }
            effects.append(&mut new_effects);
            updates.insert(var, value_id);
            *saw_assignment = true;
            Ok(true)
        }
        ASTNode::Local {
            variables,
            initial_values,
            ..
        } => {
            if variables.len() != initial_values.len() {
                return Err(format!("{error_prefix}: local init arity mismatch"));
            }
            for (name, init) in variables.iter().zip(initial_values.iter()) {
                let init_node = loop_body_lowering::local_init_node_or_null(init.as_ref());
                if !is_pure_value_expr(init_node.as_ref()) {
                    return Err(format!("{error_prefix}: local init not pure"));
                }
                if !current_bindings.contains_key(name)
                    && !builder.variable_ctx.variable_map.contains_key(name)
                {
                    return Err(format!(
                        "{error_prefix}: local init requires predeclared {}",
                        name
                    ));
                }
            }
            let (inits, mut init_effects) = loop_body_lowering::lower_local_init_values(
                builder,
                current_bindings,
                variables,
                initial_values,
                error_prefix,
            )?;
            effects.append(&mut init_effects);
            for (name, value_id) in inits {
                if updates.contains_key(&name) {
                    return Err(format!("{error_prefix}: duplicate update for {}", name));
                }
                updates.insert(name, value_id);
                *saw_assignment = true;
            }
            Ok(true)
        }
        _ => Ok(false),
    }
}

pub(in crate::mir::builder) fn current_value_for_join(
    builder: &MirBuilder,
    current_bindings: &BTreeMap<String, crate::mir::ValueId>,
    name: &str,
    error_prefix: &str,
) -> Result<crate::mir::ValueId, String> {
    if let Some(value) = current_bindings.get(name) {
        return Ok(*value);
    }
    if let Some(value) = builder.variable_ctx.variable_map.get(name) {
        return Ok(*value);
    }
    Err(format!("{error_prefix}: join value {} not found", name))
}

pub(super) fn has_any_assignment(body: &[ASTNode]) -> bool {
    body.iter().any(|stmt| match stmt {
        ASTNode::Assignment { .. } => true,
        ASTNode::Local { initial_values, .. } => initial_values.iter().all(|init| init.is_some()),
        _ => false,
    })
}

pub(super) fn is_conditional_update_branch_supported(body: &[ASTNode]) -> bool {
    let mut saw_exit = false;
    for (idx, stmt) in body.iter().enumerate() {
        let is_last = idx + 1 == body.len();
        match stmt {
            ASTNode::Assignment { target, value, .. } => {
                if !matches!(target.as_ref(), ASTNode::Variable { .. }) {
                    return false;
                }
                if !is_pure_value_expr(value) {
                    return false;
                }
            }
            ASTNode::Local {
                variables,
                initial_values,
                ..
            } => {
                if variables.len() != initial_values.len() {
                    return false;
                }
                for init in initial_values {
                    let Some(init) = init.as_ref() else {
                        return false;
                    };
                    if !is_pure_value_expr(init) {
                        return false;
                    }
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

pub(super) fn is_pure_value_expr(ast: &ASTNode) -> bool {
    match ast {
        ASTNode::Variable { .. } => true,
        ASTNode::Literal { .. } => true,
        ASTNode::BlockExpr {
            prelude_stmts,
            tail_expr,
            ..
        } => prelude_stmts.is_empty() && is_pure_value_expr(tail_expr),
        ASTNode::UnaryOp { operand, .. } => is_pure_value_expr(operand),
        ASTNode::BinaryOp {
            operator,
            left,
            right,
            ..
        } => {
            matches!(
                operator,
                BinaryOperator::Add
                    | BinaryOperator::Subtract
                    | BinaryOperator::Multiply
                    | BinaryOperator::Divide
                    | BinaryOperator::Modulo
                    | BinaryOperator::Less
                    | BinaryOperator::LessEqual
                    | BinaryOperator::Greater
                    | BinaryOperator::GreaterEqual
                    | BinaryOperator::Equal
                    | BinaryOperator::NotEqual
            ) && is_pure_value_expr(left)
                && is_pure_value_expr(right)
        }
        _ => false,
    }
}
