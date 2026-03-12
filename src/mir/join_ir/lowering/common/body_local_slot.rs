//! Phase 92 P3: ReadOnlyBodyLocalSlot Box
//!
//! Purpose: support the minimal case where a loop condition/break condition
//! references a loop-body-local variable (e.g., `ch`) that is recomputed every
//! iteration and is read-only (no assignment).
//!
//! This box is intentionally narrow and fail-fast:
//! - Supports exactly 1 body-local variable used in conditions.
//! - Requires a top-level `local <name> = <init_expr>` before the break-guard `if`.
//! - Forbids any assignment to that variable (including in nested blocks).
//!
//! NOTE: This box does NOT lower the init expression itself.
//! Lowering is handled by `LoopBodyLocalInitLowerer` (Phase 186).
//! This box only validates the contract and provides an allow-list for
//! condition lowering checks.

use crate::ast::ASTNode;
use crate::mir::join_ir::lowering::error_tags;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ReadOnlyBodyLocalSlot {
    pub name: String,
    pub init_expr: ASTNode,
    pub decl_stmt_index: usize,
    pub break_guard_stmt_index: usize,
}

/// A tiny "box" API: analyze loop body and decide whether we can allow a single
/// loop-body-local variable to be referenced from loop_break conditions.
pub struct ReadOnlyBodyLocalSlotBox;

impl ReadOnlyBodyLocalSlotBox {
    /// Extract and validate a single read-only body-local slot used in conditions.
    ///
    /// # Contract (Fail-Fast)
    /// - `names_in_conditions` must contain exactly 1 name.
    /// - A top-level `local <name> = <expr>` must exist in `body`.
    /// - The declaration statement must appear before the first top-level `if` that contains `break`.
    /// - No assignment to `<name>` may exist anywhere in the loop body (including nested statements).
    pub fn extract_single(
        names_in_conditions: &[String],
        body: &[ASTNode],
    ) -> Result<ReadOnlyBodyLocalSlot, String> {
        if names_in_conditions.is_empty() {
            return Err(error_tags::freeze(
                "[loop_break/body_local_slot/internal/empty_names] extract_single called with empty names_in_conditions",
            ));
        }
        if names_in_conditions.len() != 1 {
            return Err(error_tags::freeze(&format!(
                "[loop_break/body_local_slot/contract/multiple_vars] Unsupported: multiple LoopBodyLocal variables in condition: {:?}",
                names_in_conditions
            )));
        }

        let name = names_in_conditions[0].clone();

        let break_guard_stmt_index = find_first_top_level_break_guard_if(body).ok_or_else(|| {
            error_tags::freeze(
                "[loop_break/body_local_slot/contract/missing_break_guard] Missing top-level `if (...) { break }` (loop_break guard)",
            )
        })?;

        let (decl_stmt_index, init_expr) =
            find_top_level_local_init(body, &name).ok_or_else(|| {
                error_tags::freeze(&format!(
                    "[loop_break/body_local_slot/contract/missing_local_init] Missing top-level `local {} = <expr>` for LoopBodyLocal used in condition",
                    name
                ))
            })?;

        if decl_stmt_index >= break_guard_stmt_index {
            return Err(error_tags::freeze(&format!(
                "[loop_break/body_local_slot/contract/decl_after_break_guard] `local {}` must appear before the break guard if-statement (decl_index={}, break_if_index={})",
                name, decl_stmt_index, break_guard_stmt_index
            )));
        }

        if contains_assignment_to_name(body, &name) {
            return Err(error_tags::freeze(&format!(
                "[loop_break/body_local_slot/contract/not_readonly] `{}` must be read-only (assignment detected in loop body)",
                name
            )));
        }

        Ok(ReadOnlyBodyLocalSlot {
            name,
            init_expr,
            decl_stmt_index,
            break_guard_stmt_index,
        })
    }
}

fn find_first_top_level_break_guard_if(body: &[ASTNode]) -> Option<usize> {
    for (idx, stmt) in body.iter().enumerate() {
        if let ASTNode::If {
            then_body,
            else_body,
            ..
        } = stmt
        {
            if then_body.iter().any(|n| matches!(n, ASTNode::Break { .. })) {
                return Some(idx);
            }
            if let Some(else_body) = else_body {
                if else_body.iter().any(|n| matches!(n, ASTNode::Break { .. })) {
                    return Some(idx);
                }
            }
        }
    }
    None
}

fn find_top_level_local_init(body: &[ASTNode], name: &str) -> Option<(usize, ASTNode)> {
    for (idx, stmt) in body.iter().enumerate() {
        if let ASTNode::Local {
            variables,
            initial_values,
            ..
        } = stmt
        {
            // Keep Phase 92 P3 minimal: the statement must be a 1-variable local.
            if variables.len() != 1 {
                continue;
            }
            if variables[0] != name {
                continue;
            }
            let init = initial_values
                .get(0)
                .and_then(|v| v.as_ref())
                .map(|b| (*b.clone()).clone())?;
            return Some((idx, init));
        }
    }
    None
}

