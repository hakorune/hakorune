use crate::ast::ASTNode;
use crate::parser::NyashParser;
use crate::tests::helpers::parser::{find_method_body, parse_ok};

#[test]
fn parser_record_literal_surface_parses_explicit_named_fields() {
    let ast = parse_ok(
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
    );
    let body = find_method_body(&ast, "Main", "main");
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

#[test]
fn parser_record_update_surface_parses_explicit_named_updates() {
    let ast = parse_ok(
        r#"
record Meta {
  ptr: i64
  size: usize
}

static box Main {
  main() {
local meta = Meta { ptr: 1, size: 2 }
local next = meta with { size: 3 }
return 0
  }
}
"#,
    );
    let body = find_method_body(&ast, "Main", "main");
    let ASTNode::Local { initial_values, .. } = &body[1] else {
        panic!("expected second local statement");
    };
    let Some(value) = initial_values[0].as_deref() else {
        panic!("expected local initializer");
    };
    let ASTNode::RecordUpdate { base, updates, .. } = value else {
        panic!("expected RecordUpdate");
    };
    assert!(matches!(base.as_ref(), ASTNode::Variable { name, .. } if name == "meta"));
    assert_eq!(updates.len(), 1);
    assert_eq!(updates[0].0, "size");
}
