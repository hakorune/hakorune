use crate::ast::ASTNode;
use crate::parser::NyashParser;
use crate::r#macro::ast_json::{ast_to_json_roundtrip, json_to_ast};

fn parse(src: &str) -> ASTNode {
    NyashParser::parse_from_string(src).expect("parse ok")
}

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
fn parser_accepts_canonical_context_scope_surface() {
    let ast = parse(
        r#"
box Main {
  run(rid: RequestId) {
    context request_id: RequestId = rid {
      local value = 1
    }
    return 0
  }
}
"#,
    );

    let body = find_method_body(&ast, "Main", "run");
    let ASTNode::ContextScope {
        name,
        declared_type_name,
        source_keyword,
        body: context_body,
        ..
    } = &body[0]
    else {
        panic!("expected ContextScope");
    };
    assert_eq!(source_keyword, "context");
    assert_eq!(name, "request_id");
    assert_eq!(declared_type_name.as_deref(), Some("RequestId"));
    assert_eq!(context_body.len(), 1);
}

#[test]
fn parser_accepts_scoped_compat_spelling() {
    let ast = parse(
        r#"
box Main {
  run(rid) {
    scoped request_id = rid {
      local value = 1
    }
    return 0
  }
}
"#,
    );

    let body = find_method_body(&ast, "Main", "run");
    let ASTNode::ContextScope { source_keyword, .. } = &body[0] else {
        panic!("expected ContextScope");
    };
    assert_eq!(source_keyword, "scoped");
}

#[test]
fn parser_keeps_context_contextual_for_calls_and_bindings() {
    let ast = parse(
        r#"
box Main {
  context() {
    return 7
  }

  run() {
    local context = 1
    context()
    return context
  }
}
"#,
    );

    let body = find_method_body(&ast, "Main", "run");
    assert!(matches!(body[0], ASTNode::Local { .. }));
    assert!(matches!(body[1], ASTNode::FunctionCall { .. }));
    assert!(matches!(body[2], ASTNode::Return { .. }));
}

#[test]
fn ast_json_roundtrip_preserves_context_scope_capsule() {
    let ast = parse(
        r#"
box Main {
  run(rid: RequestId) {
    context request_id: RequestId = rid {
      local value = 1
    }
    return 0
  }
}
"#,
    );
    let json = ast_to_json_roundtrip(&ast);
    let context_json = &json["statements"][0]["methods"][0]["decl"]["body"][0];
    assert_eq!(context_json["kind"], "ContextScope");
    assert_eq!(context_json["spelling"], "context");
    assert_eq!(context_json["name"], "request_id");
    assert_eq!(context_json["declared_type"], "RequestId");

    let roundtrip = json_to_ast(&json).expect("ast json roundtrip");
    let body = find_method_body(&roundtrip, "Main", "run");
    let ASTNode::ContextScope {
        name,
        declared_type_name,
        source_keyword,
        ..
    } = &body[0]
    else {
        panic!("expected ContextScope");
    };
    assert_eq!(source_keyword, "context");
    assert_eq!(name, "request_id");
    assert_eq!(declared_type_name.as_deref(), Some("RequestId"));
}
