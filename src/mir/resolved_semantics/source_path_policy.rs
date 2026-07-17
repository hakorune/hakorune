//! One neutral source-child role to structural-path policy.
//!
//! Resolver traversal, compiler located views, and future located legacy
//! lowering must consume this vocabulary. None may keep a second
//! AST-shape-to-`SourcePathSegmentV1` decision table.

use crate::ast::ASTNode;

use super::SourcePathSegmentV1;

pub(crate) fn is_statement_expression_surface_v1(node: &ASTNode) -> bool {
    matches!(
        node,
        ASTNode::Literal { .. }
            | ASTNode::Variable { .. }
            | ASTNode::BinaryOp { .. }
            | ASTNode::UnaryOp { .. }
            | ASTNode::MethodCall { .. }
            | ASTNode::FunctionCall { .. }
            | ASTNode::Call { .. }
            | ASTNode::New { .. }
            | ASTNode::ArrayLiteral { .. }
            | ASTNode::MapLiteral { .. }
            | ASTNode::RecordLiteral { .. }
            | ASTNode::RecordUpdate { .. }
            | ASTNode::FieldAccess { .. }
            | ASTNode::Index { .. }
            | ASTNode::BlockExpr { .. }
            | ASTNode::Lambda { .. }
    )
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum ExprChildSyntaxV1<'source> {
    Node(&'source ASTNode),
    SyntheticName,
    Missing,
}

#[derive(Debug, Clone)]
pub(crate) struct ResolvedExprChildV1<'source> {
    segment: SourcePathSegmentV1,
    syntax: ExprChildSyntaxV1<'source>,
}

impl<'source> ResolvedExprChildV1<'source> {
    pub(crate) fn segment(&self) -> SourcePathSegmentV1 {
        self.segment.clone()
    }

