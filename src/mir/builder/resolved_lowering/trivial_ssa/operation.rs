//! Exact trivial operation materialization without legacy operator routing.

use crate::ast::BinaryOperator;
use crate::mir::resolved_value_profile::product::TrivialRepresentationV1;
use crate::mir::{BinaryOp, CompareOp, MirInstruction, ValueId};

use super::super::super::MirBuilder;
use super::operation_type::PreparedResolvedTrivialOperationTypeV1;

pub(super) use super::operation_type::exact_type_for_representation as mir_type;

pub(super) fn emit_binary(
    builder: &mut MirBuilder,
    operator: &BinaryOperator,
    lhs: ValueId,
    rhs: ValueId,
    representation: TrivialRepresentationV1,
) -> Result<ValueId, String> {
    let dst = builder.next_value_id();
    let prepared = PreparedResolvedTrivialOperationTypeV1::prepare(
        representation,
        builder.function_state.type_ctx.get_type(dst),
    )
    .map_err(|error| error.to_string())?;
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
    prepared.commit(dst, &mut builder.function_state.type_ctx);
    Ok(dst)
}

fn arithmetic(dst: ValueId, op: BinaryOp, lhs: ValueId, rhs: ValueId) -> MirInstruction {
    MirInstruction::BinOp { dst, op, lhs, rhs }
}

fn comparison(dst: ValueId, op: CompareOp, lhs: ValueId, rhs: ValueId) -> MirInstruction {
    MirInstruction::Compare { dst, op, lhs, rhs }
}

#[cfg(test)]
mod tests {
    use super::emit_binary;
    use crate::ast::BinaryOperator;
    use crate::mir::builder::MirBuilder;
    use crate::mir::resolved_value_profile::product::TrivialRepresentationV1;
    use crate::mir::{MirInstruction, MirType, ValueId};

    #[test]
    fn successful_operation_receipt_precedes_the_existing_exact_type_fact() {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("resolved_operation_receipt/0".to_string());

        let arithmetic = emit_binary(
            &mut builder,
            &BinaryOperator::Add,
            ValueId::new(0),
            ValueId::new(1),
            TrivialRepresentationV1::InlineI64,
        )
        .unwrap();
        let comparison = emit_binary(
            &mut builder,
            &BinaryOperator::Equal,
            arithmetic,
            ValueId::new(2),
            TrivialRepresentationV1::InlineBool,
        )
        .unwrap();

        assert_eq!(
            builder.function_state.type_ctx.get_type(arithmetic),
            Some(&MirType::Integer)
        );
        assert_eq!(
            builder.function_state.type_ctx.get_type(comparison),
            Some(&MirType::Bool)
        );
        let instructions: Vec<_> = builder
            .function_state
            .current_function
            .as_ref()
            .unwrap()
            .blocks
            .values()
            .flat_map(|block| block.instructions.iter())
            .collect();
        assert!(instructions.iter().any(|instruction| {
            matches!(instruction, MirInstruction::BinOp { dst, .. } if *dst == arithmetic)
        }));
        assert!(instructions.iter().any(|instruction| {
            matches!(instruction, MirInstruction::Compare { dst, .. } if *dst == comparison)
        }));
    }

    #[test]
    fn failed_operation_emission_leaves_no_destination_type_fact() {
        let mut builder = MirBuilder::new();

        assert_eq!(
            emit_binary(
                &mut builder,
                &BinaryOperator::Add,
                ValueId::new(0),
                ValueId::new(1),
                TrivialRepresentationV1::InlineI64,
            ),
            Err("No current basic block".to_string())
        );
        assert!(builder.function_state.type_ctx.value_types.is_empty());
        assert!(builder
            .function_state
            .type_ctx
            .value_origin_newbox
            .is_empty());
        assert!(builder.function_state.variable_ctx.variable_map.is_empty());
    }
}
