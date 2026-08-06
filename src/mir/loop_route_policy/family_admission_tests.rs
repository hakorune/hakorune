use super::{
    assemble_loop_family_admission_window_v1, DirectAccumFamilyObservationV1,
    issue_direct_accum_family_observation_v1, issue_generic_g0_family_observation_v1,
    issue_loop_cond_family_observation_v1, issue_loop_true_family_observation_v1,
    issue_nested_predicate_family_observation_v1, DirectAccumObservationContextV1,
    GenericG0ObservationContextV1, LoopCondObservationContextV1, LoopTrueObservationContextV1,
    NestedPredicateObservationContextV1, LoopFamilyAdmissionAssemblyOutcomeV1,
    LoopFamilyAdmissionIssueV1, LoopFamilyAdmissionModeV1, LoopFamilyObservationRowV1,
    LoopFamilyTagV1,
};
use crate::mir::compiler::direct_accum_observation::issue_direct_accum_source_attempt_for_test;
use crate::mir::compiler::direct_accum_projection::direct_accum_function_for_test;
use crate::mir::compiler::VerifiedResolvedSourceUnitV1;
use crate::mir::loop_structural_facts::{
    DirectAccumObservationCoverageV1, DirectAccumObservationModeV1,
    DirectAccumSourceAttemptOutcomeV1, DirectAccumSourceDeclineV1, DirectAccumSourceIdentityV1,
    GenericG0ObservationCoverageV1, GenericG0ObservationModeV1,
    GenericG0SourceAttemptOutcomeV1, GenericG0SourceDeclineV1, GenericG0SourceIdentityV1,
    LoopCondObservationCoverageV1, LoopCondObservationModeV1,
    LoopCondSourceAttemptOutcomeV1, LoopCondSourceDeclineV1, LoopCondSourceIdentityV1,
    LoopTrueObservationCoverageV1, LoopTrueObservationModeV1,
    LoopTrueSourceAttemptOutcomeV1, LoopTrueSourceDeclineV1, LoopTrueSourceIdentityV1,
    NestedPredicateObservationCoverageV1,
    NestedPredicateObservationModeV1, NestedPredicateSourceAttemptOutcomeV1,
    NestedPredicateSourceDeclineV1, NestedPredicateSourceIdentityV1,
    VerifiedDirectAccumSourceAttemptV1, VerifiedGenericG0SourceAttemptV1,
    VerifiedLoopCondSourceAttemptV1, VerifiedLoopTrueSourceAttemptV1,
    VerifiedNestedPredicateSourceAttemptV1,
};
use crate::mir::resolved_semantics::{
    FunctionOriginV1, FunctionOwnerIdV1, FunctionOwnerIssuerV1, LoopExecutionFrameKeyV1,
    SemanticOwnerSourceKindV1, SourceStmtSiteV1, VerifiedLoopFamilyWindowLeaseV1,
};

#[derive(Clone)]
struct FixtureIdentity {
    owner: FunctionOwnerIdV1,
    origin: FunctionOriginV1,
    source_kind: SemanticOwnerSourceKindV1,
    site: SourceStmtSiteV1,
    frame: LoopExecutionFrameKeyV1,
}

fn fixture() -> (VerifiedLoopFamilyWindowLeaseV1, FixtureIdentity) {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(direct_accum_function_for_test())
        .expect("fixture resolves");
    let input = unit.root_function_input().expect("root input");
    let body = input.source().root_body().expect("root body");
    let loop_stmt = input.source().body_stmt(&body, 1).expect("root loop");
    let lease = input
        .function()
        .issue_loop_family_window_lease_v1(loop_stmt.site())
        .expect("lease");
    let identity = FixtureIdentity {
        owner: lease.owner(),
        origin: lease.function_origin(),
        source_kind: lease.source_kind(),
        site: lease.site().clone(),
        frame: lease.frame(),
    };
    (lease, identity)
}

