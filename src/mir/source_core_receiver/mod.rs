//! Bounded source-only receiver representation proof for Core calls.

#![allow(dead_code)]

use crate::ast::{ASTNode, BinaryOperator, LiteralValue};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SourceCoreReceiverFactV1 {
    /// If evaluation produces a value, that value has String representation.
    ExactStringOnSuccess,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SourceCoreReceiverProofErrorV1 {
    UnsupportedLeftSpineTerminal,
}

/// A proof tied to the exact source expression that was inspected.
///
/// The source node is intentionally private: consumers may use the sealed
/// fact, but cannot turn this product into a second AST traversal authority.
#[derive(Debug)]
pub(crate) struct VerifiedSourceCoreReceiverV1<'source> {
    _source_expression: &'source ASTNode,
    fact: SourceCoreReceiverFactV1,
    _seal: SourceCoreReceiverSealV1,
}

#[derive(Debug)]
struct SourceCoreReceiverSealV1;

impl<'source> VerifiedSourceCoreReceiverV1<'source> {
    pub(crate) fn verify(
        expression: &'source ASTNode,
    ) -> Result<Self, SourceCoreReceiverProofErrorV1> {
        let mut cursor = expression;
        loop {
            match cursor {
                ASTNode::Literal {
                    value: LiteralValue::String(_),
                    ..
                } => {
                    return Ok(Self {
                        _source_expression: expression,
                        fact: SourceCoreReceiverFactV1::ExactStringOnSuccess,
                        _seal: SourceCoreReceiverSealV1,
                    });
                }
                ASTNode::BinaryOp {
                    operator: BinaryOperator::Add,
                    left,
                    ..
                } => cursor = left,
                _ => {
                    return Err(SourceCoreReceiverProofErrorV1::UnsupportedLeftSpineTerminal);
                }
            }
        }
    }

    pub(crate) const fn fact(&self) -> SourceCoreReceiverFactV1 {
        self.fact
    }
}

#[cfg(test)]
mod tests;
