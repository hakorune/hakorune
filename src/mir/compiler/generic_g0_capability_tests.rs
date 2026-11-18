use super::capability::CanonicalLoopFamilyPlanV1;
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
    assert_eq!(
        ExactCanonicalPreflightPlanV1::Loop(CanonicalLoopFamilyPlanV1::GenericG0(plan))
        .route(),
        super::source_bound_plan::CanonicalSourceRouteV1::BindingSsaTrivial
    );
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

#[test]
fn generic_g0_prepared_commit_failure_discards_unpublished_module() {
    let unit = resolved_g0();
    let plan = super::generic_g0_capability::probe_generic_g0_function_v1(
        unit.root_function_input().unwrap(),
        Some(GenericG0PolicyModeV1::Release),
    )
    .expect("generic G0 plan");
    let Some(plan) = plan else {
        panic!("expected Generic G0 loop plan")
    };
    let mut compiler = super::MirCompiler::with_options(false);
    let exact = ExactCanonicalPreflightPlanV1::Loop(CanonicalLoopFamilyPlanV1::GenericG0(plan));
    let package = compiler.bind_canonical_source(exact).unwrap();
    let ready = compiler
        .begin_canonical_invocation(package, Some("generic_g0-failed.hako"), "generic_g0/2".into())
        .unwrap()
        .lower()
        .unwrap()
        .collect()
        .unwrap()
        .complete()
        .unwrap()
        .prepare_drain()
        .unwrap()
        .drain()
        .prepare_finalization()
        .unwrap();
    let finalized = super::canonical_finalization::CanonicalModuleFinalizerV1::finalize(ready).unwrap();
    let processed = super::module_postprocess::ModulePostprocessOwnerV1::new(&mut compiler.verifier, false)
        .run(finalized)
        .unwrap();
    let prepared = compiler.prepare_module_external_commit(processed).unwrap();
    drop(prepared);
    assert!(compiler.builder.current_module.is_none());
    assert!(compiler.builder.current_function_name().is_none());
    assert!(compiler.builder.current_function_entry_block().is_none());
}
