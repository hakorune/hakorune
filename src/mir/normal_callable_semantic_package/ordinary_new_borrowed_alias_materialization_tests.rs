//! Final loan mutations preserve the original source and materialization owner.
use super::*;
use std::rc::Rc;

fn fixture() -> (Rc<OrdinaryNewClaimLedgerV1>, FunctionOwnerIdV1) {
    let text = "box Transport { birth() {} probe(p): i64 { return 0 } forward(q): i64 { local alias = q local recv = new Transport() local out = recv.probe(alias) return 0 } } static box Main { main() { local recv = new Transport() local out = recv.forward(true) return 0 } }";
    let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(text).unwrap();
    let (_, row, _, ledger, _, _) =
        crate::mir::builder::lexical_call_projection_forwarded_fixture(package, BasicBlockId(0));
    (ledger, row.call_site().owner())
}

#[test]
fn borrowed_alias_materialization_final_loan_rechecks_original_source_identity() {
    for mutation in 0..6 {
        let (ledger, owner) = fixture();
        let mut observed = 0;
        ledger
            .with_borrowed_ordinary_alias_copies_v1(owner, |_, _, _, _, value, copies| {
                assert_eq!(value, ValueId(73));
                assert_eq!(copies.len(), 1);
                observed += 1;
                Ok(())
            })
            .unwrap();
        assert_eq!(observed, 1);
        {
            let mut entries = ledger.borrowed_entry_values.borrow_mut();
            let entry = entries.get_mut(&owner).unwrap();
            let binding = *entry.aliases.keys().next().unwrap();
            let alias = Rc::get_mut(entry.aliases.get_mut(&binding).unwrap())
                .expect("unshared fixture record");
            match mutation {
                0 => {
                    alias.site = OwnedExprSiteV1::new(
                        owner,
                        SourcePathV1::root_body(99)
                            .child(SourcePathSegmentV1::Initializer(0))
                            .expr(),
                    )
                }
                1 => alias.declaration = SourceBindingSiteV1::Parameter { index: 0 },
                2 => alias.formal = binding,
                3 => alias.source_binding = binding,
                4 => alias.value = ValueId(999),
                _ => {
                    let key = alias.formal;
                    let original = entry.aliases.remove(&binding).unwrap();
                    entry.aliases.insert(key, original);
                }
            }
        }
        let mut published = 0;
        assert!(
            ledger
                .with_borrowed_ordinary_alias_copies_v1(owner, |_, _, _, _, _, _| {
                    published += 1;
                    Ok(())
                })
                .is_err(),
            "mutation {mutation}"
        );
        assert_eq!(published, 0);
    }
}

#[test]
fn borrowed_alias_materialization_final_loan_refuses_changed_entry_value_and_ordinal() {
    for ordinal in [false, true] {
        let (ledger, owner) = fixture();
        let mut entries = ledger.borrowed_entry_values.borrow_mut();
        let row = &mut entries.get_mut(&owner).unwrap().values.as_mut().unwrap()[0];
        if ordinal {
            row.0 = 9;
        } else {
            row.2 = ValueId(999);
        }
        drop(entries);
        assert!(ledger
            .with_borrowed_ordinary_alias_copies_v1(owner, |_, _, _, _, _, _| Ok(()))
            .is_err());
    }
}

fn compare_fixture() -> (
    Rc<OrdinaryNewClaimLedgerV1>,
    FunctionOwnerIdV1,
    super::super::compare_materialization::BorrowedCompareSourceLoanV1,
) {
    compare_fixture_for("alias > 0")
}

