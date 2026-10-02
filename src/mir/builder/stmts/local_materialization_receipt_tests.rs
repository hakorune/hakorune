use super::*;
use crate::mir::{BasicBlockId, MirInstruction};

#[test]
fn emitted_copy_receipt_records_the_actual_block_and_instruction() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("copy_receipt".into());
    let _scope = crate::mir::builder::vars::lexical_scope::LexicalScopeGuard::new(&mut builder);
    let initializer =
        crate::mir::builder::emission::constant::emit_integer(&mut builder, 7).unwrap();
    let block = builder.function_state.current_block.unwrap();
    let mut rows = Vec::new();
    build_local_statement_from_values_with_types_and_preclaims_with_receipt_v1(
        &mut builder,
        vec!["alias".into()],
        vec![initializer],
        vec![],
        vec![],
        &mut rows,
        &[LocalValuePlacement::Copy],
    )
    .unwrap();
    let row = &rows[0];
    let expected = MirInstruction::Copy {
        dst: row.local(),
        src: initializer,
    };
    assert_eq!(row.copy(), Some(&(block, expected.clone())));
    assert!(builder
        .function_state
        .current_function
        .as_ref()
        .unwrap()
        .blocks[&block]
        .instructions
        .contains(&expected));
}

#[test]
fn reuse_and_local_contract_write_do_not_claim_a_copy() {
    for (placement, annotation) in [
        (LocalValuePlacement::ReuseInitializer, None),
        (LocalValuePlacement::Copy, Some("u8".to_string())),
    ] {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("non_copy_receipt".into());
        let _scope = crate::mir::builder::vars::lexical_scope::LexicalScopeGuard::new(&mut builder);
        let initializer =
            crate::mir::builder::emission::constant::emit_integer(&mut builder, 7).unwrap();
        let mut rows = Vec::new();
        build_local_statement_from_values_with_types_and_preclaims_with_receipt_v1(
            &mut builder,
            vec!["local".into()],
            vec![initializer],
            vec![annotation],
            vec![],
            &mut rows,
            &[placement],
        )
        .unwrap();
        assert!(rows[0].copy().is_none());
        assert_eq!(rows[0].initializer(), initializer);
        assert_eq!(
            rows[0].local() == initializer,
            placement == LocalValuePlacement::ReuseInitializer
        );
    }
}

#[test]
fn copy_receipt_rejects_wrong_source_destination_and_instruction() {
    let source = ValueId::new(7);
    let destination = ValueId::new(8);
    for instruction in [
        MirInstruction::Copy {
            dst: destination,
            src: ValueId::new(9),
        },
        MirInstruction::Copy {
            dst: ValueId::new(9),
            src: source,
        },
        MirInstruction::Const {
            dst: destination,
            value: crate::mir::ConstValue::Integer(7),
        },
    ] {
        let error = super::super::CompletedLocalBindingV1::new(0, source, destination)
            .with_copy(Some((BasicBlockId::new(0), instruction)))
            .unwrap_err();
        assert!(error.contains("copy-receipt-drift"));
    }
}
