use super::binary::{binary_kind, BinaryKind};
use super::NormalizedExprLowererBox;
use crate::ast::{ASTNode, BinaryOperator, LiteralValue, UnaryOperator};
use crate::mir::join_ir::{ConstValue, JoinInst, MirLikeInst, UnaryOp};
use crate::mir::ValueId;
use std::collections::BTreeMap;

impl NormalizedExprLowererBox {
    pub(super) fn alloc_value_id(next_value_id: &mut u32) -> ValueId {
        let vid = ValueId(*next_value_id);
        *next_value_id += 1;
        vid
    }

    pub(super) fn lower_literal(
        value: &LiteralValue,
        body: &mut Vec<JoinInst>,
        next_value_id: &mut u32,
    ) -> Result<Option<ValueId>, String> {
        match value {
            LiteralValue::Integer(i) => {
                let dst = Self::alloc_value_id(next_value_id);
                body.push(JoinInst::Compute(MirLikeInst::Const {
                    dst,
                    value: ConstValue::Integer(*i),
                }));
                Ok(Some(dst))
            }
            LiteralValue::Bool(b) => {
                let dst = Self::alloc_value_id(next_value_id);
                body.push(JoinInst::Compute(MirLikeInst::Const {
                    dst,
                    value: ConstValue::Bool(*b),
                }));
                Ok(Some(dst))
            }
            _ => Ok(None),
        }
    }

    pub(super) fn lower_unary(
        operator: &UnaryOperator,
        operand: &ASTNode,
        env: &BTreeMap<String, ValueId>,
        body: &mut Vec<JoinInst>,
        next_value_id: &mut u32,
    ) -> Result<Option<ValueId>, String> {
        match operator {
            UnaryOperator::Minus => {
                let operand_vid = match Self::lower_int_expr(operand, env, body, next_value_id)? {
                    Some(v) => v,
                    None => return Ok(None),
                };
                let dst = Self::alloc_value_id(next_value_id);
                body.push(JoinInst::Compute(MirLikeInst::UnaryOp {
                    dst,
                    op: UnaryOp::Neg,
                    operand: operand_vid,
                }));
                Ok(Some(dst))
            }
            UnaryOperator::Not => {
                let operand_vid = match Self::lower_bool_expr(operand, env, body, next_value_id)? {
                    Some(v) => v,
                    None => return Ok(None),
                };
                let dst = Self::alloc_value_id(next_value_id);
                body.push(JoinInst::Compute(MirLikeInst::UnaryOp {
                    dst,
                    op: UnaryOp::Not,
                    operand: operand_vid,
                }));
                Ok(Some(dst))
            }
            UnaryOperator::BitNot => Ok(None),
            UnaryOperator::Weak => Ok(None), // Phase 285W-Syntax-0: Not supported in normalized lowering
        }
    }

    pub(super) fn lower_binary(
        operator: &BinaryOperator,
        left: &ASTNode,
        right: &ASTNode,
        env: &BTreeMap<String, ValueId>,
        body: &mut Vec<JoinInst>,
        next_value_id: &mut u32,
    ) -> Result<Option<ValueId>, String> {
        let Some(kind) = binary_kind(operator) else {
            return Ok(None);
        };

        match kind {
            BinaryKind::Arith(op) => {
                let lhs = match Self::lower_int_expr(left, env, body, next_value_id)? {
                    Some(v) => v,
                    None => return Ok(None),
                };
                let rhs = match Self::lower_int_expr(right, env, body, next_value_id)? {
                    Some(v) => v,
                    None => return Ok(None),
                };

                let dst = Self::alloc_value_id(next_value_id);
                body.push(JoinInst::Compute(MirLikeInst::BinOp { dst, op, lhs, rhs }));
                Ok(Some(dst))
            }
            BinaryKind::Compare(op) => {
                let lhs = match Self::lower_int_expr(left, env, body, next_value_id)? {
                    Some(v) => v,
                    None => return Ok(None),
                };
                let rhs = match Self::lower_int_expr(right, env, body, next_value_id)? {
                    Some(v) => v,
                    None => return Ok(None),
                };

                let dst = Self::alloc_value_id(next_value_id);
                body.push(JoinInst::Compute(MirLikeInst::Compare { dst, op, lhs, rhs }));
                Ok(Some(dst))
            }
        }
    }

    pub(super) fn lower_int_expr(
        ast: &ASTNode,
        env: &BTreeMap<String, ValueId>,
        body: &mut Vec<JoinInst>,
        next_value_id: &mut u32,
    ) -> Result<Option<ValueId>, String> {
        match ast {
            ASTNode::Variable { name, .. } => Ok(env.get(name).copied()),
            ASTNode::Literal { value, .. } => match value {
                LiteralValue::Integer(_) => Self::lower_literal(value, body, next_value_id),
                _ => Ok(None),
            },
            ASTNode::UnaryOp {
                operator: UnaryOperator::Minus,
                operand,
                ..
            } => Self::lower_unary(&UnaryOperator::Minus, operand, env, body, next_value_id),
            ASTNode::BinaryOp {
                operator, left, right, ..
            } => {
                let Some(BinaryKind::Arith(_)) = binary_kind(operator) else {
                    return Ok(None);
                };
                Self::lower_binary(operator, left, right, env, body, next_value_id)
            }
            _ => Ok(None),
        }
    }

    pub(super) fn lower_bool_expr(
        ast: &ASTNode,
        env: &BTreeMap<String, ValueId>,
        body: &mut Vec<JoinInst>,
        next_value_id: &mut u32,
    ) -> Result<Option<ValueId>, String> {
        match ast {
            ASTNode::Variable { name, .. } => Ok(env.get(name).copied()),
            ASTNode::Literal { value, .. } => match value {
                LiteralValue::Bool(_) => Self::lower_literal(value, body, next_value_id),
                _ => Ok(None),
            },
            ASTNode::UnaryOp {
                operator: UnaryOperator::Not,
                operand,
                ..
            } => Self::lower_unary(&UnaryOperator::Not, operand, env, body, next_value_id),
            _ => Ok(None),
        }
    }
}
