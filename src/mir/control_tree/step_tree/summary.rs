use crate::ast::ASTNode;

use super::types::AstSummary;

pub(super) fn summarize_ast(ast: &ASTNode) -> AstSummary {
    match ast {
        ASTNode::Variable { name, .. } => AstSummary::Variable(name.clone()),
        ASTNode::Literal { value, .. } => AstSummary::Literal(value.clone()),
        ASTNode::UnaryOp {
            operator, operand, ..
        } => AstSummary::Unary {
            op: operator.clone(),
            expr: Box::new(summarize_ast(operand)),
        },
        ASTNode::BinaryOp {
            operator,
            left,
            right,
            ..
        } => AstSummary::Binary {
            op: operator.clone(),
            lhs: Box::new(summarize_ast(left)),
            rhs: Box::new(summarize_ast(right)),
        },
        other => AstSummary::Other(ast_kind_name(other)),
    }
}

pub(super) fn ast_kind_name(ast: &ASTNode) -> &'static str {
    match ast {
        ASTNode::Program { .. } => "Program",
        ASTNode::Assignment { .. } => "Assignment",
        ASTNode::Print { .. } => "Print",
        ASTNode::If { .. } => "If",
        ASTNode::Loop { .. } => "Loop",
        ASTNode::While { .. } => "While",
        ASTNode::ForRange { .. } => "ForRange",
        ASTNode::Return { .. } => "Return",
        ASTNode::Break { .. } => "Break",
        ASTNode::Continue { .. } => "Continue",
        ASTNode::UsingStatement { .. } => "UsingStatement",
        ASTNode::ImportStatement { .. } => "ImportStatement",
        ASTNode::Nowait { .. } => "Nowait",
        ASTNode::AwaitExpression { .. } => "AwaitExpression",
        ASTNode::QMarkPropagate { .. } => "QMarkPropagate",
        ASTNode::MatchExpr { .. } => "MatchExpr",
        ASTNode::ArrayLiteral { .. } => "ArrayLiteral",
        ASTNode::MapLiteral { .. } => "MapLiteral",
        ASTNode::Lambda { .. } => "Lambda",
        ASTNode::BlockExpr { .. } => "BlockExpr",
        ASTNode::Arrow { .. } => "Arrow",
        ASTNode::TryCatch { .. } => "TryCatch",
        ASTNode::Throw { .. } => "Throw",
        ASTNode::BoxDeclaration { .. } => "BoxDeclaration",
        ASTNode::FunctionDeclaration { .. } => "FunctionDeclaration",
        ASTNode::GlobalVar { .. } => "GlobalVar",
        ASTNode::Literal { .. } => "Literal",
        ASTNode::Variable { .. } => "Variable",
        ASTNode::UnaryOp { .. } => "UnaryOp",
        ASTNode::BinaryOp { .. } => "BinaryOp",
        ASTNode::GroupedAssignmentExpr { .. } => "GroupedAssignmentExpr",
        ASTNode::MethodCall { .. } => "MethodCall",
        ASTNode::Call { .. } => "Call",
        ASTNode::FunctionCall { .. } => "FunctionCall",
        ASTNode::FieldAccess { .. } => "FieldAccess",
        ASTNode::Index { .. } => "Index",
        ASTNode::New { .. } => "New",
        ASTNode::This { .. } => "This",
        ASTNode::Me { .. } => "Me",
        ASTNode::FromCall { .. } => "FromCall",
        ASTNode::ThisField { .. } => "ThisField",
        ASTNode::MeField { .. } => "MeField",
        ASTNode::Local { .. } => "Local",
        ASTNode::ScopeBox { .. } => "ScopeBox",
        ASTNode::Outbox { .. } => "Outbox",
    }
}
