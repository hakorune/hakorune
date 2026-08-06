use super::{
    issue_generic_g0_candidate_v1, GenericG0CoverageV1, GenericG0PolicyContextV1,
    GenericG0PolicyModeV1, GenericG0PolicyOutcomeV1, GenericG0PolicyProfileV1,
    GenericG0PolicyRejectV1, GenericG0PolicyUnresolvedV1,
};
use crate::ast::ASTNode;
use crate::mir::compiler::generic_g0_projection::{
    issue_generic_g0_source_type_bundle_v1, issue_generic_g0_typed_source_bundle_v1,
    VerifiedGenericTypedSourceBundleG0,
};
use crate::mir::compiler::VerifiedResolvedSourceUnitV1;
use crate::mir::numeric_substrate::generic_g0::GenericG0NumericLiteralRoleV1;
use crate::mir::numeric_substrate::NumericTarget;
use crate::mir::resolved_semantics::FunctionOwnerIssuerV1;
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

fn typed_bundle(
    source: &str,
) -> (
    VerifiedGenericTypedSourceBundleG0,
    crate::mir::resolved_semantics::FunctionOwnerIdV1,
) {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(parse_function(source))
        .expect("resolve fixture");
    let input = unit.root_function_input().expect("root input");
    let owner = input.owner();
    let source_bundle = issue_generic_g0_source_type_bundle_v1(input).expect("S0B source bundle");
    let typed = issue_generic_g0_typed_source_bundle_v1(source_bundle, NumericTarget::host())
        .expect("S0C typed bundle");
    (typed, owner)
}

fn context(
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    coverage: GenericG0CoverageV1,
) -> GenericG0PolicyContextV1 {
    GenericG0PolicyContextV1::for_test(
        owner,
        GenericG0PolicyProfileV1::G0,
        GenericG0PolicyModeV1::Release,
        coverage,
    )
}

#[test]
fn positive_less_add_observation_is_move_only_and_typed() {
    let (bundle, owner) = typed_bundle(TYPED);
    let outcome =
        issue_generic_g0_candidate_v1(bundle, context(owner, GenericG0CoverageV1::Complete));
    let GenericG0PolicyOutcomeV1::Candidate(observation) = outcome else {
        panic!("canonical typed G0 must be a candidate")
    };
    assert_eq!(observation.context().owner(), owner);
    assert_eq!(
        observation.context().profile(),
        GenericG0PolicyProfileV1::G0
    );
    assert_eq!(observation.context().mode(), GenericG0PolicyModeV1::Release);
    assert_eq!(observation.bundle().numeric().literals().len(), 4);
}

#[test]
fn unsupported_comparison_is_unresolved_without_policy_fallback() {
    let source = TYPED.replace("i < 3", "i <= 3").replace("j < 3", "j <= 3");
    let (bundle, owner) = typed_bundle(&source);
    assert_eq!(
        issue_generic_g0_candidate_v1(bundle, context(owner, GenericG0CoverageV1::Complete)),
        GenericG0PolicyOutcomeV1::Unresolved(GenericG0PolicyUnresolvedV1::UnsupportedComparison)
    );
}

#[test]
fn unsupported_update_is_unresolved_without_synthetic_add() {
    let source = TYPED.replace("j = j + 1", "j = j * 2");
    let (bundle, owner) = typed_bundle(&source);
    assert_eq!(
        issue_generic_g0_candidate_v1(bundle, context(owner, GenericG0CoverageV1::Complete)),
        GenericG0PolicyOutcomeV1::Unresolved(GenericG0PolicyUnresolvedV1::UnsupportedUpdate)
    );
}

#[test]
fn non_progressing_step_is_unresolved_with_exact_role() {
    let source = TYPED
        .replace("j = j + 1", "j = j + 0")
        .replace("i = i + 1", "i = i + 0");
    let (bundle, owner) = typed_bundle(&source);
    assert_eq!(
        issue_generic_g0_candidate_v1(bundle, context(owner, GenericG0CoverageV1::Complete)),
        GenericG0PolicyOutcomeV1::Unresolved(GenericG0PolicyUnresolvedV1::NonProgressingStep {
            role: GenericG0NumericLiteralRoleV1::OuterUpdateRhs,
        })
    );
}

#[test]
fn opposite_direction_is_rejected_without_rewriting() {
    let source = TYPED.replace("i < 3", "i > 3").replace("j < 3", "j > 3");
    let (bundle, owner) = typed_bundle(&source);
    assert_eq!(
        issue_generic_g0_candidate_v1(bundle, context(owner, GenericG0CoverageV1::Complete)),
        GenericG0PolicyOutcomeV1::Rejected(GenericG0PolicyRejectV1::DirectionMismatch)
    );
}

#[test]
fn foreign_context_is_rejected_before_operator_policy() {
    let (bundle, _) = typed_bundle(TYPED);
    let mut issuer = FunctionOwnerIssuerV1::new_for_compilation().expect("owner issuer");
    let foreign_owner = issuer.issue().expect("foreign owner");
    assert_eq!(
        issue_generic_g0_candidate_v1(
            bundle,
            context(foreign_owner, GenericG0CoverageV1::Complete)
        ),
        GenericG0PolicyOutcomeV1::Rejected(GenericG0PolicyRejectV1::ForeignContext)
    );
}

#[test]
fn incomplete_coverage_is_unresolved_before_candidate_issue() {
    let (bundle, owner) = typed_bundle(TYPED);
    assert_eq!(
        issue_generic_g0_candidate_v1(bundle, context(owner, GenericG0CoverageV1::Incomplete)),
        GenericG0PolicyOutcomeV1::Unresolved(GenericG0PolicyUnresolvedV1::IncompleteCoverage)
    );
}
