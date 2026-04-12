use super::*;

#[test]
fn parser_rejects_invalid_hint_rune_value() {
    with_features(Some("rune"), || {
        let src = r#"
static box Main {
  @rune Hint(fastest)
  main() { return 0 }
}
"#;
        let err = NyashParser::parse_from_string(src).expect_err("parse should fail");
        let msg = err.to_string();
        assert!(
            msg.contains("[freeze:contract][parser/rune] Hint(inline|noinline|hot|cold)"),
            "unexpected error: {msg}"
        );
    });
}

#[test]
fn parser_rejects_invalid_contract_rune_value() {
    with_features(Some("rune"), || {
        let src = r#"
static box Main {
  @rune Contract(mutable)
  main() { return 0 }
}
"#;
        let err = NyashParser::parse_from_string(src).expect_err("parse should fail");
        let msg = err.to_string();
        assert!(
            msg.contains(
                "[freeze:contract][parser/rune] Contract(pure|readonly|no_alloc|no_safepoint)"
            ),
            "unexpected error: {msg}"
        );
    });
}

#[test]
fn parser_rejects_empty_intrinsic_candidate_rune_value() {
    with_features(Some("rune"), || {
        let src = r#"
static box Main {
  @rune IntrinsicCandidate("")
  main() { return 0 }
}
"#;
        let err = NyashParser::parse_from_string(src).expect_err("parse should fail");
        let msg = err.to_string();
        assert!(
            msg.contains(
                "[freeze:contract][parser/rune] IntrinsicCandidate(\"symbol\") with non-empty symbol"
            ),
            "unexpected error: {msg}"
        );
    });
}

#[test]
fn parser_rejects_invalid_callconv_value() {
    with_features(Some("rune"), || {
        let src = r#"
static box Main {
  @rune CallConv("sysv")
  main() { return 0 }
}
"#;
        let err = NyashParser::parse_from_string(src).expect_err("parse should fail");
        let msg = err.to_string();
        assert!(
            msg.contains("[freeze:contract][parser/rune] CallConv(\"c\")"),
            "unexpected error: {msg}"
        );
    });
}

#[test]
fn parser_rejects_invalid_ownership_value() {
    with_features(Some("rune"), || {
        let src = r#"
box Main {
  @rune Ownership(unique)
  main() { return 0 }
}
"#;
        let err = NyashParser::parse_from_string(src).expect_err("parse should fail");
        let msg = err.to_string();
        assert!(
            msg.contains("[freeze:contract][parser/rune] Ownership(owned|borrowed|shared)"),
            "unexpected error: {msg}"
        );
    });
}
