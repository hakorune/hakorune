use crate::ast::ASTNode;
use crate::tests::helpers::parser::{find_method_body, parse_ok};

#[test]
fn parser_accepts_canonical_co_task_scope_surface() {
    let source = r#"
box Main {
  run() {
    co {
      local value = 1
    }
    return 0
  }
}
"#;

    let ast = parse_ok(source);
    let body = find_method_body(&ast, "Main", "run");
    let ASTNode::TaskScope {
        source_keyword,
        body: task_body,
        ..
    } = &body[0]
    else {
        panic!("expected first statement to be TaskScope");
    };
    assert_eq!(source_keyword, "co");
    assert_eq!(task_body.len(), 1);
}

#[test]
fn parser_accepts_task_scope_compat_spelling() {
    let source = r#"
box Main {
  run() {
    task_scope {
      local value = 1
    }
    return 0
  }
}
"#;

    let ast = parse_ok(source);
    let body = find_method_body(&ast, "Main", "run");
    let ASTNode::TaskScope { source_keyword, .. } = &body[0] else {
        panic!("expected first statement to be TaskScope");
    };
    assert_eq!(source_keyword, "task_scope");
}

#[test]
fn parser_keeps_co_contextual_for_bindings_and_calls() {
    let source = r#"
box Main {
  co() {
    return 7
  }

  run() {
    local co = 1
    return me.co() + co
  }
}
"#;

    let ast = parse_ok(source);
    let body = find_method_body(&ast, "Main", "run");
    assert!(matches!(body[0], ASTNode::Local { .. }));
    assert!(matches!(body[1], ASTNode::Return { .. }));
}
