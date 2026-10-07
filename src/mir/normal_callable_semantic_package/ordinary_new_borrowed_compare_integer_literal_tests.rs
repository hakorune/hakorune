//! Original literal source membership and same-entry physical observations.
use super::*;
use crate::mir::builder::MirBuilder;
use crate::mir::resolved_semantics::SourcePathV1;
use std::rc::Rc;

fn fixture() -> (
    Rc<OrdinaryNewClaimLedgerV1>,
    FunctionOwnerIdV1,
    OwnedExprSiteV1,
) {
    let text = "box Transport { birth() {} probe(p): i64 { if p > 0 { return 1 } if 0 > p { return 2 } return 0 } } static box Main { main() { local recv = new Transport() local out = recv.probe(5) return 0 } }";
    let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(text).unwrap();
    let ledger = package.ordinary_new_claim_ledger.clone();
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
    let site = source.definitions[&owner]
        .uses
        .iter()
        .find_map(|row| match &row.kind {
            Use::CompareOperand { source, .. } => {
                source.integer_literal().map(|(site, _)| site.clone())
            }
            _ => None,
        })
        .unwrap();
    (ledger, owner, site)
}

fn builder() -> MirBuilder {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("original_compare_literal/0".into());
    builder
}

#[test]
fn borrowed_literal_original_completion_records_once_and_allows_real_reevaluation() {
    let (ledger, owner, site) = fixture();
    let mut builder = builder();
    let first =
        crate::mir::builder::emission::constant::emit_integer_recorded(&mut builder, 0).unwrap();
    let loan = ledger
        .prepare_borrowed_compare_integer_literal_v1(owner, &site, 0)
        .unwrap()
        .unwrap();
    ledger
        .record_borrowed_compare_integer_literal_v1(loan, &first)
        .unwrap();
    let duplicate = ledger
        .prepare_borrowed_compare_integer_literal_v1(owner, &site, 0)
        .unwrap()
        .unwrap();
    assert!(ledger
        .record_borrowed_compare_integer_literal_v1(duplicate, &first)
        .unwrap_err()
        .contains("duplicate-materialization"));
    let second =
        crate::mir::builder::emission::constant::emit_integer_recorded(&mut builder, 0).unwrap();
    let loan = ledger
        .prepare_borrowed_compare_integer_literal_v1(owner, &site, 0)
        .unwrap()
        .unwrap();
    ledger
        .record_borrowed_compare_integer_literal_v1(loan, &second)
        .unwrap();
    let mut values = Vec::new();
    ledger
        .with_borrowed_ordinary_compare_integer_literals_v1(
            owner,
            |_, observed_site, value, original| {
                assert_eq!(observed_site, &site);
                assert!(builder
                    .current_function_instructions()
                    .contains(&original.1));
                assert_eq!(original.0, builder.current_function_entry_block().unwrap());
                values.push(value);
                Ok(())
            },
        )
        .unwrap();
    assert_eq!(values, vec![first.value(), second.value()]);
}

#[test]
fn borrowed_literal_refuses_source_payload_foreign_owner_and_unrelated_site() {
    let (ledger, owner, site) = fixture();
    assert!(ledger
        .prepare_borrowed_compare_integer_literal_v1(owner, &site, 7)
        .is_err());
    let unrelated = OwnedExprSiteV1::new(owner, SourcePathV1::root_body(99).expr());
    assert!(ledger
        .prepare_borrowed_compare_integer_literal_v1(owner, &unrelated, 0)
        .unwrap()
        .is_none());
    let mut issuer =
        crate::mir::resolved_semantics::FunctionOwnerIssuerV1::new_for_compilation().unwrap();
    let foreign_owner = issuer.issue().unwrap();
    let foreign = OwnedExprSiteV1::new(foreign_owner, site.site().clone());
    assert!(ledger
        .prepare_borrowed_compare_integer_literal_v1(owner, &foreign, 0)
        .is_err());
}

#[test]
fn borrowed_literal_wrong_append_payload_and_missing_entry_are_refused() {
    let (ledger, owner, site) = fixture();
    let mut builder = builder();
    let wrong =
        crate::mir::builder::emission::constant::emit_integer_recorded(&mut builder, 9).unwrap();
    let loan = ledger
        .prepare_borrowed_compare_integer_literal_v1(owner, &site, 0)
        .unwrap()
        .unwrap();
    assert!(ledger
        .record_borrowed_compare_integer_literal_v1(loan, &wrong)
        .unwrap_err()
        .contains("physical-identity"));
    assert!(ledger.borrowed_entry_values.borrow()[&owner]
        .integer_literals
        .is_empty());
    let loan = ledger
        .prepare_borrowed_compare_integer_literal_v1(owner, &site, 0)
        .unwrap()
        .unwrap();
    ledger.borrowed_entry_values.borrow_mut().remove(&owner);
    assert!(ledger
        .prepare_borrowed_compare_integer_literal_v1(owner, &site, 0)
        .unwrap_err()
        .contains("entry-values-missing"));
    assert!(ledger
        .record_borrowed_compare_integer_literal_v1(loan, &wrong)
        .is_err());
}

#[test]
fn borrowed_literal_final_loan_rejects_same_value_different_source_and_changed_dst() {
    for change_site in [false, true] {
        let (ledger, owner, site) = fixture();
        let mut builder = builder();
        let completed =
            crate::mir::builder::emission::constant::emit_integer_recorded(&mut builder, 0)
                .unwrap();
        let loan = ledger
            .prepare_borrowed_compare_integer_literal_v1(owner, &site, 0)
            .unwrap()
            .unwrap();
        ledger
            .record_borrowed_compare_integer_literal_v1(loan, &completed)
            .unwrap();
        let other_site = {
            let source = ledger
                .borrowed_formal_source
                .as_ref()
                .unwrap()
                .as_ref()
                .unwrap();
            source.definitions[&owner]
                .uses
                .iter()
                .find_map(|row| match &row.kind {
                    Use::CompareOperand { source, .. } => source
                        .integer_literal()
                        .filter(|(other, _)| *other != &site)
                        .map(|(site, _)| site.clone()),
                    _ => None,
                })
                .unwrap()
        };
        let mut entries = ledger.borrowed_entry_values.borrow_mut();
        let record = entries
            .get_mut(&owner)
            .unwrap()
            .integer_literals
            .get_mut(&completed.value())
            .unwrap();
        if change_site {
            record.source.site = other_site;
        } else if let MirInstruction::Const { dst, .. } = &mut record.original.1 {
            *dst = ValueId(999);
        }
        drop(entries);
        let mut published = 0;
        assert!(ledger
            .with_borrowed_ordinary_compare_integer_literals_v1(owner, |_, _, _, _| {
                published += 1;
                Ok(())
            })
            .is_err());
        assert_eq!(published, 0);
    }
}