fn contains_assignment_to_name(body: &[ASTNode], name: &str) -> bool {
    body.iter()
        .any(|stmt| contains_assignment_to_name_in_node(stmt, name))
}

fn contains_assignment_to_name_in_node(node: &ASTNode, name: &str) -> bool {
    match node {
        ASTNode::Assignment { target, value, .. } => {
            if matches!(&**target, ASTNode::Variable { name: n, .. } if n == name) {
                return true;
            }
            contains_assignment_to_name_in_node(target, name)
                || contains_assignment_to_name_in_node(value, name)
        }
        ASTNode::Nowait { variable, .. } => variable == name,
        ASTNode::If {
            condition,
            then_body,
            else_body,
            ..
        } => {
            contains_assignment_to_name_in_node(condition, name)
                || then_body
                    .iter()
                    .any(|n| contains_assignment_to_name_in_node(n, name))
                || else_body.as_ref().is_some_and(|e| {
                    e.iter()
                        .any(|n| contains_assignment_to_name_in_node(n, name))
                })
        }
        ASTNode::Loop {
            condition, body, ..
        } => {
            contains_assignment_to_name_in_node(condition, name)
                || body
                    .iter()
                    .any(|n| contains_assignment_to_name_in_node(n, name))
        }
        ASTNode::While {
            condition, body, ..
        } => {
            contains_assignment_to_name_in_node(condition, name)
                || body
                    .iter()
                    .any(|n| contains_assignment_to_name_in_node(n, name))
        }
        ASTNode::ForRange { body, .. } => body
            .iter()
            .any(|n| contains_assignment_to_name_in_node(n, name)),
        ASTNode::TryCatch {
            try_body,
            catch_clauses,
            finally_body,
            ..
        } => {
            try_body
                .iter()
                .any(|n| contains_assignment_to_name_in_node(n, name))
                || catch_clauses.iter().any(|c| {
                    c.body
                        .iter()
                        .any(|n| contains_assignment_to_name_in_node(n, name))
                })
                || finally_body.as_ref().is_some_and(|b| {
                    b.iter()
                        .any(|n| contains_assignment_to_name_in_node(n, name))
                })
        }
        ASTNode::ScopeBox { body, .. } => body
            .iter()
            .any(|n| contains_assignment_to_name_in_node(n, name)),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{BinaryOperator, LiteralValue, Span};

    fn span() -> Span {
        Span::unknown()
    }

    fn var(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.to_string(),
            span: span(),
        }
    }

    fn lit_i(value: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: span(),
        }
    }

    fn bin(op: BinaryOperator, left: ASTNode, right: ASTNode) -> ASTNode {
        ASTNode::BinaryOp {
            operator: op,
            left: Box::new(left),
            right: Box::new(right),
            span: span(),
        }
    }

    #[test]
    fn extract_single_ok() {
        // local ch = 0; if (ch < 1) { break }
        let body = vec![
            ASTNode::Local {
                variables: vec!["ch".to_string()],
                initial_values: vec![Some(Box::new(lit_i(0)))],
                span: span(),
            },
            ASTNode::If {
                condition: Box::new(bin(BinaryOperator::Less, var("ch"), lit_i(1))),
                then_body: vec![ASTNode::Break { span: span() }],
                else_body: None,
                span: span(),
            },
        ];

        let slot = ReadOnlyBodyLocalSlotBox::extract_single(&[String::from("ch")], &body).unwrap();
        assert_eq!(slot.name, "ch");
        assert_eq!(slot.decl_stmt_index, 0);
        assert_eq!(slot.break_guard_stmt_index, 1);
    }

    #[test]
    fn extract_single_reject_assignment() {
        let body = vec![
            ASTNode::Local {
                variables: vec!["ch".to_string()],
                initial_values: vec![Some(Box::new(lit_i(0)))],
                span: span(),
            },
            ASTNode::Assignment {
                target: Box::new(var("ch")),
                value: Box::new(lit_i(1)),
                span: span(),
            },
            ASTNode::If {
                condition: Box::new(bin(BinaryOperator::Less, var("ch"), lit_i(1))),
                then_body: vec![ASTNode::Break { span: span() }],
                else_body: None,
                span: span(),
            },
        ];

        let err =
            ReadOnlyBodyLocalSlotBox::extract_single(&[String::from("ch")], &body).unwrap_err();
        assert!(err.contains("[joinir/freeze]"));
        assert!(err.contains("read-only"));
    }

    #[test]
    fn extract_single_reject_decl_after_break_if() {
        let body = vec![
            ASTNode::If {
                condition: Box::new(bin(BinaryOperator::Less, var("ch"), lit_i(1))),
                then_body: vec![ASTNode::Break { span: span() }],
                else_body: None,
                span: span(),
            },
            ASTNode::Local {
                variables: vec!["ch".to_string()],
                initial_values: vec![Some(Box::new(lit_i(0)))],
                span: span(),
            },
        ];

        let err =
            ReadOnlyBodyLocalSlotBox::extract_single(&[String::from("ch")], &body).unwrap_err();
        assert!(err.contains("[joinir/freeze]"));
        assert!(err.contains("must appear before"));
    }
}
