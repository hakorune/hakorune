use crate::parser::NyashParser;

fn direct_birth_call_source() -> &'static str {
    r#"
static box Main {
  main(args) {
    local page = new Page(1)
    page.birth(2)
    return 0
  }
}
"#
}

#[test]
fn parser_birth_rejects_direct_receiver_birth_call_legacy_expr_parser() {
    let err = NyashParser::parse_from_string(direct_birth_call_source())
        .expect_err("direct receiver birth call must reject");
    let msg = err.to_string();
    assert!(msg.contains("birth"), "{msg}");
    assert!(msg.contains("constructor hook"), "{msg}");
    assert!(
        msg.contains("direct receiver `birth(...)` calls are forbidden"),
        "{msg}"
    );
    assert!(msg.contains("use `new Box(...)` for construction"), "{msg}");
}

#[test]
fn parser_birth_direct_call_diagnostic_points_to_new_expression() {
    let err = NyashParser::parse_from_string(direct_birth_call_source())
        .expect_err("direct receiver birth call must reject");
    let msg = err.to_string();
    assert!(msg.contains("use `new Box(...)` for construction"), "{msg}");
}

#[test]
fn parser_birth_accepts_constructor_declaration() {
    NyashParser::parse_from_string(
        r#"
box Page {
  birth(id) {
  }
}
"#,
    )
    .expect("constructor declaration remains valid");
}

#[test]
fn parser_birth_rejects_parent_delegation_with_arguments() {
    // `from Parent.birth(name)` is outside the closed `from IDENT . IDENT ( )`
    // transport form (phase-296x/archive/3466), so it is a named rejection.
    let error = NyashParser::parse_from_string(
        r#"
box Parent {
  birth(name) {
  }
}

box Child {
  birth(name) {
    from Parent.birth(name)
  }
}
"#,
    )
    .expect_err("parent delegation with arguments is not a closed transport form");
    assert!(
        format!("{error:?}").contains("parser/from_transport_not_closed_form"),
        "unexpected rejection: {error:?}"
    );
}
