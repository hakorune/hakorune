use crate::parser::{NyashParser, ParserBuildConfig};
use crate::runner::modes::common_util::source_hint::prepare_normal_source_with_imports;
use crate::runner::NyashRunner;

#[test]
fn merged_parser_program_source_retains_static_target_input() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_vars(
        &[
            ("NYASH_ALLOW_USING_FILE", Some("1")),
            ("NYASH_ENABLE_USING", Some("1")),
            ("NYASH_OPERATOR_BOX_ALL", Some("0")),
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

            let parsed = NyashParser::parse_normal_callable_program_with_build_config(
                &prepared.code,
                ParserBuildConfig::default(),
            )
            .expect("merged parser source parses");
            drop(parsed);
        },
    );
}
