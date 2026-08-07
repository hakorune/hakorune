use super::family_admission_tests::{all_declined, candidate_fixture, fixture, FixtureIdentity};
use super::{
    assemble_loop_family_admission_window_v1, select_canonical_loop_family_v1,
    CanonicalLoopFamilySelectionOutcomeV1, CanonicalLoopFamilySelectionReasonV1,
    CanonicalLoopFamilySelectionV1, GenericG0FamilyObservationV1, GenericG0ObservationContextV1,
    LoopCondFamilyObservationV1, LoopCondObservationContextV1,
    LoopFamilyAdmissionAssemblyOutcomeV1, LoopFamilyAdmissionCoverageV1, LoopFamilyAdmissionModeV1,
    LoopFamilyObservationRowV1, LoopFamilyTagV1, LoopTrueFamilyObservationV1,
    LoopTrueObservationContextV1, NestedPredicateFamilyObservationV1,
    NestedPredicateObservationContextV1,
};
use crate::ast::ASTNode;
use crate::mir::compiler::generic_g0_observation::issue_generic_g0_source_attempt_with_window_for_test;
use crate::mir::compiler::loop_cond_break_continue_observation::issue_loop_cond_source_attempt_for_test;
use crate::mir::compiler::loop_true_break_continue_observation::issue_loop_true_source_attempt_for_test;
use crate::mir::compiler::nested_predicate_observation::issue_nested_predicate_source_attempt_for_test;
use crate::mir::compiler::VerifiedResolvedSourceUnitV1;
use crate::mir::loop_route_policy::{
    issue_loop_cond_family_observation_v1, issue_loop_true_family_observation_v1,
    issue_nested_predicate_family_observation_v1,
};
use crate::mir::loop_structural_facts::{
    LoopCondObservationCoverageV1, LoopCondObservationModeV1, LoopCondSourceIdentityV1,
    LoopTrueObservationCoverageV1, LoopTrueObservationModeV1, LoopTrueSourceIdentityV1,
    NestedPredicateObservationCoverageV1, NestedPredicateObservationModeV1,
    NestedPredicateSourceIdentityV1,
};
use crate::mir::resolved_semantics::VerifiedLoopFamilyWindowLeaseV1;

fn identity_from_lease(lease: &VerifiedLoopFamilyWindowLeaseV1) -> FixtureIdentity {
    FixtureIdentity {
        owner: lease.owner(),
        origin: lease.function_origin(),
        source_kind: lease.source_kind(),
        site: lease.site().clone(),
        frame: lease.frame(),
    }
}

fn nested_candidate_fixture() -> (
    VerifiedLoopFamilyWindowLeaseV1,
    FixtureIdentity,
    LoopFamilyObservationRowV1,
) {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(
        crate::mir::compiler::nested_function_for_p3_test(),
    )
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
    let identity = identity_from_lease(&lease);
    let source_identity = NestedPredicateSourceIdentityV1::new(
        identity.owner,
        identity.origin,
        identity.source_kind,
        identity.site.clone(),
        identity.frame.clone(),
    );
    let attempt = issue_nested_predicate_source_attempt_for_test(
        input,
        loop_stmt,
        source,
        Some(NestedPredicateObservationModeV1::Release),
        NestedPredicateObservationCoverageV1::Complete,
    );
    let row = issue_nested_predicate_family_observation_v1(
        attempt,
        NestedPredicateObservationContextV1::for_test(
            source_identity,
            Some(NestedPredicateObservationModeV1::Release),
            NestedPredicateObservationCoverageV1::Complete,
        ),
    )
    .into_admission_row();
    (lease, identity, row)
}

fn loop_true_candidate_fixture() -> (
    VerifiedLoopFamilyWindowLeaseV1,
    FixtureIdentity,
    LoopFamilyObservationRowV1,
) {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(
        crate::mir::compiler::loop_true_function_for_test(),
    )
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
    let identity = identity_from_lease(&lease);
    let source_identity = LoopTrueSourceIdentityV1::new(
        identity.owner,
        identity.origin,
        identity.source_kind,
        identity.site.clone(),
        identity.frame.clone(),
    );
    let attempt = issue_loop_true_source_attempt_for_test(
        input,
        loop_stmt,
        source,
        Some(LoopTrueObservationModeV1::Release),
        LoopTrueObservationCoverageV1::Complete,
    );
    let row = issue_loop_true_family_observation_v1(
        attempt,
        LoopTrueObservationContextV1::for_test(
            source_identity,
            Some(LoopTrueObservationModeV1::Release),
            LoopTrueObservationCoverageV1::Complete,
        ),
    )
    .into_admission_row();
    (lease, identity, row)
}

