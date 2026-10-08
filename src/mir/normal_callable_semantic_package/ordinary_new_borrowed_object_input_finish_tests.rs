use super::*;
use std::rc::Rc;
type Package =
    crate::mir::normal_callable_semantic_package::VerifiedNormalCallableSemanticPackageV1;

fn package() -> Package {
    crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(
        "box Token {} box Maker { make(size: i64) { return new Token() } relay() { return me.make(7) } ignored() { local item = me.make(8) return 0 } } static box Main { main() { return 0 } }"
    ).unwrap()
}

fn restage_original_inputs(package: &mut Package) -> (FunctionOwnerIdV1, Vec<OwnedExprSiteV1>) {
    let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    let source = ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    let calls: Vec<_> = source.source_incoming.exact_rows().cloned().collect();
    assert_eq!(calls.len(), 2);
    let owner = calls[0].callee;
    for call in &calls {
        let executable = ledger.borrowed_formal_actuals[&call.call].as_ref().unwrap();
        executable.require_executable_v1().unwrap();
        let candidates: Vec<_> = executable
            .ordered_arguments
            .iter()
            .enumerate()
            .map(|(ordinal, argument)| BorrowedCallActualCandidateV1 {
                ordinal: ordinal as u32,
                site: call.source.argument_sites()[ordinal].clone(),
                value: match argument {
                    LocalCallArgumentV1::Integer(value) => {
                        BorrowedCallActualValueV1::Integer(*value)
                    }
                    _ => panic!("original fixture literal"),
                },
            })
            .collect();
        let rows = super::super::object_source::prepare_object_source_actuals_v1(
            source,
            &package.parameter_contracts,
            &call.call,
            &candidates,
        )
        .unwrap()
        .unwrap();
        ledger
            .borrowed_formal_actuals
            .insert(call.call.clone(), Ok(rows));
    }
    (owner, calls.into_iter().map(|row| row.call).collect())
}

#[test]
fn typed_object_input_missing_sibling_does_not_partially_install() {
    let mut package = package();
    let (_, sites) = restage_original_inputs(&mut package);
    let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    ledger.borrowed_formal_actuals.remove(&sites[1]);
    ledger
        .finish_typed_object_input_actuals_v1(
            &package.selected,
            &package.parameter_contracts,
            &package.physical_signature,
        )
        .unwrap();
    assert!(!ledger.borrowed_formal_actuals.contains_key(&sites[1]));
    assert!(ledger.borrowed_formal_actuals[&sites[0]]
        .as_ref()
        .unwrap()
        .require_executable_v1()
        .is_err());
}

#[test]
fn typed_object_input_existing_sibling_error_refuses_whole_cohort() {
    let mut package = package();
    let (_, sites) = restage_original_inputs(&mut package);
    let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    ledger
        .borrowed_formal_actuals
        .insert(sites[1].clone(), Err("original-sibling-error".into()));
    ledger
        .finish_typed_object_input_actuals_v1(
            &package.selected,
            &package.parameter_contracts,
            &package.physical_signature,
        )
        .unwrap();
    for site in sites {
        assert_eq!(
            ledger.borrowed_formal_actuals[&site].as_ref().unwrap_err(),
            "original-sibling-error"
        );
    }
}

#[test]
fn typed_object_input_missing_completion_preserves_source_only() {
    let mut package = package();
    let (owner, sites) = restage_original_inputs(&mut package);
    let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    ledger.completion_index.remove(&owner);
    ledger
        .finish_typed_object_input_actuals_v1(
            &package.selected,
            &package.parameter_contracts,
            &package.physical_signature,
        )
        .unwrap();
    for site in sites {
        assert!(ledger.borrowed_formal_actuals[&site]
            .as_ref()
            .unwrap()
            .require_executable_v1()
            .is_err());
    }
}

#[test]
fn typed_object_input_foreign_signature_refuses_before_install() {
    let foreign = package();
    let mut package = package();
    let (_, sites) = restage_original_inputs(&mut package);
    let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    assert!(ledger
        .finish_typed_object_input_actuals_v1(
            &package.selected,
            &package.parameter_contracts,
            &foreign.physical_signature
        )
        .unwrap_err()
        .contains("signature-identity"));
    for site in sites {
        assert!(ledger.borrowed_formal_actuals[&site]
            .as_ref()
            .unwrap()
            .require_executable_v1()
            .is_err());
    }
}

#[test]
fn typed_object_input_rejected_completion_preserves_cause_for_whole_cohort() {
    let mut package = package();
    let (owner, sites) = restage_original_inputs(&mut package);
    let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    ledger.completion_index.insert(
        owner,
        Err(crate::mir::resolved_control_flow::FunctionCompletionVerificationErrorV1::SourceNavigation(
            "original-completion-error".into(),
        )),
    );
    ledger
        .finish_typed_object_input_actuals_v1(
            &package.selected,
            &package.parameter_contracts,
            &package.physical_signature,
        )
        .unwrap();
    for site in sites {
        let issue = ledger.borrowed_formal_actuals[&site].as_ref().unwrap_err();
        assert!(issue.contains("callee-completion"));
        assert!(issue.contains("original-completion-error"));
    }
    assert!(ledger.completion_index[&owner].is_err());
}

fn qualified_input(
    package: &Package,
) -> (
    LexicalInstanceCallSourceTargetV1,
    crate::mir::normal_callable_semantic_package::ObjectReturnCallQualificationV1,
) {
    let source = package
        .ordinary_new_claim_ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    let target = source
        .source_incoming
        .exact_rows()
        .find_map(|row| {
            row.source
                .require_instance()
                .ok()
                .filter(|target| target.object_return_sources().is_some())
        })
        .unwrap()
        .clone();
    let loan = target.object_return_sources().unwrap()[0].clone();
    (target, loan)
}

