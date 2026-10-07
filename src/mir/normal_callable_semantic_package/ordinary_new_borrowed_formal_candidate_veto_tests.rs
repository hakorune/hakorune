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
fn real_mimalloc_incoming_domain_keeps_all_callers_and_unresolved_source_veto() {
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
            let heap_key = CanonicalSameModuleCallableKeyV1::instance_box_method("HakoAllocHeap", "allocate", 1);
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
                assert_eq!(ingress.candidate_input_inventory_for_test(input.owner()), (15, true));
                let rejection = ingress.source_incoming.project(&BTreeSet::from([input.owner()])).unwrap_err();
                let BorrowedIncomingDraftErrorV1::UnresolvedCaller(rejected_site) = rejection else {
                    panic!("expected original unresolved receiver call: {rejection:?}");
                };
                let rejected_slot = package.batch().declarations()
                    .find(|row| row.owner() == rejected_site.owner()).unwrap().batch_slot();
                package.batch().with_lowering_input(rejected_slot, |original| {
                    let (_, call) = original.function().method_calls()
                        .find(|(site, _)| *site == rejected_site.site()).unwrap();
                    assert_eq!(call.selector(), "allocate");
                    assert_eq!(call.arity(), 1);
                }).unwrap();
                // Exact rows and unresolved same-selector calls both participate.
                // Neither missing guard proof nor a source veto can be discarded.
                assert!(!ingress.candidate_integer_agreement(contract.parameters[0].binding));
                assert!(!ingress.formal_integer_agreement(contract.parameters[0].binding));
            }).unwrap();
        });
    }).unwrap().join().expect("real imported observation");
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
