//! Source-only Instance phase is distinct from executable forwarding.
use super::*;

#[test]
fn original_instance_forward_source_refuses_execution_and_snapshot_drift() {
    // The same forwarding source without a caller entry remains nonexecuting.
    // Original guarded Mul is kept, followed by the original borrowed call.
    let source = "box Transport { birth() { } probe(p): i64 {
        if p <= 8 { return p * 2 } local out = me.sink(p) return 0
    } sink(q): i64 { return 0 } } static box Main { main() { return 0 } }";
    let mut package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(source).unwrap();
    // Check the source producer's boundary independently of later input closure.
    // Restaging passive actuals does not revoke the preexisting Completion;
    // this row itself must still refuse executable argument lending.
    let ledger = &package.ordinary_new_claim_ledger;
    let (site, identity) = ledger.borrowed_formal_actuals.iter().find_map(|(site, rows)| {
        let rows = rows.as_ref().ok()?;
        match &rows.phase {
            BorrowedCallActualEvidencePhaseV1::SourceInstance(identity)
            | BorrowedCallActualEvidencePhaseV1::ExecutableSourceInstance(identity) => Some((site.clone(), identity.clone())),
            _ => None,
        }
    }).expect("original Instance source identity retained");
    let mut staged = super::prepare_instance_source_actuals_v1(
        ledger.borrowed_formal_source.as_ref().unwrap().as_ref().unwrap(),
        &package.parameter_contracts, &site, &identity.candidates,
    ).unwrap().unwrap();
    let slot = package.batch().declarations().find(|row| row.owner() == site.owner()).unwrap().batch_slot();
    package.batch().with_lowering_input(slot, |input| {
        let previous = Ok(staged.clone());
        super::super::bind_argument_sources_v1(input,
            ledger.borrowed_formal_source.as_ref().unwrap().as_ref().unwrap(),
            Some(&package.parameter_contracts), &site, Some(&previous), Some(&mut staged)).unwrap();
        let prepared = ledger.borrowed_formal_source.as_ref().unwrap().as_ref().unwrap();
        let method = input.function().method_call_source_v1(site.site()).unwrap();
        let ordinary = super::super::call_result_source::ordinary_argument_call_source_v1(
            prepared, &site, method).unwrap().unwrap();
        ordinary.validate_input_v1(input, prepared, Some(&package.parameter_contracts)).unwrap();
        assert!(ordinary.validate_input_v1(input, prepared, None).is_err(),
            "passive Instance syntax cannot replace the complete declaration census");
        let foreign = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(source).unwrap();
        foreign.batch().with_lowering_input(slot, |other| {
            let method = other.function().method_call_source_v1(site.site()).unwrap();
            assert!(super::super::call_result_source::ordinary_argument_call_source_v1(
                prepared, &site, method).is_err(), "equal syntax cannot replace the retained original method");
        }).unwrap();
        assert_eq!(ordinary, super::super::call_result_source::original_argument_call_source_v1(
            prepared, &site).unwrap().unwrap());
    }).unwrap();
    let ledger = std::rc::Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    let mut observed = BTreeMap::from([(site.clone(), Ok(super::prepare_instance_source_actuals_v1(
        ledger.borrowed_formal_source.as_ref().unwrap().as_ref().unwrap(),
        &package.parameter_contracts, &site, &identity.candidates).unwrap().unwrap()))]);
    super::super::stage_observed_call_actuals_v1(&mut observed, &site, Ok(Some(staged)));
    ledger.borrowed_formal_actuals.insert(site.clone(), observed.remove(&site).unwrap());
    let ledger = &package.ordinary_new_claim_ledger;
    let (site, rows) = ledger.borrowed_formal_actuals.iter()
        .find(|(_, rows)| matches!(rows, Ok(rows) if matches!(rows.phase, BorrowedCallActualEvidencePhaseV1::SourceInstance(_))))
        .expect("canonical source producer remains nonexecuting");
    let rows = rows.as_ref().unwrap();
    assert!(rows.opaque_actuals.is_empty());
    assert!(rows
        .require_executable_v1()
        .unwrap_err()
        .contains("source-only-instance"));
    let ingress = ledger.borrowed_formal_source.as_ref().unwrap();
    let arguments = project_pending_instance_source_arguments_v1(
        ingress,
        &ledger.borrowed_formal_actuals,
        &package.parameter_contracts,
        site,
    )
    .unwrap()
    .unwrap();
    assert_eq!(arguments, rows.ordered_arguments);
    let target = ingress.as_ref().unwrap().source_incoming.exact_rows()
        .find(|row| &row.call == site).unwrap().source.require_instance().unwrap();
    let common = super::super::object_arguments::project_pending_object_target_arguments_v1(
        ingress, &package.parameter_contracts, &ledger.borrowed_formal_actuals, target, None,
    ).unwrap();
    let crate::mir::resolved_semantics::home_new_prefix::ObjectCallSourceSupportV1::SourceOnly(common) = common
        else { panic!("original Instance protocol stays source-only at the common target port") };
    assert_eq!(common, arguments);
    assert!(
        lend_pending_borrowed_arguments_v1(ingress, &ledger.borrowed_formal_actuals, site)
            .unwrap_err()
            .contains("source-only-instance")
    );
    for mutation in 0..5 {
        let mut pending = ledger.borrowed_formal_actuals.clone();
        let row = pending.get_mut(site).unwrap().as_mut().unwrap();
        let BorrowedCallActualEvidencePhaseV1::SourceInstance(identity) = &mut row.phase else {
            unreachable!()
        };
        match mutation {
            0 => identity.candidates[0].ordinal = 1,
            1 => identity.candidates[0].site = site.site().clone(),
            2 => identity.incoming_arguments = Box::new([]),
            3 => row.ordered_arguments = Box::new([]),
            _ => row.phase = BorrowedCallActualEvidencePhaseV1::Executable,
        }
        assert!(
            project_pending_instance_source_arguments_v1(
                ingress,
                &pending,
                &package.parameter_contracts,
                site,
            )
            .is_err(),
            "source snapshot mutation {mutation}"
        );
        assert!(super::super::object_arguments::project_pending_object_target_arguments_v1(
            ingress, &package.parameter_contracts, &pending, target, None,
        ).is_err(), "common target retains source snapshot refusal {mutation}");
    }
    let site = site.clone();
    let mut pending = ledger.borrowed_formal_actuals.clone();
    pending.insert(site.clone(), Err("original-instance-source-error".into()));
    let ledger = std::rc::Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    let ingress = ledger.borrowed_formal_source.as_mut().unwrap();
    ingress.as_mut().unwrap().incoming = Box::new([]);
    assert_eq!(
        project_pending_instance_source_arguments_v1(
            ingress,
            &pending,
            &package.parameter_contracts,
            &site,
        )
        .unwrap_err(),
        "original-instance-source-error"
    );
    pending.remove(&site);
    assert!(project_pending_instance_source_arguments_v1(
        ingress,
        &pending,
        &package.parameter_contracts,
        &site,
    )
    .unwrap_err()
    .contains("source-actuals-missing"));

    // A staged SourceInstance cannot survive loss of its original source row.
    let staged = ledger.borrowed_formal_actuals.clone();
    ingress
        .as_mut()
        .unwrap()
        .source_incoming
        .remove_exact_row_for_test(&site);
    assert!(project_pending_instance_source_arguments_v1(
        ingress,
        &staged,
        &package.parameter_contracts,
        &site,
    )
    .unwrap_err()
    .contains("source-phase-drift"));
}

