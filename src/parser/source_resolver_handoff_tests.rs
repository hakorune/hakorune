use super::*;

#[test]
fn resolver_handoff_preserves_ast_free_query_method_source() {
    let (ast, handoff) = NyashParser::parse_from_string_with_resolver_source_handoff(
        r#"
box TextLike {
    @rune CallableContract(query)
    length(): i64 { return 0 }
}
"#,
        ParserBuildConfig::default(),
    )
    .expect("ordinary Box source must issue the resolver handoff");

    assert!(matches!(ast, ASTNode::Program { .. }));
    assert_eq!(handoff.boxes().len(), 1);
    let boxed = &handoff.boxes()[0];
    assert_eq!(boxed.statement_ordinal(), 0);
    assert_eq!(boxed.name(), "TextLike");
    assert_eq!(boxed.methods().len(), 1);

    let method = &boxed.methods()[0];
    assert_eq!(method.name(), "length");
    assert_eq!(method.source_site().box_statement_ordinal(), 0);
    assert_eq!(method.source_site().member_ordinal(), 0);
    assert_eq!(method.signature().parameters().len(), 0);
    assert_eq!(method.signature().return_type_name(), Some("i64"));
    assert!(!method.signature().is_static());
    assert!(matches!(
        method.callable_contract(),
        Some(CallableContractSyntaxV1::Query { .. })
    ));
}

#[test]
fn resolver_handoff_skips_generated_rows_but_requires_explicit_source() {
    let error = NyashParser::parse_from_string_with_resolver_source_handoff(
        r#"
box GeneratedOnly {
    once value: i64 => 1
}
"#,
        ParserBuildConfig::default(),
    )
    .expect_err("generated-only Box must not become an empty resolver authority");

    assert!(error.to_string().contains("GeneratedOnly"));
}

#[test]
fn resolver_handoff_rejects_unsupported_top_level_gate_before_issuance() {
    let error = NyashParser::parse_from_string_with_resolver_source_handoff(
        r#"
if true {
    box Gated { length(): i64 { return 0 } }
}
"#,
        ParserBuildConfig::default(),
    )
    .expect_err("top-level gate is outside the bounded source seal cohort");

    assert!(error.to_string().contains("Unresolved"));
}
