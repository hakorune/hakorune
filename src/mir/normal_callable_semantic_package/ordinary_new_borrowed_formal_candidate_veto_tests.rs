//! A source candidate must retain scanner vetoes beside otherwise good input.
use super::*;

#[test]
fn candidate_integer_agreement_refuses_good_row_plus_unresolved_caller_veto() {
    let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(
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
    let (objects, integers, _) = prepare_borrowed_formal_views_v1(
        package.batch(),
        &package.selected,
        &empty_loans,
        &package.instance_constructors,
        &package.ordinary_new_claim_ledger.callable_result_classes,
        &BTreeMap::new(),
        &package.parameter_contracts,
        &definitions,
        &BTreeSet::new(),
        &inventory,
        &BTreeMap::new(),
    )
    .unwrap();
    assert!(objects.is_empty());
    assert!(integers.is_empty());
    assert_eq!(
        inventory
            .project(&definitions.keys().copied().collect())
            .unwrap_err(),
        BorrowedIncomingDraftErrorV1::UnresolvedCaller(missing)
    );
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
            let package = crate::mir::normal_callable_semantic_package::issue_normal_callable_semantic_package_with_brand_catalog_and_loop_policy_v1(
                &mut FunctionSemanticResolverSessionV1::new(93).unwrap(), consumed, Some(&catalog),
                crate::mir::builder::LoopFactsPolicyFrameV1::from_environment(), &imports,
            ).expect("one original factory");
            let claims = &package.source_static_claims_for_test;
            // Inspect the original excluded draft before attributing the
            // executable SourceOnly stop to a receiver or packet consumer.
            for (class, method, checked_return) in [
                ("LayoutBox", "class_id", false),
                ("SizeClassBox", "good_size", false),
                ("SizeClassBox", "size_to_bin", false),
                ("SizeClassBox", "normalize_size", true),
            ] {
                let key = CanonicalSameModuleCallableKeyV1::static_box_method(class, method, 1);
                let slot = package.selected.batch_slot(
                    &SelectedNormalCallableKeyV1::Cataloged(key.clone()),
                ).expect("original static chain declaration");
                let contract = package.parameter_contracts.iter()
                    .find(|row| row.batch_slot == slot).unwrap();
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
            let bin_slot = package.selected.batch_slot(&SelectedNormalCallableKeyV1::Cataloged(bin_key.clone())).unwrap();
            let bin_contract = package.parameter_contracts.iter().find(|row| row.batch_slot == bin_slot).unwrap();
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
                ("SizeClassBox", "accepts"),
            ] {
                let key = CanonicalSameModuleCallableKeyV1::static_box_method(class, method, 1);
                let slot = package.selected.batch_slot(&SelectedNormalCallableKeyV1::Cataloged(key)).unwrap();
                let formal = package.parameter_contracts.iter().find(|row| row.batch_slot == slot).unwrap().parameters[0].binding;
                if method != "accepts" {
                    assert!(!source.candidate_integer_agreement(formal));
                }
                assert!(source.checked_static_input(formal), "original checked/forward chain {class}.{method}");
            }
            let good_size = CanonicalSameModuleCallableKeyV1::static_box_method("SizeClassBox", "good_size", 1);
            let good_slot = package.selected.batch_slot(&SelectedNormalCallableKeyV1::Cataloged(good_size.clone())).unwrap();
            let good_contract = package.parameter_contracts.iter().find(|row| row.batch_slot == good_slot).unwrap();
            let bin_target = CanonicalSameModuleCallableKeyV1::static_box_method("SizeClassBox", "size_to_bin", 1);
            let bin_slot = package.selected.batch_slot(&SelectedNormalCallableKeyV1::Cataloged(bin_target.clone())).unwrap();
            let bin_contract = package.parameter_contracts.iter().find(|row| row.batch_slot == bin_slot).unwrap();
            package.batch().with_lowering_input(bin_slot, |input| {
                let loop_site = input.function().loop_sites().next().expect("original size_to_bin loop");
                let mut header_loan = None;
                let mut body_loan = None;
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
                    {
                        let retained = package.ordinary_new_claim_ledger.loop_static_source_loans.borrow();
                        let retained = retained.get(&(loop_site.clone(), owned.clone())).unwrap().as_ref().unwrap();
                        assert!(std::rc::Rc::ptr_eq(retained.original(), original));
                    }
                    let moved = package.ordinary_new_claim_ledger
                        .take_loop_static_source_call_loan_v1(loop_site, &owned)
                        .unwrap().unwrap();
                    assert!(std::rc::Rc::ptr_eq(moved.original(), original));
                    assert!(package.ordinary_new_claim_ledger
                        .take_loop_static_source_call_loan_v1(loop_site, &owned).is_none());
                    if arity == 0 { header_loan = Some(moved); } else { body_loan = Some(moved); }
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
                let moved = package.ordinary_new_claim_ledger
                    .take_loop_entry_static_i64_source_loan_v1(loop_site, initializer.declaration_site())
                    .unwrap().unwrap();
                assert!(std::rc::Rc::ptr_eq(entry.original(), moved.original()));
                assert!(package.ordinary_new_claim_ledger
                    .take_loop_entry_static_i64_source_loan_v1(loop_site, initializer.declaration_site()).is_none());
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
                let tail = package.ordinary_new_claim_ledger
                    .take_loop_tail_static_i64_source_loan_v1(loop_site)
                    .unwrap().unwrap();
                assert_eq!(tail.loop_site(), loop_site);
                assert!(tail.original().call_site().site() == site_for("huge_bin").site());
                assert!(package.ordinary_new_claim_ledger
                    .take_loop_tail_static_i64_source_loan_v1(loop_site).is_none());
                let product = crate::mir::builder::produce_static_i64_loop_semantic_v2(
                    input, loop_site, completion, moved,
                    header_loan.expect("header source"), body_loan.expect("body source"),
                    tail,
                ).expect("original source-bound V2 Recipe and JoinSig");
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
                assert!(source.incoming.iter().all(|row| row.call != owned));
                assert!(package.ordinary_new_claim_ledger.selected_static_local_source_v1(&owned).unwrap().is_none());
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
                assert!(projected.is_some(), "checked forwarding is source-only");
            }).unwrap();
            let usize_key = CanonicalSameModuleCallableKeyV1::static_box_method("SizeClassBox", "bin_size_usize", 1);
            let usize_slot = package.selected.batch_slot(&SelectedNormalCallableKeyV1::Cataloged(usize_key)).unwrap();
            package.batch().with_lowering_input(usize_slot, |input| {
                let (site, call) = input.function().method_calls()
                    .find(|(_, call)| call.selector() == "bin_size").expect("original direct CurrentOwner return");
                let owned = crate::mir::resolved_semantics::OwnedExprSiteV1::new(input.owner(), site.clone());
                let original = source.source_incoming.static_observations()[&owned].as_ref().unwrap();
                assert_eq!(original.argument_sites(), &[call.arguments()[0].site().clone()]);
                let observed = package.ordinary_new_claim_ledger
                    .local_call_for_owner(input.owner(), site)
                    .expect("original direct return retains ordered Static actual");
                assert_eq!(observed.arguments().len(), 1);
                assert!(observed.local_binding().is_none());
                assert!(package.ordinary_new_claim_ledger.completion_for_owner(input.owner()).is_some());
            }).unwrap();
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
        });
    }).unwrap().join().expect("real imported observation");
}

