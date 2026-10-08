use super::*;

fn package() -> crate::mir::normal_callable_semantic_package::VerifiedNormalCallableSemanticPackageV1
{
    crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(
        "box Token {} box Maker { make(size) { if size > 0 { return new Token() } return new Token() } relay(size: i64) { return me.make(7) } } static box Main { main() { return 0 } }"
    ).unwrap()
}
fn original(
    package: &crate::mir::normal_callable_semantic_package::VerifiedNormalCallableSemanticPackageV1,
) -> (
    ObjectReturnCallQualificationV1,
    LexicalInstanceCallSourceTargetV1,
    super::super::super::borrowed_formal_uses::BorrowedIncomingCallDraftV1,
) {
    let ledger = &package.ordinary_new_claim_ledger;
    let key = crate::mir::builder::CanonicalSameModuleCallableKeyV1::instance_box_method(
        "Maker", "relay", 1,
    );
    let loan = ledger
        .callable_result_classes
        .object_return_qualification(
            ledger.callable_result_classes.outcomes(&key).unwrap()[0].site(),
        )
        .unwrap();
    let row = ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap()
        .source_incoming
        .exact_rows()
        .find(|row| row.call == *loan.call())
        .unwrap()
        .clone();
    let target = row.source.require_instance().unwrap().clone();
    (loan, target, row)
}

#[test]
fn received_object_arguments_require_original_self_receiver_before_missing_observation() {
    let package = package();
    let (loan, target, _) = original(&package);
    assert!(target.is_self_receiver());
    let receiver = target.receiver_binding().unwrap();
    assert!(corroborate_received_object_receiver_v1(
        &target,
        &loan,
        receiver,
        None,
        &BTreeMap::new()
    )
    .unwrap_err()
    .contains("object-receiver-identity"));
    assert!(!corroborate_received_object_receiver_v1(
        &target,
        &loan,
        receiver,
        Some(receiver),
        &BTreeMap::new()
    )
    .unwrap());
}

#[test]
fn object_arguments_reject_original_incoming_callee_drift_before_profile_unavailability() {
    let mut package = package();
    let (loan, target, mut row) = original(&package);
    let ledger = std::rc::Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    row.callee = loan.call().owner();
    assert_ne!(row.callee, target.callee_owner());
    let source = ledger.borrowed_formal_source.as_mut().unwrap();
    let ingress = source.as_mut().unwrap();
    ingress.incoming = vec![row].into_boxed_slice();
    ingress.definitions.clear();
    assert!(
        project_pending_object_arguments_v1(source, &Default::default(), &target, &loan)
            .unwrap_err()
            .contains("object-source-identity")
    );
}

#[test]
fn object_arguments_demand_staged_error_before_missing_incoming_support() {
    let package = package();
    let (loan, target, _) = original(&package);
    let source = package
        .ordinary_new_claim_ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap();
    let mut actuals = PendingBorrowedFormalActualsV1::new();
    actuals.insert(
        loan.call().clone(),
        Err("selected-original-actual-error".into()),
    );
    assert_eq!(
        project_pending_object_arguments_v1(source, &actuals, &target, &loan).unwrap_err(),
        "selected-original-actual-error"
    );
}

#[test]
fn canonical_object_target_attaches_exact_original_terminal_qualification() {
    let package = package();
    let (loan, target, _) = original(&package);
    let qualifications = target
        .object_return_sources()
        .expect("canonical attachment before inventory");
    assert_eq!(qualifications.len(), 1);
    assert_eq!(qualifications[0], loan);
    assert_eq!(target.call_site(), loan.call());
    assert_eq!(target.target(), loan.key());
    assert!(qualifications[0]
        .witnesses()
        .iter()
        .zip(loan.witnesses())
        .all(|(actual, original)| std::rc::Rc::ptr_eq(actual, original)));
    assert!(
        package
            .ordinary_new_claim_ledger
            .lexical_source_targets
            .is_none(),
        "the final affine target is consumed, not duplicated for this source loan"
    );
}

#[test]
fn original_object_source_actuals_retain_order_and_refuse_executable_permission() {
    let package = package();
    let (loan, target, original) = original(&package);
    let source = package
        .ordinary_new_claim_ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap();
    let actuals = vec![BorrowedCallActualCandidateV1 {
        ordinal: 0,
        site: target.argument_sites()[0].clone(),
        value: BorrowedCallActualValueV1::Integer(7),
    }];
    let ingress = source.as_ref().unwrap();
    let rows = super::super::object_source::prepare_object_source_actuals_v1(
        ingress,
        &package.parameter_contracts,
        loan.call(),
        &actuals,
    )
    .unwrap()
    .unwrap();
    assert!(rows.opaque_actuals.is_empty());
    assert!(rows
        .require_executable_v1()
        .unwrap_err()
        .contains("source-only-object-actuals"));
    let BorrowedCallActualEvidencePhaseV1::SourceObject(identity) = &rows.phase else {
        panic!("source object")
    };
    assert_eq!(identity.source, target);
    assert_eq!(identity.incoming_arguments, original.arguments);
    assert_eq!(identity.candidates.as_ref(), actuals.as_slice());
    let mut pending = PendingBorrowedFormalActualsV1::new();
    pending.insert(loan.call().clone(), Ok(rows));
    assert!(
        matches!(project_pending_object_arguments_v1(source, &pending, &target, &loan).unwrap(),
        ObjectCallSourceSupportV1::SourceOnly(arguments) if arguments.len() == 1)
    );
    reject_borrowed_actuals_for_owner_v1(
        source,
        loan.call().owner(),
        &mut pending,
        "failed-original-walk".into(),
    );
    assert!(
        project_pending_object_arguments_v1(source, &pending, &target, &loan)
            .unwrap_err()
            .contains("failed-original-walk")
    );
}

