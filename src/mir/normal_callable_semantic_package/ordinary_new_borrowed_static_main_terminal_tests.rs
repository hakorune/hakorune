//! Original Main terminal observation and checked tagged packet admission.
use super::*;
use crate::mir::ValueId;

fn closed_package(
    source: &str,
) -> crate::mir::normal_callable_semantic_package::VerifiedNormalCallableSemanticPackageV1 {
    try_closed_package(source).unwrap()
}

fn try_closed_package(
    source: &str,
) -> Result<
    crate::mir::normal_callable_semantic_package::VerifiedNormalCallableSemanticPackageV1,
    String,
> {
    use crate::mir::builder::NormalRootExecutionConsumerV1;
    let parsed = crate::parser::NyashParser::parse_normal_callable_program_with_build_config(
        source,
        crate::parser::ParserBuildConfig::default(),
    )
    .unwrap();
    let transformed = crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        crate::r#macro::transform_normal_callable_program_v1(parsed).unwrap()
    });
    let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) = transformed else {
        panic!("original source-backed program");
    };
    let catalog = crate::analysis::brand_program_declaration_catalog::issue_brand_program_declaration_catalog_v1(source.ast()).unwrap();
    let source = NormalRootExecutionConsumerV1::consume_once(source)
        .unwrap()
        .into_consumed_source();
    crate::mir::normal_callable_semantic_package::issue_normal_callable_semantic_package_with_selection_v1(
        &mut crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1::new(93).unwrap(),
        source, Some(&catalog), crate::mir::builder::LoopFactsPolicyFrameV1::from_environment(), &[],
        crate::mir::normal_callable_semantic_package::PackagePhysicalSelectionV1::ClosedAppExecutable,
    ).map_err(|error| format!("{error:?}"))
}

