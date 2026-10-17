use super::*;
use crate::mir::normal_callable_semantic_package::VerifiedNormalCallableSemanticPackageV1;
use std::rc::Rc;

fn package(
    parameters: &str,
    result: &str,
    body: &str,
    arguments: &str,
) -> VerifiedNormalCallableSemanticPackageV1 {
    let text = format!("box Transport {{ birth() {{ }} probe({parameters}) {result} {{ {body} }} }} static box Main {{ main() {{ local recv = new Transport() local out = recv.probe({arguments}) return 0 }} }}");
    crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(
        &text,
    )
    .unwrap()
}

fn take(package: &VerifiedNormalCallableSemanticPackageV1) -> LexicalInstanceCallDispositionRowV1 {
    let ledger = &package.ordinary_new_claim_ledger;
    let source = ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    let call = &source.incoming[0].call;
    ledger
        .take_lexical_instance_call(call.owner(), call.site())
        .unwrap()
        .unwrap()
}

#[test]
fn borrowed_call_lends_original_actuals_after_source_and_result_corroboration() {
    for arguments in ["-7", "true", "recv"] {
        let package = package("p", ": i64", "return 0", arguments);
        let row = take(&package);
        let ledger = &package.ordinary_new_claim_ledger;
        let actuals = ledger.borrowed_call_actuals_v1(&row).unwrap().unwrap();
        let original = ledger.borrowed_formal_actuals[row.call_site()]
            .as_ref()
            .unwrap();
        assert!(std::ptr::eq(actuals, original.opaque_actuals.as_ref()));
        assert_eq!(actuals.len(), 1);
        assert_eq!(actuals[0].site, row.argument_sites()[0]);
        assert_eq!(actuals[0].formal.owner(), row.callee_owner());
        assert!(ledger
            .take_lexical_instance_call(row.call_site().owner(), row.call_site().site())
            .unwrap_err()
            .contains("already-taken"));
    }
}

#[test]
fn borrowed_call_result_accepts_exact_i64_formal_with_mixed_opaque_ordinals() {
    let package = package("p, q: i64", ": i64", "return q", "true, 7");
    let row = take(&package);
    let actuals = package
        .ordinary_new_claim_ledger
        .borrowed_call_actuals_v1(&row)
        .unwrap()
        .unwrap();
    assert_eq!(actuals.len(), 1);
    assert_eq!(actuals[0].ordinal, 0);
}

#[test]
fn borrowed_call_refuses_old_literal_i64_defaults_for_other_source_domains() {
    // A nullable result is an object-or-null contract: a null-only body
    // has no `return new ..` exit a `NullableObject` claim could name, so
    // it stays rejected beside the other unclassified scalar domains.
    for body in ["return \"text\"", "return null"] {
        let source = format!("box Transport {{ birth() {{ }} probe(p) {{ {body} }} }} static box Main {{ main() {{ local recv = new Transport() local out = recv.probe(0) return 0 }} }}");
        let error = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(&source).err().expect("selected result must be source-I64");
        let error = format!("{error:?}");
        assert!(
            error.contains("borrowed-result/source-not-i64"),
            "{body}: {error}"
        );
    }
}

#[test]
fn borrowed_call_refuses_annotation_without_explicit_value_return() {
    let invalid =
        "box Transport { birth() { } probe(p): i64 { } } static box Main { main() { return 0 } }";
    let error = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(invalid).err().expect("annotation requires a real value return");
    assert!(format!("{error:?}").contains("MissingReturnValueOnPath"));
    let source = "box Transport { birth() { } probe(p) { } } static box Main { main() { local recv = new Transport() local out = recv.probe(0) return 0 } }";
    let error = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(source).err().expect("selected call needs an explicit return");
    let error = format!("{error:?}");
    assert!(
        error.contains("borrowed-result/explicit-value-return-missing"),
        "{error}"
    );
}

