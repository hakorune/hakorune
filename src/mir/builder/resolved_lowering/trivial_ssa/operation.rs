//! Exact trivial operation materialization without legacy operator routing.

use crate::ast::BinaryOperator;
use crate::mir::resolved_value_profile::product::TrivialRepresentationV1;
use crate::mir::{BinaryOp, CompareOp, MirInstruction, ValueId};

use super::super::super::MirBuilder;

pub(super) use super::operation_type::exact_type_for_representation as mir_type;

pub(super) fn emit_binary(
    builder: &mut MirBuilder,
    operator: &BinaryOperator,
    lhs: ValueId,
    rhs: ValueId,
    representation: TrivialRepresentationV1,
) -> Result<ValueId, String> {
    let dst = builder.next_value_id();
    let instruction = match operator {
        BinaryOperator::Add => arithmetic(dst, BinaryOp::Add, lhs, rhs),
        BinaryOperator::Subtract => arithmetic(dst, BinaryOp::Sub, lhs, rhs),
        BinaryOperator::Multiply => arithmetic(dst, BinaryOp::Mul, lhs, rhs),
        BinaryOperator::Divide => arithmetic(dst, BinaryOp::Div, lhs, rhs),
        BinaryOperator::Modulo => arithmetic(dst, BinaryOp::Mod, lhs, rhs),
        BinaryOperator::BitAnd => arithmetic(dst, BinaryOp::BitAnd, lhs, rhs),
        BinaryOperator::BitOr => arithmetic(dst, BinaryOp::BitOr, lhs, rhs),
        BinaryOperator::BitXor => arithmetic(dst, BinaryOp::BitXor, lhs, rhs),
        BinaryOperator::Shl => arithmetic(dst, BinaryOp::Shl, lhs, rhs),
        BinaryOperator::Shr => arithmetic(dst, BinaryOp::Shr, lhs, rhs),
        BinaryOperator::Equal => comparison(dst, CompareOp::Eq, lhs, rhs),
        BinaryOperator::NotEqual => comparison(dst, CompareOp::Ne, lhs, rhs),
        BinaryOperator::Less => comparison(dst, CompareOp::Lt, lhs, rhs),
        BinaryOperator::LessEqual => comparison(dst, CompareOp::Le, lhs, rhs),
        BinaryOperator::Greater => comparison(dst, CompareOp::Gt, lhs, rhs),
        BinaryOperator::GreaterEqual => comparison(dst, CompareOp::Ge, lhs, rhs),
        BinaryOperator::And | BinaryOperator::Or => {
            return Err("[freeze:contract][trivial_ssa/operator_outside_profile]".to_string())
        }
    };
    builder.emit_instruction(instruction)?;
    builder
        .function_state
        .type_ctx
        .value_types
        .insert(dst, mir_type(representation));
    Ok(dst)
}

fn arithmetic(dst: ValueId, op: BinaryOp, lhs: ValueId, rhs: ValueId) -> MirInstruction {
    MirInstruction::BinOp { dst, op, lhs, rhs }
}

fn comparison(dst: ValueId, op: CompareOp, lhs: ValueId, rhs: ValueId) -> MirInstruction {
    MirInstruction::Compare { dst, op, lhs, rhs }
}
