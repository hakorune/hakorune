//! CompareEmissionBox — 比較命令発行の薄いヘルパ（仕様不変）

use crate::mir::builder::MirBuilder;
use crate::mir::{CompareOp, MirInstruction, ValueId};

use super::compare_type::PreparedCanonicalCompareBoolTypeV1;

#[inline]
pub fn emit_to(
    b: &mut MirBuilder,
    dst: ValueId,
    op: CompareOp,
    lhs: ValueId,
    rhs: ValueId,
) -> Result<(), String> {
    require_existing_current_compare_block(b)?;
    let prepared =
        PreparedCanonicalCompareBoolTypeV1::prepare(b.function_state.type_ctx.get_type(dst))
            .map_err(|error| error.to_string())?;
    b.emit_instruction(MirInstruction::Compare { dst, op, lhs, rhs })?;
    prepared.commit(dst, &mut b.function_state.type_ctx);
    Ok(())
}

fn require_existing_current_compare_block(builder: &MirBuilder) -> Result<(), String> {
    let block = builder
        .function_state
        .current_block
        .ok_or_else(|| "[freeze:contract][compare_emission/current_block_missing]".to_string())?;
    let function = builder
        .function_state
        .current_function
        .as_ref()
        .ok_or_else(|| {
            "[freeze:contract][compare_emission/current_function_missing]".to_string()
        })?;
    if !function.blocks.contains_key(&block) {
        return Err(format!(
            "[freeze:contract][compare_emission/current_block_not_in_function] fn={} block={block:?}",
            function.signature.name
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::emit_to;
    use crate::mir::{BasicBlockId, CompareOp, MirBuilder, MirInstruction, MirType, ValueId};

    #[test]
    fn valid_builder_compare_emits_one_instruction_and_keeps_bool_receipt() {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("compare_receipt/0".to_string());
        let destination = builder.alloc_value_for_test();

        emit_to(
            &mut builder,
            destination,
            CompareOp::Eq,
            ValueId::new(0),
            ValueId::new(1),
        )
        .unwrap();

        assert_eq!(
            builder.function_state.type_ctx.get_type(destination),
            Some(&MirType::Bool)
        );
        assert!(builder
            .function_state
            .current_function
            .as_ref()
            .unwrap()
            .blocks
            .values()
            .flat_map(|block| block.instructions.iter())
            .any(|instruction| matches!(
                instruction,
                MirInstruction::Compare { dst, .. } if *dst == destination
            )));
    }

    #[test]
    fn absent_builder_context_returns_before_bool_publication() {
        let mut builder = MirBuilder::new();
        let destination = builder.alloc_value_for_test();

        assert!(emit_to(
            &mut builder,
            destination,
            CompareOp::Eq,
            ValueId::new(0),
            ValueId::new(1),
        )
        .is_err());
        assert_eq!(builder.function_state.type_ctx.get_type(destination), None);
        assert!(builder.function_state.type_ctx.value_kinds.is_empty());
    }

    #[test]
    fn stale_current_block_rejects_before_instruction_or_bool_publication() {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("compare_stale_block/0".to_string());
        let destination = builder.alloc_value_for_test();
        let stale_block = BasicBlockId::new(9_999);
        builder.function_state.current_block = Some(stale_block);

        let error = emit_to(
            &mut builder,
            destination,
            CompareOp::Eq,
            ValueId::new(0),
            ValueId::new(1),
        )
        .unwrap_err();

        assert!(error.contains("[freeze:contract][compare_emission/current_block_not_in_function]"));
        assert_eq!(builder.function_state.type_ctx.get_type(destination), None);
        assert!(!builder
            .function_state
            .current_function
            .as_ref()
            .unwrap()
            .blocks
            .contains_key(&stale_block));
    }
}