#[test]
fn borrowed_call_refuses_missing_or_uncorroborated_result_source() {
    for missing in [false, true] {
        let mut package = package("p", ": i64", "return 0", "0");
        let row = take(&package);
        let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
        if missing {
            ledger.borrowed_i64_results.clear();
        } else {
            ledger
                .borrowed_i64_results
                .get_mut(&row.callee_owner())
                .unwrap()
                .as_mut()
                .unwrap()
                .contract_corroborated = false;
        }
        let error = ledger.borrowed_call_actuals_v1(&row).unwrap_err();
        assert!(
            error.contains(if missing {
                "result-source-missing"
            } else {
                "result-not-corroborated"
            }),
            "{error}"
        );
    }

    // Erasing retained products cannot turn missing original ingress into
    // permission. Reuse the result refusal owner for this combined boundary.
    for mutation in 0..4 {
        let mut package = package(
            "p, q",
            ": i64",
            "if p <= q { return p * q } return 0",
            "5, 6",
        );
        let row = take(&package);
        let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
        let proof = ledger
            .borrowed_i64_results
            .get_mut(&row.callee_owner())
            .unwrap()
            .as_mut()
            .unwrap();
        assert_eq!(proof.multiplications.len(), 1);
        proof.source_facts_mut_for_test().multiplications = Box::new([]);
        proof.contract_corroborated = false;
        let expected = match mutation {
            0 => "mul-return-coverage",
            1 => {
                ledger.borrowed_formal_source = None;
                "mul-source-missing"
            }
            2 => {
                ledger.borrowed_formal_source = Some(Err("original-ingress-failed".into()));
                "original-ingress-failed"
            }
            _ => {
                ledger
                    .borrowed_formal_source
                    .as_mut()
                    .unwrap()
                    .as_mut()
                    .unwrap()
                    .definitions
                    .remove(&row.callee_owner());
                "mul-entry-owner"
            }
        };
        ledger.corroborate_borrowed_i64_result_v1(row.source_target(), &package.result_contracts);
        let issue = ledger.borrowed_i64_results[&row.callee_owner()]
            .as_ref()
            .unwrap_err();
        assert!(issue.contains(expected), "mutation {mutation}: {issue}");
    }
}

#[test]
fn borrowed_call_refuses_foreign_disposition() {
    let own = package("p", ": i64", "return 0", "0");
    let foreign = package("p", ": i64", "return 0", "0");
    let row = take(&foreign);
    assert!(own
        .ordinary_new_claim_ledger
        .borrowed_call_actuals_v1(&row)
        .unwrap_err()
        .contains("disposition-not-owned-and-taken"));
}

#[test]
fn borrowed_call_refuses_changed_argument_site_and_result() {
    for change in [false, true] {
        let package = package("p", ": i64", "return 0", "0");
        let mut row = take(&package);
        if change {
            row.result = Some(InvokeCallResultKind::Handle);
        } else {
            row.source.argument_sites[0] = row.source.receiver_site.clone();
        }
        let error = package
            .ordinary_new_claim_ledger
            .borrowed_call_actuals_v1(&row)
            .unwrap_err();
        assert!(
            error.contains(if change {
                "borrowed-call/result-mismatch"
            } else {
                "disposition-identity"
            }),
            "{error}"
        );
    }
}

#[test]
fn borrowed_call_refuses_failed_actual_before_lending_rows() {
    let mut package = package("p", ": i64", "return 0", "0");
    let row = take(&package);
    *Rc::get_mut(&mut package.ordinary_new_claim_ledger)
        .unwrap()
        .borrowed_formal_actuals
        .values_mut()
        .next()
        .unwrap() = Err("borrowed-actual/unsupported-or-unavailable".into());
    let error = package
        .ordinary_new_claim_ledger
        .borrowed_call_actuals_v1(&row)
        .unwrap_err();
    assert!(
        error.contains("borrowed-actual/unsupported-or-unavailable"),
        "{error}"
    );
}

