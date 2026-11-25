//! Typed integer operation emission for scheduled Loop blocks.
//!
//! This is a thin typed bridge over existing MIR emission. It owns no route,
//! recipe, CFG, SSA, PHI, or publication policy.

use crate::mir::builder::emission::compare;
use crate::mir::builder::MirBuilder;
use crate::mir::{BinaryOp, CompareOp, MirInstruction, MirType, ValueId};

pub(in crate::mir::builder) fn emit_add_i64_at(
    builder: &mut MirBuilder,
    block: crate::mir::BasicBlockId,
    lhs: ValueId,
    rhs: ValueId,
) -> Result<ValueId, String> {
    emit_binary_i64_at(builder, block, BinaryOp::Add, lhs, rhs)
}

pub(in crate::mir::builder) fn emit_add_i64_at_with_dst(
    builder: &mut MirBuilder,
    block: crate::mir::BasicBlockId,
    dst: ValueId,
    lhs: ValueId,
    rhs: ValueId,
) -> Result<(), String> {
    require_i64_operands_at(builder, block, lhs, rhs)?;
    builder.emit_instruction_at(
        block,
        MirInstruction::BinOp {
            dst,
            op: BinaryOp::Add,
            lhs,
            rhs,
        },
    )?;
    builder
        .function_state
        .type_ctx
        .value_types
        .insert(dst, MirType::Integer);
    Ok(())
}

pub(in crate::mir::builder) fn emit_sub_i64_at(
    builder: &mut MirBuilder,
    block: crate::mir::BasicBlockId,
    lhs: ValueId,
    rhs: ValueId,
) -> Result<ValueId, String> {
    emit_binary_i64_at(builder, block, BinaryOp::Sub, lhs, rhs)
}

pub(in crate::mir::builder) fn emit_compare_i64_at(
    builder: &mut MirBuilder,
    block: crate::mir::BasicBlockId,
    op: CompareOp,
    lhs: ValueId,
    rhs: ValueId,
) -> Result<ValueId, String> {
    require_i64_operands_at(builder, block, lhs, rhs)?;
    let dst = builder.next_value_id();
    compare::emit_to_at(builder, block, dst, op, lhs, rhs)?;
    Ok(dst)
}

pub(in crate::mir::builder) fn emit_compare_i64_at_with_dst(
    builder: &mut MirBuilder,
    block: crate::mir::BasicBlockId,
    dst: ValueId,
    op: CompareOp,
    lhs: ValueId,
    rhs: ValueId,
) -> Result<(), String> {
    require_i64_operands_at(builder, block, lhs, rhs)?;
    compare::emit_to_at(builder, block, dst, op, lhs, rhs)
}

fn emit_binary_i64_at(
    builder: &mut MirBuilder,
    block: crate::mir::BasicBlockId,
    op: BinaryOp,
    lhs: ValueId,
    rhs: ValueId,
) -> Result<ValueId, String> {
    require_i64_operands_at(builder, block, lhs, rhs)?;
    let dst = builder.next_value_id();
    builder.emit_instruction_at(block, MirInstruction::BinOp { dst, op, lhs, rhs })?;
    builder
        .function_state
        .type_ctx
        .value_types
        .insert(dst, MirType::Integer);
    Ok(dst)
}

fn require_i64_operands_at(
    builder: &MirBuilder,
    block: crate::mir::BasicBlockId,
    lhs: ValueId,
    rhs: ValueId,
) -> Result<(), String> {
    let function = builder
        .function_state
        .current_function
        .as_ref()
        .ok_or_else(|| "[freeze:contract][loop_operation/current_function_missing]".to_string())?;
    if function.get_block(block).is_none() {
        return Err(format!(
            "[freeze:contract][loop_operation/target_block_not_in_function] block={block:?}"
        ));
    }
    for (label, value) in [("lhs", lhs), ("rhs", rhs)] {
        if builder.function_state.type_ctx.get_type(value) != Some(&MirType::Integer) {
            return Err(format!(
                "[freeze:contract][loop_operation/i64_operand_type] operand={label} value={value:?}"
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::builder::emission::constant;
    use crate::mir::builder::MirBuilder;

    #[test]
    fn ast_free_i64_operations_publish_canonical_types() {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("loop_operation_facade/0".to_owned());
        let block = builder.current_block_id().expect("entry block");
        let lhs = constant::emit_integer(&mut builder, 1).expect("lhs");
        let rhs = constant::emit_integer(&mut builder, 2).expect("rhs");
        let sum = emit_add_i64_at(&mut builder, block, lhs, rhs).expect("sum");
        let predicate = emit_compare_i64_at(&mut builder, block, CompareOp::Lt, sum, rhs)
            .expect("predicate");
        assert_eq!(
            builder.function_state.type_ctx.get_type(sum),
            Some(&MirType::Integer)
        );
        assert_eq!(
            builder.function_state.type_ctx.get_type(predicate),
            Some(&MirType::Bool)
        );
    }

    #[test]
    fn missing_i64_operand_rejects_before_instruction() {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("loop_operation_facade/reject".to_owned());
        let before = builder
            .function_state
            .current_function
            .as_ref()
            .expect("function")
            .entry_block()
            .instructions
            .len();
        let block = builder.current_block_id().expect("entry block");
        let error = emit_add_i64_at(&mut builder, block, ValueId::new(99), ValueId::new(100))
            .unwrap_err();
        assert!(error.contains("i64_operand_type"));
        let after = builder
            .function_state
            .current_function
            .as_ref()
            .expect("function")
            .entry_block()
            .instructions
            .len();
        assert_eq!(before, after);
    }
}
