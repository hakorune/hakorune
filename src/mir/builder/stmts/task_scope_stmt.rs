//! Structured task-scope lowering.
//!
//! `co { ... }` / compatibility `task_scope { ... }` are lexical ownership
//! boundaries. Runtime task-group truth remains in `runtime::global_hooks`;
//! MIRBuilder only places enter/exit calls.

use super::super::{MirBuilder, ValueId};
use crate::ast::ASTNode;

const EARLY_EXIT_TAG: &str = "[freeze:contract][co/early-exit-unsupported]";

pub(in crate::mir::builder) fn build_task_scope_statement(
    builder: &mut MirBuilder,
    body: Vec<ASTNode>,
    source_keyword: String,
) -> Result<ValueId, String> {
    if let Some(reason) = first_unsupported_early_exit(&body) {
        return Err(format!(
            "{} spelling={} reason={} CONC-CO-MIR-001 v0 is normal-completion-only; scope-exit cleanup lowering is owned by CONC-CO-MIR-002",
            EARLY_EXIT_TAG, source_keyword, reason
        ));
    }

    builder.emit_extern_call("env.task_scope", "push", Vec::new(), None)?;
    let result = builder.cf_block(body)?;
    builder.emit_extern_call("env.task_scope", "pop", Vec::new(), None)?;
    Ok(result)
}

fn first_unsupported_early_exit(statements: &[ASTNode]) -> Option<&'static str> {
    statements
        .iter()
        .find_map(|stmt| first_unsupported_early_exit_in_node(stmt, 0))
}

fn first_unsupported_early_exit_in_node(node: &ASTNode, loop_depth: usize) -> Option<&'static str> {
    match node {
        ASTNode::Return { .. } => Some("return"),
        ASTNode::Throw { .. } => Some("throw"),
        ASTNode::Break { .. } if loop_depth == 0 => Some("break"),
        ASTNode::Continue { .. } if loop_depth == 0 => Some("continue"),
        ASTNode::If {
            condition,
            then_body,
            else_body,
            ..
        } => first_unsupported_early_exit_in_node(condition, loop_depth)
            .or_else(|| first_unsupported_early_exit_in_block(then_body, loop_depth))
            .or_else(|| {
                else_body
                    .as_deref()
                    .and_then(|body| first_unsupported_early_exit_in_block(body, loop_depth))
            }),
        ASTNode::Loop {
            condition, body, ..
        } => first_unsupported_early_exit_in_node(condition, loop_depth)
            .or_else(|| first_unsupported_early_exit_in_block(body, loop_depth + 1)),
        ASTNode::TryCatch {
            try_body,
            catch_clauses,
            finally_body,
            ..
        } => first_unsupported_early_exit_in_block(try_body, loop_depth)
            .or_else(|| {
                catch_clauses.iter().find_map(|clause| {
                    first_unsupported_early_exit_in_block(&clause.body, loop_depth)
                })
            })
            .or_else(|| {
                finally_body
                    .as_deref()
                    .and_then(|body| first_unsupported_early_exit_in_block(body, loop_depth))
            }),
        ASTNode::TaskScope { body, .. }
        | ASTNode::ScopeBox { body, .. }
        | ASTNode::FastMemRegion { body, .. }
        | ASTNode::Program {
            statements: body, ..
        } => first_unsupported_early_exit_in_block(body, loop_depth),
        ASTNode::ContextScope { value, body, .. } => {
            first_unsupported_early_exit_in_node(value, loop_depth)
                .or_else(|| first_unsupported_early_exit_in_block(body, loop_depth))
        }
        ASTNode::Nowait { expression, .. }
        | ASTNode::AwaitExpression { expression, .. }
        | ASTNode::Print { expression, .. } => {
            first_unsupported_early_exit_in_node(expression, loop_depth)
        }
        ASTNode::Local { initial_values, .. } | ASTNode::Outbox { initial_values, .. } => {
            initial_values.iter().find_map(|value| {
                value
                    .as_deref()
                    .and_then(|node| first_unsupported_early_exit_in_node(node, loop_depth))
            })
        }
        ASTNode::Assignment { target, value, .. } => {
            first_unsupported_early_exit_in_node(target, loop_depth)
                .or_else(|| first_unsupported_early_exit_in_node(value, loop_depth))
        }
        ASTNode::BinaryOp { left, right, .. } => {
            first_unsupported_early_exit_in_node(left, loop_depth)
                .or_else(|| first_unsupported_early_exit_in_node(right, loop_depth))
        }
        ASTNode::UnaryOp { operand, .. }
        | ASTNode::QMarkPropagate {
            expression: operand,
            ..
        } => first_unsupported_early_exit_in_node(operand, loop_depth),
        ASTNode::MethodCall {
            object, arguments, ..
        } => first_unsupported_early_exit_in_node(object, loop_depth).or_else(|| {
            arguments
                .iter()
                .find_map(|arg| first_unsupported_early_exit_in_node(arg, loop_depth))
        }),
        ASTNode::FunctionCall { arguments, .. }
        | ASTNode::New { arguments, .. }
        | ASTNode::ArrayLiteral {
            elements: arguments,
            ..
        } => arguments
            .iter()
            .find_map(|arg| first_unsupported_early_exit_in_node(arg, loop_depth)),
        ASTNode::MapLiteral { entries, .. } => entries
            .iter()
            .find_map(|(_, value)| first_unsupported_early_exit_in_node(value, loop_depth)),
        ASTNode::Lambda { .. } => None,
        _ => None,
    }
}

fn first_unsupported_early_exit_in_block(
    statements: &[ASTNode],
    loop_depth: usize,
) -> Option<&'static str> {
    statements
        .iter()
        .find_map(|stmt| first_unsupported_early_exit_in_node(stmt, loop_depth))
}
