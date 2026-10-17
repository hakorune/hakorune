//! Original Instance input/result laws and whole-cohort refusals.
use super::*;

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
        let rows = super::super::super::instance_source::prepare_instance_source_actuals_v1(
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
    super::super::super::recheck_finished_instance_input_v1(
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
    assert!(super::super::super::recheck_finished_instance_input_v1(
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
    assert!(super::super::super::recheck_finished_instance_input_v1(
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
    assert!(super::super::super::recheck_finished_instance_input_v1(
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
