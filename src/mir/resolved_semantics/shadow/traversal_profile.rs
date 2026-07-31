//! Responsibility gates for the one shared shadow traversal.
//!
//! The Script profile is intentionally narrower than the Function/Lambda
//! profile. A rejected responsibility must be reported before any child is
//! traversed so RootLower remains the user-diagnostic authority.

use crate::ast::{ASTNode, UnaryOperator};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum ShadowTraversalProfileV1 {
    FullFunctionV1,
    ScriptLexicalCoreV1,
}

impl ShadowTraversalProfileV1 {
    pub(super) fn allows_statement(self, statement: &ASTNode) -> bool {
        match self {
            Self::FullFunctionV1 => true,
            Self::ScriptLexicalCoreV1 => match statement {
                ASTNode::Print { .. } => true,
                ASTNode::ScopeBox { .. } => true,
                ASTNode::FastMemRegion { .. } => true,
                ASTNode::Local {
                    variables,
                    initial_values,
                    declared_type_names,
                    ..
                } => {
                    variables.len() == 1
                        && initial_values.len() == 1
                        && (declared_type_names.is_empty()
                            || (declared_type_names.len() == 1
                                && declared_type_names[0].is_none()))
                }
                _ => self.allows_expression(statement),
            },
        }
    }

    pub(super) fn allows_expression(self, expression: &ASTNode) -> bool {
        match self {
            Self::FullFunctionV1 => true,
            Self::ScriptLexicalCoreV1 => match expression {
                ASTNode::Literal { .. } | ASTNode::Variable { .. } => true,
                ASTNode::UnaryOp { operator, .. } => *operator != UnaryOperator::Weak,
                ASTNode::BinaryOp { .. }
                | ASTNode::AwaitExpression { .. }
                | ASTNode::CheckExpr { .. } => true,
                _ => false,
            },
        }
    }
}
