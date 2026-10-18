//! The existing source actual family gains whole zero-input finish boundaries.
use super::*;
type Package =
    crate::mir::normal_callable_semantic_package::VerifiedNormalCallableSemanticPackageV1;
fn package(text: &str) -> Package {
    crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(
        text,
    )
    .unwrap()
}
const TEXT: &str = "static box Layout { word() { return 8 } run() { local a = me.word() local b = me.word() return a + b } } static box Main { main() { return 0 } }";
fn originals(package: &Package) -> Vec<Rc<StaticIncomingSourceV1>> {
    package
        .ordinary_new_claim_ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap()
        .source_incoming
        .exact_rows()
        .filter_map(|call| match &call.source {
            super::super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(
                original,
            ) if original.target().name() == "word" => Some(Rc::clone(original)),
            _ => None,
        })
        .collect()
}
fn restage(package: &mut Package, sources: &[Rc<StaticIncomingSourceV1>]) {
    let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    let ingress = ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    for source in sources {
        let rows = super::super::static_source::prepare_static_source_actuals_v1(
            ingress,
            &package.parameter_contracts,
            source.call_site(),
            &[],
        )
        .unwrap()
        .unwrap();
        ledger
            .borrowed_formal_actuals
            .insert(source.call_site().clone(), Ok(rows));
    }
}
fn finish(package: &mut Package) -> Result<(), String> {
    Rc::get_mut(&mut package.ordinary_new_claim_ledger)
        .unwrap()
        .finish_static_input_actuals_v1(
            &package.selected,
            &package.parameter_contracts,
            &package.physical_signature,
            &package.result_contracts,
        )
}
#[test]
fn static_zero_input_finish_preserves_original_cohort_and_refusal_boundaries() {
    let ready = package(TEXT);
    let calls = originals(&ready);
    assert_eq!(calls.len(), 2);
    for call in &calls {
        let ledger = &ready.ordinary_new_claim_ledger;
        assert!(ledger
            .checked_completed_static_zero_arguments_v1(call)
            .unwrap()
            .unwrap()
            .is_empty());
        assert!(
            !ledger
                .borrowed_formal_source
                .as_ref()
                .unwrap()
                .as_ref()
                .unwrap()
                .definitions
                .contains_key(&call.callee_owner()),
            "no fake borrowed callee entry"
        );
    }
    let annotated = package(&TEXT.replacen("word()", "word(): i64", 1));
    for call in originals(&annotated) {
        assert!(annotated
            .ordinary_new_claim_ledger
            .checked_completed_static_zero_arguments_v1(&call)
            .unwrap()
            .is_some());
    }
    // Local, both binary orders, condition, direct/binary terminal and a
    // qualified sibling all stage the original zero-input source.
    for body in [
        "local a = me.word() return a",
        "local a = me.word() + 1 return a",
        "local a = 1 + me.word() return a",
        "if me.word() > 0 { return 1 } return 0",
        "return me.word()",
        "return 1 + me.word()",
        "local a = Layout.word() return me.word()",
    ] {
        let variant = package(&format!("static box Layout {{ word() {{ return 8 }} run() {{ {body} }} }} static box Main {{ main() {{ return 0 }} }}"));
        let calls = originals(&variant);
        assert!(!calls.is_empty(), "{body}");
        for call in calls {
            assert!(
                variant
                    .ordinary_new_claim_ledger
                    .checked_completed_static_zero_arguments_v1(&call)
                    .unwrap()
                    .is_some(),
                "{body}"
            );
        }
    }
    for mutation in 0..9 {
        let mut variant = package(TEXT);
        let calls = originals(&variant);
        let owner = calls[0].callee_owner();
        restage(&mut variant, &calls);
        let ledger = Rc::get_mut(&mut variant.ordinary_new_claim_ledger).unwrap();
        match mutation {
            0 => {
                ledger.borrowed_formal_actuals.remove(calls[1].call_site());
            }
            1 => {
                ledger.borrowed_formal_actuals.insert(
                    calls[1].call_site().clone(),
                    Err("original-sibling-refusal".into()),
                );
            }
            2 => {
                ledger.completion_index.remove(&owner);
            }
            3 => {
                ledger.completion_index.insert(owner, Err(crate::mir::resolved_control_flow::FunctionCompletionVerificationErrorV1::OwnerClosureMismatch));
            }
            4 => {
                let unrelated = ledger
                    .completion_index
                    .iter()
                    .find(|(key, row)| **key != owner && row.is_ok())
                    .unwrap()
                    .1
                    .as_ref()
                    .unwrap()
                    .clone();
                ledger.completion_index.insert(owner, Ok(unrelated));
            }
            5 => {
                let rows = ledger
                    .borrowed_formal_actuals
                    .get_mut(calls[1].call_site())
                    .unwrap()
                    .as_mut()
                    .unwrap();
                rows.ordered_arguments = vec![LocalCallArgumentV1::Integer(8)].into_boxed_slice();
            }
            6 => {
                ledger
                    .borrowed_formal_source
                    .as_mut()
                    .unwrap()
                    .as_mut()
                    .unwrap()
                    .source_incoming
                    .duplicate_object_row_for_test(calls[1].call_site());
            }
            7 => {
                let rows = ledger
                    .borrowed_formal_actuals
                    .get_mut(calls[1].call_site())
                    .unwrap()
                    .as_mut()
                    .unwrap();
                let BorrowedCallActualEvidencePhaseV1::SourceStatic(identity) = &mut rows.phase
                else {
                    panic!("source phase");
                };
                identity.source = Rc::clone(&calls[0]);
            }
            _ => {}
        }
        let result = if mutation == 8 {
            let foreign = package(TEXT);
            Rc::get_mut(&mut variant.ordinary_new_claim_ledger)
                .unwrap()
                .finish_static_input_actuals_v1(
                    &variant.selected,
                    &variant.parameter_contracts,
                    &foreign.physical_signature,
                    &variant.result_contracts,
                )
        } else {
            finish(&mut variant)
        };
        if mutation >= 4 {
            assert!(result.is_err(), "mutation {mutation}: {result:?}");
        }
        let ledger = &variant.ordinary_new_claim_ledger;
        assert!(
            ledger.borrowed_formal_actuals[calls[0].call_site()]
                .as_ref()
                .map_or(true, |row| row.require_executable_v1().is_err()),
            "no partial install, mutation {mutation}"
        );
    }
    for mutation in 0..7 {
        let mut variant = package(TEXT);
        let calls = originals(&variant);
        if mutation == 6 {
            restage(&mut variant, &calls[1..]);
        }
        let ledger = Rc::get_mut(&mut variant.ordinary_new_claim_ledger).unwrap();
        match mutation {
            0 => {
                ledger.borrowed_formal_actuals.remove(calls[1].call_site());
            }
            1 => {
                ledger.borrowed_formal_actuals.insert(
                    calls[1].call_site().clone(),
                    Err("late-sibling-refusal".into()),
                );
            }
            2 => {
                ledger.completion_index.remove(&calls[0].callee_owner());
            }
            3 => {
                ledger
                    .borrowed_formal_source
                    .as_mut()
                    .unwrap()
                    .as_mut()
                    .unwrap()
                    .source_incoming
                    .duplicate_object_row_for_test(calls[1].call_site());
            }
            5 => {
                let foreign = package(TEXT);
                let foreign_owner = originals(&foreign)[0].callee_owner();
                let replacement = foreign.ordinary_new_claim_ledger.completion_index
                    [&foreign_owner]
                    .as_ref()
                    .unwrap()
                    .clone();
                ledger
                    .completion_index
                    .insert(calls[0].callee_owner(), Ok(replacement));
            }
            6 => {}
            _ => {
                let foreign = package(TEXT);
                let foreign_call = originals(&foreign)[0].clone();
                let issue = ledger
                    .checked_completed_static_zero_arguments_v1(&foreign_call)
                    .unwrap_err();
                assert!(
                    issue.contains("original-source-drift")
                        || issue.contains("original-source-missing"),
                    "{issue}"
                );
            }
        }
        if mutation != 4 {
            assert!(
                ledger
                    .checked_completed_static_zero_arguments_v1(&calls[0])
                    .map_or(true, |rows| rows.is_none()),
                "late mutation {mutation}"
            );
        }
    }
    let unknown = package("static box Layout { word() { return 8 } run(p) { local a = me.word() local b = p.word() return 0 } } static box Main { main() { return 0 } }");
    let calls = originals(&unknown);
    assert_eq!(
        calls.len(),
        1,
        "known source retained beside unknown caller"
    );
    assert!(unknown
        .ordinary_new_claim_ledger
        .checked_completed_static_zero_arguments_v1(&calls[0])
        .unwrap_err()
        .contains("incoming-coverage"));
}