use super::super::super::super::result_class_claim::OrdinaryNewResultClassClaimsV1;
use super::super::super::borrowed_formal_source::{
    collect_borrowed_source_drafts_v1, finish_ingress_from_drafts_v1, prepare_original_incoming_v1,
};
use std::collections::BTreeSet;
#[test]
fn array_actual_retains_original_shared_loan_and_refuses_executable_upgrade() {
    let text = "box Token { value: i64 birth(v) { me.value = v } } box Door { make(flag) { if flag == 0 { return null } return new Token(1) } consume(token) { if token == null { return 0 } return 1 } consumeBad(token) { if token == null { return 0 } return 1 } collect(flag) { if flag == 0 { return 0 } local bad = new ArrayBox() bad.push(0) me.consumeBad(bad.get(0)) local refused = new ArrayBox() refused.push(me.consumeBad(null)) me.consumeBad(refused.get(0)) local arr = new ArrayBox() arr.push(me.make(flag)) me.consume(arr.get(0)) arr.push(me.make(2)) me.consume(arr.get(1)) return 0 } inspect(flag) { local good = me.make(flag) me.consumeBad(good) return 0 } forward(token: Token) { me.consume(token) return 0 } } static box Main { main() { return 0 } }";
    super::super::super::super::result_class_claim::with_source_claims_fixture(
        text,
        |batch, selected, constructors, parameters, main, writers| {
            let (fields, _, classes, _) =
                super::super::super::super::coseal_issue::source_claims::prepare_source_claims(
                    batch,
                    selected,
                    main,
                    constructors,
                    parameters,
                    std::rc::Rc::clone(writers), None,
                )
                .unwrap();
            let names: Vec<Box<str>> = batch
                .ordinary_box_coverage()
                .rows()
                .iter()
                .map(|row| row.name().into())
                .collect();
            let needs = super::super::super::super::lexical_instance_call::prepare_lexical_source_targets_v1(batch, selected, &BTreeMap::new(), &names, &fields, &classes, parameters, main.map(|main| main.caller_source_ref_v1()), &mut |_,_,_| Ok(None), None, None).unwrap();
            let mut calls = BTreeMap::new();
            for need in &needs {
                if let Some(super::super::super::super::lexical_instance_call::PreparedSourceCallNeedV1::Lexical(row)) = need.as_ref().unwrap() {
                calls.insert(row.call_site().clone(), row);
            }
            }
            let target = calls
                .values()
                .find(|row| row.target().name() == "consume")
                .unwrap();
            let entry =
                crate::mir::resolved_semantics::VerifiedInstanceEntryHomeCatalogV1::issue(
                    Box::new([]),
                );
            let drafts = collect_borrowed_source_drafts_v1(
                batch,
                selected,
                parameters,
                None,
                main.map(|row| row.batch_slot()),
                None,
                &entry,
                constructors,
                None,
            )
            .unwrap();
            let brands = crate::analysis::brand_program_declaration_catalog::issue_brand_program_declaration_catalog_v1(batch.source_ast()).unwrap();
            let inventory = prepare_original_incoming_v1(
                Some(&brands), batch, selected, &BTreeMap::new(), parameters, &calls, &drafts.0, &drafts.1,
                None, &BTreeMap::new(), main, None, constructors,
                &entry, &BTreeMap::new(), &classes, &drafts.3, &BTreeMap::new(), &[], &[], None,
            );
            let mut ingress = finish_ingress_from_drafts_v1(
                Some(&brands),
                batch,
                selected,
                parameters,
                &entry,
                constructors,
                &BTreeMap::new(),
                &classes,
                calls.clone(),
                None,
                main,
                drafts,
                BTreeMap::new(),
                inventory,
            )
            .unwrap();
            assert!(ingress.original_array_argument_source_v1(target).unwrap());
            let refused = calls.values().filter(|row| row.target().name() == "consumeBad")
                .find_map(|row| {
                    let read = OwnedExprSiteV1::new(row.call_site().owner(), row.argument_sites()[0].clone());
                    if ingress.array_element_source_v1(&read).is_ok() { return None; }
                    ingress.array_holder_structure_source_v1(&read).ok().flatten()
                }).unwrap();
            assert_eq!(refused.appends().len(), 1);
            let read = OwnedExprSiteV1::new(target.call_site().owner(), target.argument_sites()[0].clone());
            assert!(std::ptr::eq(ingress.array_holder_structure_source_v1(&read).unwrap().unwrap().as_ref(),
                ingress.array_element_source_v1(&read).unwrap().unwrap().holder()));
            let forward = calls.values().find(|row| row.target().name() == "consume"
                && parameters.iter().find(|contract| contract.owner == row.call_site().owner())
                    .unwrap().parameters.iter().any(|formal| matches!(formal.kind, CallableParameterContractKindV1::DeclaredObject(_)))).unwrap();
            let root = parameters.iter().find(|contract| contract.owner == forward.call_site().owner()).unwrap().parameters[0].binding;
            let forward_candidates = [BorrowedCallActualCandidateV1 { ordinal: 0,
                site: forward.argument_sites()[0].clone(), value: BorrowedCallActualValueV1::SelfRooted { binding: root, root } }];
            let forward_rows = prepare_instance_source_actuals_v1(&ingress, parameters, forward.call_site(), &forward_candidates).unwrap().unwrap();
            let forward_original = ingress.source_incoming.exact_rows().find(|row| &row.call == forward.call_site()).unwrap();
            assert!(revalidate_passive_instance_incoming_v1(&ingress, parameters, forward_original, &forward_rows, &classes).unwrap());
            assert!(forward_rows.require_executable_v1().is_err(), "passive validation lends no execution");
            let ordinary = calls.values().find(|row| row.target().name() == "make").unwrap();
            assert!(!ingress.original_array_argument_source_v1(ordinary).unwrap(),
                "ordinary scalar arguments do not select the Array source family");
            let bad = calls.values().find(|row| row.target().name() == "consumeBad"
                && ingress.array_element_source_v1(&OwnedExprSiteV1::new(row.call_site().owner(), row.argument_sites()[0].clone())).is_err()).unwrap();
            assert!(ingress.original_array_argument_source_v1(bad).is_err(),
                "selected source refusal cannot turn into unselection");
            // The initial Home walk borrows the same holder before any
            // producer Normal seal. Both past and future Push obligations
            // must retain its exact source identity.
            let contract = parameters.iter().find(|row| row.owner == target.call_site().owner()).unwrap();
            batch.with_lowering_input(contract.batch_slot, |input| {
                let ledger = input.forest().callable_source_ledger(input.owner()).unwrap();
                let read = ledger.method_call(&target.argument_sites()[0]).unwrap();
                let identity = crate::mir::resolved_semantics::NamedArrayConstructionSourceV1::issue(
                    &ledger, read, batch.ordinary_box_coverage(), &brands,
                ).unwrap();
                let construction = OwnedExprSiteV1::new(identity.owner(), identity.construction().clone());
                let holder = ingress.named_array_holder_source_v1(&construction, identity.binding(), true).unwrap().unwrap();
                assert_eq!(holder.binding(), identity.binding());
                assert_eq!(holder.construction(), construction);
                let appends: Vec<_> = ledger.method_calls().filter_map(|(site, _)| {
                    let call = OwnedExprSiteV1::new(input.owner(), site.clone());
                    holder.append_candidate(&call).map(|candidate| (call, candidate))
                }).collect();
                assert_eq!(appends.len(), 2);
                for (call, candidate) in appends {
                    let current = ingress.named_array_holder_source_v1(&call, identity.binding(), false).unwrap().unwrap();
                    assert_eq!(current, holder);
                    assert_eq!(current.append_candidate(&call), Some(candidate));
                }
                assert!(ingress.named_array_holder_source_v1(target.call_site(), identity.binding(), true).unwrap().is_none());
                let bad_target = calls.values().find(|row| row.target().name() == "consumeBad").unwrap();
                let bad_read = ledger.method_call(&bad_target.argument_sites()[0]).unwrap();
                let bad = crate::mir::resolved_semantics::NamedArrayConstructionSourceV1::issue(
                    &ledger, bad_read, batch.ordinary_box_coverage(), &brands,
                ).unwrap();
                let bad_construction = OwnedExprSiteV1::new(bad.owner(), bad.construction().clone());
                let denied = ingress.named_array_holder_source_v1(&bad_construction, bad.binding(), true).unwrap_err();
                assert_eq!(ingress.array_element_source_v1(&OwnedExprSiteV1::new(input.owner(), bad_target.argument_sites()[0].clone())).unwrap_err(), denied,
                    "holder and original read retain the same selected refusal");
                // An additional good incoming class cannot repair the
                // selected Array refusal or mint a formal class agreement.
                let bad_formal = parameters.iter().find(|row| row.owner == bad_target.callee_owner()).unwrap().parameters[0].binding;
                assert!(ingress.source_object_view_v1(bad_formal).unwrap().is_none());
            }).unwrap();
            let candidates = vec![BorrowedCallActualCandidateV1 {
                ordinal: 0,
                site: target.argument_sites()[0].clone(),
                value: batch.with_lowering_input(contract.batch_slot, |input|
                    crate::mir::resolved_semantics::home_new_prefix::borrowed_actual_source_atom_v1(input, &target.argument_sites()[0]).unwrap()).unwrap(),
            }];
            let rows = super::super::super::borrowed_formal_actuals::instance_source::prepare_instance_source_actuals_v1(&ingress, parameters, target.call_site(), &candidates).unwrap().unwrap();
            let super::super::super::borrowed_formal_actuals::model::BorrowedCallActualEvidencePhaseV1::SourceInstance(identity) = &rows.phase else { panic!("conditional source phase") };
            assert_eq!(identity.array_loans.len(), 1);
            let (_, loan) = &identity.array_loans[0];
            assert_eq!(loan.read().site(), &candidates[0].site);
            assert_eq!(loan.producer_sources_v1().count(), 2, "future producer remains mandatory");
            batch.with_lowering_input(contract.batch_slot, |input| {
                let completion = crate::mir::resolved_control_flow::verify_function_completion_v1(input).unwrap();
                let source_ledger = input.forest().callable_source_ledger(input.owner()).unwrap();
                let read = source_ledger.method_call(&target.argument_sites()[0]).unwrap();
                let holder = crate::mir::resolved_semantics::NamedArrayConstructionSourceV1::issue(
                    &source_ledger, read, batch.ordinary_box_coverage(), &brands,
                ).unwrap();
                let crate::mir::resolved_semantics::SourcePathSegmentV1::Body(construction) = holder.construction().node().segments()[0] else { panic!("original holder") };
                let mut before = 0;
                let mut after = 0;
                for exit in completion.explicit_sites() {
                    let crate::mir::resolved_semantics::SourcePathSegmentV1::Body(order) = exit.node().segments()[0] else { panic!("original body exit") };
                    let acquired = order > construction;
                    assert_eq!(loan.validate_holder_exit_source_v1(exit, usize::from(acquired)).unwrap(), acquired);
                    assert!(loan.validate_holder_exit_source_v1(exit, usize::from(!acquired)).is_err());
                    assert!(loan.validate_holder_exit_source_v1(exit, 2).is_err());
                    if acquired { after += 1 } else { before += 1 }
                }
                assert!(before > 0 && after > 0, "both original acquisition boundaries covered");
            }).unwrap();
            // Original Append membership registers both real producer calls in
            // the sole census; source/input identity still grants no execution.
            let producer_inputs = loan.original_producer_inputs_v1(&ingress, parameters).unwrap();
            assert_eq!(producer_inputs.len(), 2);
            for producer in loan.producer_sources_v1() {
                let current = ingress.original_array_producer_source_v1(producer.source()).unwrap().unwrap();
                assert!(std::ptr::eq(current, producer));
                assert!(ingress.original_object_producer_callee_v1(producer.source()).unwrap());
                assert!(producer_inputs.iter().any(|(row, _)| &row.call == producer.source().call_site()));
                let mut drift = producer.source().clone();
                drift.argument_sites = Box::new([]);
                assert!(ingress.original_array_producer_source_v1(&drift).unwrap_err().contains("original-target-drift"));
            }
            assert!(ingress.original_array_producer_source_v1(forward).unwrap().is_none());
            let refused_candidate = calls.values().find(|row| row.call_site().owner() == read.owner()
                && refused.appends().iter().any(|append| append.candidate() == row.call_site().site())).unwrap();
            let refused_read = OwnedExprSiteV1::new(read.owner(), refused.reads()[0].call().clone());
            assert_eq!(ingress.original_array_producer_source_v1(refused_candidate).unwrap_err(),
                ingress.array_element_source_v1(&refused_read).unwrap_err());

            assert!(rows.opaque_actuals.is_empty());
            loan.recheck(&ingress, &classes, &calls).unwrap();
            assert!(loan.recheck(&ingress, &classes, &BTreeMap::new()).is_err());
            assert!(rows.require_executable_v1().is_err());
            let original = ingress
                .source_incoming
                .exact_rows()
                .find(|row| &row.call == target.call_site())
                .unwrap();
            assert!(revalidate_passive_instance_incoming_v1(&ingress, parameters, original, &rows, &classes).unwrap(),
                "whole original source agreement grants no live Array read");
            use std::sync::Arc;
            let BorrowedCallActualValueV1::MethodCall(method) = &candidates[0].value else { panic!("original method source") };
            let (reissued, different) = batch.with_lowering_input(contract.batch_slot, |input| {
                let again = crate::mir::resolved_semantics::home_new_prefix::borrowed_actual_source_atom_v1(input, &candidates[0].site).unwrap();
                let BorrowedCallActualValueV1::MethodCall(again) = again else { panic!("same observation") };
                assert!(Arc::ptr_eq(method, &again));
                let mut issued = crate::mir::resolved_semantics::issue_resolved_method_call_sources_v1(input.body_shape().unwrap()).unwrap();
                let reissued = Arc::new(issued.remove(&candidates[0].site).unwrap());
                assert_eq!(method.as_ref(), reissued.as_ref());
                assert!(!Arc::ptr_eq(method, &reissued));
                let other = input.function().method_calls().find(|(site, row)|
                    *site != &candidates[0].site && row.selector() == "get").unwrap().0;
                (reissued, input.function().method_call_source_v1(other).unwrap().clone())
            }).unwrap();
            for mutation in 0..4 {
                let mut changed = candidates.clone();
                match mutation {
                    0 => changed[0].value = BorrowedCallActualValueV1::MethodCall(reissued.clone()),
                    1 => changed[0].value = BorrowedCallActualValueV1::MethodCall(different.clone()),
                    2 => changed[0].site = target.call_site().site().clone(),
                    _ => changed[0].ordinal = 1,
                }
                assert!(super::super::super::borrowed_formal_actuals::instance_source::prepare_instance_source_actuals_v1(
                    &ingress, parameters, target.call_site(), &changed).unwrap_err().contains("method-original-source-drift"));
                let mut forged = rows.clone();
                let super::super::super::borrowed_formal_actuals::model::BorrowedCallActualEvidencePhaseV1::SourceInstance(forged_identity) = &mut forged.phase else { panic!("source phase") };
                forged_identity.candidates = changed.into_boxed_slice();
                let forged_identity = forged_identity.clone();
                assert!(revalidate_passive_instance_incoming_v1(&ingress, parameters, original, &forged, &classes)
                    .unwrap_err().contains("method-original-source-drift"));
                assert!(super::super::super::borrowed_formal_actuals::instance_source::reconstruct_instance_source_actuals_v1(
                    &ingress, parameters, original, &forged_identity, None).unwrap_err().contains("method-original-source-drift"));
            }
            let mut drifted = rows.clone();
            drifted.ordered_arguments = Box::new([]);
            assert!(revalidate_passive_instance_incoming_v1(&ingress, parameters, original, &drifted, &classes)
                .unwrap_err().contains("reconstructed-snapshot"));
            assert!(super::super::super::borrowed_formal_actuals::instance_source::reconstruct_instance_source_actuals_v1(&ingress, parameters, original, identity, None).is_err());
            let foreign = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog("static box Main { main() { return 0 } }").unwrap();
            assert!(super::super::super::borrowed_formal_actuals::instance_source::reconstruct_instance_source_actuals_v1(
                &ingress, parameters, original, identity, Some(&foreign.ordinary_new_claim_ledger),
            ).unwrap_err().contains("foreign-ingress"));
            let mut forged = rows.clone();
            forged.phase = super::super::super::borrowed_formal_actuals::model::BorrowedCallActualEvidencePhaseV1::ExecutableSourceInstance(identity.clone());
            assert!(forged.require_executable_v1().is_err());
            assert!(revalidate_passive_instance_incoming_v1(&ingress, parameters, original, &forged, &classes)
                .unwrap_err().contains("array-execution-unconnected"));
            assert!(loan
                .recheck(&ingress, &OrdinaryNewResultClassClaimsV1::new(), &calls)
                .is_err());
            let original_producer = loan.producer_sources_v1().next().unwrap().source();
            let holder = ingress.array_holder_structure_source_v1(&read).unwrap().unwrap();
            assert_eq!(holder.reads().len(), 2);
            let sibling = holder.reads().iter().find(|row| row.call() != read.site()).unwrap();
            let sibling = OwnedExprSiteV1::new(read.owner(), sibling.call().clone());
            ingress.erase_array_read_for_test(&sibling);
            assert!(ingress.original_array_producer_source_v1(original_producer).unwrap_err().contains("original-read-missing"));

        },
    );
}

