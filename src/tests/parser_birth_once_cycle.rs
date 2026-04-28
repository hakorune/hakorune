use crate::parser::NyashParser;

#[test]
fn birth_once_cycle_is_detected_inside_arrow_return_expression() {
    let err = crate::tests::helpers::env::with_env_var("NYASH_ENABLE_UNIFIED_MEMBERS", "1", || {
        NyashParser::parse_from_string(
            r#"
box CyclicBirthOnce {
  birth_once a: IntegerBox => me.b
  birth_once b: IntegerBox => me.a
}
"#,
        )
        .unwrap_err()
    });

    let err = err.to_string();
    assert!(
        err.contains("birth_once declarations must not have cyclic dependencies"),
        "unexpected error: {err}"
    );
}
