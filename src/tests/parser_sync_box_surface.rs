use crate::ast::ASTNode;
use crate::parser::NyashParser;
use crate::r#macro::ast_json::{ast_to_json_roundtrip, json_to_ast};

fn parse(src: &str) -> ASTNode {
    NyashParser::parse_from_string(src).expect("parse ok")
}

fn find_box<'a>(ast: &'a ASTNode, box_name: &str) -> &'a ASTNode {
    let ASTNode::Program { statements, .. } = ast else {
        panic!("expected Program");
    };
    statements
        .iter()
        .find(|stmt| matches!(stmt, ASTNode::BoxDeclaration { name, .. } if name == box_name))
        .expect("box declaration not found")
}

#[test]
fn parser_accepts_sync_box_as_contextual_surface() {
    let ast = parse(
        r#"
sync box Counter {
  value: i64

  inc(delta: i64): i64 {
    me.value = me.value + delta
    return me.value
  }
}
"#,
    );

    let ASTNode::BoxDeclaration {
        name,
        is_sync,
        is_record,
        is_static,
        field_decls,
        methods,
        ..
    } = find_box(&ast, "Counter")
    else {
        panic!("expected BoxDeclaration");
    };

    assert_eq!(name, "Counter");
    assert!(*is_sync, "sync box must carry the sync capsule");
    assert!(!*is_record, "sync box is not a record");
    assert!(!*is_static, "sync box is not a static box");
    assert_eq!(field_decls.len(), 1);
    assert!(methods.contains_key("inc"));
}

#[test]
fn parser_keeps_plain_box_non_sync() {
    let ast = parse(
        r#"
box Plain {
  value: i64
}
"#,
    );

    let ASTNode::BoxDeclaration { is_sync, .. } = find_box(&ast, "Plain") else {
        panic!("expected BoxDeclaration");
    };
    assert!(!*is_sync, "ordinary box must not become sync");
}

#[test]
fn parser_keeps_sync_contextual_for_bindings() {
    let ast = parse(
        r#"
box Main {
  run() {
    local sync = 1
    return sync
  }
}
"#,
    );

    let ASTNode::BoxDeclaration { methods, .. } = find_box(&ast, "Main") else {
        panic!("expected BoxDeclaration");
    };
    let ASTNode::FunctionDeclaration { body, .. } = methods.get("run").expect("run method") else {
        panic!("expected FunctionDeclaration");
    };
    assert!(matches!(body[0], ASTNode::Local { .. }));
    assert!(matches!(body[1], ASTNode::Return { .. }));
}

#[test]
fn ast_json_roundtrip_preserves_sync_box_capsule() {
    let ast = parse(
        r#"
sync box Counter {
  value: i64
}
"#,
    );
    let json = ast_to_json_roundtrip(&ast);
    assert_eq!(json["statements"][0]["kind"], "BoxDeclaration");
    assert_eq!(json["statements"][0]["is_sync"], true);

    let roundtrip = json_to_ast(&json).expect("ast json roundtrip");
    let ASTNode::BoxDeclaration { is_sync, .. } = find_box(&roundtrip, "Counter") else {
        panic!("expected BoxDeclaration");
    };
    assert!(*is_sync, "roundtrip must preserve sync capsule");
}

#[test]
fn sync_box_rejects_await_in_methods() {
    let error = NyashParser::parse_from_string(
        r#"
sync box Counter {
  read(fut) {
    return await fut
  }
}
"#,
    )
    .expect_err("sync method await must fail-fast");

    let message = error.to_string();
    assert!(message.contains("[sync_box/wait_forbidden]"), "{message}");
    assert!(message.contains("wait_kind=await"), "{message}");
}

#[test]
fn sync_box_rejects_nowait_in_methods() {
    let error = NyashParser::parse_from_string(
        r#"
sync box Counter {
  read() {
    nowait fut = me.compute()
    return 0
  }

  compute() {
    return 1
  }
}
"#,
    )
    .expect_err("sync method nowait must fail-fast");

    let message = error.to_string();
    assert!(message.contains("[sync_box/wait_forbidden]"), "{message}");
    assert!(message.contains("wait_kind=nowait"), "{message}");
}