fn compare_fixture_for(
    condition: &str,
) -> (
    Rc<OrdinaryNewClaimLedgerV1>,
    FunctionOwnerIdV1,
    super::super::compare_materialization::BorrowedCompareSourceLoanV1,
) {
    let text = "box Transport { birth() {} probe(p): i64 { return 0 } forward(q): i64 { local alias = q local recv = new Transport() local out = recv.probe(alias) if alias > 0 { return 0 } return 0 } } static box Main { main() { local recv = new Transport() local out = recv.forward(true) return 0 } }";
    let text = text.replace("if alias > 0", &format!("if {condition}"));
    let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(&text).unwrap();
    let (_, row, _, ledger, _, _) =
        crate::mir::builder::lexical_call_projection_forwarded_fixture(package, BasicBlockId(0));
    let owner = row.call_site().owner();
    let source = ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    let binary = source.definitions[&owner]
        .uses
        .iter()
        .find_map(|row| match &row.kind {
            Use::CompareOperand { binary, .. } => Some(binary.clone()),
            _ => None,
        })
        .unwrap();
    let compare = ledger
        .prepare_borrowed_compare_source_v1(owner, &binary, Some(crate::mir::CompareOp::Gt))
        .unwrap()
        .unwrap();
    (ledger, owner, compare)
}
#[test]
fn borrowed_compare_carrier_loan_shares_original_alias_chain_without_integer_grant() {
    let (ledger, owner, compare) = compare_fixture();
    let loans = ledger.borrow_compare_carrier_operands_v1(&compare).unwrap();
    let second = ledger.borrow_compare_carrier_operands_v1(&compare).unwrap();
    assert_eq!(loans.len(), 1);
    assert_eq!(loans[0].owner(), owner);
    assert_eq!(loans[0].binary(), compare.site());
    assert_eq!(loans[0].site(), compare.operand_sites().0);
    assert_eq!(loans[0].entry_value(), ValueId(72));
    assert_eq!(loans[0].value(), ValueId(73));
    let copies = loans[0].original_copies().unwrap();
    assert_eq!(copies.len(), 1);
    assert!(matches!(
        copies[0].1,
        MirInstruction::Copy {
            dst: ValueId(73),
            src: ValueId(72)
        }
    ));
    assert!(
        std::ptr::eq(copies, second[0].original_copies().unwrap()),
        "same original immutable alias evidence"
    );
    ledger
        .verify_compare_carrier_operand_loan_v1(&compare, &loans[0])
        .unwrap();
}
#[test]
fn borrowed_compare_carrier_loan_refuses_foreign_owner_and_changed_entry() {
    let (ledger, owner, compare) = compare_fixture();
    let loans = ledger.borrow_compare_carrier_operands_v1(&compare).unwrap();
    let (foreign, _, foreign_compare) = compare_fixture();
    assert!(foreign
        .borrow_compare_carrier_operands_v1(&compare)
        .is_err());
    assert!(ledger
        .verify_compare_carrier_operand_loan_v1(&foreign_compare, &loans[0])
        .is_err());
    ledger
        .borrowed_entry_values
        .borrow_mut()
        .get_mut(&owner)
        .unwrap()
        .values
        .as_mut()
        .unwrap()[0]
        .2 = ValueId(999);
    assert!(ledger
        .verify_compare_carrier_operand_loan_v1(&compare, &loans[0])
        .is_err());
}
#[test]
fn borrowed_compare_carrier_loan_refuses_replaced_equal_alias_record() {
    let (ledger, owner, compare) = compare_fixture();
    let loans = ledger.borrow_compare_carrier_operands_v1(&compare).unwrap();
    let mut entries = ledger.borrowed_entry_values.borrow_mut();
    let entry = entries.get_mut(&owner).unwrap();
    let binding = *entry.aliases.keys().next().unwrap();
    let old = &entry.aliases[&binding];
    let replacement = Rc::new(BorrowedAliasMaterializationV1 {
        site: old.site.clone(),
        declaration: old.declaration.clone(),
        formal: old.formal,
        source_binding: old.source_binding,
        value: old.value,
        proof: old.proof.clone(),
    });
    entry.aliases.insert(binding, replacement);
    drop(entries);
    assert!(ledger
        .verify_compare_carrier_operand_loan_v1(&compare, &loans[0])
        .unwrap_err()
        .contains("carrier-identity"));
}

#[test]
fn borrowed_compare_carrier_loan_keeps_each_source_side_with_shared_values() {
    for condition in [
        "0 > alias",
        "q > 0",
        "q > q",
        "alias > alias",
        "alias > q",
        "q > alias",
    ] {
        let (ledger, owner, compare) = compare_fixture_for(condition);
        let loans = ledger.borrow_compare_carrier_operands_v1(&compare).unwrap();
        let expected = if condition.contains('0') { 1 } else { 2 };
        assert_eq!(loans.len(), expected, "{condition}");
        for loan in &loans {
            assert_eq!(loan.owner(), owner);
            assert!(
                loan.site() == compare.operand_sites().0
                    || loan.site() == compare.operand_sites().1
            );
            ledger
                .verify_compare_carrier_operand_loan_v1(&compare, loan)
                .unwrap();
            if loan.value() == loan.entry_value() {
                assert!(loan.original_copies().unwrap().is_empty());
            }
        }
        if expected == 2 {
            assert_ne!(loans[0].site(), loans[1].site());
            if condition == "q > q" || condition == "alias > alias" {
                assert_eq!(
                    loans[0].value(),
                    loans[1].value(),
                    "both source uses share one carrier"
                );
            }
        }
        if condition == "0 > alias" {
            assert_eq!(loans[0].site(), compare.operand_sites().1);
        }
    }
}
