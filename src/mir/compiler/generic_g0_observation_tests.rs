use super::generic_g0_observation::issue_generic_g0_source_attempt_for_test;
use crate::ast::ASTNode;
use crate::mir::compiler::VerifiedResolvedSourceUnitV1;
use crate::mir::loop_structural_facts::{
    GenericG0ObservationCoverageV1, GenericG0ObservationModeV1, GenericG0SourceAttemptOutcomeV1,
    GenericG0SourceRejectV1, GenericG0SourceUnresolvedV1,
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
    mode: GenericG0ObservationModeV1,
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
        Some(mode),
        coverage,
    )
}

#[test]
fn canonical_source_is_candidate_in_each_mode() {
    for mode in [
        GenericG0ObservationModeV1::Release,
        GenericG0ObservationModeV1::Strict,
        GenericG0ObservationModeV1::StrictPlannerRequired,
    ] {
        let attempt = attempt(TYPED, mode, GenericG0ObservationCoverageV1::Complete);
        assert!(matches!(
            attempt.into_parts().0,
            GenericG0SourceAttemptOutcomeV1::Candidate(_)
        ));
    }
}

#[test]
fn known_extra_body_is_declined_without_policy() {
    let source = TYPED.replace("    return j", "    local extra = 0\n    return j");
    let attempt = attempt(
        &source,
        GenericG0ObservationModeV1::Release,
        GenericG0ObservationCoverageV1::Complete,
    );
    assert!(matches!(
        attempt.into_parts().0,
        GenericG0SourceAttemptOutcomeV1::Declined(_)
    ));
}

#[test]
fn missing_return_annotation_is_unresolved() {
    let source = TYPED.replace(": i64 {", "{");
    let attempt = attempt(
        &source,
        GenericG0ObservationModeV1::Release,
        GenericG0ObservationCoverageV1::Complete,
    );
    assert_eq!(
        attempt.into_parts().0,
        GenericG0SourceAttemptOutcomeV1::Unresolved(GenericG0SourceUnresolvedV1::TypeUnavailable)
    );
}

#[test]
fn non_i64_parameter_is_rejected_as_type_conflict() {
    let source = TYPED.replace("i: i64", "i: bool");
    let attempt = attempt(
        &source,
        GenericG0ObservationModeV1::Release,
        GenericG0ObservationCoverageV1::Complete,
    );
    assert_eq!(
        attempt.into_parts().0,
        GenericG0SourceAttemptOutcomeV1::Rejected(GenericG0SourceRejectV1::TypeConflict)
    );
}

#[test]
fn foreign_resolver_source_is_rejected_before_projection() {
    let unit_a =
        VerifiedResolvedSourceUnitV1::resolve_function(parse_function(TYPED)).expect("resolve A");
    let unit_b =
        VerifiedResolvedSourceUnitV1::resolve_function(parse_function(TYPED)).expect("resolve B");
    let input_a = unit_a.root_function_input().expect("root A");
    let input_b = unit_b.root_function_input().expect("root B");
    let body_a = input_a.source().root_body().expect("body A");
    let body_b = input_b.source().root_body().expect("body B");
    let _loop_a = input_a.source().body_stmt(&body_a, 0).expect("loop A");
    let loop_b = input_b.source().body_stmt(&body_b, 0).expect("loop B");
    let source_b = input_b
        .function()
        .resolved_loop_source(loop_b.site())
        .expect("source B");
    let attempt = issue_generic_g0_source_attempt_for_test(
        input_a,
        loop_b,
        source_b,
        NumericTarget::host(),
        Some(GenericG0ObservationModeV1::Release),
        GenericG0ObservationCoverageV1::Complete,
    );
    assert_eq!(
        attempt.into_parts().0,
        GenericG0SourceAttemptOutcomeV1::Rejected(GenericG0SourceRejectV1::ForeignOwner)
    );
}
