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
fn parser_accepts_static_const_u16_table_const_exprs() {
    let ast = NyashParser::parse_from_string(
        r#"
static const SIZE_CLASS: u16[] = [8 + 8, 3 * 8, 1 << 5, (40 - 8) | 1]
static box Main {
  main() {
    return 0
  }
}
"#,
    )
    .expect("parse static const table const expressions");

    let ASTNode::Program { statements, .. } = ast else {
        panic!("expected program");
    };
    let ASTNode::StaticConstTable { values, .. } = &statements[0] else {
        panic!("expected StaticConstTable");
    };

    assert_eq!(values, &vec![16, 24, 32, 33]);
}

#[test]
fn parser_rejects_static_const_u16_expr_out_of_range() {
    let err = NyashParser::parse_from_string("static const BAD: u16[] = [1 << 16]").unwrap_err();
    assert!(
        err.to_string()
            .contains("[static-const/value-out-of-range]"),
        "{err}"
    );
}

#[test]
fn parser_rejects_static_const_u16_negative_shift_operand() {
    let err = NyashParser::parse_from_string("static const BAD: u16[] = [(-1) >> 1]").unwrap_err();
    assert!(
        err.to_string()
            .contains("[static-const/unsupported-initializer]"),
        "{err}"
    );
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
