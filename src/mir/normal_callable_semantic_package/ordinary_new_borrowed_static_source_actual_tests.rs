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

const TEXT: &str = "static box Layout { pick(p) { local alias: i64 = p if p <= 0 { return 0 } return 1 } } box Heap { lookup(size) { local k = Layout.pick(size) return 0 } } static box Main { main() { local heap = new Heap() local k = heap.lookup(7) local a = Layout.pick(8) return 0 } }";

#[test]
fn current_owner_nonopaque_actual_retains_original_source_without_entry() {
    let package = package("static box Layout { pick(p: i64): i64 { return p + 1 } run(): i64 { local k = me.pick(8) return k } } static box Main { main() { return 0 } }");
    let ingress = package
        .ordinary_new_claim_ledger
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
        .find(|row| row.target().name() == "pick" && row.current_owner_source().is_some())
        .expect("original CurrentOwner inventory source");
    let actual = BorrowedCallActualCandidateV1 {
        ordinal: 0,
        site: original.argument_sites()[0].clone(),
        value: BorrowedCallActualValueV1::Integer(8),
    };
    let rows = prepare_static_source_actuals_v1(
        ingress,
        &package.parameter_contracts,
        original.call_site(),
        &[actual.clone()],
    )
    .unwrap()
    .expect("source-only CurrentOwner actuals");
    let BorrowedCallActualEvidencePhaseV1::SourceStatic(identity) = &rows.phase else {
        panic!("no executable phase");
    };
    assert!(Rc::ptr_eq(&identity.source, original));
    assert!(rows.require_executable_v1().is_err());
    assert!(matches!(
        rows.ordered_arguments.as_ref(),
        [LocalCallArgumentV1::Integer(8)]
    ));
    let wrong = BorrowedCallActualCandidateV1 {
        value: BorrowedCallActualValueV1::Bool(true),
        ..actual
    };
    assert!(prepare_static_source_actuals_v1(
        ingress,
        &package.parameter_contracts,
        original.call_site(),
        &[wrong],
    )
    .unwrap_err()
    .contains("nonopaque-integer-unproved"));
}

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

const CLOSED_TEXT: &str = "static box Layout { pick(p) { if p <= 0 { return 0 } return 1 } } box Heap { lookup(size) { local k = Layout.pick(size) return 0 } } static box Main { main() { local heap = new Heap() local k = heap.lookup(7) local a = Layout.pick(8) return 0 } }";

#[test]
fn static_entry_profile_closes_original_incoming_and_retains_completion() {
    let package = package(CLOSED_TEXT);
    let claims = &package.source_static_claims_for_test;
    let ledger = &package.ordinary_new_claim_ledger;
    let source = ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    let fact = source.static_arguments.values().next().unwrap();
    assert!(source
        .definitions
        .contains_key(&fact.call_source().callee_owner()));
    assert!(source.definitions.contains_key(&fact.call().owner()));
    assert_eq!(
        source
            .incoming
            .iter()
            .filter(|row| row.callee == fact.call_source().callee_owner())
            .count(),
        2
    );
    let actuals = ledger.borrowed_formal_actuals[fact.call()]
        .as_ref()
        .unwrap();
    actuals.require_executable_v1().unwrap();
    let (claim, _) = claims
        .claim_target(fact.call_source().caller(), fact.call().site())
        .unwrap();
    let arguments = project_pending_static_source_arguments_v1(
        ledger.borrowed_formal_source.as_ref().unwrap(),
        &ledger.borrowed_formal_actuals,
        fact.call(),
        claim,
    )
    .unwrap()
    .unwrap();
    let observation = ledger
        .local_call_for_owner(fact.call().owner(), fact.call().site())
        .unwrap();
    assert_eq!(arguments.as_ref(), observation.arguments());
    for call in source
        .incoming
        .iter()
        .filter(|row| row.callee == fact.call_source().callee_owner())
    {
        let super::super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(
            original,
        ) = &call.source
        else {
            panic!("original Static cohort");
        };
        let (claim, _) = claims
            .claim_target(original.caller(), call.call.site())
            .unwrap();
        let projected = project_pending_static_source_arguments_v1(
            ledger.borrowed_formal_source.as_ref().unwrap(),
            &ledger.borrowed_formal_actuals,
            &call.call,
            claim,
        )
        .unwrap()
        .unwrap();
        let flow = ledger
            .local_call_for_owner(call.call.owner(), call.call.site())
            .expect("every original Static caller retains canonical flow");
        assert_eq!(projected.as_ref(), flow.arguments());
        assert!(matches!(
            projected.as_ref(),
            [LocalCallArgumentV1::BorrowedActual { ordinal: 0, .. }]
        ));
    }
    assert!(!ledger
        .borrowed_ordinary_entry_receiver_required_v1(fact.call_source().callee_owner())
        .unwrap());
    assert!(ledger
        .borrowed_ordinary_entry_receiver_required_v1(fact.call().owner())
        .unwrap());
}