#[test]
fn real_mimalloc_static_loop_route_uses_one_source_bound_v2_product() {
    std::thread::Builder::new().name("mimalloc-static-loop-route".into())
        .stack_size(32 * 1024 * 1024).spawn(|| {
        let env_updates: Vec<_> = crate::test_support::JOINIR_DEFAULT_MODE.into_iter().chain([
            ("NYASH_ALLOW_USING_FILE", Some("1")), ("NYASH_ENABLE_USING", Some("1")),
            ("NYASH_OPERATOR_BOX_ALL", Some("0")), ("NYASH_MACRO_DISABLE", Some("1")),
        ]).collect();
        crate::test_support::with_env_vars(&env_updates, || {
            use crate::mir::builder::{NormalRootExecutionConsumerV1, SelectedNormalCallableKeyV1};
            use crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1;
            use crate::runner::modes::common_util::normal_callable::{
                materialize_normal_callable_program_with_identity_and_lineage_v1,
                NormalCallableMaterializationOutcomeV1,
            };
            let filename = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("apps/mimalloc-lite/main.hako");
            let code = std::fs::read_to_string(&filename).unwrap();
            let runner = crate::runner::NyashRunner::new(Default::default());
            let prepared = crate::runner::modes::common_util::source_hint::prepare_normal_source_with_imports(
                &runner, filename.to_str().unwrap(), &code,
            ).unwrap();
            let imports: Vec<_> = prepared.imports.into_iter().collect();
            let transformed = materialize_normal_callable_program_with_identity_and_lineage_v1(
                prepared.code, runner.parser_build_config(), filename.to_string_lossy().into_owned(), prepared.lineage,
            ).unwrap();
            let NormalCallableMaterializationOutcomeV1::SourceBacked(source) = transformed else { panic!("source-backed") };
            let catalog = crate::analysis::brand_program_declaration_catalog::issue_brand_program_declaration_catalog_v1(source.ast()).unwrap();
            let consumed = NormalRootExecutionConsumerV1::consume_once(source).unwrap().into_consumed_source();
            let package = crate::mir::normal_callable_semantic_package::issue_normal_callable_semantic_package_with_brand_catalog_and_loop_policy_v1(
                &mut FunctionSemanticResolverSessionV1::new(94).unwrap(), consumed, Some(&catalog),
                crate::mir::builder::LoopFactsPolicyFrameV1::from_environment(), &imports,
            ).unwrap();
            let key = CanonicalSameModuleCallableKeyV1::static_box_method("SizeClassBox", "size_to_bin", 1);
            let slot = package.selected.batch_slot(&SelectedNormalCallableKeyV1::Cataloged(key)).unwrap();
            package.batch().with_lowering_input(slot, |input| {
                let loop_site = input.function().loop_sites().next().unwrap();
                let claims = &package.ordinary_new_claim_ledger;
                assert!(claims.expects_loop_static_source_loan_v1(input.owner(), loop_site));
                let first = crate::mir::builder::stop_after_selected_semantic_product(input, claims, loop_site).unwrap_err();
                assert!(first.contains("static-i64-v2/physical-unavailable"), "{first}");
                let second = crate::mir::builder::stop_after_selected_semantic_product(input, claims, loop_site).unwrap_err();
                assert!(second.contains("static-i64-v2/source-unavailable"), "{second}");
            }).unwrap();
            let other = CanonicalSameModuleCallableKeyV1::static_box_method("SizeClassBox", "bin_size", 1);
            let other_slot = package.selected.batch_slot(&SelectedNormalCallableKeyV1::Cataloged(other)).unwrap();
            package.batch().with_lowering_input(other_slot, |input| {
                let loop_site = input.function().loop_sites().next().unwrap();
                assert!(!package.ordinary_new_claim_ledger.expects_loop_static_source_loan_v1(input.owner(), loop_site));
                assert!(crate::mir::builder::stop_after_selected_semantic_product(
                    input, &package.ordinary_new_claim_ledger, loop_site,
                ).is_ok(), "unselected Loop keeps its existing route");
            }).unwrap();
        });
    }).unwrap().join().expect("original selected Loop route");
}

