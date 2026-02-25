use crate::ast::BinaryOperator;
use crate::mir::join_ir::{BinOpKind, CompareOp};

pub(super) enum BinaryKind {
    Arith(BinOpKind),
    Compare(CompareOp),
}

pub(super) fn binary_kind(op: &BinaryOperator) -> Option<BinaryKind> {
    match op {
        BinaryOperator::Add => Some(BinaryKind::Arith(BinOpKind::Add)),
        BinaryOperator::Subtract => Some(BinaryKind::Arith(BinOpKind::Sub)),
        BinaryOperator::Multiply => Some(BinaryKind::Arith(BinOpKind::Mul)),
        BinaryOperator::Divide => Some(BinaryKind::Arith(BinOpKind::Div)),
        BinaryOperator::Equal => Some(BinaryKind::Compare(CompareOp::Eq)),
        BinaryOperator::NotEqual => Some(BinaryKind::Compare(CompareOp::Ne)),
        BinaryOperator::Less => Some(BinaryKind::Compare(CompareOp::Lt)),
        BinaryOperator::LessEqual => Some(BinaryKind::Compare(CompareOp::Le)),
        BinaryOperator::Greater => Some(BinaryKind::Compare(CompareOp::Gt)),
        BinaryOperator::GreaterEqual => Some(BinaryKind::Compare(CompareOp::Ge)),
        _ => None,
    }
}
