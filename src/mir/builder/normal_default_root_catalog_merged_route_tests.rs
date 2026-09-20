use crate::mir::builder::{
    CallableMainMaterializationPolicyV1, NormalRuntimeInputSnapshotV1,
    PreparedNormalDefaultProgramRootV1,
};
use crate::runner::modes::common_util::normal_callable::{
    materialize_normal_callable_program_with_identity_and_lineage_v1,
    NormalCallableMaterializationOutcomeV1,
};
use crate::runner::modes::common_util::source_hint::prepare_normal_source_with_imports;
use crate::runner::NyashRunner;

#[test]
fn merged_parser_program_source_stops_at_named_loop_boundary_before_static_target() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    // The loop boundary reads the ambient JoinIR mode keys, so pin the
    // default mode to keep the observed terminal stable under a concurrent
    // strict/planner_required window.
    let env_updates: Vec<(&'static str, Option<&'static str>)> =
        crate::test_support::JOINIR_DEFAULT_MODE
            .into_iter()
            .chain([
                ("NYASH_ALLOW_USING_FILE", Some("1")),
                ("NYASH_ENABLE_USING", Some("1")),
                ("NYASH_OPERATOR_BOX_ALL", Some("0")),
                ("NYASH_MACRO_DISABLE", Some("1")),
            ])
            .collect();
    crate::test_support::with_env_vars(&env_updates, || {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let filename = root.join("lang/src/compiler/parser/program/parser_program_box.hako");
        let code = std::fs::read_to_string(&filename).expect("parser program source");
        let runner = NyashRunner::new(Default::default());
        let prepared = prepare_normal_source_with_imports(
            &runner,
            filename.to_str().expect("utf8 parser path"),
            &code,
        )
        .expect("merged parser source");

        assert!(prepared.code.contains("static box ParserProgramBox"));
        assert!(prepared.code.contains("static box ParserStringUtilsBox"));
        assert!(prepared.code.contains("ParserStringUtilsBox.starts_with"));
        assert!(prepared.lineage.segments().len() > 1);
        assert!(prepared.lineage.edges().len() >= 3);

        let crate::runner::modes::common_util::source_hint::PreparedSourceWithImports {
            code: prepared_code,
            lineage: prepared_lineage,
            ..
        } = prepared;
        let transformed = materialize_normal_callable_program_with_identity_and_lineage_v1(
            prepared_code,
            runner.parser_build_config(),
            filename.to_string_lossy().into_owned(),
            prepared_lineage,
        )
        .expect("merged parser materialization");
        let NormalCallableMaterializationOutcomeV1::SourceBacked(source) = transformed else {
            panic!("merged parser source must stay source-backed")
        };
        let lineage = source.source_lineage().expect("parser source lineage");
        assert!(lineage.merged_source_lineage().is_some());
        let root = PreparedNormalDefaultProgramRootV1::from_callable_source(source);
        let rejected = super::normal_default_root_catalog_lifecycle_tests::session()
            .complete_normal_default_program_root_catalog_lifecycle(
                root,
                CallableMainMaterializationPolicyV1::Omitted,
                NormalRuntimeInputSnapshotV1::empty(),
            )
            .expect_err("parser program must stop at its named loop boundary");
        let message = rejected.error().to_string();
        // The selected LoopCond and LoopTrue source handoffs now cross the
        // preceding parser-loop boundaries. The complete merged package still
        // contains a LoopBreakRecipe route outside this source tuple, so the
        // existing source bridge stops before catalog installation with its
        // named non-selected-route terminal.
        assert!(
            message.contains("callable-loop/route-not-front-selected")
                && message.contains("GenericLoopV1NotSelected"),
            "unexpected parser loop terminal: {message}"
        );
        rejected.discard();
    });
}

