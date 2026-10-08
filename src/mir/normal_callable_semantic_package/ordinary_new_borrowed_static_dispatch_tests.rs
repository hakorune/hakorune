//! Exact original fact membership keeps unrelated ingress failures local.
use super::*;
use std::collections::BTreeSet;

#[test]
fn static_dispatch_preserves_selected_error_and_unselected_literal_scope() {
    let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(
        "static box Layout { pick(p) { return 0 } } box Heap { lookup(size) { local k = Layout.pick(size) return 0 } } static box Main { main() { local heap = new Heap() local k = heap.lookup(7) local a = Layout.pick(8) return 0 } }"
    ).unwrap();
    let source = package
        .ordinary_new_claim_ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap();
    let ingress = source.as_ref().unwrap();
    let sites: BTreeSet<_> = ingress
        .static_arguments
        .keys()
        .map(|(site, _)| site.clone())
        .collect();
    assert_eq!(sites.len(), 1);
    let selected = sites.iter().next().unwrap();
    let main = package.catalog.catalog().source_backed_app_main().unwrap();
    let main_contract = package.parameter_contracts.iter().find(|row| row.parameters.is_empty()
        && row.mode == crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1::StaticBoxMethod).unwrap();
    let (sibling, sibling_claim) = package
        .batch()
        .with_lowering_input(main_contract.batch_slot, |input| {
            input
                .function()
                .method_calls()
                .find_map(|(site, _)| {
                    package
                        .source_static_claims_for_test
                        .claim_target(main.catalog_key(), site)
                        .map(|(claim, _)| {
                            (
                                OwnedExprSiteV1::new(input.owner(), site.clone()),
                                claim.clone(),
                            )
                        })
                })
                .unwrap()
        })
        .unwrap();
    let fact = ingress.static_arguments.values().next().unwrap();
    let (selected_claim, _) = package
        .source_static_claims_for_test
        .claim_target(fact.call_source().caller(), selected.site())
        .unwrap();
    let demand = |site: &OwnedExprSiteV1, source, selection| {
        let claim = if site == selected {
            selected_claim
        } else {
            &sibling_claim
        };
        let mut pending = BTreeMap::new();
        let result = borrowed_call_arguments_callback_v1(
            &Ok(Vec::new()),
            &package.parameter_contracts,
            &[],
            None,
            source,
            selection,
            &mut pending,
            &BTreeMap::new(),
            &mut |_| None,
            site,
            BorrowedCallActualRequestV1::QualifiedStaticSourceArguments(claim),
            &package.ordinary_new_claim_ledger.callable_result_classes,
            &package.ordinary_new_claim_ledger.receiver_call_observations,
        );
        assert!(pending.is_empty());
        result
    };
    for (site, expected) in [(selected, true), (&sibling, false)] {
        let contract = package
            .parameter_contracts
            .iter()
            .find(|row| row.owner == site.owner())
            .unwrap();
        package
            .batch()
            .with_lowering_input(contract.batch_slot, |input| {
                assert_eq!(
                    super::super::walk_triggers::has_static_source_local_v1(
                        input,
                        &Ok(sites.clone())
                    ),
                    expected
                );
            })
            .unwrap();
    }
    let failure = Err("ingress-sentinel".to_owned());
    let selection = Ok(sites.clone());
    assert!(
        matches!(demand(selected, &failure, &selection), Err(OrdinaryNewCoSealIssueV1::BorrowedFormalIngress { issue, .. }) if issue == "ingress-sentinel")
    );
    assert!(demand(&sibling, &failure, &selection).unwrap().is_none());
    assert!(
        matches!(demand(selected, source, &Err("selection-sentinel".into())), Err(OrdinaryNewCoSealIssueV1::BorrowedFormalIngress { issue, .. }) if issue == "selection-sentinel")
    );
}

#[test]
fn static_dispatch_selected_missing_fact_cannot_fall_back() {
    let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(
        "static box Layout { pick(p) { return 0 } } box Heap { lookup(size) { local k = Layout.pick(size) return 0 } } static box Main { main() { local heap = new Heap() local k = heap.lookup(7) return 0 } }"
    ).unwrap();
    let ingress = package
        .ordinary_new_claim_ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    let fact = ingress.static_arguments.values().next().unwrap();
    let site = fact.call().clone();
    let (claim, _) = package
        .source_static_claims_for_test
        .claim_target(fact.call_source().caller(), site.site())
        .unwrap();
    let claim = claim.clone();
    let foreign = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog("static box Main { main() { return 0 } }").unwrap();
    let missing = foreign
        .ordinary_new_claim_ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap();
    assert!(missing.as_ref().unwrap().static_arguments.is_empty());
    let result = borrowed_call_arguments_callback_v1(
        &Ok(Vec::new()),
        &package.parameter_contracts,
        &[],
        None,
        missing,
        &Ok(BTreeSet::from([site.clone()])),
        &mut BTreeMap::new(),
        &BTreeMap::new(),
        &mut |_| None,
        &site,
        BorrowedCallActualRequestV1::QualifiedStaticSourceArguments(&claim),
        &package.ordinary_new_claim_ledger.callable_result_classes,
        &package.ordinary_new_claim_ledger.receiver_call_observations,
    );
    assert!(
        matches!(result, Err(OrdinaryNewCoSealIssueV1::BorrowedFormalIngress { issue, .. }) if issue.contains("source-selection-identity"))
    );
}
