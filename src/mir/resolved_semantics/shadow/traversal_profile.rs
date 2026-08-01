//! Responsibility gates for the one shared shadow traversal.
//!
//! The Script profile is intentionally narrower than the Function/Lambda
//! profile. A rejected responsibility must be reported before any child is
//! traversed so RootLower remains the user-diagnostic authority.

use crate::ast::ASTNode;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ShadowTraversalProfileV1 {
    FullFunctionV1,
    ScriptLexicalCoreV1,
    ScriptLambdaLeafV1,
}
impl ShadowTraversalProfileV1 {
    pub(super) fn allows_statement(self, statement: &ASTNode) -> bool {
        match self {
            Self::FullFunctionV1 => true,
            Self::ScriptLexicalCoreV1 | Self::ScriptLambdaLeafV1 => match statement {
                ASTNode::Print { .. } => true,
                ASTNode::Nowait { .. } => true,
                ASTNode::ScopeBox { .. } => true,
                ASTNode::TaskScope { .. } => true,
                ASTNode::FastMemRegion { .. } => true,
                ASTNode::Outbox { .. } => true,
                ASTNode::Assignment { target, .. } | ASTNode::CompoundAssignment { target, .. } => {
                    matches!(target.as_ref(), ASTNode::Variable { .. })
                }
                ASTNode::Local {
                    variables,
                    initial_values,
                    declared_type_names,
                    ..
                } => {
                    variables.len() == 1
                        && initial_values.len() == 1
                        && (declared_type_names.is_empty()
                            || (declared_type_names.len() == 1 && declared_type_names[0].is_none()))
                }
                _ => self.allows_expression(statement),
            },
        }
    }

    pub(super) fn allows_expression(self, expression: &ASTNode) -> bool {
        match self {
            Self::FullFunctionV1 => true,
            Self::ScriptLexicalCoreV1 | Self::ScriptLambdaLeafV1 => match expression {
                ASTNode::Literal { .. } | ASTNode::Variable { .. } => true,
                ASTNode::Lambda { .. } => matches!(self, Self::ScriptLexicalCoreV1),
                ASTNode::UnaryOp { .. } => true,
                ASTNode::BinaryOp { .. }
                | ASTNode::AwaitExpression { .. }
                | ASTNode::CheckExpr { .. }
                | ASTNode::GroupedAssignmentExpr { .. }
                | ASTNode::ArrayLiteral { .. }
                | ASTNode::MapLiteral { .. }
                | ASTNode::RecordLiteral { .. }
                | ASTNode::EnumMatchExpr { .. } => true,
                ASTNode::FromCall { .. } => matches!(self, Self::ScriptLexicalCoreV1),
                ASTNode::BlockExpr {
                    prelude_stmts,
                    tail_expr,
                    ..
                } => {
                    prelude_stmts
                        .iter()
                        .all(|statement| self.allows_block_expr_prelude_statement(statement))
                        && self.is_block_expr_pure_expression(tail_expr)
                }
                _ => false,
            },
        }
    }

    fn allows_block_expr_prelude_statement(self, statement: &ASTNode) -> bool {
        match statement {
            ASTNode::Print { expression, .. } => self.is_block_expr_pure_expression(expression),
            expression => self.is_block_expr_pure_expression(expression),
        }
    }

    fn is_block_expr_pure_expression(self, expression: &ASTNode) -> bool {
        match expression {
            ASTNode::Literal { .. } => true,
            ASTNode::UnaryOp { operand, .. } => self.is_block_expr_pure_expression(operand),
            ASTNode::BinaryOp { left, right, .. } => {
                self.is_block_expr_pure_expression(left)
                    && self.is_block_expr_pure_expression(right)
            }
            ASTNode::AwaitExpression { expression, .. } => {
                self.is_block_expr_pure_expression(expression)
            }
            ASTNode::CheckExpr { items, .. } => items
                .iter()
                .all(|item| self.is_block_expr_pure_expression(&item.expression)),
            ASTNode::BlockExpr {
                prelude_stmts,
                tail_expr,
                ..
            } => {
                prelude_stmts
                    .iter()
                    .all(|statement| self.allows_block_expr_prelude_statement(statement))
                    && self.is_block_expr_pure_expression(tail_expr)
            }
            _ => false,
        }
    }
}
