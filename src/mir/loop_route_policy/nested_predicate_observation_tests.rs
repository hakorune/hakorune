use super::{
    issue_nested_predicate_family_observation_v1, NestedPredicateFamilyObservationV1,
    NestedPredicateObservationContextV1, NestedPredicateObservationDeclineV1,
    NestedPredicateObservationRejectV1, NestedPredicateObservationUnresolvedV1,
};
use crate::ast::ASTNode;
use crate::mir::compiler::nested_predicate_observation::issue_nested_predicate_source_attempt_for_test;
use crate::mir::compiler::VerifiedResolvedSourceUnitV1;
use crate::mir::loop_structural_facts::{
    NestedPredicateObservationCoverageV1, NestedPredicateObservationModeV1,
    NestedPredicateSourceAttemptOutcomeV1, NestedPredicateSourceIdentityV1,
    NestedPredicateSourceUnresolvedV1, VerifiedNestedPredicateSourceAttemptV1,
};

fn prepared(
    tree: ASTNode,
    attempt_mode: Option<NestedPredicateObservationModeV1>,
    attempt_coverage: NestedPredicateObservationCoverageV1,
    context_mode: Option<NestedPredicateObservationModeV1>,
    context_coverage: NestedPredicateObservationCoverageV1,
) -> (
    crate::mir::loop_structural_facts::VerifiedNestedPredicateSourceAttemptV1,
    NestedPredicateObservationContextV1,
) {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(tree).expect("fixture resolves");
    let input = unit.root_function_input().expect("function input");
    let body = input.source().root_body().expect("function body");
    let root = input.source().body_stmt(&body, 1).expect("root loop");
    let source = input
        .function()
        .resolved_loop_source(root.site())
        .expect("root source");
    let identity = NestedPredicateSourceIdentityV1::new(
        input.owner(),
        input.function().function_origin(),
        input.function().source_kind(),
        root.site().clone(),
        source.frame_key(),
    );
    let attempt = issue_nested_predicate_source_attempt_for_test(
        input,
        root,
        source,
        attempt_mode,
        attempt_coverage,
    );
    let context =
        NestedPredicateObservationContextV1::for_test(identity, context_mode, context_coverage);
    (attempt, context)
}

#[test]
fn exact_nested_projection_is_candidate_in_all_sealed_modes() {
    for mode in [
        NestedPredicateObservationModeV1::Release,
        NestedPredicateObservationModeV1::Strict,
        NestedPredicateObservationModeV1::StrictPlannerRequired,
    ] {
        let (attempt, context) = prepared(
            crate::mir::compiler::nested_function_for_p3_test(),
            Some(mode),
            NestedPredicateObservationCoverageV1::Complete,
            Some(mode),
            NestedPredicateObservationCoverageV1::Complete,
        );
        let NestedPredicateFamilyObservationV1::Candidate(candidate) =
            issue_nested_predicate_family_observation_v1(attempt, context)
        else {
            panic!("exact NestedPredicate must be a candidate")
        };
        assert_eq!(candidate.context().mode(), Some(mode));
        assert_eq!(
            candidate.observation().root_frame_key(),
            candidate.context().identity().frame()
        );
    }
}

#[test]
fn known_non_nested_shape_declines_without_route_fallback() {
    let mut tree = crate::mir::compiler::nested_function_for_p3_test();
    let ASTNode::FunctionDeclaration { body, .. } = &mut tree else {
        panic!("fixture root")
    };
    let ASTNode::Loop { body: outer, .. } = &mut body[1] else {
        panic!("outer loop")
    };
    let ASTNode::Loop { body: inner, .. } = &mut outer[2] else {
        panic!("inner loop")
    };
    inner.pop();
    let (attempt, context) = prepared(
        tree,
        Some(NestedPredicateObservationModeV1::Release),
        NestedPredicateObservationCoverageV1::Complete,
        Some(NestedPredicateObservationModeV1::Release),
        NestedPredicateObservationCoverageV1::Complete,
    );
    assert_eq!(
        issue_nested_predicate_family_observation_v1(attempt, context),
        NestedPredicateFamilyObservationV1::Declined(
            NestedPredicateObservationDeclineV1::NotNestedPredicateShape
        )
    );
}