#[test]
fn main_qualified_terminal_retains_original_literal_and_checked_packet() {
    let text = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("lang/src/hako_alloc/memory/size_class_box.hako"),
    )
    .unwrap();
    let package = closed_package(&format!(
        "{text}\nstatic box Main {{ main() {{ return SizeClassBox.size_to_bin(9) }} }}"
    ));
    let ledger = &package.ordinary_new_claim_ledger;
    let ingress = ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    let owner = ingress.app_main_owner.expect("original AppMain co-seal");
    let original = ingress
        .source_incoming
        .static_observations()
        .values()
        .filter_map(|row| row.as_ref().ok())
        .find(|row| row.call_site().owner() == owner)
        .expect("Main incoming");
    assert!(ingress.is_static_terminal_source_v1(original));
    let main = crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::borrow_app_main_source_v1(
        package.batch(),
        package.catalog.catalog().source_backed_app_main(),
    )
    .unwrap()
    .unwrap();
    package.batch().with_lowering_input(main.batch_slot(), |input| {
        let claim = crate::mir::normal_callable_semantic_package::qualified_static_call_claim::claim_for_source_site_v1(
            &package.source_static_claims_for_test,
            Some(main.catalog_key()),
            &input,
            original.call_site(),
            &package.selected,
            &package.parameter_contracts,
            Some(&main),
        ).unwrap().expect("original Main result claim");
        assert!(claim.is_qualified_terminal(), "Main terminal source membership");
    }).unwrap();
    let candidates = [BorrowedCallActualCandidateV1 {
        ordinal: 0,
        site: original.argument_sites()[0].clone(),
        value: BorrowedCallActualValueV1::Integer(9),
    }];
    let source_only = prepare_static_source_actuals_v1(
        ingress,
        &package.parameter_contracts,
        original.call_site(),
        &candidates,
    )
    .unwrap()
    .unwrap();
    let BorrowedCallActualEvidencePhaseV1::SourceStatic(identity) = &source_only.phase else {
        panic!("preparation alone does not grant entry");
    };
    assert!(Rc::ptr_eq(&identity.source, original));
    assert_eq!(identity.candidates.as_ref(), candidates);
    assert!(source_only.require_executable_v1().is_err());
    let staged = ledger.borrowed_formal_actuals[original.call_site()]
        .as_ref()
        .unwrap();
    assert!(matches!(
        staged.phase,
        BorrowedCallActualEvidencePhaseV1::ExecutableStaticTagged(_)
    ));
    let selected = ledger
        .selected_static_local_source_v1(original.call_site())
        .unwrap()
        .unwrap();
    assert!(Rc::ptr_eq(&selected, original));
    let actuals = ledger
        .checked_static_tagged_packet_actuals_v1(original)
        .unwrap()
        .unwrap();
    assert!(matches!(
        actuals,
        [PreparedBorrowedFormalActualV1 {
            ordinal: 0,
            source: BorrowedFormalActualSourceV1::Integer(9),
            ..
        }]
    ));
    let observation = ledger
        .local_call_for_owner(owner, original.call_site().site())
        .unwrap_or_else(|| {
            panic!(
                "Main Call observation missing: {:?}",
                ledger.completion_for_owner(owner)
            )
        });
    assert!(observation.local_binding().is_none());
    assert!(
        matches!(observation.arguments(), [LocalCallArgumentV1::BorrowedActual { ordinal: 0, site }]
        if site == &original.argument_sites()[0])
    );
    assert!(ledger.completion_for_owner(owner).is_some());
    assert!(!ledger.root_instance_call_expected(owner));
    // The same original Return membership applies to conditional exits in a
    // canonical child caller; no Main identity or Direct row is fabricated.
    let child = format!("box RootAnchor {{}} {text} static box LayoutBox {{ class_size(i) {{ if i == 0 {{ return SizeClassBox.bin_size(4) }} if i == 1 {{ return SizeClassBox.bin_size(8) }} return -1 }} }} static box Main {{ main() {{ local anchor = new RootAnchor() local n = LayoutBox.class_size(0) return 0 }} }}");
    let package = closed_package(&child);
    let ledger = &package.ordinary_new_claim_ledger;
    let ingress = ledger.borrowed_formal_source.as_ref().unwrap().as_ref().unwrap();
    let originals: Vec<_> = ingress.source_incoming.static_observations().values()
        .filter_map(|row| row.as_ref().ok())
        .filter(|row| row.caller().owner() == "LayoutBox" && row.caller().name() == "class_size")
        .collect();
    assert_eq!(originals.len(), 2, "both original conditional Return Calls");
    let owner = originals[0].call_site().owner();
    assert_ne!(ingress.app_main_owner, Some(owner), "child keeps its own identity");
    assert_eq!(ledger.call_relations_for_owner(owner).len(), 2, "same canonical child Completion retains both original Calls");
    let mut literals = Vec::new();
    let mut missing_packets = Vec::new();
    for original in originals {
        assert!(ingress.is_static_terminal_source_v1(original));
        let slot = package.batch().declarations().find(|row| row.owner() == owner).unwrap().batch_slot();
        let qualified = package.batch().with_lowering_input(slot, |input| {
            crate::mir::normal_callable_semantic_package::qualified_static_call_claim::claim_for_source_site_v1(
                &package.source_static_claims_for_test, Some(original.caller()), &input,
                original.call_site(), &package.selected, &package.parameter_contracts, None,
            ).map(|claim| claim.is_some_and(|claim| claim.is_qualified_terminal()))
        }).unwrap();
        assert!(ledger.checked_completed_static_scalar_cohort_v1(ingress, original.callee_owner()).unwrap().is_some(),
            "whole original scalar cohort: {:?}", ingress.source_incoming.exact_rows()
                .filter(|row| row.callee == original.callee_owner())
                .map(|row| (&row.source, ledger.borrowed_formal_actuals.get(&row.call)))
                .collect::<Vec<_>>());
        assert!(ledger.local_call_for_owner(owner, original.call_site().site()).is_some(), "original ExpressionValue observation retained");
        let Some(selected) = ledger.selected_static_local_source_v1(original.call_site()).unwrap() else {
            missing_packets.push(format!("site={:?} qualified={qualified:?} role={:?} flow={:?} terminal_count={} actual={:?}",
                original.call_site(), package.selected.role_for_batch_slot(slot),
                ledger.completion_for_owner(owner).and_then(|row| row.cleanup().root_flow()),
                ledger.call_relations_for_owner(owner).len(), ledger.borrowed_formal_actuals.get(original.call_site())));
            continue;
        };
        assert!(Rc::ptr_eq(&selected, original));
        let observation = ledger.local_call_for_owner(owner, original.call_site().site()).unwrap();
        assert!(observation.local_binding().is_none());
        assert!(matches!(observation.arguments(), [LocalCallArgumentV1::BorrowedActual { ordinal: 0, site }]
            if site == &original.argument_sites()[0]));
        let terminal = ledger.call_relations_for_owner(owner).into_iter()
            .find(|row| row.call_site() == original.call_site().site()).unwrap();
        assert!(terminal.arguments().is_empty());
        assert!(ledger.completion_for_owner(owner).unwrap().explicit_sites().contains(terminal.return_site()));
        let [actual] = ledger.borrowed_static_packet_actuals_v1(original).unwrap().unwrap() else { panic!("one original actual") };
        let BorrowedFormalActualSourceV1::Integer(value) = actual.source else { panic!("original literal source") };
        literals.push(value);
    }
    literals.sort();
    let _ring0 = crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DERIVE", "", || {
        let parsed = crate::parser::NyashParser::parse_normal_callable_program_with_build_config(&child, crate::parser::ParserBuildConfig::default()).unwrap();
        let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) = crate::r#macro::transform_normal_callable_program_v1(parsed).unwrap() else { panic!("original source lost") };
        let outcome = crate::mir::MirCompiler::with_options(false).compile_normal_with_published(
            crate::mir::NormalCompileRequestV1::for_mir_mode_callable_source(source, None, Default::default()).for_closed_app_executable(),
            |view, verification| {
                verification.as_ref().map_err(|errors| format!("{errors:?}"))?;
                let key = crate::mir::builder::CanonicalSameModuleCallableKeyV1::static_box_method("LayoutBox", "class_size", 1);
                let function = view.definition(&key).ok_or_else(|| "original child missing".to_owned())?;
                assert_eq!(function.blocks.values().flat_map(|block| block.all_instructions()).filter(|row|
                    matches!(row, crate::mir::MirInstruction::Invoke { operation: crate::mir::instruction::InvokeOperation::Call { .. }, .. })).count(), 2);
                let input = view.issue_lifecycle_physical_abi_input()?;
                crate::mir::emit_lifecycle_physical_abi_json(&input)?;
                Ok(())
            });
        match outcome {
            Ok(crate::mir::NormalPublishedCompileOutcome::Consumed(())) => {}
            Ok(_) => panic!("canonical child publication missing; source preparation: {missing_packets:?}"),
            Err(error) => panic!("original child compiler failed: {error}; source preparation: {missing_packets:?}"),
        }
    });
    assert!(missing_packets.is_empty(), "source packet obligations: {missing_packets:?}");
    assert_eq!(literals, [4, 8]);

}