fn candidate_fixture() -> (
    VerifiedLoopFamilyWindowLeaseV1,
    FixtureIdentity,
    LoopFamilyObservationRowV1,
) {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(direct_accum_function_for_test())
        .expect("fixture resolves");
    let input = unit.root_function_input().expect("root input");
    let body = input.source().root_body().expect("root body");
    let loop_stmt = input.source().body_stmt(&body, 1).expect("root loop");
    let source = input
        .function()
        .resolved_loop_source(loop_stmt.site())
        .expect("source");
    let lease = input
        .function()
        .issue_loop_family_window_lease_v1(loop_stmt.site())
        .expect("lease");
    let identity = FixtureIdentity {
        owner: lease.owner(),
        origin: lease.function_origin(),
        source_kind: lease.source_kind(),
        site: lease.site().clone(),
        frame: lease.frame(),
    };
    let source_identity = DirectAccumSourceIdentityV1::new(
        identity.owner,
        identity.origin,
        identity.source_kind,
        identity.site.clone(),
        identity.frame.clone(),
    );
    let attempt = issue_direct_accum_source_attempt_for_test(
        input,
        loop_stmt,
        source,
        Some(DirectAccumObservationModeV1::Release),
        DirectAccumObservationCoverageV1::Complete,
    );
    let context = DirectAccumObservationContextV1::for_test(
        source_identity,
        Some(DirectAccumObservationModeV1::Release),
        DirectAccumObservationCoverageV1::Complete,
    );
    let row = issue_direct_accum_family_observation_v1(attempt, context).into_admission_row();
    (lease, identity, row)
}

fn direct_declined(
    expected: &FixtureIdentity,
    observed: &FixtureIdentity,
    mode: Option<DirectAccumObservationModeV1>,
) -> LoopFamilyObservationRowV1 {
    let attempt = VerifiedDirectAccumSourceAttemptV1::new(
        DirectAccumSourceAttemptOutcomeV1::Declined(DirectAccumSourceDeclineV1::NotDirectAccumShape),
        DirectAccumSourceIdentityV1::new(
            observed.owner,
            observed.origin,
            observed.source_kind,
            observed.site.clone(),
            observed.frame.clone(),
        ),
        mode,
        DirectAccumObservationCoverageV1::Complete,
    );
    let context = DirectAccumObservationContextV1::for_test(
        DirectAccumSourceIdentityV1::new(
            expected.owner,
            expected.origin,
            expected.source_kind,
            expected.site.clone(),
            expected.frame.clone(),
        ),
        mode,
        DirectAccumObservationCoverageV1::Complete,
    );
    issue_direct_accum_family_observation_v1(attempt, context).into_admission_row()
}

fn nested_declined(identity: &FixtureIdentity) -> LoopFamilyObservationRowV1 {
    let source_identity = NestedPredicateSourceIdentityV1::new(
        identity.owner,
        identity.origin,
        identity.source_kind,
        identity.site.clone(),
        identity.frame.clone(),
    );
    let attempt = VerifiedNestedPredicateSourceAttemptV1::new(
        NestedPredicateSourceAttemptOutcomeV1::Declined(
            NestedPredicateSourceDeclineV1::NotNestedPredicateShape,
        ),
        source_identity.clone(),
        Some(NestedPredicateObservationModeV1::Release),
        NestedPredicateObservationCoverageV1::Complete,
    );
    let context = NestedPredicateObservationContextV1::for_test(
        source_identity,
        Some(NestedPredicateObservationModeV1::Release),
        NestedPredicateObservationCoverageV1::Complete,
    );
    issue_nested_predicate_family_observation_v1(attempt, context).into_admission_row()
}