fn loop_cond_candidate_fixture() -> (
    VerifiedLoopFamilyWindowLeaseV1,
    FixtureIdentity,
    LoopFamilyObservationRowV1,
) {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(
        crate::mir::compiler::loop_cond_function_for_test(),
    )
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
    let identity = identity_from_lease(&lease);
    let source_identity = LoopCondSourceIdentityV1::new(
        identity.owner,
        identity.origin,
        identity.source_kind,
        identity.site.clone(),
        identity.frame.clone(),
    );
    let attempt = issue_loop_cond_source_attempt_for_test(
        input,
        loop_stmt,
        source,
        Some(LoopCondObservationModeV1::Release),
        LoopCondObservationCoverageV1::Complete,
    );
    let row = issue_loop_cond_family_observation_v1(
        attempt,
        LoopCondObservationContextV1::for_test(
            source_identity,
            Some(LoopCondObservationModeV1::Release),
            LoopCondObservationCoverageV1::Complete,
        ),
    )
    .into_admission_row();
    (lease, identity, row)
}

fn generic_candidate_fixture() -> (
    VerifiedLoopFamilyWindowLeaseV1,
    FixtureIdentity,
    LoopFamilyObservationRowV1,
) {
    let (_, lease, identity, row) = generic_candidate_fixture_with_unit();
    (lease, identity, row)
}

fn generic_candidate_fixture_with_unit() -> (
    VerifiedResolvedSourceUnitV1,
    VerifiedLoopFamilyWindowLeaseV1,
    FixtureIdentity,
    LoopFamilyObservationRowV1,
) {
    let source = r#"
function generic_g0(i: i64, j: i64): i64 {
    loop(i < 3) {
        loop(j < 3) {
            j = j + 1
        }
        i = i + 1
    }
    return j
}
"#;
    let program = crate::parser::NyashParser::parse_from_string(source).expect("fixture parses");
    let function = match program {
        ASTNode::Program { statements, .. } => statements
            .into_iter()
            .find(|node| matches!(node, ASTNode::FunctionDeclaration { .. }))
            .expect("function fixture"),
        _ => panic!("fixture is a program"),
    };
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(function).expect("resolves");
    let input = unit.root_function_input().expect("root input");
    let body = input.source().root_body().expect("root body");
    let loop_stmt = input.source().body_stmt(&body, 0).expect("root loop");
    let source = input
        .function()
        .resolved_loop_source(loop_stmt.site())
        .expect("source");
    let lease = input
        .function()
        .issue_loop_family_window_lease_v1(loop_stmt.site())
        .expect("lease");
    let identity = identity_from_lease(&lease);
    let source_identity = crate::mir::loop_structural_facts::GenericG0SourceIdentityV1::new(
        identity.owner,
        identity.origin,
        identity.source_kind,
        identity.site.clone(),
        identity.frame.clone(),
    );
    let attempt = issue_generic_g0_source_attempt_with_window_for_test(
        input,
        loop_stmt,
        source,
        &lease,
        crate::mir::numeric_substrate::NumericTarget::host(),
        Some(crate::mir::loop_structural_facts::GenericG0ObservationModeV1::Release),
        crate::mir::loop_structural_facts::GenericG0ObservationCoverageV1::Complete,
    );
    let row = super::issue_generic_g0_family_observation_v1(
        attempt,
        GenericG0ObservationContextV1::for_test(
            source_identity,
            Some(crate::mir::loop_structural_facts::GenericG0ObservationModeV1::Release),
            crate::mir::loop_structural_facts::GenericG0ObservationCoverageV1::Complete,
        ),
    )
    .into_admission_row();
    (unit, lease, identity, row)
}

/// Shared caller-zero Generic selection for downstream Recipe tests.  The
/// source projector and five-row admission window remain owned by this test
/// module; no production selector caller is introduced.
pub(crate) fn generic_selection_for_test() -> CanonicalLoopFamilySelectionV1 {
    let (_, selection) = generic_source_unit_and_selection_for_test();
    selection
}

