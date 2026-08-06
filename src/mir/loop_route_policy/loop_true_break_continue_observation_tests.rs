use super::{
    issue_loop_true_family_observation_v1, LoopTrueFamilyObservationV1,
    LoopTrueObservationContextV1, LoopTrueObservationDeclineV1, LoopTrueObservationRejectV1,
    LoopTrueObservationUnresolvedV1,
};
use crate::ast::{ASTNode, LiteralValue, Span};
use crate::mir::compiler::loop_true_break_continue_observation::issue_loop_true_source_attempt_for_test;
use crate::mir::compiler::VerifiedResolvedSourceUnitV1;
use crate::mir::loop_structural_facts::{
    LoopTrueObservationCoverageV1, LoopTrueObservationModeV1, LoopTrueSourceAttemptOutcomeV1,
    LoopTrueSourceIdentityV1, LoopTrueSourceRejectV1, LoopTrueSourceUnresolvedV1,
    VerifiedLoopTrueSourceAttemptV1,
};
use crate::mir::resolved_semantics::FunctionOriginV1;

fn boolean(value: bool) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Bool(value),
        span: Span::unknown(),
    }
}

fn prepared(
    tree: ASTNode,
    attempt_mode: Option<LoopTrueObservationModeV1>,
    attempt_coverage: LoopTrueObservationCoverageV1,
    context_mode: Option<LoopTrueObservationModeV1>,
    context_coverage: LoopTrueObservationCoverageV1,
) -> (
    VerifiedLoopTrueSourceAttemptV1,
    LoopTrueObservationContextV1,
) {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(tree).expect("fixture resolves");
    let input = unit.root_function_input().expect("function input");
    let body = input.source().root_body().expect("function body");
    let root = input.source().body_stmt(&body, 1).expect("root loop");
    let source = input
        .function()
        .resolved_loop_source(root.site())
        .expect("root source");
    let identity = LoopTrueSourceIdentityV1::new(
        input.owner(),
        input.function().function_origin(),
        input.function().source_kind(),
        root.site().clone(),
        source.frame_key(),
    );
    let attempt = issue_loop_true_source_attempt_for_test(
        input,
        root,
        source,
        attempt_mode,
        attempt_coverage,
    );
    let context = LoopTrueObservationContextV1::for_test(identity, context_mode, context_coverage);
    (attempt, context)
}

#[test]
fn exact_loop_true_projection_is_candidate_in_all_sealed_modes() {
    for mode in [
        LoopTrueObservationModeV1::Release,
        LoopTrueObservationModeV1::Strict,
        LoopTrueObservationModeV1::StrictPlannerRequired,
    ] {
        let (attempt, context) = prepared(
            crate::mir::compiler::loop_true_function_for_test(),
            Some(mode),
            LoopTrueObservationCoverageV1::Complete,
            Some(mode),
            LoopTrueObservationCoverageV1::Complete,
        );
        let LoopTrueFamilyObservationV1::Candidate(candidate) =
            issue_loop_true_family_observation_v1(attempt, context)
        else {
            panic!("exact LoopTrue projection must be a candidate")
        };
        assert_eq!(candidate.context().mode(), Some(mode));
        assert_eq!(
            candidate.observation().root_frame_key(),
            candidate.context().identity().frame()
        );
    }
}

#[test]
fn known_non_loop_true_shape_declines_without_route_fallback() {
    let mut tree = crate::mir::compiler::loop_true_function_for_test();
    let ASTNode::FunctionDeclaration { body, .. } = &mut tree else {
        panic!("fixture root")
    };
    let ASTNode::Loop { condition, .. } = &mut body[1] else {
        panic!("loop root")
    };
    *condition = Box::new(boolean(false));
    let (attempt, context) = prepared(
        tree,
        Some(LoopTrueObservationModeV1::Release),
        LoopTrueObservationCoverageV1::Complete,
        Some(LoopTrueObservationModeV1::Release),
        LoopTrueObservationCoverageV1::Complete,
    );
    assert_eq!(
        issue_loop_true_family_observation_v1(attempt, context),
        LoopTrueFamilyObservationV1::Declined(
            LoopTrueObservationDeclineV1::NotLoopTrueBreakContinueShape
        )
    );
}

#[test]
fn incomplete_window_is_unresolved_before_shape_disposition() {
    let (attempt, context) = prepared(
        crate::mir::compiler::loop_true_function_for_test(),
        Some(LoopTrueObservationModeV1::Release),
        LoopTrueObservationCoverageV1::Complete,
        Some(LoopTrueObservationModeV1::Release),
        LoopTrueObservationCoverageV1::Incomplete,
    );
    assert_eq!(
        issue_loop_true_family_observation_v1(attempt, context),
        LoopTrueFamilyObservationV1::Unresolved(
            LoopTrueObservationUnresolvedV1::IncompleteCoverage
        )
    );
}

#[test]
fn unsealed_mode_is_unresolved_without_policy_guess() {
    let (attempt, context) = prepared(
        crate::mir::compiler::loop_true_function_for_test(),
        None,
        LoopTrueObservationCoverageV1::Complete,
        None,
        LoopTrueObservationCoverageV1::Complete,
    );
    assert_eq!(
        issue_loop_true_family_observation_v1(attempt, context),
        LoopTrueFamilyObservationV1::Unresolved(LoopTrueObservationUnresolvedV1::ModeUnsealed)
    );
}