fn loop_true_declined(identity: &FixtureIdentity) -> LoopFamilyObservationRowV1 {
    let source_identity = LoopTrueSourceIdentityV1::new(
        identity.owner,
        identity.origin,
        identity.source_kind,
        identity.site.clone(),
        identity.frame.clone(),
    );
    let attempt = VerifiedLoopTrueSourceAttemptV1::new(
        LoopTrueSourceAttemptOutcomeV1::Declined(
            LoopTrueSourceDeclineV1::NotLoopTrueBreakContinueShape,
        ),
        source_identity.clone(),
        Some(LoopTrueObservationModeV1::Release),
        LoopTrueObservationCoverageV1::Complete,
    );
    let context = LoopTrueObservationContextV1::for_test(
        source_identity,
        Some(LoopTrueObservationModeV1::Release),
        LoopTrueObservationCoverageV1::Complete,
    );
    issue_loop_true_family_observation_v1(attempt, context).into_admission_row()
}

fn loop_cond_declined(identity: &FixtureIdentity) -> LoopFamilyObservationRowV1 {
    let source_identity = LoopCondSourceIdentityV1::new(
        identity.owner,
        identity.origin,
        identity.source_kind,
        identity.site.clone(),
        identity.frame.clone(),
    );
    let attempt = VerifiedLoopCondSourceAttemptV1::new(
        LoopCondSourceAttemptOutcomeV1::Declined(
            LoopCondSourceDeclineV1::NotLoopCondBreakContinueShape,
        ),
        source_identity.clone(),
        Some(LoopCondObservationModeV1::Release),
        LoopCondObservationCoverageV1::Complete,
    );
    let context = LoopCondObservationContextV1::for_test(
        source_identity,
        Some(LoopCondObservationModeV1::Release),
        LoopCondObservationCoverageV1::Complete,
    );
    issue_loop_cond_family_observation_v1(attempt, context).into_admission_row()
}

fn generic_declined(identity: &FixtureIdentity) -> LoopFamilyObservationRowV1 {
    let source_identity = GenericG0SourceIdentityV1::new(
        identity.owner,
        identity.origin,
        identity.source_kind,
        identity.site.clone(),
        identity.frame.clone(),
    );
    let attempt = VerifiedGenericG0SourceAttemptV1::new(
        GenericG0SourceAttemptOutcomeV1::Declined(GenericG0SourceDeclineV1::NotGenericG0Shape),
        source_identity.clone(),
        Some(GenericG0ObservationModeV1::Release),
        GenericG0ObservationCoverageV1::Complete,
    );
    let context = GenericG0ObservationContextV1::for_test(
        source_identity,
        Some(GenericG0ObservationModeV1::Release),
        GenericG0ObservationCoverageV1::Complete,
    );
    issue_generic_g0_family_observation_v1(attempt, context).into_admission_row()
}

fn all_declined(identity: &FixtureIdentity) -> Box<[LoopFamilyObservationRowV1]> {
    vec![
        generic_declined(identity),
        loop_cond_declined(identity),
        direct_declined(identity, identity, Some(DirectAccumObservationModeV1::Release)),
        nested_declined(identity),
        loop_true_declined(identity),
    ]
    .into_boxed_slice()
}

#[test]
fn exact_five_rows_are_canonicalized_without_candidate_counting() {
    let (lease, identity) = fixture();
    let outcome = assemble_loop_family_admission_window_v1(lease, all_declined(&identity));
    let LoopFamilyAdmissionAssemblyOutcomeV1::Ready(window) = outcome else {
        panic!("five complete declined rows must form a ready window")
    };
    assert_eq!(window.mode(), LoopFamilyAdmissionModeV1::Release);
    assert_eq!(window.coverage(), super::LoopFamilyAdmissionCoverageV1::Complete);
    assert!(matches!(
        window.rows().direct_accum(),
        DirectAccumFamilyObservationV1::Declined { .. }
    ));
    assert!(matches!(
        window.rows().generic_g0(),
        super::GenericG0FamilyObservationV1::Declined { .. }
    ));
}

