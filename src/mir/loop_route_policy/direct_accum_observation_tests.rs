use super::{
    issue_direct_accum_family_observation_v1, DirectAccumFamilyObservationV1,
    DirectAccumObservationContextV1, DirectAccumObservationDeclineV1,
    DirectAccumObservationRejectV1, DirectAccumObservationUnresolvedV1,
};
use crate::ast::ASTNode;
use crate::mir::compiler::direct_accum_observation::issue_direct_accum_source_attempt_for_test;
use crate::mir::compiler::direct_accum_projection::direct_accum_function_for_test;
use crate::mir::compiler::VerifiedResolvedSourceUnitV1;
use crate::mir::loop_structural_facts::{
    DirectAccumObservationCoverageV1, DirectAccumObservationModeV1, DirectAccumSourceIdentityV1,
};

fn prepared(
    tree: ASTNode,
    attempt_mode: Option<DirectAccumObservationModeV1>,
    attempt_coverage: DirectAccumObservationCoverageV1,
    context_mode: Option<DirectAccumObservationModeV1>,
    context_coverage: DirectAccumObservationCoverageV1,
) -> (
    crate::mir::loop_structural_facts::VerifiedDirectAccumSourceAttemptV1,
    DirectAccumObservationContextV1,
) {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(tree).expect("fixture resolves");
    let input = unit.root_function_input().expect("function input");
    let body = input.source().root_body().expect("root body");
    let loop_stmt = input.source().body_stmt(&body, 1).expect("loop statement");
    let source = input
        .function()
        .resolved_loop_source(loop_stmt.site())
        .expect("resolved loop source");
    let identity = DirectAccumSourceIdentityV1::new(
        input.owner(),
        input.function().function_origin(),
        input.function().source_kind(),
        loop_stmt.site().clone(),
        source.frame_key(),
    );
    let attempt = issue_direct_accum_source_attempt_for_test(
        input,
        loop_stmt,
        source,
        attempt_mode,
        attempt_coverage,
    );
    let context =
        DirectAccumObservationContextV1::for_test(identity, context_mode, context_coverage);
    (attempt, context)
}

#[test]
fn exact_direct_accum_is_candidate_in_all_sealed_modes() {
    for mode in [
        DirectAccumObservationModeV1::Release,
        DirectAccumObservationModeV1::Strict,
        DirectAccumObservationModeV1::StrictPlannerRequired,
    ] {
        let (attempt, context) = prepared(
            direct_accum_function_for_test(),
            Some(mode),
            DirectAccumObservationCoverageV1::Complete,
            Some(mode),
            DirectAccumObservationCoverageV1::Complete,
        );
        let DirectAccumFamilyObservationV1::Candidate(candidate) =
            issue_direct_accum_family_observation_v1(attempt, context)
        else {
            panic!("exact DirectAccum must be a candidate")
        };
        assert_eq!(candidate.context().mode(), Some(mode));
        assert_eq!(
            candidate.observation().frame_key(),
            candidate.context().identity().frame().clone()
        );
    }
}

#[test]
fn known_non_direct_shape_declines_without_family_fallback() {
    let mut tree = direct_accum_function_for_test();
    let ASTNode::FunctionDeclaration { body, .. } = &mut tree else {
        panic!("fixture root")
    };
    let ASTNode::Loop {
        body: loop_body, ..
    } = &mut body[1]
    else {
        panic!("fixture loop")
    };
    loop_body.pop();
    let (attempt, context) = prepared(
        tree,
        Some(DirectAccumObservationModeV1::Release),
        DirectAccumObservationCoverageV1::Complete,
        Some(DirectAccumObservationModeV1::Release),
        DirectAccumObservationCoverageV1::Complete,
    );
    assert_eq!(
        issue_direct_accum_family_observation_v1(attempt, context),
        DirectAccumFamilyObservationV1::Declined(
            DirectAccumObservationDeclineV1::NotDirectAccumShape
        )
    );
}