#[test]
fn mode_mismatch_is_rejected_before_candidate_issue() {
    let (attempt, context) = prepared(
        crate::mir::compiler::loop_true_function_for_test(),
        Some(LoopTrueObservationModeV1::Release),
        LoopTrueObservationCoverageV1::Complete,
        Some(LoopTrueObservationModeV1::Strict),
        LoopTrueObservationCoverageV1::Complete,
    );
    assert_eq!(
        issue_loop_true_family_observation_v1(attempt, context),
        LoopTrueFamilyObservationV1::Rejected(LoopTrueObservationRejectV1::ModeMismatch)
    );
}

#[test]
fn foreign_context_is_rejected_before_shape_policy() {
    let (attempt, context) = prepared(
        crate::mir::compiler::loop_true_function_for_test(),
        Some(LoopTrueObservationModeV1::Release),
        LoopTrueObservationCoverageV1::Complete,
        Some(LoopTrueObservationModeV1::Release),
        LoopTrueObservationCoverageV1::Complete,
    );
    let foreign = crate::mir::resolved_semantics::FunctionOwnerIssuerV1::new_for_compilation()
        .expect("owner issuer")
        .issue()
        .expect("foreign owner");
    let identity = LoopTrueSourceIdentityV1::new(
        foreign,
        context.identity().function_origin(),
        context.identity().source_kind(),
        context.identity().site().clone(),
        context.identity().frame().clone(),
    );
    let foreign_context =
        LoopTrueObservationContextV1::for_test(identity, context.mode(), context.coverage());
    assert_eq!(
        issue_loop_true_family_observation_v1(attempt, foreign_context),
        LoopTrueFamilyObservationV1::Rejected(LoopTrueObservationRejectV1::ForeignContext)
    );
}

#[test]
fn source_lookup_remains_unresolved() {
    let (attempt, context) = prepared(
        crate::mir::compiler::loop_true_function_for_test(),
        Some(LoopTrueObservationModeV1::Release),
        LoopTrueObservationCoverageV1::Complete,
        Some(LoopTrueObservationModeV1::Release),
        LoopTrueObservationCoverageV1::Complete,
    );
    let (_, identity, mode, coverage) = attempt.into_parts();
    let attempt = VerifiedLoopTrueSourceAttemptV1::new(
        LoopTrueSourceAttemptOutcomeV1::Unresolved(LoopTrueSourceUnresolvedV1::SourceLookup),
        identity,
        mode,
        coverage,
    );
    assert_eq!(
        issue_loop_true_family_observation_v1(attempt, context),
        LoopTrueFamilyObservationV1::Unresolved(LoopTrueObservationUnresolvedV1::Source(
            LoopTrueSourceUnresolvedV1::SourceLookup,
        ))
    );
}

#[test]
fn candidate_identity_is_rechecked_after_context_seal() {
    let (attempt, context) = prepared(
        crate::mir::compiler::loop_true_function_for_test(),
        Some(LoopTrueObservationModeV1::Release),
        LoopTrueObservationCoverageV1::Complete,
        Some(LoopTrueObservationModeV1::Release),
        LoopTrueObservationCoverageV1::Complete,
    );
    let (outcome, old_identity, mode, coverage) = attempt.into_parts();
    let identity = LoopTrueSourceIdentityV1::new(
        old_identity.owner(),
        FunctionOriginV1::new(999, 999),
        old_identity.source_kind(),
        old_identity.site().clone(),
        old_identity.frame().clone(),
    );
    let attempt = VerifiedLoopTrueSourceAttemptV1::new(outcome, identity, mode, coverage);
    let context = LoopTrueObservationContextV1::for_test(
        attempt_identity(&attempt),
        context.mode(),
        context.coverage(),
    );
    assert_eq!(
        issue_loop_true_family_observation_v1(attempt, context),
        LoopTrueFamilyObservationV1::Rejected(
            LoopTrueObservationRejectV1::CandidateIdentityMismatch
        )
    );
}

fn attempt_identity(attempt: &VerifiedLoopTrueSourceAttemptV1) -> LoopTrueSourceIdentityV1 {
    attempt.identity().clone()
}

#[test]
fn exit_target_conflict_is_rejected() {
    let (attempt, context) = prepared(
        crate::mir::compiler::loop_true_function_for_test(),
        Some(LoopTrueObservationModeV1::Release),
        LoopTrueObservationCoverageV1::Complete,
        Some(LoopTrueObservationModeV1::Release),
        LoopTrueObservationCoverageV1::Complete,
    );
    let (_, identity, mode, coverage) = attempt.into_parts();
    let attempt = VerifiedLoopTrueSourceAttemptV1::new(
        LoopTrueSourceAttemptOutcomeV1::Rejected(LoopTrueSourceRejectV1::ExitTargetMismatch),
        identity,
        mode,
        coverage,
    );
    assert_eq!(
        issue_loop_true_family_observation_v1(attempt, context),
        LoopTrueFamilyObservationV1::Rejected(LoopTrueObservationRejectV1::Source(
            LoopTrueSourceRejectV1::ExitTargetMismatch,
        ))
    );
}
