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
    let diagnostic = ledger
        .typed_object_input_completion_diagnostic_v1(owner)
        .unwrap();
    assert!(
        diagnostic.contains("pending=completion-missing"),
        "{diagnostic}"
    );
    assert!(
        diagnostic.contains(&format!("callee={owner:?}")),
        "{diagnostic}"
    );
    for site in sites {
        let error = ledger.borrowed_formal_actuals[&site]
            .as_ref()
            .unwrap()
            .require_executable_v1()
            .unwrap_err();
        assert!(error.contains("source-only-object-actuals"), "{error}");
        assert!(error.contains(&format!("call={site:?}")), "{error}");
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

#[test]
fn guarded_object_actuals_refuse_static_source_and_reconstructed_snapshot_drift() {
    let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(
        "box Token {} box Maker { make(size) { if size <= 0 { return new Token() } return new Token() } qualified() { return me.make(7) } relay(size) { if size <= 0 { return 0 } local item = me.make(size) return 0 } } static box Layout { id(p): i64 { return 0 } } static box Main { main() { local n = Layout.id(7) return 0 } }",
    ).unwrap();
    let ledger = &package.ordinary_new_claim_ledger;
    let source = ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    let (call, rows, identity) = source
        .source_incoming
        .exact_rows()
        .find_map(|call| {
            let rows = ledger
                .borrowed_formal_actuals
                .get(&call.call)?
                .as_ref()
                .ok()?;
            if let BorrowedCallActualEvidencePhaseV1::ExecutableSourceObject(identity) = &rows.phase
            {
                Some((call, rows, identity))
            } else {
                None
            }
        })
        .expect("reconstructed original guarded input");
    super::super::object_source::recheck_object_source_actuals_v1(
        source,
        &package.parameter_contracts,
        call,
        rows,
        identity,
        None,
    )
    .unwrap();
    let static_call = source
        .source_incoming
        .exact_rows()
        .find(|call| call.source.instance().is_none())
        .unwrap();
    assert!(super::super::revalidate_guarded_object_incoming_v1(
        source,
        &package.parameter_contracts,
        static_call,
        rows
    ,
        None,
    )
    .unwrap_err()
    .contains("noninstance-source"));
    for mutation in 0..3 {
        let mut drifted = rows.clone();
        match mutation {
            0 => drifted.phase = BorrowedCallActualEvidencePhaseV1::Executable,
            1 => drifted.opaque_actuals[0].ordinal += 1,
            _ => drifted.opaque_actuals[0].source = BorrowedFormalActualSourceV1::Integer(7),
        }
        assert!(super::super::revalidate_passive_object_incoming_v1(
            source,
            &package.parameter_contracts,
            call,
            &drifted
        ,
        None,
    )
        .is_err());
    }
}

#[test]
fn object_input_finishes_original_literal_sibling_without_a_guarded_declaration() {
    let mut package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(
        "box Token {} box Maker { make(size) { if size <= 0 { return new Token() } return new Token() } qualified() { return me.make(7) } relay(size) { if size <= 0 { return 0 } local item = me.make(size) return 0 } } static box Layout { id(p): i64 { return 0 } } static box Main { main() { local n = Layout.id(7) return 0 } }",
    ).unwrap();
    let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    let source = ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    let (call, literal) = source
        .source_incoming
        .exact_rows()
        .find_map(|call| {
            let rows = ledger
                .borrowed_formal_actuals
                .get(&call.call)?
                .as_ref()
                .ok()?;
            let [actual] = rows.opaque_actuals.as_ref() else {
                return None;
            };
            let BorrowedFormalActualSourceV1::Integer(value) = actual.source else {
                return None;
            };
            call.source.instance().map(|_| (call.clone(), value))
        })
        .expect("same original literal sibling");
    assert!(ledger
        .checked_guarded_instance_definition_v1(source, call.call.owner())
        .unwrap()
        .is_none());
    let candidates = [BorrowedCallActualCandidateV1 {
        ordinal: 0,
        site: call.source.argument_sites()[0].clone(),
        value: BorrowedCallActualValueV1::Integer(literal),
    }];
    let original = super::super::object_source::prepare_object_source_actuals_v1(
        source,
        &package.parameter_contracts,
        &call.call,
        &candidates,
    )
    .unwrap()
    .unwrap();
    ledger
        .borrowed_formal_actuals
        .insert(call.call.clone(), Ok(original));
    ledger
        .finish_typed_object_input_actuals_v1(
            &package.selected,
            &package.parameter_contracts,
            &package.physical_signature,
        )
        .unwrap();
    let source = ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    let finished = ledger.borrowed_formal_actuals[&call.call].as_ref().unwrap();
    finished.ordered_arguments_for_v1(&call).unwrap();
    assert!(
        !super::super::revalidate_guarded_object_incoming_v1(
            source,
            &package.parameter_contracts,
            &call,
            finished,
            None,
        )
        .unwrap(),
        "literal source is not a guarded reverse dependency"
    );
}

#[test]
fn original_instance_input_keeps_result_law_and_rechecks_whole_cohort() {
    for (mixed, missing_guard) in [(true, false), (false, false), (true, true)] {
    let text = "box Token {} box Maker { sink(size) { if size <= 0 { return new Token() } return new Token() } blocked(p): i64 { local ignored: i64 = p return 0 } take(h: Token, size): i64 { if h == null { return 0 } if size <= 0 { return 0 } local item = me.sink(size) return 0 } relay(h: Token, size): i64 { if h == null { return 0 } if size <= 0 { return 0 } local out = me.take(h, size) return 0 } relayAlias(h: Token, size): i64 { if h == null { return 0 } if size <= 0 { return 0 } local alias = h local out = me.take(alias, size) return 0 } } static box Main { main() { return 0 } }";
    let global = text.replace("take(h: Token, size)", "take(h, size)");
    let text = if mixed { text } else { global.as_str() };
    let missing_guard_text = text
        .replace("local out = me.take(h, size)", "local anchor = me.take(h, size) local out = me.take(h, size)")
        .replace("local out = me.take(alias, size)", "local anchor = me.take(alias, size) local out = me.take(alias, size)");
    let text = if missing_guard { missing_guard_text.as_str() } else { text };
    let mut package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(text).unwrap();
    let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    if missing_guard {
        let source = ledger.borrowed_formal_source.as_mut().unwrap().as_mut().unwrap();
        let calls: std::collections::BTreeMap<_, _> = source.source_incoming.exact_rows()
            .filter(|call| call.source.target().name() == "take").map(|call| (call.call.owner(), call.call.clone())).collect();
        for call in calls.into_values() { assert!(source.guarded_actuals.remove(&(call, 1)).is_some()); }
    }
    let source = ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    let original = source
        .source_incoming
        .exact_rows()
        .find(|call| {
            call.source.target().owner() == "Maker" && call.source.target().name() == "take"
        })
        .expect("original non-Object Instance incoming")
        .clone();
    if !missing_guard { assert!(
        matches!(
            ledger.borrowed_formal_actuals.get(&original.call),
            Some(Ok(PreparedBorrowedCallActualsV1 {
                phase: BorrowedCallActualEvidencePhaseV1::ExecutableSourceInstance(_),
                ..
            }))
        ),
        "the production probe and input issuer must finish the original row"
    ); }
    // The missing-guard mutation is reconstructed from source below, without
    // assuming its deliberately changed cohort was already executable.
    let original_calls: Vec<_> = source
        .source_incoming
        .exact_rows()
        .filter(|call| call.callee == original.callee)
        .cloned()
        .collect();
    assert_eq!(original_calls.len(), if missing_guard { 4 } else { 2 });
    for call in &original_calls {
        let draft = source
            .source_only_definitions
            .get(&call.call.owner())
            .unwrap();
        let candidates: Vec<_> = call
            .source
            .argument_sites()
            .iter()
            .enumerate()
            .map(|(i, site)| {
                let row = draft
                    .uses
                    .iter()
                    .find(|row| row.site.site() == site)
                    .unwrap();
                BorrowedCallActualCandidateV1 {
                    ordinal: i as u32,
                    site: site.clone(),
                    value: BorrowedCallActualValueV1::SelfRooted {
                        binding: row.binding,
                        root: row.formal,
                    },
                }
            })
            .collect();
        if missing_guard && !source.guarded_actuals.contains_key(&(call.call.clone(), 1)) {
            let BorrowedCallActualValueV1::SelfRooted { binding, root } = candidates[1].value else { panic!("original tagged formal") };
            let selected = source.original_mixed_tagged_forward_v1(
                &package.parameter_contracts, &call.call, 1, &candidates[1].site, binding, root,
            ).unwrap().expect("the independently qualified mixed frame lends tagged input");
            assert!(std::ptr::eq(selected, draft));
            assert!(source.original_mixed_tagged_forward_v1(
                &package.parameter_contracts, &call.call, 0, &candidates[1].site, binding, root,
            ).is_err(), "a different ordinal cannot borrow the selected source");
        }
        let rows = super::super::instance_source::prepare_instance_source_actuals_v1(
            source,
            &package.parameter_contracts,
            &call.call,
            &candidates,
        )
        .unwrap()
        .expect("original Instance input producer");
        ledger
            .borrowed_formal_actuals
            .insert(call.call.clone(), Ok(rows));
    }
    assert!(
        ledger
            .checked_source_instance_input_entry_v1(source, original.callee)
            .unwrap()
            .is_some(),
        "original mixed frame missing; guards={:?}",
        source.guarded_actuals
    );
    ledger
        .finish_original_instance_input_actuals_v1(
            &package.selected,
            &package.parameter_contracts,
            &package.physical_signature,
            &package.result_contracts,
        )
        .unwrap();
    let source = ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    for call in &original_calls {
        let rows = ledger.borrowed_formal_actuals[&call.call].as_ref().unwrap();
        rows.require_executable_v1().unwrap();
        assert!(matches!(rows.opaque_actuals[1].source, BorrowedFormalActualSourceV1::Forwarded { .. }));
    }
    let rows = ledger.borrowed_formal_actuals[&original.call]
        .as_ref()
        .unwrap();
    let BorrowedCallActualEvidencePhaseV1::ExecutableSourceInstance(identity) = &rows.phase else {
        panic!(
            "whole original Instance input must finish: {:?}; {}",
            rows.phase,
            ledger
                .typed_object_input_completion_diagnostic_v1(original.callee)
                .unwrap()
        );
    };
    assert!(ledger.selected_instance_input_protocol_v1(source, original.callee).unwrap()
        == Some(OriginalInstanceInputKindV1::SourceInstance));
    assert!(ledger.checked_completed_original_instance_target_arguments_v1(
        original.source.require_instance().unwrap(), &package.selected,
        &package.parameter_contracts, &package.physical_signature,
    ).unwrap().is_some_and(|arguments| std::ptr::eq(arguments, rows.ordered_arguments.as_ref())));
    assert!(identity.source.object_return_sources().is_none());
    assert!(identity.source.object_source_forwards().is_none());
    assert!(!source
        .source_incoming
        .has_object_input_callee_v1(&identity.source));
    super::super::recheck_finished_instance_input_v1(
        ledger,
        source,
        &package.parameter_contracts,
        &original,
        rows,
    )
    .unwrap();
    // Both original calls are required even when the selected row is ready.
    let sibling = source
        .source_incoming
        .exact_rows()
        .find(|call| call.callee == original.callee && call.call != original.call)
        .expect("original alias sibling")
        .call
        .clone();
    let saved_sibling = ledger.borrowed_formal_actuals.remove(&sibling).unwrap();
    let source = ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    let rows = ledger.borrowed_formal_actuals[&original.call]
        .as_ref()
        .unwrap();
    assert!(super::super::recheck_finished_instance_input_v1(
        ledger,
        source,
        &package.parameter_contracts,
        &original,
        rows
    )
    .unwrap_err()
    .contains("cohort-unfinished"));
    ledger
        .borrowed_formal_actuals
        .insert(sibling, saved_sibling);
    // Removing own Completion must invalidate an already finished actual row.
    let saved = ledger.completion_index.remove(&original.callee).unwrap();
    let source = ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    let rows = ledger.borrowed_formal_actuals[&original.call]
        .as_ref()
        .unwrap();
    assert!(super::super::recheck_finished_instance_input_v1(
        ledger,
        source,
        &package.parameter_contracts,
        &original,
        rows
    )
    .unwrap_err()
    .contains("cohort-unfinished"));
    ledger.completion_index.insert(original.callee, saved);
    // Borrowing a foreign original signature must fail even after installation.
    let foreign = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(text).unwrap();
    ledger.original_instance_input_issuance.as_mut().unwrap().1 =
        foreign.physical_signature.clone();
    let source = ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    let rows = ledger.borrowed_formal_actuals[&original.call]
        .as_ref()
        .unwrap();
    assert!(super::super::recheck_finished_instance_input_v1(
        ledger,
        source,
        &package.parameter_contracts,
        &original,
        rows
    )
    .unwrap_err()
    .contains("signature-identity"));
    }
}
