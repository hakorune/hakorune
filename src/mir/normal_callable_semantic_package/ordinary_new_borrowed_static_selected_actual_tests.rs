use super::*;

#[test]
fn selected_static_cohort_forwarded_actuals_reject_staged_candidate_drift() {
    let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(
        "static box Size { norm(size) { if size <= 0 { return 1 } return size } run(size) { local n = me.norm(size) return 0 } } static box Layout { pass(size) { return Size.norm(size) } } static box Main { main() { return 0 } }",
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
    let cohort = ingress.static_incoming_cohort_v1(original).unwrap();
    assert_eq!(cohort.len(), 2);
    let fact = ingress
        .static_arguments
        .get(&(original.call_site().clone(), 0))
        .unwrap();
    let receipt = issue_original_static_forwarded_actual_v1(
        ingress,
        &ledger.borrowed_formal_actuals,
        original,
        fact.formal(),
    )
    .unwrap();
    assert!(receipt.corroborates(original, fact.formal()));
    for source in &cohort {
        let fact = ingress
            .static_arguments
            .get(&(source.call_site().clone(), 0))
            .unwrap();
        assert!(issue_original_static_forwarded_actual_v1(
            ingress,
            &ledger.borrowed_formal_actuals,
            source,
            fact.formal(),
        )
        .unwrap()
        .corroborates(source, fact.formal()));
    }

    let mut changed = ledger.borrowed_formal_actuals.clone();
    let sibling = cohort.iter().find(|row| row.is_qualified()).unwrap();
    let staged = changed
        .get_mut(sibling.call_site())
        .unwrap()
        .as_mut()
        .unwrap();
    let BorrowedCallActualEvidencePhaseV1::SourceStatic(identity) = &mut staged.phase else {
        panic!("original source phase");
    };
    identity.candidates[0].value = BorrowedCallActualValueV1::Bool(true);
    let sibling_formal = ingress
        .static_arguments
        .get(&(sibling.call_site().clone(), 0))
        .unwrap()
        .formal();
    assert!(
        issue_original_static_forwarded_actual_v1(ingress, &changed, sibling, sibling_formal,)
            .unwrap_err()
            .contains("selected-forwarded-actual-unavailable")
    );
}
