use super::*;

#[test]
fn selected_current_owner_forwarded_actual_rejects_staged_candidate_drift() {
    let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(
        "static box Size { norm(size) { if size <= 0 { return 1 } return size } run(size) { local n = me.norm(size) return 0 } } static box Main { main() { return 0 } }",
    )
    .unwrap();
    let ledger = &package.ordinary_new_claim_ledger;
    let ingress = ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    let original = ingress
        .source_incoming
        .static_observations()
        .values()
        .filter_map(|row| row.as_ref().ok())
        .find(|row| row.target().name() == "norm" && row.current_owner_source().is_some())
        .unwrap();
    let fact = ingress
        .static_arguments
        .get(&(original.call_site().clone(), 0))
        .unwrap();
    let receipt = issue_selected_current_owner_forwarded_actual_v1(
        ingress,
        &ledger.borrowed_formal_actuals,
        original,
        fact.formal(),
    )
    .unwrap();
    assert!(receipt.corroborates(original, fact.formal()));

    let mut changed = ledger.borrowed_formal_actuals.clone();
    let staged = changed
        .get_mut(original.call_site())
        .unwrap()
        .as_mut()
        .unwrap();
    let BorrowedCallActualEvidencePhaseV1::SourceStatic(identity) = &mut staged.phase else {
        panic!("original source phase");
    };
    identity.candidates[0].value = BorrowedCallActualValueV1::Bool(true);
    assert!(issue_selected_current_owner_forwarded_actual_v1(
        ingress,
        &changed,
        original,
        fact.formal(),
    )
    .unwrap_err()
    .contains("selected-forwarded-actual-unavailable"));
}
