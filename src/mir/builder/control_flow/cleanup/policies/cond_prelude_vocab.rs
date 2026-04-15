//! Condition prelude vocabulary SSOT.
//!
//! Purpose: keep the accepted statement vocabulary for `CondBlockView::prelude_stmts`
//! consistent across:
//! - Facts (accept-shape checks)
//! - Normalizer/lowering (actual effect lowering)
//!
//! Contract (v1):
//! - Prelude statements must be "stmt-only effects" (no control-flow exits).
//! - Exit checks are done separately via `ASTNode::contains_non_local_exit()`.

use crate::ast::ASTNode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum CondPreludeStmtKind {
    Local,
    Assignment,
    If,
    Loop,
    MethodCall,
    FunctionCall,
    Print,
}

pub(in crate::mir::builder) fn classify_cond_prelude_stmt(
    stmt: &ASTNode,
) -> Option<CondPreludeStmtKind> {
    match stmt {
        ASTNode::Local { .. } => Some(CondPreludeStmtKind::Local),
        ASTNode::Assignment { .. } => Some(CondPreludeStmtKind::Assignment),
        ASTNode::If { .. } => Some(CondPreludeStmtKind::If),
        ASTNode::Loop { .. } | ASTNode::While { .. } | ASTNode::ForRange { .. } => {
            Some(CondPreludeStmtKind::Loop)
        }
        ASTNode::MethodCall { .. } => Some(CondPreludeStmtKind::MethodCall),
        ASTNode::FunctionCall { .. } => Some(CondPreludeStmtKind::FunctionCall),
        ASTNode::Print { .. } => Some(CondPreludeStmtKind::Print),
        _ => None,
    }
}

pub(in crate::mir::builder) fn stmt_has_loop_like_stmt(stmt: &ASTNode) -> bool {
    match stmt {
        ASTNode::Loop { .. } | ASTNode::While { .. } | ASTNode::ForRange { .. } => true,
        ASTNode::If {
            then_body,
            else_body,
            ..
        } => {
            then_body.iter().any(stmt_has_loop_like_stmt)
                || else_body
                    .as_ref()
                    .is_some_and(|body| body.iter().any(stmt_has_loop_like_stmt))
        }
        ASTNode::Program { statements, .. } => statements.iter().any(stmt_has_loop_like_stmt),
        ASTNode::ScopeBox { body, .. } => body.iter().any(stmt_has_loop_like_stmt),
        _ => false,
    }
}

pub(in crate::mir::builder) fn prelude_has_loop_like_stmt(stmts: &[ASTNode]) -> bool {
    stmts.iter().any(stmt_has_loop_like_stmt)
}

#[cfg(test)]
mod tests {
    use super::{classify_cond_prelude_stmt, prelude_has_loop_like_stmt, CondPreludeStmtKind};
    use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};

    fn var(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.to_string(),
            span: Span::unknown(),
        }
    }

    fn lit_i(n: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(n),
            span: Span::unknown(),
        }
    }

    fn bin_lt(lhs: ASTNode, rhs: ASTNode) -> ASTNode {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(lhs),
            right: Box::new(rhs),
            span: Span::unknown(),
        }
    }

    #[test]
    fn cond_prelude_vocab_accepts_if_and_loop_like_stmt() {
        let if_stmt = ASTNode::If {
            condition: Box::new(ASTNode::Literal {
                value: LiteralValue::Bool(true),
                span: Span::unknown(),
            }),
            then_body: vec![],
            else_body: None,
            span: Span::unknown(),
        };
        let loop_stmt = ASTNode::Loop {
            condition: Box::new(bin_lt(var("i"), lit_i(1))),
            body: vec![],
            span: Span::unknown(),
        };
        assert_eq!(
            classify_cond_prelude_stmt(&if_stmt),
            Some(CondPreludeStmtKind::If)
        );
        assert_eq!(
            classify_cond_prelude_stmt(&loop_stmt),
            Some(CondPreludeStmtKind::Loop)
        );
    }

    #[test]
    fn prelude_loop_detection_recurses_into_if_branches() {
        let nested_loop = ASTNode::Loop {
            condition: Box::new(bin_lt(var("j"), lit_i(1))),
            body: vec![],
            span: Span::unknown(),
        };
        let prelude = vec![ASTNode::If {
            condition: Box::new(ASTNode::Literal {
                value: LiteralValue::Bool(true),
                span: Span::unknown(),
            }),
            then_body: vec![nested_loop],
            else_body: None,
            span: Span::unknown(),
        }];
        assert!(prelude_has_loop_like_stmt(&prelude));
    }
}
