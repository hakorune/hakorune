use crate::mir::MirCompiler;
use crate::parser::NyashParser;
use crate::runner::modes::common_util::source_hint::compile_with_source_hint;

fn ensure_stage3_env() {
    let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
    std::env::set_var("NYASH_FEATURES", "stage3");
    std::env::set_var("NYASH_PARSER_ALLOW_SEMICOLON", "1");
}

#[test]
fn mir_direct_route_decode_escapes_fixture_still_rejects_nested_loop_route() {
    ensure_stage3_env();

    let source =
        include_str!("../../apps/tests/phase29bq_selfhost_blocker_decode_escapes_if_idx12_min.hako");
    let ast = NyashParser::parse_from_string(source).expect("fixture should parse");
    let mut compiler = MirCompiler::with_options(false);

    let error = compile_with_source_hint(
        &mut compiler,
        ast,
        Some("apps/tests/phase29bq_selfhost_blocker_decode_escapes_if_idx12_min.hako"),
    )
    .expect_err("direct CLI-equivalent route should still reject this fixture");

    assert!(
        error.contains("[joinir/freeze]"),
        "expected joinir freeze tag, got: {error}"
    );
    assert!(
        error.contains("nested_loop_not_allowed"),
        "expected nested_loop_not_allowed, got: {error}"
    );
}