#[test]
fn borrowed_call_refuses_changed_nonopaque_site_target_slot_and_receiver() {
    for change in 0..3 {
        let package = package("p, q: i64", ": i64", "return q", "true, 7");
        let mut row = take(&package);
        match change {
            0 => row.source.argument_sites[1] = row.source.receiver_site.clone(),
            1 => row.source.target_batch_slot += 1,
            _ => row.source.receiver_site = row.source.argument_sites[0].clone(),
        }
        assert!(package
            .ordinary_new_claim_ledger
            .borrowed_call_actuals_v1(&row)
            .unwrap_err()
            .contains("disposition-identity"));
    }
}

#[test]
fn borrowed_call_refuses_unconsumed_ready_row() {
    let package = package("p", ": i64", "return 0", "0");
    let ledger = &package.ordinary_new_claim_ledger;
    let slots = ledger.lexical_instance_calls.borrow();
    let row = slots
        .values()
        .find_map(|slot| match slot {
            LexicalInstanceCallDispositionSlotV1::Ready(row) => Some(row),
            _ => None,
        })
        .unwrap();
    assert!(ledger
        .borrowed_call_actuals_v1(row)
        .unwrap_err()
        .contains("disposition-not-owned-and-taken"));
}

#[test]
fn borrowed_call_result_corroboration_rejects_changed_return_site() {
    let mut package = package("p", ": i64", "return 0", "0");
    let row = take(&package);
    let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    let proof = ledger
        .borrowed_i64_results
        .get_mut(&row.callee_owner())
        .unwrap()
        .as_mut()
        .unwrap();
    proof.source_facts_mut_for_test().returns[0] =
        OwnedExprSiteV1::new(row.callee_owner(), row.argument_sites()[0].clone());
    ledger.corroborate_borrowed_i64_result_v1(row.source_target(), &package.result_contracts);
    assert!(ledger
        .borrowed_call_actuals_v1(&row)
        .unwrap_err()
        .contains("borrowed-result/result-contract-mismatch"));
}

#[test]
fn prewalk_projection_keeps_source_actual_result_priority_without_corroboration_cycle() {
    let mut package = package("p", ": i64", "return 0", "true");
    let row = take(&package);
    let site = row.call_site().clone();
    let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    let demand = |ledger: &crate::mir::normal_callable_semantic_package::ordinary_new_coseal::OrdinaryNewClaimLedgerV1| {
        super::super::project_pending_borrowed_i64_arguments_v1(
            ledger.borrowed_formal_source.as_ref().unwrap(),
            &ledger.borrowed_formal_actuals,
            &ledger.borrowed_i64_results,
            &site,
        )
    };
    ledger
        .borrowed_i64_results
        .get_mut(&row.callee_owner())
        .unwrap()
        .as_mut()
        .unwrap()
        .contract_corroborated = false;
    let arguments = demand(ledger).unwrap().unwrap();
    assert!(matches!(
        arguments.as_ref(),
        [
            crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1::BorrowedActual {
                ordinal: 0,
                ..
            }
        ]
    ));
    assert!(ledger
        .borrowed_call_actuals_v1(&row)
        .unwrap_err()
        .contains("result-not-corroborated"));
    let actuals = ledger.borrowed_formal_actuals.remove(&site).unwrap();
    assert!(demand(ledger).unwrap_err().contains("actuals-missing"));
    ledger.borrowed_formal_actuals.insert(site.clone(), actuals);
    let proof = ledger
        .borrowed_i64_results
        .remove(&row.callee_owner())
        .unwrap();
    assert!(demand(ledger)
        .unwrap_err()
        .contains("result-source-missing"));
    ledger
        .borrowed_i64_results
        .insert(row.callee_owner(), proof);
    let excluded = OwnedExprSiteV1::new(site.owner(), row.argument_sites()[0].clone());
    assert!(super::super::project_pending_borrowed_i64_arguments_v1(
        ledger.borrowed_formal_source.as_ref().unwrap(),
        &BTreeMap::new(),
        &crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::BorrowedResultCohortV1::default(),
        &excluded,
    )
    .unwrap()
    .is_none());
    *ledger
        .borrowed_i64_results
        .get_mut(&row.callee_owner())
        .unwrap() = Err("result-sentinel".into());
    assert_eq!(demand(ledger).unwrap_err(), "result-sentinel");
    *ledger.borrowed_formal_actuals.get_mut(&site).unwrap() = Err("actual-sentinel".into());
    assert_eq!(demand(ledger).unwrap_err(), "actual-sentinel");
    ledger.borrowed_formal_source = Some(Err("source-sentinel".into()));
    assert_eq!(demand(ledger).unwrap_err(), "source-sentinel");
}

