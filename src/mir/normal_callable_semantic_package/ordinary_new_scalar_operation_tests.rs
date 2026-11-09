//! Original source membership and physical append refusals through the ledger.
use super::*;
use crate::ast::BinaryOperator;
use crate::mir::builder::MirBuilder;

#[test]
fn original_scalar_append_requires_same_source_and_exact_ordered_producer() {
    let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(
        include_str!("../../../lang/src/hako_alloc/memory/size_class_box.hako")
    ).unwrap();
    let ledger = &package.ordinary_new_claim_ledger;
    let sources: Vec<_> = package
        .batch()
        .declarations()
        .flat_map(|declaration| {
            ledger
                .completion_for_owner(declaration.owner())
                .and_then(|row| row.cleanup().root_flow())
                .map(|flow| flow.scalar_operations().cloned().collect::<Vec<_>>())
                .unwrap_or_default()
        })
        .collect();
    // B29–B31: the three mandatory sources coexist with one source-anchored optional Add.
    assert_eq!(
        sources
            .iter()
            .filter(|s| !s.original_needs_consumer_receipt_v1())
            .count(),
        3
    );
    let optional: Vec<_> = sources
        .iter()
        .filter(|s| s.original_needs_consumer_receipt_v1())
        .collect();
    let [add] = optional.as_slice() else {
        panic!("one optional Add source")
    };
    assert_eq!(source_operator(add).unwrap(), BinaryOp::Add);
    assert_eq!(add.positive_divisor(), None);
    assert_eq!(
        add.initializer().unwrap().initializer_site(),
        Some(add.binary().site())
    );
    assert_eq!(add.plan().nodes()[0].binary(), Some(add.binary()));
    for source in sources {
        let op = source_operator(&source).unwrap();
        let site = OwnedExprSiteV1::new(source.owner(), source.binary().site().clone());
        assert!(ledger
            .prepare_scalar_operation_source_v1(source.owner(), &site, Some(BinaryOp::Mul))
            .is_err());
        let loan = ledger
            .prepare_scalar_operation_source_v1(source.owner(), &site, Some(op))
            .unwrap()
            .unwrap();
        assert!(Rc::ptr_eq(&source, &loan));
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("scalar_original_test/0".into());
        let left = crate::mir::builder::emission::constant::emit_integer(&mut builder, 7).unwrap();
        let right = crate::mir::builder::emission::constant::emit_integer(&mut builder, 4).unwrap();
        let operator = match op {
            BinaryOp::Add => BinaryOperator::Add,
            BinaryOp::Sub => BinaryOperator::Subtract,
            BinaryOp::Div => BinaryOperator::Divide,
            BinaryOp::Mod => BinaryOperator::Modulo,
            _ => panic!("unexpected retained scalar law: {op:?}"),
        };
        let (completed, relations) = builder
            .build_source_scalar_from_values_recorded(operator.clone(), left, right)
            .unwrap();
        let entry = builder.current_block_for_test().unwrap();
        let mut physical = MirFunction::new(
            crate::mir::FunctionSignature {
                name: "scalar_original_test/0".into(),
                params: vec![],
                return_type: crate::mir::MirType::Integer,
                effects: crate::mir::EffectMask::PURE,
            },
            entry,
        );
        physical.blocks.get_mut(&entry).unwrap().instructions =
            builder.current_function_instructions();
        let function = &physical;
        let error = ledger
            .record_scalar_operation_append_v1(
                Rc::new((*loan).clone()),
                (left, right),
                relations.clone(),
                &completed,
                builder.local_ssa_physical_correspondence_v1(),
                function,
            )
            .unwrap_err();
        assert!(error.contains("source-membership"));
        let error = ledger
            .record_scalar_operation_append_v1(
                Rc::clone(&loan),
                (right, left),
                relations.clone(),
                &completed,
                builder.local_ssa_physical_correspondence_v1(),
                function,
            )
            .unwrap_err();
        assert!(error.contains("operand-correspondence"));
        let mut missing = (*function).clone();
        for block in missing.blocks.values_mut() {
            block
                .instructions
                .retain(|row| row.dst_value() != Some(completed.value()));
        }
        assert!(ledger
            .record_scalar_operation_append_v1(
                Rc::clone(&loan),
                (left, right),
                relations.clone(),
                &completed,
                builder.local_ssa_physical_correspondence_v1(),
                &missing
            )
            .is_err());
        if op == BinaryOp::Div {
            let mut changed = (*function).clone();
            for block in changed.blocks.values_mut() {
                for row in &mut block.instructions {
                    if let MirInstruction::Const { dst, value } = row {
                        if *dst == right {
                            *value = ConstValue::Integer(0);
                        }
                    }
                }
            }
            assert!(ledger
                .record_scalar_operation_append_v1(
                    Rc::clone(&loan),
                    (left, right),
                    relations.clone(),
                    &completed,
                    builder.local_ssa_physical_correspondence_v1(),
                    &changed
                )
                .unwrap_err()
                .contains("original-producer"));
        }
        let mut foreign_builder = MirBuilder::new();
        foreign_builder.enter_function_for_test("scalar_original_test/0".into());
        let foreign_left =
            crate::mir::builder::emission::constant::emit_integer(&mut foreign_builder, 7).unwrap();
        let foreign_right =
            crate::mir::builder::emission::constant::emit_integer(&mut foreign_builder, 4).unwrap();
        let (_, foreign_relations) = foreign_builder
            .build_source_scalar_from_values_recorded(operator, foreign_left, foreign_right)
            .unwrap();
        assert_eq!(foreign_relations[0].values(), relations[0].values());
        assert!(ledger
            .record_scalar_operation_append_v1(
                Rc::clone(&loan),
                (left, right),
                foreign_relations,
                &completed,
                builder.local_ssa_physical_correspondence_v1(),
                function,
            )
            .unwrap_err()
            .contains("foreign-index"));
        let row = ledger
            .record_scalar_operation_append_v1(
                Rc::clone(&loan),
                (left, right),
                relations.clone(),
                &completed,
                builder.local_ssa_physical_correspondence_v1(),
                function,
            )
            .unwrap();
        assert!(Rc::ptr_eq(row.source(), &source));
        assert_eq!(row.children(), (left, right));
        assert!(ledger
            .record_scalar_operation_append_v1(
                loan,
                (left, right),
                relations,
                &completed,
                builder.local_ssa_physical_correspondence_v1(),
                function
            )
            .unwrap_err()
            .contains("duplicate-original"));
    }
}