#[test]
fn main_terminal_checked_packet_rejects_actual_and_order_drift() {
    let text = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("lang/src/hako_alloc/memory/size_class_box.hako"),
    )
    .unwrap();
    for mutation in 0..5 {
        let mut package = closed_package(&format!(
            "{text}\nstatic box Main {{ main() {{ return SizeClassBox.size_to_bin(9) }} }}"
        ));
        let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
        let ingress = ledger
            .borrowed_formal_source
            .as_ref()
            .unwrap()
            .as_ref()
            .unwrap();
        let owner = ingress.app_main_owner.unwrap();
        let original = ingress
            .source_incoming
            .static_observations()
            .values()
            .filter_map(|row| row.as_ref().ok())
            .find(|row| row.call_site().owner() == owner)
            .unwrap()
            .clone();
        assert!(ledger
            .checked_static_tagged_packet_actuals_v1(&original)
            .unwrap()
            .is_some());
        let rows = ledger
            .borrowed_formal_actuals
            .get_mut(original.call_site())
            .unwrap()
            .as_mut()
            .unwrap();
        match mutation {
            0 => rows.opaque_actuals[0].source = BorrowedFormalActualSourceV1::Integer(10),
            1 => rows.opaque_actuals[0].ordinal = 1,
            2 => rows.opaque_actuals[0].site = original.call_site().site().clone(),
            3 => {
                rows.ordered_arguments[0] = LocalCallArgumentV1::BorrowedActual {
                    ordinal: 1,
                    site: original.argument_sites()[0].clone(),
                }
            }
            4 => {
                rows.opaque_actuals[0].formal =
                    BindingRefV1::new(owner, rows.opaque_actuals[0].formal.binding())
            }
            _ => unreachable!(),
        }
        assert!(
            ledger
                .checked_static_tagged_packet_actuals_v1(&original)
                .is_err(),
            "mutation {mutation} must not reach executable packet"
        );
    }
}

