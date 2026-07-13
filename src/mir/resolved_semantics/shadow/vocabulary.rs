//! Exhaustive SA1 accepted syntax inventory and SA3-B1-C disposition.
//!
//! The disposition classifier is inventory-only. It deliberately does not
//! participate in statement or expression traversal, so adding a disposition
//! cannot widen resolver behavior.

use crate::ast::ASTNode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ShadowAstDispositionV0 {
    CurrentResolvedStatement,
    CurrentResolvedExpression,
    SemanticallyTransparentCandidate,
    ExplicitUnsupported,
}

/// Classifies every canonical AST variant without changing resolver behavior.
///
/// `SemanticallyTransparentCandidate` is not acceptance: a later slice must
/// prove and connect any traversal. `This` is intentionally inventoried as
/// unsupported even though the current expression resolver still accepts it;
/// removing that legacy acceptance is a pending correction.
pub(super) fn classify_shadow_ast_disposition_v0(node: &ASTNode) -> ShadowAstDispositionV0 {
    use ShadowAstDispositionV0::{
        CurrentResolvedExpression, CurrentResolvedStatement, ExplicitUnsupported,
        SemanticallyTransparentCandidate,
    };

    match node {
        ASTNode::Assignment { .. }
        | ASTNode::CompoundAssignment { .. }
        | ASTNode::If { .. }
        | ASTNode::Loop { .. }
        | ASTNode::Return { .. }
        | ASTNode::Break { .. }
        | ASTNode::Continue { .. }
        | ASTNode::Print { .. }
        | ASTNode::Nowait { .. }
        | ASTNode::Local { .. }
        | ASTNode::ScopeBox { .. }
        | ASTNode::TaskScope { .. }
        | ASTNode::FastMemRegion { .. }
        | ASTNode::Outbox { .. } => CurrentResolvedStatement,

        ASTNode::Literal { .. }
        | ASTNode::Variable { .. }
        | ASTNode::UnaryOp { .. }
        | ASTNode::BinaryOp { .. }
        | ASTNode::MethodCall { .. }
        | ASTNode::FieldAccess { .. }
        | ASTNode::Index { .. }
        | ASTNode::New { .. }
        | ASTNode::Me { .. }
        | ASTNode::FunctionCall { .. }
        | ASTNode::AwaitExpression { .. }
        | ASTNode::ArrayLiteral { .. }
        | ASTNode::MapLiteral { .. }
        | ASTNode::RecordLiteral { .. }
        | ASTNode::RecordUpdate { .. }
        | ASTNode::CheckExpr { .. }
        | ASTNode::FromCall { .. }
        | ASTNode::Call { .. }
        | ASTNode::GroupedAssignmentExpr { .. } => CurrentResolvedExpression,

        ASTNode::Program { .. }
        | ASTNode::UsingStatement { .. }
        | ASTNode::ImportStatement { .. }
        | ASTNode::StaticConstTable { .. } => SemanticallyTransparentCandidate,

        ASTNode::LoopRange { .. }
        | ASTNode::BuildGate { .. }
        | ASTNode::ContextScope { .. }
        | ASTNode::QMarkPropagate { .. }
        | ASTNode::MatchExpr { .. }
        | ASTNode::EnumMatchExpr { .. }
        | ASTNode::Lambda { .. }
        | ASTNode::BlockExpr { .. }
        | ASTNode::Arrow { .. }
        | ASTNode::TryCatch { .. }
        | ASTNode::Throw { .. }
        | ASTNode::BoxDeclaration { .. }
        | ASTNode::FunctionDeclaration { .. }
        | ASTNode::EnumDeclaration { .. }
        | ASTNode::BrandDeclaration { .. }
        | ASTNode::TypeAliasDeclaration { .. }
        | ASTNode::GlobalVar { .. }
        | ASTNode::This { .. }
        | ASTNode::ThisField { .. }
        | ASTNode::MeField { .. } => ExplicitUnsupported,
    }
}

pub(super) const SHADOW_ACCEPTED_STATEMENTS_V0: &[&str] = &[
    "Local",
    "Outbox",
    "Nowait",
    "Assignment",
    "CompoundAssignment",
    "ScopeBox",
    "TaskScope",
    "FastMemRegion",
    "If",
    "Loop",
    "Break",
    "Continue",
    "Return",
    "Print",
    "ClosedExpressionStatement",
];

pub(super) const SHADOW_ACCEPTED_EXPRESSIONS_V0: &[&str] = &[
    "Literal",
    "Variable",
    "Me",
    "UnaryOp",
    "BinaryOp",
    "MethodCall",
    "FieldAccess",
    "Index",
    "FunctionCall",
    "New",
    "AwaitExpression",
    "ArrayLiteral",
    "MapLiteral",
    "RecordLiteral",
    "RecordUpdate",
    "CheckExpr",
    "FromCall",
    "Call",
    "GroupedAssignmentExpr",
];

pub(super) const SHADOW_ACCEPTED_ASSIGNMENT_TARGETS_V0: &[&str] =
    &["Variable", "FieldAccess", "Index"];