/// TASK4-I64RESULT-S0: an unannotated declaration stays `Unannotated` while
/// the complete source-I64 return set proves the executable projection —
/// the same Completion/site corroboration the annotated lane demands, never
/// a manufactured annotation or an unconstrained `None` acceptance.
#[test]
fn borrowed_call_result_accepts_unannotated_complete_i64_source() {
    for (parameters, body, arguments) in [
        ("p", "return 0", "0"),
        ("p", "if p > 0 { return 0 } return 1", "0"),
        ("p, q: i64", "return q", "true, 7"),
        ("p, q", "if p <= q { return p * q } return 0", "5, 6"),
        ("p", "if p <= 8 { return -2 * p } return 0", "5"),
    ] {
        let package = package(parameters, "", body, arguments);
        let row = take(&package);
        assert_eq!(row.result, Some(InvokeCallResultKind::I64), "{body}");
        let ledger = &package.ordinary_new_claim_ledger;
        let proof = ledger.borrowed_i64_results[&row.callee_owner()]
            .as_ref()
            .unwrap();
        assert!(matches!(proof.class, BorrowedResultClassV1::I64), "{body}");
        assert!(proof.contract_corroborated, "{body}");
        let contract = package
            .result_contracts
            .row(row.source_target().target_batch_slot())
            .unwrap()
            .borrow();
        assert!(contract.result().is_none(), "{body}");
        assert_eq!(
            contract.declared_result(),
            &crate::mir::resolved_control_flow::DeclaredFunctionResultContractV1::Unannotated,
            "{body}"
        );
        ledger.borrowed_call_actuals_v1(&row).unwrap().unwrap();
    }
}

/// The unannotated arm borrows only the existing I64 source vocabulary:
/// opaque/other-domain returns, mixed exits, implicit exits and other
/// declared annotations never acquire the projection.
#[test]
fn borrowed_call_result_keeps_unannotated_i64_bounded() {
    for (label, result, body, token) in [
        (
            "opaque-return",
            "",
            "return p",
            "borrowed-result/source-not-i64",
        ),
        (
            "mixed-exit",
            "",
            "if p > 0 { return 0 } return null",
            "borrowed-result/source-class-mixed",
        ),
        (
            "implicit-exit",
            "",
            "local x = 1",
            "borrowed-result/explicit-value-return-missing",
        ),
        (
            "bool-annotation",
            ": bool",
            "return 0",
            "UnsupportedResultAnnotation",
        ),
        (
            "void-annotation",
            ": void",
            "return 0",
            "ReturnContractMismatch",
        ),
    ] {
        let source = format!("box Transport {{ birth() {{ }} probe(p) {result} {{ {body} }} }} static box Main {{ main() {{ local recv = new Transport() local out = recv.probe(0) return 0 }} }}");
        match crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(&source) {
            Err(error) => {
                let error = format!("{error:?}");
                assert!(error.contains(token), "{label}: {error}");
            }
            // An opaque-return body leaves the callee outside the borrowed
            // profile at the use-draft frontier, so no result row exists to
            // demand at package level; the pin is that no callee ever
            // acquires the corroborated I64 permission.
            Ok(package) => assert!(
                package
                    .ordinary_new_claim_ledger
                    .borrowed_i64_results
                    .values()
                    .all(|proof| proof.is_err()),
                "{label}: an opaque-return callee acquired an I64 permission"
            ),
        }
    }
}