#[test]
fn owning_object_local_input_keeps_qualified_sibling_and_refuses_phase_and_census_drift() {
    for formal in ["size", "size: i64"] {
        let text = format!("box Token {{}} box Maker {{
            make({formal}) {{ if size <= 0 {{ return null }} return new Token() }}
            relay(size: i64) {{ local out = me.make(size) return out }}
            qualified() {{ return me.make(7) }}
        }} static box Main {{ main() {{ return 0 }} }}");
        let mut package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(&text).unwrap();
        let ledger = &package.ordinary_new_claim_ledger;
        let source = ledger.borrowed_formal_source.as_ref().unwrap();
        let ingress = source.as_ref().unwrap();
        let targets: Vec<_> = ingress.source_incoming.exact_rows()
            .filter_map(|row| row.source.instance())
            .filter(|target| target.target().name() == "make").cloned().collect();
        assert_eq!(targets.len(), 2, "LOCAL and direct sibling share one callee");
        let local = targets.iter().find(|target| target.owning_object_local_input_v1().is_some()).unwrap();
        let sibling = targets.iter().find(|target| target.owning_object_local_input_v1().is_none()).unwrap();
        for target in &targets {
            assert!(ingress.original_owning_object_input_callee_v1(target).unwrap());
            assert!(ingress.source_incoming.has_object_input_callee_v1(target));
            assert!(project_pending_instance_source_arguments_v1(source,
                &ledger.borrowed_formal_actuals, &package.parameter_contracts,
                target.call_site()).unwrap().is_some());
            let mut pending = ledger.borrowed_formal_actuals.clone();
            pending.get_mut(target.call_site()).unwrap().as_mut().unwrap().phase =
                BorrowedCallActualEvidencePhaseV1::SourceInstance(InstanceSourceActualIdentityV1 {
                    source: target.clone(), incoming_arguments: Box::new([]),
                    candidates: Box::new([]), argument_sources: BTreeMap::new(), array_loans: Box::new([]), field_loans: Box::new([]),
                });
            assert!(project_pending_instance_source_arguments_v1(source, &pending,
                &package.parameter_contracts, target.call_site()).unwrap_err()
                .contains("source-phase-drift"), "wrong Instance phase cannot recurse into Object");
        }
        let mut missing_loan = local.clone();
        missing_loan.owning_object_input = None;
        assert!(ingress.original_owning_object_input_source_v1(&missing_loan).is_err());
        let loan = ingress.original_owning_object_input_source_v1(local).unwrap().unwrap();
        ledger.callable_result_classes.recheck_owning_object_local_input_roots_v1(loan).unwrap();
        let local_site = local.call_site().clone();
        let sibling = sibling.clone();
        let ledger = std::rc::Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
        ledger.borrowed_formal_source.as_mut().unwrap().as_mut().unwrap()
            .source_incoming.remove_exact_row_for_test(&local_site);
        assert!(ledger.borrowed_formal_source.as_ref().unwrap().as_ref().unwrap()
            .original_owning_object_input_callee_v1(&sibling).is_err(),
            "loss of sole LOCAL demand must not silently deselect its sibling");
    }
}

