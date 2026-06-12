use super::*;
use crate::ast::Span;
use crate::mir::function::{FastMemRegionMetadata, FastMemRegionOrigin};
use crate::mir::instruction::{FastMemRegionId, MemOpAccess, MemOpKind};
use crate::mir::types::BinaryOp;
use crate::mir::{EffectMask, MirInstruction};
use crate::mir::{FunctionSignature, MirType};

fn test_function(instructions: Vec<MirInstruction>) -> MirFunction {
    let signature = FunctionSignature {
        name: "Main.fastmem/0".to_string(),
        params: vec![],
        return_type: MirType::Integer,
        effects: EffectMask::PURE,
    };
    let mut function = MirFunction::new(signature, BasicBlockId::new(0));
    function
        .metadata
        .fastmem_regions
        .push(FastMemRegionMetadata {
            id: FastMemRegionId::new(0),
            contract: "PageMapV0".to_string(),
            source_span: Span::unknown(),
            origin: FastMemRegionOrigin::SourceFastMemBlock,
            body_statement_count: 1,
            emitted_memop_count: instructions
                .iter()
                .filter(|instruction| matches!(instruction, MirInstruction::MemOp { .. }))
                .count(),
        });
    let block = function
        .get_block_mut(BasicBlockId::new(0))
        .expect("entry block");
    for instruction in instructions {
        block.add_instruction(instruction);
    }
    function
}

fn memop(
    kind: MemOpKind,
    dst: Option<ValueId>,
    operands: Vec<ValueId>,
    access: Option<MemOpAccess>,
    effects: EffectMask,
) -> MirInstruction {
    MirInstruction::MemOp {
        region: FastMemRegionId::new(0),
        kind,
        dst,
        operands,
        access,
        effects,
    }
}

