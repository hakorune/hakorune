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
    crate::tests::helpers::env::with_env_var("NYASH_PARSER_TOKEN_CURSOR", "0", || {
        let err = NyashParser::parse_from_string(direct_birth_call_source())
            .expect_err("direct receiver birth call must reject");
        let msg = err.to_string();
        assert!(msg.contains("birth"), "{msg}");
        assert!(msg.contains("constructor hook"), "{msg}");
        assert!(
            msg.contains("direct source birth calls are forbidden"),
            "{msg}"
        );
    });
}

#[test]
fn parser_birth_rejects_direct_receiver_birth_call_token_cursor() {
    crate::tests::helpers::env::with_env_var("NYASH_PARSER_TOKEN_CURSOR", "1", || {
        let err = NyashParser::parse_from_string(direct_birth_call_source())
            .expect_err("direct receiver birth call must reject");
        let msg = err.to_string();
        assert!(msg.contains("birth"), "{msg}");
        assert!(msg.contains("constructor hook"), "{msg}");
        assert!(
            msg.contains("direct source birth calls are forbidden"),
            "{msg}"
        );
    });
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
fn parser_birth_keeps_parent_constructor_delegation() {
    NyashParser::parse_from_string(
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
    .expect("parent constructor delegation remains valid");
}
