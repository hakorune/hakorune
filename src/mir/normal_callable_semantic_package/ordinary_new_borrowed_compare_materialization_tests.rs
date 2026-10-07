//! Exact source loan and original completion, without final operand inference.
use super::*;
use crate::mir::builder::MirBuilder;
use std::rc::Rc;

fn fixture(
    condition: &str,
) -> (
    Rc<OrdinaryNewClaimLedgerV1>,
    FunctionOwnerIdV1,
    OwnedExprSiteV1,
) {
    let text = format!("box Transport {{ birth() {{}} probe(p,q): i64 {{ if {condition} {{ return 1 }} return 0 }} }} static box Main {{ main() {{ local recv = new Transport() local out = recv.probe(5,6) return 0 }} }}");
    let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(&text).unwrap();
    let ledger = Rc::clone(&package.ordinary_new_claim_ledger);
    let contract = package
        .parameter_contracts
        .iter()
        .find(|row| row.parameters.len() == 2)
        .unwrap();
    let owner = contract.owner;
    ledger
        .record_borrowed_ordinary_entry_values_v1(
            owner,
            Ok(contract
                .parameters
                .iter()
                .enumerate()
                .map(|(i, p)| (i as u32, p.binding, ValueId(70 + i as u32)))
                .collect::<Vec<_>>()
                .into_boxed_slice()),
        )
        .unwrap();
    let source = ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    let site = source.definitions[&owner]
        .uses
        .iter()
        .find_map(|row| match &row.kind {
            Use::CompareOperand { binary, .. } => Some(binary.clone()),
            _ => None,
        })
        .unwrap();
    (ledger, owner, site)
}

fn completed(
    operator: crate::ast::BinaryOperator,
) -> (
    crate::mir::builder::ops::CompletedOrdinaryBinaryV1,
    (ValueId, ValueId),
) {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("compare_source_completion/0".into());
    let left = crate::mir::builder::emission::constant::emit_integer(&mut builder, 5).unwrap();
    let right = crate::mir::builder::emission::constant::emit_integer(&mut builder, 6).unwrap();
    (
        builder
            .build_binary_op_from_values_recorded(operator, left, right)
            .unwrap(),
        (left, right),
    )
}

#[test]
fn borrowed_compare_original_completion_retains_each_canonical_operand_receipt() {
    for condition in ["p > 0", "0 > p", "p > q"] {
        let (ledger, owner, site) = fixture(condition);
        let loan = ledger
            .prepare_borrowed_compare_source_v1(owner, &site, Some(CompareOp::Gt))
            .unwrap()
            .unwrap();
        assert_eq!(
            loan.witnesses.len(),
            if condition == "p > q" { 2 } else { 1 }
        );
        let source = ledger
            .borrowed_formal_source
            .as_ref()
            .unwrap()
            .as_ref()
            .unwrap();
        for witness in &loan.witnesses {
            let original = source.definitions[&owner]
                .uses
                .iter()
                .find_map(|row| match &row.kind {
                    Use::CompareOperand { binary, source }
                        if binary == &site && row.binding == witness.binding =>
                    {
                        Some(source.comparison_parts().3)
                    }
                    _ => None,
                })
                .unwrap();
            assert!(std::ptr::eq(original, witness.envelope));
        }
        let (completed, children) = completed(crate::ast::BinaryOperator::Greater);
        ledger
            .record_borrowed_compare_v1(loan, children, &completed)
            .unwrap();
        let mut count = 0;
        ledger
            .with_borrowed_ordinary_compares_v1(owner, |loan, raw, original| {
                assert_eq!(loan.site(), &site);
                assert_eq!(raw, children);
                assert_eq!(original, completed.comparison_original().unwrap());
                count += 1;
                Ok(())
            })
            .unwrap();
        assert_eq!(count, 1, "two borrowed rows share one actual append");
        let duplicate = ledger
            .prepare_borrowed_compare_source_v1(owner, &site, Some(CompareOp::Gt))
            .unwrap()
            .unwrap();
        assert!(ledger
            .record_borrowed_compare_v1(duplicate, children, &completed)
            .unwrap_err()
            .contains("duplicate-materialization"));
    }
}