#[test]
fn real_mimalloc_static_loop_v2_rejects_source_and_home_mutations() {
    std::thread::Builder::new().name("mimalloc-static-loop-negatives".into())
        .stack_size(32 * 1024 * 1024).spawn(|| {
        let env_updates: Vec<_> = crate::test_support::JOINIR_DEFAULT_MODE.into_iter().chain([
            ("NYASH_ALLOW_USING_FILE", Some("1")), ("NYASH_ENABLE_USING", Some("1")),
            ("NYASH_OPERATOR_BOX_ALL", Some("0")), ("NYASH_MACRO_DISABLE", Some("1")),
        ]).collect();
        crate::test_support::with_env_vars(&env_updates, || {
            use crate::mir::builder::{NormalRootExecutionConsumerV1, SelectedNormalCallableKeyV1};
            use crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1;
            use crate::runner::modes::common_util::normal_callable::{
                materialize_normal_callable_program_with_identity_and_lineage_v1,
                NormalCallableMaterializationOutcomeV1,
            };
            let filename = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("apps/mimalloc-lite/main.hako");
            let code = std::fs::read_to_string(&filename).unwrap();
            for (case, (before, after)) in [
                ("return bin\n      }\n      bin = bin + 1", "local skipped = bin\n      }\n      bin = bin + 1"),
                ("bin = bin + 1\n    }\n    return me.huge_bin()", "bin = true\n    }\n    return me.huge_bin()"),
                ("return me.huge_bin()\n  }", "return 73\n  }"),
                ("return words * me.word_size()", "print(0)\n    return words * me.word_size()"),
            ].into_iter().enumerate() {
                let runner = crate::runner::NyashRunner::new(Default::default());
                let prepared = crate::runner::modes::common_util::source_hint::prepare_normal_source_with_imports(
                    &runner, filename.to_str().unwrap(), &code,
                ).unwrap();
                assert_eq!(prepared.code.matches(before).count(), 1, "one original Loop source");
                let imports: Vec<_> = prepared.imports.into_iter().collect();
                let changed = prepared.code.replacen(before, after, 1);
                let transformed = materialize_normal_callable_program_with_identity_and_lineage_v1(
                    changed, runner.parser_build_config(), filename.to_string_lossy().into_owned(), prepared.lineage,
                ).unwrap();
                let NormalCallableMaterializationOutcomeV1::SourceBacked(source) = transformed else { panic!("source-backed") };
                let catalog = crate::analysis::brand_program_declaration_catalog::issue_brand_program_declaration_catalog_v1(source.ast()).unwrap();
                let consumed = NormalRootExecutionConsumerV1::consume_once(source).unwrap().into_consumed_source();
                let package = crate::mir::normal_callable_semantic_package::issue_normal_callable_semantic_package_with_brand_catalog_and_loop_policy_v1(
                    &mut FunctionSemanticResolverSessionV1::new(95).unwrap(), consumed, Some(&catalog),
                    crate::mir::builder::LoopFactsPolicyFrameV1::from_environment(), &imports,
                );
                if case == 1 {
                    let error = match package {
                        Ok(_) => panic!("Bool update must stop in package"),
                        Err(error) => error,
                    };
                    assert!(format!("{error:?}").contains("source-actual-unavailable"), "{error:?}");
                    continue;
                }
                let package = package.expect("missing inner return reaches V2 source producer");
                let key = CanonicalSameModuleCallableKeyV1::static_box_method("SizeClassBox", "size_to_bin", 1);
                let slot = package.selected.batch_slot(&SelectedNormalCallableKeyV1::Cataloged(key)).unwrap();
                package.batch().with_lowering_input(slot, |input| {
                    let loop_site = input.function().loop_sites().next().unwrap();
                    if case == 3 {
                        let home = package.ordinary_new_claim_ledger
                            .take_loop_static_home_neutral_v1(loop_site)
                            .expect("selected Loop has Home proof attempt")
                            .unwrap_err();
                        assert!(home.contains("static-home-effect-unavailable"), "{home}");
                        return;
                    }
                    let error = crate::mir::builder::stop_after_selected_semantic_product(
                        input, &package.ordinary_new_claim_ledger, loop_site,
                    ).unwrap_err();
                    let expected = match case {
                        2 => "static-i64-v2/source-unavailable",
                        _ => "static-i64-v2/source]",
                    };
                    assert!(error.contains(expected), "{before}: {error}");
                }).unwrap();
            }
        });
    }).unwrap().join().expect("changed original Loop rejects");
}

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
fn passive_stored_dispatch_excludes_only_exact_different_targets() {
    use super::super::source::{
        CallTargetReferenceV1, PreparedSourceCallNeedV1, PreparedSourceNeedsV1,
        StoredReceiverSourceV1,
    };
    use crate::mir::resolved_semantics::BodyExpressionShapeV1;
    let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(
        "box Item { value: i64 birth() { me.value = 5 } }
        box Leaf { flag: i64 birth() { me.flag = 0 }
            read(p: Item) { if p == null { return 7 } return p.value } }
        box Parent { left: Leaf right: Leaf
            birth() { me.left = new Leaf() me.right = new Leaf() }
            first(p: Item) { return me.left.read(p) }
            second(p: Item) { return me.right.read(p) } }
        box Other { read(p) { return 0 } }
        static box Main { main() { local parent = new Parent() local item = new Item()
            local other = new Other() local z = other.read(7)
            local ignored = parent.second(item) return parent.first(item) } }").unwrap();
    let source = package
        .ordinary_new_claim_ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    let mut passive = Vec::new();
    let mut calls = BTreeMap::new();
    let mut leaf_owner = None;
    for row in source.incoming.iter() {
        let target = row.source.require_instance().unwrap();
        if let Some((parent_binding, parent_site, _, _)) = target.stored_receiver() {
            leaf_owner = Some(target.callee_owner());
            let caller = package
                .parameter_contracts
                .iter()
                .find(|c| c.owner == row.call.owner())
                .unwrap();
            let crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(caller_key) = package
                .selected
                .key_for_batch_slot(caller.batch_slot)
                .unwrap()
            else {
                panic!("instance caller")
            };
            let field_name = package
                .batch()
                .with_lowering_input(caller.batch_slot, |input| {
                    let Some(BodyExpressionShapeV1::FieldAccess { field, .. }) = input
                        .body_shape()
                        .unwrap()
                        .expression_shape(target.receiver_site())
                    else {
                        panic!("field source")
                    };
                    field.clone()
                })
                .unwrap();
            passive.push((
                CallTargetReferenceV1::from_target(target),
                StoredReceiverSourceV1 {
                    parent_binding,
                    parent_site: parent_site.clone(),
                    parent_class: caller_key.owner().into(),
                    field_name,
                    child_class: target.target().owner().into(),
                },
            ));
        } else {
            calls.insert(row.call.clone(), target);
        }
    }
    assert_eq!(passive.len(), 2);
    let other_key = hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::instance_box_method(
        "Other", "read", 1,
    );
    let other = package
        .parameter_contracts
        .iter()
        .find(|c| {
            matches!(package.selected.key_for_batch_slot(c.batch_slot),
            Some(crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(k)) if k == &other_key)
        })
        .unwrap()
        .owner;
    let scope = package
        .batch()
        .declarations()
        .map(|row| row.owner())
        .collect();
    let scan = |needs: Option<&PreparedSourceNeedsV1>| {
        inventory_borrowed_incoming_with_stored_dispatch_v1(
            package.batch(),
            &package.selected,
            &source.definitions,
            &package.parameter_contracts,
            &calls,
            &scope,
            None,
            needs,
        )
    };
    let prepared =
        |pairs: Vec<(CallTargetReferenceV1, StoredReceiverSourceV1)>| -> PreparedSourceNeedsV1 {
            Ok(pairs
                .into_iter()
                .map(|(reference, receiver)| {
                    Ok(Some(PreparedSourceCallNeedV1::Stored {
                        reference,
                        receiver,
                        result_requirement: super::super::LexicalCallSourceResultRequirementV1::ExistingBorrowedResult,
                    }))
                })
                .collect())
        };
    let missing = scan(None).unwrap();
    assert!(matches!(
        missing.project(&BTreeSet::from([other])),
        Err(BorrowedIncomingDraftErrorV1::UnresolvedCaller(_))
    ));
    let original = prepared(passive.clone());
    let checked = scan(Some(&original)).unwrap();
    let rows = checked.project(&BTreeSet::from([other])).unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].source.target(), &other_key);
    assert!(matches!(
        checked.project(&BTreeSet::from([leaf_owner.unwrap()])),
        Err(BorrowedIncomingDraftErrorV1::UnresolvedCaller(_))
    ));
    for change in 0..7 {
        let mut pairs = passive.clone();
        match change {
            0 => pairs.push(pairs[0].clone()),
            1 => pairs[0].0.receiver_site = pairs[0].0.argument_sites[0].clone(),
            2 => pairs[0].1.field_name = "missing".into(),
            3 => pairs[0].0.target_batch_slot += 1,
            4 => pairs[0].0.argument_sites[0] = pairs[0].0.receiver_site.clone(),
            5 => pairs[0].1.child_class = "Other".into(),
            _ => pairs[0].1.parent_site = pairs[0].0.receiver_site.clone(),
        }
        let changed = prepared(pairs);
        assert!(scan(Some(&changed)).is_err(), "identity change {change}");
    }
}
