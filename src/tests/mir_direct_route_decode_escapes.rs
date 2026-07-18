use crate::mir::{MirCompiler, MirVerifier};
use crate::parser::NyashParser;
use crate::runner::modes::common_util::source_hint::compile_with_source_hint;

fn ensure_stage3_env() {
    let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
    std::env::set_var("NYASH_FEATURES", "stage3");
    std::env::set_var("NYASH_PARSER_ALLOW_SEMICOLON", "1");
}

#[test]
fn mir_direct_route_decode_escapes_fixture_accepts_in_release_route() {
    crate::test_support::with_env_vars(&[("HAKO_JOINIR_DEBUG", None)], || {
        ensure_stage3_env();

        let source = include_str!(
            "../../apps/tests/phase29bq_selfhost_blocker_decode_escapes_if_idx12_min.hako"
        );
        let ast = NyashParser::parse_from_string(source).expect("fixture should parse");
        let mut compiler = MirCompiler::with_options(false);

        let result = compile_with_source_hint(
            &mut compiler,
            ast,
            Some("apps/tests/phase29bq_selfhost_blocker_decode_escapes_if_idx12_min.hako"),
        )
        .expect("direct CLI-equivalent release route should lower this fixture");

        let mut verifier = MirVerifier::new();
        verifier
            .verify_module(&result.module)
            .expect("release-route MIR should verify");
    });
}

#[test]
fn mir_direct_route_decode_escapes_fixture_accepts_in_joinir_debug_shadow_route() {
    crate::test_support::with_env_vars(&[("HAKO_JOINIR_DEBUG", Some("1"))], || {
        ensure_stage3_env();

        let source = include_str!(
            "../../apps/tests/phase29bq_selfhost_blocker_decode_escapes_if_idx12_min.hako"
        );
        let ast = NyashParser::parse_from_string(source).expect("fixture should parse");
        let mut compiler = MirCompiler::with_options(false);
        let result = compile_with_source_hint(
            &mut compiler,
            ast,
            Some("apps/tests/phase29bq_selfhost_blocker_decode_escapes_if_idx12_min.hako"),
        )
        .expect("debug shadow route should lower this fixture");

        let mut verifier = MirVerifier::new();
        verifier
            .verify_module(&result.module)
            .expect("lowered MIR should verify");
    });
}
