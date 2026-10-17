//! A source candidate must retain scanner vetoes beside otherwise good input.
use super::*;

#[test]
fn candidate_integer_agreement_refuses_good_row_plus_unresolved_caller_veto() {
    let mut package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(
        "static box Layout { class_id(size) { local alias: i64 = size return 0 } } box Heap { lookup(size) { local k = Layout.class_id(size) return 0 } } static box Main { main() { local heap = new Heap() local a = heap.lookup(7) local b = heap.lookup(8) return 0 } }",
    ).unwrap();
    let source = package
        .ordinary_new_claim_ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    let fact = source.static_arguments.values().next().unwrap();
    assert!(source.candidate_integer_agreement(fact.formal()));
    assert!(!source.formal_integer_agreement(fact.formal()));
    assert!(!source.contains_definition_for_test(fact.formal().owner()));
    assert_eq!(
        source.candidate_input_inventory_for_test(fact.formal().owner()),
        (2, false)
    );
    let contract = package
        .parameter_contracts
        .iter()
        .find(|row| row.owner == fact.call().owner())
        .unwrap();
    let draft = package
        .batch()
        .with_lowering_input(contract.batch_slot, |input| {
            draft_borrowed_formal_uses_v1(input, contract, &package.instance_constructors, None)
                .unwrap()
        })
        .unwrap();
    let definitions = BTreeMap::from([(contract.owner, draft)]);
    let mut calls: BTreeMap<_, _> = source
        .source_incoming
        .exact_rows()
        .filter(|row| row.callee == contract.owner)
        .map(|row| (row.call.clone(), row.source.require_instance().unwrap()))
        .collect();
    assert_eq!(calls.len(), 2);
    let missing = calls.keys().next_back().unwrap().clone();
    calls.remove(&missing);
    let scope = package
        .batch()
        .declarations()
        .map(|row| row.owner())
        .collect();
    let inventory = inventory_borrowed_incoming_calls_v1(
        package.batch(),
        &package.selected,
        &definitions,
        &package.parameter_contracts,
        &calls,
        &scope,
        None,
    )
    .unwrap();
    assert_eq!(inventory.exact_rows().count(), 1);
    assert!(inventory.vetoed_owners().contains(&contract.owner));
    let empty_loans =
        crate::mir::resolved_semantics::VerifiedInstanceEntryHomeCatalogV1::issue(Box::new([]));
    let original_seeds = source_seeds::prepare_original_formal_seeds_v1(
        package.batch(), &package.selected, &empty_loans, &package.instance_constructors,
        &package.ordinary_new_claim_ledger.callable_result_classes, &BTreeMap::new(),
        &package.parameter_contracts, &definitions, &inventory, &BTreeMap::new(), None, &BTreeMap::new(),
    ).unwrap();
    let (objects, integers, _) = original_seeds.selected_views_v1(
        &BTreeMap::new(), package.batch(), &package.instance_constructors,
        &package.parameter_contracts, &BTreeSet::new(), &inventory,
    ).unwrap();
    assert!(objects.is_empty());
    assert!(integers.is_empty());
    assert_eq!(
        inventory
            .project(&definitions.keys().copied().collect())
            .unwrap_err(),
        BorrowedIncomingDraftErrorV1::UnresolvedCaller(missing)
    );
    let owner = contract.owner;
    let ledger = std::rc::Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    ledger.borrowed_formal_source.as_mut().unwrap().as_mut().unwrap().source_incoming = inventory;
    let ingress = ledger.borrowed_formal_source.as_ref().unwrap().as_ref().unwrap();
    assert!(ledger.original_instance_input_requires_reconstruction_v1(ingress, owner)
        .unwrap_err().contains("instance-input/original-incoming-coverage"));
}

/// Source observation uses the original identity even when ClosedApp omits
/// physical selection. Omission is verified, never treated as a missing row.
fn original_static_contract<'a>(
    package: &'a crate::mir::normal_callable_semantic_package::VerifiedNormalCallableSemanticPackageV1,
    key: &CanonicalSameModuleCallableKeyV1,
) -> &'a OwnedCallableParameterContractDeclarationV1 {
    let mut declarations = package.batch().declarations().filter(|row| {
        match package.selected.key_for_batch_slot(row.batch_slot()) {
            Some(SelectedNormalCallableKeyV1::Cataloged(selected)) => selected == key,
            None => package.source_backed_callable_catalog()
                .omitted_closed_app_static_for_identity(row.identity()) == Some(key),
            Some(_) => false,
        }
    });
    let declaration = declarations.next().expect("one original Static declaration");
    assert!(declarations.next().is_none(), "unique original source key");
    let mut rows = package.parameter_contracts.iter().filter(|row| row.owner == declaration.owner());
    let contract = rows.next().expect("same original parameter catalog");
    assert!(rows.next().is_none());
    assert_eq!(contract.batch_slot, declaration.batch_slot());
    assert_eq!(contract.mode, CallableParameterDeclarationModeV1::StaticBoxMethod);
    assert_eq!(contract.parameters.len(), key.arity() as usize);
    package.batch().with_lowering_input(contract.batch_slot, |input| {
        assert_eq!(input.owner(), contract.owner);
        for (ordinal, parameter) in contract.parameters.iter().enumerate() {
            assert_eq!(parameter.ordinal as usize, ordinal);
            assert_eq!(input.function().declaration_binding(
                &crate::mir::resolved_semantics::SourceBindingSiteV1::Parameter { index: parameter.ordinal }),
                Some(parameter.binding));
        }
    }).unwrap();
    contract
}

