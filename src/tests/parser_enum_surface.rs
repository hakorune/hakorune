use crate::ast::{ASTNode, EnumVariantDecl};
use crate::parser::NyashParser;

#[test]
fn parse_enum_surface_keeps_type_parameters_and_variants() {
    let src = r#"
enum Option<T> {
  None
  Some(T)
}
"#;

    let ast = NyashParser::parse_from_string(src).expect("parse ok");
    let ASTNode::Program { statements, .. } = ast else {
        panic!("expected program");
    };

    let Some(ASTNode::EnumDeclaration {
        name,
        type_parameters,
        variants,
        ..
    }) = statements.first()
    else {
        panic!("expected enum declaration");
    };

    assert_eq!(name, "Option");
    assert_eq!(type_parameters, &vec!["T".to_string()]);
    assert_eq!(
        variants,
        &vec![
            EnumVariantDecl {
                name: "None".to_string(),
                payload_type_name: None,
                record_field_decls: vec![],
            },
            EnumVariantDecl {
                name: "Some".to_string(),
                payload_type_name: Some("T".to_string()),
                record_field_decls: vec![],
            },
        ]
    );
}

#[test]
fn parse_enum_surface_accepts_record_variants() {
    let src = r#"
enum Token {
  Ident { name: String }
  Eof
}
"#;

    let ast = NyashParser::parse_from_string(src).expect("parse ok");
    let ASTNode::Program { statements, .. } = ast else {
        panic!("expected program");
    };

    let Some(ASTNode::EnumDeclaration { variants, .. }) = statements.first() else {
        panic!("expected enum declaration");
    };

    assert_eq!(variants.len(), 2);
    assert_eq!(variants[0].name, "Ident");
    assert!(variants[0].payload_type_name.is_none());
    assert_eq!(variants[0].record_field_decls.len(), 1);
    assert_eq!(variants[0].record_field_decls[0].name, "name");
    assert_eq!(
        variants[0].record_field_decls[0]
            .declared_type_name
            .as_deref(),
        Some("String")
    );
    assert_eq!(variants[1].name, "Eof");
    assert!(variants[1].record_field_decls.is_empty());
}

#[test]
fn parse_enum_surface_rejects_multi_payload_tuple_variants_for_now() {
    let src = r#"
enum Pair {
  Both(Integer, Integer)
}
"#;

    let error = NyashParser::parse_from_string(src).expect_err("multi-payload tuple stays out");
    let message = error.to_string();
    assert!(message.contains("single payload variant in the current enum surface"));
}