#[test]
fn static_entry_main_literal_retains_only_canonical_local_flow_without_home() {
    let package = package("static box Layout { pick(p) { return 0 } } static box Main { main() { local k = Layout.pick(7) return 0 } }");
    let claims = &package.source_static_claims_for_test;
    let ledger = &package.ordinary_new_claim_ledger;
    let source = ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    assert!(
        source.static_arguments.is_empty(),
        "no formal-root fact in parameterless Main"
    );
    let call = source
        .incoming
        .iter()
        .find(|call| call.source.instance().is_none())
        .unwrap();
    let super::super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(
        original,
    ) = &call.source
    else {
        unreachable!()
    };
    let (claim, _) = claims
        .claim_target(original.caller(), call.call.site())
        .unwrap();
    let arguments = project_pending_static_source_arguments_v1(
        ledger.borrowed_formal_source.as_ref().unwrap(),
        &ledger.borrowed_formal_actuals,
        &call.call,
        claim,
    )
    .unwrap()
    .unwrap();
    let flow = ledger
        .local_call_for_owner(call.call.owner(), call.call.site())
        .unwrap();
    assert_eq!(arguments.as_ref(), flow.arguments());
    assert!(flow.prior_homes().is_empty());
}

#[test]
fn static_entry_transports_integer_bool_null_without_promoting_integer_agreement() {
    let package = package("static box Layout { pick(p) { return 0 } } static box Main { main() { local a = Layout.pick(-7) local b = Layout.pick(true) local c = Layout.pick(null) return 0 } }");
    let ledger = &package.ordinary_new_claim_ledger;
    let source = ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    assert_eq!(source.incoming.len(), 3);
    let owner = source.incoming[0].callee;
    let formal = source.incoming[0].arguments[0].2;
    assert!(!source.formal_integer_agreement(formal));
    let mut tags = std::collections::BTreeSet::new();
    for call in &source.incoming {
        let actuals = ledger.borrowed_formal_actuals[&call.call].as_ref().unwrap();
        actuals.require_executable_v1().unwrap();
        tags.insert(match actuals.opaque_actuals[0].source {
            BorrowedFormalActualSourceV1::Integer(-7) => 1,
            BorrowedFormalActualSourceV1::Bool(true) => 2,
            BorrowedFormalActualSourceV1::Null => 3,
            ref other => panic!("unexpected original actual {other:?}"),
        });
        assert!(ledger
            .local_call_for_owner(call.call.owner(), call.call.site())
            .is_some());
    }
    assert_eq!(tags, std::collections::BTreeSet::from([1, 2, 3]));
    assert!(!ledger
        .borrowed_ordinary_entry_receiver_required_v1(owner)
        .unwrap());
}