#[test]
fn incomplete_window_is_unresolved_before_shape_disposition() {
    let (attempt, context) = prepared(
        direct_accum_function_for_test(),
        Some(DirectAccumObservationModeV1::Release),
        DirectAccumObservationCoverageV1::Complete,
        Some(DirectAccumObservationModeV1::Release),
        DirectAccumObservationCoverageV1::Incomplete,
    );
    assert_eq!(
        issue_direct_accum_family_observation_v1(attempt, context),
        DirectAccumFamilyObservationV1::Unresolved(
            DirectAccumObservationUnresolvedV1::IncompleteCoverage
        )
    );
}

#[test]
fn unsealed_mode_is_unresolved_without_policy_guess() {
    let (attempt, context) = prepared(
        direct_accum_function_for_test(),
        None,
        DirectAccumObservationCoverageV1::Complete,
        None,
        DirectAccumObservationCoverageV1::Complete,
    );
    assert_eq!(
        issue_direct_accum_family_observation_v1(attempt, context),
        DirectAccumFamilyObservationV1::Unresolved(
            DirectAccumObservationUnresolvedV1::ModeUnsealed
        )
    );
}

#[test]
fn mode_mismatch_is_rejected_before_candidate_issue() {
    let (attempt, context) = prepared(
        direct_accum_function_for_test(),
        Some(DirectAccumObservationModeV1::Release),
        DirectAccumObservationCoverageV1::Complete,
        Some(DirectAccumObservationModeV1::Strict),
        DirectAccumObservationCoverageV1::Complete,
    );
    assert_eq!(
        issue_direct_accum_family_observation_v1(attempt, context),
        DirectAccumFamilyObservationV1::Rejected(DirectAccumObservationRejectV1::ModeMismatch)
    );
}

#[test]
fn foreign_owner_is_rejected_before_shape_policy() {
    let (attempt, context) = prepared(
        direct_accum_function_for_test(),
        Some(DirectAccumObservationModeV1::Release),
        DirectAccumObservationCoverageV1::Complete,
        Some(DirectAccumObservationModeV1::Release),
        DirectAccumObservationCoverageV1::Complete,
    );
    let foreign = crate::mir::resolved_semantics::FunctionOwnerIssuerV1::new_for_compilation()
        .expect("owner issuer")
        .issue()
        .expect("foreign owner");
    let identity = DirectAccumSourceIdentityV1::new(
        foreign,
        context.identity().function_origin(),
        context.identity().source_kind(),
        context.identity().site().clone(),
        context.identity().frame().clone(),
    );
    let foreign_context =
        DirectAccumObservationContextV1::for_test(identity, context.mode(), context.coverage());
    assert_eq!(
        issue_direct_accum_family_observation_v1(attempt, foreign_context),
        DirectAccumFamilyObservationV1::Rejected(DirectAccumObservationRejectV1::ForeignContext)
    );
}

#[test]
fn source_reject_is_typed_without_legacy_route_import() {
    let mut tree = direct_accum_function_for_test();
    let ASTNode::FunctionDeclaration { body, .. } = &mut tree else {
        panic!("fixture root")
    };
    let ASTNode::Loop { condition, .. } = &mut body[1] else {
        panic!("fixture loop")
    };
    let ASTNode::BinaryOp { left, .. } = condition.as_mut() else {
        panic!("fixture condition")
    };
    *left = Box::new(ASTNode::Variable {
        name: "sum".into(),
        span: crate::ast::Span::unknown(),
    });
    let (attempt, context) = prepared(
        tree,
        Some(DirectAccumObservationModeV1::Release),
        DirectAccumObservationCoverageV1::Complete,
        Some(DirectAccumObservationModeV1::Release),
        DirectAccumObservationCoverageV1::Complete,
    );
    assert!(matches!(
        issue_direct_accum_family_observation_v1(attempt, context),
        DirectAccumFamilyObservationV1::Rejected(DirectAccumObservationRejectV1::Source(_))
    ));
}