#[test]
fn real_mimalloc_incoming_domain_keeps_all_callers_without_false_stored_veto() {
    std::thread::Builder::new().name("mimalloc-original-static-claim".into())
        .stack_size(32 * 1024 * 1024).spawn(|| {
        let env_updates: Vec<_> = crate::test_support::JOINIR_DEFAULT_MODE.into_iter().chain([
            ("NYASH_ALLOW_USING_FILE", Some("1")), ("NYASH_ENABLE_USING", Some("1")),
            ("NYASH_OPERATOR_BOX_ALL", Some("0")), ("NYASH_MACRO_DISABLE", Some("1")),
        ]).collect();
        crate::test_support::with_env_vars(&env_updates, || {
            use crate::mir::builder::{NormalRootExecutionConsumerV1, SelectedNormalCallableKeyV1};
            use crate::mir::normal_callable_semantic_package::qualified_static_call_claim::caller_key_for_function;
            use crate::mir::resolved_semantics::{FunctionSemanticResolverSessionV1, SourcePathSegmentV1};
            use crate::runner::modes::common_util::normal_callable::{
                materialize_normal_callable_program_with_identity_and_lineage_v1,
                NormalCallableMaterializationOutcomeV1,
            };
            let filename = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("apps/mimalloc-lite/main.hako");
            let code = std::fs::read_to_string(&filename).unwrap();
            let runner = crate::runner::NyashRunner::new(Default::default());
            let prepared = crate::runner::modes::common_util::source_hint::prepare_normal_source_with_imports(
                &runner, filename.to_str().unwrap(), &code,
            ).expect("same production normal import preparation");
            assert!(prepared.lineage.edges.len() > 1);
            let imports: Vec<_> = prepared.imports.into_iter().collect();
            let transformed = materialize_normal_callable_program_with_identity_and_lineage_v1(
                prepared.code, runner.parser_build_config(), filename.to_string_lossy().into_owned(), prepared.lineage,
            ).expect("unchanged imported callable source");
            let NormalCallableMaterializationOutcomeV1::SourceBacked(source) = transformed else {
                panic!("mimalloc must remain source-backed")
            };
            let catalog = crate::analysis::brand_program_declaration_catalog::issue_brand_program_declaration_catalog_v1(source.ast()).unwrap();
            let consumed = NormalRootExecutionConsumerV1::consume_once(source).unwrap().into_consumed_source();
            let package = crate::mir::normal_callable_semantic_package::issue_normal_callable_semantic_package_with_selection_v1(
                &mut FunctionSemanticResolverSessionV1::new(93).unwrap(), consumed, Some(&catalog),
                crate::mir::builder::LoopFactsPolicyFrameV1::from_environment(), &imports,
                crate::mir::normal_callable_semantic_package::PackagePhysicalSelectionV1::ClosedAppExecutable,
            ).expect("one original factory");
            // The unchanged application's saved issuer draft, with its original
            // nullable/result context. Re-drafting with None loses that context.
            let resize_key = CanonicalSameModuleCallableKeyV1::instance_box_method("HakoAllocPage", "resizeInPlace", 2);
            let resize_slot = package.selected.batch_slot(&SelectedNormalCallableKeyV1::Cataloged(resize_key)).unwrap();
            let resize_contract = package.parameter_contracts.iter().find(|row| row.batch_slot == resize_slot).unwrap();
            let ingress = package.ordinary_new_claim_ledger.borrowed_formal_source.as_ref().unwrap().as_ref().unwrap();
            let resize_draft = ingress.source_definition_for(resize_contract.owner).expect("actual saved resize draft");
            package.batch().with_lowering_input(resize_slot, |input| {
                let formal = resize_contract.parameters[1].binding;
                let calls: Vec<_> = input.function().method_calls().filter(|(_, call)| call.selector() == "set"
                    && call.arguments().get(1).is_some_and(|arg|
                        input.function().variable_ref(arg.site()) == Some(crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(formal))))
                    .collect();
                assert_eq!(calls.len(), 1, "actual requested-size Set source");
                let (call_site, call) = calls[0];
                let rows: Vec<_> = resize_draft.uses.iter().filter(|row| row.formal == formal
                    && &row.site.site() == &call.arguments()[1].site()).collect();
                assert_eq!(rows.len(), 1, "actual value operand source identity");
                let BorrowedFormalUseDraftKindV1::ArrayElementFieldIndexValue { call: original, index, reach, .. } = &rows[0].kind else {
                    panic!("actual Set must retain full field-index source requirement; kind={:?}", std::mem::discriminant(&rows[0].kind));
                };
                assert_eq!(original.site(), call_site);
                assert_eq!(original.owner(), input.owner());
                assert_eq!(reach.operand(), (rows[0].binding, formal, &rows[0].site));
                assert!(reach.corroborates(input), "same original Integer guard and source reach");
                let index_site = call.arguments()[0].site();
                let Some(crate::mir::resolved_semantics::BodyExpressionShapeV1::FieldAccess { object, .. }) = input.body_shape().unwrap().expression_shape(index_site)
                    else { panic!("actual handle field index") };
                let Some(crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(binding)) = input.function().variable_ref(object)
                    else { panic!("actual handle lexical receiver") };
                assert!(index.matches_source(input, index_site, binding, resize_draft.origins[&binding]),
                    "same original field access and nonnull guard");
            }).unwrap();
            let claims = &package.source_static_claims_for_test;
            let uncalled_accepts = CanonicalSameModuleCallableKeyV1::static_box_method(
                "LayoutBox", "accepts", 1,
            );
            assert!(package.declaration_catalog().declaration(&uncalled_accepts).is_some());
            assert!(package.selected.batch_slot(&SelectedNormalCallableKeyV1::Cataloged(
                uncalled_accepts.clone(),
            )).is_none(), "closed App EXE omits only physical selected membership");
            assert!(claims.current_owner_sources_for_test().any(|(caller, _, row)| {
                caller == &uncalled_accepts
                    && row.route().target()
                        == &CanonicalSameModuleCallableKeyV1::static_box_method(
                            "LayoutBox", "class_id", 1,
                        )
            }), "original unselected accepts -> class_id source row remains");
            // Inspect the original excluded draft before attributing the
            // executable SourceOnly stop to a receiver or packet consumer.
            for (class, method, checked_return) in [
                ("LayoutBox", "class_id", false),
                ("SizeClassBox", "good_size", false),
                ("SizeClassBox", "size_to_bin", false),
                ("SizeClassBox", "normalize_size", true),
            ] {
                let key = CanonicalSameModuleCallableKeyV1::static_box_method(class, method, 1);
                let contract = original_static_contract(&package, &key);
                let slot = contract.batch_slot;
                let original = package.batch().with_lowering_input(slot, |input| {
                    super::super::borrowed_formal_uses::draft_borrowed_formal_uses_v1(
                        input, contract, &package.instance_constructors, None,
                    )
                }).unwrap();
                if class == "LayoutBox" || method == "good_size" {
                    assert!(claims.contains_exact_i64_target(&key), "original qualified target {key:?}");
                } else {
                    let current: Vec<_> = claims.current_owner_sources_for_test()
                        .filter(|(_, _, row)| row.route().target() == &key).collect();
                    assert!(!current.is_empty(), "original current-owner route {key:?}");
                    for (caller, site, row) in current {
                        assert_eq!(caller.owner(), key.owner());
                        assert!(std::ptr::eq(claims.current_owner_source(caller, site).unwrap(), row));
                        assert!(claims.claim_target(caller, site).is_none());
                    }
                }
                let original = original.expect("original source draft");
                if checked_return {
                    let row = original.uses.iter().find(|row| matches!(row.kind,
                        BorrowedFormalUseDraftKindV1::IntegerReturn { .. })).expect("original checked Return");
                    assert_eq!(row.site.owner(), contract.owner);
                    assert_eq!(row.site.site().node().segments(), &[
                        SourcePathSegmentV1::Body(1), SourcePathSegmentV1::Value,
                    ]);
                    let BorrowedFormalUseDraftKindV1::IntegerReturn { guard, .. } = &row.kind else { unreachable!() };
                    assert!(original.uses.iter().any(|compare| match &compare.kind {
                        BorrowedFormalUseDraftKindV1::CompareOperand { binary, source } =>
                            guard.matches_compare(compare.formal, binary, source),
                        _ => false,
                    }), "SAME original guard, not merely a Return admission");
                }
            }
            let bin_key = CanonicalSameModuleCallableKeyV1::static_box_method("SizeClassBox", "bin_size", 1);
            let bin_contract = original_static_contract(&package, &bin_key);
            let bin_slot = bin_contract.batch_slot;
            let operand_context = super::super::borrowed_formal_uses::StaticOperandContextV1 {
                index: claims, selected: &package.selected, contracts: &package.parameter_contracts, caller: &bin_key,
            };
            package.batch().with_lowering_input(bin_slot, |input| {
                let (call_site, call) = input.function().method_calls().find(|(_, call)| call.selector() == "max_regular_bin").unwrap();
                let binary = input.function().expression_source().binaries().find(|binary| binary.rhs() == call_site).unwrap();
                let formal = bin_contract.parameters[0].binding;
                let Some(BorrowedFormalUseDraftKindV1::CompareOperand { source, .. }) =
                    super::super::borrowed_formal_uses::compare_operand_kind(input,
                        &BTreeMap::from([(formal, formal)]), &package.instance_constructors,
                        None, binary.lhs(), Some(&operand_context)).unwrap() else { panic!("original bin comparison child") };
                let original = source.integer_call_source().unwrap();
                assert_eq!(original.call_site().site(), call_site);
                assert_eq!(original.target(), &CanonicalSameModuleCallableKeyV1::static_box_method("SizeClassBox", "max_regular_bin", 0));
                let loan = claims.incoming_source(&bin_key, original.call_site(), call,
                    &package.selected, &package.parameter_contracts, None).unwrap().unwrap();
                assert!(loan.corroborates_retained(original));
                assert!(original.required_i64_arguments().is_empty());
                assert!(original.require_qualified().is_err());
                let ingress = package.ordinary_new_claim_ledger.borrowed_formal_source.as_ref().unwrap().as_ref().unwrap();
                assert!(ingress.source_definition_for(input.owner()).is_some(), "original Mul closes formal-use source only");
                assert!(!ingress.contains_definition_for_test(input.owner()), "CurrentOwner-only source is not executable entry");
                let draft = ingress.source_definition_for(input.owner()).unwrap();
                let row = draft.uses.iter().find(|row| matches!(row.kind, BorrowedFormalUseDraftKindV1::MulOperand { .. }))
                    .expect("unchanged bin_size original guarded Mul");
                assert!(draft.mul_operand_at(input, &row.site).unwrap().is_some());
            }).unwrap();
            let facts = &package.ordinary_new_claim_ledger.callable_result_classes;
            use crate::mir::normal_callable_semantic_package::ordinary_new_coseal::result_class_claim::{OrdinaryNewResultClassV1, ResultWitnessStepV1};
            let page_key = CanonicalSameModuleCallableKeyV1::instance_box_method("HakoAllocPage", "allocate", 1);
            assert!(matches!(facts.get(&page_key), Some(OrdinaryNewResultClassV1::NullableObject(name)) if name.as_ref() == "HakoAllocHandle"));
            let page_exits = facts.outcomes(&page_key).expect("all original Page exits");
            assert_eq!(page_exits.len(), 3);
            let page_witnesses: Vec<_> = page_exits.iter().flat_map(|row| row.witnesses()).collect();
            assert_eq!(page_witnesses.len(), 3);
            assert_eq!(page_witnesses.iter().filter(|row| matches!(row.step(), ResultWitnessStepV1::NullLiteral)).count(), 2);
            assert_eq!(page_witnesses.iter().filter(|row| matches!(row.step(), ResultWitnessStepV1::FreshConstruction)).count(), 1);
            assert!(facts.object_return_dependencies(&page_key, page_exits[0].site().owner()).is_none(), "leaf Fresh/null has no call dependency");
            let heap_key = CanonicalSameModuleCallableKeyV1::instance_box_method("HakoAllocHeap", "allocate", 1);
            assert!(matches!(facts.get(&heap_key), Some(OrdinaryNewResultClassV1::NullableObject(name)) if name.as_ref() == "HakoAllocHandle"));
            let heap_exits = facts.outcomes(&heap_key).expect("BOTH Page calls plus null");
            assert_eq!(heap_exits.len(), 3);
            assert_eq!(heap_exits.iter().flat_map(|row| row.witnesses()).count(), 7);
            let result_key = CanonicalSameModuleCallableKeyV1::instance_box_method("HakoAllocHeap", "allocateResult", 1);
            let result_slot = package.selected.batch_slot(&SelectedNormalCallableKeyV1::Cataloged(result_key)).unwrap();
            let result_contract = package.parameter_contracts.iter().find(|row| row.batch_slot == result_slot).unwrap();
            package.batch().with_lowering_input(result_slot, |input| {
                let (site, _) = input.function().method_calls().find(|(_, call)| call.selector() == "allocate")
                    .expect("original nullable acquisition");
                let site = crate::mir::resolved_semantics::OwnedExprSiteV1::new(input.owner(), site.clone());
                let completion = package.ordinary_new_claim_ledger.completion_for_owner(result_contract.owner).unwrap();
                let calls: Vec<_> = completion.cleanup().root_flow().unwrap().local_calls().iter()
                    .filter(|call| call.site() == &site).collect();
                assert_eq!(calls.len(), 1, "one original acquisition, independent of terminal exits");
                let call = calls[0];
                assert_eq!(call.result(), crate::mir::resolved_semantics::home_new_prefix::LocalCallResultClassV1::Nullable);
                let binding = call.local_binding().unwrap().1;
                assert_eq!(package.ordinary_new_claim_ledger.borrowed_nullable_actual_class_v1(binding).unwrap().as_deref(),
                    Some("HakoAllocHandle"), "same source acquisition and callee claim");
            }).unwrap();
            let wrapper_key = CanonicalSameModuleCallableKeyV1::instance_box_method("HakoAllocHeap", "reallocResult", 2);
            let wrapper_slot = package.selected.batch_slot(&SelectedNormalCallableKeyV1::Cataloged(wrapper_key.clone())).unwrap();
            let wrapper_contract = package.parameter_contracts.iter().find(|row| row.batch_slot == wrapper_slot).unwrap();
            let mut original_supports = Vec::new();
            for witness in facts.outcomes(&wrapper_key).unwrap().iter().flat_map(|row| row.witnesses()) {
                if matches!(witness.step(), ResultWitnessStepV1::FreshConstruction) {
                    original_supports.extend(package.ordinary_new_claim_ledger.returned_new_field_supports_v1(witness.site()).unwrap());
                }
            }
            assert_eq!(original_supports.len(), 2, "both original Page resize branches retain the handle formal");
            for support in original_supports {
                assert_eq!(support.formal, wrapper_contract.parameters[0].binding);
                assert_eq!(support.ordinal, 0);
                assert!(facts.construction_child_relation_loans().any(|relation|
                    std::rc::Rc::ptr_eq(relation, &support.relation)), "same original child relation allocation");
            }
            let dependencies = facts.object_return_dependencies(&heap_key, heap_exits[0].site().owner()).expect("both original call-return loans");
            assert_eq!(dependencies.len(), 2);
            for dependency in &dependencies {
                assert_eq!(dependency.key(), &page_key);
                assert_eq!(dependency.witnesses().len(), 3);
                for witness in dependency.witnesses() {
                    let ResultWitnessStepV1::Call { key, callee, substitution, .. } = witness.step() else { panic!("original call witness") };
                    assert_eq!(key, &page_key);
                    assert!(substitution.is_none());
                    assert!(page_witnesses.iter().any(|original| std::rc::Rc::ptr_eq(original, callee)), "same leaf witness, no reissue");
                }
            }
            let source = package.ordinary_new_claim_ledger.borrowed_formal_source.as_ref().unwrap()
                .as_ref().expect("retained original source incoming");
            for (class, method) in [
                ("SizeClassBox", "normalize_size"),
                ("SizeClassBox", "size_to_bin"),
                ("SizeClassBox", "good_size"),
            ] {
                let key = CanonicalSameModuleCallableKeyV1::static_box_method(class, method, 1);
                let formal = original_static_contract(&package, &key).parameters[0].binding;
                assert!(source.checked_static_input(formal), "original checked/forward chain {class}.{method}");
            }
            // An omitted original declaration keeps its source claims, but
            // has neither a source draft nor a checked executable input.
            let size_accepts = CanonicalSameModuleCallableKeyV1::static_box_method("SizeClassBox", "accepts", 1);
            let accepts_contract = original_static_contract(&package, &size_accepts);
            assert!(package.selected.batch_slot(&SelectedNormalCallableKeyV1::Cataloged(size_accepts.clone())).is_none());
            assert!(source.source_definition_for(accepts_contract.owner).is_none());
            assert!(!source.checked_static_input(accepts_contract.parameters[0].binding));
            for (method, arity) in [("size_to_bin", 1), ("huge_bin", 0)] {
                let target = CanonicalSameModuleCallableKeyV1::static_box_method("SizeClassBox", method, arity);
                let retained: Vec<_> = claims.current_owner_sources_for_test()
                    .filter(|(caller, _, row)| *caller == &size_accepts && row.route().target() == &target)
                    .collect();
                assert_eq!(retained.len(), 1, "one original omitted caller edge {method}");
                package.batch().with_lowering_input(accepts_contract.batch_slot, |input| {
                    let call = input.function().method_call(retained[0].1).expect("same original call site");
                    assert_eq!(call.selector(), method);
                    assert_eq!(call.arguments().len(), arity as usize);
                }).unwrap();
            }
            let class_id = CanonicalSameModuleCallableKeyV1::static_box_method("LayoutBox", "class_id", 1);
            let class_id_formal = original_static_contract(&package, &class_id).parameters[0].binding;
            assert_eq!(source.candidate_input_inventory_for_test(class_id_formal.owner()), (1, false));
            assert!(source.candidate_integer_agreement(class_id_formal));
            assert!(source.source_incoming.static_observations().values().filter_map(|row| row.as_ref().ok())
                .all(|row| row.caller() != &uncalled_accepts));
            let good_size = CanonicalSameModuleCallableKeyV1::static_box_method("SizeClassBox", "good_size", 1);
            let good_contract = original_static_contract(&package, &good_size);
            let good_slot = good_contract.batch_slot;
            let bin_target = CanonicalSameModuleCallableKeyV1::static_box_method("SizeClassBox", "size_to_bin", 1);
            let bin_contract = original_static_contract(&package, &bin_target);
            let bin_slot = bin_contract.batch_slot;
            package.batch().with_lowering_input(bin_slot, |input| {
                let loop_site = input.function().loop_sites().next().expect("original size_to_bin loop");
                let ledger = &package.ordinary_new_claim_ledger;
                assert!(ledger.expects_loop_static_source_loan_v1(input.owner(), loop_site));
                let issued_product = crate::mir::builder::take_at_function_entry_v2(input, ledger)
                    .unwrap().expect("same preissued source-bound product");
                let product = issued_product.semantic();
                for (selector, arity) in [("max_regular_bin", 0), ("bin_size", 1)] {
                    let (site, call) = input.function().method_calls()
                        .find(|(_, call)| call.selector() == selector)
                        .expect("original size_to_bin loop call");
                    let owned = crate::mir::resolved_semantics::OwnedExprSiteV1::new(input.owner(), site.clone());
                    let original = source.source_incoming.static_observations()[&owned]
                        .as_ref().expect("original loop Static Rc");
                    let claim = crate::mir::normal_callable_semantic_package::qualified_static_call_claim::claim_for_source_site_v1(
                        claims, Some(&bin_target), &input, &owned, &package.selected,
                        &package.parameter_contracts, None,
                    ).unwrap().expect("same issuer accepts original loop call");
                    assert_eq!(call.arity(), arity);
                    assert!(claim.corroborates_source(&owned,
                        crate::mir::resolved_semantics::ResolvedMethodCallReceiverSourceV1::CurrentOwner,
                        arity));
                    assert_eq!(original.call_site(), &owned);
                    assert_eq!(original.argument_sites().len(), arity as usize);
                    let loan = crate::mir::normal_callable_semantic_package::ordinary_new_coseal::loop_static_source_loan::issue_loop_static_source_call_loan_v1(
                        claims, source, &package.selected, &package.parameter_contracts,
                        &bin_target, input, loop_site, &owned,
                    ).unwrap().expect("original Loop Static source loan");
                    assert_eq!(loan.loop_site(), loop_site);
                    assert!(std::rc::Rc::ptr_eq(loan.original(), original));
                    assert!(loan.claim().corroborates_source(&owned, call.receiver(), arity));
                    assert_eq!(loan.placement(), &input.function().resolved_loop_placement(loop_site, site).unwrap().unwrap());
                    assert!(!ledger.loop_static_source_loans.borrow().contains_key(&(loop_site.clone(), owned.clone())));
                    let retained = if arity == 0 { product.source_calls().1 } else { product.source_calls().2 };
                    assert!(std::rc::Rc::ptr_eq(retained.original(), original));
                    assert_eq!(retained.placement(), loan.placement());
                    assert!(ledger.take_loop_static_source_call_loan_v1(loop_site, &owned).is_none());
                }
                let outside = input.function().method_calls()
                    .find(|(_, call)| call.selector() == "normalize_size").unwrap().0;
                let outside = crate::mir::resolved_semantics::OwnedExprSiteV1::new(input.owner(), outside.clone());
                let initializer = input.function().expression_source().initializers()
                    .find(|row| row.initializer_site() == Some(outside.site()))
                    .expect("original n initializer");
                let entry = crate::mir::normal_callable_semantic_package::ordinary_new_coseal::loop_static_source_loan::issue_loop_entry_static_i64_source_loan_v1(
                    claims, source, &package.selected, &package.parameter_contracts,
                    &bin_target, input, loop_site, initializer.declaration_site(),
                ).unwrap().expect("original pre-loop I64 result source");
                assert_eq!(entry.loop_site(), loop_site);
                assert_eq!(entry.declaration(), initializer.declaration_site());
                assert_eq!(entry.original().call_site(), &outside);
                assert!(entry.claim().corroborates_source(
                    &outside, crate::mir::resolved_semantics::ResolvedMethodCallReceiverSourceV1::CurrentOwner, 1));
                assert!(std::rc::Rc::ptr_eq(entry.original(), product.source_calls().0.original()));
                assert!(ledger.take_loop_entry_static_i64_source_loan_v1(loop_site, initializer.declaration_site()).is_none());
                let completion = package.ordinary_new_claim_ledger
                    .completion_for_owner(input.owner()).expect("original full source Completion");
                let site_for = |selector| {
                    let site = input.function().method_calls()
                        .find(|(_, call)| call.selector() == selector).unwrap().0;
                    crate::mir::resolved_semantics::OwnedExprSiteV1::new(input.owner(), site.clone())
                };
                let wrong_header = crate::mir::normal_callable_semantic_package::ordinary_new_coseal::loop_static_source_loan::issue_loop_static_source_call_loan_v1(
                    claims, source, &package.selected, &package.parameter_contracts,
                    &bin_target, input, loop_site, &site_for("bin_size"),
                ).unwrap().unwrap();
                let wrong_body = crate::mir::normal_callable_semantic_package::ordinary_new_coseal::loop_static_source_loan::issue_loop_static_source_call_loan_v1(
                    claims, source, &package.selected, &package.parameter_contracts,
                    &bin_target, input, loop_site, &site_for("max_regular_bin"),
                ).unwrap().unwrap();
                let negative_entry = crate::mir::normal_callable_semantic_package::ordinary_new_coseal::loop_static_source_loan::issue_loop_entry_static_i64_source_loan_v1(
                    claims, source, &package.selected, &package.parameter_contracts,
                    &bin_target, input, loop_site, initializer.declaration_site(),
                ).unwrap().unwrap();
                let negative_tail = crate::mir::normal_callable_semantic_package::ordinary_new_coseal::loop_static_source_loan::issue_loop_tail_static_i64_source_loan_v1(
                    claims, source, &package.selected, &package.parameter_contracts,
                    &bin_target, input, loop_site,
                ).unwrap().unwrap();
                assert!(crate::mir::builder::produce_static_i64_loop_semantic_v2(
                    input, loop_site, completion, negative_entry, wrong_header, wrong_body, negative_tail,
                ).is_err(), "swapped original CallSlot proofs must fail before MIR");
                let tail = product.tail_call();
                assert_eq!(tail.loop_site(), loop_site);
                assert!(tail.original().call_site().site() == site_for("huge_bin").site());
                assert!(ledger.take_loop_tail_static_i64_source_loan_v1(loop_site).is_none());
                assert!(crate::mir::builder::take_at_function_entry_v2(input, ledger)
                    .unwrap_err().contains("static-i64-v2/source-unavailable"));
                assert_eq!(product.recipe().as_recipe().items.len(), 13);
                assert_eq!(product.join().after_binding().raw(), 0);
                assert_ne!(product.roles().n_binding, product.roles().bin_binding);
                assert_eq!((product.roles().header_call.raw(), product.roles().body_call.raw(), product.roles().backedge_write.raw()), (1, 4, 12));
                assert_eq!(product.source_calls().1.placement(), &crate::mir::resolved_semantics::ResolvedLoopPlacementV1::Condition);
                assert_eq!(product.tail_call().return_site(), package.ordinary_new_claim_ledger
                    .completion_for_owner(input.owner()).unwrap().explicit_sites().last().unwrap());
                assert!(crate::mir::normal_callable_semantic_package::ordinary_new_coseal::loop_static_source_loan::issue_loop_static_source_call_loan_v1(
                    claims, source, &package.selected, &package.parameter_contracts,
                    &bin_target, input, loop_site, &outside,
                ).is_err(), "pre-loop call cannot impersonate a Loop CallSlot");
            }).unwrap();
            package.batch().with_lowering_input(good_slot, |input| {
                let mut calls = input.function().method_calls().filter(|(_, call)| call.selector() == "size_to_bin");
                let (site, call) = calls.next().expect("original good_size CurrentOwner call");
                assert!(calls.next().is_none());
                assert_eq!(call.receiver(), crate::mir::resolved_semantics::ResolvedMethodCallReceiverSourceV1::CurrentOwner);
                let owned = crate::mir::resolved_semantics::OwnedExprSiteV1::new(input.owner(), site.clone());
                let fact = source.static_arguments.get(&(owned.clone(), 0)).expect("original CurrentOwner opaque argument fact");
                let observed = source.source_incoming.static_observations()[&owned].as_ref().unwrap();
                assert!(std::rc::Rc::ptr_eq(fact.retained_call_source(), observed));
                assert_eq!(fact.binding(), good_contract.parameters[0].binding);
                assert_eq!(fact.formal(), good_contract.parameters[0].binding);
                assert_eq!(fact.use_site().site(), call.arguments()[0].site());
                assert_eq!(fact.target(), &bin_target);
                assert_eq!(fact.target_formal(), bin_contract.parameters[0].binding);
                assert_eq!(observed.current_owner_source(), claims.current_owner_source(&good_size, site));
                assert!(!observed.is_qualified());
                assert!(observed.require_qualified().is_err());
                let cohort = source.static_incoming_cohort_v1(observed).unwrap();
                assert!(!cohort.is_empty());
                let transport: Vec<_> = source.incoming.iter().filter(|row| row.callee == bin_contract.owner).collect();
                assert_eq!(transport.len(), cohort.len(), "complete original incoming census");
                for original in &cohort {
                    let rows: Vec<_> = transport.iter().filter(|row| &row.call == original.call_site()).collect();
                    assert_eq!(rows.len(), 1);
                    let super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(retained) = &rows[0].source
                        else { panic!("original Static transport") };
                    assert!(std::rc::Rc::ptr_eq(retained, original));
                    let arguments: Vec<_> = original.parameters().iter()
                        .filter(|parameter| parameter.kind.is_ordinary_borrowed_handle())
                        .map(|parameter| (parameter.ordinal, original.argument_sites()[parameter.ordinal as usize].clone(), parameter.binding)).collect();
                    assert_eq!(rows[0].arguments.as_ref(), arguments.as_slice());
                    let selected = package.ordinary_new_claim_ledger
                        .selected_static_local_source_v1(original.call_site()).unwrap().unwrap();
                    assert_eq!(selected.call_site(), original.call_site());
                }
                let claim = crate::mir::normal_callable_semantic_package::qualified_static_call_claim::claim_for_source_site_v1(
                    claims, Some(&good_size), &input, &owned, &package.selected,
                    &package.parameter_contracts, None,
                ).unwrap().unwrap();
                let projected = super::super::borrowed_formal_actuals::project_pending_current_owner_static_source_arguments_v1(
                    package.ordinary_new_claim_ledger.borrowed_formal_source.as_ref().unwrap(),
                    &package.ordinary_new_claim_ledger.borrowed_formal_actuals,
                    &owned,
                    &claim,
                ).unwrap();
                assert!(projected.is_some(), "checked forwarding retains original source");
            }).unwrap();
            let usize_key = CanonicalSameModuleCallableKeyV1::static_box_method("SizeClassBox", "bin_size_usize", 1);
            assert!(package.declaration_catalog().declaration(&usize_key).is_some());
            assert!(package.selected.batch_slot(&SelectedNormalCallableKeyV1::Cataloged(usize_key.clone())).is_none());
            assert!(claims.current_owner_sources_for_test().any(|(caller, _, row)| {
                caller == &usize_key && row.route().target() == &bin_key
            }), "uncalled usize wrapper keeps original source route without a physical claim");
            let stored: Vec<_> = source.source_incoming.exact_rows().filter_map(|row| row.source.instance())
                .filter(|row| row.call_site().owner() == heap_exits[0].site().owner()
                    && row.target() == &page_key && row.stored_receiver().is_some()).collect();
            assert_eq!(stored.len(), 2, "BOTH original Stored Page source rows");
            for target in stored {
                assert!(target.has_object_source_requirement());
                assert_eq!(target.object_return_sources().unwrap().len(), 1);
                assert!(target.receiver_binding().is_err(), "Stored remains outside lexical receiver binding");
            }
            let slot = package.selected.batch_slot(&SelectedNormalCallableKeyV1::Cataloged(heap_key.clone())).unwrap();
            let contract = package.parameter_contracts.iter().find(|row| row.batch_slot == slot).unwrap();
            assert_eq!(contract.parameters[0].kind, crate::mir::callable_parameter_contract::CallableParameterContractKindV1::OpaqueHandle);
            package.batch().with_lowering_input(slot, |input| {
                let (site, _) = input.function().method_calls().find(|(_, call)| call.selector() == "class_id").unwrap();
                assert_eq!(site.node().segments(), &[SourcePathSegmentV1::Body(0), SourcePathSegmentV1::Initializer(0)]);
                let caller = caller_key_for_function(&package.selected, slot, false, None).unwrap();
                assert_eq!(caller, heap_key);
                let (claim, target) = claims.claim_target(&caller, site).expect("original imported class_id claim");
                assert_eq!(target, &CanonicalSameModuleCallableKeyV1::static_box_method("LayoutBox", "class_id", 1));
                // ExactI64 result membership does not demand an integer input
                // here. Opaque source arguments still need their own scalar view.
                assert!(claim.required_i64_arguments().is_empty());
                let ingress = package.ordinary_new_claim_ledger.borrowed_formal_source
                    .as_ref().unwrap().as_ref().unwrap();
                let call = crate::mir::resolved_semantics::OwnedExprSiteV1::new(input.owner(), site.clone());
                let fact = ingress.static_arguments.get(&(call.clone(), 0))
                    .expect("same original opaque static argument fact");
                assert_eq!(fact.call(), &call);
                assert_eq!(fact.ordinal(), 0);
                assert_eq!(fact.target(), target);
                assert_eq!(fact.formal(), contract.parameters[0].binding);
                assert_eq!(fact.binding(), contract.parameters[0].binding);
                assert_eq!(fact.use_site().site(), input.function().method_calls()
                    .find(|(observed, _)| *observed == site).unwrap().1.arguments()[0].site());
                assert!(fact.required_i64_arguments().is_empty());
                assert_ne!(fact.target_formal().owner(), input.owner());
                let target_contract = package.parameter_contracts.iter()
                    .find(|row| row.owner == fact.target_formal().owner()).unwrap();
                assert_eq!(target_contract.mode, crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1::StaticBoxMethod);
                let target_formal = &target_contract.parameters[fact.ordinal() as usize];
                assert_eq!(target_formal.binding, fact.target_formal());
                assert_eq!(target_formal.kind, crate::mir::callable_parameter_contract::CallableParameterContractKindV1::OpaqueHandle,
                    "the original static input stays opaque despite its constant-i64 result");
                assert!(!ingress.contains_definition_for_test(input.owner()));
                assert_eq!(ingress.candidate_input_inventory_for_test(input.owner()), (15, false));
                let rows = ingress.source_incoming.project(&BTreeSet::from([input.owner()])).unwrap();
                assert_eq!(rows.len(), 15, "all original Heap callers remain mandatory");
                assert!(rows.iter().all(|row| row.callee == input.owner() && row.source.target() == &heap_key));
                // Exact different Page dispatch removes a false veto, not missing
                // outgoing transport; exact guard facts qualify original incoming actuals.
                assert!(ingress.candidate_integer_agreement(contract.parameters[0].binding));
                assert!(!ingress.formal_integer_agreement(contract.parameters[0].binding));
                assert!(!ingress.guarded_actuals.is_empty(), "original checked guards retained before transport pruning");
            }).unwrap();
            let bin_slot = package.selected.batch_slot(
                &SelectedNormalCallableKeyV1::Cataloged(bin_target.clone())).unwrap();
            let bin_owner = package.parameter_contracts.iter()
                .find(|row| row.batch_slot == bin_slot).unwrap().owner;
            assert!(!package.ordinary_new_claim_ledger
                .borrowed_ordinary_entry_receiver_required_v1(bin_owner).unwrap());
            let original = source.source_incoming.static_observations().values()
                .filter_map(|row| row.as_ref().ok()).find(|row| row.callee_owner() == bin_owner).unwrap().clone();
            let pair_sites: Vec<_> = source.static_incoming_cohort_v1(&original).unwrap()
                .iter().map(|row| row.call_site().clone()).collect();
            let mut package = package;
            let ledger = std::rc::Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
            assert!(ledger.checked_completed_static_one_actuals_v1(&original).unwrap().is_some(),
                "original whole-input lender is complete before mutation");
            assert!(ledger.borrowed_formal_actuals.remove(original.call_site()).is_some());
            assert!(ledger.checked_completed_static_one_actuals_v1(&original).unwrap().is_none(),
                "missing actual invalidates the original completed input");
            assert!(ledger.selected_static_local_source_v1(&pair_sites[0]).is_err(),
                "one missing actual invalidates the selected cohort packet");
        });
    }).unwrap().join().expect("real imported observation");
}

