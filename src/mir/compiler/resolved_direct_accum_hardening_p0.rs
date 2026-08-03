//! H1/H2 ingress hardening for the resolved DirectAccum production lane.
//!
//! The failure seam is test-only and sits after candidate preparation, so the
//! complete physical/collector/finalization/postprocess chain is exercised
//! without adding a production fault toggle or a second compiler ingress.

use super::{MirCompiler, VerifiedResolvedSourceUnitV1};

fn source() -> VerifiedResolvedSourceUnitV1 {
    VerifiedResolvedSourceUnitV1::resolve_function(
        super::direct_accum_projection::direct_accum_function_for_test(),
    )
    .expect("DirectAccum hardening fixture resolves")
}

#[test]
fn production_failure_after_prepare_discards_candidate_and_reuses_compiler() {
    let unit = source();
    let mut compiler = MirCompiler::with_options(false);
    compiler.builder.set_source_file_hint("before.hako");
    compiler.builder.next_value_id();
    compiler.builder.next_block_id();
    let before = compiler.builder.loop_candidate_test_fingerprint();

    let error = super::resolved_direct_accum_cutover::
        compile_direct_accum_source_bound_with_prepared_failure_for_test(
            &mut compiler,
            unit.lowering_input(),
            Some("failed.hako"),
        )
        .expect_err("prepared commit failure must be terminal");
    assert!(matches!(
        error,
        super::CanonicalLoweringErrorV1::BuilderContract { detail }
            if detail.contains("test_injected_prepared_commit_failure")
    ));
    assert_eq!(compiler.builder.loop_candidate_test_fingerprint(), before);
    assert!(compiler.builder.current_module.is_none());

    let result = compiler
        .compile_resolved(unit.lowering_input(), Some("reused.hako"))
        .expect("same compiler must accept a fresh DirectAccum request");
    assert_eq!(result.verification_result, Ok(()));
    assert_eq!(result.module.functions.len(), 1);
    assert_eq!(
        compiler.builder.current_source_file().as_deref(),
        Some("reused.hako")
    );
}

#[test]
fn successful_direct_accum_public_result_uses_final_barrier_contract() {
    let unit = source();
    let mut compiler = MirCompiler::with_options(false);

    let result = compiler
        .compile_resolved(unit.lowering_input(), Some("success.hako"))
        .expect("DirectAccum success");

    assert_eq!(result.verification_result, Ok(()));
    assert_eq!(result.module.functions.len(), 1);
    assert!(compiler.builder.current_module.is_none());
}
