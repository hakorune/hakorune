//! Neutral structural projection from one canonical source path into an AST.
//!
//! This module owns only the closed `SourcePathSegmentV1` traversal. It does
//! not own function identity, semantic resolution, call routing, or lowering.

use crate::ast::ASTNode;

use super::{SourceNodeSiteV1, SourcePathSegmentV1};

#[derive(Debug, Clone, Copy)]
pub(in crate::mir) enum ProjectedSourceNodeV1<'source> {
    Node(&'source ASTNode),
    Body(&'source [ASTNode]),
    SyntheticName,
}

pub(in crate::mir) fn project_source_node_v1<'source>(
    root: &'source ASTNode,
    site: &SourceNodeSiteV1,
) -> Option<ProjectedSourceNodeV1<'source>> {
    let mut projected = ProjectedSourceNodeV1::Node(root);
    for segment in site.segments() {
        projected = project_segment(projected, segment)?;
    }
    Some(projected)
}

#[allow(clippy::match_same_arms)]
fn project_segment<'source>(
    parent: ProjectedSourceNodeV1<'source>,
    segment: &SourcePathSegmentV1,
) -> Option<ProjectedSourceNodeV1<'source>> {
    let ProjectedSourceNodeV1::Node(parent) = parent else {
        return None;
    };
    let projected = match (parent, segment) {
        (ASTNode::FunctionDeclaration { body, .. }, SourcePathSegmentV1::FunctionBody) => {
            ProjectedSourceNodeV1::Body(body)
        }
        (ASTNode::FunctionDeclaration { body, .. }, SourcePathSegmentV1::Body(index)) => {
            ProjectedSourceNodeV1::Node(body.get(*index as usize)?)
        }
        (ASTNode::Lambda { body, .. }, SourcePathSegmentV1::LambdaBodyRoot) => {
            ProjectedSourceNodeV1::Body(body)
        }
        (ASTNode::Lambda { body, .. }, SourcePathSegmentV1::LambdaBody(index)) => {
            ProjectedSourceNodeV1::Node(body.get(*index as usize)?)
        }
        (ASTNode::Local { initial_values, .. }, SourcePathSegmentV1::Initializer(index))
        | (ASTNode::Outbox { initial_values, .. }, SourcePathSegmentV1::Initializer(index)) => {
            ProjectedSourceNodeV1::Node(initial_values.get(*index as usize)?.as_deref()?)
        }
        (ASTNode::Assignment { target, .. }, SourcePathSegmentV1::Target)
        | (ASTNode::CompoundAssignment { target, .. }, SourcePathSegmentV1::Target) => {
            ProjectedSourceNodeV1::Node(target)
        }
        (ASTNode::Assignment { value, .. }, SourcePathSegmentV1::Value)
        | (ASTNode::CompoundAssignment { value, .. }, SourcePathSegmentV1::Value) => {
            ProjectedSourceNodeV1::Node(value)
        }
        (ASTNode::Print { expression, .. }, SourcePathSegmentV1::Value)
        | (ASTNode::Nowait { expression, .. }, SourcePathSegmentV1::Value) => {
            ProjectedSourceNodeV1::Node(expression)
        }
        (ASTNode::Return { value, .. }, SourcePathSegmentV1::Value) => {
            ProjectedSourceNodeV1::Node(value.as_deref()?)
        }
        (ASTNode::ScopeBox { body, .. }, SourcePathSegmentV1::ScopeBodyRoot) => {
            ProjectedSourceNodeV1::Body(body)
        }
        (ASTNode::ScopeBox { body, .. }, SourcePathSegmentV1::ScopeBody(index)) => {
            ProjectedSourceNodeV1::Node(body.get(*index as usize)?)
        }
        (ASTNode::TaskScope { body, .. }, SourcePathSegmentV1::TaskScopeBodyRoot) => {
            ProjectedSourceNodeV1::Body(body)
        }
        (ASTNode::TaskScope { body, .. }, SourcePathSegmentV1::TaskScopeBody(index)) => {
            ProjectedSourceNodeV1::Node(body.get(*index as usize)?)
        }
        (ASTNode::FastMemRegion { body, .. }, SourcePathSegmentV1::FastMemBodyRoot) => {
            ProjectedSourceNodeV1::Body(body)
        }
        (ASTNode::FastMemRegion { body, .. }, SourcePathSegmentV1::FastMemBody(index)) => {
            ProjectedSourceNodeV1::Node(body.get(*index as usize)?)
        }
        (ASTNode::If { condition, .. }, SourcePathSegmentV1::IfCondition) => {
            ProjectedSourceNodeV1::Node(condition)
        }
        (ASTNode::If { then_body, .. }, SourcePathSegmentV1::IfThenBody) => {
            ProjectedSourceNodeV1::Body(then_body)
        }
        (ASTNode::If { then_body, .. }, SourcePathSegmentV1::IfThen(index)) => {
            ProjectedSourceNodeV1::Node(then_body.get(*index as usize)?)
        }
        (ASTNode::If { else_body, .. }, SourcePathSegmentV1::IfElseBody) => {
            ProjectedSourceNodeV1::Body(else_body.as_deref()?)
        }
        (ASTNode::If { else_body, .. }, SourcePathSegmentV1::IfElse(index)) => {
            ProjectedSourceNodeV1::Node(else_body.as_deref()?.get(*index as usize)?)
        }
        (ASTNode::Loop { condition, .. }, SourcePathSegmentV1::LoopCondition) => {
            ProjectedSourceNodeV1::Node(condition)
        }
        (ASTNode::Loop { body, .. }, SourcePathSegmentV1::LoopBodyRoot) => {
            ProjectedSourceNodeV1::Body(body)
        }
        (ASTNode::Loop { body, .. }, SourcePathSegmentV1::LoopBody(index)) => {
            ProjectedSourceNodeV1::Node(body.get(*index as usize)?)
        }
        (ASTNode::BlockExpr { prelude_stmts, .. }, SourcePathSegmentV1::BlockExprPreludeRoot) => {
            ProjectedSourceNodeV1::Body(prelude_stmts)
        }
        (
            ASTNode::BlockExpr { prelude_stmts, .. },
            SourcePathSegmentV1::BlockExprPrelude(index),
        ) => ProjectedSourceNodeV1::Node(prelude_stmts.get(*index as usize)?),
        (ASTNode::BlockExpr { tail_expr, .. }, SourcePathSegmentV1::BlockExprTail) => {
            ProjectedSourceNodeV1::Node(tail_expr)
        }
        (ASTNode::UnaryOp { operand, .. }, SourcePathSegmentV1::Operand)
        | (
            ASTNode::AwaitExpression {
                expression: operand,
                ..
            },
            SourcePathSegmentV1::Operand,
        ) => ProjectedSourceNodeV1::Node(operand),
        (ASTNode::BinaryOp { left, .. }, SourcePathSegmentV1::Lhs) => {
            ProjectedSourceNodeV1::Node(left)
        }
        (ASTNode::BinaryOp { right, .. }, SourcePathSegmentV1::Rhs) => {
            ProjectedSourceNodeV1::Node(right)
        }
        (ASTNode::ArrayLiteral { elements, .. }, SourcePathSegmentV1::Element(index)) => {
            ProjectedSourceNodeV1::Node(elements.get(*index as usize)?)
        }
        (ASTNode::MapLiteral { entries, .. }, SourcePathSegmentV1::EntryValue(index)) => {
            ProjectedSourceNodeV1::Node(&entries.get(*index as usize)?.1)
        }
        (ASTNode::RecordLiteral { fields, .. }, SourcePathSegmentV1::FieldValue(index)) => {
            ProjectedSourceNodeV1::Node(&fields.get(*index as usize)?.1)
        }
        (ASTNode::RecordUpdate { base, .. }, SourcePathSegmentV1::Base) => {
            ProjectedSourceNodeV1::Node(base)
        }
        (ASTNode::RecordUpdate { updates, .. }, SourcePathSegmentV1::UpdateValue(index)) => {
            ProjectedSourceNodeV1::Node(&updates.get(*index as usize)?.1)
        }
        (ASTNode::CheckExpr { items, .. }, SourcePathSegmentV1::CheckItem(index)) => {
            ProjectedSourceNodeV1::Node(&items.get(*index as usize)?.expression)
        }
        (ASTNode::GroupedAssignmentExpr { rhs, .. }, SourcePathSegmentV1::Value) => {
            ProjectedSourceNodeV1::Node(rhs)
        }
        (ASTNode::GroupedAssignmentExpr { .. }, SourcePathSegmentV1::Target) => {
            ProjectedSourceNodeV1::SyntheticName
        }
        (ASTNode::MethodCall { object, .. }, SourcePathSegmentV1::Receiver)
        | (ASTNode::FieldAccess { object, .. }, SourcePathSegmentV1::Receiver) => {
            ProjectedSourceNodeV1::Node(object)
        }
        (ASTNode::Index { target, .. }, SourcePathSegmentV1::Target) => {
            ProjectedSourceNodeV1::Node(target)
        }
        (ASTNode::Index { index, .. }, SourcePathSegmentV1::Argument(0)) => {
            ProjectedSourceNodeV1::Node(index)
        }
        (ASTNode::Call { callee, .. }, SourcePathSegmentV1::Callee) => {
            ProjectedSourceNodeV1::Node(callee)
        }
        (ASTNode::MethodCall { arguments, .. }, SourcePathSegmentV1::Argument(index))
        | (ASTNode::FunctionCall { arguments, .. }, SourcePathSegmentV1::Argument(index))
        | (ASTNode::FromCall { arguments, .. }, SourcePathSegmentV1::Argument(index))
        | (ASTNode::Call { arguments, .. }, SourcePathSegmentV1::Argument(index))
        | (ASTNode::New { arguments, .. }, SourcePathSegmentV1::Argument(index)) => {
            ProjectedSourceNodeV1::Node(arguments.get(*index as usize)?)
        }
        (
            ASTNode::New {
                field_initializers, ..
            },
            SourcePathSegmentV1::Initializer(index),
        ) => ProjectedSourceNodeV1::Node(&field_initializers.get(*index as usize)?.1),
        _ => return None,
    };
    Some(projected)
}
