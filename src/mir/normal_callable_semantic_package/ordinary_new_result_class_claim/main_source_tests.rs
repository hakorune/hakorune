//! Original AppMain joins the existing source result solver, without child selection.
use super::super::super::coseal_issue::source_claims::prepare_source_claims;
use super::*;
use std::rc::Rc;

fn source(formal: &str, actual: &str, received: bool, nullable: bool) -> String {
    let call = format!("maker.make({actual})");
    let body = if received {
        format!("local item = {call} return item")
    } else {
        format!("return {call}")
    };
    let null_exit = if nullable {
        "if true { return null }"
    } else {
        ""
    };
    format!("box Token {{}} box Maker {{ make({formal}) {{ {null_exit} return new Token() }} }} static box Main {{ main() {{ local maker = new Maker() {body} }} }}")
}

#[test]
fn main_direct_and_received_share_original_source_outcomes_and_callee_witnesses() {
    for received in [false, true] {
        for nullable in [false, true] {
            for (formal, actual) in [("size: i64", "7"), ("size", "7"), ("", "")] {
                let text = source(formal, actual, received, nullable);
                with_source_claims_fixture(
                    &text,
                    |batch, selected, constructors, parameters, main, writers| {
                        let main = main.expect("original Main co-seal");
                        assert!(selected
                            .batch_slot(&SelectedNormalCallableKeyV1::Cataloged(
                                main.catalog_key().clone()
                            ))
                            .is_none());
                        let (_, _, facts, _) = prepare_source_claims(
                            batch,
                            selected,
                            Some(main),
                            constructors,
                            parameters,
                            std::rc::Rc::clone(writers), None,
                        )
                        .unwrap();
                        let root = facts
                            .outcomes(main.catalog_key())
                            .expect("Root source result");
                        assert_eq!(root.len(), 1);
                        let exit = &root[0];
                        let owner = batch
                            .declarations()
                            .find(|row| row.batch_slot() == main.batch_slot())
                            .unwrap()
                            .owner();
                        assert_eq!(exit.site().owner(), owner);
                        let loan = facts
                            .object_return_qualification(exit.site())
                            .expect("same result qualification");
                        let key = CanonicalSameModuleCallableKeyV1::instance_box_method(
                            "Maker",
                            "make",
                            if formal.is_empty() { 0 } else { 1 },
                        );
                        assert_eq!(loan.key(), &key);
                        assert_eq!(loan.value(), exit.site());
                        assert_eq!(loan.call().owner(), owner);
                        assert_eq!(loan.call() == loan.value(), !received);
                        let expected = if nullable {
                            OrdinaryNewResultClassV1::NullableObject("Token".into())
                        } else {
                            OrdinaryNewResultClassV1::Object("Token".into())
                        };
                        assert_eq!(loan.class(), &expected);
                        batch
                            .with_lowering_input(main.batch_slot(), |input| {
                                assert!(input.function().method_call(loan.call().site()).is_some());
                            })
                            .unwrap();
                        let callee = facts.outcomes(&key).unwrap();
                        for (witness, retained) in loan.witnesses().iter().zip(exit.witnesses()) {
                            assert!(Rc::ptr_eq(witness, retained));
                            let ResultWitnessStepV1::Call {
                                site,
                                key: target,
                                callee: original,
                                ..
                            } = witness.step()
                            else {
                                panic!("original Call witness")
                            };
                            assert_eq!(site, loan.call());
                            assert_eq!(target, &key);
                            assert!(callee
                                .iter()
                                .flat_map(|exit| exit.witnesses())
                                .any(|witness| Rc::ptr_eq(witness, original)));
                        }
                    },
                );
            }
        }
    }
    // Storage results must retain their own source origin, never the old
    // owned-return projection. These boundaries were not exercised by Fresh.
    for (extra, birth_tail, between, accepted) in [
        ("", "", "", true),
        ("escape() { return me.stats }", "", "", false),
        ("change(key) { me.stats.set(key, new Other()) }", "", "", false),
        ("clear() { me.stats.clear() }", "", "", false),
        ("", "me.stats.set(\"seed\", new Other())", "", false),
        ("", "me.stats.clear()", "", false),
        ("change(key) { me.stats.set(key, new Item()) }", "", "me.change(key)", false),
    ] {
        let text = format!("box Item {{ read() {{ return 1 }} }} box Other {{}} box Holder {{ init {{ stats }} birth() {{ me.stats = new MapBox() {birth_tail} }} find(key) {{ if me.stats.has(key) {{ return me.stats.get(key) }} local item = new Item() me.stats.set(key, item) {between} return item }} probe(key) {{ local item = me.stats.get(key) if item == null {{ return 0 }} local value = item.read() return value }} {extra} }} static box Main {{ main() {{ return 0 }} }}");
        with_source_claims_fixture(&text, |batch, selected, constructors, parameters, main, writers| {
            let common = crate::mir::callable_parameter_contract::issue_callable_parameter_contract_v1(batch).unwrap();
            let entries = crate::mir::resolved_semantics::CallableHomeAbiIssuerV1::issue_source_entry_home_catalog_v1(batch, &common).unwrap();
            let (fields, _, facts, _) = prepare_source_claims(batch, selected, main, constructors, parameters,
                std::rc::Rc::clone(writers), Some(&entries)).unwrap();
            let key = CanonicalSameModuleCallableKeyV1::instance_box_method("Holder", "find", 1);
            let slot = selected.batch_slot(&SelectedNormalCallableKeyV1::Cataloged(key.clone())).unwrap();
            batch.with_lowering_input(slot, |input| {
                let get = input.function().method_calls().find(|(_, call)| call.selector() == "get").unwrap().0;
                let at = OwnedExprSiteV1::new(input.owner(), get.clone());
                let entry = entries.for_batch_slot(slot).unwrap();
                let receiver = crate::mir::normal_callable_semantic_package::ordinary_new_coseal::entry_receiver_box_proof(selected, batch, Some(entry), slot).unwrap();
                let source = crate::mir::normal_callable_semantic_package::ordinary_new_coseal::StoredMapGetSourceLoanV1::issue_for_site_v1(
                    input, &at, batch, selected, constructors, Some(receiver), Some(entry), &facts, &fields).unwrap();
                // A same-class replacement invalidates Return borrowing, not
                // the storage class. Escape/mixed writers/unknown mutations reject both.
                assert_eq!(source.is_some(), accepted || !between.is_empty());
                if let Some(source) = source {
                    assert_eq!(source.payload_source_v1().payload_class_v1(), "Item");
                    assert_eq!(source.result_source_v1().is_some(), accepted);
                }
            }).unwrap();
            let probe = CanonicalSameModuleCallableKeyV1::instance_box_method("Holder", "probe", 1);
            let probe_slot = selected.batch_slot(&SelectedNormalCallableKeyV1::Cataloged(probe)).unwrap();
            let names: Vec<Box<str>> = batch.ordinary_box_coverage().rows().iter().map(|row| row.name().into()).collect();
            let needs = super::super::super::lexical_instance_call::prepare_lexical_source_targets_v1(
                batch, selected, &BTreeMap::new(), &names, &fields, &facts, parameters,
                main.map(|main| main.caller_source_ref_v1()), &mut |_, _, _| Ok(None), Some(constructors), Some(&entries)).unwrap();
            let target = needs.iter().filter_map(|row| row.as_ref().unwrap().as_ref()).find_map(|need| match need {
                super::super::super::lexical_instance_call::PreparedSourceCallNeedV1::Lexical(source)
                    if source.target().owner() == "Item" && source.target().name() == "read" => Some(source), _ => None,
            });
            assert_eq!(target.is_some(), accepted || !between.is_empty());
            if let Some(target) = target {
                assert!(target.has_borrowed_receiver_source_v1());
                assert!(target.borrowed_call_receiver_source_v1().is_none());
                let direct = target.direct_map_receiver_source_v1().unwrap();
                batch.with_lowering_input(probe_slot, |input| assert!(direct.matches_original_v1(input))).unwrap();
            }
            assert_eq!(facts.outcomes(&key).is_some(), accepted, "Map source boundary: {extra}");
            assert!(facts.get(&key).is_none());
            if accepted {
                let exits = facts.outcomes(&key).unwrap();
                assert_eq!(exits.len(), 2);
                for exit in exits {
                    assert!(exit.alternatives().iter().all(|origin| matches!(origin, ResultValueOriginV1::MapHeld { class, .. } if class.as_ref() == "Item")));
                    let loan = facts.map_stored_source_v1(exit.site()).unwrap().expect("same original storage Return");
                    assert!(exit.witnesses().iter().all(|root| matches!(root.step(),
                        ResultWitnessStepV1::MapStored { source } if Rc::ptr_eq(source, &loan))));
                    assert!(facts.object_return_qualification(exit.site()).is_none());
                }
            }
        });
    }
    let json = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/apps/json-stream-aggregator/main.hako"));
    with_source_claims_fixture(json, |batch, selected, constructors, parameters, main, writers| {
        let common = crate::mir::callable_parameter_contract::issue_callable_parameter_contract_v1(batch).unwrap();
        let entries = crate::mir::resolved_semantics::CallableHomeAbiIssuerV1::issue_source_entry_home_catalog_v1(batch, &common).unwrap();
        let (fields, _, facts, _) = prepare_source_claims(batch, selected, main, constructors, parameters,
            std::rc::Rc::clone(writers), Some(&entries)).unwrap();
        let key = CanonicalSameModuleCallableKeyV1::instance_box_method("JsonStreamAggregator", "statsFor", 1);
        assert!(facts.get(&key).is_none());
        let exits = facts.outcomes(&key).expect("unmodified JSON Map-held result source");
        assert_eq!(exits.len(), 2);
        assert!(exits.iter().flat_map(|exit| exit.alternatives()).all(|origin|
            matches!(origin, ResultValueOriginV1::MapHeld { class, .. } if class.as_ref() == "UserStats")));
        let caller = CanonicalSameModuleCallableKeyV1::instance_box_method("JsonStreamAggregator", "ingestLine", 1);
        let slot = selected.batch_slot(&SelectedNormalCallableKeyV1::Cataloged(caller)).unwrap();
        let (call, destination, key_binding) = batch.with_lowering_input(slot, |input| {
            let init = input.function().expression_source().initializers().find(|init|
                init.initializer_site().and_then(|site| input.function().method_call(site))
                    .is_some_and(|call| call.selector() == "statsFor")).unwrap();
            let site = init.initializer_site().unwrap();
            let actual = input.function().method_call(site).unwrap().arguments()[0].site();
            let crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(binding) = input.function().variable_ref(actual).unwrap()
                else { panic!("original local key actual"); };
            (OwnedExprSiteV1::new(input.owner(), site.clone()), init.binding(), binding)
        }).unwrap();
        let loan = facts.borrowed_map_local_receiver_source_v1(batch, selected, &fields, parameters,
            &call, destination, None).unwrap().expect("original Map LOCAL result");
        assert!(loan.is_map_slot());
        assert_eq!(loan.callee_ordinal(), None);
        assert_eq!(loan.root(), None);
        assert_ne!(loan.support_root(), key_binding);
        assert_eq!(loan.witnesses().len(), 2);
        assert!(loan.witnesses().iter().all(|root| matches!(root.origin(),
            ResultValueOriginV1::MapHeld { key, class, .. } if *key == key_binding && class.as_ref() == "UserStats")));
        facts.recheck_borrowed_nullable_local_source_v1(batch, selected, &fields, parameters, &loan, None).unwrap();
    });
    // The unchanged app's source supports both stored Return roles without
    // inventing a formal-forward result or granting its pending Call an ABI.
    let linked_source = format!("{}\n{json}", include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/lang/src/shared/common/string_helpers.hako")));
    let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog_selection(
        &linked_source, crate::mir::normal_callable_semantic_package::PackagePhysicalSelectionV1::ClosedAppExecutable).unwrap();
    // The original Text-bearing Birth must retain its store uses even though
    // owned initialization is not executable yet. It must not acquire I64 ABI.
    let birth = package.instance_constructors().rows().iter()
        .find(|row| row.box_name() == "UserStats").unwrap();
    let [formal] = birth.formal_contracts() else { panic!("original Birth formal"); };
    assert_eq!(formal.declaration(), crate::mir::normal_callable_semantic_package::BirthFormalDeclarationClassV1::Unannotated);
    assert!(matches!(formal.uses(), crate::mir::normal_callable_semantic_package::BirthFormalUseCoverageV1::UnqualifiedFieldStores { sites }
        if sites.len() == 1));
    assert_eq!(formal.disposition(), crate::mir::normal_callable_semantic_package::BirthFormalPhysicalDispositionV1::UnavailableTaggedOrCheckedRepresentation);
    assert!(birth.construction_source().is_ok());
    let plan = birth.construction().as_ref().expect("original Text-only Birth execution projection");
    let original = birth.construction_source().as_ref().unwrap();
    assert!(Rc::ptr_eq(plan.source_v1(), original));
    assert_eq!(original.stores()[0].mode(), Err(crate::mir::normal_callable_semantic_package::ConstructionUnavailableV1::FieldContractUnsupported));
    let text = plan.store_execution_v1(0).unwrap();
    assert_eq!(text.mode(), crate::mir::instruction::FieldSetModeV1::DynamicTextInitialize);
    assert!(text.text_input_v1().unwrap().text_only());
    assert!(text.fault_discharge().is_empty());
    let next = plan.store_execution_v1(1).unwrap();
    assert_eq!(next.fault_discharge().len(), 1);
    assert_eq!(next.fault_discharge()[0].field, original.stores()[0].field());
    assert!(original.stores()[1].fault_discharge().is_empty());
    let ledger = &package.ordinary_new_claim_ledger;
    let key = CanonicalSameModuleCallableKeyV1::instance_box_method("JsonStreamAggregator", "statsFor", 1);
    let owner = package.parameter_contracts.iter().find(|row|
        package.selected.key_for_batch_slot(row.batch_slot)
            == Some(&SelectedNormalCallableKeyV1::Cataloged(key.clone()))).unwrap().owner;
    assert!(ledger.callable_result_source_v1(owner).unwrap().unwrap().borrowed_nullable().is_some());
    // Original New actual and the same borrowed evaluation survive co-seal.
    let claims = ledger.pending_claims_for_test();
    let claim = claims.values().find(|claim| claim.site().owner() == owner && claim.core.class.as_ref() == "UserStats").unwrap();
    assert_eq!(claim.construction().as_ref().unwrap(), plan);
    let sum = ledger.constructor_text_store_sum_v1(birth, &original.stores()[0]).unwrap();
    assert!(Rc::ptr_eq(text.text_input_v1().unwrap(), &sum));
    let [argument] = claim.argument_rows().unwrap() else { panic!("original New actual"); };
    let crate::mir::normal_callable_semantic_package::OrdinaryNewTrivialArgumentKindV1::Expression { evaluation } = argument.kind()
        else { panic!("Text must not become a raw scalar/object argument: {:?}", argument.kind()); };
    assert!(matches!(evaluation.source().root(), crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::NewArgument { construction, ordinal: 0, .. }
        if construction.site() == claim.site().site() && construction.arguments().first() == Some(argument.site())));
    assert!(evaluation.reference_coverage_is_exact());
    assert!(evaluation.references().values().any(|reference| matches!(reference,
        crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionReferenceV1::FiniteEntry { .. })));
    assert!(evaluation.owned_result_nodes().is_empty());
    let observation = crate::mir::resolved_semantics::home_new_prefix::SelectedNewArgumentObservationV1::new(claim.site().clone(), Ok(vec![
        crate::mir::resolved_semantics::home_new_prefix::SelectedNewArgumentV1::new(0, argument.site().clone(),
            crate::mir::resolved_semantics::home_new_prefix::SelectedNewArgumentKindV1::Expression { evaluation: Rc::clone(evaluation) })].into_boxed_slice()));
    let converted = crate::mir::normal_callable_semantic_package::ordinary_new_coseal::coseal_helpers::convert_selected_new_arguments(observation).unwrap();
    let crate::mir::normal_callable_semantic_package::OrdinaryNewTrivialArgumentKindV1::Expression { evaluation: retained } = converted[0].kind() else { unreachable!() };
    assert!(Rc::ptr_eq(evaluation, retained));

}

#[test]
fn main_observer_missing_or_foreign_loan_cannot_issue_root_facts() {
    let text = source("size: i64", "7", false, false);
    with_source_claims_fixture(&text, |batch, selected, constructors, parameters, main, writers| {
        let main = main.unwrap();
        let (_, _, facts, _) =
            prepare_source_claims(batch, selected, None, constructors, parameters, std::rc::Rc::clone(writers), None,).unwrap();
        assert!(facts.outcomes(main.catalog_key()).is_none());
        assert!(
            facts.contains_key(&CanonicalSameModuleCallableKeyV1::instance_box_method(
                "Maker", "make", 1
            ))
        );
        with_source_claims_fixture(&text, |_, _, _, _, foreign_main, _| {
            let foreign_main = foreign_main.unwrap();
            assert_eq!(foreign_main.catalog_key(), main.catalog_key());
            assert_eq!(foreign_main.batch_slot(), main.batch_slot());
            assert!(matches!(
                prepare_source_claims(
                    batch,
                    selected,
                    Some(foreign_main),
                    constructors,
                    parameters,
                    std::rc::Rc::clone(writers), None,
                ),
                Err(super::super::super::OrdinaryNewCoSealIssueV1::AppMainIdentityMissing)
            ));
        });
    });
}

#[test]
fn main_finish_independently_rejects_missing_foreign_and_drifted_source() {
    let text = source("size", "7", true, false);
    with_source_claims_fixture(&text, |batch, selected, constructors, parameters, main, writers| {
        let main = main.unwrap();
        let (fields, _, _, _) =
            prepare_source_claims(batch, selected, Some(main), constructors, parameters, std::rc::Rc::clone(writers), None,).unwrap();
        let draft = || {
            let mut draft = OrdinaryNewResultClassClaimDraftV1::new();
            for row in batch.declarations() {
                let key = if row.batch_slot() == main.batch_slot() {
                    Some(main.catalog_key())
                } else {
                    match selected.key_for_batch_slot(row.batch_slot()) {
                        Some(SelectedNormalCallableKeyV1::Cataloged(key)) => Some(key),
                        _ => None,
                    }
                };
                if let Some(key) = key {
                    batch
                        .with_lowering_input(row.batch_slot(), |input| {
                            draft.observe_function(input, key, row.batch_slot())
                        })
                        .unwrap();
                }
            }
            assert!(draft
                .rows
                .iter()
                .any(|row| row.key == *main.catalog_key() && row.batch_slot == main.batch_slot()));
            draft
        };
        let callee = CanonicalSameModuleCallableKeyV1::instance_box_method("Maker", "make", 1);
        let callee_slot = selected
            .batch_slot(&SelectedNormalCallableKeyV1::Cataloged(callee.clone()))
            .unwrap();
        assert!(draft()
            .finish(
                batch.ordinary_box_coverage(),
                batch,
                selected,
                Some(main),
                &fields,
                parameters, constructors, None,
            )
            .outcomes(main.catalog_key())
            .is_some());
        let missing = draft().finish(
            batch.ordinary_box_coverage(),
            batch,
            selected,
            None,
            &fields,
            parameters, constructors, None,
        );
        assert!(missing.outcomes(main.catalog_key()).is_none());
        assert!(missing.contains_key(&callee));
        with_source_claims_fixture(&text, |foreign_batch, _, _, _, foreign_main, _| {
            let foreign_main = foreign_main.unwrap();
            assert_eq!(foreign_main.catalog_key(), main.catalog_key());
            let foreign_owner = foreign_batch
                .declarations()
                .find(|row| row.batch_slot() == foreign_main.batch_slot())
                .unwrap()
                .owner();
            let foreign = draft().finish(
                batch.ordinary_box_coverage(),
                batch,
                selected,
                Some(foreign_main),
                &fields,
                parameters, constructors, None,
            );
            assert!(foreign.outcomes(main.catalog_key()).is_none());
            assert!(foreign.contains_key(&callee));
            for corruption in 0..3 {
                let mut draft = draft();
                let row = draft
                    .rows
                    .iter_mut()
                    .find(|row| row.key == *main.catalog_key())
                    .unwrap();
                let altered_key =
                    CanonicalSameModuleCallableKeyV1::static_box_method("Foreign", "main", 0);
                match corruption {
                    0 => {
                        let site = &mut row.exits[0].0;
                        *site = crate::mir::resolved_semantics::OwnedExprSiteV1::new(
                            foreign_owner,
                            site.site().clone(),
                        );
                    }
                    1 => row.batch_slot = callee_slot,
                    2 => row.key = altered_key.clone(),
                    _ => unreachable!(),
                }
                let facts = draft.finish(
                    batch.ordinary_box_coverage(),
                    batch,
                    selected,
                    Some(main),
                    &fields,
                    parameters, constructors, None,
                );
                assert!(facts.outcomes(main.catalog_key()).is_none());
                assert!(facts.outcomes(&altered_key).is_none());
                assert!(facts.contains_key(&callee));
            }
        });
    });
}
