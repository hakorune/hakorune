use crate::ast::ASTNode;
use crate::parser::NyashParser;

fn find_method_body<'a>(ast: &'a ASTNode, box_name: &str, method_name: &str) -> &'a [ASTNode] {
    let ASTNode::Program { statements, .. } = ast else {
        panic!("expected Program");
    };
    for statement in statements {
        if let ASTNode::BoxDeclaration { name, methods, .. } = statement {
            if name != box_name {
                continue;
            }
            if let Some(ASTNode::FunctionDeclaration { body, .. }) = methods.get(method_name) {
                return body;
            }
        }
    }
    panic!("method not found: {}.{}", box_name, method_name);
}

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

    let ast = NyashParser::parse_from_string(source).expect("parse co scope");
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

    let ast = NyashParser::parse_from_string(source).expect("parse task_scope compat");
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

    let ast = NyashParser::parse_from_string(source).expect("parse contextual co");
    let body = find_method_body(&ast, "Main", "run");
    assert!(matches!(body[0], ASTNode::Local { .. }));
    assert!(matches!(body[1], ASTNode::Return { .. }));
}