#[test]
fn original_argument_lender_preserves_identity_without_result_authority() {
    let mut package = package("p", ": i64", "return 0", "true");
    let row = take(&package);
    let site = row.call_site().clone();
    let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    ledger
        .borrowed_i64_results
        .remove(&row.callee_owner())
        .unwrap()
        .unwrap();
    let source = ledger.borrowed_formal_source.as_ref().unwrap();
    let actuals = &ledger.borrowed_formal_actuals;
    let (incoming, arguments) =
        super::super::borrowed_formal_actuals::lend_pending_borrowed_arguments_v1(
            source, actuals, &site,
        )
        .unwrap()
        .unwrap();
    let original = &source.as_ref().unwrap().incoming[0];
    assert!(std::ptr::eq(incoming, original));
    assert!(std::ptr::eq(
        arguments,
        actuals[&site].as_ref().unwrap().ordered_arguments.as_ref()
    ));
    assert_eq!(incoming.callee, row.callee_owner());
    assert!(super::super::project_pending_borrowed_i64_arguments_v1(
        source,
        actuals,
        &ledger.borrowed_i64_results,
        &site
    )
    .unwrap_err()
    .contains("result-source-missing"));
    assert!(
        ledger.borrowed_call_actuals_v1(&row).is_err(),
        "lending arguments grants no result"
    );
    let excluded = OwnedExprSiteV1::new(site.owner(), row.argument_sites()[0].clone());
    assert!(
        super::super::borrowed_formal_actuals::lend_pending_borrowed_arguments_v1(
            source,
            &BTreeMap::new(),
            &excluded
        )
        .unwrap()
        .is_none()
    );
}

#[test]
fn original_argument_lender_rejects_duplicate_incoming_and_argument_drift() {
    for duplicate in [false, true] {
        let mut package = package("p", ": i64", "return 0", "true");
        let row = take(&package);
        let site = row.call_site().clone();
        let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
        if duplicate {
            let source = ledger
                .borrowed_formal_source
                .as_mut()
                .unwrap()
                .as_mut()
                .unwrap();
            let mut incoming = source.incoming.to_vec();
            incoming.push(incoming[0].clone());
            source.incoming = incoming.into_boxed_slice();
        } else {
            let actuals = ledger
                .borrowed_formal_actuals
                .get_mut(&site)
                .unwrap()
                .as_mut()
                .unwrap();
            actuals.ordered_arguments[0] =
                crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1::Integer(1);
        }
        let error = super::super::borrowed_formal_actuals::lend_pending_borrowed_arguments_v1(
            ledger.borrowed_formal_source.as_ref().unwrap(),
            &ledger.borrowed_formal_actuals,
            &site,
        )
        .unwrap_err();
        assert!(
            error.contains(if duplicate {
                "source-identity"
            } else {
                "ordered-arguments-identity"
            }),
            "{error}"
        );
    }
}

