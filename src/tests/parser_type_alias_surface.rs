use crate::ast::ASTNode;
use crate::parser::NyashParser;

#[test]
fn parser_type_alias_surface_parses_metadata_only_alias() {
    let ast = NyashParser::parse_from_string(
        r#"
type Bytes = usize
type PageList = Array<PageId>
"#,
    )
    .expect("parse type aliases");

    let ASTNode::Program { statements, .. } = ast else {
        panic!("expected Program");
    };
    assert_eq!(statements.len(), 2);

    let ASTNode::TypeAliasDeclaration {
        name,
        target_type_name,
        ..
    } = &statements[0]
    else {
        panic!("expected first type alias declaration");
    };
    assert_eq!(name, "Bytes");
    assert_eq!(target_type_name, "usize");

    let ASTNode::TypeAliasDeclaration {
        name,
        target_type_name,
        ..
    } = &statements[1]
    else {
        panic!("expected second type alias declaration");
    };
    assert_eq!(name, "PageList");
    assert_eq!(target_type_name, "Array<PageId>");
}

#[test]
fn parser_type_alias_surface_rejects_missing_target_type() {
    let err = NyashParser::parse_from_string("type Bytes =").unwrap_err();
    assert!(err.to_string().contains("type alias target"), "{err}");
}
