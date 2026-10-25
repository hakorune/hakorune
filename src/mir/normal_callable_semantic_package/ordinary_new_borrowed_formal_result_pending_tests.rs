//! Pending source evidence cannot regain permission through a result annotation.
use super::*;
use crate::mir::resolved_semantics::home_new_prefix::TerminalRelationV1;

#[test]
fn pending_stored_result_rejects_projection_and_completion_permission() {
    for annotated in [false, true] {
        let result = if annotated { ": i64" } else { "" };
        let text = format!(
            "box Item {{ value: i64 birth() {{ me.value = 5 }} }}
            box Leaf {{ flag: i64 birth() {{ me.flag = 0 }}
                read(p){result} {{ if p == null {{ return 7 }} return p.value }} }}
            box Parent {{ child: Leaf birth() {{ me.child = new Leaf() }}
                run(p: Item){result} {{ return me.child.read(p) }} }}
            static box Main {{ main() {{ local parent = new Parent() local item = new Item()
                return parent.run(item) }} }}"
        );
        let mut package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(&text)
            .expect("healthy opaque callee with declared forwarding caller");
        let ledger = std::rc::Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
        let source = ledger
            .borrowed_formal_source
            .as_ref()
            .unwrap()
            .as_ref()
            .unwrap();
        let target = source
            .incoming
            .iter()
            .find(|row| row.source.stored_receiver().is_some())
            .expect("source-issued stored receiver")
            .source
            .clone();
        let terminal = ledger
            .call_relations_for_owner(target.call_site().owner())
            .into_iter()
            .find(|terminal| terminal.call_site() == target.call_site().site())
            .expect("original stored Return keeps its scalar source loan");
        let retained = terminal.source_scalar().expect("same result source lender");
        assert_eq!(
            terminal.scalar_kind(),
            crate::mir::resolved_semantics::home_new_prefix::SourceScalarKind::Integer
        );
        assert_eq!(retained.source(), target.require_instance().unwrap());
        let current = super::super::scalar_loan::issue_source_scalar_result_loan_v1(
            ledger.borrowed_formal_source.as_ref().unwrap(),
            &ledger.borrowed_i64_results,
            retained.source(),
        )
        .unwrap();
        assert_eq!(retained, &current, "same target and original facts Rc");
        let prepared: Vec<_> = source
            .incoming
            .iter()
            .map(|row| Ok(Some(row.source.require_instance().unwrap().clone())))
            .collect();
        let proof = ledger
            .borrowed_i64_results
            .get_mut(&target.callee_owner())
            .unwrap()
            .as_mut()
            .unwrap();
        assert!(proof.contract_corroborated);
        proof.phase = BorrowedResultSourcePhaseV1::Pending {
            fields: Box::new([]),
            calls: Box::new([]),
        };
        let issue = super::super::super::borrowed_formal_actuals::project_pending_borrowed_i64_arguments_v1(
            ledger.borrowed_formal_source.as_ref().unwrap(), &ledger.borrowed_formal_actuals,
            &ledger.borrowed_i64_results, target.call_site()).unwrap_err();
        assert!(
            issue.contains("borrowed-result/source-not-sealed"),
            "{issue}"
        );
        ledger.corroborate_borrowed_result_cohort_v1(&prepared, &package.result_contracts);
        let issue = ledger
            .borrowed_i64_results
            .get(&target.callee_owner())
            .unwrap()
            .as_ref()
            .unwrap_err();
        assert!(
            issue.contains("borrowed-result/source-not-sealed"),
            "{issue}"
        );
    }
}

#[test]
fn grounded_weak_stored_receiver_retains_issuance_failure() {
    let text = "box Item { value: i64 birth() { me.value = 5 } }
        box Leaf { flag: i64 birth() { me.flag = 0 }
            read(p) { if p == null { return 7 } return p.value } }
        box Parent { weak child: Leaf birth() { me.child = new Leaf() }
            run(p: Item) { return me.child.read(p) } }
        static box Main { main() { local parent = new Parent() local item = new Item()
            return parent.run(item) } }";
    let issue = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(text)
        .expect_err("a weak provider is not an owned stored receiver");
    let issue = format!("{issue:?}");
    assert!(
        issue.contains("stored-child/receiver-unavailable"),
        "{issue}"
    );
    assert!(issue.contains("BorrowedFormalIngress"), "{issue}");
}