#[path = "ordinary_new_borrowed_formal_candidate_static_loop_tests.rs"]
mod static_loop_tests;

#[test]
fn candidate_integer_forward_retains_agreement_without_reviving_transport() {
    let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(
        "static box Layout { class_id(size) { local alias: i64 = size return 0 } } box Heap { lookup(size) { local k = Layout.class_id(size) local recv = new Heap() local out = recv.sink(size) return 0 } sink(value) { local k = Layout.class_id(value) return 0 } } static box Main { main() { local heap = new Heap() local out = heap.lookup(7) return 0 } }",
    ).unwrap();
    let source = package
        .ordinary_new_claim_ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    assert_eq!(source.static_arguments.len(), 2);
    for fact in source.static_arguments.values() {
        assert!(source.candidate_integer_agreement(fact.formal()));
        assert!(!source.formal_integer_agreement(fact.formal()));
        assert!(!source.contains_definition_for_test(fact.formal().owner()));
        assert!(source.formal_object_view(fact.formal()).is_none());
        assert_eq!(
            source.candidate_input_inventory_for_test(fact.formal().owner()),
            (1, false)
        );
    }
    assert!(source.incoming.is_empty());
    assert!(source.forwards.is_empty());
}

#[test]
fn original_qualified_opaque_input_demands_whole_callee_without_global_cycle() {
    let mut package = original_qualified_package(false);
    let ledger = std::rc::Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    let source = ledger.borrowed_formal_source.as_ref().unwrap().as_ref().unwrap();
    let calls: Vec<_> = source.source_incoming.exact_rows()
        .filter(|row| row.source.target().owner() == "Maker" && row.source.target().name() == "make")
        .cloned().collect();
    assert_eq!(calls.len(), 2, "every original callee edge");
    let target = calls[0].source.require_instance().unwrap().clone();
    let contract = package.parameter_contracts.iter().find(|row| row.owner == target.callee_owner()).unwrap();
    assert!(source.definitions.contains_key(&contract.owner), "original global callee frame selects the owner");
    assert!(ledger.selected_object_input_contract_v1(source, contract).unwrap());
    let unrelated = source.source_incoming.exact_rows()
        .find(|row| row.source.target().owner() == "Layout").unwrap().call.clone();
    let issuance = ledger.original_instance_input_issuance.take().unwrap();
    assert!(ledger.checked_completed_opaque_object_target_arguments_v1(&target, contract).unwrap().is_none(), "source qualification is no completed input");
    ledger.original_instance_input_issuance = Some(issuance);
    let original_other = ledger.borrowed_formal_actuals.insert(unrelated.clone(), Err("unrelated-static-refusal".into())).unwrap();
    let args = ledger.checked_completed_opaque_object_target_arguments_v1(&target, contract)
        .unwrap().expect("original family has no dependency on another consumer");
    assert_eq!(args, ledger.checked_completed_typed_object_target_arguments_v1(
        &target, &package.selected, &package.parameter_contracts, &package.physical_signature,
    ).unwrap().unwrap());
    let sibling = calls[1].call.clone();
    let retained = ledger.borrowed_formal_actuals.remove(&sibling).unwrap();
    assert!(ledger.checked_completed_opaque_object_target_arguments_v1(&target, contract).unwrap().is_none());
    ledger.borrowed_formal_actuals.insert(sibling.clone(), Err("original-callee-sibling-refusal".into()));
    assert_eq!(ledger.checked_completed_opaque_object_target_arguments_v1(&target, contract).unwrap_err(), "original-callee-sibling-refusal");
    ledger.borrowed_formal_actuals.insert(sibling, retained);
    ledger.borrowed_formal_actuals.insert(unrelated, original_other);
}