#[test]
fn object_source_actuals_reject_arity_ordinal_and_original_argument_site_drift() {
    let package = package();
    let (loan, target, _) = original(&package);
    let source = package
        .ordinary_new_claim_ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    for mutation in 0..3 {
        let mut actuals = vec![BorrowedCallActualCandidateV1 {
            ordinal: 0,
            site: target.argument_sites()[0].clone(),
            value: BorrowedCallActualValueV1::Integer(7),
        }];
        match mutation {
            0 => actuals.clear(),
            1 => actuals[0].ordinal = 1,
            2 => actuals[0].site = loan.call().site().clone(),
            _ => unreachable!(),
        }
        assert!(
            super::super::object_source::prepare_object_source_actuals_v1(
                source,
                &package.parameter_contracts,
                loan.call(),
                &actuals,
            )
            .unwrap_err()
            .contains("source-actual-identity")
        );
    }
}

#[test]
fn failed_object_source_walk_revokes_unobserved_raw_call_without_execution_grant() {
    let package = package();
    let (loan, target, _) = original(&package);
    let source = package
        .ordinary_new_claim_ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap();
    let mut pending = PendingBorrowedFormalActualsV1::new();
    reject_borrowed_actuals_for_owner_v1(
        source,
        loan.call().owner(),
        &mut pending,
        "unobserved-source-walk-failed".into(),
    );
    assert!(
        project_pending_object_arguments_v1(source, &pending, &target, &loan)
            .unwrap_err()
            .contains("unobserved-source-walk-failed")
    );
    let ingress = source.as_ref().unwrap();
    for (owner, draft) in &ingress.definitions {
        assert!(!ingress.source_only_definitions.contains_key(owner));
        assert!(std::ptr::eq(
            draft,
            ingress.source_definition_for(*owner).unwrap()
        ));
    }
    for (owner, draft) in &ingress.source_only_definitions {
        assert!(!ingress.definitions.contains_key(owner));
        assert!(std::ptr::eq(
            draft,
            ingress.source_definition_for(*owner).unwrap()
        ));
    }
}

#[test]
fn object_forward_source_keeps_final_callee_without_fabricating_caller_definition() {
    let mut package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(
        "box Token {} box Maker { make(size) { if size > 0 { return new Token() } return new Token() } relay(size) { return me.make(size) } } static box Main { main() { local maker = new Maker() local out = maker.relay(7) return 0 } }"
    ).unwrap();
    let (loan, target, row) = original(&package);
    let forward = target.object_source_forwards().unwrap()[0].clone();
    let actual = BorrowedCallActualCandidateV1 {
        ordinal: forward.ordinal(),
        site: forward.site().site().clone(),
        value: BorrowedCallActualValueV1::SelfRooted {
            binding: forward.binding(),
            root: forward.source_formal(),
        },
    };
    let ledger = std::rc::Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    let source = ledger.borrowed_formal_source.as_mut().unwrap();
    assert!(source
        .as_ref()
        .unwrap()
        .definitions
        .contains_key(&loan.call().owner()));
    let executable = prepare_borrowed_call_actuals_v1(
        source,
        &package.parameter_contracts,
        loan.call(),
        &[actual.clone()],
        &[],
        None,
        &mut |_| None,
    )
    .unwrap()
    .unwrap();
    executable.require_executable_v1().unwrap();
    source
        .as_mut()
        .unwrap()
        .definitions
        .remove(&loan.call().owner());
    assert!(source
        .as_ref()
        .unwrap()
        .incoming
        .iter()
        .any(|original| original.call == row.call));
    let pending = prepare_borrowed_call_actuals_v1(
        source,
        &package.parameter_contracts,
        loan.call(),
        &[actual.clone()],
        &[],
        None,
        &mut |_| None,
    )
    .unwrap()
    .unwrap();
    assert!(pending.opaque_actuals.is_empty());
    assert!(pending
        .require_executable_v1()
        .unwrap_err()
        .contains("source-only-object-actuals"));
    let mut staged = PendingBorrowedFormalActualsV1::new();
    staged.insert(loan.call().clone(), Ok(pending));
    assert!(matches!(
        project_pending_object_arguments_v1(source, &staged, &target, &loan).unwrap(),
        crate::mir::resolved_semantics::home_new_prefix::ObjectCallSourceSupportV1::SourceOnly(_)
    ));
    for change in 0..3 {
        let mut changed = actual.clone();
        match change {
            0 => changed.ordinal += 1,
            1 => changed.site = loan.call().site().clone(),
            _ => {
                changed.value = BorrowedCallActualValueV1::SelfRooted {
                    binding: forward.binding(),
                    root: target.receiver_binding().unwrap(),
                }
            }
        }
        assert!(
            prepare_borrowed_call_actuals_v1(
                source,
                &package.parameter_contracts,
                loan.call(),
                &[changed],
                &[],
                None,
                &mut |_| None
            )
            .is_err(),
            "change={change}"
        );
    }
}
