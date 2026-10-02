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
    for body in ["return true", "return \"text\"", "return null"] {
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
    proof.returns[0] = OwnedExprSiteV1::new(row.callee_owner(), row.argument_sites()[0].clone());
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
        &BTreeMap::new(),
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
