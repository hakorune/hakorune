//! Carrier variable collection helpers (analysis-only).

use crate::ast::ASTNode;
use std::collections::BTreeMap;

mod continue_only;

/// Carrier variable sets (SSOT entry point).
///
/// vars は collect_carrier_vars の返却順序を保持する（順序不変）。
/// 後続で BTreeSet が必要なら呼び出し側で変換する。
pub struct CarrierSets {
    pub vars: Vec<String>,
}

/// Collect carrier vars from AST body (wrapper for SSOT entry).
pub(in crate::mir::builder) fn collect_from_body(body: &[ASTNode]) -> CarrierSets {
    CarrierSets {
        vars: collect_carrier_vars(body),
    }
}

pub(in crate::mir::builder) fn collect_carrier_vars(body: &[ASTNode]) -> Vec<String> {
    let mut loop_locals = BTreeMap::<String, ()>::new();
    for stmt in body {
        collect_local_vars_from_stmt(stmt, &mut loop_locals);
    }

    let mut carriers = BTreeMap::<String, ()>::new();
    for stmt in body {
        collect_carrier_vars_from_stmt(stmt, &loop_locals, &mut carriers);
    }
    carriers.keys().cloned().collect()
}

fn collect_carrier_vars_from_stmt(
    stmt: &ASTNode,
    loop_locals: &BTreeMap<String, ()>,
    carriers: &mut BTreeMap<String, ()>,
) {
    match stmt {
        ASTNode::Program { statements, .. } => {
            for stmt in statements {
                collect_carrier_vars_from_stmt(stmt, loop_locals, carriers);
            }
        }
        ASTNode::Assignment { target, .. } => {
            let ASTNode::Variable { name, .. } = target.as_ref() else {
                return;
            };
            if !loop_locals.contains_key(name) {
                carriers.insert(name.clone(), ());
            }
        }
        ASTNode::Loop { body, .. } | ASTNode::LoopRange { body, .. } => {
            for stmt in body {
                collect_carrier_vars_from_stmt(stmt, loop_locals, carriers);
            }
        }
        ASTNode::If {
            then_body,
            else_body,
            ..
        } => {
            for stmt in then_body {
                collect_carrier_vars_from_stmt(stmt, loop_locals, carriers);
            }
            if let Some(else_body) = else_body {
                for stmt in else_body {
                    collect_carrier_vars_from_stmt(stmt, loop_locals, carriers);
                }
            }
        }
        ASTNode::ScopeBox { body, .. } => {
            for stmt in body {
                collect_carrier_vars_from_stmt(stmt, loop_locals, carriers);
            }
        }
        _ => {}
    }
}

fn collect_local_vars_from_stmt(stmt: &ASTNode, locals: &mut BTreeMap<String, ()>) {
    match stmt {
        ASTNode::Program { statements, .. } => {
            for stmt in statements {
                collect_local_vars_from_stmt(stmt, locals);
            }
        }
        ASTNode::Local { variables, .. } => {
            for name in variables {
                locals.insert(name.clone(), ());
            }
        }
        ASTNode::Loop { body, .. } | ASTNode::LoopRange { body, .. } => {
            for stmt in body {
                collect_local_vars_from_stmt(stmt, locals);
            }
        }
        ASTNode::If {
            then_body,
            else_body,
            ..
        } => {
            for stmt in then_body {
                collect_local_vars_from_stmt(stmt, locals);
            }
            if let Some(else_body) = else_body {
                for stmt in else_body {
                    collect_local_vars_from_stmt(stmt, locals);
                }
            }
        }
        ASTNode::ScopeBox { body, .. } => {
            for stmt in body {
                collect_local_vars_from_stmt(stmt, locals);
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};

    fn v(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.to_string(),
            span: Span::unknown(),
        }
    }

    fn lit(value: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: Span::unknown(),
        }
    }

    fn inc(name: &str) -> ASTNode {
        ASTNode::Assignment {
            target: Box::new(v(name)),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(v(name)),
                right: Box::new(lit(1)),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }
    }

    #[test]
    fn collect_from_body_keeps_continue_prelude_and_tail_carriers() {
        let body = vec![
            ASTNode::Local {
                variables: vec!["ch".to_string()],
                initial_values: vec![None],
                declared_type_names: Vec::new(),
                span: Span::unknown(),
            },
            ASTNode::If {
                condition: Box::new(v("is_sep")),
                then_body: vec![
                    inc("pos"),
                    ASTNode::Continue {
                        span: Span::unknown(),
                    },
                ],
                else_body: None,
                span: Span::unknown(),
            },
            inc("count"),
            inc("pos"),
        ];

        let carriers = collect_from_body(&body).vars;

        assert!(carriers.contains(&"pos".to_string()));
        assert!(carriers.contains(&"count".to_string()));
        assert!(!carriers.contains(&"ch".to_string()));
    }
}

// ============================================================================
// ContinueWithReturn recipe carrier collection (SSOT entry)
// ============================================================================

// ============================================================================
// ContinueOnlyStmtRecipe carrier collection (SSOT entry, T28 実装移設)
// ============================================================================
