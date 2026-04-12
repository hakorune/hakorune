use super::*;

#[test]
fn parser_rejects_unknown_rune_name_fail_fast() {
    with_features(Some("rune"), || {
        let src = r#"
@rune Strange
static box Main {
  main() { return 0 }
}
"#;
        let err = NyashParser::parse_from_string(src).expect_err("parse should fail");
        let msg = err.to_string();
        assert!(
            msg.contains(
                "[freeze:contract][parser/rune] supported: Public|Internal|FfiSafe|Symbol|CallConv|ReturnsOwned|FreeWith|Ownership|Hint|Contract|IntrinsicCandidate"
            ),
            "unexpected error: {msg}"
        );
    });
}

#[test]
fn parser_rejects_rune_on_non_declaration_statement() {
    with_features(Some("rune"), || {
        let src = r#"
static box Main {
  main() {
    @rune Public
    local x = 1
    return x
  }
}
"#;
        let err = NyashParser::parse_from_string(src).expect_err("parse should fail");
        let msg = err.to_string();
        assert!(
            msg.contains("[freeze:contract][parser/rune] declaration required after @rune")
                || msg.contains("[freeze:contract][parser/rune] invalid placement"),
            "unexpected error: {msg}"
        );
    });
}

#[test]
fn parser_rejects_canonical_optimization_rune_on_non_declaration_statement() {
    with_features(Some("rune"), || {
        let src = r#"
static box Main {
  main() {
    @rune Hint(inline)
    local x = 1
    return x
  }
}
"#;
        let err = NyashParser::parse_from_string(src).expect_err("parse should fail");
        let msg = err.to_string();
        assert!(
            msg.contains("[freeze:contract][parser/rune] declaration required after @rune")
                || msg.contains("[freeze:contract][parser/rune] invalid placement"),
            "unexpected error: {msg}"
        );
    });
}

#[test]
fn parser_rejects_duplicate_runes_on_same_declaration() {
    with_features(Some("rune"), || {
        let src = r#"
static box Main {
  @rune Symbol("main_a")
  @rune Symbol("main_b")
  main() { return 0 }
}
"#;
        let err = NyashParser::parse_from_string(src).expect_err("parse should fail");
        let msg = err.to_string();
        assert!(
            msg.contains("[freeze:contract][parser/rune] duplicate rune Symbol"),
            "unexpected error: {msg}"
        );
    });
}

#[test]
fn parser_rejects_conflicting_visibility_runes() {
    with_features(Some("rune"), || {
        let src = r#"
@rune Public
@rune Internal
static box Main {
  main() { return 0 }
}
"#;
        let err = NyashParser::parse_from_string(src).expect_err("parse should fail");
        let msg = err.to_string();
        assert!(
            msg.contains("[freeze:contract][parser/rune] conflicting visibility runes"),
            "unexpected error: {msg}"
        );
    });
}

#[test]
fn parser_rejects_non_visibility_rune_on_box_target() {
    with_features(Some("rune"), || {
        let src = r#"
@rune Symbol("main_sym")
static box Main {
  main() { return 0 }
}
"#;
        let err = NyashParser::parse_from_string(src).expect_err("parse should fail");
        let msg = err.to_string();
        assert!(
            msg.contains("[freeze:contract][parser/rune] box target supports only Public|Internal"),
            "unexpected error: {msg}"
        );
    });
}

#[test]
fn parser_accepts_abi_runes_on_static_box_method() {
    with_features(Some("rune"), || {
        let src = r#"
static box Main {
  @rune Symbol("main_sym")
  @rune CallConv("c")
  main() { return 0 }
}
"#;
        let ast = NyashParser::parse_from_string(src).expect("parse should succeed");
        let (_box_runes, method_runes) = find_box_and_method_runes(&ast, "Main", "main");
        assert_eq!(
            method_runes,
            vec![
                ("Symbol".to_string(), vec!["main_sym".to_string()]),
                ("CallConv".to_string(), vec!["c".to_string()])
            ]
        );
    });
}

#[test]
fn parser_rejects_abi_rune_on_instance_method() {
    with_features(Some("rune"), || {
        let src = r#"
box Main {
  @rune Symbol("main_sym")
  main() { return 0 }
}
"#;
        let err = NyashParser::parse_from_string(src).expect_err("parse should fail");
        let msg = err.to_string();
        assert!(
            msg.contains(
                "[freeze:contract][parser/rune] instance method target supports only Public|Internal|Ownership|Hint|Contract|IntrinsicCandidate"
            ),
            "unexpected error: {msg}"
        );
    });
}

#[test]
fn parser_rejects_abi_rune_on_constructor() {
    with_features(Some("rune"), || {
        let src = r#"
box Main {
  @rune Symbol("ctor_sym")
  birth() {}
}
"#;
        let err = NyashParser::parse_from_string(src).expect_err("parse should fail");
        let msg = err.to_string();
        assert!(
            msg.contains(
                "[freeze:contract][parser/rune] constructor target supports only Public|Internal|Ownership|Hint|Contract|IntrinsicCandidate"
            ),
            "unexpected error: {msg}"
        );
    });
}

#[test]
fn parser_rejects_abi_rune_on_interface_method() {
    with_features(Some("rune"), || {
        let src = r#"
interface box Main {
  @rune Symbol("iface_sym")
  run()
}
"#;
        let err = NyashParser::parse_from_string(src).expect_err("parse should fail");
        let msg = err.to_string();
        assert!(
            msg.contains(
                "[freeze:contract][parser/rune] interface method target supports only Public|Internal|Ownership|Hint|Contract|IntrinsicCandidate"
            ),
            "unexpected error: {msg}"
        );
    });
}
