//! Shared helpers for the loop_cond_break_continue family.
//!
//! The former `lower_loop_cond_break_continue` oracle pipeline entry was
//! retired with the oracle-only chain; what remains here are the carrier /
//! accept-kind helpers still used by the live source lowerers.

use crate::mir::builder::control_flow::facts::loop_cond_break_continue::LoopCondBreakAcceptKind;
use crate::mir::builder::MirBuilder;
use std::collections::{BTreeMap, BTreeSet};

pub(super) const LOOP_COND_ERR: &str = "[normalizer] loop_cond_break_continue";

/// Facts->Lower contract pin: every issued accept kind must have a lowering
/// arm in both the raw and the located-source consumers. Keep exhaustive.
pub(super) const fn pin_accept_kind_contract(kind: LoopCondBreakAcceptKind) {
    match kind {
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
        LoopCondBreakAcceptKind::NoExitBody => (),
    }
}

pub(super) fn extend_unique_carriers(carrier_vars: &mut Vec<String>, more: Vec<String>) {
    for name in more {
        if !carrier_vars.iter().any(|existing| existing == &name) {
            carrier_vars.push(name);
        }
    }
}

pub(super) fn collect_carrier_vars_from_condition(
    builder: &MirBuilder,
    condition: &crate::ast::ASTNode,
) -> Vec<String> {
    let mut vars = BTreeSet::<String>::new();
    collect_vars_from_expr(condition, &mut vars);

    let mut carriers = BTreeMap::<String, ()>::new();
    for name in vars {
        if builder
            .function_state
            .variable_ctx
            .variable_map
            .contains_key(&name)
        {
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
        ASTNode::New {
            arguments,
            field_initializers,
            ..
        } => {
            for arg in arguments {
                collect_vars_from_expr(arg, vars);
            }
            for (_, initializer) in field_initializers {
                collect_vars_from_expr(initializer, vars);
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
