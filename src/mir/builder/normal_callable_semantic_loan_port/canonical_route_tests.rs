use crate::mir::compiler::VerifiedResolvedSourceUnitV1;

#[test]
fn generic_g0_selection_rejects_missing_policy_mode() {
    let function = crate::tests::helpers::parser::parse_fixture_method(
        r#"static box G0Fixture {
            generic_g0(i: i64, j: i64): i64 {
                loop(i < 3) { loop(j < 3) { j = j + 1 } i = i + 1 }
                return j
            }
        }"#,
        "G0Fixture",
        "generic_g0",
    );
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(function)
        .expect("Generic G0 fixture resolves");
    let input = unit.root_function_input().expect("root function input");
    let result = super::canonical_route::try_classify_generic_g0_route_v1(input, None);
    assert!(matches!(result, Err(error) if error.contains("CapabilityNotActivated")));
}
