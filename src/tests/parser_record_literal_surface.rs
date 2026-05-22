use crate::ast::ASTNode;
use crate::r#macro::ast_json::{ast_to_json_roundtrip, json_to_ast};
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
fn parser_record_literal_surface_parses_shorthand_field() {
    let ast = parse_ok(
        r#"
record Meta {
  ptr: i64
}

static box Main {
  main() {
local ptr = 1
local meta = Meta { ptr }
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
    let ASTNode::RecordLiteral { fields, .. } = value else {
        panic!("expected RecordLiteral");
    };
    assert_eq!(fields.len(), 1);
    assert_eq!(fields[0].0, "ptr");
    assert!(matches!(&fields[0].1, ASTNode::Variable { name, .. } if name == "ptr"));
}

#[test]
fn parser_record_literal_surface_allows_same_type_and_value_name() {
    let ast = parse_ok(
        r#"
record Meta {
  Meta: i64 = 0
}

static box Main {
  main() {
local Meta = 1
local rec = Meta { Meta }
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
    let ASTNode::RecordLiteral {
        record_type_name,
        fields,
        ..
    } = value
    else {
        panic!("expected RecordLiteral");
    };
    assert_eq!(record_type_name, "Meta");
    assert_eq!(fields.len(), 1);
    assert_eq!(fields[0].0, "Meta");
    assert!(matches!(&fields[0].1, ASTNode::Variable { name, .. } if name == "Meta"));
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

#[test]
fn parser_record_surface_parses_field_defaults_and_empty_literal() {
    let ast = parse_ok(
        r#"
record Meta {
  ptr: i64 = 0
  size: i64 = -1
}

static box Main {
  main() {
local meta = Meta {}
return 0
  }
}
"#,
    );
    let ASTNode::Program { statements, .. } = &ast else {
        panic!("expected Program");
    };
    let ASTNode::BoxDeclaration {
        field_decls,
        is_record,
        ..
    } = &statements[0]
    else {
        panic!("expected record declaration");
    };
    assert!(*is_record);
    assert_eq!(field_decls.len(), 2);
    assert!(field_decls.iter().all(|decl| decl.default_value.is_some()));

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
        panic!("expected empty RecordLiteral");
    };
    assert_eq!(record_type_name, "Meta");
    assert!(fields.is_empty());
}

#[test]
fn parser_record_surface_roundtrips_field_defaults_through_ast_json() {
    let ast = parse_ok(
        r#"
record Meta {
  ptr: i64 = 0
}
"#,
    );
    let roundtrip = json_to_ast(&ast_to_json_roundtrip(&ast)).expect("roundtrip");
    let ASTNode::Program { statements, .. } = &roundtrip else {
        panic!("expected Program");
    };
    let ASTNode::BoxDeclaration { field_decls, .. } = &statements[0] else {
        panic!("expected record declaration");
    };
    assert_eq!(field_decls.len(), 1);
    assert!(field_decls[0].default_value.is_some());
}

#[test]
fn parser_record_update_surface_parses_shorthand_update() {
    let ast = parse_ok(
        r#"
record Meta {
  ptr: i64
  size: i64
}

static box Main {
  main() {
local meta = Meta { ptr: 1, size: 2 }
local size = 3
local next = meta with { size }
return 0
  }
}
"#,
    );
    let body = find_method_body(&ast, "Main", "main");
    let ASTNode::Local { initial_values, .. } = &body[2] else {
        panic!("expected third local statement");
    };
    let Some(value) = initial_values[0].as_deref() else {
        panic!("expected local initializer");
    };
    let ASTNode::RecordUpdate { updates, .. } = value else {
        panic!("expected RecordUpdate");
    };
    assert_eq!(updates.len(), 1);
    assert_eq!(updates[0].0, "size");
    assert!(matches!(&updates[0].1, ASTNode::Variable { name, .. } if name == "size"));
}