    pub(crate) const fn syntax(&self) -> ExprChildSyntaxV1<'source> {
        self.syntax
    }
}

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
        self.resolve(parent).map(|child| child.segment())
    }

    pub(crate) fn resolve<'source>(
        self,
        parent: &'source ASTNode,
    ) -> Option<ResolvedExprChildV1<'source>> {
        let (segment, syntax) = match (self, parent) {
            (Self::LocalInitializer(index), ASTNode::Local { initial_values, .. }) => (
                SourcePathSegmentV1::Initializer(index),
                initial_values
                    .get(index as usize)
                    .and_then(Option::as_deref)
                    .map(ExprChildSyntaxV1::Node)
                    .unwrap_or(ExprChildSyntaxV1::Missing),
            ),
            (Self::AssignmentTarget, ASTNode::Assignment { target, .. })
            | (Self::CompoundAssignmentTarget, ASTNode::CompoundAssignment { target, .. }) => {
                (SourcePathSegmentV1::Target, ExprChildSyntaxV1::Node(target))
            }
            (Self::AssignmentValue, ASTNode::Assignment { value, .. })
            | (Self::CompoundAssignmentValue, ASTNode::CompoundAssignment { value, .. }) => {
                (SourcePathSegmentV1::Value, ExprChildSyntaxV1::Node(value))
            }
            (Self::PrintValue, ASTNode::Print { expression, .. })
            | (Self::NowaitValue, ASTNode::Nowait { expression, .. }) => (
                SourcePathSegmentV1::Value,
                ExprChildSyntaxV1::Node(expression),
            ),
            (Self::ReturnValue, ASTNode::Return { value, .. }) => (
                SourcePathSegmentV1::Value,
                value
                    .as_deref()
                    .map(ExprChildSyntaxV1::Node)
                    .unwrap_or(ExprChildSyntaxV1::Missing),
            ),
            (Self::UnaryOperand, ASTNode::UnaryOp { operand, .. }) => (
                SourcePathSegmentV1::Operand,
                ExprChildSyntaxV1::Node(operand),
            ),
            (Self::AwaitOperand, ASTNode::AwaitExpression { expression, .. }) => (
                SourcePathSegmentV1::Operand,
                ExprChildSyntaxV1::Node(expression),
            ),
            (Self::BinaryLeft, ASTNode::BinaryOp { left, .. }) => {
                (SourcePathSegmentV1::Lhs, ExprChildSyntaxV1::Node(left))
            }
            (Self::BinaryRight, ASTNode::BinaryOp { right, .. }) => {
                (SourcePathSegmentV1::Rhs, ExprChildSyntaxV1::Node(right))
            }
            (Self::IfCondition, ASTNode::If { condition, .. }) => (
                SourcePathSegmentV1::IfCondition,
                ExprChildSyntaxV1::Node(condition),
            ),
            (Self::LoopCondition, ASTNode::Loop { condition, .. }) => (
                SourcePathSegmentV1::LoopCondition,
                ExprChildSyntaxV1::Node(condition),
            ),
            (Self::BlockExprTail, ASTNode::BlockExpr { tail_expr, .. }) => (
                SourcePathSegmentV1::BlockExprTail,
                ExprChildSyntaxV1::Node(tail_expr),
            ),
            (Self::Receiver, ASTNode::MethodCall { object, .. })
            | (Self::Receiver, ASTNode::FieldAccess { object, .. }) => (
                SourcePathSegmentV1::Receiver,
                ExprChildSyntaxV1::Node(object),
            ),
            (Self::IndexTarget, ASTNode::Index { target, .. }) => {
                (SourcePathSegmentV1::Target, ExprChildSyntaxV1::Node(target))
            }
            (Self::IndexSubscript, ASTNode::Index { index, .. }) => (
                SourcePathSegmentV1::Argument(0),
                ExprChildSyntaxV1::Node(index),
            ),
            (Self::CallCallee, ASTNode::Call { callee, .. }) => {
                (SourcePathSegmentV1::Callee, ExprChildSyntaxV1::Node(callee))
            }
            (Self::CallArgument(index), ASTNode::MethodCall { arguments, .. })
            | (Self::CallArgument(index), ASTNode::FunctionCall { arguments, .. })
            | (Self::CallArgument(index), ASTNode::FromCall { arguments, .. })
            | (Self::CallArgument(index), ASTNode::Call { arguments, .. })
            | (Self::CallArgument(index), ASTNode::New { arguments, .. }) => (
                SourcePathSegmentV1::Argument(index),
                arguments
                    .get(index as usize)
                    .map(ExprChildSyntaxV1::Node)
                    .unwrap_or(ExprChildSyntaxV1::Missing),
            ),
            (Self::ArrayElement(index), ASTNode::ArrayLiteral { elements, .. }) => (
                SourcePathSegmentV1::Element(index),
                elements
                    .get(index as usize)
                    .map(ExprChildSyntaxV1::Node)
                    .unwrap_or(ExprChildSyntaxV1::Missing),
            ),
            (Self::MapEntryValue(index), ASTNode::MapLiteral { entries, .. }) => (
                SourcePathSegmentV1::EntryValue(index),
                entries
                    .get(index as usize)
                    .map(|entry| ExprChildSyntaxV1::Node(&entry.1))
                    .unwrap_or(ExprChildSyntaxV1::Missing),
            ),
            (Self::RecordFieldValue(index), ASTNode::RecordLiteral { fields, .. }) => (
                SourcePathSegmentV1::FieldValue(index),
                fields
                    .get(index as usize)
                    .map(|field| ExprChildSyntaxV1::Node(&field.1))
                    .unwrap_or(ExprChildSyntaxV1::Missing),
            ),
            (Self::RecordUpdateBase, ASTNode::RecordUpdate { base, .. }) => {
                (SourcePathSegmentV1::Base, ExprChildSyntaxV1::Node(base))
            }
            (Self::RecordUpdateValue(index), ASTNode::RecordUpdate { updates, .. }) => (
                SourcePathSegmentV1::UpdateValue(index),
                updates
                    .get(index as usize)
                    .map(|update| ExprChildSyntaxV1::Node(&update.1))
                    .unwrap_or(ExprChildSyntaxV1::Missing),
            ),
            (Self::CheckItem(index), ASTNode::CheckExpr { items, .. }) => (
                SourcePathSegmentV1::CheckItem(index),
                items
                    .get(index as usize)
                    .map(|item| ExprChildSyntaxV1::Node(&item.expression))
                    .unwrap_or(ExprChildSyntaxV1::Missing),
            ),
            (
                Self::NewFieldInitializer(index),
                ASTNode::New {
                    field_initializers, ..
                },
            ) => (
                SourcePathSegmentV1::Initializer(index),
                field_initializers
                    .get(index as usize)
                    .map(|field| ExprChildSyntaxV1::Node(&field.1))
                    .unwrap_or(ExprChildSyntaxV1::Missing),
            ),
            (Self::GroupedAssignmentTarget, ASTNode::GroupedAssignmentExpr { .. }) => (
                SourcePathSegmentV1::Target,
                ExprChildSyntaxV1::SyntheticName,
            ),
            (Self::GroupedAssignmentValue, ASTNode::GroupedAssignmentExpr { rhs, .. }) => {
                (SourcePathSegmentV1::Value, ExprChildSyntaxV1::Node(rhs))
            }
            _ => return None,
        };
        Some(ResolvedExprChildV1 { segment, syntax })
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct ResolvedBodyChildV1<'source> {
    kind: SourceBodyKindV1,
    statements: Option<&'source [ASTNode]>,
}