#[test]
fn typed_object_input_completed_lender_borrows_original_arguments() {
    let package = package();
    let (target, loan) = qualified_input(&package);
    let ledger = &package.ordinary_new_claim_ledger;
    let args = ledger
        .checked_completed_typed_object_arguments_v1(
            &target,
            &loan,
            &package.selected,
            &package.parameter_contracts,
            &package.physical_signature,
        )
        .unwrap()
        .unwrap();
    assert_eq!(args, &[LocalCallArgumentV1::Integer(7)]);
    let original = &ledger.borrowed_formal_actuals[loan.call()]
        .as_ref()
        .unwrap()
        .ordered_arguments;
    assert_eq!(args.as_ptr(), original.as_ptr());
}

#[test]
fn typed_object_input_completed_lender_rechecks_missing_or_refused_sibling() {
    let mut package = package();
    let (target, loan) = qualified_input(&package);
    let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    let sibling = ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap()
        .source_incoming
        .exact_rows()
        .find(|row| row.call != *loan.call())
        .unwrap()
        .call
        .clone();
    let retained = ledger.borrowed_formal_actuals.remove(&sibling).unwrap();
    assert!(ledger
        .checked_completed_typed_object_arguments_v1(
            &target,
            &loan,
            &package.selected,
            &package.parameter_contracts,
            &package.physical_signature,
        )
        .unwrap()
        .is_none());
    ledger
        .borrowed_formal_actuals
        .insert(sibling.clone(), Err("late-sibling-refusal".into()));
    assert_eq!(
        ledger
            .checked_completed_typed_object_arguments_v1(
                &target,
                &loan,
                &package.selected,
                &package.parameter_contracts,
                &package.physical_signature,
            )
            .unwrap_err(),
        "late-sibling-refusal"
    );
    ledger.borrowed_formal_actuals.insert(sibling, retained);
}

#[test]
fn typed_object_input_completed_lender_never_installs_source_rows() {
    let mut package = package();
    let (target, loan) = qualified_input(&package);
    let (_, sites) = restage_original_inputs(&mut package);
    let ledger = &package.ordinary_new_claim_ledger;
    assert!(ledger
        .checked_completed_typed_object_arguments_v1(
            &target,
            &loan,
            &package.selected,
            &package.parameter_contracts,
            &package.physical_signature,
        )
        .unwrap()
        .is_none());
    for site in sites {
        assert!(ledger.borrowed_formal_actuals[&site]
            .as_ref()
            .unwrap()
            .require_executable_v1()
            .is_err());
    }
}

#[test]
fn typed_object_input_completed_lender_rechecks_completion_and_foreign_signature() {
    let foreign = package();
    let mut package = package();
    let (target, loan) = qualified_input(&package);
    let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    assert!(ledger
        .checked_completed_typed_object_arguments_v1(
            &target,
            &loan,
            &package.selected,
            &package.parameter_contracts,
            &foreign.physical_signature,
        )
        .unwrap_err()
        .contains("signature-identity"));
    ledger.completion_index.remove(&target.callee_owner());
    assert!(ledger
        .checked_completed_typed_object_arguments_v1(
            &target,
            &loan,
            &package.selected,
            &package.parameter_contracts,
            &package.physical_signature,
        )
        .unwrap()
        .is_none());
}

#[test]
fn typed_object_input_target_lender_shares_original_storage_and_sibling_failures() {
    let foreign = package();
    let (foreign_target, foreign_loan) = qualified_input(&foreign);
    let mut package = package();
    let (target, loan) = qualified_input(&package);
    let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    let generic = |ledger: &OrdinaryNewClaimLedgerV1,
                   target: &LexicalInstanceCallSourceTargetV1| {
        ledger
            .checked_completed_typed_object_target_arguments_v1(
                target,
                &package.selected,
                &package.parameter_contracts,
                &package.physical_signature,
            )
            .map(|rows| rows.map(|rows| (rows.as_ptr(), rows.to_vec())))
    };
    let qualified = ledger
        .checked_completed_typed_object_arguments_v1(
            &target,
            &loan,
            &package.selected,
            &package.parameter_contracts,
            &package.physical_signature,
        )
        .unwrap()
        .unwrap();
    assert_eq!(
        generic(ledger, &target).unwrap().unwrap().0,
        qualified.as_ptr()
    );
    let sibling_target = ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap()
        .source_incoming
        .exact_rows()
        .find(|row| row.call != *loan.call())
        .unwrap()
        .source
        .require_instance()
        .unwrap()
        .clone();
    assert!(sibling_target.object_return_sources().is_none());
    assert_eq!(
        generic(ledger, &sibling_target).unwrap().unwrap().1,
        vec![LocalCallArgumentV1::Integer(8)]
    );
    let sibling = sibling_target.call_site().clone();
    let retained = ledger.borrowed_formal_actuals.remove(&sibling).unwrap();
    assert!(generic(ledger, &target).unwrap().is_none());
    assert!(ledger
        .checked_completed_typed_object_arguments_v1(
            &target,
            &loan,
            &package.selected,
            &package.parameter_contracts,
            &package.physical_signature,
        )
        .unwrap()
        .is_none());
    ledger
        .borrowed_formal_actuals
        .insert(sibling.clone(), Err("shared-sibling-refusal".into()));
    assert_eq!(
        generic(ledger, &target).unwrap_err(),
        "shared-sibling-refusal"
    );
    assert_eq!(
        ledger
            .checked_completed_typed_object_arguments_v1(
                &target,
                &loan,
                &package.selected,
                &package.parameter_contracts,
                &package.physical_signature,
            )
            .unwrap_err(),
        "shared-sibling-refusal"
    );
    ledger.borrowed_formal_actuals.insert(sibling, retained);
}
