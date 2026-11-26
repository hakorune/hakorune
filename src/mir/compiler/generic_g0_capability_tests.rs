use super::lowering_input::{CanonicalLoweringErrorV1, VerifiedResolvedSourceUnitV1};
use crate::ast::ASTNode;
use crate::mir::loop_route_policy::GenericG0PolicyModeV1;

const G0: &str = r#"
box G0Fixture {
generic_g0(i: i64, j: i64): i64 {
    loop(i < 3) {
        loop(j < 3) {
            j = j + 1
        }
        i = i + 1
    }
    return j
}
}
"#;

fn parse_function(source: &str) -> ASTNode {
    crate::tests::helpers::parser::parse_fixture_method(source, "G0Fixture", "generic_g0")
}

fn resolved_g0() -> VerifiedResolvedSourceUnitV1 {
    VerifiedResolvedSourceUnitV1::resolve_function(parse_function(G0)).expect("fixture resolves")
}

#[test]
fn production_preflight_issues_generic_g0_plan_with_trivial_lifecycle() {
    let unit = resolved_g0();
    let plan = super::generic_g0_capability::probe_generic_g0_function_v1(
        unit.root_function_input().unwrap(),
        Some(GenericG0PolicyModeV1::Release),
    )
    .expect("generic G0 plan");
    let Some(plan) = plan else {
        panic!("expected Generic G0 loop plan")
    };
    assert_eq!(
        plan.source_parent().declaration_header().name(),
        "generic_g0"
    );
    let admission = super::generic_g0_physical_operation_cohort::
        issue_generic_g0_physical_emitter_admission_from_source_parent_v1(plan.into_source_parent())
        .expect("the plan reaches the normal Generic G0 physical admission");
    assert_eq!(admission.symbol().as_mir_name(), "generic_g0/2");
}

#[test]
fn marked_generic_g0_rejects_invalid_policy_mode_without_fallback() {
    let unit = resolved_g0();
    assert!(matches!(
        super::generic_g0_capability::probe_generic_g0_function_v1(unit.root_function_input().unwrap(), None),
        Err(CanonicalLoweringErrorV1::CapabilityNotActivated {
            boundary: "generic_g0_policy_mode",
        })
    ));
}
