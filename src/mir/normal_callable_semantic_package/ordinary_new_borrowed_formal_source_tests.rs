use super::*;

fn package(
    body: &str,
    sink: &str,
    main: &str,
) -> crate::mir::normal_callable_semantic_package::VerifiedNormalCallableSemanticPackageV1 {
    let source = format!(
        "box Transport {{ birth() {{ }} probe(p): i64 {{ {body} }} \
         sink(q): i64 {{ {sink} }} }} static box Main {{ main() {{ {main} }} }}"
    );
    crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(
        &source,
    )
    .expect("source-backed ordinary package")
}

#[test]
fn package_issuer_prepares_ignored_formal_without_changing_source_kind() {
    let package = package(
        "return 0",
        "return 0",
        "local recv = new Transport() local out = recv.probe(0) return 0",
    );
    let prepared = package
        .ordinary_new_claim_ledger
        .borrowed_formal_source
        .as_ref()
        .expect("production source preparation")
        .as_ref()
        .expect("complete source cohort");
    assert_eq!(prepared.definitions.len(), 1);
    assert_eq!(prepared.incoming.len(), 1);
    assert!(prepared.forwards.is_empty());
    let callee = prepared.incoming[0].callee;
    let contract = package
        .parameter_contracts
        .iter()
        .find(|row| row.owner == callee)
        .unwrap();
    assert_eq!(
        contract.parameters[0].kind,
        CallableParameterContractKindV1::OpaqueHandle
    );
    assert!(prepared.definitions[&callee].uses.is_empty());
}

#[test]
fn finite_mutual_forwarding_is_prepared_by_the_real_package_issuer() {
    let package = package(
        "local alias = p local recv = new Transport() local out = recv.sink(alias) return 0",
        "local recv = new Transport() local out = recv.probe(q) return 0",
        "local recv = new Transport() local out = recv.probe(-1) return 0",
    );
    let prepared = package
        .ordinary_new_claim_ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .expect("complete finite graph");
    assert_eq!(prepared.definitions.len(), 2);
    assert_eq!(prepared.forwards.len(), 2);
    assert_eq!(prepared.incoming.len(), 3);
}

#[test]
fn legitimate_nontransport_use_is_outside_profile_before_selection() {
    for body in ["return p", "p = 1 return 0", "local a: i64 = p return 0"] {
        let package = package(
            body,
            "return 0",
            "local recv = new Transport() local out = recv.probe(0) return 0",
        );
        let prepared = package
            .ordinary_new_claim_ledger
            .borrowed_formal_source
            .as_ref()
            .unwrap()
            .as_ref()
            .expect("profile-outside is not source corruption");
        assert!(prepared.definitions.is_empty(), "body: {body}");
        assert!(prepared.incoming.is_empty());
    }
}

#[test]
fn forwarding_to_outside_profile_removes_dependent_cohort_before_selection() {
    let package = package(
        "local recv = new Transport() local out = recv.sink(p) return 0",
        "return q",
        "local recv = new Transport() local out = recv.probe(0) return 0",
    );
    let prepared = package
        .ordinary_new_claim_ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .expect("finite profile pruning");
    assert!(prepared.definitions.is_empty());
    assert!(prepared.forwards.is_empty());
}

#[test]
fn source_contract_identity_corruption_is_not_profile_outside() {
    let package = package(
        "return 0",
        "return 0",
        "local recv = new Transport() local out = recv.probe(0) return 0",
    );
    let slots = package
        .ordinary_new_claim_ledger
        .lexical_instance_calls
        .borrow();
    let prepared = Ok(slots
        .values()
        .filter_map(|slot| match slot {
            crate::mir::normal_callable_semantic_package::disposition_slot::DispositionSlotV1::Ready(row) => {
                Some(Ok(Some(row.source_target().clone())))
            }
            _ => None,
        })
        .collect());
    let contract = package
        .parameter_contracts
        .iter()
        .find(|row| row.parameters.len() == 1)
        .unwrap();
    let mut contracts = package
        .parameter_contracts
        .iter()
        .map(|row| OwnedCallableParameterContractDeclarationV1 {
            owner: row.owner,
            batch_slot: row.batch_slot,
            mode: row.mode,
            parameters: row
                .parameters
                .iter()
                .map(
                    |formal| crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractV1 {
                        ordinal: formal.ordinal,
                        binding: formal.binding,
                        kind: formal.kind,
                    },
                )
                .collect(),
        })
        .collect::<Vec<_>>();
    contracts
        .iter_mut()
        .find(|row| row.owner == contract.owner)
        .unwrap()
        .parameters[0]
        .ordinal = 1;
    let empty_loans =
        crate::mir::resolved_semantics::VerifiedInstanceEntryHomeCatalogV1::issue(Box::new([]));
    let error = prepare_borrowed_formal_ingress_v1(
        package.batch(),
        &package.selected,
        &contracts,
        &prepared,
        None,
        None,
        &empty_loans,
        package.instance_constructors(),
    )
    .expect_err("sealed identity mismatch");
    assert!(error.contains("borrowed-formal/source-identity"), "{error}");
}

#[test]
fn selected_cohort_unresolved_incoming_is_retained_as_named_error() {
    let source = "box Transport { birth() { } probe(p): i64 { local bad = me.probe(1) return 0 } sink(q): i64 { return 0 } } static box Main { main() { local recv = new Transport() local good = recv.probe(0) return 0 } }";
    let error = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(source).err().expect("unresolved selected incoming is terminal before continuation");
    let error = format!("{error:?}");
    assert!(
        error.contains("borrowed-formal/incoming-coverage"),
        "{error}"
    );
    assert!(error.contains("UnresolvedCaller"), "{error}");
}

#[test]
fn final_corroboration_rejects_changed_incoming_argument_ordinal() {
    let package = package(
        "return 0",
        "return 0",
        "local recv = new Transport() local out = recv.probe(0) return 0",
    );
    let original = package
        .ordinary_new_claim_ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    let caller = original.incoming[0].call.owner();
    let app_main_slot = package
        .batch()
        .declarations()
        .find(|row| row.owner() == caller)
        .unwrap()
        .batch_slot();
    let slots = package
        .ordinary_new_claim_ledger
        .lexical_instance_calls
        .borrow();
    let source_rows = slots.values().filter_map(|slot| match slot {
        crate::mir::normal_callable_semantic_package::disposition_slot::DispositionSlotV1::Ready(row) => Some(Ok(Some(row.source_target().clone()))),
        _ => None,
    }).collect::<Vec<_>>();
    let prepared = Ok(source_rows);
    let empty_loans =
        crate::mir::resolved_semantics::VerifiedInstanceEntryHomeCatalogV1::issue(Box::new([]));
    let mut borrowed = prepare_borrowed_formal_ingress_v1(
        package.batch(),
        &package.selected,
        &package.parameter_contracts,
        &prepared,
        Some(app_main_slot),
        None,
        &empty_loans,
        package.instance_constructors(),
    )
    .unwrap();
    borrowed
        .corroborate_source_targets(prepared.as_ref().unwrap())
        .unwrap();
    borrowed.incoming[0].arguments[0].0 = 1;
    let error = borrowed
        .corroborate_source_targets(prepared.as_ref().unwrap())
        .unwrap_err();
    assert!(
        error.contains("borrowed-formal/final-incoming-drift"),
        "{error}"
    );
}
