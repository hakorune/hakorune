    use super::{
        CompletedDraftSignatureViewV1, DraftPublicationPolicyV1, FunctionDraftKeyV1,
        ModuleDraftAdmissionErrorV1, ModuleDraftCollectorV1,
    };
    use crate::mir::resolved_semantics::FunctionOwnerIssuerV1;
    use crate::mir::{BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirType};
    use std::panic::{catch_unwind, AssertUnwindSafe};

    fn draft(symbol: &str, arity: usize) -> MirFunction {
        MirFunction::new(
            FunctionSignature {
                name: symbol.to_string(),
                params: vec![MirType::Integer; arity],
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        )
    }

    #[test]
    fn header_view_borrows_the_same_collector_owned_draft() {
        let mut collector = ModuleDraftCollectorV1::default();
        let prepared = collector
            .prepare_admission(
                FunctionDraftKeyV1::LegacySymbol("Parser.skip/1".into()),
                "Parser.skip/1".into(),
                1,
                DraftPublicationPolicyV1::LegacyReplaceWholePair,
            )
            .unwrap();
        prepared.seal(draft("Parser.skip/1", 1)).unwrap().collect();

        let signature = collector.signature("Parser.skip/1").unwrap();
        assert_eq!(signature.params, vec![MirType::Integer]);
        assert_eq!(signature.return_type, MirType::Integer);
        assert!(collector.contains_symbol("Parser.skip/1"));
        assert!(!collector.contains_symbol("Parser.missing/0"));
        assert_eq!(collector.symbol_count(), 1);
    }

    #[test]
    fn header_view_visits_same_owned_symbols_in_deterministic_order() {
        let mut collector = ModuleDraftCollectorV1::default();
        for symbol in ["Zeta.run/0", "Alpha.run/2"] {
            let arity = symbol.rsplit_once('/').unwrap().1.parse::<usize>().unwrap();
            let prepared = collector
                .prepare_admission(
                    FunctionDraftKeyV1::LegacySymbol(symbol.into()),
                    symbol.into(),
                    arity,
                    DraftPublicationPolicyV1::LegacyReplaceWholePair,
                )
                .unwrap();
            prepared.seal(draft(symbol, arity)).unwrap().collect();
        }

        let mut visited = Vec::new();
        collector.visit_symbols(&mut |symbol| visited.push(symbol.to_owned()));
        assert_eq!(visited, ["Alpha.run/2", "Zeta.run/0"]);
        assert_eq!(collector.symbol_count(), 2);
        assert_eq!(collector.signature("Alpha.run/2").unwrap().params.len(), 2);
    }

    #[test]
    fn legacy_replacement_discards_the_whole_old_draft_pair() {
        let mut collector = ModuleDraftCollectorV1::default();
        for return_type in [MirType::Integer, MirType::String] {
            let prepared = collector
                .prepare_admission(
                    FunctionDraftKeyV1::LegacySymbol("Legacy.f/0".into()),
                    "Legacy.f/0".into(),
                    0,
                    DraftPublicationPolicyV1::LegacyReplaceWholePair,
                )
                .unwrap();
            let mut next = draft("Legacy.f/0", 0);
            next.signature.return_type = return_type;
            prepared.seal(next).unwrap().collect();
        }

        assert_eq!(
            collector.signature("Legacy.f/0").unwrap().return_type,
            MirType::String
        );
    }

    #[test]
    fn canonical_duplicate_rejects_before_draft_seal_or_collection() {
        let mut collector = ModuleDraftCollectorV1::default();
        let key = FunctionDraftKeyV1::LegacySymbol("Canonical.f/0".into());
        let prepared = collector
            .prepare_admission(
                key.clone(),
                "Canonical.f/0".into(),
                0,
                DraftPublicationPolicyV1::CanonicalRejectDuplicate,
            )
            .unwrap();
        prepared.seal(draft("Canonical.f/0", 0)).unwrap().collect();

        let error = collector
            .prepare_admission(
                key,
                "Canonical.f/0".into(),
                0,
                DraftPublicationPolicyV1::CanonicalRejectDuplicate,
            )
            .unwrap_err();
        assert!(matches!(
            error,
            ModuleDraftAdmissionErrorV1::DuplicateKey(_)
        ));
        assert_eq!(
            collector.signature("Canonical.f/0").unwrap().params.len(),
            0
        );
    }

    #[test]
    fn resolved_owner_key_is_distinct_from_legacy_symbol_identity() {
        let mut issuer = FunctionOwnerIssuerV1::new_for_compilation().unwrap();
        let owner = issuer.issue().unwrap();
        let mut collector = ModuleDraftCollectorV1::default();

        collector
            .prepare_admission(
                FunctionDraftKeyV1::CanonicalResolvedOwner(owner),
                "canonical_a_plus/0".into(),
                0,
                DraftPublicationPolicyV1::CanonicalRejectDuplicate,
            )
            .unwrap()
            .seal(draft("canonical_a_plus/0", 0))
            .unwrap()
            .collect();

        let error = collector
            .prepare_admission(
                FunctionDraftKeyV1::CanonicalResolvedOwner(owner),
                "canonical_a_plus/0".into(),
                0,
                DraftPublicationPolicyV1::CanonicalRejectDuplicate,
            )
            .unwrap_err();
        assert!(matches!(
            error,
            ModuleDraftAdmissionErrorV1::DuplicateKey(
                FunctionDraftKeyV1::CanonicalResolvedOwner(actual)
            ) if actual == owner
        ));
    }

    #[test]
    fn canonical_symbol_collision_rejects_a_distinct_resolved_owner_key() {
        let mut issuer = FunctionOwnerIssuerV1::new_for_compilation().unwrap();
        let first_owner = issuer.issue().unwrap();
        let second_owner = issuer.issue().unwrap();
        let mut collector = ModuleDraftCollectorV1::default();

        collector
            .prepare_admission(
                FunctionDraftKeyV1::CanonicalResolvedOwner(first_owner),
                "same_symbol/0".into(),
                0,
                DraftPublicationPolicyV1::CanonicalRejectDuplicate,
            )
            .unwrap()
            .seal(draft("same_symbol/0", 0))
            .unwrap()
            .collect();

        let error = collector
            .prepare_admission(
                FunctionDraftKeyV1::CanonicalResolvedOwner(second_owner),
                "same_symbol/0".into(),
                0,
                DraftPublicationPolicyV1::CanonicalRejectDuplicate,
            )
            .unwrap_err();
        assert!(matches!(
            error,
            ModuleDraftAdmissionErrorV1::DuplicateSymbol(symbol) if symbol == "same_symbol/0"
        ));
        assert_eq!(collector.symbol_count(), 1);
        assert!(collector.contains_symbol("same_symbol/0"));
    }

    #[test]
    fn signature_or_arity_drift_rejects_without_collector_mutation() {
        let mut collector = ModuleDraftCollectorV1::default();
        let prepared = collector
            .prepare_admission(
                FunctionDraftKeyV1::Main,
                "main".into(),
                0,
                DraftPublicationPolicyV1::CanonicalRejectDuplicate,
            )
            .unwrap();
        let error = prepared.seal(draft("main", 1)).unwrap_err();
        assert!(matches!(
            error,
            ModuleDraftAdmissionErrorV1::ArityMismatch { .. }
        ));
        assert!(collector.signature("main").is_none());
    }

    #[test]
    fn p0_main_and_synthetic_condition_drafts_share_one_header_view() {
        let mut collector = ModuleDraftCollectorV1::default();
        for (key, symbol, arity, policy) in [
            (
                FunctionDraftKeyV1::Main,
                "main",
                0,
                DraftPublicationPolicyV1::LegacyReplaceWholePair,
            ),
            (
                FunctionDraftKeyV1::SyntheticConditionFn,
                "condition_fn",
                1,
                DraftPublicationPolicyV1::CanonicalRejectDuplicate,
            ),
        ] {
            collector
                .prepare_admission(key, symbol.into(), arity, policy)
                .unwrap()
                .seal(draft(symbol, arity))
                .unwrap()
                .collect();
        }

        let mut symbols = Vec::new();
        collector.visit_symbols(&mut |symbol| symbols.push(symbol.to_owned()));
        assert_eq!(symbols, ["condition_fn", "main"]);
        assert_eq!(collector.symbol_count(), 2);
    }

    #[test]
    fn p0_route_policy_matrix_covers_every_root_and_child_family() {
        use super::super::module_invocation_route_matrix::{
            InvocationIdentityV1, InvocationRouteMatrixV1,
        };
        let mut issuer = FunctionOwnerIssuerV1::new_for_compilation().unwrap();
        let mut collector = ModuleDraftCollectorV1::default();
        for row in InvocationRouteMatrixV1::rows() {
            let (symbol, key) = match row.identity() {
                InvocationIdentityV1::Main => ("main".to_owned(), FunctionDraftKeyV1::Main),
                InvocationIdentityV1::SyntheticConditionFn => (
                    "condition_fn".to_owned(),
                    FunctionDraftKeyV1::SyntheticConditionFn,
                ),
                InvocationIdentityV1::LegacySymbol => {
                    let symbol = format!("p0/{}/0", row.name());
                    (symbol.clone(), FunctionDraftKeyV1::LegacySymbol(symbol))
                }
                InvocationIdentityV1::CanonicalResolvedOwner => {
                    let symbol = format!("p0/{}/0", row.name());
                    (
                        symbol,
                        FunctionDraftKeyV1::CanonicalResolvedOwner(issuer.issue().unwrap()),
                    )
                }
                InvocationIdentityV1::CanonicalCallable => continue,
            };
            collector
                .prepare_admission(key, symbol.clone(), 0, row.publication())
                .unwrap()
                .seal(draft(&symbol, 0))
                .unwrap()
                .collect();
        }

        assert_eq!(collector.symbol_count(), 7);
        assert!(collector.contains_symbol("main"));
        assert!(collector.contains_symbol("condition_fn"));
        assert!(collector.contains_symbol("p0/canonical_a_plus_child/0"));
        assert!(!collector.contains_symbol("p0/binding_ssa_acyclic_module/0"));
    }

    #[test]
    fn p0_admission_failures_stop_before_collecting_a_new_draft() {
        let mut collector = ModuleDraftCollectorV1::default();
        collector
            .prepare_admission(
                FunctionDraftKeyV1::LegacySymbol("canonical/0".into()),
                "canonical/0".into(),
                0,
                DraftPublicationPolicyV1::CanonicalRejectDuplicate,
            )
            .unwrap()
            .seal(draft("canonical/0", 0))
            .unwrap()
            .collect();

        let duplicate = collector.prepare_admission(
            FunctionDraftKeyV1::LegacySymbol("canonical/0".into()),
            "canonical/0".into(),
            0,
            DraftPublicationPolicyV1::CanonicalRejectDuplicate,
        );
        assert!(matches!(
            duplicate,
            Err(ModuleDraftAdmissionErrorV1::DuplicateKey(_))
        ));

        let mismatch = collector
            .prepare_admission(
                FunctionDraftKeyV1::LegacySymbol("arity/0".into()),
                "arity/0".into(),
                0,
                DraftPublicationPolicyV1::LegacyReplaceWholePair,
            )
            .unwrap()
            .seal(draft("arity/0", 1));
        assert!(matches!(
            mismatch,
            Err(ModuleDraftAdmissionErrorV1::ArityMismatch { .. })
        ));
        assert_eq!(collector.symbol_count(), 1);
        assert!(collector.contains_symbol("canonical/0"));
        assert!(!collector.contains_symbol("arity/0"));
    }

    #[test]
    fn p0_unwind_before_collect_leaves_the_collector_unchanged() {
        let mut collector = ModuleDraftCollectorV1::default();
        let unwind = catch_unwind(AssertUnwindSafe(|| {
            let prepared = collector
                .prepare_admission(
                    FunctionDraftKeyV1::LegacySymbol("unwind/0".into()),
                    "unwind/0".into(),
                    0,
                    DraftPublicationPolicyV1::LegacyReplaceWholePair,
                )
                .unwrap();
            let _unpublished = prepared.seal(draft("unwind/0", 0)).unwrap();
            panic!("P0 unwind before collect");
        }));

        assert!(unwind.is_err());
        assert_eq!(collector.symbol_count(), 0);
        assert!(!collector.contains_symbol("unwind/0"));
    }

    #[test]
    fn legacy_index_drift_is_rejected_before_collect_mutation() {
        let mut collector = ModuleDraftCollectorV1::default();
        collector.inject_symbol_index_drift_for_test(
            "drift/0",
            FunctionDraftKeyV1::LegacySymbol("other/0".into()),
        );

        let error = collector.prepare_admission(
            FunctionDraftKeyV1::LegacySymbol("drift/0".into()),
            "drift/0".into(),
            0,
            DraftPublicationPolicyV1::LegacyReplaceWholePair,
        );
        assert!(matches!(
            error,
            Err(ModuleDraftAdmissionErrorV1::IndexDrift { .. })
        ));
        assert_eq!(collector.symbol_count(), 1);
        assert!(collector.contains_symbol("drift/0"));
    }
