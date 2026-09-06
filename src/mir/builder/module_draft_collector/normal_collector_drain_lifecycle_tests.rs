use super::*;
use crate::mir::builder::module_draft_collector::CompletedDraftSignatureViewV1;
use crate::mir::callable_result_representation::VerifiedStaticCallResultPublicationOwnerV1;
use crate::mir::resolved_semantics::{SourceExprSiteV1, SourceNodeSiteV1, SourcePathSegmentV1};
use crate::mir::{BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirType};

fn brand() -> ModuleInvocationBrandV1 {
    ModuleInvocationBrandV1::test_with_ordinal(701)
}

fn draft(symbol: &str) -> MirFunction {
    draft_with_arity(symbol, 0)
}

fn draft_with_arity(symbol: &str, arity: usize) -> MirFunction {
    MirFunction::new(
        FunctionSignature {
            name: symbol.to_owned(),
            params: vec![MirType::Integer; arity],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    )
}

fn collect(collector: &mut ModuleDraftCollectorV1, symbol: &str) {
    collector
        .prepare_admission(
            FunctionDraftKeyV1::LegacySymbol(symbol.to_owned()),
            symbol.to_owned(),
            0,
            DraftPublicationPolicyV1::LegacyReplaceWholePair,
        )
        .unwrap()
        .seal(draft(symbol))
        .unwrap()
        .collect();
}

fn collect_cataloged(collector: &mut ModuleDraftCollectorV1) {
    let key = crate::mir::builder::CanonicalSameModuleCallableKeyV1::test_static_box_method(
        "ParserScanLoopBox",
        "skip_while",
        4,
    );
    collector
        .prepare_admission(
            FunctionDraftKeyV1::CatalogedBoxMethod(key),
            "ParserScanLoopBox.skip_while/4".to_owned(),
            4,
            DraftPublicationPolicyV1::CanonicalRejectDuplicate,
        )
        .unwrap()
        .seal(draft_with_arity("ParserScanLoopBox.skip_while/4", 4))
        .unwrap()
        .collect();
}

#[test]
fn retained_construction_moves_only_through_normal_drain() {
    let key = CanonicalSameModuleCallableKeyV1::birth_constructor("Page", 0);
    let symbol = key.mir_symbol_projection();
    let collect_birth = || {
        let mut collector = ModuleDraftCollectorV1::with_brand(brand());
        collector
            .prepare_admission(
                FunctionDraftKeyV1::CatalogedConstructor(key.clone()),
                symbol.clone(),
                1,
                DraftPublicationPolicyV1::CanonicalRejectDuplicate,
            )
            .unwrap()
            .seal(draft_with_arity(&symbol, 1))
            .unwrap()
            .retain_construction(Some(
                RetainedConstructionValidation::empty_for_transport_test(),
            ))
            .unwrap()
            .collect();
        collector
    };
    let mut collector = collect_birth();
    assert!(collector.has_retained_construction());
    assert!(matches!(
        collector.prepare_admission(
            FunctionDraftKeyV1::CatalogedConstructor(key.clone()),
            symbol.clone(),
            1,
            DraftPublicationPolicyV1::LegacyReplaceWholePair,
        ),
        Err(super::super::ModuleDraftAdmissionErrorV1::ConstructionPayloadBoundary)
    ));
    assert!(
        collector.has_retained_construction(),
        "failed replacement retains the exact entry"
    );
    let mut module = MirModule::new("retained".into());
    let retained = collector
        .prepare_normal_collector_drain(&mut module, brand())
        .unwrap()
        .commit();
    assert_eq!(retained.len(), 1);
    let (actual, validation) = retained.into_iter().next().unwrap();
    assert_eq!(actual, key);
    validation
        .validate_after_compiler_finishing(&module.functions[&symbol])
        .unwrap();
    assert!(collect_birth().into_draft_functions().is_err());
    assert!(collect_birth()
        .into_single_observation_draft(&symbol)
        .is_err());

    let mut occupied = MirModule::new("occupied".into());
    occupied.add_function(draft_with_arity(&symbol, 1));
    let rejected = collect_birth()
        .prepare_normal_collector_drain(&mut occupied, brand())
        .unwrap_err();
    let (retained, _) = rejected.into_parts();
    assert!(retained.has_retained_construction());
    assert_eq!(occupied.functions.len(), 1);

    let mut collector = ModuleDraftCollectorV1::with_brand(brand());
    assert!(collector
        .prepare_admission(
            FunctionDraftKeyV1::LegacySymbol("legacy".into()),
            "legacy".into(),
            0,
            DraftPublicationPolicyV1::LegacyReplaceWholePair,
        )
        .unwrap()
        .seal(draft("legacy"))
        .unwrap()
        .retain_construction(Some(
            RetainedConstructionValidation::empty_for_transport_test()
        ))
        .is_err());
    assert!(
        collector
            .prepare_admission(
                FunctionDraftKeyV1::CatalogedConstructor(key),
                symbol.clone(),
                1,
                DraftPublicationPolicyV1::CanonicalRejectDuplicate,
            )
            .unwrap()
            .seal(draft_with_arity(&symbol, 1))
            .unwrap()
            .retain_construction(Some(
                RetainedConstructionValidation::empty_for_transport_test()
            ))
            .unwrap()
            .retain_construction(None)
            .is_err(),
        "cannot clear a retained payload"
    );
    assert!(collector.drafts.is_empty());
}

#[test]
fn object_definitions_move_once_with_drafts_and_reject_partial_publication() {
    use crate::mir::function::{CanonicalObjectDefinitionV1, UserBoxFieldDecl};
    use hakorune_mir_defs::{CanonicalFieldRefV1, CanonicalObjectIdV1};
    let payload = || {
        vec![CanonicalObjectDefinitionV1::from_source_declaration(
            "Page".into(),
            vec![UserBoxFieldDecl {
                name: "value".into(),
                declared_type_name: Some("i64".into()),
                is_weak: false,
            }]
            .into_boxed_slice(),
            Ok(()),
            crate::mir::function::ObjectDestructionDispositionV1::PlainI64NoHook,
        )]
        .into_boxed_slice()
    };
    let id = CanonicalObjectIdV1::from_declaration_index(0).unwrap();
    let mut required = ModuleDraftCollectorV1::with_required_object_definitions(brand());
    collect(&mut required, "must_not_publish");
    let mut missing = MirModule::new("missing".into());
    assert!(matches!(
        required
            .prepare_normal_collector_drain(&mut missing, brand())
            .unwrap_err()
            .error(),
        NormalCollectorDrainLifecycleErrorV1::ObjectDefinitionsMissing
    ));
    assert!(missing.functions.is_empty());
    let mut empty = ModuleDraftCollectorV1::with_required_object_definitions(brand());
    empty
        .install_object_definitions(Box::new([]), brand())
        .unwrap();
    empty
        .prepare_normal_collector_drain(&mut missing, brand())
        .unwrap()
        .commit();
    assert!(
        missing.preflight_object_definition_install().is_err(),
        "empty transfer is installed"
    );
    let mut collector = ModuleDraftCollectorV1::with_brand(brand());
    assert!(collector
        .install_object_definitions(payload(), ModuleInvocationBrandV1::test_with_ordinal(702))
        .is_err());
    collector
        .install_object_definitions(payload(), brand())
        .unwrap();
    assert!(collector
        .install_object_definitions(payload(), brand())
        .is_err());
    collect(&mut collector, "helper");
    let mut module = MirModule::new("objects".into());
    collector
        .prepare_normal_collector_drain(&mut module, brand())
        .unwrap()
        .commit();
    assert!(module.functions.contains_key("helper"));
    assert_eq!(
        module
            .canonical_object_definition(id)
            .unwrap()
            .diagnostic_name(),
        "Page"
    );
    assert_eq!(
        module
            .canonical_field_definition(
                CanonicalFieldRefV1::from_declaration_ordinal(id, 0).unwrap()
            )
            .unwrap()
            .name,
        "value"
    );
    assert!(module
        .canonical_field_definition(CanonicalFieldRefV1::from_declaration_ordinal(id, 1).unwrap())
        .is_none());
    assert!(module
        .canonical_object_definition(CanonicalObjectIdV1::from_declaration_index(1).unwrap())
        .is_none());

    let mut repeated = ModuleDraftCollectorV1::with_brand(brand());
    repeated
        .install_object_definitions(payload(), brand())
        .unwrap();
    collect(&mut repeated, "must_not_publish");
    assert!(repeated
        .prepare_normal_collector_drain(&mut module, brand())
        .is_err());
    assert!(!module.functions.contains_key("must_not_publish"));

    let mut rejected = ModuleDraftCollectorV1::with_brand(brand());
    rejected
        .install_object_definitions(payload(), brand())
        .unwrap();
    collect(&mut rejected, "bad");
    rejected.key_by_symbol.clear();
    let mut empty = MirModule::new("rejected".into());
    assert!(rejected
        .prepare_normal_collector_drain(&mut empty, brand())
        .is_err());
    assert!(empty.functions.is_empty());
    assert!(empty.canonical_object_definition(id).is_none());
}

#[test]
fn birth_definition_survives_atomic_drain_with_n_plus_one_abi() {
    use crate::mir::builder::CanonicalSameModuleCallableKeyV1;
    let key = CanonicalSameModuleCallableKeyV1::birth_constructor("Page", 2);
    let symbol = key.mir_symbol_projection();
    let mut collector = ModuleDraftCollectorV1::with_brand(brand());
    collector
        .prepare_admission(
            FunctionDraftKeyV1::CatalogedConstructor(key.clone()),
            symbol.clone(),
            3,
            DraftPublicationPolicyV1::CanonicalRejectDuplicate,
        )
        .unwrap()
        .seal(draft_with_arity(&symbol, 3))
        .unwrap()
        .collect();
    let mut module = MirModule::new("birth".into());
    assert!(matches!(
        module.preflight_cataloged_constructor(&key, &symbol, 2),
        Err(CanonicalCallableDefinitionPublicationErrorV1::KeyArityMismatch { .. })
    ));
    assert!(matches!(
        module.preflight_cataloged_box_method(&key, &symbol, 3),
        Err(CanonicalCallableDefinitionPublicationErrorV1::KeyNamespaceMismatch { .. })
    ));
    assert!(matches!(
        module.preflight_cataloged_constructor(&key, "Wrong.birth/2", 3),
        Err(CanonicalCallableDefinitionPublicationErrorV1::KeySymbolMismatch { .. })
    ));
    assert_eq!(module.canonical_callable_definition_count(), 0);
    collector
        .prepare_normal_collector_drain(&mut module, brand())
        .unwrap()
        .commit();
    assert_eq!(
        module.canonical_callable_definition_symbol(&key),
        Some(symbol.as_str())
    );
    assert_eq!(module.canonical_callable_definition_count(), 1);
    assert!(matches!(
        module.add_cataloged_constructor(key, draft_with_arity(&symbol, 3)),
        Err(CanonicalCallableDefinitionPublicationErrorV1::DuplicateKey { .. })
    ));
    assert_eq!(module.canonical_callable_definition_count(), 1);
}

#[test]
fn mixed_normal_drain_accepts_legacy_and_cataloged_rows() {
    let mut collector = ModuleDraftCollectorV1::with_brand(brand());
    collect(&mut collector, "legacy/0");
    collect_cataloged(&mut collector);
    let mut target = MirModule::new("normal".to_owned());

    collector
        .prepare_normal_collector_drain(&mut target, brand())
        .unwrap()
        .commit();

    assert_eq!(
        target.function_names(),
        vec!["ParserScanLoopBox.skip_while/4", "legacy/0"]
    );
    let key = crate::mir::builder::CanonicalSameModuleCallableKeyV1::test_static_box_method(
        "ParserScanLoopBox",
        "skip_while",
        4,
    );
    assert_eq!(target.canonical_callable_definition_count(), 1);
    assert_eq!(
        target.canonical_callable_definition_symbol(&key),
        Some("ParserScanLoopBox.skip_while/4")
    );
}

#[test]
fn normal_drain_rejects_cataloged_legacy_policy_without_publication() {
    let key = crate::mir::builder::CanonicalSameModuleCallableKeyV1::test_static_box_method(
        "ParserScanLoopBox",
        "skip_while",
        4,
    );
    let mut collector = ModuleDraftCollectorV1::with_brand(brand());
    collector
        .prepare_admission(
            FunctionDraftKeyV1::CatalogedBoxMethod(key),
            "ParserScanLoopBox.skip_while/4".to_owned(),
            4,
            DraftPublicationPolicyV1::LegacyReplaceWholePair,
        )
        .unwrap()
        .seal(draft_with_arity("ParserScanLoopBox.skip_while/4", 4))
        .unwrap()
        .collect();
    let mut target = MirModule::new("normal".to_owned());

    let rejected = collector
        .prepare_normal_collector_drain(&mut target, brand())
        .unwrap_err();
    assert!(matches!(
        rejected.error(),
        NormalCollectorDrainLifecycleErrorV1::FinalAdmissionDrift {
            key: FunctionDraftKeyV1::CatalogedBoxMethod(_)
        }
    ));
    let (collector, _) = rejected.into_parts();
    assert_eq!(collector.symbol_count(), 1);
    assert!(target.function_names().is_empty());
    assert_eq!(target.canonical_callable_definition_count(), 0);
}

#[test]
fn prepared_normal_drain_preserves_final_legacy_key_order_until_commit() {
    let mut collector = ModuleDraftCollectorV1::with_brand(brand());
    collect(&mut collector, "Zeta.run/0");
    collect(&mut collector, "Alpha.run/0");
    let mut target = MirModule::new("normal".to_owned());

    collector
        .prepare_normal_collector_drain(&mut target, brand())
        .unwrap()
        .commit();

    assert_eq!(target.function_names(), vec!["Alpha.run/0", "Zeta.run/0"]);
}

#[test]
fn empty_normal_collector_prepares_and_commits_without_publication() {
    let collector = ModuleDraftCollectorV1::with_brand(brand());
    let mut target = MirModule::new("normal".to_owned());

    collector
        .prepare_normal_collector_drain(&mut target, brand())
        .unwrap()
        .commit();

    assert!(target.function_names().is_empty());
}

#[test]
fn normal_drain_rejection_retains_collector_and_leaves_target_unchanged() {
    let mut collector = ModuleDraftCollectorV1::with_brand(brand());
    collect(&mut collector, "same/0");
    let mut target = MirModule::new("normal".to_owned());
    target.add_function(draft("same/0"));

    let rejected = collector
        .prepare_normal_collector_drain(&mut target, brand())
        .unwrap_err();
    assert!(matches!(
        rejected.error(),
        NormalCollectorDrainLifecycleErrorV1::Publication(_)
    ));
    let (collector, _) = rejected.into_parts();
    assert_eq!(collector.symbol_count(), 1);
    assert_eq!(target.function_names(), vec!["same/0"]);
}

#[test]
fn normal_drain_rejects_foreign_or_missing_session_brand_without_consuming_rows() {
    let mut collector = ModuleDraftCollectorV1::with_brand(brand());
    collect(&mut collector, "same/0");
    let mut target = MirModule::new("normal".to_owned());

    let rejected = collector
        .prepare_normal_collector_drain(
            &mut target,
            ModuleInvocationBrandV1::test_with_ordinal(702),
        )
        .unwrap_err();
    assert!(matches!(
        rejected.error(),
        NormalCollectorDrainLifecycleErrorV1::BrandMismatch
    ));
    let (collector, _) = rejected.into_parts();
    assert_eq!(collector.symbol_count(), 1);
    assert!(target.function_names().is_empty());
}

#[test]
fn normal_drain_rejects_nonlegacy_final_rows_without_consuming_them() {
    let mut collector = ModuleDraftCollectorV1::with_brand(brand());
    collector
        .prepare_admission(
            FunctionDraftKeyV1::Main,
            "main".to_owned(),
            0,
            DraftPublicationPolicyV1::LegacyReplaceWholePair,
        )
        .unwrap()
        .seal(draft("main"))
        .unwrap()
        .collect();
    let mut target = MirModule::new("normal".to_owned());

    let rejected = collector
        .prepare_normal_collector_drain(&mut target, brand())
        .unwrap_err();
    assert!(matches!(
        rejected.error(),
        NormalCollectorDrainLifecycleErrorV1::NonLegacyKey { .. }
    ));
    let (collector, _) = rejected.into_parts();
    assert_eq!(collector.symbol_count(), 1);
    assert!(target.function_names().is_empty());
}

#[test]
fn normal_drain_rejects_unconsumed_static_publication_before_commit() {
    let mut collector = ModuleDraftCollectorV1::with_brand(brand());
    collect(&mut collector, "same/0");
    let caller = crate::mir::builder::CanonicalSameModuleCallableKeyV1::test_static_box_method(
        "Owner", "caller", 0,
    );
    let target = crate::mir::builder::CanonicalSameModuleCallableKeyV1::test_static_box_method(
        "Owner", "target", 0,
    );
    let site = SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
        SourcePathSegmentV1::Body(0),
    ]));
    collector
        .install_static_result_publication_owner(
            VerifiedStaticCallResultPublicationOwnerV1::target_only_for_test(caller, site, target),
        )
        .unwrap();
    let mut module = MirModule::new("normal".to_owned());

    let rejected = collector
        .prepare_normal_collector_drain(&mut module, brand())
        .unwrap_err();
    assert!(matches!(
        rejected.error(),
        NormalCollectorDrainLifecycleErrorV1::StaticResultPublicationResidual(
            StaticCallResultPublicationOwnerFinishErrorV1::UnconsumedTargetOnly { .. }
        )
    ));
    let (collector, _) = rejected.into_parts();
    assert_eq!(collector.symbol_count(), 1);
    assert!(module.function_names().is_empty());
}
