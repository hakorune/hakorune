//! Same source port retains Static symbolic actuals without executable proofs.
fn package(
    source: &str,
) -> crate::mir::normal_callable_semantic_package::VerifiedNormalCallableSemanticPackageV1 {
    crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(
        source,
    )
    .unwrap()
}
use super::*;

const TEXT: &str = "static box Layout { pick(p) { return 0 } } box Heap { lookup(size) { local k = Layout.pick(size) return 0 } } static box Main { main() { local heap = new Heap() local k = heap.lookup(7) local a = Layout.pick(8) return 0 } }";

#[test]
fn static_source_port_retains_original_forward_and_denies_executable() {
    let package = package(TEXT);
    let claims = &package.source_static_claims_for_test;
    let ledger = &package.ordinary_new_claim_ledger;
    let ingress = ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    let fact = ingress.static_arguments.values().next().unwrap();
    let rows = ledger
        .borrowed_formal_actuals
        .get(fact.call())
        .expect("same production Observe probe stages Static source")
        .as_ref()
        .unwrap();
    let BorrowedCallActualEvidencePhaseV1::SourceStatic(identity) = &rows.phase else {
        panic!("Static source phase");
    };
    assert!(Rc::ptr_eq(&identity.source, fact.retained_call_source()));
    assert!(rows.opaque_actuals.is_empty());
    assert!(rows
        .require_executable_v1()
        .unwrap_err()
        .contains("source-only-static-actuals"));
    let (claim, _) = claims
        .claim_target(fact.call_source().caller(), fact.call().site())
        .unwrap();
    let projected = project_pending_static_source_arguments_v1(
        ledger.borrowed_formal_source.as_ref().unwrap(),
        &ledger.borrowed_formal_actuals,
        fact.call(),
        claim,
    )
    .unwrap()
    .unwrap();
    assert_eq!(projected.as_ref(), rows.ordered_arguments.as_ref());
    assert!(
        matches!(projected.as_ref(), [LocalCallArgumentV1::BorrowedActual { ordinal: 0, site }] if site == fact.use_site().site())
    );
    assert!(!ingress.definitions.contains_key(&fact.call().owner()));
    assert!(!ingress
        .definitions
        .contains_key(&fact.call_source().callee_owner()));
    assert!(
        ingress.incoming.is_empty(),
        "no fake transport definition/incoming"
    );
    assert!(
        project_pending_borrowed_i64_arguments_v1(
            ledger.borrowed_formal_source.as_ref().unwrap(),
            &ledger.borrowed_formal_actuals,
            &ledger.borrowed_i64_results,
            fact.call(),
        )
        .unwrap()
        .is_none(),
        "Static source cannot become ScalarArguments"
    );
}

#[test]
fn static_source_port_checks_required_integer_and_exact_forward_identity() {
    let package = package(&TEXT.replacen("return 0", "return p + 1", 1));
    let claims = &package.source_static_claims_for_test;
    let ledger = &package.ordinary_new_claim_ledger;
    let ingress = ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    let fact = ingress.static_arguments.values().next().unwrap();
    let (claim, _) = claims
        .claim_target(fact.call_source().caller(), fact.call().site())
        .unwrap();
    assert_eq!(claim.required_i64_arguments(), &[0]);
    let actual = BorrowedCallActualCandidateV1 {
        ordinal: fact.ordinal(),
        site: fact.use_site().site().clone(),
        value: BorrowedCallActualValueV1::SelfRooted {
            binding: fact.binding(),
            root: fact.formal(),
        },
    };
    for mutation in 0..5 {
        let mut candidate = actual.clone();
        match mutation {
            0 => candidate.value = BorrowedCallActualValueV1::Bool(true),
            1 => candidate.ordinal = 1,
            2 => candidate.site = fact.call().site().clone(),
            3 => {
                candidate.value = BorrowedCallActualValueV1::SelfRooted {
                    binding: fact.binding(),
                    root: fact.target_formal(),
                }
            }
            _ => candidate.value = BorrowedCallActualValueV1::Unknown,
        }
        let mut pending = PendingBorrowedFormalActualsV1::new();
        stage_borrowed_call_actuals_v1(
            &mut pending,
            fact.call(),
            prepare_static_source_actuals_v1(
                ingress,
                &package.parameter_contracts,
                fact.call(),
                &[candidate],
            ),
        );
        assert!(
            project_pending_static_source_arguments_v1(
                ledger.borrowed_formal_source.as_ref().unwrap(),
                &pending,
                fact.call(),
                claim
            )
            .is_err(),
            "mutation {mutation}"
        );
    }
    let mut pending = PendingBorrowedFormalActualsV1::new();
    stage_borrowed_call_actuals_v1(
        &mut pending,
        fact.call(),
        prepare_static_source_actuals_v1(
            ingress,
            &package.parameter_contracts,
            fact.call(),
            &[actual],
        ),
    );
    assert!(project_pending_static_source_arguments_v1(
        ledger.borrowed_formal_source.as_ref().unwrap(),
        &pending,
        fact.call(),
        claim
    )
    .unwrap()
    .is_some());
}

