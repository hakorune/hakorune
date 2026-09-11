use super::capability::{
    CanonicalFirstFamilyPlanV1, CanonicalLoopFamilyPlanV1,
};
use super::lowering_input::{CanonicalLoweringErrorV1, VerifiedResolvedSourceUnitV1};
use super::source_bound_plan::ExactCanonicalPreflightPlanV1;
use crate::ast::ASTNode;
use crate::mir::loop_route_policy::GenericG0PolicyModeV1;
use crate::parser::NyashParser;

const G0: &str = r#"
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
        .expect("function fixture")
}

fn resolved_g0() -> VerifiedResolvedSourceUnitV1 {
    VerifiedResolvedSourceUnitV1::resolve_function(parse_function(G0)).expect("fixture resolves")
}

#[test]
fn production_preflight_issues_generic_g0_plan_with_trivial_lifecycle() {
    let unit = resolved_g0();
    let plan = super::generic_g0_capability::verify_with_generic_g0_mode_v1(
        &unit,
        Some(GenericG0PolicyModeV1::Release),
    )
    .expect("generic G0 plan");
    let CanonicalFirstFamilyPlanV1::Loop(CanonicalLoopFamilyPlanV1::GenericG0(plan)) = plan else {
        panic!("expected Generic G0 loop plan")
    };
    assert_eq!(plan.source_parent().declaration_header().name(), "generic_g0");
    assert_eq!(
        ExactCanonicalPreflightPlanV1::from_first_family(
            CanonicalFirstFamilyPlanV1::Loop(CanonicalLoopFamilyPlanV1::GenericG0(plan)),
        )
        .route(),
        super::source_bound_plan::CanonicalSourceRouteV1::BindingSsaTrivial
    );
}

#[test]
fn marked_generic_g0_rejects_invalid_policy_mode_without_fallback() {
    let unit = resolved_g0();
    assert!(matches!(
        super::generic_g0_capability::verify_with_generic_g0_mode_v1(&unit, None),
        Err(CanonicalLoweringErrorV1::CapabilityNotActivated {
            boundary: "generic_g0_policy_mode",
        })
    ));
}

#[test]
fn i0_package_seals_header_then_stops_before_physical_lowering() {
    let unit = resolved_g0();
    let plan = super::generic_g0_capability::verify_with_generic_g0_mode_v1(
        &unit,
        Some(GenericG0PolicyModeV1::Strict),
    )
    .expect("generic G0 plan");
    let plan = ExactCanonicalPreflightPlanV1::from_first_family(plan);
    assert_eq!(
        plan.route(),
        super::source_bound_plan::CanonicalSourceRouteV1::BindingSsaTrivial
    );

    let mut compiler = super::MirCompiler::new();
    let package = compiler
        .bind_canonical_source(plan)
        .expect("source package binds");
    let rejected = match compiler.lower_canonical_source(package, None) {
        Ok(_) => panic!("I0 must stop before physical lowering"),
        Err(rejected) => rejected,
    };
    assert!(matches!(
        rejected.error(),
        super::source_bound_package::CanonicalPlanLoweringErrorV1::GenericG0NotActivated
    ));
}
