use crate::ast::{ASTNode, LiteralValue};
use crate::tests::helpers::parser::{find_method_body, parse_ok};

#[test]
fn parser_box_new_field_initializer_surface_parses_explicit_fields() {
    let ast = parse_ok(
        r#"
box Report {
  accepted: i64 = 0
  reason: i64 = 0
}

static box Main {
  main() {
    local report = new Report { accepted: 1, reason: 2 }
    return report
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
    let ASTNode::New {
        class,
        arguments,
        field_initializers,
        ..
    } = value
    else {
        panic!("expected New node");
    };
    assert_eq!(class, "Report");
    assert!(arguments.is_empty());
    assert_eq!(field_initializers.len(), 2);
    assert_eq!(field_initializers[0].0, "accepted");
    assert_eq!(field_initializers[1].0, "reason");
    assert!(matches!(
        &field_initializers[0].1,
        ASTNode::Literal {
            value: LiteralValue::Integer(1),
            ..
        }
    ));
    assert!(matches!(
        &field_initializers[1].1,
        ASTNode::Literal {
            value: LiteralValue::Integer(2),
            ..
        }
    ));
}

#[test]
fn parser_box_new_field_initializer_surface_keeps_empty_initializer_compatible() {
    let ast = parse_ok(
        r#"
box Report {
  accepted: i64 = 0
}

static box Main {
  main() {
    local report = new Report()
    return report
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
    let ASTNode::New {
        arguments,
        field_initializers,
        ..
    } = value
    else {
        panic!("expected New node");
    };
    assert!(arguments.is_empty());
    assert!(field_initializers.is_empty());
}