#[test]
fn conflicting_conditional_classes_do_not_depend_on_wrapper_order() {
    let first = "first(p: A) { return me.child.read(p) }";
    let second = "second(p: B) { return me.child.read(p) }";
    for methods in [format!("{first} {second}"), format!("{second} {first}")] {
        let text = format!(
            "box A {{ value: i64 birth() {{ me.value = 5 }} }}
            box B {{ other: i64 birth() {{ me.other = 6 }} }}
            box Leaf {{ flag: i64 birth() {{ me.flag = 0 }}
                read(p) {{ if p == null {{ return 7 }} return p.value }} }}
            box Parent {{ child: Leaf birth() {{ me.child = new Leaf() }} {methods} }}
            static box Main {{ main() {{ local parent = new Parent() local a = new A()
                local b = new B() local ignored = parent.second(b) return parent.first(a) }} }}"
        );
        let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(&text)
            .expect("unselected conditional conflict retains the existing outside-profile boundary");
        let ledger = &package.ordinary_new_claim_ledger;
        let source = ledger
            .borrowed_formal_source
            .as_ref()
            .unwrap()
            .as_ref()
            .unwrap();
        assert!(
            source.definitions.is_empty(),
            "no conditional candidate enters strict ingress"
        );
        assert!(source.incoming.is_empty());
        assert_eq!(
            source.object_views.len(),
            2,
            "only original declared constraints remain"
        );
        for (formal, view) in &source.object_views {
            assert!(view.is_declared());
            assert!(matches!(view.class(), "A" | "B"));
            assert_eq!(source.source_object_view_v1(*formal).unwrap(), Some(view));
            assert!(
                source.formal_object_view(*formal).is_none(),
                "source class grants no field permission"
            );
            assert!(!source.formal_field_read_target(formal.owner()));
        }
        assert!(
            ledger.borrowed_i64_results.iter().all(|(_, result)| result.is_err()),
            "refused source rows must not mint any result permission"
        );
        let calls = ledger.lexical_instance_calls.borrow();
        let mut lexical = 0;
        for row in calls.values() {
            if let crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::LexicalInstanceCallDispositionSlotV1::Ready(row) = row {
                assert!(row.source_target().stored_receiver().is_none(), "no stored call is promoted");
                lexical += 1;
            }
        }
        assert_eq!(
            lexical, 2,
            "both existing main lexical calls remain selected"
        );
    }
}

#[test]
fn selected_stored_result_retains_foreign_actual_ingress_failure() {
    let text = "box Item { value: i64 birth() { me.value = 5 } }
        box Other { value: i64 birth() { me.value = 6 } }
        box Leaf { flag: i64 birth() { me.flag = 0 }
            read(p) { if p == null { return 7 } return p.value } }
        box Parent { child: Leaf birth() { me.child = new Leaf() }
            run(p: Item) { return me.child.read(p) } }
        static box Main { main() { local parent = new Parent() local item = new Other()
            return parent.run(item) } }";
    let issue = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(text)
        .expect_err("foreign class cannot satisfy the declared incoming view");
    let issue = format!("{issue:?}");
    assert!(
        issue.contains("borrowed-view/declared-object-class"),
        "{issue}"
    );
    assert!(
        !issue.contains("stored-child/result-source-missing"),
        "{issue}"
    );
}


