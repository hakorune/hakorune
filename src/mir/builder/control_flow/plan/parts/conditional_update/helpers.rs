use super::parts_exit;
use crate::ast::{ASTNode, BinaryOperator};
use crate::mir::builder::control_flow::plan::CoreExitPlan;
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

pub(in crate::mir::builder) fn current_value_for_join(
    builder: &MirBuilder,
    current_bindings: &BTreeMap<String, crate::mir::ValueId>,
    name: &str,
    error_prefix: &str,
) -> Result<crate::mir::ValueId, String> {
    if let Some(value) = current_bindings.get(name) {
        return Ok(*value);
    }
    if let Some(value) = builder.function_state.variable_ctx.variable_map.get(name) {
        return Ok(*value);
    }
    Err(format!("{error_prefix}: join value {} not found", name))
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
