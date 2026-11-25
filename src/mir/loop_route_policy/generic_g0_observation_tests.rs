use super::generic_g0_observation::{
    issue_generic_g0_family_observation_v1, GenericG0FamilyObservationV1,
    GenericG0ObservationContextV1, GenericG0ObservationRejectV1, GenericG0ObservationUnresolvedV1,
};
use crate::ast::ASTNode;
use crate::mir::compiler::generic_g0_observation::issue_generic_g0_source_attempt_v1;
use crate::mir::compiler::VerifiedResolvedSourceUnitV1;
use crate::mir::loop_structural_facts::{
    GenericG0ObservationCoverageV1, GenericG0ObservationModeV1,
};
use crate::mir::numeric_substrate::NumericTarget;
use crate::parser::NyashParser;

const TYPED: &str = r#"
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

fn parse_function(source: &str) -> ASTNode {
    let program = NyashParser::parse_from_string(source).expect("fixture parses");
    let ASTNode::Program { statements, .. } = program else {
        panic!("fixture must produce a Program")
    };
    statements
        .into_iter()
        .find(|node| matches!(node, ASTNode::FunctionDeclaration { .. }))
        .expect("fixture function")
}

pub(crate) fn generic_g0_source_unit_for_test() -> VerifiedResolvedSourceUnitV1 {
    VerifiedResolvedSourceUnitV1::resolve_function(parse_function(TYPED)).expect("resolve fixture")
}

pub(crate) fn generic_g0_observation_for_test(
    unit: &VerifiedResolvedSourceUnitV1,
) -> (
    super::VerifiedGenericFamilyObservationG0,
    crate::mir::resolved_semantics::VerifiedLoopFamilyWindowLeaseV1,
) {
    use super::{
        issue_generic_g0_candidate_v1, GenericG0CoverageV1, GenericG0PolicyContextV1,
        GenericG0PolicyModeV1, GenericG0PolicyOutcomeV1, GenericG0PolicyProfileV1,
    };
    use crate::mir::compiler::generic_g0_projection::handoff::issue_generic_g0_policy_handoff_with_window_v1;

    let input = unit.root_function_input().expect("root input");
    let body = input.source().root_body().expect("root body");
    let root_loop = input.source().body_stmt(&body, 0).expect("root loop");
    let lease = input
        .function()
        .issue_loop_family_window_lease_v1(root_loop.site())
        .expect("resolver lease");
    let handoff = issue_generic_g0_policy_handoff_with_window_v1(
        input, &lease, NumericTarget::host(),
    )
    .expect("source handoff");
    let context = GenericG0PolicyContextV1::from_observation(
        input.owner(),
        GenericG0PolicyProfileV1::G0,
        GenericG0PolicyModeV1::Release,
        GenericG0CoverageV1::Complete,
    );
    let GenericG0PolicyOutcomeV1::Candidate(observation) =
        issue_generic_g0_candidate_v1(handoff, context)
    else {
        panic!("exact Generic G0 source must produce a policy-checked observation");
    };
    (observation, lease)
}

#[test]
fn single_observation_moves_the_resolver_lease_into_generic_demand() {
    use crate::mir::loop_recipe_contract::issue_generic_g0_recipe_demand_from_observation_v1;

    let unit = generic_g0_source_unit_for_test();
    let (observation, lease) = generic_g0_observation_for_test(&unit);
    let owner = lease.owner();
    let origin = lease.function_origin();
    let site = lease.site().clone();
    let frame = lease.frame();
    let scope_region = lease.scope_region();
    let demand = issue_generic_g0_recipe_demand_from_observation_v1(observation, lease)
        .expect("one observation and its resolver lease issue one owned demand");

    let (lease, brand, bundle, post_loop_read, profile, mode, coverage, _roles) =
        demand.into_parts();
    assert_eq!(lease.owner(), owner);
    assert_eq!(lease.function_origin(), origin);
    assert_eq!(lease.site(), &site);
    assert!(lease.frame().matches(&frame));
    assert_eq!(lease.scope_region(), scope_region);
    assert!(brand.matches_window(&lease));
    assert_eq!(profile, super::GenericG0PolicyProfileV1::G0);
    assert_eq!(mode, super::LoopFamilyAdmissionModeV1::Release);
    assert_eq!(coverage, super::LoopFamilyAdmissionCoverageV1::Complete);
    let structural = bundle.source().structural();
    assert_eq!(structural.root_loop(), &site);
    assert_eq!(post_loop_read.binding(), structural.tail().binding);
}

fn attempt(
    source: &str,
    mode: Option<GenericG0ObservationModeV1>,
    coverage: GenericG0ObservationCoverageV1,
) -> crate::mir::loop_structural_facts::VerifiedGenericG0SourceAttemptV1 {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(parse_function(source))
        .expect("resolve fixture");
    let input = unit.root_function_input().expect("root input");
    let body = input.source().root_body().expect("root body");
    let loop_stmt = input.source().body_stmt(&body, 0).expect("root loop");
    let source = input
        .function()
        .resolved_loop_source(loop_stmt.site())
        .expect("root source");
    issue_generic_g0_source_attempt_v1(
        input,
        loop_stmt,
        source,
        NumericTarget::host(),
        mode,
        coverage,
    )
}