#[test]
fn scalar_source_loan_requires_same_live_original_result_without_execution_permission() {
    use crate::mir::resolved_semantics::home_new_prefix::SourceScalarKind;
    for (body, kind) in [
        ("return 7", SourceScalarKind::Integer),
        ("return true", SourceScalarKind::Bool),
    ] {
        for mutation in 0..5 {
            let mut package = package("p", "", body, "0");
            let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
            let original = ledger
                .borrowed_formal_source
                .as_ref()
                .unwrap()
                .as_ref()
                .unwrap()
                .incoming[0]
                .source
                .require_instance()
                .unwrap()
                .clone();
            let source = ledger.borrowed_formal_source.as_ref().unwrap();
            let loan =
                issue_source_scalar_result_loan_v1(source, &ledger.borrowed_i64_results, &original)
                    .unwrap();
            assert_eq!(loan.kind(), kind);
            assert_eq!(loan, issue_source_scalar_result_loan_v1(source, &ledger.borrowed_i64_results, &original).unwrap());
            validate_source_scalar_result_loan_v1(
                source,
                &ledger.borrowed_i64_results,
                &loan,
                &original,
            )
            .unwrap();
            corroborate_source_scalar_result_loan_v1(
                source, &ledger.borrowed_i64_results, &loan, &original,
                &package.result_contracts,
            ).unwrap();
            let owner = original.callee_owner();
            let expected = match mutation {
                0 => {
                    ledger.borrowed_i64_results.remove(&owner);
                    "scalar-loan/result-missing"
                }
                1 => {
                    ledger
                        .borrowed_i64_results
                        .insert(owner, Err("original-result-error".into()));
                    "original-result-error"
                }
                2 => {
                    // Equivalent copied facts still are not the retained original.
                    ledger
                        .borrowed_i64_results
                        .get_mut(&owner)
                        .unwrap()
                        .as_mut()
                        .unwrap()
                        .source_facts_mut_for_test();
                    "scalar-loan/result-identity"
                }
                3 => {
                    ledger.borrowed_formal_source = None;
                    ""
                }
                _ => {
                    // Source readiness intentionally grants no execution waiver.
                    ledger
                        .borrowed_i64_results
                        .get_mut(&owner)
                        .unwrap()
                        .as_mut()
                        .unwrap()
                        .contract_corroborated = false;
                    ""
                }
            };
            if mutation == 3 {
                assert!(validate_source_scalar_result_loan_v1(
                    &Err("original-ingress-error".into()),
                    &ledger.borrowed_i64_results,
                    &loan,
                    &original
                )
                .unwrap_err()
                .contains("original-ingress-error"));
            } else {
                let result = validate_source_scalar_result_loan_v1(
                    ledger.borrowed_formal_source.as_ref().unwrap(),
                    &ledger.borrowed_i64_results,
                    &loan,
                    &original,
                );
                if expected.is_empty() {
                    result.unwrap();
                } else {
                    assert!(result.unwrap_err().contains(expected));
                }
                let corroboration = corroborate_source_scalar_result_loan_v1(
                    ledger.borrowed_formal_source.as_ref().unwrap(),
                    &ledger.borrowed_i64_results, &loan, &original,
                    &package.result_contracts,
                );
                assert!(corroboration.unwrap_err().contains(if mutation == 4 {
                    "scalar-loan/result-contract-mismatch"
                } else { expected }));
            }
        }
    }
    // Complete Bool source/result agreement permits a typed row, not unchecked input.
    let source = "box Transport { flag: i64 birth() { me.flag = 0 } probe(p) { return true } }
        box Parent { child: Transport birth() { me.child = new Transport() }
            run(p) { return me.child.probe(p) } }
        static box Main { main() { local recv = new Parent() return recv.run(true) } }";
    let mut package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(source)
        .unwrap();
    let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    let site = ledger.lexical_instance_calls.borrow().iter().find_map(|(site, slot)| {
        match slot {
            super::super::LexicalInstanceCallDispositionSlotV1::Ready(row)
                if row.result() == Some(InvokeCallResultKind::Bool) => Some(site.clone()),
            _ => None,
        }
    }).expect("original stored-child Bool call has a typed disposition");
    let row = ledger.take_lexical_instance_call(site.owner(), site.site()).unwrap().unwrap();
    let loan = row.scalar_result_source_v1().expect("original Bool source loan");
    assert_eq!(loan.kind(), SourceScalarKind::Bool);
    corroborate_source_scalar_result_loan_v1(
        ledger.borrowed_formal_source.as_ref().unwrap(), &ledger.borrowed_i64_results,
        loan, row.source_target(), &package.result_contracts,
    ).unwrap();
    assert!(!ledger.uses_object_packet_protocol_v1(&row).unwrap());
    let ingress = ledger.borrowed_formal_source.as_mut().unwrap().as_mut().unwrap();
    ingress.incoming = ingress.incoming.iter().filter(|call| call.call != site)
        .cloned().collect::<Vec<_>>().into_boxed_slice();
    ingress.definitions.remove(&row.callee_owner());
    assert!(ledger.borrowed_call_actuals_v1(&row).unwrap_err()
        .contains("bool-input-incomplete"));
    // Removing the selected original result must invalidate the already retained row.
    ledger.borrowed_i64_results.remove(&row.callee_owner());
    assert!(validate_source_scalar_result_loan_v1(
        ledger.borrowed_formal_source.as_ref().unwrap(), &ledger.borrowed_i64_results,
        loan, row.source_target(),
    ).unwrap_err().contains("scalar-loan/result-missing"));
}