#[test]
fn current_owner_static_scalar_return_retains_its_exact_child_terminal() {
    let ready = package("static box Layout { word(bin) { if bin <= 8 { return bin * 8 } return 0 } relay(bin: usize) { return me.word(bin) } } static box Main { main() { return 0 } }");
    let ledger = &ready.ordinary_new_claim_ledger;
    let incoming = ledger.borrowed_formal_source.as_ref().unwrap().as_ref().unwrap();
    let call = incoming.source_incoming.exact_rows()
        .find(|row| matches!(&row.source,
            super::super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(source)
                if source.target().name() == "word"))
        .unwrap();
    let owner = call.call.owner();
    let completion = ledger.completion_for_owner(owner).unwrap();
    let [exit] = completion.explicit_sites() else { panic!("one exact relay exit required") };
    let (_, terminal) = ledger.call_source_completion_for_owner_at(owner, exit)
        .expect("original Static scalar return must survive child coseal");
    assert_eq!(terminal.owner(), owner);
    assert_eq!(terminal.return_site(), exit);
    assert_eq!(terminal.call_site(), call.call.site());
    assert!(ledger.selected_static_local_source_v1(&call.call).unwrap().is_some());
    assert!(ledger.has_routed_static_local_for_owner_v1(owner).unwrap());
    assert!(ledger.normal_exit_projection_v1(owner, exit).unwrap().is_some());
}