#[test]
fn returned_field_input_keeps_qualified_object_phase_and_refuses_instance_drift() {
    let text = "box Token {} box Wrapper { child: Token birth(child) { me.child = child } }
        box Maker { make() { return new Token() }
            seed() { local h = me.make() return new Wrapper(h) }
            consume(h) { if h == null { return new Token() } return new Token() }
            qualified(h) { if h == null { return new Token() } return me.consume(h) }
            probe() { local a = me.seed() local b = me.consume(a.child) return 0 } }
        static box Main { main() { return 0 } }";
    let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(text).unwrap();
    let ledger = &package.ordinary_new_claim_ledger;
    let source = ledger.borrowed_formal_source.as_ref().unwrap();
    let ingress = source.as_ref().unwrap();
    let call = ingress.source_incoming.exact_rows().find(|row|
        ingress.returned_field_argument_selected_v1(&row.call)).expect("original Field argument");
    let target = call.source.require_instance().unwrap();
    assert!(ingress.source_incoming.has_object_input_callee_v1(target));
    assert!(super::super::object_source::selected_returned_field_input_v1(ingress, &call.call).unwrap());
    assert!(super::super::object_source::selected_original_object_phase_v1(ingress, &call.call).unwrap());
    let pending = &ledger.borrowed_formal_actuals;
    let rows = pending.get(&call.call).unwrap().as_ref().unwrap();
    assert!(matches!(rows.phase, BorrowedCallActualEvidencePhaseV1::SourceObject(_)
        | BorrowedCallActualEvidencePhaseV1::ExecutableSourceObject(_)));
    let before = pending.clone();
    let actual = project_pending_instance_source_arguments_v1(source, pending,
        &package.parameter_contracts, &call.call).unwrap().expect("Object delegation");
    assert_eq!(actual, rows.ordered_arguments);
    assert_eq!(&before, pending, "source projection must not issue an executable phase");
    let mut forged = pending.clone();
    forged.get_mut(&call.call).unwrap().as_mut().unwrap().phase =
        BorrowedCallActualEvidencePhaseV1::SourceInstance(InstanceSourceActualIdentityV1 {
            source: target.clone(), incoming_arguments: call.arguments.clone(), candidates: Box::new([]), argument_sources: BTreeMap::new(),
            array_loans: Box::new([]), field_loans: Box::new([]),
        });
    assert!(project_pending_instance_source_arguments_v1(source, &forged,
        &package.parameter_contracts, &call.call).unwrap_err().contains("source-phase-drift"));
    let mut missing = pending.clone();
    missing.remove(&call.call);
    assert!(project_pending_instance_source_arguments_v1(source, &missing,
        &package.parameter_contracts, &call.call).is_err());
    missing.insert(call.call.clone(), Err("retained-field-input-poison".into()));
    assert_eq!(project_pending_instance_source_arguments_v1(source, &missing,
        &package.parameter_contracts, &call.call).unwrap_err(), "retained-field-input-poison");
}
