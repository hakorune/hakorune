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
        let text = format!("box Item {{}} box Other {{}} box Holder {{ init {{ stats }} birth() {{ me.stats = new MapBox() {birth_tail} }} find(key) {{ if me.stats.has(key) {{ return me.stats.get(key) }} local item = new Item() me.stats.set(key, item) {between} return item }} {extra} }} static box Main {{ main() {{ return 0 }} }}");
        with_source_claims_fixture(&text, |batch, selected, constructors, parameters, main, writers| {
            let common = crate::mir::callable_parameter_contract::issue_callable_parameter_contract_v1(batch).unwrap();
            let entries = crate::mir::resolved_semantics::CallableHomeAbiIssuerV1::issue_source_entry_home_catalog_v1(batch, &common).unwrap();
            let (_, _, facts, _) = prepare_source_claims(batch, selected, main, constructors, parameters,
                std::rc::Rc::clone(writers), Some(&entries)).unwrap();
            let key = CanonicalSameModuleCallableKeyV1::instance_box_method("Holder", "find", 1);
            assert_eq!(facts.outcomes(&key).is_some(), accepted, "Map source boundary: {extra}");
            assert!(facts.get(&key).is_none());
            if accepted {
                let exits = facts.outcomes(&key).unwrap();
                assert_eq!(exits.len(), 2);
                for exit in exits {
                    assert!(exit.alternatives().iter().all(|origin| matches!(origin, ResultValueOriginV1::MapHeld { class, .. } if class.as_ref() == "Item")));
                    assert!(exit.witnesses().iter().all(|root| matches!(root.step(), ResultWitnessStepV1::MapStored { .. })));
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
        let loan = facts.borrowed_nullable_local_source_v1(batch, selected, &fields, parameters,
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
