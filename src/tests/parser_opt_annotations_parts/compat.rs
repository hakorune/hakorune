use super::*;

#[test]
fn tokenizer_accepts_rune_prefix_by_default() {
    with_features(None, || {
        let mut t = NyashTokenizer::new("@rune Inline(required)");
        let tokens = t.tokenize().expect("tokenize canonical rune");
        assert!(tokens
            .iter()
            .any(|token| matches!(token.token_type, crate::tokenizer::TokenType::AT)));
    });
}

#[test]
fn parser_accepts_annotations_as_noop_when_feature_on() {
    with_features(Some("stage3,opt-annotations"), || {
        let src = r#"
static box Main {
  @hint(inline)
  main() {
    @contract(no_alloc)
    local x = 1
    return x
  }
}
"#;
        let ast = NyashParser::parse_from_string(src).expect("parse with annotations");
        let body = find_method_body(&ast, "Main", "main");
        assert_eq!(
            body.len(),
            2,
            "annotation directives must not produce AST nodes"
        );
    });
}

#[test]
fn parser_accepts_body_position_legacy_annotations_as_noop_under_rune_gate() {
    with_features(Some("stage3,rune"), || {
        let src = r#"
static box Main {
  main() {
    @contract(no_alloc)
    local x = 1
    return x
  }
}
"#;
        let ast = NyashParser::parse_from_string(src)
            .expect("parse with legacy annotation under rune gate");
        let body = find_method_body(&ast, "Main", "main");
        assert_eq!(
            body.len(),
            2,
            "body-position legacy annotations stay noop during compat window"
        );
    });
}

#[test]
fn parser_rejects_unknown_hint_argument_fail_fast() {
    with_features(Some("stage3,opt-annotations"), || {
        let src = r#"
static box Main {
  @hint(fastest)
  main() { return 0 }
}
"#;
        let err = NyashParser::parse_from_string(src).expect_err("parse should fail");
        let msg = err.to_string();
        assert!(
            msg.contains("[freeze:contract][parser/annotation] @hint(inline|noinline|hot|cold)"),
            "unexpected error: {msg}"
        );
    });
}

#[test]
fn parser_rejects_unknown_annotation_name_fail_fast() {
    with_features(Some("stage3,opt-annotations"), || {
        let src = r#"
static box Main {
  @speed(hot)
  main() { return 0 }
}
"#;
        let err = NyashParser::parse_from_string(src).expect_err("parse should fail");
        let msg = err.to_string();
        assert!(
            msg.contains(
                "[freeze:contract][parser/annotation] supported: hint|contract|intrinsic_candidate"
            ),
            "unexpected error: {msg}"
        );
    });
}