fn original_qualified_package(scalar: bool) -> crate::mir::normal_callable_semantic_package::VerifiedNormalCallableSemanticPackageV1 {
    if scalar {
        return crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog("box Handle { block_id: i64 birth() { me.block_id = 0 } } box Maker { values: ArrayBox = new ArrayBox() birth() {} make(handle): i64 { if handle == null { return 0 } me.values.set(0, 7) return me.values.get(handle.block_id) } } box Holder { child: Maker birth() { me.child = new Maker() } relay(handle: Handle): i64 { if handle == null { return 0 } return me.child.make(handle) } relayAlias(handle: Handle): i64 { if handle == null { return 0 } local alias = handle return me.child.make(alias) } qualified(): i64 { local h = new Handle() local n = me.relay(h) return me.relayAlias(h) } } static box Layout { id(p): i64 { return 0 } } static box Main { main() { local n = Layout.id(7) return 0 } }").unwrap();
    }
    crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog("box Token {} box Maker { make(size) { if size <= 0 { return new Token() } return new Token() } qualified() { return me.make(7) } relay(size) { if size <= 0 { return 0 } local item = me.make(size) return 0 } } static box Layout { id(p): i64 { return 0 } } static box Main { main() { local n = Layout.id(7) return 0 } }").unwrap()
}