/// Keep the resolver unit beside the selected candidate for compiler-side
/// ingress tests.  The selection owns only AST-free evidence; the returned
/// unit is the exact resolver view that issued that evidence.
pub(crate) fn generic_source_unit_and_selection_for_test() -> (
    VerifiedResolvedSourceUnitV1,
    CanonicalLoopFamilySelectionV1,
) {
    let (unit, lease, identity, candidate) = generic_candidate_fixture_with_unit();
    let mut rows = all_declined(&identity).into_vec();
    rows[0] = candidate;
    let window = match assemble_loop_family_admission_window_v1(lease, rows.into_boxed_slice()) {
        LoopFamilyAdmissionAssemblyOutcomeV1::Ready(window) => window,
        _ => panic!("generic candidate window must be ready"),
    };
    match select_canonical_loop_family_v1(window) {
        CanonicalLoopFamilySelectionOutcomeV1::Selected(selection) => (unit, selection),
        _ => panic!("generic candidate must be selected"),
    }
}

fn assert_selected(
    factory: fn() -> (
        VerifiedLoopFamilyWindowLeaseV1,
        FixtureIdentity,
        LoopFamilyObservationRowV1,
    ),
    slot: usize,
    expected: LoopFamilyTagV1,
) {
    let (lease, identity, candidate) = factory();
    let mut rows = all_declined(&identity).into_vec();
    rows[slot] = candidate;
    let window = match assemble_loop_family_admission_window_v1(lease, rows.into_boxed_slice()) {
        LoopFamilyAdmissionAssemblyOutcomeV1::Ready(window) => window,
        _ => panic!("one candidate plus four declines must be ready"),
    };
    match select_canonical_loop_family_v1(window) {
        CanonicalLoopFamilySelectionOutcomeV1::Selected(selection) => {
            assert_eq!(selection.candidate().tag(), expected);
            assert_eq!(selection.mode(), LoopFamilyAdmissionModeV1::Release);
            assert_eq!(
                selection.coverage(),
                LoopFamilyAdmissionCoverageV1::Complete
            );
            assert_eq!(selection.lease().owner(), identity.owner);
        }
        _ => panic!("one candidate must be selected"),
    }
}

#[test]
fn each_family_candidate_is_selected_from_a_ready_window() {
    assert_selected(candidate_fixture, 2, LoopFamilyTagV1::DirectAccum);
    assert_selected(
        nested_candidate_fixture,
        3,
        LoopFamilyTagV1::NestedPredicate,
    );
    assert_selected(
        loop_true_candidate_fixture,
        4,
        LoopFamilyTagV1::LoopTrueBreakContinue,
    );
    assert_selected(
        loop_cond_candidate_fixture,
        1,
        LoopFamilyTagV1::LoopCondBreakContinue,
    );
    assert_selected(generic_candidate_fixture, 0, LoopFamilyTagV1::GenericG0);
}

#[test]
fn five_declined_rows_are_unresolved_and_retain_all_evidence() {
    let (lease, identity) = fixture();
    let owner = identity.owner;
    let window = match assemble_loop_family_admission_window_v1(lease, all_declined(&identity)) {
        LoopFamilyAdmissionAssemblyOutcomeV1::Ready(window) => window,
        _ => panic!("five declined rows must be ready for selector"),
    };
    match select_canonical_loop_family_v1(window) {
        CanonicalLoopFamilySelectionOutcomeV1::Unresolved(failure) => {
            assert_eq!(
                failure.reason(),
                CanonicalLoopFamilySelectionReasonV1::OutOfWindow
            );
            assert_eq!(failure.lease().owner(), owner);
            assert!(matches!(
                failure.rows().direct_accum(),
                super::DirectAccumFamilyObservationV1::Declined { .. }
            ));
            assert!(matches!(
                failure.rows().nested_predicate(),
                NestedPredicateFamilyObservationV1::Declined { .. }
            ));
            assert!(matches!(
                failure.rows().loop_true(),
                LoopTrueFamilyObservationV1::Declined { .. }
            ));
            assert!(matches!(
                failure.rows().loop_cond(),
                LoopCondFamilyObservationV1::Declined { .. }
            ));
            assert!(matches!(
                failure.rows().generic_g0(),
                GenericG0FamilyObservationV1::Declined { .. }
            ));
        }
        _ => panic!("five declined rows must remain unresolved"),
    }
}