#[test]
fn borrowed_compare_source_requires_exact_operator_owner_and_entry() {
    let (ledger, owner, site) = fixture("p > q");
    assert!(ledger
        .prepare_borrowed_compare_source_v1(owner, &site, Some(CompareOp::Le))
        .is_err());
    assert!(ledger
        .prepare_borrowed_compare_source_v1(owner, &site, None)
        .is_err());
    let unrelated = OwnedExprSiteV1::new(
        owner,
        crate::mir::resolved_semantics::SourcePathV1::root_body(99).expr(),
    );
    assert!(ledger
        .prepare_borrowed_compare_source_v1(owner, &unrelated, Some(CompareOp::Gt))
        .unwrap()
        .is_none());
    let mut issuer =
        crate::mir::resolved_semantics::FunctionOwnerIssuerV1::new_for_compilation().unwrap();
    let foreign = OwnedExprSiteV1::new(issuer.issue().unwrap(), site.site().clone());
    assert!(ledger
        .prepare_borrowed_compare_source_v1(owner, &foreign, Some(CompareOp::Gt))
        .is_err());
    ledger.borrowed_entry_values.borrow_mut().remove(&owner);
    assert!(ledger
        .prepare_borrowed_compare_source_v1(owner, &site, Some(CompareOp::Gt))
        .unwrap_err()
        .contains("entry-values-missing"));
}

#[test]
fn borrowed_compare_rejects_arithmetic_wrong_operator_and_missing_entry_completion() {
    for operator in [
        crate::ast::BinaryOperator::Add,
        crate::ast::BinaryOperator::LessEqual,
    ] {
        let (ledger, owner, site) = fixture("p > q");
        let loan = ledger
            .prepare_borrowed_compare_source_v1(owner, &site, Some(CompareOp::Gt))
            .unwrap()
            .unwrap();
        let (completed, children) = completed(operator);
        assert!(ledger
            .record_borrowed_compare_v1(loan, children, &completed)
            .is_err());
        assert!(ledger.borrowed_entry_values.borrow()[&owner]
            .comparisons
            .is_empty());
    }
    let (ledger, owner, site) = fixture("p > q");
    let loan = ledger
        .prepare_borrowed_compare_source_v1(owner, &site, Some(CompareOp::Gt))
        .unwrap()
        .unwrap();
    ledger.borrowed_entry_values.borrow_mut().remove(&owner);
    let (completed, children) = completed(crate::ast::BinaryOperator::Greater);
    assert!(ledger
        .record_borrowed_compare_v1(loan, children, &completed)
        .is_err());
}

#[test]
fn borrowed_compare_changed_source_receipt_and_physical_operator_refuse_before_visit() {
    for change_source in [true, false] {
        let (ledger, owner, site) = fixture("p > q");
        let loan = ledger
            .prepare_borrowed_compare_source_v1(owner, &site, Some(CompareOp::Gt))
            .unwrap()
            .unwrap();
        let (completed, children) = completed(crate::ast::BinaryOperator::Greater);
        ledger
            .record_borrowed_compare_v1(loan, children, &completed)
            .unwrap();
        let mut entries = ledger.borrowed_entry_values.borrow_mut();
        let record = entries
            .get_mut(&owner)
            .unwrap()
            .comparisons
            .get_mut(&completed.value())
            .unwrap();
        let record =
            Rc::get_mut(record).expect("no installed consumer in this source-observation test");
        if change_source {
            record.source.witnesses[0].envelope = crate::mir::dynamic_operator_contract::issue_dynamic_operator_execution_envelope_v1(
                crate::mir::dynamic_operator_contract::DynamicOperatorDomainV1::new(Family::LessEqual,Class::NormalInteger,Class::NormalInteger)).unwrap();
        } else if let MirInstruction::Compare { op, .. } = &mut record.original.1 {
            *op = CompareOp::Le;
        }
        drop(entries);
        assert!(ledger
            .with_borrowed_ordinary_compares_v1(owner, |_, _, _| panic!(
                "drift must refuse before visit"
            ))
            .is_err());
    }
}

#[test]
fn borrowed_compare_reuse_coverage_requires_original_source_and_same_records() {
    let (ledger, owner, site) = fixture("p > q");
    // Missing both observation and installed consumer must not erase source demand.
    assert!(ledger
        .verify_borrowed_compare_reuse_v1(owner, std::iter::empty())
        .unwrap_err()
        .contains("reuse-coverage"));
    let loan = ledger
        .prepare_borrowed_compare_source_v1(owner, &site, Some(CompareOp::Gt))
        .unwrap()
        .unwrap();
    let (completed, children) = completed(crate::ast::BinaryOperator::Greater);
    let record = ledger
        .record_borrowed_compare_v1(loan, children, &completed)
        .unwrap();
    ledger
        .verify_borrowed_compare_reuse_v1(owner, std::iter::once(&record))
        .unwrap();
    assert!(ledger
        .verify_borrowed_compare_reuse_v1(owner, std::iter::empty())
        .unwrap_err()
        .contains("reuse-coverage"));
    ledger
        .borrowed_entry_values
        .borrow_mut()
        .get_mut(&owner)
        .unwrap()
        .comparisons
        .clear();
    assert!(ledger
        .verify_borrowed_compare_reuse_v1(owner, std::iter::once(&record))
        .unwrap_err()
        .contains("reuse-coverage"));
    assert!(ledger
        .verify_borrowed_compare_reuse_v1(owner, std::iter::empty())
        .unwrap_err()
        .contains("reuse-coverage"));
}