fn error_text(function: &MirFunction) -> String {
    check_fastmem_regions(function)
        .expect_err("expected fastmem verification violation")
        .into_iter()
        .map(|error| error.to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn accepts_region_metadata_and_memop_shapes() {
    let function = test_function(vec![
        memop(
            MemOpKind::AddrOf,
            Some(ValueId::new(1)),
            vec![ValueId::new(0)],
            None,
            EffectMask::PURE,
        ),
        memop(
            MemOpKind::LogicalShr,
            Some(ValueId::new(2)),
            vec![ValueId::new(1), ValueId::new(0)],
            None,
            EffectMask::PURE,
        ),
        memop(
            MemOpKind::FieldStore,
            None,
            vec![ValueId::new(2), ValueId::new(0)],
            Some(MemOpAccess::field("local_free_head")),
            EffectMask::WRITE,
        ),
    ]);

    assert!(
        check_fastmem_regions(&function).is_ok(),
        "{}",
        error_text(&function)
    );
}

#[test]
fn accepts_owner_eq_memop_as_branch_condition() {
    let function = test_function(vec![
        memop(
            MemOpKind::OwnerEq,
            Some(ValueId::new(3)),
            vec![ValueId::new(1), ValueId::new(2)],
            None,
            EffectMask::PURE,
        ),
        MirInstruction::Branch {
            condition: ValueId::new(3),
            then_bb: BasicBlockId::new(1),
            else_bb: BasicBlockId::new(2),
            then_edge_args: None,
            else_edge_args: None,
        },
    ]);

    assert!(
        check_fastmem_regions(&function).is_ok(),
        "{}",
        error_text(&function)
    );
}

#[test]
fn rejects_memop_value_escape_to_return() {
    let function = test_function(vec![
        memop(
            MemOpKind::AddrOf,
            Some(ValueId::new(1)),
            vec![ValueId::new(0)],
            None,
            EffectMask::PURE,
        ),
        MirInstruction::Return {
            value: Some(ValueId::new(1)),
        },
    ]);

    let text = error_text(&function);
    assert!(text.contains("memop-value-escapes"), "{}", text);
    assert!(text.contains("barrier=return"), "{}", text);
}

#[test]
fn rejects_memop_value_escape_to_store_value() {
    let function = test_function(vec![
        memop(
            MemOpKind::AddrOf,
            Some(ValueId::new(1)),
            vec![ValueId::new(0)],
            None,
            EffectMask::PURE,
        ),
        MirInstruction::Store {
            value: ValueId::new(1),
            ptr: ValueId::new(9),
        },
    ]);

    let text = error_text(&function);
    assert!(text.contains("memop-value-escapes"), "{}", text);
    assert!(text.contains("barrier=store_like"), "{}", text);
}

#[test]
fn rejects_memop_value_escape_to_call_arg() {
    let function = test_function(vec![
        memop(
            MemOpKind::AddrOf,
            Some(ValueId::new(1)),
            vec![ValueId::new(0)],
            None,
            EffectMask::PURE,
        ),
        MirInstruction::Call {
            dst: None,
            func: ValueId::INVALID,
            callee: Some(crate::mir::Callee::Extern("env.test.sink".to_string())),
            args: vec![ValueId::new(1)],
            effects: EffectMask::IO,
        },
    ]);

    let text = error_text(&function);
    assert!(text.contains("memop-value-escapes"), "{}", text);
    assert!(text.contains("barrier=call"), "{}", text);
}

#[test]
fn rejects_memop_value_escape_to_debug_observe() {
    let function = test_function(vec![
        memop(
            MemOpKind::AddrOf,
            Some(ValueId::new(1)),
            vec![ValueId::new(0)],
            None,
            EffectMask::PURE,
        ),
        MirInstruction::Debug {
            value: ValueId::new(1),
            message: "observe".to_string(),
        },
    ]);

    let text = error_text(&function);
    assert!(text.contains("memop-value-escapes"), "{}", text);
    assert!(text.contains("barrier=debug_observe"), "{}", text);
}

#[test]
fn rejects_layout_ref_escape_to_ordinary_use() {
    let function = test_function(vec![
        memop(
            MemOpKind::TableIndex,
            Some(ValueId::new(1)),
            vec![ValueId::new(0), ValueId::new(9)],
            None,
            EffectMask::PURE,
        ),
        MirInstruction::BinOp {
            dst: ValueId::new(2),
            op: BinaryOp::Add,
            lhs: ValueId::new(1),
            rhs: ValueId::new(9),
        },
    ]);

    let text = error_text(&function);
    assert!(text.contains("memop-value-escapes"), "{}", text);
    assert!(text.contains("barrier=ordinary_use"), "{}", text);
}

#[test]
fn accepts_addr_of_numeric_use_in_binop() {
    let function = test_function(vec![
        memop(
            MemOpKind::AddrOf,
            Some(ValueId::new(1)),
            vec![ValueId::new(0)],
            None,
            EffectMask::PURE,
        ),
        MirInstruction::BinOp {
            dst: ValueId::new(2),
            op: BinaryOp::Add,
            lhs: ValueId::new(1),
            rhs: ValueId::new(9),
        },
    ]);

    assert!(
        check_fastmem_regions(&function).is_ok(),
        "{}",
        error_text(&function)
    );
}

#[test]
fn accepts_table_index_bridge_uses_for_field_access_and_copy() {
    let function = test_function(vec![
        memop(
            MemOpKind::TableIndex,
            Some(ValueId::new(1)),
            vec![ValueId::new(0), ValueId::new(9)],
            Some(MemOpAccess::table("page_table")),
            EffectMask::READ,
        ),
        MirInstruction::Copy {
            dst: ValueId::new(2),
            src: ValueId::new(1),
        },
        MirInstruction::FieldGet {
            dst: ValueId::new(3),
            base: ValueId::new(2),
            field: "used".to_string(),
            declared_type: None,
        },
        MirInstruction::FieldSet {
            base: ValueId::new(2),
            field: "used".to_string(),
            value: ValueId::new(3),
            declared_type: None,
        },
    ]);

    assert!(
        check_fastmem_regions(&function).is_ok(),
        "{}",
        error_text(&function)
    );
}

#[test]
fn single_input_phi_propagates_memop_escape_origin() {
    let function = test_function(vec![
        memop(
            MemOpKind::AddrOf,
            Some(ValueId::new(1)),
            vec![ValueId::new(0)],
            None,
            EffectMask::PURE,
        ),
        MirInstruction::Phi {
            dst: ValueId::new(2),
            inputs: vec![(BasicBlockId::new(0), ValueId::new(1))],
            type_hint: None,
        },
        MirInstruction::Return {
            value: Some(ValueId::new(2)),
        },
    ]);

    let text = error_text(&function);
    assert!(text.contains("memop-value-escapes"), "{}", text);
    assert!(text.contains("barrier=return"), "{}", text);
}

#[test]
fn multi_input_phi_is_memop_escape_barrier() {
    let function = test_function(vec![
        memop(
            MemOpKind::AddrOf,
            Some(ValueId::new(1)),
            vec![ValueId::new(0)],
            None,
            EffectMask::PURE,
        ),
        MirInstruction::Phi {
            dst: ValueId::new(3),
            inputs: vec![
                (BasicBlockId::new(0), ValueId::new(1)),
                (BasicBlockId::new(1), ValueId::new(2)),
            ],
            type_hint: None,
        },
    ]);

    let text = error_text(&function);
    assert!(text.contains("memop-value-escapes"), "{}", text);
    assert!(text.contains("barrier=phi_merge"), "{}", text);
}

#[test]
fn rejects_wrong_memop_effects() {
    let function = test_function(vec![memop(
        MemOpKind::FieldLoad,
        Some(ValueId::new(1)),
        vec![ValueId::new(0)],
        Some(MemOpAccess::field("owner_id")),
        EffectMask::PURE,
    )]);

    let text = error_text(&function);
    assert!(text.contains("effect-mask-mismatch"), "{}", text);
}

#[test]
fn rejects_unknown_region() {
    let mut function = test_function(vec![MirInstruction::MemOp {
        region: FastMemRegionId::new(7),
        kind: MemOpKind::AddrOf,
        dst: Some(ValueId::new(1)),
        operands: vec![ValueId::new(0)],
        access: None,
        effects: EffectMask::PURE,
    }]);
    function.metadata.fastmem_regions[0].emitted_memop_count = 0;

    let text = error_text(&function);
    assert!(text.contains("unknown-region"), "{}", text);
}

#[test]
fn rejects_layout_table_memops_without_symbolic_access_ids() {
    let field = test_function(vec![memop(
        MemOpKind::FieldLoad,
        Some(ValueId::new(1)),
        vec![ValueId::new(0)],
        None,
        EffectMask::READ,
    )]);
    let field_text = error_text(&field);
    assert!(
        field_text.contains("field-access-missing-field-id"),
        "{}",
        field_text
    );

    let table = test_function(vec![memop(
        MemOpKind::TableIndex,
        Some(ValueId::new(1)),
        vec![ValueId::new(0), ValueId::new(2)],
        None,
        EffectMask::READ,
    )]);
    let table_text = error_text(&table);
    assert!(
        table_text.contains("table-index-missing-table-id"),
        "{}",
        table_text
    );
}
