use super::*;
use crate::mir::{BasicBlock, CompareOp, ConstValue, EffectMask, FunctionSignature, MirType};
fn fixture(side: usize, cross: bool) -> (MirFunction, Binding, Vec<Binding>, Binding) {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "literal/0".into(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId(0),
    );
    let root = (
        BasicBlockId(0),
        MirInstruction::Const {
            dst: ValueId(1),
            value: ConstValue::Integer(10),
        },
    );
    function
        .blocks
        .get_mut(&root.0)
        .unwrap()
        .add_instruction(root.1.clone());
    let consumer = if cross {
        function
            .blocks
            .get_mut(&root.0)
            .unwrap()
            .add_instruction(MirInstruction::Jump {
                target: BasicBlockId(1),
                edge_args: None,
            });
        function.add_block(BasicBlock::new(BasicBlockId(1)));
        BasicBlockId(1)
    } else {
        root.0
    };
    let copy = (
        consumer,
        MirInstruction::Copy {
            dst: ValueId(2),
            src: ValueId(1),
        },
    );
    let operand = if cross { ValueId(2) } else { ValueId(1) };
    let copies = if cross {
        function
            .blocks
            .get_mut(&consumer)
            .unwrap()
            .add_instruction(copy.1.clone());
        vec![copy]
    } else {
        vec![]
    };
    let compare = (
        consumer,
        MirInstruction::Compare {
            dst: ValueId(3),
            op: CompareOp::Gt,
            lhs: if side == 0 { operand } else { ValueId(9) },
            rhs: if side == 1 { operand } else { ValueId(9) },
        },
    );
    function
        .blocks
        .get_mut(&consumer)
        .unwrap()
        .add_instruction(compare.1.clone());
    (function, root, copies, compare)
}
#[test]
fn borrowed_literal_consumer_preserves_both_orders_and_dominating_copy() {
    for side in [0, 1] {
        for cross in [false, true] {
            let (function, root, copies, compare) = fixture(side, cross);
            check_literal_group(&function, &root, &copies, &compare, side).unwrap();
            assert!(check_literal_group(&function, &root, &copies, &compare, 1 - side).is_err());
        }
    }
}
#[test]
fn borrowed_literal_consumer_rejects_equal_payload_foreign_const_and_reordering() {
    let (mut function, root, copies, mut compare) = fixture(1, false);
    let foreign = MirInstruction::Const {
        dst: ValueId(4),
        value: ConstValue::Integer(10),
    };
    function
        .blocks
        .get_mut(&root.0)
        .unwrap()
        .instructions
        .insert(1, foreign);
    if let MirInstruction::Compare { rhs, .. } = &mut compare.1 {
        *rhs = ValueId(4);
    }
    function.blocks.get_mut(&root.0).unwrap().instructions[2] = compare.1.clone();
    assert!(check_literal_group(&function, &root, &copies, &compare, 1)
        .unwrap_err()
        .contains("operand-identity"));
    let (mut function, root, copies, compare) = fixture(1, false);
    function
        .blocks
        .get_mut(&root.0)
        .unwrap()
        .instructions
        .swap(0, 1);
    assert!(check_literal_group(&function, &root, &copies, &compare, 1)
        .unwrap_err()
        .contains("order"));
}
#[test]
fn borrowed_literal_consumer_rejects_unreachable_copy_and_deleted_root() {
    let (mut function, root, copies, compare) = fixture(1, true);
    function.blocks.get_mut(&root.0).unwrap().terminator = None;
    function.blocks.get_mut(&root.0).unwrap().successors.clear();
    function
        .blocks
        .get_mut(&root.0)
        .unwrap()
        .instructions
        .retain(|i| !matches!(i, MirInstruction::Jump { .. }));
    assert!(check_literal_group(&function, &root, &copies, &compare, 1)
        .unwrap_err()
        .contains("dominance"));
    let (mut function, root, copies, compare) = fixture(1, false);
    function
        .blocks
        .get_mut(&root.0)
        .unwrap()
        .instructions
        .remove(0);
    assert!(check_literal_group(&function, &root, &copies, &compare, 1).is_err());
}

#[test]
fn borrowed_literal_consumer_failed_handoff_preserves_unpublished_state() {
    let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(
        "box Transport { birth() {} probe(p): i64 { if p > 0 { return 1 } return 0 } } static box Main { main() { local recv = new Transport() return recv.probe(5) } }").unwrap();
    let ledger = &package.ordinary_new_claim_ledger;
    let contract = package
        .parameter_contracts
        .iter()
        .find(|row| row.parameters.len() == 1)
        .unwrap();
    let owner = contract.owner;
    ledger
        .record_borrowed_ordinary_entry_values_v1(
            owner,
            Ok(vec![(0, contract.parameters[0].binding, ValueId(72))].into_boxed_slice()),
        )
        .unwrap();
    let source = ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    let site = source.definitions[&owner].uses.iter().find_map(|row| {
        if let super::super::super::super::borrowed_formal_uses::BorrowedFormalUseDraftKindV1::CompareOperand { source, .. } = &row.kind {
            source.integer_literal().map(|(site, _)| site.clone())
        } else { None }
    }).unwrap();
    let mut builder = crate::mir::builder::MirBuilder::new();
    builder.enter_function_for_test("literal_handoff/0".into());
    let completed =
        crate::mir::builder::emission::constant::emit_integer_recorded(&mut builder, 0).unwrap();
    let loan = ledger
        .prepare_borrowed_compare_integer_literal_v1(owner, &site, 0)
        .unwrap()
        .unwrap();
    let record = ledger
        .record_borrowed_compare_integer_literal_v1(loan, &completed)
        .unwrap();
    let function = fixture(1, false).0;
    for _ in 0..2 {
        let error = ledger
            .record_borrowed_compare_literal_consumers_v1(
                owner,
                &function,
                std::iter::once((&record, vec![])),
            )
            .unwrap_err();
        assert!(error.contains("operand-coverage"), "{error}");
        assert!(ledger.borrowed_entry_values.borrow()[&owner]
            .literal_consumers
            .is_none());
    }
}
