use crate::ast::ASTNode;
use crate::parser::NyashParser;

#[test]
fn parser_accepts_static_const_u16_table_decl() {
    let ast = NyashParser::parse_from_string(
        r#"
static const SIZE_CLASS: u16[] = [8, 16, 24, 32]
static box Main {
  main() {
    return 0
  }
}
"#,
    )
    .expect("parse static const table");

    let ASTNode::Program { statements, .. } = ast else {
        panic!("expected program");
    };
    let ASTNode::StaticConstTable {
        name,
        element_type,
        values,
        ..
    } = &statements[0]
    else {
        panic!("expected StaticConstTable");
    };

    assert_eq!(name, "SIZE_CLASS");
    assert_eq!(element_type, "u16");
    assert_eq!(values, &vec![8, 16, 24, 32]);
}

#[test]
fn parser_rejects_static_const_non_u16_table_decl() {
    let err = NyashParser::parse_from_string("static const BAD: u32[] = [1]").unwrap_err();
    assert!(
        err.to_string()
            .contains("[static-const/unsupported-element]"),
        "{err}"
    );
}
