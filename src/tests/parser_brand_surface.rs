use crate::ast::ASTNode;
use crate::parser::NyashParser;

#[test]
fn parser_brand_surface_parses_brand_declaration_metadata() {
    let ast = NyashParser::parse_from_string(
        r#"
brand PageId: i64
brand Bytes: usize
"#,
    )
    .expect("parse brand declarations");

    let ASTNode::Program { statements, .. } = ast else {
        panic!("expected Program");
    };
    assert_eq!(statements.len(), 2);

    let ASTNode::BrandDeclaration {
        name,
        underlying_type_name,
        ..
    } = &statements[0]
    else {
        panic!("expected first brand declaration");
    };
    assert_eq!(name, "PageId");
    assert_eq!(underlying_type_name, "i64");

    let ASTNode::BrandDeclaration {
        name,
        underlying_type_name,
        ..
    } = &statements[1]
    else {
        panic!("expected second brand declaration");
    };
    assert_eq!(name, "Bytes");
    assert_eq!(underlying_type_name, "usize");
}

#[test]
fn parser_brand_surface_rejects_missing_underlying_type() {
    let err = NyashParser::parse_from_string("brand PageId:").unwrap_err();
    assert!(err.to_string().contains("brand underlying type"), "{err}");
}