#[test]
fn main_terminal_finish_rejects_foreign_verified_entry_brand() {
    let text = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("lang/src/hako_alloc/memory/size_class_box.hako"),
    )
    .unwrap();
    let source =
        format!("{text}\nstatic box Main {{ main() {{ return SizeClassBox.size_to_bin(9) }} }}");
    let mut original = closed_package(&source);
    let foreign = closed_package(&source);
    assert!(!original
        .selected
        .catalog_brand()
        .is_same(foreign.selected.catalog_brand()));
    let ledger = Rc::get_mut(&mut original.ordinary_new_claim_ledger).unwrap();
    let error = ledger
        .finish_static_tagged_input_actuals_v1(
            &foreign.selected,
            &original.parameter_contracts,
            &original.physical_signature,
            &original.result_contracts,
        )
        .unwrap_err();
    assert!(
        error.contains("static-tagged/product-source-drift"),
        "{error}"
    );
}

#[test]
fn main_terminal_finish_does_not_validate_unrequested_entry() {
    let text = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("lang/src/hako_alloc/memory/size_class_box.hako"),
    )
    .unwrap();
    let mut original = package(&format!(
        "{text}\nstatic box Main {{ main() {{ return 0 }} }}"
    ));
    let ledger = Rc::get_mut(&mut original.ordinary_new_claim_ledger).unwrap();
    let before = ledger.borrowed_formal_actuals.clone();
    let selected_before = ledger.borrowed_static_source_sites.clone();
    let (owner, product) = ledger
        .preissued_static_i64_products
        .borrow_mut()
        .pop_first()
        .expect("existing unrelated verified entry");
    let other_owner = original
        .parameter_contracts
        .iter()
        .find(|row| row.owner != owner)
        .unwrap()
        .owner;
    ledger
        .preissued_static_i64_products
        .borrow_mut()
        .insert(other_owner, product);
    ledger
        .finish_static_tagged_input_actuals_v1(
            &original.selected,
            &original.parameter_contracts,
            &original.physical_signature,
            &original.result_contracts,
        )
        .unwrap();
    assert_eq!(ledger.borrowed_formal_actuals, before);
    assert_eq!(ledger.borrowed_static_source_sites, selected_before);
}

