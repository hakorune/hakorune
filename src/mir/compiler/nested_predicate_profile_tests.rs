use super::capability::{CanonicalFirstFamilyPlanV1, CanonicalLoweringPreflightV1};
use super::nested_predicate_producer_tests::nested_function;
use super::nested_predicate_profile::CanonicalNestedPredicatePlanV1;
use super::VerifiedResolvedSourceUnitV1;

fn nested_plan() -> CanonicalNestedPredicatePlanV1<'static> {
    let unit = Box::leak(Box::new(
        VerifiedResolvedSourceUnitV1::resolve_function(nested_function())
            .expect("nested source unit"),
    ));
    let plan = CanonicalLoweringPreflightV1::verify(unit).expect("nested preflight");
    let CanonicalFirstFamilyPlanV1::Loop(
        super::capability::CanonicalLoopFamilyPlanV1::NestedPredicate(plan),
    ) = plan
    else {
        panic!("Nested must win before DirectAccum");
    };
    plan
}

#[test]
fn nested_probe_wins_over_overlapping_direct_accum_envelope() {
    let plan = nested_plan();
    assert_eq!(plan.input().owner(), plan.loop_stmt().owner());
    assert_eq!(plan.claims().prefix().owner(), plan.input().owner());
    assert_eq!(plan.emission().topology().owner(), plan.input().owner());
}

#[test]
fn nested_plan_seals_existing_binding_ssa_header_family() {
    let plan = nested_plan();
    let header = plan
        .seal_resolved_owner_header_v1()
        .expect("nested owner header");
    assert_eq!(
        header.family(),
        super::capability::ResolvedOwnerHeaderFamilyV1::TrivialBindingSsa
    );
    assert_eq!(header.owner(), plan.input().owner());
}

#[test]
fn compile_resolved_nested_predicate_uses_the_candidate_physicalizer() {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(nested_function())
        .expect("nested source unit");
    let mut compiler = super::MirCompiler::with_options(false);

    let result = compiler
        .compile_resolved(unit.lowering_input(), Some("nested_predicate.hako"))
        .expect("nested predicate source-bound compilation");

    assert_eq!(result.verification_result, Ok(()));
    assert_eq!(result.module.functions.len(), 1);
    assert!(result
        .module
        .functions
        .contains_key("nested_loop_minimal/0"));
    assert!(compiler.builder.current_module.is_none());
}

#[test]
fn compile_resolved_nested_predicate_does_not_enter_legacy_loop_physicalizer() {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(nested_function())
        .expect("nested source unit");
    let mut compiler = super::MirCompiler::with_options(false);
    crate::mir::builder::reset_loop_physical_effect_probe();

    compiler
        .compile_resolved(unit.lowering_input(), Some("nested-no-legacy.hako"))
        .expect("nested source-bound compilation");

    assert_eq!(crate::mir::builder::take_loop_physical_effect_probe(), 0);
}

#[test]
fn compile_resolved_nested_predicate_reuses_one_compiler_after_commit() {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(nested_function())
        .expect("nested source unit");
    let mut compiler = super::MirCompiler::with_options(false);

    for source_file in ["nested-first.hako", "nested-second.hako"] {
        let result = compiler
            .compile_resolved(unit.lowering_input(), Some(source_file))
            .expect("fresh nested source-bound compilation");
        assert_eq!(result.verification_result, Ok(()));
        assert_eq!(result.module.functions.len(), 1);
        assert!(compiler.builder.current_module.is_none());
    }
}

#[test]
fn nested_canonical_digest_is_fixed() {
    let plan = nested_plan();
    let recipe = plan.emission().recipe().as_recipe();
    let join_sig = plan.emission().join_sig().as_sig();
    let topology = plan.emission().topology();
    let semantic_digest = (
        recipe.loops.len(),
        recipe.blocks.len(),
        recipe.items.len(),
        recipe.values.len(),
        join_sig.loops.len(),
        topology.ports().len(),
        topology.edges().len(),
        topology.predecessor_seals().len(),
        topology.source_roles().len(),
    );
    assert_eq!(semantic_digest, (2, 4, 20, 18, 2, 10, 11, 8, 10));
}

#[test]
fn nested_prepared_failure_drops_candidate_and_preserves_fresh_reuse() {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(nested_function())
        .expect("nested source unit");
    let mut compiler = super::MirCompiler::with_options(false);
    compiler.builder.set_source_file_hint("before.hako");
    compiler.builder.next_value_id();
    compiler.builder.next_block_id();
    let before = compiler.builder.loop_candidate_test_fingerprint();

    let error = super::resolved_nested_predicate_cutover::
        compile_nested_predicate_source_bound_with_prepared_failure_for_test(
            &mut compiler,
            unit.lowering_input(),
            Some("failed.hako"),
        )
        .expect_err("prepared commit failure must be terminal");
    assert!(matches!(
        error,
        super::CanonicalLoweringErrorV1::ResolvedCutover(
            super::lowering_input::CanonicalResolvedCutoverFailureV1::NestedPredicate(
                super::lowering_input::CanonicalResolvedCutoverStageErrorV1::ExternalCommit(
                    super::external_commit::ExternalCommitPreparationErrorV1::EvidenceMismatch
                )
            )
        )
    ));
    assert_eq!(compiler.builder.loop_candidate_test_fingerprint(), before);
    assert!(compiler.builder.current_module.is_none());

    let result = compiler
        .compile_resolved(unit.lowering_input(), Some("reused.hako"))
        .expect("same compiler must accept a fresh Nested compilation");
    assert_eq!(result.verification_result, Ok(()));
    assert_eq!(result.module.functions.len(), 1);
    assert_eq!(
        compiler.builder.current_source_file().as_deref(),
        Some("reused.hako")
    );
}