#[test]
fn static_source_port_repeated_observation_and_failed_walk_revoke_source() {
    let package = package(TEXT);
    let claims = &package.source_static_claims_for_test;
    let ledger = &package.ordinary_new_claim_ledger;
    let ingress = ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    let fact = ingress.static_arguments.values().next().unwrap();
    let (claim, _) = claims
        .claim_target(fact.call_source().caller(), fact.call().site())
        .unwrap();
    let first = BorrowedCallActualCandidateV1 {
        ordinal: fact.ordinal(),
        site: fact.use_site().site().clone(),
        value: BorrowedCallActualValueV1::SelfRooted {
            binding: fact.binding(),
            root: fact.formal(),
        },
    };
    let mut pending = PendingBorrowedFormalActualsV1::new();
    for _ in 0..2 {
        stage_borrowed_call_actuals_v1(
            &mut pending,
            fact.call(),
            prepare_static_source_actuals_v1(
                ingress,
                &package.parameter_contracts,
                fact.call(),
                &[first.clone()],
            ),
        );
    }
    assert!(
        pending[fact.call()].is_ok(),
        "same source visit is idempotent"
    );
    let changed = BorrowedCallActualCandidateV1 {
        value: BorrowedCallActualValueV1::Bool(true),
        ..first
    };
    stage_borrowed_call_actuals_v1(
        &mut pending,
        fact.call(),
        prepare_static_source_actuals_v1(
            ingress,
            &package.parameter_contracts,
            fact.call(),
            &[changed],
        ),
    );
    assert!(pending[fact.call()]
        .as_ref()
        .unwrap_err()
        .contains("repeated-walk-drift"));
    let mut pending = ledger.borrowed_formal_actuals.clone();
    reject_borrowed_actuals_for_owner_v1(
        ledger.borrowed_formal_source.as_ref().unwrap(),
        fact.call().owner(),
        &mut pending,
        "original-walk-rejected".into(),
    );
    assert!(project_pending_static_source_arguments_v1(
        ledger.borrowed_formal_source.as_ref().unwrap(),
        &pending,
        fact.call(),
        claim
    )
    .unwrap_err()
    .contains("original-walk-rejected"));
}

#[test]
fn static_source_port_rejects_substituted_candidate_and_corrupted_ordered_view() {
    let package = package(TEXT);
    let claims = &package.source_static_claims_for_test;
    let ledger = &package.ordinary_new_claim_ledger;
    let ingress = ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    let fact = ingress.static_arguments.values().next().unwrap();
    let (claim, _) = claims
        .claim_target(fact.call_source().caller(), fact.call().site())
        .unwrap();
    for value in [
        BorrowedCallActualValueV1::Integer(7),
        BorrowedCallActualValueV1::Bool(true),
        BorrowedCallActualValueV1::Scalar(fact.target_formal(), SourceScalarKind::Integer),
    ] {
        let candidate = BorrowedCallActualCandidateV1 {
            ordinal: fact.ordinal(),
            site: fact.use_site().site().clone(),
            value,
        };
        assert!(prepare_static_source_actuals_v1(
            ingress,
            &package.parameter_contracts,
            fact.call(),
            &[candidate]
        )
        .unwrap_err()
        .contains("forward-source-identity"));
    }
    for argument in [
        LocalCallArgumentV1::Integer(7),
        LocalCallArgumentV1::BorrowedActual {
            ordinal: 1,
            site: fact.use_site().site().clone(),
        },
        LocalCallArgumentV1::BorrowedActual {
            ordinal: 0,
            site: fact.call().site().clone(),
        },
    ] {
        let mut pending = ledger.borrowed_formal_actuals.clone();
        pending
            .get_mut(fact.call())
            .unwrap()
            .as_mut()
            .unwrap()
            .ordered_arguments[0] = argument;
        assert!(project_pending_static_source_arguments_v1(
            ledger.borrowed_formal_source.as_ref().unwrap(),
            &pending,
            fact.call(),
            claim
        )
        .unwrap_err()
        .contains("source-projection-identity"));
    }
}

#[test]
fn static_source_local_retains_original_observation_in_canonical_completion() {
    let with_home = TEXT.replace("box Heap", "box Token {} box Heap").replace(
        "local k = Layout.pick(size)",
        "local token = new Token() local k = Layout.pick(size)",
    );
    for (text, home_count) in [(TEXT, 0), (with_home.as_str(), 1)] {
        let package = package(text);
        let ledger = &package.ordinary_new_claim_ledger;
        let source = ledger
            .borrowed_formal_source
            .as_ref()
            .unwrap()
            .as_ref()
            .unwrap();
        let fact = source.static_arguments.values().next().unwrap();
        let observation = ledger
            .local_call_for_owner(fact.call().owner(), fact.call().site())
            .expect("original Static local flows through canonical completion");
        assert_eq!(observation.owner(), fact.call().owner());
        assert_eq!(observation.site(), fact.call());
        let (_, destination) = observation
            .local_binding()
            .expect("original receiving local");
        assert_eq!(destination.owner(), fact.call().owner());
        assert_eq!(observation.prior_homes().len(), home_count);
        assert!(observation
            .prior_homes()
            .iter()
            .all(|home| home.owner() == fact.call().owner()));
        assert!(
            matches!(observation.arguments(), [LocalCallArgumentV1::BorrowedActual { ordinal: 0, site }] if site == fact.use_site().site())
        );
        assert!(ledger
            .local_call_for_owner(fact.call_source().callee_owner(), fact.call().site())
            .is_none());
        assert!(ledger
            .local_call_for_owner(fact.call().owner(), fact.use_site().site())
            .is_none());
        assert!(ledger.borrowed_formal_actuals[fact.call()]
            .as_ref()
            .unwrap()
            .require_executable_v1()
            .is_err());
        assert!(!source.definitions.contains_key(&fact.call().owner()));
        assert!(source.incoming.is_empty());
    }
}
