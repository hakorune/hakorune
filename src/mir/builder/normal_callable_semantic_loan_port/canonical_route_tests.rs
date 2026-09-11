use super::{classify_canonical_callable_route, CanonicalCallableRouteV1};
use crate::ast::ASTNode;
use crate::mir::compiler::{
    callable_single_loop_static_fixture_tests::static_fixture_for_test,
    direct_accum_projection::direct_accum_function_for_test, VerifiedResolvedSourceUnitV1,
};
use crate::parser::NyashParser;

#[test]
fn direct_accum_selection_uses_the_canonical_route() {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(direct_accum_function_for_test())
        .expect("DirectAccum fixture must resolve");
    let input = unit.root_function_input().expect("root function input");
    let route = classify_canonical_callable_route(
        input,
        Some(crate::mir::loop_route_policy::GenericG0PolicyModeV1::Release),
    )
    .expect("canonical preflight");
    assert!(matches!(route, CanonicalCallableRouteV1::DirectAccum(_)));
}

#[test]
fn callable_single_loop_selection_uses_the_source_bound_route() {
    let module = static_fixture_for_test();
    let header = module
        .source()
        .catalog()
        .index()
        .resolve_free_static_source_call("int_to_str", 1)
        .expect("int_to_str header");
    let input = module
        .function_input(header.source_key())
        .expect("int_to_str input");
    let route = classify_canonical_callable_route(
        input,
        Some(crate::mir::loop_route_policy::GenericG0PolicyModeV1::Release),
    )
    .expect("canonical preflight");
    assert!(matches!(route, CanonicalCallableRouteV1::CallableSingleLoop(_)));
}

#[test]
fn generic_g0_selection_rejects_missing_policy_mode() {
    let program = NyashParser::parse_from_string(
        r#"static function generic_g0(i: i64, j: i64): i64 {
            loop(i < 3) { loop(j < 3) { j = j + 1 } i = i + 1 }
            return j
        }"#,
    )
    .expect("Generic G0 fixture parses");
    let function = match program {
        ASTNode::Program { statements, .. } => statements
            .into_iter()
            .find(|node| matches!(node, ASTNode::FunctionDeclaration { .. }))
            .expect("Generic G0 function"),
        _ => panic!("fixture must be a program"),
    };
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(function)
        .expect("Generic G0 fixture resolves");
    let input = unit.root_function_input().expect("root function input");
    let result = super::generic_g0::try_select_top_level_generic_g0_plan_v1(input, None);
    assert!(matches!(result, Err(error) if error.contains("CapabilityNotActivated")));
}
