use crate::ast::ASTNode;
use crate::parser::NyashParser;

#[test]
fn parser_record_literal_surface_parses_explicit_named_fields() {
    let ast = NyashParser::parse_from_string(
        r#"
record Meta {
  ptr: i64
  size: usize
}

static box Main {
  main() {
local meta = Meta { ptr: 1, size: 2 }
return 0
  }
}
"#,
    )
    .expect("parse record literal");

    let ASTNode::Program { statements, .. } = ast else {
        panic!("expected Program");
    };
    let main_box = statements
        .iter()
        .find(|stmt| matches!(stmt, ASTNode::BoxDeclaration { name, .. } if name == "Main"))
        .expect("Main box");
    let ASTNode::BoxDeclaration { methods, .. } = main_box else {
        panic!("expected box");
    };
    let ASTNode::FunctionDeclaration { body, .. } = &methods["main"] else {
        panic!("expected main method");
    };
    let ASTNode::Local { initial_values, .. } = &body[0] else {
        panic!("expected local statement");
    };
    let Some(value) = initial_values[0].as_deref() else {
        panic!("expected local initializer");
    };
    let ASTNode::RecordLiteral {
        record_type_name,
        fields,
        ..
    } = value
    else {
        panic!("expected RecordLiteral");
    };
    assert_eq!(record_type_name, "Meta");
    assert_eq!(fields.len(), 2);
    assert_eq!(fields[0].0, "ptr");
    assert_eq!(fields[1].0, "size");
}

#[test]
fn parser_record_literal_surface_rejects_shorthand_field() {
    let err = NyashParser::parse_from_string(
        r#"
static box Main {
  main() {
local ptr = 1
local meta = Meta { ptr }
return 0
  }
}
"#,
    )
    .expect_err("record literal shorthand is deferred");
    assert!(err.to_string().contains("COLON"), "{err}");
}