#[test]
fn incomplete_window_is_unresolved_before_shape_disposition() {
    let (attempt, context) = prepared(
        crate::mir::compiler::nested_function_for_p3_test(),
        Some(NestedPredicateObservationModeV1::Release),
        NestedPredicateObservationCoverageV1::Complete,
        Some(NestedPredicateObservationModeV1::Release),
        NestedPredicateObservationCoverageV1::Incomplete,
    );
    assert_eq!(
        issue_nested_predicate_family_observation_v1(attempt, context),
        NestedPredicateFamilyObservationV1::Unresolved(
            NestedPredicateObservationUnresolvedV1::IncompleteCoverage
        )
    );
}

#[test]
fn unsealed_mode_is_unresolved_without_policy_guess() {
    let (attempt, context) = prepared(
        crate::mir::compiler::nested_function_for_p3_test(),
        None,
        NestedPredicateObservationCoverageV1::Complete,
        None,
        NestedPredicateObservationCoverageV1::Complete,
    );
    assert_eq!(
        issue_nested_predicate_family_observation_v1(attempt, context),
        NestedPredicateFamilyObservationV1::Unresolved(
            NestedPredicateObservationUnresolvedV1::ModeUnsealed
        )
    );
}

#[test]
fn mode_mismatch_is_rejected_before_candidate_issue() {
    let (attempt, context) = prepared(
        crate::mir::compiler::nested_function_for_p3_test(),
        Some(NestedPredicateObservationModeV1::Release),
        NestedPredicateObservationCoverageV1::Complete,
        Some(NestedPredicateObservationModeV1::Strict),
        NestedPredicateObservationCoverageV1::Complete,
    );
    assert_eq!(
        issue_nested_predicate_family_observation_v1(attempt, context),
        NestedPredicateFamilyObservationV1::Rejected(
            NestedPredicateObservationRejectV1::ModeMismatch
        )
    );
}

#[test]
fn foreign_owner_is_rejected_before_shape_policy() {
    let (attempt, context) = prepared(
        crate::mir::compiler::nested_function_for_p3_test(),
        Some(NestedPredicateObservationModeV1::Release),
        NestedPredicateObservationCoverageV1::Complete,
        Some(NestedPredicateObservationModeV1::Release),
        NestedPredicateObservationCoverageV1::Complete,
    );
    let foreign = crate::mir::resolved_semantics::FunctionOwnerIssuerV1::new_for_compilation()
        .expect("owner issuer")
        .issue()
        .expect("foreign owner");
    let identity = NestedPredicateSourceIdentityV1::new(
        foreign,
        context.identity().function_origin(),
        context.identity().source_kind(),
        context.identity().site().clone(),
        context.identity().frame().clone(),
    );
    let foreign_context =
        NestedPredicateObservationContextV1::for_test(identity, context.mode(), context.coverage());
    assert_eq!(
        issue_nested_predicate_family_observation_v1(attempt, foreign_context),
        NestedPredicateFamilyObservationV1::Rejected(
            NestedPredicateObservationRejectV1::ForeignContext
        )
    );
}

#[test]
fn missing_forest_root_is_unresolved_source_lookup() {
    let (attempt, context) = prepared(
        crate::mir::compiler::nested_function_for_p3_test(),
        Some(NestedPredicateObservationModeV1::Release),
        NestedPredicateObservationCoverageV1::Complete,
        Some(NestedPredicateObservationModeV1::Release),
        NestedPredicateObservationCoverageV1::Complete,
    );
    let (_, identity, mode, coverage) = attempt.into_parts();
    let attempt = VerifiedNestedPredicateSourceAttemptV1::new(
        NestedPredicateSourceAttemptOutcomeV1::Unresolved(
            NestedPredicateSourceUnresolvedV1::SourceLookup,
        ),
        identity,
        mode,
        coverage,
    );
    assert_eq!(
        issue_nested_predicate_family_observation_v1(attempt, context),
        NestedPredicateFamilyObservationV1::Unresolved(
            NestedPredicateObservationUnresolvedV1::Source(
                crate::mir::loop_structural_facts::NestedPredicateSourceUnresolvedV1::SourceLookup
            )
        )
    );
}
