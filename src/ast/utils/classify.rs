use crate::ast::ASTNode;

impl ASTNode {
    /// True for AST nodes that can appear as expression values.
    pub fn is_expression(&self) -> bool {
        matches!(
            self,
            ASTNode::Literal { .. }
                | ASTNode::Variable { .. }
                | ASTNode::BinaryOp { .. }
                | ASTNode::CheckExpr { .. }
                | ASTNode::UnaryOp { .. }
                | ASTNode::FunctionCall { .. }
                | ASTNode::Call { .. }
                | ASTNode::MethodCall { .. }
                | ASTNode::FieldAccess { .. }
                | ASTNode::New { .. }
                | ASTNode::This { .. }
                | ASTNode::Me { .. }
                | ASTNode::FromCall { .. }
                | ASTNode::ThisField { .. }
                | ASTNode::MeField { .. }
                | ASTNode::Index { .. }
                | ASTNode::MatchExpr { .. }
                | ASTNode::EnumMatchExpr { .. }
                | ASTNode::QMarkPropagate { .. }
                | ASTNode::Lambda { .. }
                | ASTNode::ArrayLiteral { .. }
                | ASTNode::MapLiteral { .. }
                | ASTNode::RecordLiteral { .. }
                | ASTNode::RecordUpdate { .. }
                | ASTNode::BlockExpr { .. }
                | ASTNode::AwaitExpression { .. }
                | ASTNode::GroupedAssignmentExpr { .. }
        )
    }
}