#[test]
fn overlap_rejects_without_dropping_the_consumed_window() {
    // The two candidates are genuine typed observer products. The test-only
    // transport seam below bypasses assembler identity checks solely to cover
    // selector cardinality; production input is still assembler Ready only.
    let (lease, identity, direct) = candidate_fixture();
    let (_, _, loop_cond) = loop_cond_candidate_fixture();
    let mut rows = all_declined(&identity).into_vec();
    rows[2] = direct;
    rows[1] = loop_cond;
    let generic = rows.remove(0);
    let loop_cond = rows.remove(0);
    let direct = rows.remove(0);
    let nested = rows.remove(0);
    let loop_true = rows.remove(0);
    let rows = super::VerifiedLoopFamilyAdmissionRowsV1::from_parts_for_test(
        match direct {
            LoopFamilyObservationRowV1::DirectAccum(row) => row,
            _ => unreachable!(),
        },
        match nested {
            LoopFamilyObservationRowV1::NestedPredicate(row) => row,
            _ => unreachable!(),
        },
        match loop_true {
            LoopFamilyObservationRowV1::LoopTrue(row) => row,
            _ => unreachable!(),
        },
        match loop_cond {
            LoopFamilyObservationRowV1::LoopCond(row) => row,
            _ => unreachable!(),
        },
        match generic {
            LoopFamilyObservationRowV1::GenericG0(row) => row,
            _ => unreachable!(),
        },
    );
    let window = super::VerifiedLoopFamilyAdmissionWindowV1::from_parts_for_test(
        lease,
        rows,
        LoopFamilyAdmissionModeV1::Release,
        LoopFamilyAdmissionCoverageV1::Complete,
    );
    match select_canonical_loop_family_v1(window) {
        CanonicalLoopFamilySelectionOutcomeV1::Rejected(failure) => {
            assert_eq!(
                failure.reason(),
                CanonicalLoopFamilySelectionReasonV1::Overlap
            );
            assert_eq!(failure.lease().owner(), identity.owner);
            assert!(matches!(
                failure.rows().direct_accum(),
                super::DirectAccumFamilyObservationV1::Candidate(_)
            ));
            assert!(matches!(
                failure.rows().loop_cond(),
                LoopCondFamilyObservationV1::Candidate(_)
            ));
        }
        _ => panic!("two candidates must reject as overlap"),
    }
}

#[test]
fn selected_generic_window_is_consumed_into_one_demand_lease() {
    let (lease, identity, generic) = generic_candidate_fixture();
    let mut rows = all_declined(&identity).into_vec();
    rows[0] = generic;
    let window = match assemble_loop_family_admission_window_v1(lease, rows.into_boxed_slice()) {
        LoopFamilyAdmissionAssemblyOutcomeV1::Ready(window) => window,
        _ => panic!("one Generic candidate plus four declines must be ready"),
    };
    let selection = match select_canonical_loop_family_v1(window) {
        CanonicalLoopFamilySelectionOutcomeV1::Selected(selection) => selection,
        _ => panic!("one Generic candidate must be selected"),
    };
    let demand = crate::mir::loop_recipe_contract::issue_generic_g0_recipe_demand_v1(selection)
        .expect("selected Generic candidate must produce the caller-zero demand");
    let (demand_lease, brand, bundle, post_loop_read, profile, mode, coverage, _role_lease) =
        demand.into_parts();
    assert_eq!(demand_lease.owner(), identity.owner);
    assert_eq!(demand_lease.site(), &identity.site);
    assert!(brand.matches_window(&demand_lease));
    assert_eq!(profile, super::GenericG0PolicyProfileV1::G0);
    assert_eq!(mode, LoopFamilyAdmissionModeV1::Release);
    assert_eq!(coverage, LoopFamilyAdmissionCoverageV1::Complete);
    assert_eq!(
        bundle.source().structural().root_loop(),
        demand_lease.site()
    );
    assert_eq!(
        post_loop_read.binding(),
        bundle.source().structural().tail().binding
    );
}

#[test]
fn demand_rejects_a_selected_non_generic_family() {
    let (lease, identity, direct) = candidate_fixture();
    let mut rows = all_declined(&identity).into_vec();
    rows[2] = direct;
    let window = match assemble_loop_family_admission_window_v1(lease, rows.into_boxed_slice()) {
        LoopFamilyAdmissionAssemblyOutcomeV1::Ready(window) => window,
        _ => panic!("one DirectAccum candidate plus four declines must be ready"),
    };
    let selection = match select_canonical_loop_family_v1(window) {
        CanonicalLoopFamilySelectionOutcomeV1::Selected(selection) => selection,
        _ => panic!("one DirectAccum candidate must be selected"),
    };
    assert_eq!(
        crate::mir::loop_recipe_contract::issue_generic_g0_recipe_demand_v1(selection),
        Err(
            crate::mir::loop_recipe_contract::GenericG0RecipeDemandIssueV1::SelectedOtherFamily(
                LoopFamilyTagV1::DirectAccum,
            )
        )
    );
}