#[test]
fn static_expression_input_retains_original_frames_and_atomic_cohort() {
    let text = "static box Values { sink(p, q) { return 0 } run(): i64 { local a = Values.sink(\"abc\".substring(0,2).substring(0,1), \"xy\".substring(0,1)) local b = Values.sink(\"def\".substring(0,1), \"yz\".substring(0,1)) return 0 } } static box Main { main(args) { local out = Values.run() return 0 } }";
    let make = || crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog_selection(
        text, crate::mir::normal_callable_semantic_package::PackagePhysicalSelectionV1::ClosedAppExecutable,
    ).unwrap();
    let originals = |package: &Package| -> Vec<Rc<StaticIncomingSourceV1>> {
        package.ordinary_new_claim_ledger.borrowed_formal_source.as_ref().unwrap().as_ref().unwrap()
            .source_incoming.source_rows_v1().filter_map(|row| match &row.source {
                super::super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(original)
                    if original.target().name() == "sink" => Some(Rc::clone(original)),
                _ => None,
            }).collect()
    };
    // Semantic issue can retain unsupported input as passive source. The
    // contract is no executable actual and no Text source loan for Token.
    let declared_object = format!("box Token {{}} {}", text.replacen("sink(p, q)", "sink(p: Token, q)", 1));
    let unready = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog_selection(
        &declared_object, crate::mir::normal_callable_semantic_package::PackagePhysicalSelectionV1::ClosedAppExecutable,
    ).unwrap();
    let refusal = unready.ordinary_new_claim_ledger.borrowed_formal_source.as_ref().unwrap()
        .as_ref().unwrap_err();
    assert!(refusal.contains("borrowed-view/declared-object-class"), "{refusal}");
    let mut refused = 0;
    for declaration in unready.batch().declarations() {
        unready.batch().with_lowering_input(declaration.batch_slot(), |input| {
            for (site, call) in input.function().method_calls().filter(|(_, call)| call.selector() == "sink") {
                let error = unready.ordinary_new_claim_ledger.borrowed_expression_actual_source_v1(
                    &OwnedExprSiteV1::new(input.owner(), site.clone()), 0, call.arguments()[0].site()).unwrap_err();
                assert!(error.contains("borrowed-view/declared-object-class"), "{error}");
                refused += 1;
            }
        }).unwrap();
    }
    assert_eq!(refused, 2, "every original call demands the retained class refusal");
    let ready = make();
    let calls = originals(&ready);
    assert_eq!(calls.len(), 2);
    let ledger = &ready.ordinary_new_claim_ledger;
    let domains = ledger.checked_static_input_domains_for_owner_v1(calls[0].callee_owner()).unwrap();
    assert_eq!(domains.len(), 2);
    let ingress = ledger.borrowed_formal_source.as_ref().unwrap().as_ref().unwrap();
    for (ordinal, domain) in domains.iter().enumerate() {
        assert_eq!(domain.ordinal(), ordinal as u32);
        assert!(domain.text_only());
        assert!(Rc::ptr_eq(domain.source_sum_v1(),
            &ingress.lend_source_value_sum_v1(domain.binding()).unwrap()));
    }
    for call in &calls {
        let row = ledger.checked_completed_static_expression_input_v1(call).unwrap().unwrap();
        row.require_executable_v1().unwrap();
        let snapshot = row.source_static_snapshot_v1().unwrap();
        assert!(snapshot.require_executable_v1().is_err(), "retained source snapshot stays passive");
        assert!(snapshot.opaque_actuals.is_empty());
        assert_eq!(row.opaque_actuals.len(), 2);
        for actual in row.opaque_actuals.iter() {
            let canonical = ledger.qualified_argument_expression_frame_v1(call.call_site(), actual.ordinal, &actual.site).unwrap();
            let BorrowedFormalActualSourceV1::Expression(source) = &actual.source else { panic!("computed Text source required") };
            assert!(Rc::ptr_eq(source.frame(), &canonical));
            assert!(actual.source.caller_formal_origin_v1().is_none());
            assert_eq!(actual, &ledger.borrowed_expression_actual_source_v1(call.call_site(), actual.ordinal, &actual.site).unwrap());
            assert!(ledger.argument_expression_frame_v1(call.call_site(), actual.ordinal, &actual.site).is_err(),
                "computed Text cannot enter the direct scalar finisher");
        }
    }
    // Completed consumers must reject reordered, missing and equal-reissued
    // domains rather than reconstructing permission from physical Text values.
    for mutation in 0..3 {
        let mut variant = make();
        let calls = originals(&variant);
        let ledger = Rc::get_mut(&mut variant.ordinary_new_claim_ledger).unwrap();
        let rows = ledger.borrowed_formal_actuals.get_mut(calls[1].call_site()).unwrap().as_mut().unwrap();
        let BorrowedCallActualEvidencePhaseV1::ExecutableStaticInput(proof) = &mut rows.phase else { unreachable!() };
        let StaticInputEvidenceV1::Qualified { domains, .. } = &mut proof.evidence else { unreachable!() };
        match mutation {
            0 => domains.swap(0, 1),
            1 => *domains = Box::new([]),
            2 => {
                let original = &domains[0];
                domains[0] = super::super::super::borrowed_formal_entry::BorrowedFormalDomainV1::from_source_v1(
                    original.ordinal(), original.binding(), Rc::new(original.source_sum_v1().as_ref().clone()));
            }
            _ => unreachable!(),
        }
        assert!(ledger.checked_completed_static_expression_input_v1(&calls[0]).is_err(), "domain mutation {mutation}");
        assert!(ledger.checked_static_input_domains_for_owner_v1(calls[0].callee_owner()).is_err(), "domain mutation {mutation}");
    }
    for mutation in 0..6 {
        let mut variant = make();
        let calls = originals(&variant);
        let ledger = Rc::get_mut(&mut variant.ordinary_new_claim_ledger).unwrap();
        for call in &calls {
            let snapshot = ledger.borrowed_formal_actuals[call.call_site()].as_ref().unwrap()
                .source_static_snapshot_v1().unwrap().clone();
            ledger.borrowed_formal_actuals.insert(call.call_site().clone(), Ok(snapshot));
        }
        match mutation {
            0 => { ledger.borrowed_formal_actuals.remove(calls[1].call_site()); }
            1 => { ledger.borrowed_formal_actuals.insert(calls[1].call_site().clone(), Err("qualified-sibling-refusal".into())); }
            2 => { ledger.completion_index.remove(&calls[0].callee_owner()); }
            3 | 4 => {
                let rows = ledger.borrowed_formal_actuals.get_mut(calls[1].call_site()).unwrap().as_mut().unwrap();
                let BorrowedCallActualEvidencePhaseV1::SourceStatic(identity) = &mut rows.phase else { unreachable!() };
                if mutation == 3 { identity.candidates[0].ordinal = 1; }
                else {
                    let super::super::static_source::StaticArgumentExpressionV1::Qualified(frame) = identity.argument_sources.get_mut(&0).unwrap()
                        else { panic!("original qualified frame") };
                    *frame = Rc::new(frame.as_ref().clone());
                }
            }
            5 => {
                let foreign = ledger.completion_index.iter().find(|(owner, row)|
                    **owner != calls[0].callee_owner() && row.is_ok()).unwrap().1.as_ref().unwrap().clone();
                ledger.completion_index.insert(calls[0].callee_owner(), Ok(foreign));
            }
            _ => unreachable!(),
        }
        let result = finish(&mut variant);
        if mutation >= 3 { assert!(result.is_err(), "mutation {mutation}: {result:?}"); }
        assert!(variant.ordinary_new_claim_ledger.borrowed_formal_actuals[calls[0].call_site()]
            .as_ref().map_or(true, |row| row.require_executable_v1().is_err()), "no partial install: {mutation}");
    }
    // A demand must not turn a merely finishable sibling into an executed row.
    let mut variant = make();
    let calls = originals(&variant);
    let ledger = Rc::get_mut(&mut variant.ordinary_new_claim_ledger).unwrap();
    let snapshot = ledger.borrowed_formal_actuals[calls[1].call_site()].as_ref().unwrap()
        .source_static_snapshot_v1().unwrap().clone();
    ledger.borrowed_formal_actuals.insert(calls[1].call_site().clone(), Ok(snapshot));
    assert!(ledger.checked_completed_static_expression_input_v1(&calls[0]).is_err());
    assert!(ledger.borrowed_formal_actuals[calls[1].call_site()].as_ref().unwrap().require_executable_v1().is_err());
}
