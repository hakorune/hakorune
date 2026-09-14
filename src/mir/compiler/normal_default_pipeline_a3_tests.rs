use super::*;
use crate::parser::NyashParser;

#[test]
fn mixed_source_reaches_semantic_package_without_compatibility_retry() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let parsed = NyashParser::parse_normal_callable_program_with_build_config(
            "function helper(value: i64): i64 { return value }\n\
             static box Main { main() { return helper(2) } }",
            crate::parser::ParserBuildConfig::default(),
        )
        .expect("same-brand mixed source");
        let transformed = crate::r#macro::transform_normal_callable_program_v1(parsed)
            .expect("exact source transform");
        let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) = transformed
        else {
            panic!("mixed source must remain source-backed")
        };
        let result = MirCompiler::with_options(false)
            .compile_normal(NormalCompileRequestV1::for_mir_mode_callable_source(
                source,
                Some("a3-mixed-package.hako"),
                HashMap::new(),
            ))
            .expect("source-backed package handoff");
        let helper = hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::free_function(
            "helper", 1,
        );
        assert_eq!(
            result.module.canonical_callable_definition_symbol(&helper),
            Some("helper/1")
        );
    });
}