#[test]
fn field_array_result_retains_one_read_and_all_original_returns() {
    for (body, expected_returns) in [
        ("return me.values.get(0)", 1),
        ("return me.values.length()", 1),
        ("local v = me.values.length() if handle <= 0 { return v } return v", 2),
        ("local v = me.values.get(0) if handle <= 0 { return v } return v", 2),
        ("local v = me.values.get(0) local h = new Handle(v) local peer = new Handle(v) return v", 1),
    ] {
        let text = format!("box Handle {{ n: i64 birth(n) {{ me.n = n }} }} box Page {{ values: ArrayBox = new ArrayBox() birth() {{ }} probe(handle): i64 {{ if handle <= 0 {{ return 0 }} me.values.set(0, 7) {body} }} relay(handle): i64 {{ if handle <= 0 {{ return 0 }} return me.probe(handle) }} }} static box Main {{ main() {{ local scalar = 7 local h = new Handle(scalar) return 0 }} }}");
        let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(&text).unwrap();
        let ledger = &package.ordinary_new_claim_ledger;
        let contract = package.parameter_contracts.iter().find(|row|
            package.selected.key_for_batch_slot(row.batch_slot).is_some_and(|key|
                matches!(key, crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(key) if key.owner() == "Page" && key.name() == "probe"))).unwrap();
        let proof = ledger.borrowed_i64_results[&contract.owner].as_ref().unwrap();
        proof.require_source_i64_v1().unwrap();
        assert_eq!(proof.field_array_reads.len(), 1, "one exact call irrespective of Return count");
        let read = &proof.field_array_reads[0];
        assert_eq!(read.return_sites().len(), expected_returns);
        for returned in read.return_sites() {
            assert!(ledger.terminal_relations_for_owner(contract.owner).iter().any(|relation|
                matches!(relation, TerminalRelationV1::I64Scalar(row) if row.value_site() == returned.site())),
                "required scalar walk must retain each original read Return relation");
        }
        assert_eq!(read.call().owner(), contract.owner);
        assert_eq!(read.receiver().owner(), contract.owner);
        if let Some(index) = read.index() { assert_eq!(index.owner(), contract.owner); }
        assert_eq!(read.is_count(), body.contains("length()"));
        assert_eq!(read.provider().owner(), "Page");
        if body.contains("new Handle") {
            let claims = ledger.claims.borrow();
            let arguments: Vec<_> = claims.values().flat_map(|claim| claim.core.argument_rows.as_ref().into_iter().flatten())
                .filter_map(|argument| argument.field_array_read().map(|source| (argument, source))).collect();
            assert_eq!(arguments.len(), 2, "both New uses retain the original Array read");
            assert!(Rc::ptr_eq(arguments[0].1, arguments[1].1), "one original local wrapper across New uses");
            let ingress = ledger.borrowed_formal_source.as_ref().unwrap().as_ref().unwrap();
            let passive = BorrowedResultCohortV1::prepared(BTreeMap::new(),
                Rc::clone(ledger.borrowed_i64_results.field_candidates.as_ref().unwrap()));
            let unavailable = BorrowedResultCohortV1::unavailable("test/original-new-unavailable".into());
            let fresh = |argument: &crate::mir::normal_callable_semantic_package::ordinary_new_coseal::OrdinaryNewTrivialArgumentV1|
                crate::mir::normal_callable_semantic_package::ordinary_new_coseal::OrdinaryNewTrivialArgumentV1::new(
                    argument.owner(), argument.new_site().clone(), argument.ordinal(), argument.site().clone(), argument.kind().clone());
            let scalar = claims.values().flat_map(|claim| claim.core.argument_rows.as_ref().into_iter().flatten())
                .find(|argument| argument.owner() != contract.owner && matches!(argument.kind(),
                    crate::mir::normal_callable_semantic_package::ordinary_new_coseal::OrdinaryNewTrivialArgumentKindV1::Local { .. })).unwrap();
            super::super::super::super::field_array_read_source::retain_argument_read(package.batch(), &package.selected,
                &package.instance_constructors, &unavailable, &ledger.array_i64_fields, &mut fresh(scalar)).unwrap();
            for (argument, source) in arguments {
                let mut retained = fresh(argument);
                super::super::super::super::field_array_read_source::retain_argument_read(package.batch(), &package.selected,
                    &package.instance_constructors, &passive, &ledger.array_i64_fields, &mut retained).unwrap();
                assert!(Rc::ptr_eq(retained.field_array_read().unwrap(), source), "source loan survives absent transport/result seal");
                assert!(super::super::super::super::field_array_read_source::retain_argument_read(package.batch(), &package.selected,
                    &package.instance_constructors, &unavailable, &ledger.array_i64_fields, &mut fresh(argument))
                    .unwrap_err().contains("test/original-new-unavailable"), "Array cannot recapture after lost source");
                assert!(Rc::ptr_eq(source.read(), read.read()), "Return and New use SAME read source");
                assert!(Rc::ptr_eq(source, ingress.original_new_argument_array_read_v1(argument.new_site(),
                    argument.ordinal(), argument.site(), source.binding()).unwrap().unwrap()));
                assert!(ingress.original_new_argument_array_read_v1(argument.new_site(), argument.ordinal() + 1,
                    argument.site(), source.binding()).is_err(), "foreign ordinal cannot reuse read");
                assert!(source.read().recheck_census(contract.owner, &BTreeMap::new()).is_err());
            }
        }
        read.recheck_census(contract.owner, &proof.returns, &ledger.array_i64_fields).unwrap();
        assert_eq!(read.recheck_census(contract.owner, &proof.returns, &BTreeMap::new()).is_ok(), read.is_count());
        assert!(read.recheck_census(contract.owner, &[], &ledger.array_i64_fields).is_err());
    }
}

#[test]
fn parameterless_result_source_requires_original_parameter_census() {
    let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(
        "box Probe { zero() { return true } one(p) { return true } } static box Main { main() { return 0 } }",
    ).unwrap();
    for arity in [0, 1] {
        let contract = package.parameter_contracts.iter().find(|row|
            matches!(package.selected.key_for_batch_slot(row.batch_slot),
                Some(crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(key))
                    if key.owner() == "Probe" && key.arity() == arity)).unwrap();
        let proposed = OwnedCallableParameterContractDeclarationV1 {
            owner: contract.owner, batch_slot: contract.batch_slot,
            mode: contract.mode, parameters: Box::new([]),
        };
        let fields = composition::FieldCandidatesV1::prepare(package.batch(), &package.selected,
            &package.ordinary_new_claim_ledger.field_write_claims, &package.instance_constructors,
            Err(&"test/no-original-source".to_owned()), &BTreeMap::new(), &package.parameter_contracts, &[]);
        let result = source_result_pending(package.batch(), &package.selected, &[proposed],
            &BTreeMap::new(), &[], &package.instance_constructors, &BTreeMap::new(), &fields, None, None, &package.ordinary_new_claim_ledger.field_write_claims, contract.owner);
        if arity == 0 {
            let result = result.unwrap();
            assert_eq!(result.class, BorrowedResultClassV1::Bool);
            assert_eq!(result.returns.len(), 1);
            assert!(result.parameterless_instance.as_ref().is_some_and(|(owner, slot, key)|
                *owner == contract.owner && *slot == contract.batch_slot && key.arity() == 0));
        } else {
            assert!(result.unwrap_err().contains("borrowed-result/source-contract-identity"));
        }
    }
}