#[test]
fn static_entry_refuses_same_shaped_reissued_incoming_rc() {
    let mut package = package("static box Layout { pick(p) { return 0 } } static box Main { main() { local a = Layout.pick(7) return 0 } }");
    let main = super::super::super::borrow_app_main_source_v1(
        package.batch(),
        package.catalog.catalog().source_backed_app_main(),
    )
    .unwrap()
    .unwrap();
    let retained = Rc::clone(
        package
            .ordinary_new_claim_ledger
            .borrowed_formal_source
            .as_ref()
            .unwrap()
            .as_ref()
            .unwrap()
            .source_incoming
            .static_observations()
            .values()
            .next()
            .unwrap()
            .as_ref()
            .unwrap(),
    );
    let reissued = package
        .batch()
        .with_lowering_input(main.batch_slot(), |input| {
            let (_, call) = input.function().method_calls().next().unwrap();
            package
                .source_static_claims_for_test
                .incoming_source(
                    main.catalog_key(),
                    retained.call_site(),
                    call,
                    &package.selected,
                    &package.parameter_contracts,
                    Some(&main),
                )
                .unwrap()
                .unwrap()
                .retain()
        })
        .unwrap();
    assert!(!Rc::ptr_eq(&retained, &reissued));
    drop(main);
    let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    let source = ledger
        .borrowed_formal_source
        .as_mut()
        .unwrap()
        .as_mut()
        .unwrap();
    let call = &mut source.incoming[0];
    call.source =
        super::super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(
            reissued,
        );
    let site = call.call.clone();
    let actual = BorrowedCallActualCandidateV1 {
        ordinal: 0,
        site: call.arguments[0].1.clone(),
        value: BorrowedCallActualValueV1::Integer(7),
    };
    let error = prepare_borrowed_call_actuals_v1(
        ledger.borrowed_formal_source.as_ref().unwrap(),
        &package.parameter_contracts,
        &site,
        &[actual],
        &[],
        None,
        &mut |_| None,
    )
    .unwrap_err();
    assert!(error.contains("executable-source-identity"), "{error}");
    let (claim, _) = package
        .source_static_claims_for_test
        .claim_target(retained.caller(), site.site())
        .unwrap();
    assert!(project_pending_static_source_arguments_v1(
        ledger.borrowed_formal_source.as_ref().unwrap(),
        &ledger.borrowed_formal_actuals,
        &site,
        claim
    )
    .unwrap_err()
    .contains("executable-source-identity"));
}

#[test]
fn static_entry_installed_loan_retains_all_actuals_and_physical_source() {
    let package = package("static box Layout { pick(p) { return 0 } } static box Main { main() { local a = Layout.pick(-7) local b = Layout.pick(true) local c = Layout.pick(null) return 0 } }");
    let ledger = Rc::clone(&package.ordinary_new_claim_ledger);
    let owner = ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap()
        .incoming[0]
        .callee;
    let slot = package
        .parameter_contracts
        .iter()
        .find(|row| row.owner == owner)
        .unwrap()
        .batch_slot;
    let key = package.selected.key_for_batch_slot(slot).unwrap().clone();
    let completion = Rc::clone(ledger.completion_index[&owner].as_ref().unwrap());
    assert_eq!(completion.owner(), owner);
    let mut context = crate::mir::builder::CompilationContext::new();
    let installed = package.prepare_install(&mut context).unwrap().commit();
    installed
        .begin_lowering(&context)
        .unwrap()
        .with_selected_lowering_input(&key, |input| {
            let entry = ledger
                .borrowed_ordinary_entry_source_v1(&input)
                .unwrap()
                .unwrap();
            assert_eq!(entry.owner(), owner);
            assert_eq!(entry.incoming().len(), 3);
            assert_eq!(entry.formals().len(), 1);
            assert!(Rc::ptr_eq(
                &completion,
                ledger.completion_index[&owner].as_ref().unwrap()
            ));
            assert!(entry
                .incoming_targets()
                .all(|target| target.unwrap().target().namespace()
                    == crate::mir::builder::SameModuleCallableNamespaceV1::StaticBoxMethod));
        })
        .unwrap();
}

#[test]
fn static_entry_mode_lender_rejects_mixed_original_incoming_modes() {
    let mut package = package(CLOSED_TEXT);
    let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    let source = ledger
        .borrowed_formal_source
        .as_mut()
        .unwrap()
        .as_mut()
        .unwrap();
    let instance = source
        .incoming
        .iter()
        .find(|row| row.source.instance().is_some())
        .unwrap()
        .source
        .clone();
    let call = source
        .incoming
        .iter_mut()
        .find(|row| row.source.instance().is_none())
        .unwrap();
    let owner = call.callee;
    call.source = instance;
    assert!(ledger
        .borrowed_ordinary_entry_receiver_required_v1(owner)
        .unwrap_err()
        .contains("incoming-mode-drift"));
}
