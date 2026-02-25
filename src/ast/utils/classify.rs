use crate::ast::{ASTNode, ASTNodeType};

impl ASTNode {
    /// Structure/Expression/Statement の分類
    pub fn classify(&self) -> ASTNodeType {
        use ASTNodeType::{Expression as E, Statement as S, Structure as St};
        match self {
            ASTNode::BoxDeclaration { .. } => St,
            ASTNode::FunctionDeclaration { .. } => St,
            ASTNode::If { .. } => St,
            ASTNode::Loop { .. } => St,
            ASTNode::While { .. } => St,
            ASTNode::ForRange { .. } => St,
            ASTNode::TryCatch { .. } => St,
            ASTNode::ScopeBox { .. } => St,
            ASTNode::Literal { .. } => E,
            ASTNode::Variable { .. } => E,
            ASTNode::BinaryOp { .. } => E,
            ASTNode::UnaryOp { .. } => E,
            ASTNode::FunctionCall { .. } => E,
            ASTNode::Call { .. } => E,
            ASTNode::MethodCall { .. } => E,
            ASTNode::FieldAccess { .. } => E,
            ASTNode::New { .. } => E,
            ASTNode::This { .. } => E,
            ASTNode::Me { .. } => E,
            ASTNode::FromCall { .. } => E,
            ASTNode::ThisField { .. } => E,
            ASTNode::MeField { .. } => E,
            ASTNode::Index { .. } => E,
            ASTNode::MatchExpr { .. } => E,
            ASTNode::QMarkPropagate { .. } => E,
            ASTNode::Lambda { .. } => E,
            ASTNode::ArrayLiteral { .. } => E,
            ASTNode::MapLiteral { .. } => E,
            ASTNode::BlockExpr { .. } => E,
            ASTNode::AwaitExpression { .. } => E,
            ASTNode::GroupedAssignmentExpr { .. } => E,
            ASTNode::Program { .. } => S,
            ASTNode::Assignment { .. } => S,
            ASTNode::Print { .. } => S,
            ASTNode::Return { .. } => S,
            ASTNode::Break { .. } => S,
            ASTNode::Continue { .. } => S,
            ASTNode::UsingStatement { .. } => S,
            ASTNode::ImportStatement { .. } => S,
            ASTNode::GlobalVar { .. } => S,
            ASTNode::Local { .. } => S,
            ASTNode::Outbox { .. } => S,
            ASTNode::Nowait { .. } => S,
            ASTNode::Arrow { .. } => S,
            ASTNode::Throw { .. } => S,
        }
    }

    pub fn is_structure(&self) -> bool {
        matches!(self.classify(), ASTNodeType::Structure)
    }

    pub fn is_expression(&self) -> bool {
        matches!(self.classify(), ASTNodeType::Expression)
    }

    pub fn is_statement(&self) -> bool {
        matches!(self.classify(), ASTNodeType::Statement)
    }
}