impl<'source> ResolvedBodyChildV1<'source> {
    pub(crate) const fn kind(self) -> SourceBodyKindV1 {
        self.kind
    }

    pub(crate) const fn statements(self) -> Option<&'source [ASTNode]> {
        self.statements
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
        self.resolve(parent).map(ResolvedBodyChildV1::kind)
    }

    pub(crate) fn resolve<'source>(
        self,
        parent: &'source ASTNode,
    ) -> Option<ResolvedBodyChildV1<'source>> {
        let (kind, statements) = match (self, parent) {
            (Self::FunctionBody, ASTNode::FunctionDeclaration { body, .. }) => {
                (SourceBodyKindV1::Function, Some(body.as_slice()))
            }
            (Self::LambdaBody, ASTNode::Lambda { body, .. }) => {
                (SourceBodyKindV1::Lambda, Some(body.as_slice()))
            }
            (Self::ScopeBody, ASTNode::ScopeBox { body, .. }) => {
                (SourceBodyKindV1::Scope, Some(body.as_slice()))
            }
            (Self::TaskScopeBody, ASTNode::TaskScope { body, .. }) => {
                (SourceBodyKindV1::TaskScope, Some(body.as_slice()))
            }
            (Self::FastMemBody, ASTNode::FastMemRegion { body, .. }) => {
                (SourceBodyKindV1::FastMem, Some(body.as_slice()))
            }
            (Self::IfThen, ASTNode::If { then_body, .. }) => {
                (SourceBodyKindV1::IfThen, Some(then_body.as_slice()))
            }
            (Self::IfElse, ASTNode::If { else_body, .. }) => {
                (SourceBodyKindV1::IfElse, else_body.as_deref())
            }
            (Self::LoopBody, ASTNode::Loop { body, .. }) => {
                (SourceBodyKindV1::Loop, Some(body.as_slice()))
            }
            (Self::BlockExprPrelude, ASTNode::BlockExpr { prelude_stmts, .. }) => (
                SourceBodyKindV1::BlockExprPrelude,
                Some(prelude_stmts.as_slice()),
            ),
            _ => return None,
        };
        Some(ResolvedBodyChildV1 { kind, statements })
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

#[cfg(test)]
mod tests {
    use crate::ast::{ASTNode, LiteralValue, Span};

    use super::{ExprChildRoleV1, ExprChildSyntaxV1, SourcePathSegmentV1};

    #[test]
    fn grouped_assignment_target_is_path_only_synthetic_syntax() {
        let grouped = ASTNode::GroupedAssignmentExpr {
            lhs: "value".into(),
            rhs: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(1),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };
        let child = ExprChildRoleV1::GroupedAssignmentTarget
            .resolve(&grouped)
            .expect("grouped target role");
        assert_eq!(child.segment(), SourcePathSegmentV1::Target);
        assert!(matches!(child.syntax(), ExprChildSyntaxV1::SyntheticName));
    }

    #[test]
    fn missing_index_preserves_role_segment_for_existing_navigator_errors() {
        let local = ASTNode::Local {
            variables: vec!["value".into()],
            initial_values: vec![None],
            declared_type_names: vec![None],
            span: Span::unknown(),
        };
        let role = ExprChildRoleV1::LocalInitializer(9);
        assert_eq!(
            role.segment_for(&local),
            Some(SourcePathSegmentV1::Initializer(9))
        );
        assert!(matches!(
            role.resolve(&local).expect("matching local role").syntax(),
            ExprChildSyntaxV1::Missing
        ));
    }
}