#[test]
fn qualified_scalar_family_keeps_preflight_and_retained_loss_fail_closed() {
    let text = "box Handle { block_id: i64 birth() { me.block_id = 0 } } box Leaf { probe(p: Handle) { if p == null { return false } return true } } box Parent { child: Leaf birth() { me.child = new Leaf() } run(p: Handle) { if p == null { return false } return me.child.probe(p) } qualified() { local h = new Handle() return me.run(h) } } static box Main { main() { return 0 } }";
    let mut package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(text).unwrap();
    let ledger = std::rc::Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    let ingress = ledger.borrowed_formal_source.as_ref().unwrap().as_ref().unwrap();
    let original = ingress.source_incoming.exact_rows().find(|row|
        row.source.target().owner() == "Parent" && row.source.target().name() == "run").unwrap().clone();
    let target = original.source.require_instance().unwrap();
    assert!(target.stored_receiver().is_none());
    assert!(!ingress.source_only_definitions.contains_key(&original.call.owner()));
    let prepared: super::super::super::PreparedLexicalInstanceCallSourceTargetsV1 = Ok(ingress.source_incoming.exact_rows()
        .filter_map(|row| row.source.instance().cloned().map(|target| Ok(Some(target)))).collect());
    let healthy = source_scalar_result_arguments_v1(&prepared, &package.parameter_contracts,
        ledger.borrowed_formal_source.as_ref().unwrap(), &ledger.borrowed_formal_actuals,
        &ledger.borrowed_i64_results, &original.call, &ledger.callable_result_classes).unwrap().unwrap();
    let crate::mir::resolved_semantics::home_new_prefix::BorrowedCallArgumentsV1::SourceResult { loan: crate::mir::resolved_semantics::home_new_prefix::LocalCallResultSourceV1::Scalar(loan), .. } = healthy else {
        panic!("original dependency anchors qualified incoming");
    };
    for missing in [false, true] {
        let saved = ledger.borrowed_i64_results.remove(&original.callee).unwrap();
        if !missing { ledger.borrowed_i64_results.insert(original.callee, Err("original-result-refusal".into())); }
        let initial = source_scalar_result_arguments_v1(&prepared, &package.parameter_contracts,
            ledger.borrowed_formal_source.as_ref().unwrap(), &ledger.borrowed_formal_actuals,
            &ledger.borrowed_i64_results, &original.call, &ledger.callable_result_classes);
        assert!(!matches!(initial, Ok(Some(_))), "no source loan without original result");
        let integer = super::super::super::borrowed_formal_actuals::project_pending_i64_result_arguments_v1(
            ledger.borrowed_formal_source.as_ref().unwrap(), &ledger.borrowed_formal_actuals,
            &package.parameter_contracts, &ledger.borrowed_i64_results, &original.call, true);
        assert!(!matches!(integer, Ok(Some(_))), "no Integer permission after failed scalar preflight");
        assert!(validate_source_scalar_result_loan_v1(ledger.borrowed_formal_source.as_ref().unwrap(),
            &ledger.borrowed_i64_results, &loan, target).is_err());
        ledger.borrowed_i64_results.insert(original.callee, saved);
    }
    for change in 0..3 {
        let proof = ledger.borrowed_i64_results.get_mut(&original.callee).unwrap().as_mut().unwrap();
        let saved = proof.source_facts.clone();
        let dependency = &mut proof.source_facts_mut_for_test().dependencies[0];
        match change {
            0 => dependency.receiver_site = dependency.argument_sites[0].clone(),
            1 => dependency.target_batch_slot += 1,
            _ => dependency.callee_owner = original.call.owner(),
        }
        assert!(source_scalar_result_arguments_v1(&prepared, &package.parameter_contracts,
            ledger.borrowed_formal_source.as_ref().unwrap(), &ledger.borrowed_formal_actuals,
            &ledger.borrowed_i64_results, &original.call, &ledger.callable_result_classes).is_err());
        ledger.borrowed_i64_results.get_mut(&original.callee).unwrap().as_mut().unwrap().source_facts = saved;
    }
}
