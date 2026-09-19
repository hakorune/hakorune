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
    crate::test_support::with_env_vars(
        &[
            ("NYASH_ALLOW_USING_FILE", Some("1")),
            ("NYASH_ENABLE_USING", Some("1")),
            ("NYASH_OPERATOR_BOX_ALL", Some("0")),
            ("NYASH_MACRO_DISABLE", Some("1")),
        ],
        || {
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
            assert!(
                message.contains(
                    "[freeze:contract][callable-loop/route-not-front-selected] GenericLoopV1NotSelected"
                ),
                "unexpected parser loop terminal: {message}"
            );
            rejected.discard();
        },
    );
}
