use super::{
    issue_loop_cond_family_observation_v1, LoopCondFamilyObservationV1,
    LoopCondObservationContextV1, LoopCondObservationDeclineV1, LoopCondObservationRejectV1,
    LoopCondObservationUnresolvedV1,
};
use crate::ast::{ASTNode, LiteralValue, Span};
use crate::mir::compiler::loop_cond_break_continue_observation::issue_loop_cond_source_attempt_for_test;
use crate::mir::compiler::VerifiedResolvedSourceUnitV1;
use crate::mir::loop_structural_facts::{
    LoopCondObservationCoverageV1, LoopCondObservationModeV1, LoopCondSourceAttemptOutcomeV1,
    LoopCondSourceIdentityV1, LoopCondSourceRejectV1, LoopCondSourceUnresolvedV1,
    VerifiedLoopCondSourceAttemptV1,
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
    attempt_mode: Option<LoopCondObservationModeV1>,
    attempt_coverage: LoopCondObservationCoverageV1,
    context_mode: Option<LoopCondObservationModeV1>,
    context_coverage: LoopCondObservationCoverageV1,
) -> (
    VerifiedLoopCondSourceAttemptV1,
    LoopCondObservationContextV1,
) {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(tree).expect("fixture resolves");
    let input = unit.root_function_input().expect("function input");
    let body = input.source().root_body().expect("function body");
    let root = input.source().body_stmt(&body, 1).expect("root loop");
    let source = input
        .function()
        .resolved_loop_source(root.site())
        .expect("root source");
    let identity = LoopCondSourceIdentityV1::new(
        input.owner(),
        input.function().function_origin(),
        input.function().source_kind(),
        root.site().clone(),
        source.frame_key(),
    );
    let attempt = issue_loop_cond_source_attempt_for_test(
        input,
        root,
        source,
        attempt_mode,
        attempt_coverage,
    );
    let context = LoopCondObservationContextV1::for_test(identity, context_mode, context_coverage);
    (attempt, context)
}

#[test]
fn exact_loop_cond_projection_is_candidate_in_all_sealed_modes() {
    for mode in [
        LoopCondObservationModeV1::Release,
        LoopCondObservationModeV1::Strict,
        LoopCondObservationModeV1::StrictPlannerRequired,
    ] {
        let (attempt, context) = prepared(
            crate::mir::compiler::loop_cond_function_for_test(),
            Some(mode),
            LoopCondObservationCoverageV1::Complete,
            Some(mode),
            LoopCondObservationCoverageV1::Complete,
        );
        let LoopCondFamilyObservationV1::Candidate(candidate) =
            issue_loop_cond_family_observation_v1(attempt, context)
        else {
            panic!("exact LoopCond projection must be a candidate")
        };
        assert_eq!(candidate.context().mode(), Some(mode));
        assert_eq!(
            candidate.observation().root_frame_key(),
            candidate.context().identity().frame()
        );
    }
}

#[test]
fn known_loop_true_overlap_declines_without_route_fallback() {
    let mut tree = crate::mir::compiler::loop_cond_function_for_test();
    let ASTNode::FunctionDeclaration { body, .. } = &mut tree else {
        panic!("fixture root")
    };
    let ASTNode::Loop { condition, .. } = &mut body[1] else {
        panic!("loop root")
    };
    *condition = Box::new(boolean(true));
    let (attempt, context) = prepared(
        tree,
        Some(LoopCondObservationModeV1::Release),
        LoopCondObservationCoverageV1::Complete,
        Some(LoopCondObservationModeV1::Release),
        LoopCondObservationCoverageV1::Complete,
    );
    assert_eq!(
        issue_loop_cond_family_observation_v1(attempt, context),
        LoopCondFamilyObservationV1::Declined(
            LoopCondObservationDeclineV1::NotLoopCondBreakContinueShape
        )
    );
}

#[test]
fn incomplete_window_is_unresolved_before_shape_disposition() {
    let (attempt, context) = prepared(
        crate::mir::compiler::loop_cond_function_for_test(),
        Some(LoopCondObservationModeV1::Release),
        LoopCondObservationCoverageV1::Complete,
        Some(LoopCondObservationModeV1::Release),
        LoopCondObservationCoverageV1::Incomplete,
    );
    assert_eq!(
        issue_loop_cond_family_observation_v1(attempt, context),
        LoopCondFamilyObservationV1::Unresolved(
            LoopCondObservationUnresolvedV1::IncompleteCoverage
        )
    );
}

#[test]
fn unsealed_mode_is_unresolved_without_policy_guess() {
    let (attempt, context) = prepared(
        crate::mir::compiler::loop_cond_function_for_test(),
        None,
        LoopCondObservationCoverageV1::Complete,
        None,
        LoopCondObservationCoverageV1::Complete,
    );
    assert_eq!(
        issue_loop_cond_family_observation_v1(attempt, context),
        LoopCondFamilyObservationV1::Unresolved(LoopCondObservationUnresolvedV1::ModeUnsealed)
    );
}

