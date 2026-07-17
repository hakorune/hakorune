//! One neutral source-child role to structural-path policy.
//!
//! Resolver traversal, compiler located views, and future located legacy
//! lowering must consume this vocabulary. None may keep a second
//! AST-shape-to-`SourcePathSegmentV1` decision table.

use crate::ast::ASTNode;

use super::SourcePathSegmentV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ExprChildRoleV1 {
    LocalInitializer(u32),
    AssignmentTarget,
    AssignmentValue,
    CompoundAssignmentTarget,
    CompoundAssignmentValue,
    PrintValue,
    NowaitValue,
    ReturnValue,
    UnaryOperand,
    AwaitOperand,
    BinaryLeft,
    BinaryRight,
    IfCondition,
    LoopCondition,
    BlockExprTail,
    Receiver,
    IndexTarget,
    IndexSubscript,
    CallCallee,
    CallArgument(u32),
    ArrayElement(u32),
    MapEntryValue(u32),
    RecordFieldValue(u32),
    RecordUpdateBase,
    RecordUpdateValue(u32),
    CheckItem(u32),
    NewFieldInitializer(u32),
    GroupedAssignmentTarget,
    GroupedAssignmentValue,
}

impl ExprChildRoleV1 {
    pub(crate) fn segment_for(self, parent: &ASTNode) -> Option<SourcePathSegmentV1> {
        let segment = match (self, parent) {
            (Self::LocalInitializer(index), ASTNode::Local { .. }) => {
                SourcePathSegmentV1::Initializer(index)
            }
            (Self::AssignmentTarget, ASTNode::Assignment { .. }) => SourcePathSegmentV1::Target,
            (Self::AssignmentValue, ASTNode::Assignment { .. }) => SourcePathSegmentV1::Value,
            (Self::CompoundAssignmentTarget, ASTNode::CompoundAssignment { .. }) => {
                SourcePathSegmentV1::Target
            }
            (Self::CompoundAssignmentValue, ASTNode::CompoundAssignment { .. }) => {
                SourcePathSegmentV1::Value
            }
            (Self::PrintValue, ASTNode::Print { .. })
            | (Self::NowaitValue, ASTNode::Nowait { .. })
            | (Self::ReturnValue, ASTNode::Return { .. }) => SourcePathSegmentV1::Value,
            (Self::UnaryOperand, ASTNode::UnaryOp { .. })
            | (Self::AwaitOperand, ASTNode::AwaitExpression { .. }) => SourcePathSegmentV1::Operand,
            (Self::BinaryLeft, ASTNode::BinaryOp { .. }) => SourcePathSegmentV1::Lhs,
            (Self::BinaryRight, ASTNode::BinaryOp { .. }) => SourcePathSegmentV1::Rhs,
            (Self::IfCondition, ASTNode::If { .. }) => SourcePathSegmentV1::IfCondition,
            (Self::LoopCondition, ASTNode::Loop { .. }) => SourcePathSegmentV1::LoopCondition,
            (Self::BlockExprTail, ASTNode::BlockExpr { .. }) => SourcePathSegmentV1::BlockExprTail,
            (Self::Receiver, ASTNode::MethodCall { .. } | ASTNode::FieldAccess { .. }) => {
                SourcePathSegmentV1::Receiver
            }
            (Self::IndexTarget, ASTNode::Index { .. }) => SourcePathSegmentV1::Target,
            (Self::IndexSubscript, ASTNode::Index { .. }) => SourcePathSegmentV1::Argument(0),
            (Self::CallCallee, ASTNode::Call { .. }) => SourcePathSegmentV1::Callee,
            (
                Self::CallArgument(index),
                ASTNode::MethodCall { .. }
                | ASTNode::FunctionCall { .. }
                | ASTNode::FromCall { .. }
                | ASTNode::Call { .. }
                | ASTNode::New { .. },
            ) => SourcePathSegmentV1::Argument(index),
            (Self::ArrayElement(index), ASTNode::ArrayLiteral { .. }) => {
                SourcePathSegmentV1::Element(index)
            }
            (Self::MapEntryValue(index), ASTNode::MapLiteral { .. }) => {
                SourcePathSegmentV1::EntryValue(index)
            }
            (Self::RecordFieldValue(index), ASTNode::RecordLiteral { .. }) => {
                SourcePathSegmentV1::FieldValue(index)
            }
            (Self::RecordUpdateBase, ASTNode::RecordUpdate { .. }) => SourcePathSegmentV1::Base,
            (Self::RecordUpdateValue(index), ASTNode::RecordUpdate { .. }) => {
                SourcePathSegmentV1::UpdateValue(index)
            }
            (Self::CheckItem(index), ASTNode::CheckExpr { .. }) => {
                SourcePathSegmentV1::CheckItem(index)
            }
            (Self::NewFieldInitializer(index), ASTNode::New { .. }) => {
                SourcePathSegmentV1::Initializer(index)
            }
            (Self::GroupedAssignmentTarget, ASTNode::GroupedAssignmentExpr { .. }) => {
                SourcePathSegmentV1::Target
            }
            (Self::GroupedAssignmentValue, ASTNode::GroupedAssignmentExpr { .. }) => {
                SourcePathSegmentV1::Value
            }
            _ => return None,
        };
        Some(segment)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BodyChildRoleV1 {
    FunctionBody,
    LambdaBody,
    ScopeBody,
    TaskScopeBody,
    FastMemBody,
    IfThen,
    IfElse,
    LoopBody,
    BlockExprPrelude,
}

impl BodyChildRoleV1 {
    pub(crate) fn kind_for(self, parent: &ASTNode) -> Option<SourceBodyKindV1> {
        match (self, parent) {
            (Self::FunctionBody, ASTNode::FunctionDeclaration { .. }) => {
                Some(SourceBodyKindV1::Function)
            }
            (Self::LambdaBody, ASTNode::Lambda { .. }) => Some(SourceBodyKindV1::Lambda),
            (Self::ScopeBody, ASTNode::ScopeBox { .. }) => Some(SourceBodyKindV1::Scope),
            (Self::TaskScopeBody, ASTNode::TaskScope { .. }) => Some(SourceBodyKindV1::TaskScope),
            (Self::FastMemBody, ASTNode::FastMemRegion { .. }) => Some(SourceBodyKindV1::FastMem),
            (Self::IfThen, ASTNode::If { .. }) => Some(SourceBodyKindV1::IfThen),
            (Self::IfElse, ASTNode::If { .. }) => Some(SourceBodyKindV1::IfElse),
            (Self::LoopBody, ASTNode::Loop { .. }) => Some(SourceBodyKindV1::Loop),
            (Self::BlockExprPrelude, ASTNode::BlockExpr { .. }) => {
                Some(SourceBodyKindV1::BlockExprPrelude)
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SourceBodyKindV1 {
    Function,
    Lambda,
    Scope,
    TaskScope,
    FastMem,
    IfThen,
    IfElse,
    Loop,
    BlockExprPrelude,
}

impl SourceBodyKindV1 {
    pub(crate) fn root_segment(self) -> Option<SourcePathSegmentV1> {
        match self {
            Self::Function | Self::Lambda => None,
            Self::Scope => Some(SourcePathSegmentV1::ScopeBodyRoot),
            Self::TaskScope => Some(SourcePathSegmentV1::TaskScopeBodyRoot),
            Self::FastMem => Some(SourcePathSegmentV1::FastMemBodyRoot),
            Self::IfThen => Some(SourcePathSegmentV1::IfThenBody),
            Self::IfElse => Some(SourcePathSegmentV1::IfElseBody),
            Self::Loop => Some(SourcePathSegmentV1::LoopBodyRoot),
            Self::BlockExprPrelude => Some(SourcePathSegmentV1::BlockExprPreludeRoot),
        }
    }

    pub(crate) fn item_segment(self, index: u32) -> SourcePathSegmentV1 {
        match self {
            Self::Function => SourcePathSegmentV1::Body(index),
            Self::Lambda => SourcePathSegmentV1::LambdaBody(index),
            Self::Scope => SourcePathSegmentV1::ScopeBody(index),
            Self::TaskScope => SourcePathSegmentV1::TaskScopeBody(index),
            Self::FastMem => SourcePathSegmentV1::FastMemBody(index),
            Self::IfThen => SourcePathSegmentV1::IfThen(index),
            Self::IfElse => SourcePathSegmentV1::IfElse(index),
            Self::Loop => SourcePathSegmentV1::LoopBody(index),
            Self::BlockExprPrelude => SourcePathSegmentV1::BlockExprPrelude(index),
        }
    }
}