#[test]
fn borrowed_compare_consumer_handoff_rejects_missing_and_orphan_groups() {
    use crate::mir::{EffectMask, FunctionSignature, MirFunction, MirType};
    let (ledger, owner, site) = fixture("p > q");
    let loan = ledger
        .prepare_borrowed_compare_source_v1(owner, &site, Some(CompareOp::Gt))
        .unwrap()
        .unwrap();
    let loans = ledger.borrow_compare_carrier_operands_v1(&loan).unwrap();
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("consumer_groups/0".into());
    let children = (ValueId(70), ValueId(71));
    let completed = builder
        .build_binary_op_from_values_recorded(
            crate::ast::BinaryOperator::Greater,
            children.0,
            children.1,
        )
        .unwrap();
    let record = ledger
        .record_borrowed_compare_v1(loan, children, &completed)
        .unwrap();
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "consumer_groups/0".into(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        record.original().0,
    );
    let branch = (
        record.original().0,
        MirInstruction::Branch {
            condition: record.value(),
            then_bb: BasicBlockId(1),
            else_bb: BasicBlockId(2),
            then_edge_args: None,
            else_edge_args: None,
        },
    );
    let crate::mir::MirInstruction::Compare { lhs, rhs, .. } = record.original().1 else {
        panic!("Compare");
    };
    let copies: Vec<_> = [(children.0, lhs), (children.1, rhs)]
        .into_iter()
        .filter(|(src, dst)| src != dst)
        .map(|(src, dst)| {
            (
                record.original().0,
                crate::mir::MirInstruction::Copy { src, dst },
            )
        })
        .collect();
    let block = function.blocks.get_mut(&record.original().0).unwrap();
    for copy in &copies {
        block.add_instruction(copy.1.clone());
    }
    block.add_instruction(record.original().1.clone());
    block.add_instruction(branch.1.clone());
    ledger
        .record_borrowed_compare_consumers_v1(
            owner,
            &function,
            std::iter::once((&record, vec![], vec![branch])),
        )
        .unwrap();
    function.params = vec![children.0, children.1];
    let loans: Vec<_> = loans.into_iter().map(Rc::new).collect();
    ledger
        .record_borrowed_compare_carrier_consumers_v1(
            owner,
            &function,
            loans.iter().map(|loan| {
                (loan, copies.iter().filter(|copy|
            matches!(copy.1, crate::mir::MirInstruction::Copy { src, .. } if src == loan.value()))
            .cloned().collect())
            }),
        )
        .unwrap();
    ledger.borrowed_compare_bindings_v1(owner).unwrap();
    ledger
        .verify_finished_borrowed_compares_v1(owner, &function, |binding| Ok(binding.clone()))
        .unwrap();
    assert!(ledger
        .record_borrowed_compare_consumers_v1(
            owner,
            &function,
            std::iter::once((&record, vec![], vec![]))
        )
        .is_err());
    let mut entries = ledger.borrowed_entry_values.borrow_mut();
    let groups = entries
        .get_mut(&owner)
        .unwrap()
        .comparison_consumers
        .as_mut()
        .unwrap();
    let original = groups[&record.value()].clone();
    groups.insert(ValueId(u32::MAX), original.clone());
    drop(entries);
    assert!(ledger
        .borrowed_compare_bindings_v1(owner)
        .unwrap_err()
        .contains("consumer-coverage"));
    assert!(ledger
        .verify_finished_borrowed_compares_v1(owner, &function, |binding| Ok(binding.clone()))
        .unwrap_err()
        .contains("consumer-coverage"));
    let mut entries = ledger.borrowed_entry_values.borrow_mut();
    let groups = entries
        .get_mut(&owner)
        .unwrap()
        .comparison_consumers
        .as_mut()
        .unwrap();
    groups.clear();
    drop(entries);
    assert!(ledger
        .borrowed_compare_bindings_v1(owner)
        .unwrap_err()
        .contains("consumer-coverage"));
    assert!(ledger
        .verify_finished_borrowed_compares_v1(owner, &function, |binding| Ok(binding.clone()))
        .unwrap_err()
        .contains("consumer-coverage"));
}