fn observe(
    attempt: crate::mir::loop_structural_facts::VerifiedGenericG0SourceAttemptV1,
    mode: Option<GenericG0ObservationModeV1>,
    coverage: GenericG0ObservationCoverageV1,
) -> GenericG0FamilyObservationV1 {
    let identity = attempt.identity().clone();
    issue_generic_g0_family_observation_v1(
        attempt,
        GenericG0ObservationContextV1::for_test(identity, mode, coverage),
    )
}

#[test]
fn canonical_candidate_is_normalized_in_all_modes() {
    for mode in [
        GenericG0ObservationModeV1::Release,
        GenericG0ObservationModeV1::Strict,
        GenericG0ObservationModeV1::StrictPlannerRequired,
    ] {
        let attempt = attempt(TYPED, Some(mode), GenericG0ObservationCoverageV1::Complete);
        let observation = observe(
            attempt,
            Some(mode),
            GenericG0ObservationCoverageV1::Complete,
        );
        let GenericG0FamilyObservationV1::Candidate(candidate) = observation else {
            panic!("exact Generic G0 must be a candidate")
        };
        assert_eq!(candidate.evidence().observed_mode(), Some(mode));
        assert_eq!(
            candidate.evidence().observed_coverage(),
            GenericG0ObservationCoverageV1::Complete
        );
    }
}

#[test]
fn known_shape_is_declined_without_policy_fallback() {
    let source = TYPED.replace("    return j", "    local extra = 0\n    return j");
    let attempt = attempt(
        &source,
        Some(GenericG0ObservationModeV1::Release),
        GenericG0ObservationCoverageV1::Complete,
    );
    assert!(matches!(
        observe(
            attempt,
            Some(GenericG0ObservationModeV1::Release),
            GenericG0ObservationCoverageV1::Complete,
        ),
        GenericG0FamilyObservationV1::Declined {
            reason: _,
            evidence: _
        }
    ));
}

#[test]
fn unsupported_policy_shape_stays_unresolved() {
    let source = TYPED.replace("i < 3", "i <= 3").replace("j < 3", "j <= 3");
    let attempt = attempt(
        &source,
        Some(GenericG0ObservationModeV1::Release),
        GenericG0ObservationCoverageV1::Complete,
    );
    assert!(matches!(
        observe(
            attempt,
            Some(GenericG0ObservationModeV1::Release),
            GenericG0ObservationCoverageV1::Complete,
        ),
        GenericG0FamilyObservationV1::Unresolved {
            reason: GenericG0ObservationUnresolvedV1::Policy(_),
            evidence: _,
        }
    ));
}

#[test]
fn direction_conflict_stays_rejected() {
    let source = TYPED.replace("i < 3", "i > 3").replace("j < 3", "j > 3");
    let attempt = attempt(
        &source,
        Some(GenericG0ObservationModeV1::Release),
        GenericG0ObservationCoverageV1::Complete,
    );
    assert!(matches!(
        observe(
            attempt,
            Some(GenericG0ObservationModeV1::Release),
            GenericG0ObservationCoverageV1::Complete,
        ),
        GenericG0FamilyObservationV1::Rejected {
            reason: GenericG0ObservationRejectV1::Policy(_),
            evidence: _,
        }
    ));
}

#[test]
fn incomplete_coverage_is_unresolved_before_policy() {
    let attempt = attempt(
        TYPED,
        Some(GenericG0ObservationModeV1::Release),
        GenericG0ObservationCoverageV1::Complete,
    );
    let observation = observe(
        attempt,
        Some(GenericG0ObservationModeV1::Release),
        GenericG0ObservationCoverageV1::Incomplete,
    );
    let GenericG0FamilyObservationV1::Unresolved { reason, evidence } = observation else {
        panic!("incomplete coverage must remain unresolved")
    };
    assert_eq!(reason, GenericG0ObservationUnresolvedV1::IncompleteCoverage);
    assert_eq!(
        evidence.expected().coverage(),
        GenericG0ObservationCoverageV1::Incomplete
    );
}

#[test]
fn mode_mismatch_is_rejected_before_policy() {
    let attempt = attempt(
        TYPED,
        Some(GenericG0ObservationModeV1::Release),
        GenericG0ObservationCoverageV1::Complete,
    );
    let observation = observe(
        attempt,
        Some(GenericG0ObservationModeV1::Strict),
        GenericG0ObservationCoverageV1::Complete,
    );
    let GenericG0FamilyObservationV1::Rejected { reason, evidence } = observation else {
        panic!("mode mismatch must be rejected")
    };
    assert_eq!(reason, GenericG0ObservationRejectV1::ModeMismatch);
    assert_eq!(
        evidence.observed_mode(),
        Some(GenericG0ObservationModeV1::Release)
    );
    assert_eq!(
        evidence.expected().mode(),
        Some(GenericG0ObservationModeV1::Strict)
    );
}

#[test]
fn unsealed_mode_is_unresolved() {
    let attempt = attempt(TYPED, None, GenericG0ObservationCoverageV1::Complete);
    let observation = observe(attempt, None, GenericG0ObservationCoverageV1::Complete);
    let GenericG0FamilyObservationV1::Unresolved { reason, evidence } = observation else {
        panic!("unsealed mode must remain unresolved")
    };
    assert_eq!(reason, GenericG0ObservationUnresolvedV1::ModeUnsealed);
    assert_eq!(evidence.observed_mode(), None);
    assert_eq!(evidence.expected().mode(), None);
}