/// Diagnostic pin for the selected source-loop edges: the merged parser
/// source inventory still observes the target call inside the parse loop,
/// while the full package remains bounded by the existing non-selected route
/// terminal before parser publication.
#[test]
fn merged_parser_static_inventory_probe() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    let env_updates: Vec<(&'static str, Option<&'static str>)> =
        crate::test_support::JOINIR_DEFAULT_MODE
            .into_iter()
            .chain([
                ("NYASH_ALLOW_USING_FILE", Some("1")),
                ("NYASH_ENABLE_USING", Some("1")),
                ("NYASH_OPERATOR_BOX_ALL", Some("0")),
                ("NYASH_MACRO_DISABLE", Some("1")),
            ])
            .collect();
    crate::test_support::with_env_vars(&env_updates, || {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let filename = root.join("lang/src/compiler/parser/program/parser_program_box.hako");
        let code = std::fs::read_to_string(&filename).expect("parser program source");
        let runner = NyashRunner::new(Default::default());
        let prepared = prepare_normal_source_with_imports(
            &runner,
            filename.to_str().expect("utf8 parser path"),
            &code,
        )
        .expect("merged parser source");
        let transformed = materialize_normal_callable_program_with_identity_and_lineage_v1(
            prepared.code,
            runner.parser_build_config(),
            filename.to_string_lossy().into_owned(),
            prepared.lineage,
        )
        .expect("merged parser materialization");
        let NormalCallableMaterializationOutcomeV1::SourceBacked(source) = transformed else {
            panic!("merged parser source must stay source-backed")
        };

        let mut resolver =
            crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1::new(4_219)
                .expect("resolver");
        let package = crate::mir::normal_callable_semantic_package::issue_normal_callable_semantic_package_v1(
            &mut resolver,
            source,
        )
        .expect("merged parser semantic package");
        let declarations = package.declaration_catalog();
        let imports = crate::mir::source_call_target::VerifiedStaticImportAliasViewV1::seal(
            declarations,
            std::iter::empty::<(String, String)>(),
        )
        .expect("empty import view seals against merged declarations");
        let inventory =
            crate::mir::source_call_target::VerifiedWholeSourceStaticCallTargetInventoryV1::verify(
                declarations,
                &imports,
            )
            .expect("merged parser inventory");

        // The merged inventory observes every caller and publishes the
        // acceptance tuple rows: `ParserProgramBox.parse/2` carries
        // `ParserStringUtilsBox.starts_with/3` targets inside the loop body.
        assert!(inventory
            .first_method_observation_unavailability()
            .is_none());
        let parse_key =
            crate::mir::builder::CanonicalSameModuleCallableKeyV1::test_static_box_method(
                "ParserProgramBox",
                "parse",
                2,
            );
        let starts_with_targets = inventory
            .calls()
            .filter(|row| row.call().caller() == &parse_key)
            .filter(|row| row.call().method() == "starts_with")
            .filter(|row| {
                row.call().site().node().segments().iter().any(|segment| {
                    matches!(
                        segment,
                        crate::mir::resolved_semantics::SourcePathSegmentV1::LoopBody(_)
                    )
                })
            })
            .filter(|row| inventory.target(&parse_key, row.call().site()).is_some())
            .count();
        assert!(
            starts_with_targets > 0,
            "merged parser inventory must publish starts_with targets inside the parse loop"
        );
        // The first reached armed LoopCond loop belongs to
        // `StringHelpers.index_of/3`; its loop items are Bound-receiver
        // `substring` calls, so none publish a static target row — that is
        // the `SourceCallOutsideSelectedFamily` boundary the guard pins.
        let index_of_key =
            crate::mir::builder::CanonicalSameModuleCallableKeyV1::test_static_box_method(
                "StringHelpers",
                "index_of",
                3,
            );
        let index_of_published = inventory
            .calls()
            .filter(|row| row.call().caller() == &index_of_key)
            .filter(|row| inventory.target(&index_of_key, row.call().site()).is_some())
            .count();
        assert_eq!(
            index_of_published, 0,
            "index_of loop items must stay unpublished (Bound receivers)"
        );
    });
}