#[test]
fn original_qualified_body_entry_shares_whole_native_input_owner() {
    for scalar in [false, true] {
    for mutation in 0..4 {
        let mut package = original_qualified_package(scalar);
        {
            let ledger = std::rc::Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
            let source = ledger.borrowed_formal_source.as_ref().unwrap().as_ref().unwrap();
            let mut own = source.source_incoming.exact_rows().filter(|call|
                call.source.target().owner() == "Maker" && call.source.target().name() == "make");
            let first = own.next().unwrap();
            let sibling = own.next().unwrap().call.clone();
            let unrelated = source.source_incoming.exact_rows().find(|call|
                call.source.target().owner() == "Layout").unwrap().call.clone();
            let kind = ledger.selected_original_instance_input_kind_v1(source, first.callee).unwrap();
            assert!(kind.is_some(), "original source participant must select native input");
            assert!(ledger.original_instance_input_kind_v1(source, first.callee).unwrap() == kind);
            assert!(ledger.selected_instance_input_protocol_v1(source, first.callee).unwrap() == kind);
            assert_eq!(ledger.selected_original_object_input_owner_v1(source, first.callee).unwrap(), !scalar);
            if scalar {
                ledger.borrowed_formal_actuals[&first.call].as_ref().unwrap().require_executable_v1().unwrap();
                assert!(ledger.callable_scalar_result_participant_v1(first.callee).unwrap());
            }
            ledger.borrowed_formal_actuals.insert(unrelated, Err("unrelated-static-refusal".into()));
            let call_site = first.call.clone();
            match mutation {
                0 => {}
                1 => { ledger.borrowed_formal_actuals.remove(&sibling); }
                2 => { ledger.borrowed_formal_actuals.insert(sibling, Err("original-callee-sibling-refusal".into())); }
                _ => { ledger.original_instance_input_issuance = None; }
            }
            if scalar {
                let row = ledger.take_lexical_instance_call(call_site.owner(), call_site.site()).unwrap().unwrap();
                let actuals = ledger.borrowed_call_actuals_v1(&row);
                match mutation {
                    0 => assert!(std::ptr::eq(actuals.unwrap().unwrap(), ledger.borrowed_formal_actuals[&call_site].as_ref().unwrap().opaque_actuals.as_ref())),
                    1 => assert!(actuals.unwrap_err().contains("original-input-not-ready")),
                    2 => assert_eq!(actuals.unwrap_err(), "original-callee-sibling-refusal"),
                    _ => assert!(actuals.unwrap_err().contains("original-issuance-missing")),
                }
            }
        }
        let key = crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(
            hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::instance_box_method("Maker", "make", 1));
        let mut context = crate::mir::builder::CompilationContext::new();
        let entry_ledger = std::rc::Rc::clone(&package.ordinary_new_claim_ledger);
        let installed = package.prepare_install(&mut context).unwrap().commit();
        installed.begin_lowering(&context).unwrap().with_selected_lowering_input(&key, |input| {
            let loan = entry_ledger.borrowed_ordinary_entry_source_v1(&input);
            match mutation {
                0 => { let loan = loan.unwrap().unwrap(); assert_eq!(loan.incoming_targets().count(), 2); assert_eq!(loan.incoming().unwrap().len(), 2); }
                1 => assert!(loan.unwrap_err().contains("original-input-not-ready")),
                2 => assert_eq!(loan.unwrap_err(), "original-callee-sibling-refusal"),
                _ => assert!(loan.unwrap_err().contains("original-issuance-missing")),
            }
        }).unwrap();
    }
    }
}