#[test]
fn mode_mismatch_is_rejected_before_candidate_issue() {
    let (attempt, context) = prepared(
        crate::mir::compiler::loop_cond_function_for_test(),
        Some(LoopCondObservationModeV1::Release),
        LoopCondObservationCoverageV1::Complete,
        Some(LoopCondObservationModeV1::Strict),
        LoopCondObservationCoverageV1::Complete,
    );
    assert_eq!(
        issue_loop_cond_family_observation_v1(attempt, context),
        LoopCondFamilyObservationV1::Rejected(LoopCondObservationRejectV1::ModeMismatch)
    );
}

#[test]
fn foreign_context_is_rejected_before_shape_policy() {
    let (attempt, context) = prepared(
        crate::mir::compiler::loop_cond_function_for_test(),
        Some(LoopCondObservationModeV1::Release),
        LoopCondObservationCoverageV1::Complete,
        Some(LoopCondObservationModeV1::Release),
        LoopCondObservationCoverageV1::Complete,
    );
    let foreign = crate::mir::resolved_semantics::FunctionOwnerIssuerV1::new_for_compilation()
        .expect("owner issuer")
        .issue()
        .expect("foreign owner");
    let identity = LoopCondSourceIdentityV1::new(
        foreign,
        context.identity().function_origin(),
        context.identity().source_kind(),
        context.identity().site().clone(),
        context.identity().frame().clone(),
    );
    let foreign_context =
        LoopCondObservationContextV1::for_test(identity, context.mode(), context.coverage());
    assert_eq!(
        issue_loop_cond_family_observation_v1(attempt, foreign_context),
        LoopCondFamilyObservationV1::Rejected(LoopCondObservationRejectV1::ForeignContext)
    );
}

#[test]
fn source_lookup_remains_unresolved() {
    let (attempt, context) = prepared(
        crate::mir::compiler::loop_cond_function_for_test(),
        Some(LoopCondObservationModeV1::Release),
        LoopCondObservationCoverageV1::Complete,
        Some(LoopCondObservationModeV1::Release),
        LoopCondObservationCoverageV1::Complete,
    );
    let (_, identity, mode, coverage) = attempt.into_parts();
    let attempt = VerifiedLoopCondSourceAttemptV1::new(
        LoopCondSourceAttemptOutcomeV1::Unresolved(LoopCondSourceUnresolvedV1::SourceLookup),
        identity,
        mode,
        coverage,
    );
    assert_eq!(
        issue_loop_cond_family_observation_v1(attempt, context),
        LoopCondFamilyObservationV1::Unresolved(LoopCondObservationUnresolvedV1::Source(
            LoopCondSourceUnresolvedV1::SourceLookup,
        ))
    );
}

#[test]
fn candidate_identity_is_rechecked_after_context_seal() {
    let (attempt, context) = prepared(
        crate::mir::compiler::loop_cond_function_for_test(),
        Some(LoopCondObservationModeV1::Release),
        LoopCondObservationCoverageV1::Complete,
        Some(LoopCondObservationModeV1::Release),
        LoopCondObservationCoverageV1::Complete,
    );
    let (outcome, old_identity, mode, coverage) = attempt.into_parts();
    let identity = LoopCondSourceIdentityV1::new(
        old_identity.owner(),
        FunctionOriginV1::new(999, 999),
        old_identity.source_kind(),
        old_identity.site().clone(),
        old_identity.frame().clone(),
    );
    let attempt = VerifiedLoopCondSourceAttemptV1::new(outcome, identity, mode, coverage);
    let context = LoopCondObservationContextV1::for_test(
        attempt_identity(&attempt),
        context.mode(),
        context.coverage(),
    );
    assert_eq!(
        issue_loop_cond_family_observation_v1(attempt, context),
        LoopCondFamilyObservationV1::Rejected(
            LoopCondObservationRejectV1::CandidateIdentityMismatch
        )
    );
}

fn attempt_identity(attempt: &VerifiedLoopCondSourceAttemptV1) -> LoopCondSourceIdentityV1 {
    attempt.identity().clone()
}

#[test]
fn exit_target_conflict_is_rejected() {
    let (attempt, context) = prepared(
        crate::mir::compiler::loop_cond_function_for_test(),
        Some(LoopCondObservationModeV1::Release),
        LoopCondObservationCoverageV1::Complete,
        Some(LoopCondObservationModeV1::Release),
        LoopCondObservationCoverageV1::Complete,
    );
    let (_, identity, mode, coverage) = attempt.into_parts();
    let attempt = VerifiedLoopCondSourceAttemptV1::new(
        LoopCondSourceAttemptOutcomeV1::Rejected(LoopCondSourceRejectV1::ExitTargetMismatch),
        identity,
        mode,
        coverage,
    );
    assert_eq!(
        issue_loop_cond_family_observation_v1(attempt, context),
        LoopCondFamilyObservationV1::Rejected(LoopCondObservationRejectV1::Source(
            LoopCondSourceRejectV1::ExitTargetMismatch,
        ))
    );
}
