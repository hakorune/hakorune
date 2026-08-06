use super::generic_g0_observation::{
    issue_generic_g0_family_observation_v1, GenericG0FamilyObservationV1,
    GenericG0ObservationContextV1, GenericG0ObservationRejectV1, GenericG0ObservationUnresolvedV1,
};
use crate::ast::ASTNode;
use crate::mir::compiler::generic_g0_observation::issue_generic_g0_source_attempt_for_test;
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
    issue_generic_g0_source_attempt_for_test(
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
