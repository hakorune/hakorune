use super::*;
use crate::ast::{DeclarationAttrs, RuneAttr, Span};
use crate::parser::{NyashParser, ParserBuildConfig};

fn method(attrs: Vec<RuneAttr>) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: "length".to_owned(),
        params: Vec::new(),
        param_decls: Vec::new(),
        return_type_name: Some("i64".to_owned()),
        body: Vec::new(),
        contracts: Vec::new(),
        uses: Vec::new(),
        is_static: false,
        is_override: false,
        attrs: DeclarationAttrs { runes: attrs },
        span: Span::unknown(),
    }
}

fn rune(name: &str, arg: &str) -> RuneAttr {
    RuneAttr {
        name: name.to_owned(),
        args: vec![arg.to_owned()],
    }
}

#[test]
fn query_normalizes_to_typed_syntax_and_keeps_rune_coordinate() {
    let syntax = CallableContractSyntaxV1::from_instance_method(&method(vec![
        rune("Hint", "hot"),
        rune("CallableContract", "query"),
    ]))
    .expect("query rune should normalize");
    assert!(matches!(syntax, CallableContractSyntaxV1::Query { .. }));
    assert_eq!(syntax.source_site().rune_ordinal(), 1);
}

#[test]
fn missing_query_does_not_issue_typed_syntax() {
    assert!(CallableContractSyntaxV1::from_instance_method(&method(vec![])).is_none());
    assert!(
        CallableContractSyntaxV1::from_instance_method(&method(vec![rune(
            "CallableContract",
            "other",
        )]))
        .is_none()
    );
}

#[test]
fn parser_rejects_unknown_query_value() {
    let error = NyashParser::parse_from_string(
        "box X { @rune CallableContract(other) run() { return 0 } }",
    )
    .expect_err("unknown CallableContract value must fail at syntax");
    assert!(error.to_string().contains("CallableContract(query)"));
}

#[test]
fn parser_rejects_duplicate_query_rune() {
    let error = NyashParser::parse_from_string(
        "box X { @rune CallableContract(query) @rune CallableContract(query) run() { return 0 } }",
    )
    .expect_err("duplicate CallableContract must fail at syntax");
    assert!(error
        .to_string()
        .contains("duplicate rune CallableContract"));
}

#[test]
fn parser_rejects_query_on_static_box_method() {
    let error = NyashParser::parse_from_string(
        "static box X { @rune CallableContract(query) run() { return 0 } }",
    )
    .expect_err("CallableContract is instance-method syntax only");
    assert!(error
        .to_string()
        .contains("allowed only on instance methods"));
}

#[test]
fn parser_build_config_entry_keeps_query_syntax_bounded() {
    let parsed = NyashParser::parse_from_string_with_build_config(
        "box X { @rune CallableContract(query) run() { return 0 } }",
        ParserBuildConfig::default(),
    )
    .expect("query syntax should remain accepted through the normal parser");
    assert!(matches!(parsed, crate::ast::ASTNode::Program { .. }));
}