#[test]
fn main_terminal_packet_denies_restored_source_only_actual() {
    let text = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("lang/src/hako_alloc/memory/size_class_box.hako"),
    )
    .unwrap();
    let mut package = closed_package(&format!(
        "{text}\nstatic box Main {{ main() {{ return SizeClassBox.size_to_bin(9) }} }}"
    ));
    let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    let ingress = ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    let original = ingress
        .source_incoming
        .static_observations()
        .values()
        .filter_map(|row| row.as_ref().ok())
        .find(|row| ingress.is_static_terminal_source_v1(row))
        .unwrap()
        .clone();
    let pending = prepare_static_source_actuals_v1(
        ingress,
        &package.parameter_contracts,
        original.call_site(),
        &[BorrowedCallActualCandidateV1 {
            ordinal: 0,
            site: original.argument_sites()[0].clone(),
            value: BorrowedCallActualValueV1::Integer(9),
        }],
    )
    .unwrap()
    .unwrap();
    ledger
        .borrowed_formal_actuals
        .insert(original.call_site().clone(), Ok(pending));
    assert!(ledger
        .checked_static_tagged_packet_actuals_v1(&original)
        .unwrap()
        .is_none());
    assert!(ledger
        .selected_static_local_source_v1(original.call_site())
        .unwrap()
        .is_none());
    assert!(!ledger
        .borrowed_static_packet_actuals_v1(&original)
        .is_ok_and(|row| row.is_some()));
}

#[test]
fn main_static_terminal_membership_rejects_assignment_and_declared_object() {
    use crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::borrow_app_main_source_v1;
    use crate::mir::normal_callable_semantic_package::qualified_static_call_claim::claim_for_source_site_v1;
    let fixtures = [
        "static box Helpers { pick(p) { return 7 } } static box Main { main() { local x = 0 x = Helpers.pick(9) return x } }",
        "box Token { birth() { } } static box Helpers { pick(p: Token): i64 { return 7 } } static box Main { main() { return Helpers.pick(9) } }",
    ];
    for text in fixtures {
        let package = package(text);
        let main = borrow_app_main_source_v1(
            package.batch(),
            package.catalog.catalog().source_backed_app_main(),
        )
        .unwrap()
        .unwrap();
        let ingress = package
            .ordinary_new_claim_ledger
            .borrowed_formal_source
            .as_ref()
            .unwrap()
            .as_ref()
            .unwrap();
        let owner = ingress.app_main_owner.unwrap();
        let original = ingress
            .source_incoming
            .static_observations()
            .values()
            .filter_map(|row| row.as_ref().ok())
            .find(|row| row.call_site().owner() == owner)
            .unwrap();
        assert!(!ingress.is_static_terminal_source_v1(original));
        package
            .batch()
            .with_lowering_input(main.batch_slot(), |input| {
                let claim = claim_for_source_site_v1(
                    &package.source_static_claims_for_test,
                    Some(main.catalog_key()),
                    &input,
                    original.call_site(),
                    &package.selected,
                    &package.parameter_contracts,
                    Some(&main),
                )
                .unwrap()
                .unwrap();
                assert!(!claim.is_qualified_terminal());
            })
            .unwrap();
    }
}

