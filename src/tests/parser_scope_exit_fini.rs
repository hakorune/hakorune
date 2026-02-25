use crate::ast::ASTNode;
use crate::parser::NyashParser;

fn enable_stage3() {
    std::env::set_var("NYASH_FEATURES", "stage3");
}

fn function_body<'a>(ast: &'a ASTNode, function_name: &str) -> &'a Vec<ASTNode> {
    fn walk<'a>(node: &'a ASTNode, function_name: &str) -> Option<&'a Vec<ASTNode>> {
        match node {
            ASTNode::Program { statements, .. } => {
                for stmt in statements {
                    if let Some(body) = walk(stmt, function_name) {
                        return Some(body);
                    }
                }
                None
            }
            ASTNode::FunctionDeclaration { name, body, .. } if name == function_name => Some(body),
            _ => None,
        }
    }

    walk(ast, function_name).expect("target function body not found")
}

#[test]
fn fini_wraps_only_suffix_statements() {
    enable_stage3();
    let src = r#"
function main() {
  print("pre")
  fini {
    print("cleanup")
  }
  return 1
}
"#;
    let ast = NyashParser::parse_from_string(src).expect("parse ok");
    let body = function_body(&ast, "main");
    assert_eq!(
        body.len(),
        2,
        "pre-statement must remain outside fini wrapper"
    );
    assert!(matches!(body[0], ASTNode::Print { .. }));
    match &body[1] {
        ASTNode::TryCatch {
            try_body,
            catch_clauses,
            finally_body,
            ..
        } => {
            assert!(
                catch_clauses.is_empty(),
                "fini wrapper must not add catch clauses"
            );
            assert!(matches!(try_body.as_slice(), [ASTNode::Return { .. }]));
            assert!(
                finally_body.is_some(),
                "fini wrapper must carry cleanup body"
            );
        }
        _ => panic!("expected TryCatch wrapper for fini suffix"),
    }
}

#[test]
fn local_fini_sugar_keeps_local_then_wraps_tail() {
    enable_stage3();
    let src = r#"
function main() {
  local x = 1 fini {
    print(x)
  }
  x = x + 1
  return x
}
"#;
    let ast = NyashParser::parse_from_string(src).expect("parse ok");
    let body = function_body(&ast, "main");
    assert!(matches!(body.first(), Some(ASTNode::Local { .. })));
    match body.get(1) {
        Some(ASTNode::TryCatch {
            try_body,
            catch_clauses,
            finally_body,
            ..
        }) => {
            assert!(
                catch_clauses.is_empty(),
                "local-fini lowers to cleanup-only wrapper"
            );
            assert_eq!(try_body.len(), 2, "tail statements must be wrapped");
            assert!(
                finally_body.is_some(),
                "local-fini must register cleanup body"
            );
        }
        _ => panic!("expected local-fini wrapper after local declaration"),
    }
}

#[test]
fn fini_is_lifo_order_in_ast_shape() {
    enable_stage3();
    let src = r#"
function main() {
  fini { print("A") }
  fini { print("B") }
  return 0
}
"#;
    let ast = NyashParser::parse_from_string(src).expect("parse ok");
    let body = function_body(&ast, "main");
    assert_eq!(
        body.len(),
        1,
        "consecutive fini handlers should nest into one suffix chain"
    );
    let outer = body.first().expect("outer wrapper");
    match outer {
        ASTNode::TryCatch {
            try_body,
            finally_body,
            ..
        } => {
            assert!(
                finally_body.is_some(),
                "outer wrapper must carry first fini"
            );
            match try_body.as_slice() {
                [ASTNode::TryCatch {
                    finally_body: inner_finally,
                    ..
                }] => {
                    assert!(
                        inner_finally.is_some(),
                        "inner wrapper must carry second fini"
                    );
                }
                _ => panic!("expected nested TryCatch for LIFO fini order"),
            }
        }
        _ => panic!("expected outer TryCatch for fini chain"),
    }
}

#[test]
fn fini_block_rejects_non_local_exit() {
    enable_stage3();
    let src = r#"
function main() {
  fini {
    return 1
  }
  return 0
}
"#;
    assert!(
        NyashParser::parse_from_string(src).is_err(),
        "fini must reject return/break/continue/throw"
    );
}