#[test]
fn one_candidate_is_accepted_without_selector_overlap_logic() {
    let (lease, identity, direct_candidate) = candidate_fixture();
    let mut rows = all_declined(&identity).into_vec();
    rows[2] = direct_candidate;
    let outcome = assemble_loop_family_admission_window_v1(lease, rows.into_boxed_slice());
    let LoopFamilyAdmissionAssemblyOutcomeV1::Ready(window) = outcome else {
        panic!("one candidate plus four declines must be selector-ready")
    };
    assert!(matches!(
        window.rows().direct_accum(),
        DirectAccumFamilyObservationV1::Candidate(_)
    ));
}

#[test]
fn missing_row_is_unresolved_and_retains_lease_and_rows() {
    let (lease, identity) = fixture();
    let mut rows = all_declined(&identity).into_vec();
    rows.pop();
    let outcome = assemble_loop_family_admission_window_v1(lease, rows.into_boxed_slice());
    let LoopFamilyAdmissionAssemblyOutcomeV1::Unresolved(failure) = outcome else {
        panic!("missing family row must remain unresolved")
    };
    assert_eq!(failure.rows().len(), 4);
    assert_eq!(failure.lease().owner(), identity.owner);
    assert!(failure.issues().iter().any(|issue| matches!(
        issue,
        LoopFamilyAdmissionIssueV1::MissingFamilyObservation { .. }
    )));
}

#[test]
fn duplicate_tag_is_rejected_before_selection() {
    let (lease, identity) = fixture();
    let mut rows = all_declined(&identity).into_vec();
    rows[0] = direct_declined(
        &identity,
        &identity,
        Some(DirectAccumObservationModeV1::Release),
    );
    let outcome = assemble_loop_family_admission_window_v1(lease, rows.into_boxed_slice());
    let LoopFamilyAdmissionAssemblyOutcomeV1::Rejected(failure) = outcome else {
        panic!("duplicate family tag must be rejected")
    };
    assert!(failure.issues().iter().any(|issue| matches!(
        issue,
        LoopFamilyAdmissionIssueV1::DuplicateFamily(LoopFamilyTagV1::DirectAccum)
    )));
}

#[test]
fn foreign_observed_identity_is_rejected_and_retained() {
    let (lease, identity) = fixture();
    let foreign_owner = FunctionOwnerIssuerV1::new_for_compilation()
        .expect("owner issuer")
        .issue()
        .expect("foreign owner");
    let foreign = FixtureIdentity {
        owner: foreign_owner,
        ..identity.clone()
    };
    let mut rows = all_declined(&identity).into_vec();
    rows[2] = direct_declined(
        &identity,
        &foreign,
        Some(DirectAccumObservationModeV1::Release),
    );
    let outcome = assemble_loop_family_admission_window_v1(lease, rows.into_boxed_slice());
    let LoopFamilyAdmissionAssemblyOutcomeV1::Rejected(failure) = outcome else {
        panic!("foreign row identity must be rejected")
    };
    assert!(failure.issues().iter().any(|issue| matches!(
        issue,
        LoopFamilyAdmissionIssueV1::ForeignIdentity(LoopFamilyTagV1::DirectAccum)
    )));
}

#[test]
fn unsealed_mode_is_unresolved_without_guessing() {
    let (lease, identity) = fixture();
    let mut rows = all_declined(&identity).into_vec();
    rows[2] = direct_declined(&identity, &identity, None);
    let outcome = assemble_loop_family_admission_window_v1(lease, rows.into_boxed_slice());
    let LoopFamilyAdmissionAssemblyOutcomeV1::Unresolved(failure) = outcome else {
        panic!("unsealed mode must remain unresolved")
    };
    assert!(failure.issues().iter().any(|issue| matches!(
        issue,
        LoopFamilyAdmissionIssueV1::ModeUnsealed(LoopFamilyTagV1::DirectAccum)
    )));
}