#[test]
fn main_terminal_tagged_entry_keeps_canonical_actuals_and_whole_sibling_veto() {
    let text = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("lang/src/hako_alloc/memory/size_class_box.hako"),
    )
    .unwrap();
    for sibling in 0..3 {
        let extra = match sibling {
            0 => "",
            1 => "static box Heap { lookup(size) { if size <= 0 { return 0 } local k = SizeClassBox.size_to_bin(size) return 0 } }",
            _ => "box Heap { lookup(size) { local k = SizeClassBox.size_to_bin(size) return 0 } }",
        };
        let main = match sibling {
            0 => "return SizeClassBox.size_to_bin(9)",
            1 => "local k = Heap.lookup(7) return SizeClassBox.size_to_bin(9)",
            _ => "local heap = new Heap() local k = heap.lookup(7) return SizeClassBox.size_to_bin(9)",
        };
        let mut package = closed_package(&format!(
            "{text}\n{extra}\nstatic box Main {{ main() {{ {main} }} }}"
        ));
        let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
        let ingress = ledger
            .borrowed_formal_source
            .as_ref()
            .unwrap()
            .as_ref()
            .unwrap();
        let original = ingress
            .source_incoming
            .static_observations()
            .values()
            .filter_map(|row| row.as_ref().ok())
            .find(|row| ingress.is_static_terminal_source_v1(row))
            .unwrap()
            .clone();
        let owner = original.callee_owner();
        let formal = original.parameters()[0].binding;
        let foreign = ingress
            .source_only_definitions
            .values()
            .chain(ingress.definitions.values())
            .flat_map(|row| row.origins.keys())
            .copied()
            .find(|binding| binding.owner() != owner)
            .unwrap();
        assert!(!ledger.has_borrowed_ordinary_entry_v1(owner));
        for bad in [
            Box::new([(1, formal, ValueId(0))]),
            Box::new([(0, foreign, ValueId(0))]),
        ] {
            assert!(ledger
                .validate_borrowed_ordinary_entry_values_v1(owner, &Ok(bad))
                .is_err());
            assert!(!ledger.has_borrowed_ordinary_entry_v1(owner));
        }
        let rows = ledger
            .checked_completed_static_tagged_entry_v1(ingress, owner)
            .unwrap()
            .expect("finished whole tagged cohort");
        assert_eq!(rows.len(), if sibling > 0 { 2 } else { 1 });
        let sibling_site = rows
            .iter()
            .find(|row| row.call != *original.call_site())
            .map(|row| row.call.clone());
        if let Some(site) = sibling_site {
            assert!(matches!(
                ledger.borrowed_formal_actuals[&site]
                    .as_ref()
                    .unwrap()
                    .phase,
                BorrowedCallActualEvidencePhaseV1::SourceStatic(_)
            ));
            assert!(ledger.borrowed_formal_actuals[&site]
                .as_ref()
                .unwrap()
                .require_executable_v1()
                .is_err());
            let saved = ledger.borrowed_formal_actuals.remove(&site).unwrap();
            assert!(
                matches!(
                    ledger.borrowed_formal_actuals[original.call_site()]
                        .as_ref()
                        .unwrap()
                        .phase,
                    BorrowedCallActualEvidencePhaseV1::ExecutableStaticTagged(_)
                ),
                "Main's finished literal row remains while only Heap's row is absent"
            );
            let ingress = ledger
                .borrowed_formal_source
                .as_ref()
                .unwrap()
                .as_ref()
                .unwrap();
            assert!(ledger
                .checked_completed_static_tagged_entry_v1(ingress, owner)
                .unwrap_err()
                .contains("static-tagged/cohort-unfinished"));
            assert!(ledger
                .validate_borrowed_ordinary_entry_values_v1(
                    owner,
                    &Ok(Box::new([(0, formal, ValueId(0))]))
                )
                .is_err());
            ledger.borrowed_formal_actuals.insert(site, saved);
        }
        let values: Box<[_]> = Box::new([(0, formal, ValueId(0))]);
        ledger
            .record_borrowed_ordinary_entry_values_v1(owner, Ok(values.clone()))
            .unwrap();
        assert!(ledger
            .record_borrowed_ordinary_entry_values_v1(owner, Ok(values.clone()))
            .unwrap_err()
            .contains("duplicate-entry-values"));
        assert_eq!(
            ledger.borrowed_ordinary_entry_values_v1(owner).unwrap(),
            values
        );
        let entry = ledger
            .finalized_borrowed_ordinary_entry_source_v1(owner)
            .unwrap();
        assert_eq!(entry.formals(), [(0, formal)]);
        assert!(entry
            .incoming()
            .unwrap_err()
            .contains("canonical-packet-actual-required"));
        let incoming = entry
            .incoming_targets()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        assert_eq!(incoming.len(), if sibling > 0 { 2 } else { 1 });
    }
}
