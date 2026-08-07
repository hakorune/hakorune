//! Neutral source-shape vocabulary for the caller-zero callable Loop profile.
//!
//! This module records only as-written call/literal/operator shapes.  It owns
//! no resolver target, Recipe relation, ValueId, CFG, or physical policy.

#![cfg(test)]

use crate::ast::{ASTNode, BinaryOperator, LiteralValue};

use super::located::LocatedExprV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SyntaxBinaryOperatorV1 {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    BitAnd,
    BitOr,
    BitXor,
    Shl,
    Shr,
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SourceLiteralShapeV1 {
    Integer(i64),
    TypedInteger {
        value: i64,
        declared_type_name: Box<str>,
    },
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SourceReceiverShapeV1 {
    Me,
    This,
    Other,
    FreeStatic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SourceCallKindV1 {
    Method(SourceReceiverShapeV1),
    FreeStatic,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SourceCallBoundaryShapeV1 {
    kind: SourceCallKindV1,
    argument_count: u32,
}

impl SourceCallBoundaryShapeV1 {
    pub(crate) const fn method(receiver: SourceReceiverShapeV1, argument_count: u32) -> Self {
        Self {
            kind: SourceCallKindV1::Method(receiver),
            argument_count,
        }
    }

    pub(crate) const fn free_static(argument_count: u32) -> Self {
        Self {
            kind: SourceCallKindV1::FreeStatic,
            argument_count,
        }
    }

    pub(crate) const fn kind(&self) -> SourceCallKindV1 {
        self.kind
    }

    pub(crate) const fn argument_count(&self) -> u32 {
        self.argument_count
    }

    /// Compatibility accessor for the current MethodCall-only prepare row.
    /// The static profile must use `kind()`, never `Other`, as its proof.
    pub(crate) const fn receiver(&self) -> SourceReceiverShapeV1 {
        match self.kind {
            SourceCallKindV1::Method(receiver) => receiver,
            SourceCallKindV1::FreeStatic => SourceReceiverShapeV1::FreeStatic,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SourceExprShapeV1 {
    Variable,
    Literal(SourceLiteralShapeV1),
    MethodCall(SourceCallBoundaryShapeV1),
    FreeStaticCall(SourceCallBoundaryShapeV1),
    Other,
}

pub(crate) fn binary_operator_shape(operator: &BinaryOperator) -> SyntaxBinaryOperatorV1 {
    match operator {
        BinaryOperator::Add => SyntaxBinaryOperatorV1::Add,
        BinaryOperator::Subtract => SyntaxBinaryOperatorV1::Subtract,
        BinaryOperator::Multiply => SyntaxBinaryOperatorV1::Multiply,
        BinaryOperator::Divide => SyntaxBinaryOperatorV1::Divide,
        BinaryOperator::Modulo => SyntaxBinaryOperatorV1::Modulo,
        BinaryOperator::BitAnd => SyntaxBinaryOperatorV1::BitAnd,
        BinaryOperator::BitOr => SyntaxBinaryOperatorV1::BitOr,
        BinaryOperator::BitXor => SyntaxBinaryOperatorV1::BitXor,
        BinaryOperator::Shl => SyntaxBinaryOperatorV1::Shl,
        BinaryOperator::Shr => SyntaxBinaryOperatorV1::Shr,
        BinaryOperator::Equal => SyntaxBinaryOperatorV1::Equal,
        BinaryOperator::NotEqual => SyntaxBinaryOperatorV1::NotEqual,
        BinaryOperator::Less => SyntaxBinaryOperatorV1::Less,
        BinaryOperator::Greater => SyntaxBinaryOperatorV1::Greater,
        BinaryOperator::LessEqual => SyntaxBinaryOperatorV1::LessEqual,
        BinaryOperator::GreaterEqual => SyntaxBinaryOperatorV1::GreaterEqual,
        BinaryOperator::And => SyntaxBinaryOperatorV1::And,
        BinaryOperator::Or => SyntaxBinaryOperatorV1::Or,
    }
}

pub(crate) fn literal_shape(value: &LiteralValue) -> SourceLiteralShapeV1 {
    match value {
        LiteralValue::Integer(value) => SourceLiteralShapeV1::Integer(*value),
        LiteralValue::TypedInteger {
            value,
            declared_type_name,
        } => SourceLiteralShapeV1::TypedInteger {
            value: *value,
            declared_type_name: declared_type_name.clone().into_boxed_str(),
        },
        _ => SourceLiteralShapeV1::Other,
    }
}

pub(crate) fn literal_shape_from_expr(expr: &LocatedExprV1<'_>) -> Option<SourceLiteralShapeV1> {
    match expr.node() {
        ASTNode::Literal { value, .. } => Some(literal_shape(value)),
        _ => None,
    }
}

pub(crate) fn expr_shape(node: &ASTNode) -> SourceExprShapeV1 {
    match node {
        ASTNode::Variable { .. } => SourceExprShapeV1::Variable,
        ASTNode::Literal { value, .. } => SourceExprShapeV1::Literal(literal_shape(value)),
        ASTNode::MethodCall {
            object, arguments, ..
        } => SourceExprShapeV1::MethodCall(SourceCallBoundaryShapeV1::method(
            receiver_shape(object),
            arguments.len() as u32,
        )),
        ASTNode::FunctionCall { arguments, .. } => SourceExprShapeV1::FreeStaticCall(
            SourceCallBoundaryShapeV1::free_static(arguments.len() as u32),
        ),
        _ => SourceExprShapeV1::Other,
    }
}

pub(crate) fn receiver_shape(node: &ASTNode) -> SourceReceiverShapeV1 {
    match node {
        ASTNode::Me { .. } | ASTNode::MeField { .. } => SourceReceiverShapeV1::Me,
        ASTNode::This { .. } | ASTNode::ThisField { .. } => SourceReceiverShapeV1::This,
        _ => SourceReceiverShapeV1::Other,
    }
}
