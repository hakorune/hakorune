use super::capability::{CanonicalFirstFamilyPlanV1, CanonicalLoopFamilyPlanV1};
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
    assert_eq!(
        plan.source_parent().declaration_header().name(),
        "generic_g0"
    );
    assert_eq!(
        ExactCanonicalPreflightPlanV1::from_first_family(CanonicalFirstFamilyPlanV1::Loop(
            CanonicalLoopFamilyPlanV1::GenericG0(plan)
        ),)
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
fn production_generic_g0_reaches_single_publication_terminal() {
    let unit = resolved_g0();
    let mut compiler = super::MirCompiler::with_options(false);
    let result = compiler
        .compile_resolved(unit.lowering_input(), Some("generic_g0.hako"))
        .expect("Generic G0 source-bound compilation");

    assert!(result.verification_result.is_ok());
    assert_eq!(result.module.functions.len(), 1);
    let function = result
        .module
        .get_function("generic_g0/2")
        .expect("logical source symbol remains generic_g0/2");
    assert_eq!(function.signature.name, "generic_g0/2");
    assert_eq!(function.signature.params.len(), 3);
    assert_eq!(function.params.len(), 3);
    let declarations = &function.metadata.declared_param_decls;
    assert_eq!(declarations.len(), 3);
    assert!(declarations[0].implicit_receiver);
    assert_eq!(declarations[0].declared_type_name, None);
    assert!(function
        .signature
        .params
        .iter()
        .all(|ty| *ty == crate::mir::MirType::Integer));
    assert_eq!(function.metadata.parameter_entry_contracts.len(), 2);
    for (index, contract) in function
        .metadata
        .parameter_entry_contracts
        .iter()
        .enumerate()
    {
        assert_eq!(contract.formal_parameter_index, index + 1);
        assert_eq!(contract.source_parameter_index, index);
        assert_eq!(contract.parameter_value_id, function.params[index + 1]);
    }
    assert_eq!(
        function.metadata.declared_return_type_name.as_deref(),
        Some("i64")
    );
    assert!(function.metadata.return_exit_contract.is_some());
}

#[test]
fn generic_g0_prepared_commit_failure_discards_unpublished_module() {
    let unit = resolved_g0();
    let plan = super::generic_g0_capability::verify_with_generic_g0_mode_v1(
        &unit,
        Some(GenericG0PolicyModeV1::Release),
    )
    .expect("generic G0 plan");
    let CanonicalFirstFamilyPlanV1::Loop(CanonicalLoopFamilyPlanV1::GenericG0(plan)) = plan else {
        panic!("expected Generic G0 loop plan")
    };
    let mut compiler = super::MirCompiler::with_options(false);

    let error = super::resolved_generic_g0_cutover::
        compile_generic_g0_source_bound_with_prepared_failure_for_test(
            &mut compiler,
            plan,
            Some("generic_g0-failed.hako"),
        )
        .expect_err("prepared commit failure must be terminal");
    assert!(matches!(
        error,
        super::CanonicalLoweringErrorV1::BuilderContract { detail }
            if detail.contains("generic_g0/test_injected_prepared_commit_failure")
    ));
    assert!(compiler.builder.current_module.is_none());
    assert!(compiler.builder.current_function_name().is_none());
    assert!(compiler.builder.current_function_entry_block().is_none());
}
