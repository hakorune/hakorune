//! Whole-cohort scalar closure and the sibling-removal refusal.
use super::*;
use crate::mir::ValueId;

const SOURCE: &str = "static box Size { mul(p) { if p <= 8 { return p * 2 } return 0 } first(x: usize) { return me.mul(x) } second() { local y = 4 return me.mul(y) } } static box Main { main() { return 0 } }";

#[test]
fn current_owner_scalar_callers_finish_together_and_lend_the_mul_entry() {
    let mut package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(SOURCE).unwrap();
    let ledger = &package.ordinary_new_claim_ledger;
    let ingress = ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    let owner = package
        .parameter_contracts
        .iter()
        .find(|row| {
            package
                .selected
                .key_for_batch_slot(row.batch_slot)
                .is_some_and(|key| {
                    matches!(key,
                SelectedNormalCallableKeyV1::Cataloged(key) if key.name() == "mul")
                })
        })
        .unwrap()
        .owner;
    assert!(ingress.source_only_definitions.contains_key(&owner));
    let cohort = ledger
        .checked_completed_static_scalar_cohort_v1(ingress, owner)
        .unwrap()
        .unwrap();
    assert_eq!(cohort.len(), 2);
    for call in cohort.iter() {
        assert!(ledger.selected_static_local_source_v1(&call.call).unwrap().is_some());
    }
    let contract = package
        .parameter_contracts
        .iter()
        .find(|row| row.owner == owner)
        .unwrap();
    ledger
        .record_borrowed_ordinary_entry_values_v1(
            owner,
            Ok(Box::new([(0, contract.parameters[0].binding, ValueId(73))])),
        )
        .unwrap();
    let mul = ingress.source_only_definitions[&owner].uses.iter()
        .find_map(|row| match &row.kind {
            super::super::super::borrowed_formal_uses::BorrowedFormalUseDraftKindV1::MulOperand { binary, .. } => Some(binary.clone()),
            _ => None,
        }).unwrap();
    assert!(ledger
        .prepare_borrowed_mul_source_v1(owner, &mul, Some(crate::mir::BinaryOp::Mul))
        .unwrap()
        .is_some());
    let removed = cohort[0].call.clone();
    drop(cohort);
    let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    let saved = ledger.borrowed_formal_actuals.remove(&removed).unwrap();
    let ingress = ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    assert!(ledger
        .checked_completed_static_scalar_cohort_v1(ingress, owner)
        .unwrap()
        .is_none());
    assert!(ledger
        .prepare_borrowed_mul_source_v1(owner, &mul, Some(crate::mir::BinaryOp::Mul))
        .unwrap_err()
        .contains("source-only-entry"));
    ledger
        .borrowed_formal_actuals
        .insert(removed.clone(), saved);
    let row = ledger
        .borrowed_formal_actuals
        .get_mut(&removed)
        .unwrap()
        .as_mut()
        .unwrap();
    row.opaque_actuals[0].source = BorrowedFormalActualSourceV1::Bool(true);
    let ingress = ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    assert!(ledger
        .checked_completed_static_scalar_cohort_v1(ingress, owner)
        .unwrap_err()
        .contains("actual-identity"));
}
